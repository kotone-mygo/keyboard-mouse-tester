#![cfg_attr(
    all(target_os = "windows", not(debug_assertions)),
    windows_subsystem = "windows"
)]

mod app;
mod keyboard;
mod mouse;
mod state;

use eframe::egui;

fn main() -> eframe::Result {
    // On WSL2, the Wayland display connection can be unstable.
    // Force X11 backend to avoid connection drops.
    #[cfg(target_os = "linux")]
    if std::fs::read_to_string("/proc/version")
        .map(|v| v.contains("Microsoft") || v.contains("WSL"))
        .unwrap_or(false)
    {
        // SAFETY: called before any threads, single-threaded context
        unsafe {
            std::env::set_var("WAYLAND_DISPLAY", "");
            std::env::set_var("WINIT_UNIX_BACKEND", "x11");
        }
    }

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1400.0, 920.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Keyboard & Mouse Tester",
        options,
        Box::new(|_cc| Ok(Box::new(app::App::new()))),
    )
}
