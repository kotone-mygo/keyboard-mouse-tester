use std::sync::mpsc::{self, Receiver};
use eframe::egui;
use crate::event::AppEvent;
use crate::input;
use crate::keyboard;
use crate::mouse;
use crate::state::AppState;

pub struct App {
    state: AppState,
    receiver: Receiver<AppEvent>,
}

impl App {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel();
        input::start_listener(tx);
        Self {
            state: AppState::new(),
            receiver: rx,
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        while let Ok(event) = self.receiver.try_recv() {
            self.state.process_event(event);
        }

        egui::TopBottomPanel::top("status").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("Keyboard & Mouse Tester");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(format!("Total: {}", self.state.total_presses));
                    ui.separator();
                    ui.label(format!("Mouse: ({:.0}, {:.0})", self.state.mouse.x, self.state.mouse.y));
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                keyboard::draw_keyboard(ui, &self.state);
                ui.separator();
                mouse::draw_mouse_area(ui, &self.state);
            });
        });

        ctx.request_repaint();
    }
}
