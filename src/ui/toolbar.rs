use egui::{self, Color32, RichText};
use crate::engine::project::Project;

pub fn show(ctx: &egui::Context, project: &mut Project, show_media_browser: &mut bool) {
    egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
        egui::menu::bar(ui, |ui| {
            ui.menu_button("File", |ui| {
                if ui.button("New Project").clicked() {
                    *project = Project::default();
                    ui.close_menu();
                }
                if ui.button("Open Project...").clicked() {
                    if let Some(path) = rfd::FileDialog::new()
                        .add_filter("CapRawr Project", &["caprawr"])
                        .pick_file()
                    {
                        if let Ok(p) = Project::load(&path) {
                            *project = p;
                        }
                    }
                    ui.close_menu();
                }
                if ui.button("Save Project...").clicked() {
                    if let Some(path) = rfd::FileDialog::new()
                        .add_filter("CapRawr Project", &["caprawr"])
                        .save_file()
                    {
                        let _ = project.save(&path);
                    }
                    ui.close_menu();
                }
                ui.separator();
                if ui.button("Exit").clicked() {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }
            });

            ui.menu_button("Edit", |ui| {
                if ui.button("Undo").clicked() { ui.close_menu(); }
                if ui.button("Redo").clicked() { ui.close_menu(); }
            });

            ui.separator();

            // Transport controls
            if ui.button("⏮").clicked() {
                project.playhead = 0.0;
            }
            let play_text = if project.playing { "⏸" } else { "▶" };
            if ui.button(play_text).clicked() {
                project.playing = !project.playing;
            }
            if ui.button("⏭").clicked() {
                project.playhead = project.duration();
            }

            ui.separator();

            // Timecode display
            let mins = (project.playhead / 60.0) as u32;
            let secs = (project.playhead % 60.0) as u32;
            let frames = ((project.playhead % 1.0) * project.fps) as u32;
            ui.label(
                RichText::new(format!("{:02}:{:02}:{:02}", mins, secs, frames))
                    .monospace()
                    .color(Color32::WHITE),
            );

            ui.separator();

            if ui.button("📁 Import Media").clicked() {
                *show_media_browser = true;
            }

            // Add track buttons
            ui.separator();
            if ui.button("+ Video Track").clicked() {
                project.timeline.add_video_track();
            }
            if ui.button("+ Audio Track").clicked() {
                project.timeline.add_audio_track();
            }

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label(
                    RichText::new(format!("{}x{} @ {}fps", project.width, project.height, project.fps))
                        .small()
                        .color(Color32::GRAY),
                );
                ui.label(
                    RichText::new("CapRawr")
                        .strong()
                        .color(Color32::from_rgb(255, 100, 50)),
                );
            });
        });
    });
}
