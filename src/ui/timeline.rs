use egui::{self, Color32, Rect, Sense, Stroke, Vec2, pos2};
use crate::engine::project::Project;
use crate::engine::track::TrackKind;

const TRACK_HEIGHT: f32 = 40.0;
const HEADER_WIDTH: f32 = 150.0;
const RULER_HEIGHT: f32 = 24.0;

pub fn show(ctx: &egui::Context, project: &mut Project) {
    egui::TopBottomPanel::bottom("timeline")
        .resizable(true)
        .min_height(150.0)
        .default_height(250.0)
        .show(ctx, |ui| {
            ui.heading("Timeline");
            ui.separator();

            let zoom = project.timeline_zoom as f32;

            // Zoom slider
            ui.horizontal(|ui| {
                ui.label("Zoom:");
                let mut z = project.timeline_zoom;
                if ui.add(egui::Slider::new(&mut z, 20.0..=500.0).logarithmic(true)).changed() {
                    project.timeline_zoom = z;
                }
            });

            egui::ScrollArea::both()
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    let total_duration = project.duration().max(30.0) as f32;
                    let content_width = HEADER_WIDTH + total_duration * zoom;
                    let track_count = project.timeline.tracks.len();
                    let content_height = RULER_HEIGHT + track_count as f32 * TRACK_HEIGHT;

                    let (response, painter) = ui.allocate_painter(
                        Vec2::new(content_width, content_height.max(150.0)),
                        Sense::click_and_drag(),
                    );
                    let origin = response.rect.min;

                    // Background
                    painter.rect_filled(response.rect, 0.0, Color32::from_gray(30));

                    // --- Time ruler ---
                    let ruler_rect = Rect::from_min_size(
                        pos2(origin.x + HEADER_WIDTH, origin.y),
                        Vec2::new(total_duration * zoom, RULER_HEIGHT),
                    );
                    painter.rect_filled(ruler_rect, 0.0, Color32::from_gray(40));

                    // Tick marks every second
                    let step = if zoom < 40.0 { 5.0 } else if zoom < 100.0 { 1.0 } else { 0.5 };
                    let mut t = 0.0_f32;
                    while t <= total_duration {
                        let x = origin.x + HEADER_WIDTH + t * zoom;
                        let is_major = (t % 5.0).abs() < 0.01;
                        let h = if is_major { RULER_HEIGHT } else { RULER_HEIGHT * 0.5 };
                        painter.line_segment(
                            [pos2(x, origin.y + RULER_HEIGHT - h), pos2(x, origin.y + RULER_HEIGHT)],
                            Stroke::new(1.0_f32, Color32::GRAY),
                        );
                        if is_major {
                            let secs = t as u32;
                            let label = format!("{}:{:02}", secs / 60, secs % 60);
                            painter.text(
                                pos2(x + 2.0, origin.y + 2.0),
                                egui::Align2::LEFT_TOP,
                                label,
                                egui::FontId::monospace(10.0),
                                Color32::LIGHT_GRAY,
                            );
                        }
                        t += step;
                    }

                    // --- Tracks ---
                    for (i, track) in project.timeline.tracks.iter().enumerate() {
                        let y = origin.y + RULER_HEIGHT + i as f32 * TRACK_HEIGHT;

                        // Track header
                        let header_rect = Rect::from_min_size(
                            pos2(origin.x, y),
                            Vec2::new(HEADER_WIDTH, TRACK_HEIGHT),
                        );
                        let color = match track.kind {
                            TrackKind::Video => Color32::from_rgb(40, 40, 55),
                            TrackKind::Audio => Color32::from_rgb(40, 55, 40),
                            TrackKind::Text => Color32::from_rgb(55, 40, 40),
                        };
                        painter.rect_filled(header_rect, 0.0, color);
                        painter.text(
                            pos2(origin.x + 8.0, y + TRACK_HEIGHT / 2.0),
                            egui::Align2::LEFT_CENTER,
                            &track.name,
                            egui::FontId::proportional(12.0),
                            Color32::WHITE,
                        );

                        // Track lane background
                        let lane_rect = Rect::from_min_size(
                            pos2(origin.x + HEADER_WIDTH, y),
                            Vec2::new(total_duration * zoom, TRACK_HEIGHT),
                        );
                        let lane_bg = if i % 2 == 0 { Color32::from_gray(25) } else { Color32::from_gray(28) };
                        painter.rect_filled(lane_rect, 0.0, lane_bg);

                        // Clips
                        for clip in &track.clips {
                            let cx = origin.x + HEADER_WIDTH + clip.position as f32 * zoom;
                            let cw = clip.duration as f32 * zoom;
                            let clip_rect = Rect::from_min_size(
                                pos2(cx, y + 2.0),
                                Vec2::new(cw.max(2.0), TRACK_HEIGHT - 4.0),
                            );
                            let clip_color = match track.kind {
                                TrackKind::Video => Color32::from_rgb(80, 120, 200),
                                TrackKind::Audio => Color32::from_rgb(80, 200, 120),
                                TrackKind::Text => Color32::from_rgb(200, 120, 80),
                            };
                            let selected = project.selected_clip == Some(clip.id);
                            painter.rect_filled(clip_rect, 3.0, clip_color);
                            if selected {
                                painter.rect_stroke(
                                    clip_rect,
                                    3.0,
                                    Stroke::new(2.0_f32, Color32::WHITE),
                                    egui::StrokeKind::Outside,
                                );
                            }
                            // Clip name
                            if cw > 30.0 {
                                let text_rect = clip_rect.shrink(4.0);
                                painter.text(
                                    text_rect.min,
                                    egui::Align2::LEFT_TOP,
                                    &clip.name,
                                    egui::FontId::proportional(10.0),
                                    Color32::WHITE,
                                );
                            }
                        }
                    }

                    // --- Playhead ---
                    let ph_x = origin.x + HEADER_WIDTH + project.playhead as f32 * zoom;
                    painter.line_segment(
                        [pos2(ph_x, origin.y), pos2(ph_x, origin.y + content_height)],
                        Stroke::new(2.0_f32, Color32::RED),
                    );
                    // Playhead handle
                    painter.circle_filled(pos2(ph_x, origin.y + RULER_HEIGHT), 5.0, Color32::RED);

                    // Click on ruler to move playhead
                    if response.clicked() {
                        if let Some(pos) = response.interact_pointer_pos() {
                            if pos.y < origin.y + RULER_HEIGHT {
                                let t = ((pos.x - origin.x - HEADER_WIDTH) / zoom).max(0.0);
                                project.playhead = t as f64;
                            }
                        }
                    }

                    // Click on clip to select
                    if response.clicked() {
                        if let Some(pos) = response.interact_pointer_pos() {
                            let track_idx = ((pos.y - origin.y - RULER_HEIGHT) / TRACK_HEIGHT) as usize;
                            if track_idx < project.timeline.tracks.len() {
                                let time = ((pos.x - origin.x - HEADER_WIDTH) / zoom) as f64;
                                project.selected_clip = project.timeline.tracks[track_idx]
                                    .clips
                                    .iter()
                                    .find(|c| time >= c.position && time <= c.end())
                                    .map(|c| c.id);
                                project.selected_track = Some(track_idx);
                            }
                        }
                    }
                });
        });
}
