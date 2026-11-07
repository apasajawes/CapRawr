use eframe::egui;
use std::time::Instant;
use crate::engine::project::Project;
use crate::render::compositor::Compositor;
use crate::ui;

pub struct CapRawrApp {
    project: Project,
    compositor: Compositor,
    preview_texture: Option<egui::TextureHandle>,
    last_rendered_time: f64,
    last_tick: Instant,
    show_media_browser: bool,
}

impl CapRawrApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            project: Project::default(),
            compositor: Compositor::new(),
            preview_texture: None,
            last_rendered_time: -1.0,
            last_tick: Instant::now(),
            show_media_browser: false,
        }
    }
}

impl eframe::App for CapRawrApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let now = Instant::now();
        let dt = now.duration_since(self.last_tick).as_secs_f64();
        self.last_tick = now;

        // Playback handling
        if self.project.playing {
            self.project.playhead += dt;
            let max_d = self.project.duration();
            if max_d > 0.0 && self.project.playhead >= max_d {
                self.project.playhead = 0.0;
            }
            // Request continuous repainting during playback
            ctx.request_repaint();
        }

        // Update preview frame if playhead moved significantly (> 33ms / 1 frame)
        if (self.project.playhead - self.last_rendered_time).abs() > 0.033 {
            let width = self.project.width;
            let height = self.project.height;
            let raw_rgba = self.compositor.render_frame(&self.project, self.project.playhead);

            let color_image = egui::ColorImage::from_rgba_unmultiplied(
                [width as usize, height as usize],
                &raw_rgba,
            );

            match &mut self.preview_texture {
                Some(tex) => tex.set(color_image, egui::TextureOptions::LINEAR),
                None => {
                    self.preview_texture = Some(ctx.load_texture(
                        "preview_frame",
                        color_image,
                        egui::TextureOptions::LINEAR,
                    ));
                }
            }
            self.last_rendered_time = self.project.playhead;
        }

        ui::toolbar::show(ctx, &mut self.project, &mut self.show_media_browser);
        ui::preview::show(ctx, &self.project, self.preview_texture.as_ref());
        ui::properties::show(ctx, &mut self.project);
        ui::timeline::show(ctx, &mut self.project);

        if self.show_media_browser {
            ui::media_browser::show(ctx, &mut self.project, &mut self.show_media_browser);
        }
    }
}
