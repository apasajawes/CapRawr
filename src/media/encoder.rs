use std::path::{Path, PathBuf};
use std::process::Command;
use anyhow::{Context, Result, anyhow};
use crate::engine::project::Project;
use crate::engine::track::TrackKind;
use super::ffmpeg::find_ffmpeg;

#[derive(Debug, Clone)]
pub struct ExportSettings {
    pub output_path: PathBuf,
    pub width: u32,
    pub height: u32,
    pub fps: f64,
    pub bitrate_kbps: u32,
    pub video_codec: String,
}

impl Default for ExportSettings {
    fn default() -> Self {
        Self {
            output_path: PathBuf::from("export.mp4"),
            width: 1920,
            height: 1080,
            fps: 30.0,
            bitrate_kbps: 8000,
            video_codec: "auto".into(),
        }
    }
}

/// Helper: check if file is an image format
pub fn is_image_file(path: &Path) -> bool {
    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
        matches!(
            ext.to_ascii_lowercase().as_str(),
            "png" | "jpg" | "jpeg" | "webp" | "bmp" | "gif" | "tiff"
        )
    } else {
        false
    }
}

/// Detect the best available hardware/software H.264 video encoder
pub fn detect_best_video_encoder(ffmpeg_bin: &Path) -> String {
    if let Ok(output) = Command::new(ffmpeg_bin).arg("-encoders").output() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        // Preference: NVENC > MediaFoundation (Windows standard) > AMF > QSV > libx264
        if stdout.contains("h264_nvenc") {
            return "h264_nvenc".into();
        }
        if stdout.contains("h264_mf") {
            return "h264_mf".into();
        }
        if stdout.contains("h264_amf") {
            return "h264_amf".into();
        }
        if stdout.contains("h264_qsv") {
            return "h264_qsv".into();
        }
        if stdout.contains("libx264") {
            return "libx264".into();
        }
    }
    // Fallback
    "h264_mf".into()
}

/// Detect best audio encoder
pub fn detect_best_audio_encoder(ffmpeg_bin: &Path) -> String {
    if let Ok(output) = Command::new(ffmpeg_bin).arg("-encoders").output() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        if stdout.contains("aac") {
            return "aac".into();
        }
        if stdout.contains("aac_mf") {
            return "aac_mf".into();
        }
    }
    "aac".into()
}

/// Build FFmpeg command to render and export project
pub fn build_export_command(
    ffmpeg_bin: &Path,
    project: &Project,
    settings: &ExportSettings,
) -> Result<Command> {
    // Gather all visible video clips in chronological order
    let mut clips: Vec<_> = project
        .timeline
        .tracks
        .iter()
        .filter(|t| t.kind == TrackKind::Video && t.visible)
        .flat_map(|t| t.clips.iter())
        .collect();

    if clips.is_empty() {
        return Err(anyhow!("Timeline has no video clips to export"));
    }

    clips.sort_by(|a, b| a.position.partial_cmp(&b.position).unwrap());

    let mut cmd = Command::new(ffmpeg_bin);
    cmd.arg("-y"); // overwrite output without prompt

    // Add inputs
    for clip in &clips {
        let is_img = is_image_file(&clip.source);
        if is_img {
            cmd.arg("-loop").arg("1")
               .arg("-t").arg(format!("{:.3}", clip.duration))
               .arg("-i").arg(&clip.source);
        } else {
            let actual_duration = clip.duration / clip.speed.max(0.01);
            cmd.arg("-ss").arg(format!("{:.3}", clip.source_start))
               .arg("-t").arg(format!("{:.3}", actual_duration))
               .arg("-i").arg(&clip.source);
        }
    }

    // Build filter_complex: scale & pad each input, then concat
    let mut filter_complex = String::new();
    let num_clips = clips.len();

    for (i, clip) in clips.iter().enumerate() {
        let mut filter = format!(
            "[{}:v]scale={}:{}:force_original_aspect_ratio=decrease,pad={}:{}:(ow-iw)/2:(oh-ih)/2,setsar=1,fps={}",
            i, settings.width, settings.height, settings.width, settings.height, settings.fps
        );
        if (clip.speed - 1.0).abs() > 0.01 && !is_image_file(&clip.source) {
            let pts_factor = 1.0 / clip.speed.max(0.01);
            filter.push_str(&format!(",setpts={:.4}*PTS", pts_factor));
        }
        filter.push_str(&format!("[v{}];", i));
        filter_complex.push_str(&filter);
    }

    // Concat inputs
    for i in 0..num_clips {
        filter_complex.push_str(&format!("[v{}]", i));
    }
    filter_complex.push_str(&format!("concat=n={}:v=1:a=0[outv]", num_clips));

    cmd.arg("-filter_complex").arg(filter_complex);
    cmd.arg("-map").arg("[outv]");

    // Codec selection
    let encoder = if settings.video_codec == "auto" {
        detect_best_video_encoder(ffmpeg_bin)
    } else {
        settings.video_codec.clone()
    };

    cmd.arg("-c:v").arg(&encoder);
    cmd.arg("-b:v").arg(format!("{}k", settings.bitrate_kbps));
    cmd.arg("-pix_fmt").arg("yuv420p");
    cmd.arg(&settings.output_path);

    Ok(cmd)
}

/// Export project synchronously
pub fn export_project(project: &Project, settings: &ExportSettings) -> Result<()> {
    let ffmpeg_bin = find_ffmpeg()
        .ok_or_else(|| anyhow!("ffmpeg not found in PATH or CapCut directory"))?;

    let mut cmd = build_export_command(&ffmpeg_bin, project, settings)?;
    tracing::info!("Executing export command: {:?}", cmd);

    let output = cmd.output().context("Failed to spawn ffmpeg export process")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow!("ffmpeg export failed:\n{}", stderr));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_image_file() {
        assert!(is_image_file(Path::new("photo.png")));
        assert!(is_image_file(Path::new("photo.JPG")));
        assert!(!is_image_file(Path::new("video.mp4")));
        assert!(!is_image_file(Path::new("video.mkv")));
    }

    #[test]
    fn test_detect_video_encoder() {
        if let Some(ffmpeg) = find_ffmpeg() {
            let enc = detect_best_video_encoder(&ffmpeg);
            assert!(!enc.is_empty());
        }
    }

    #[test]
    fn test_build_export_command_empty_fails() {
        let p = Project::default();
        let settings = ExportSettings::default();
        let res = build_export_command(Path::new("ffmpeg"), &p, &settings);
        assert!(res.is_err());
    }

    #[test]
    fn test_build_export_command_with_clip() {
        let mut p = Project::default();
        p.timeline.add_video_track();
        p.timeline.tracks[0].add_clip(crate::engine::clip::Clip::new(
            PathBuf::from("test.mp4"),
            10.0,
        ));
        let settings = ExportSettings::default();
        let res = build_export_command(Path::new("ffmpeg"), &p, &settings);
        assert!(res.is_ok());
    }
}
