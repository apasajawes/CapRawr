use serde::{Deserialize, Serialize};
use crate::engine::clip::Clip;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TrackKind {
    Video,
    Audio,
    Text,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Track {
    pub name: String,
    pub kind: TrackKind,
    pub clips: Vec<Clip>,
    pub muted: bool,
    pub locked: bool,
    pub visible: bool,
}

impl Track {
    pub fn new(name: String, kind: TrackKind) -> Self {
        Self {
            name,
            kind,
            clips: Vec::new(),
            muted: false,
            locked: false,
            visible: true,
        }
    }

    pub fn duration(&self) -> f64 {
        self.clips
            .iter()
            .map(|c| c.position + c.duration)
            .fold(0.0_f64, f64::max)
    }

    pub fn add_clip(&mut self, clip: Clip) {
        self.clips.push(clip);
        self.clips.sort_by(|a, b| a.position.partial_cmp(&b.position).unwrap());
    }
}
