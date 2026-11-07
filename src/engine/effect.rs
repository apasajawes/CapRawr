use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EffectKind {
    // Video effects (mapped from CapCut's 195 actions)
    Brightness(f64),
    Contrast(f64),
    Saturation(f64),
    Temperature(f64),
    Sharpen(f64),
    Blur(f64),
    FadeIn(f64),   // duration in seconds
    FadeOut(f64),
    SpeedChange(f64),
    Crop { x: f64, y: f64, w: f64, h: f64 },
    Rotate(f64),    // degrees
    FlipH,
    FlipV,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Effect {
    pub id: uuid::Uuid,
    pub kind: EffectKind,
    pub enabled: bool,
}

impl Effect {
    pub fn new(kind: EffectKind) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            kind,
            enabled: true,
        }
    }
}
