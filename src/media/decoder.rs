use std::path::Path;

#[derive(Debug, Clone)]
pub struct VideoInfo {
    pub width: u32,
    pub height: u32,
    pub duration: f64,
    pub fps: f64,
    pub codec: String,
}

/// Metadata probe using ffprobe if available
pub fn probe_video(_path: &Path) -> anyhow::Result<VideoInfo> {
    // ponytail: fallback values — replace with real ffprobe execution
    Ok(VideoInfo {
        width: 1920,
        height: 1080,
        duration: 10.0,
        fps: 30.0,
        codec: "h264".into(),
    })
}
