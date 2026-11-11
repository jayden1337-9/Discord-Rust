mod app;
mod theme;
mod ui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1180.0, 760.0])
            .with_min_inner_size([720.0, 480.0])
            .with_title("Discord Rust"),
        ..Default::default()
    };

    eframe::run_native(
        "Discord Rust",
        options,
        Box::new(|cc| {
            theme::configure(&cc.egui_ctx);
            Ok(Box::new(app::DiscordRustApp::default()))
        }),
    )
}
