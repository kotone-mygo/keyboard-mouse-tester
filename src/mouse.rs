use eframe::egui::{self, Color32, Rect, Rounding, Vec2};
use crate::state::AppState;

pub fn draw_mouse_area(ui: &mut egui::Ui, state: &AppState) {
    ui.vertical(|ui| {
        ui.heading("Mouse");

        ui.horizontal(|ui| {
            draw_button(ui, "L", state.mouse.left);
            draw_button(ui, "M", state.mouse.middle);
            draw_button(ui, "R", state.mouse.right);
        });

        ui.add_space(8.0);
        ui.label(format!("X: {:.0}", state.mouse.x));
        ui.label(format!("Y: {:.0}", state.mouse.y));
        ui.add_space(12.0);

        ui.heading("Events");
        let log_text: String = state
            .event_log
            .iter()
            .rev()
            .map(|s| s.as_str())
            .collect::<Vec<_>>()
            .join("\n");
        egui::ScrollArea::vertical()
            .max_height(300.0)
            .show(ui, |ui| {
                ui.monospace(log_text);
            });
    });
}

fn draw_button(ui: &mut egui::Ui, label: &str, pressed: bool) {
    let size = Vec2::new(44.0, 44.0);
    let (response, painter) = ui.allocate_painter(size, egui::Sense::hover());
    let rect = Rect::from_min_size(response.rect.min, size);

    let bg = if pressed {
        Color32::from_rgb(60, 180, 255)
    } else {
        Color32::from_gray(45)
    };
    painter.rect_filled(rect, Rounding::same(6), bg);
    painter.text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        label,
        egui::FontId::proportional(16.0),
        Color32::WHITE,
    );
}
