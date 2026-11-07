use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Mutex;
use crate::engine::project::Project;
use crate::engine::track::TrackKind;
use crate::media::ffmpeg::extract_frame;

pub struct Compositor {
    // Cache: (PathBuf, quantized_time_100ms) -> cached RGBA frame
    frame_cache: Mutex<HashMap<(PathBuf, u64), Vec<u8>>>,
}

impl Compositor {
    pub fn new() -> Self {
        Self {
            frame_cache: Mutex::new(HashMap::new()),
        }
    }

    /// Render a single frame at the given timestamp (seconds)
    pub fn render_frame(&self, project: &Project, time: f64) -> Vec<u8> {
        let width = project.width;
        let height = project.height;

        // Find active video clip at `time` (reverse iterate so higher tracks overlay lower ones)
        for track in project.timeline.tracks.iter().rev() {
            if track.kind != TrackKind::Video || !track.visible {
                continue;
            }

            for clip in &track.clips {
                if time >= clip.position && time <= clip.end() {
                    let clip_local_time = (time - clip.position) * clip.speed + clip.source_start;

                    // Quantize to ~10fps for caching to avoid hammering ffmpeg
                    let quant_key = (clip_local_time * 10.0).round() as u64;
                    let cache_key = (clip.source.clone(), quant_key);

                    if let Ok(cache) = self.frame_cache.lock() {
                        if let Some(cached) = cache.get(&cache_key) {
                            return cached.clone();
                        }
                    }

                    // Try to extract frame using ffmpeg
                    if let Ok(img) = extract_frame(&clip.source, clip_local_time, width, height) {
                        let bytes = img.into_raw();
                        if let Ok(mut cache) = self.frame_cache.lock() {
                            // Bound cache size
                            if cache.len() > 100 {
                                cache.clear();
                            }
                            cache.insert(cache_key, bytes.clone());
                        }
                        return bytes;
                    }
                }
            }
        }

        // Default: blank dark gray frame
        vec![30u8; (width * height * 4) as usize]
    }
}
