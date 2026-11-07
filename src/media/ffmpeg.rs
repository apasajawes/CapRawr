use std::path::{Path, PathBuf};
use std::process::Command;
use anyhow::{Context, Result, anyhow};

/// Find ffmpeg executable: check PATH first, fallback to CapCut directory
pub fn find_ffmpeg() -> Option<PathBuf> {
    if let Ok(output) = Command::new("ffmpeg").arg("-version").output() {
        if output.status.success() {
            return Some("ffmpeg".into());
        }
    }

    // Fallback: check CapCut install directory
    let local_app_data = std::env::var("LOCALAPPDATA").ok()?;
    let capcut_dir = Path::new(&local_app_data).join("CapCut").join("Apps");
    if capcut_dir.exists() {
        if let Ok(entries) = std::fs::read_dir(&capcut_dir) {
            for entry in entries.flatten() {
                let candidate = entry.path().join("ffmpeg.exe");
                if candidate.exists() {
                    return Some(candidate);
                }
            }
        }
    }

    None
}

/// Find ffprobe executable
pub fn find_ffprobe() -> Option<PathBuf> {
    if let Ok(output) = Command::new("ffprobe").arg("-version").output() {
        if output.status.success() {
            return Some("ffprobe".into());
        }
    }

    // Fallback: check CapCut install directory
    let local_app_data = std::env::var("LOCALAPPDATA").ok()?;
    let capcut_dir = Path::new(&local_app_data).join("CapCut").join("Apps");
    if capcut_dir.exists() {
        if let Ok(entries) = std::fs::read_dir(&capcut_dir) {
            for entry in entries.flatten() {
                let candidate = entry.path().join("ffprobe.exe");
                if candidate.exists() {
                    return Some(candidate);
                }
            }
        }
    }

    None
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct FfprobeOutput {
    pub streams: Option<Vec<FfprobeStream>>,
    pub format: Option<FfprobeFormat>,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct FfprobeStream {
    pub index: usize,
    pub codec_type: String,
    pub codec_name: Option<String>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub r_frame_rate: Option<String>,
    pub duration: Option<String>,
    pub nb_frames: Option<String>,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct FfprobeFormat {
    pub duration: Option<String>,
    pub size: Option<String>,
    pub bit_rate: Option<String>,
    pub format_name: Option<String>,
}

/// Probe media file using ffprobe JSON output
pub fn probe_file(path: &Path) -> Result<FfprobeOutput> {
    let probe_bin = find_ffprobe()
        .ok_or_else(|| anyhow!("ffprobe not found in PATH or CapCut directory"))?;

    let output = Command::new(probe_bin)
        .arg("-v").arg("quiet")
        .arg("-print_format").arg("json")
        .arg("-show_format")
        .arg("-show_streams")
        .arg(path)
        .output()
        .context("Failed to run ffprobe")?;

    if !output.status.success() {
        return Err(anyhow!("ffprobe failed on {}", path.display()));
    }

    let parsed: FfprobeOutput = serde_json::from_slice(&output.stdout)
        .context("Failed to parse ffprobe JSON output")?;

    Ok(parsed)
}

/// Extract a single frame as raw RGBA image at a given timestamp
pub fn extract_frame(
    video_path: &Path,
    timestamp_sec: f64,
    target_width: u32,
    target_height: u32,
) -> Result<image::RgbaImage> {
    let ffmpeg_bin = find_ffmpeg()
        .ok_or_else(|| anyhow!("ffmpeg not found in PATH or CapCut directory"))?;

    let output = Command::new(ffmpeg_bin)
        .arg("-ss").arg(format!("{:.3}", timestamp_sec))
        .arg("-i").arg(video_path)
        .arg("-vf").arg(format!("scale={}:{}", target_width, target_height))
        .arg("-vframes").arg("1")
        .arg("-f").arg("rawvideo")
        .arg("-pix_fmt").arg("rgba")
        .arg("-")
        .output()
        .context("Failed to extract frame with ffmpeg")?;

    let expected_len = (target_width * target_height * 4) as usize;
    if output.stdout.len() < expected_len {
        return Err(anyhow!(
            "ffmpeg output too short: got {} bytes, expected {}",
            output.stdout.len(),
            expected_len
        ));
    }

    image::RgbaImage::from_raw(target_width, target_height, output.stdout[..expected_len].to_vec())
        .ok_or_else(|| anyhow!("Failed to construct RgbaImage from raw bytes"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_ffmpeg_or_capcut_fallback() {
        // Must locate ffmpeg either on PATH or from CapCut installation
        let ffmpeg = find_ffmpeg();
        assert!(ffmpeg.is_some(), "ffmpeg should be found via PATH or CapCut folder");
    }

    #[test]
    fn test_find_ffprobe_or_capcut_fallback() {
        let ffprobe = find_ffprobe();
        assert!(ffprobe.is_some(), "ffprobe should be found via PATH or CapCut folder");
    }
}

