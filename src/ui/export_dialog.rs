use std::path::PathBuf;
use std::sync::mpsc::{Receiver, channel};
use egui::{self, Color32, RichText};
use crate::engine::project::Project;
use crate::media::encoder::{ExportSettings, export_project};

#[derive(Debug, Clone, PartialEq)]
pub enum ExportStatus {
    Idle,
    Exporting,
    Success(PathBuf),
    Failed(String),
}

pub struct ExportState {
    pub open: bool,
    pub settings: ExportSettings,
    pub status: ExportStatus,
    rx: Option<Receiver<Result<PathBuf, String>>>,
    resolution_preset: usize,
    fps_preset: usize,
    bitrate_preset: usize,
}

impl Default for ExportState {
    fn default() -> Self {
        Self {
            open: false,
            settings: ExportSettings::default(),
            status: ExportStatus::Idle,
            rx: None,
            resolution_preset: 0, // 1080p
            fps_preset: 1,        // 30 fps
            bitrate_preset: 1,    // 8 Mbps
        }
    }
}

pub fn show(ctx: &egui::Context, project: &Project, state: &mut ExportState) {
    // Check background worker channel
    if let Some(rx) = &state.rx {
        if let Ok(res) = rx.try_recv() {
            match res {
                Ok(path) => state.status = ExportStatus::Success(path),
                Err(err) => state.status = ExportStatus::Failed(err),
            }
            state.rx = None;
        }
    }

    if !state.open {
        return;
    }

    let mut open = state.open;
    let mut close_clicked = false;
    egui::Window::new("Export Video")
        .open(&mut open)
        .default_width(450.0)
        .resizable(false)
        .show(ctx, |ui| {
            ui.heading("Render & Export Settings");
            ui.separator();

            // Destination path
            ui.horizontal(|ui| {
                ui.label("Output File:");
                let path_display = state.settings.output_path.display().to_string();
                ui.monospace(path_display);
                if ui.button("Browse...").clicked() {
                    if let Some(path) = rfd::FileDialog::new()
                        .set_file_name("caprawr_export.mp4")
                        .add_filter("MP4 Video", &["mp4"])
                        .save_file()
                    {
                        state.settings.output_path = path;
                    }
                }
            });

            ui.add_space(8.0);

            // Resolution Presets
            ui.horizontal(|ui| {
                ui.label("Resolution:");
                let presets = [
                    ("1080p Full HD (1920x1080)", 1920, 1080),
                    ("4K UHD (3840x2160)", 3840, 2160),
                    ("720p HD (1280x720)", 1280, 720),
                    ("9:16 Shorts/Reels (1080x1920)", 1080, 1920),
                    ("1:1 Square (1080x1080)", 1080, 1080),
                ];
                egui::ComboBox::from_id_salt("res_preset")
                    .selected_text(presets[state.resolution_preset].0)
                    .show_ui(ui, |ui| {
                        for (idx, &(label, w, h)) in presets.iter().enumerate() {
                            if ui.selectable_value(&mut state.resolution_preset, idx, label).clicked() {
                                state.settings.width = w;
                                state.settings.height = h;
                            }
                        }
                    });
            });

            // Frame Rate
            ui.horizontal(|ui| {
                ui.label("Frame Rate:");
                let fps_options = [24.0, 30.0, 60.0];
                let fps_labels = ["24 FPS (Cinematic)", "30 FPS (Standard)", "60 FPS (Smooth)"];
                egui::ComboBox::from_id_salt("fps_preset")
                    .selected_text(fps_labels[state.fps_preset])
                    .show_ui(ui, |ui| {
                        for (idx, &label) in fps_labels.iter().enumerate() {
                            if ui.selectable_value(&mut state.fps_preset, idx, label).clicked() {
                                state.settings.fps = fps_options[idx];
                            }
                        }
                    });
            });

            // Bitrate
            ui.horizontal(|ui| {
                ui.label("Bitrate:");
                let bitrates = [4000, 8000, 16000, 25000];
                let bitrate_labels = [
                    "4 Mbps (Web / Small)",
                    "8 Mbps (Standard 1080p)",
                    "16 Mbps (High Quality)",
                    "25 Mbps (4K / Archival)",
                ];
                egui::ComboBox::from_id_salt("bitrate_preset")
                    .selected_text(bitrate_labels[state.bitrate_preset])
                    .show_ui(ui, |ui| {
                        for (idx, &label) in bitrate_labels.iter().enumerate() {
                            if ui.selectable_value(&mut state.bitrate_preset, idx, label).clicked() {
                                state.settings.bitrate_kbps = bitrates[idx];
                            }
                        }
                    });
            });

            ui.separator();

            // Status display
            match &state.status {
                ExportStatus::Idle => {
                    ui.label("Ready to export.");
                }
                ExportStatus::Exporting => {
                    ui.horizontal(|ui| {
                        ui.spinner();
                        ui.label(RichText::new("Exporting video... please wait").color(Color32::YELLOW));
                    });
                    ctx.request_repaint(); // keep spinner animating
                }
                ExportStatus::Success(path) => {
                    ui.label(
                        RichText::new(format!("Export successful!\n{}", path.display()))
                            .color(Color32::GREEN)
                            .strong(),
                    );
                }
                ExportStatus::Failed(err) => {
                    ui.label(
                        RichText::new(format!("Export failed: {}", err))
                            .color(Color32::RED)
                            .small(),
                    );
                }
            }

            ui.add_space(10.0);

            // Action Buttons
            ui.horizontal(|ui| {
                let is_exporting = matches!(state.status, ExportStatus::Exporting);

                if ui
                    .add_enabled(!is_exporting, egui::Button::new("🚀 Start Export"))
                    .clicked()
                {
                    state.status = ExportStatus::Exporting;
                    let (tx, rx) = channel();
                    state.rx = Some(rx);

                    let project_clone = project.clone();
                    let settings_clone = state.settings.clone();

                    std::thread::spawn(move || {
                        let res = export_project(&project_clone, &settings_clone)
                            .map(|_| settings_clone.output_path)
                            .map_err(|e| e.to_string());
                        let _ = tx.send(res);
                    });
                }

                if ui.button("Close").clicked() {
                    close_clicked = true;
                }
            });
        });

    if close_clicked {
        open = false;
    }
    state.open = open;
}
