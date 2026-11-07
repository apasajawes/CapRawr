use egui::{self, Color32, RichText};
use crate::engine::project::Project;

pub fn show(ctx: &egui::Context, project: &mut Project) {
    egui::SidePanel::right("properties")
        .min_width(200.0)
        .default_width(250.0)
        .show(ctx, |ui| {
            ui.heading("Properties");
            ui.separator();

            // Project settings
            egui::CollapsingHeader::new("Project").default_open(true).show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Name:");
                    ui.text_edit_singleline(&mut project.name);
                });
                ui.horizontal(|ui| {
                    ui.label("Width:");
                    let mut w = project.width as i32;
                    if ui.add(egui::DragValue::new(&mut w).range(1..=7680)).changed() {
                        project.width = w as u32;
                    }
                });
                ui.horizontal(|ui| {
                    ui.label("Height:");
                    let mut h = project.height as i32;
                    if ui.add(egui::DragValue::new(&mut h).range(1..=4320)).changed() {
                        project.height = h as u32;
                    }
                });
                ui.horizontal(|ui| {
                    ui.label("FPS:");
                    ui.add(egui::DragValue::new(&mut project.fps).range(1.0..=240.0));
                });

                // Presets
                ui.label(RichText::new("Presets").small().color(Color32::GRAY));
                ui.horizontal_wrapped(|ui| {
                    if ui.small_button("1080p").clicked() { project.width = 1920; project.height = 1080; }
                    if ui.small_button("4K").clicked() { project.width = 3840; project.height = 2160; }
                    if ui.small_button("720p").clicked() { project.width = 1280; project.height = 720; }
                    if ui.small_button("9:16").clicked() { project.width = 1080; project.height = 1920; }
                    if ui.small_button("1:1").clicked() { project.width = 1080; project.height = 1080; }
                });
            });

            ui.separator();

            // Selected clip properties
            if let Some(clip_id) = project.selected_clip {
                let clip = project.timeline.tracks.iter_mut()
                    .flat_map(|t| t.clips.iter_mut())
                    .find(|c| c.id == clip_id);

                if let Some(clip) = clip {
                    egui::CollapsingHeader::new("Clip").default_open(true).show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.label("Name:");
                            ui.text_edit_singleline(&mut clip.name);
                        });
                        ui.horizontal(|ui| {
                            ui.label("Position:");
                            ui.add(egui::DragValue::new(&mut clip.position).speed(0.1).suffix("s"));
                        });
                        ui.horizontal(|ui| {
                            ui.label("Duration:");
                            ui.add(egui::DragValue::new(&mut clip.duration).speed(0.1).suffix("s").range(0.01..=f64::MAX));
                        });
                        ui.horizontal(|ui| {
                            ui.label("Speed:");
                            ui.add(egui::DragValue::new(&mut clip.speed).speed(0.05).range(0.1..=10.0).suffix("x"));
                        });
                        ui.horizontal(|ui| {
                            ui.label("Volume:");
                            ui.add(egui::Slider::new(&mut clip.volume, 0.0..=2.0));
                        });
                        ui.horizontal(|ui| {
                            ui.label("Opacity:");
                            ui.add(egui::Slider::new(&mut clip.opacity, 0.0..=1.0));
                        });
                        ui.label(
                            RichText::new(format!("Source: {}", clip.source.display()))
                                .small()
                                .color(Color32::GRAY),
                        );
                    });
                }
            } else {
                ui.label(RichText::new("No clip selected").color(Color32::GRAY));
            }
        });
}
