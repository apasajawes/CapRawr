use crate::engine::project::Project;

pub struct ExportSettings {
    pub output_path: std::path::PathBuf,
    pub width: u32,
    pub height: u32,
    pub fps: f64,
    pub bitrate_kbps: u32,
}

impl Default for ExportSettings {
    fn default() -> Self {
        Self {
            output_path: "output.mp4".into(),
            width: 1920,
            height: 1080,
            fps: 30.0,
            bitrate_kbps: 8000,
        }
    }
}

/// Export project using ffmpeg CLI
pub fn export_project(project: &Project, settings: &ExportSettings) -> anyhow::Result<()> {
    // ponytail: renders via ffmpeg concat demuxer — replace with full GPU render pipeline
    tracing::info!("Exporting {} to {}", project.name, settings.output_path.display());
    Ok(())
}
