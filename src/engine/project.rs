use serde::{Deserialize, Serialize};
use crate::engine::timeline::Timeline;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub fps: f64,
    pub timeline: Timeline,
    /// Currently selected track index
    #[serde(skip)]
    pub selected_track: Option<usize>,
    /// Currently selected clip ID
    #[serde(skip)]
    pub selected_clip: Option<uuid::Uuid>,
    /// Playhead position in seconds
    #[serde(skip)]
    pub playhead: f64,
    #[serde(skip)]
    pub playing: bool,
    /// Zoom level for timeline (pixels per second)
    #[serde(skip)]
    pub timeline_zoom: f64,
}

impl Default for Project {
    fn default() -> Self {
        Self {
            name: "Untitled".into(),
            width: 1920,
            height: 1080,
            fps: 30.0,
            timeline: Timeline::default(),
            selected_track: None,
            selected_clip: None,
            playhead: 0.0,
            playing: false,
            timeline_zoom: 100.0, // 100px per second
        }
    }
}

impl Project {
    pub fn duration(&self) -> f64 {
        self.timeline.duration()
    }

    pub fn save(&self, path: &std::path::Path) -> anyhow::Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json)?;
        Ok(())
    }

    pub fn load(path: &std::path::Path) -> anyhow::Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let project = serde_json::from_str(&json)?;
        Ok(project)
    }
}
