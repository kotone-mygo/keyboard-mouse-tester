use crate::keyboard;
use crate::mouse;
use crate::state::AppState;
use eframe::egui;

pub struct App {
    state: AppState,
}

impl App {
    pub fn new() -> Self {
        Self {
            state: AppState::new(),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let (events, mouse_pos, modifiers) = ctx.input(|i| {
            let events = i.raw.events.clone();
            let mouse_pos = i.pointer.hover_pos();
            let modifiers = i.modifiers;
            (events, mouse_pos, modifiers)
        });

        for event in &events {
            match event {
                egui::Event::Key {
                    key,
                    pressed,
                    repeat,
                    ..
                } => {
                    if *repeat && *pressed {
                        continue;
                    }
                    self.state.process_key(*key, *pressed);
                }
                egui::Event::PointerButton {
                    button, pressed, ..
                } => {
                    self.state.process_pointer_button(*button, *pressed);
                }
                egui::Event::MouseWheel { delta, .. } => {
                    self.state.process_scroll(delta.x as f64, delta.y as f64);
                }
                _ => {}
            }
        }

        if let Some(pos) = mouse_pos {
            self.state.process_mouse_move(pos.x as f64, pos.y as f64);
        }

        egui::TopBottomPanel::top("status").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("Keyboard & Mouse Tester");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let mut mod_str = String::new();
                    if modifiers.shift {
                        mod_str.push_str("Shift ");
                    }
                    if modifiers.ctrl {
                        mod_str.push_str("Ctrl ");
                    }
                    if modifiers.alt {
                        mod_str.push_str("Alt ");
                    }
                    if modifiers.command {
                        mod_str.push_str("Cmd ");
                    }
                    if !mod_str.is_empty() {
                        ui.label(mod_str.trim_end());
                        ui.separator();
                    }
                    ui.label(format!("Total: {}", self.state.total_presses));
                    ui.separator();
                    ui.label(format!(
                        "Mouse: ({:.0}, {:.0})",
                        self.state.mouse.x, self.state.mouse.y
                    ));
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                keyboard::draw_keyboard(ui, &self.state);
                ui.separator();
                mouse::draw_mouse_area(ui, &self.state);
            });
        });

        ctx.request_repaint();
    }
}
