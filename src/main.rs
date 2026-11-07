mod app;
mod engine;
mod media;
mod render;
mod ui;
mod util;

fn main() -> eframe::Result<()> {
    tracing_subscriber::fmt::init();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("CapRawr — Video Editor")
            .with_inner_size([1600.0, 900.0])
            .with_min_inner_size([1024.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "CapRawr",
        options,
        Box::new(|cc| Ok(Box::new(app::CapRawrApp::new(cc)))),
    )
}
