use eframe::egui::{self, Color32, CornerRadius, Rect, Vec2};
use rdev::Key;
use crate::state::AppState;

struct KeyDef {
    row: usize,
    col: f32,
    width: f32,
    label: &'static str,
    key: Key,
}

const UNIT: f32 = 52.0;
const KEY_H: f32 = 44.0;
const GAP: f32 = 4.0;

fn layout() -> Vec<KeyDef> {
    vec![
        KeyDef { row: 0, col: 0.0, width: 1.0, label: "Esc", key: Key::Escape },
        KeyDef { row: 0, col: 2.0, width: 1.0, label: "F1", key: Key::F1 },
        KeyDef { row: 0, col: 3.0, width: 1.0, label: "F2", key: Key::F2 },
        KeyDef { row: 0, col: 4.0, width: 1.0, label: "F3", key: Key::F3 },
        KeyDef { row: 0, col: 5.0, width: 1.0, label: "F4", key: Key::F4 },
        KeyDef { row: 0, col: 6.5, width: 1.0, label: "F5", key: Key::F5 },
        KeyDef { row: 0, col: 7.5, width: 1.0, label: "F6", key: Key::F6 },
        KeyDef { row: 0, col: 8.5, width: 1.0, label: "F7", key: Key::F7 },
        KeyDef { row: 0, col: 9.5, width: 1.0, label: "F8", key: Key::F8 },
        KeyDef { row: 0, col: 11.0, width: 1.0, label: "F9", key: Key::F9 },
        KeyDef { row: 0, col: 12.0, width: 1.0, label: "F10", key: Key::F10 },
        KeyDef { row: 0, col: 13.0, width: 1.0, label: "F11", key: Key::F11 },
        KeyDef { row: 0, col: 14.0, width: 1.0, label: "F12", key: Key::F12 },
        KeyDef { row: 0, col: 15.5, width: 1.0, label: "PrtSc", key: Key::PrintScreen },
        KeyDef { row: 0, col: 16.5, width: 1.0, label: "ScrLk", key: Key::ScrollLock },
        KeyDef { row: 0, col: 17.5, width: 1.0, label: "Pause", key: Key::Pause },

        KeyDef { row: 1, col: 0.0, width: 1.0, label: "~", key: Key::BackQuote },
        KeyDef { row: 1, col: 1.0, width: 1.0, label: "1", key: Key::Num1 },
        KeyDef { row: 1, col: 2.0, width: 1.0, label: "2", key: Key::Num2 },
        KeyDef { row: 1, col: 3.0, width: 1.0, label: "3", key: Key::Num3 },
        KeyDef { row: 1, col: 4.0, width: 1.0, label: "4", key: Key::Num4 },
        KeyDef { row: 1, col: 5.0, width: 1.0, label: "5", key: Key::Num5 },
        KeyDef { row: 1, col: 6.0, width: 1.0, label: "6", key: Key::Num6 },
        KeyDef { row: 1, col: 7.0, width: 1.0, label: "7", key: Key::Num7 },
        KeyDef { row: 1, col: 8.0, width: 1.0, label: "8", key: Key::Num8 },
        KeyDef { row: 1, col: 9.0, width: 1.0, label: "9", key: Key::Num9 },
        KeyDef { row: 1, col: 10.0, width: 1.0, label: "0", key: Key::Num0 },
        KeyDef { row: 1, col: 11.0, width: 1.0, label: "-", key: Key::Minus },
        KeyDef { row: 1, col: 12.0, width: 1.0, label: "=", key: Key::Equal },
        KeyDef { row: 1, col: 13.0, width: 2.0, label: "Back", key: Key::Backspace },

        KeyDef { row: 2, col: 0.0, width: 1.5, label: "Tab", key: Key::Tab },
        KeyDef { row: 2, col: 1.5, width: 1.0, label: "Q", key: Key::KeyQ },
        KeyDef { row: 2, col: 2.5, width: 1.0, label: "W", key: Key::KeyW },
        KeyDef { row: 2, col: 3.5, width: 1.0, label: "E", key: Key::KeyE },
        KeyDef { row: 2, col: 4.5, width: 1.0, label: "R", key: Key::KeyR },
        KeyDef { row: 2, col: 5.5, width: 1.0, label: "T", key: Key::KeyT },
        KeyDef { row: 2, col: 6.5, width: 1.0, label: "Y", key: Key::KeyY },
        KeyDef { row: 2, col: 7.5, width: 1.0, label: "U", key: Key::KeyU },
        KeyDef { row: 2, col: 8.5, width: 1.0, label: "I", key: Key::KeyI },
        KeyDef { row: 2, col: 9.5, width: 1.0, label: "O", key: Key::KeyO },
        KeyDef { row: 2, col: 10.5, width: 1.0, label: "P", key: Key::KeyP },
        KeyDef { row: 2, col: 11.5, width: 1.0, label: "[", key: Key::LeftBracket },
        KeyDef { row: 2, col: 12.5, width: 1.0, label: "]", key: Key::RightBracket },
        KeyDef { row: 2, col: 13.5, width: 1.5, label: "\\", key: Key::BackSlash },

        KeyDef { row: 3, col: 0.0, width: 1.75, label: "Caps", key: Key::CapsLock },
        KeyDef { row: 3, col: 1.75, width: 1.0, label: "A", key: Key::KeyA },
        KeyDef { row: 3, col: 2.75, width: 1.0, label: "S", key: Key::KeyS },
        KeyDef { row: 3, col: 3.75, width: 1.0, label: "D", key: Key::KeyD },
        KeyDef { row: 3, col: 4.75, width: 1.0, label: "F", key: Key::KeyF },
        KeyDef { row: 3, col: 5.75, width: 1.0, label: "G", key: Key::KeyG },
        KeyDef { row: 3, col: 6.75, width: 1.0, label: "H", key: Key::KeyH },
        KeyDef { row: 3, col: 7.75, width: 1.0, label: "J", key: Key::KeyJ },
        KeyDef { row: 3, col: 8.75, width: 1.0, label: "K", key: Key::KeyK },
        KeyDef { row: 3, col: 9.75, width: 1.0, label: "L", key: Key::KeyL },
        KeyDef { row: 3, col: 10.75, width: 1.0, label: ";", key: Key::SemiColon },
        KeyDef { row: 3, col: 11.75, width: 1.0, label: "'", key: Key::Quote },
        KeyDef { row: 3, col: 12.75, width: 2.25, label: "Enter", key: Key::Return },

        KeyDef { row: 4, col: 0.0, width: 2.25, label: "Shift", key: Key::ShiftLeft },
        KeyDef { row: 4, col: 2.25, width: 1.0, label: "Z", key: Key::KeyZ },
        KeyDef { row: 4, col: 3.25, width: 1.0, label: "X", key: Key::KeyX },
        KeyDef { row: 4, col: 4.25, width: 1.0, label: "C", key: Key::KeyC },
        KeyDef { row: 4, col: 5.25, width: 1.0, label: "V", key: Key::KeyV },
        KeyDef { row: 4, col: 6.25, width: 1.0, label: "B", key: Key::KeyB },
        KeyDef { row: 4, col: 7.25, width: 1.0, label: "N", key: Key::KeyN },
        KeyDef { row: 4, col: 8.25, width: 1.0, label: "M", key: Key::KeyM },
        KeyDef { row: 4, col: 9.25, width: 1.0, label: ",", key: Key::Comma },
        KeyDef { row: 4, col: 10.25, width: 1.0, label: ".", key: Key::Dot },
        KeyDef { row: 4, col: 11.25, width: 1.0, label: "/", key: Key::Slash },
        KeyDef { row: 4, col: 12.25, width: 2.75, label: "Shift", key: Key::ShiftRight },

        KeyDef { row: 5, col: 0.0, width: 1.25, label: "Ctrl", key: Key::ControlLeft },
        KeyDef { row: 5, col: 1.25, width: 1.25, label: "Win", key: Key::MetaLeft },
        KeyDef { row: 5, col: 2.5, width: 1.25, label: "Alt", key: Key::Alt },
        KeyDef { row: 5, col: 3.75, width: 6.25, label: "", key: Key::Space },
        KeyDef { row: 5, col: 10.0, width: 1.25, label: "Alt", key: Key::AltGr },
        KeyDef { row: 5, col: 11.25, width: 1.25, label: "Win", key: Key::MetaRight },
        KeyDef { row: 5, col: 12.5, width: 1.25, label: "Menu", key: Key::Function },
        KeyDef { row: 5, col: 13.75, width: 1.25, label: "Ctrl", key: Key::ControlRight },
    ]
}

