mod app;
mod theme;
mod ui;

pub fn run_desktop() -> eframe::Result<()> {
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

#[cfg(target_os = "android")]
#[no_mangle]
pub fn android_main(app: winit::platform::android::activity::AndroidApp) {
    android_logger::init_once(
        android_logger::Config::default().with_max_level(log::LevelFilter::Info),
    );

    let options = eframe::NativeOptions {
        android_app: Some(app),
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
    .expect("failed to start Discord Rust");
}
