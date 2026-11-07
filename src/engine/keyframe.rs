use serde::{Deserialize, Serialize};

/// Keyframe interpolation mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Easing {
    Linear,
    EaseIn,
    EaseOut,
    EaseInOut,
    Hold,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Keyframe {
    /// Time in seconds (relative to clip start)
    pub time: f64,
    pub value: f64,
    pub easing: Easing,
}

/// A property track that can be animated with keyframes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyframeTrack {
    pub property: String,
    pub keyframes: Vec<Keyframe>,
}

impl KeyframeTrack {
    pub fn value_at(&self, time: f64) -> f64 {
        if self.keyframes.is_empty() {
            return 0.0;
        }
        if self.keyframes.len() == 1 || time <= self.keyframes[0].time {
            return self.keyframes[0].value;
        }
        let last = self.keyframes.last().unwrap();
        if time >= last.time {
            return last.value;
        }
        // Find surrounding keyframes
        let idx = self.keyframes.partition_point(|k| k.time <= time);
        let a = &self.keyframes[idx - 1];
        let b = &self.keyframes[idx];
        let t = (time - a.time) / (b.time - a.time);
        let t = match a.easing {
            Easing::Linear => t,
            Easing::EaseIn => t * t,
            Easing::EaseOut => t * (2.0 - t),
            Easing::EaseInOut => {
                if t < 0.5 { 2.0 * t * t } else { -1.0 + (4.0 - 2.0 * t) * t }
            }
            Easing::Hold => 0.0,
        };
        a.value + (b.value - a.value) * t
    }
}
