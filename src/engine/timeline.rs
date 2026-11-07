use serde::{Deserialize, Serialize};
use crate::engine::track::{Track, TrackKind};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Timeline {
    pub tracks: Vec<Track>,
}

impl Timeline {
    pub fn duration(&self) -> f64 {
        self.tracks
            .iter()
            .map(|t| t.duration())
            .fold(0.0_f64, f64::max)
    }

    pub fn add_video_track(&mut self) -> usize {
        let idx = self.tracks.len();
        self.tracks.push(Track::new(
            format!("Video {}", self.count_kind(TrackKind::Video) + 1),
            TrackKind::Video,
        ));
        idx
    }

    pub fn add_audio_track(&mut self) -> usize {
        let idx = self.tracks.len();
        self.tracks.push(Track::new(
            format!("Audio {}", self.count_kind(TrackKind::Audio) + 1),
            TrackKind::Audio,
        ));
        idx
    }

    fn count_kind(&self, kind: TrackKind) -> usize {
        self.tracks.iter().filter(|t| t.kind == kind).count()
    }
}
