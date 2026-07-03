use crate::state::AppState;
use eframe::egui::{self, Color32, CornerRadius, Rect, Shape, Stroke, Vec2};

pub fn draw_mouse_area(ui: &mut egui::Ui, state: &AppState) {
    ui.vertical(|ui| {
        ui.heading("Mouse");

        ui.horizontal(|ui| {
            draw_button(ui, "L", state.mouse.left);
            draw_button(ui, "M", state.mouse.middle);
            draw_button(ui, "R", state.mouse.right);
            ui.add_space(12.0);
            draw_scroll_wheel(ui, state.mouse.scroll_delta);
        });

        ui.add_space(8.0);
        ui.label(format!("X: {:.0}", state.mouse.x));
        ui.label(format!("Y: {:.0}", state.mouse.y));
        ui.add_space(4.0);
        ui.label(format!(
            "Scroll: ({:.0}, {:.0})",
            state.mouse.scroll_delta.0, state.mouse.scroll_delta.1
        ));
        ui.add_space(12.0);

        ui.heading("Events");
        let log_text: String = state
            .event_log
            .iter()
            .rev()
            .map(|s| s.as_str())
            .collect::<Vec<_>>()
            .join("\n");
        egui::ScrollArea::vertical().show(ui, |ui| {
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
    painter.rect_filled(rect, CornerRadius::same(6), bg);
    painter.text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        label,
        egui::FontId::proportional(16.0),
        Color32::WHITE,
    );
}

fn draw_scroll_wheel(ui: &mut egui::Ui, delta: (f64, f64)) {
    let size = Vec2::new(24.0, 48.0);
    let (response, painter) = ui.allocate_painter(size, egui::Sense::hover());
    let rect = Rect::from_min_size(response.rect.min, size);

    painter.rect_filled(rect, CornerRadius::same(4), Color32::from_gray(40));

    let dy = delta.1;
    if dy.abs() > 0.5 {
        let center = rect.center();
        let (tip, base1, base2) = if dy > 0.0 {
            (
                egui::pos2(center.x, center.y + 8.0),
                egui::pos2(center.x - 6.0, center.y - 4.0),
                egui::pos2(center.x + 6.0, center.y - 4.0),
            )
        } else {
            (
                egui::pos2(center.x, center.y - 8.0),
                egui::pos2(center.x - 6.0, center.y + 4.0),
                egui::pos2(center.x + 6.0, center.y + 4.0),
            )
        };
        let color = if dy > 0.0 {
            Color32::from_rgb(60, 180, 255)
        } else {
            Color32::from_rgb(255, 180, 60)
        };
        painter.add(Shape::convex_polygon(
            vec![tip, base1, base2],
            color,
            Stroke::NONE,
        ));
    }
}