pub fn draw_keyboard(ui: &mut egui::Ui, state: &AppState) {
    let keys = layout();
    let total_w = 15.0 * (UNIT + GAP);
    let total_h = 6.0 * (KEY_H + GAP);

    let (response, painter) = ui.allocate_painter(
        Vec2::new(total_w, total_h + GAP),
        egui::Sense::hover(),
    );

    for kd in &keys {
        let x = response.rect.min.x + GAP + kd.col * (UNIT + GAP);
        let y = response.rect.min.y + GAP + kd.row as f32 * (KEY_H + GAP);
        let w = kd.width * UNIT - GAP;

        let key_rect = Rect::from_min_size(
            egui::pos2(x, y),
            Vec2::new(w, KEY_H),
        );

        let pressed = state.keys.get(&kd.key).map_or(false, |k| k.pressed);
        let bg = if pressed {
            Color32::from_rgb(60, 180, 255)
        } else {
            Color32::from_gray(45)
        };
        let border = if pressed {
            Color32::from_rgb(100, 220, 255)
        } else {
            Color32::from_gray(70)
        };

        painter.rect(
            key_rect,
            CornerRadius::same(4),
            bg,
            egui::Stroke::new(1.0, border),
            egui::StrokeKind::Inside,
        );

        if !kd.label.is_empty() {
            painter.text(
                key_rect.center(),
                egui::Align2::CENTER_CENTER,
                kd.label,
                egui::FontId::proportional(13.0),
                Color32::WHITE,
            );
        }
    }
}
