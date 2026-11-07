use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Clip {
    pub id: uuid::Uuid,
    /// Source file path
    pub source: PathBuf,
    /// Name displayed in timeline
    pub name: String,
    /// Position on timeline (seconds)
    pub position: f64,
    /// Duration on timeline (seconds)
    pub duration: f64,
    /// Start offset within source (seconds) — for trimming
    pub source_start: f64,
    /// Speed multiplier (1.0 = normal)
    pub speed: f64,
    /// Volume (0.0 - 1.0) for audio
    pub volume: f64,
    /// Opacity (0.0 - 1.0) for video
    pub opacity: f64,
}

impl Clip {
    pub fn new(source: PathBuf, duration: f64) -> Self {
        let name = source
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_else(|| "Clip".into());
        Self {
            id: uuid::Uuid::new_v4(),
            source,
            name,
            position: 0.0,
            duration,
            source_start: 0.0,
            speed: 1.0,
            volume: 1.0,
            opacity: 1.0,
        }
    }

    pub fn end(&self) -> f64 {
        self.position + self.duration
    }

    /// Split clip at given time (relative to timeline).
    /// Returns the new right-side clip if split point is valid.
    pub fn split_at(&mut self, time: f64) -> Option<Clip> {
        if time <= self.position || time >= self.end() {
            return None;
        }
        let split_offset = time - self.position;
        let right = Clip {
            id: uuid::Uuid::new_v4(),
            source: self.source.clone(),
            name: format!("{} (split)", self.name),
            position: time,
            duration: self.duration - split_offset,
            source_start: self.source_start + split_offset / self.speed,
            speed: self.speed,
            volume: self.volume,
            opacity: self.opacity,
        };
        self.duration = split_offset;
        Some(right)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_produces_correct_durations() {
        let mut clip = Clip::new(PathBuf::from("test.mp4"), 10.0);
        clip.position = 5.0;
        let right = clip.split_at(8.0).unwrap();
        assert!((clip.duration - 3.0).abs() < 1e-9);
        assert!((right.position - 8.0).abs() < 1e-9);
        assert!((right.duration - 7.0).abs() < 1e-9);
    }
}
