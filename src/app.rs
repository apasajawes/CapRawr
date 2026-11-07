use eframe::egui;
use crate::engine::project::Project;
use crate::ui;

pub struct CapRawrApp {
    project: Project,
    show_media_browser: bool,
}

impl CapRawrApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            project: Project::default(),
            show_media_browser: false,
        }
    }
}

impl eframe::App for CapRawrApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ui::toolbar::show(ctx, &mut self.project, &mut self.show_media_browser);
        ui::preview::show(ctx, &self.project);
        ui::properties::show(ctx, &mut self.project);
        ui::timeline::show(ctx, &mut self.project);

        if self.show_media_browser {
            ui::media_browser::show(ctx, &mut self.project, &mut self.show_media_browser);
        }
    }
}
