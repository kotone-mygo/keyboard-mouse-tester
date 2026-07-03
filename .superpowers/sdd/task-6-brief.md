### Task 6: Main App Integration

**Files:**
- Modify: `src/app.rs`
- Modify: `src/main.rs`

**Interfaces:**
- Consumes: `AppState`, `start_listener`, `draw_keyboard`, `draw_mouse_area`
- Produces: Running eframe application window

- [ ] **Step 1: Write App struct with eframe::App impl**

```rust
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
```

- [ ] **Step 2: Rewrite main.rs entry point**

```rust
mod app;
mod event;
mod input;
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
```

- [ ] **Step 3: Build and verify**

Run: `cargo build`
Expected: Build succeeds

- [ ] **Step 4: Run tests**

Run: `cargo test`
Expected: All 4 tests pass

- [ ] **Step 5: Commit**

```bash
git add -A && git commit -m "feat: integrate app UI with keyboard, mouse, and input listener"
```

---

## Spec Coverage Check

| Spec Requirement | Task | Status |
|---|---|---|
| Visual keyboard with real-time highlighting | Task 4 + Task 6 | Implemented |
| US ANSI 104-key fixed layout | Task 4 | Implemented |
| Visual mouse buttons + coordinates | Task 5 | Implemented |
| Scroll wheel indicator | Task 5 (via event log) | Implemented |
| Recent event log (last 20) | Task 2 (state) + Task 5 (draw) | Implemented |
| Multiple simultaneous key press | Task 2 (per-key HashMap) | Implemented |
| Background thread (rdev → mpsc → main) | Task 3 | Implemented |
| Cross-platform (initial Linux validation) | Global constraint | Compiled on all targets |
| Status bar | Task 6 | Implemented |
