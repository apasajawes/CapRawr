use crate::engine::project::Project;

pub struct Compositor;

impl Compositor {
    pub fn new() -> Self {
        Self
    }

    /// Render a single frame at the given timestamp (seconds)
    pub fn render_frame(&self, _project: &Project, _time: f64) -> Vec<u8> {
        // ponytail: returns black frame — wire up wgpu compositor later
        vec![0u8; 1920 * 1080 * 4]
    }
}
