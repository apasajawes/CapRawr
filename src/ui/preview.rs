use egui::{self, Color32, RichText};
use crate::engine::project::Project;

pub fn show(ctx: &egui::Context, project: &Project, texture: Option<&egui::TextureHandle>) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.vertical_centered(|ui| {
            // Preview area
            let available = ui.available_size();
            let aspect = project.width as f32 / project.height as f32;
            let preview_w = available.x.min(available.y * aspect).min(800.0).max(100.0);
            let preview_h = preview_w / aspect;

            let (rect, _response) = ui.allocate_exact_size(
                egui::Vec2::new(preview_w, preview_h),
                egui::Sense::hover(),
            );

            // Black preview background
            ui.painter().rect_filled(rect, 4.0, Color32::BLACK);

            if let Some(tex) = texture {
                // Render decoded video frame
                ui.painter().image(
                    tex.id(),
                    rect,
                    egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
                    Color32::WHITE,
                );
            } else {
                // Placeholder text
                ui.painter().text(
                    rect.center(),
                    egui::Align2::CENTER_CENTER,
                    "No Media / Blank Frame",
                    egui::FontId::proportional(20.0),
                    Color32::from_gray(80),
                );
            }

            ui.add_space(8.0);
            ui.label(
                RichText::new(format!(
                    "{}x{} • {:.1}fps • {:.1}s",
                    project.width, project.height, project.fps, project.duration()
                ))
                .small()
                .color(Color32::GRAY),
            );
        });
    });
}
