use crate::engine::clip::Clip;
use crate::engine::project::Project;
use crate::engine::track::TrackKind;
use crate::media::decoder::{probe_audio, probe_video};
use egui::{self, Color32, RichText};

pub fn show(ctx: &egui::Context, project: &mut Project, open: &mut bool) {
    egui::Window::new("Import Media")
        .open(open)
        .default_size([400.0, 300.0])
        .show(ctx, |ui| {
            ui.label("Drag & drop media files onto the window, or click Import below.");
            ui.separator();

            if ui.button("Import Video/Image Files...").clicked() {
                if let Some(paths) = rfd::FileDialog::new()
                    .add_filter(
                        "Media Files",
                        &[
                            "mp4", "mkv", "avi", "mov", "webm", "gif", "png", "jpg", "jpeg",
                            "bmp", "webp",
                        ],
                    )
                    .pick_files()
                {
                    for path in paths {
                        // Read real duration via ffprobe, fallback to 5.0s for static images
                        let duration = probe_video(&path)
                            .map(|info| info.duration)
                            .unwrap_or(5.0);

                        let clip = Clip::new(path, duration);
                        if project.timeline.tracks.is_empty() {
                            project.timeline.add_video_track();
                        }
                        // Add to first video track
                        if let Some(track) = project
                            .timeline
                            .tracks
                            .iter_mut()
                            .find(|t| t.kind == TrackKind::Video)
                        {
                            let pos = track.duration();
                            let mut c = clip;
                            c.position = pos;
                            track.add_clip(c);
                        }
                    }
                }
            }

            if ui.button("Import Audio Files...").clicked() {
                if let Some(paths) = rfd::FileDialog::new()
                    .add_filter("Audio Files", &["mp3", "wav", "aac", "ogg", "flac", "m4a"])
                    .pick_files()
                {
                    for path in paths {
                        let duration = probe_audio(&path)
                            .map(|info| info.duration)
                            .unwrap_or(5.0);

                        let clip = Clip::new(path, duration);
                        if project
                            .timeline
                            .tracks
                            .iter()
                            .all(|t| t.kind != TrackKind::Audio)
                        {
                            project.timeline.add_audio_track();
                        }
                        if let Some(track) = project
                            .timeline
                            .tracks
                            .iter_mut()
                            .find(|t| t.kind == TrackKind::Audio)
                        {
                            let pos = track.duration();
                            let mut c = clip;
                            c.position = pos;
                            track.add_clip(c);
                        }
                    }
                }
            }

            ui.separator();
            ui.label(
                RichText::new("Supported: MP4, MKV, AVI, MOV, WebM, GIF, PNG, JPG, MP3, WAV, AAC, OGG, FLAC")
                    .small()
                    .color(Color32::GRAY),
            );
        });
}
