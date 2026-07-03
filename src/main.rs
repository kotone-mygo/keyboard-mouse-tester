mod app;
mod keyboard;
mod mouse;
mod state;

use eframe::egui;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([900.0, 500.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Keyboard & Mouse Tester",
        options,
        Box::new(|_cc| Ok(Box::new(app::App::new()))),
    )
}
