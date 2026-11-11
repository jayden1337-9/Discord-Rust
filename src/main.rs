#[cfg(not(target_os = "android"))]
fn main() -> eframe::Result<()> {
    discord_rust::run_desktop()
}

#[cfg(target_os = "android")]
fn main() {}
