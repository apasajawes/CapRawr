use std::path::Path;
use anyhow::Result;
use super::ffmpeg;

#[derive(Debug, Clone)]
pub struct VideoInfo {
    pub width: u32,
    pub height: u32,
    pub duration: f64,
    pub fps: f64,
    pub codec: String,
}

#[derive(Debug, Clone)]
pub struct AudioInfo {
    pub duration: f64,
    pub sample_rate: u32,
    pub channels: u32,
    pub codec: String,
}

/// Parse fps fraction string like "30/1" or "24000/1001"
fn parse_fps(fps_str: &str) -> f64 {
    if let Some((num, den)) = fps_str.split_once('/') {
        let n: f64 = num.parse().unwrap_or(30.0);
        let d: f64 = den.parse().unwrap_or(1.0);
        if d > 0.0 { n / d } else { 30.0 }
    } else {
        fps_str.parse().unwrap_or(30.0)
    }
}

/// Metadata probe using ffprobe
pub fn probe_video(path: &Path) -> Result<VideoInfo> {
    let probe = ffmpeg::probe_file(path)?;

    let video_stream = probe
        .streams
        .as_ref()
        .and_then(|s| s.iter().find(|stream| stream.codec_type == "video"));

    let width = video_stream.and_then(|s| s.width).unwrap_or(1920);
    let height = video_stream.and_then(|s| s.height).unwrap_or(1080);
    let fps = video_stream
        .and_then(|s| s.r_frame_rate.as_deref())
        .map(parse_fps)
        .unwrap_or(30.0);
    let codec = video_stream
        .and_then(|s| s.codec_name.clone())
        .unwrap_or_else(|| "unknown".into());

    // Duration: prefer stream duration, fallback to format duration
    let duration = video_stream
        .and_then(|s| s.duration.as_deref())
        .and_then(|d| d.parse::<f64>().ok())
        .or_else(|| {
            probe
                .format
                .as_ref()
                .and_then(|f| f.duration.as_deref())
                .and_then(|d| d.parse::<f64>().ok())
        })
        .unwrap_or(5.0);

    Ok(VideoInfo {
        width,
        height,
        duration,
        fps,
        codec,
    })
}

/// Metadata probe for audio files
pub fn probe_audio(path: &Path) -> Result<AudioInfo> {
    let probe = ffmpeg::probe_file(path)?;

    let audio_stream = probe
        .streams
        .as_ref()
        .and_then(|s| s.iter().find(|stream| stream.codec_type == "audio"));

    let codec = audio_stream
        .and_then(|s| s.codec_name.clone())
        .unwrap_or_else(|| "unknown".into());

    let duration = audio_stream
        .and_then(|s| s.duration.as_deref())
        .and_then(|d| d.parse::<f64>().ok())
        .or_else(|| {
            probe
                .format
                .as_ref()
                .and_then(|f| f.duration.as_deref())
                .and_then(|d| d.parse::<f64>().ok())
        })
        .unwrap_or(5.0);

    Ok(AudioInfo {
        duration,
        sample_rate: 44100,
        channels: 2,
        codec,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_fps() {
        assert_eq!(parse_fps("30/1"), 30.0);
        assert_eq!(parse_fps("60/1"), 60.0);
        let fps24 = parse_fps("24000/1001");
        assert!((fps24 - 23.976).abs() < 0.001);
        assert_eq!(parse_fps("25"), 25.0);
        assert_eq!(parse_fps("invalid"), 30.0);
    }
}

