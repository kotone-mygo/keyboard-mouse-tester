# Keyboard & Mouse Tester Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build a cross-platform GUI tool that visually displays a keyboard and mouse, highlighting pressed keys/buttons in real-time.

**Architecture:** A background thread uses `rdev` to capture global input events and sends them via `mpsc` channel to the main thread. The main thread runs an `eframe` (egui) window that drains events each frame and renders the visual keyboard layout, mouse buttons, and event log.

**Tech Stack:** Rust, `eframe 0.31` (egui), `rdev 0.5`, `std::sync::mpsc`

## Global Constraints

- Must compile on Linux, Windows, and macOS (initial validation on Linux)
- `edition = "2024"` in Cargo.toml
- No external config files or data stores
- Max 20 entries in event log ring buffer
- All keyboard/mouse state lives in `AppState`
- YAGNI: no remapping, recording, replay, themes, or user-defined layouts

---

### Task 1: Project Scaffolding

**Files:**
- Modify: `Cargo.toml`
- Create: `src/event.rs`
- Create: `src/state.rs`
- Create: `src/input.rs`
- Create: `src/keyboard.rs`
- Create: `src/mouse.rs`
- Create: `src/app.rs`

- [ ] **Step 1: Update Cargo.toml with dependencies**

```toml
[package]
name = "keyboard-mouse-tester"
version = "0.1.0"
edition = "2024"

[dependencies]
eframe = "0.31"
rdev = "0.5"
```

- [ ] **Step 2: Create empty module files**

Create `src/event.rs`:
```rust
pub enum AppEvent {
}
```

Create `src/state.rs`:
```rust
pub struct AppState;
```

Create `src/input.rs`:
```rust
```

Create `src/keyboard.rs`:
```rust
```

Create `src/mouse.rs`:
```rust
```

Create `src/app.rs`:
```rust
pub struct App;
```

- [ ] **Step 3: Wire modules in main.rs**

```rust
mod app;
mod event;
mod input;
mod keyboard;
mod mouse;
mod state;

fn main() {
    println!("Keyboard & Mouse Tester");
}
```

- [ ] **Step 4: Verify it compiles**

Run: `cargo check`
Expected: Build succeeds with no errors

- [ ] **Step 5: Commit**

```bash
git add -A && git commit -m "chore: scaffold project structure"
```

---

### Task 2: Core State Model + Event Processing

**Files:**
- Modify: `src/event.rs`
- Modify: `src/state.rs`

**Interfaces:**
- Consumes: nothing
- Produces: `AppEvent` enum, `AppState` with `process_event(&mut self, AppEvent)` method

- [ ] **Step 1: Write AppEvent enum in event.rs**

```rust
use rdev::{Button, Key};

#[derive(Clone, Debug)]
pub enum AppEvent {
    KeyPressed(Key),
    KeyReleased(Key),
    MouseMoved(f64, f64),
    ButtonPressed(Button),
    ButtonReleased(Button),
    WheelScrolled(f64, f64),
}
```

- [ ] **Step 2: Write AppState + KeyState + MouseState in state.rs**

```rust
use std::collections::{HashMap, VecDeque};
use rdev::{Button, Key};
use crate::event::AppEvent;

#[derive(Clone)]
pub struct KeyState {
    pub pressed: bool,
    pub press_count: u64,
}

#[derive(Clone)]
pub struct MouseState {
    pub x: f64,
    pub y: f64,
    pub left: bool,
    pub middle: bool,
    pub right: bool,
}

pub struct AppState {
    pub keys: HashMap<Key, KeyState>,
    pub mouse: MouseState,
    pub event_log: VecDeque<String>,
    pub total_presses: u64,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            keys: HashMap::new(),
            mouse: MouseState {
                x: 0.0,
                y: 0.0,
                left: false,
                middle: false,
                right: false,
            },
            event_log: VecDeque::new(),
            total_presses: 0,
        }
    }

    pub fn process_event(&mut self, event: AppEvent) {
        match event {
            AppEvent::KeyPressed(key) => {
                let state = self.keys.entry(key).or_insert(KeyState {
                    pressed: false,
                    press_count: 0,
                });
                if !state.pressed {
                    state.pressed = true;
                    state.press_count += 1;
                    self.total_presses += 1;
                }
                self.log(format!("{:?}", key));
            }
            AppEvent::KeyReleased(key) => {
                if let Some(state) = self.keys.get_mut(&key) {
                    state.pressed = false;
                }
            }
            AppEvent::MouseMoved(x, y) => {
                self.mouse.x = x;
                self.mouse.y = y;
            }
            AppEvent::ButtonPressed(button) => {
                match button {
                    Button::Left => self.mouse.left = true,
                    Button::Middle => self.mouse.middle = true,
                    Button::Right => self.mouse.right = true,
                    _ => {}
                }
                self.log(format!("Mouse {:?}", button));
            }
            AppEvent::ButtonReleased(button) => {
                match button {
                    Button::Left => self.mouse.left = false,
                    Button::Middle => self.mouse.middle = false,
                    Button::Right => self.mouse.right = false,
                    _ => {}
                }
            }
            AppEvent::WheelScrolled(dx, dy) => {
                self.log(format!("Scroll ({:.0}, {:.0})", dx, dy));
            }
        }
    }

    fn log(&mut self, msg: String) {
        self.event_log.push_back(msg);
        while self.event_log.len() > 20 {
            self.event_log.pop_front();
        }
    }
}
```

- [ ] **Step 3: Run to verify it compiles**

Run: `cargo check`
Expected: Build succeeds

- [ ] **Step 4: Add unit tests to state.rs**

Append at end of `src/state.rs`:
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use rdev::{Key, Button};

    #[test]
    fn test_key_press_release() {
        let mut state = AppState::new();
        state.process_event(AppEvent::KeyPressed(Key::KeyA));
        assert!(state.keys[&Key::KeyA].pressed);
        assert_eq!(state.keys[&Key::KeyA].press_count, 1);

        state.process_event(AppEvent::KeyPressed(Key::KeyA));
        assert_eq!(state.keys[&Key::KeyA].press_count, 1);

        state.process_event(AppEvent::KeyReleased(Key::KeyA));
        assert!(!state.keys[&Key::KeyA].pressed);
    }

    #[test]
    fn test_mouse_click() {
        let mut state = AppState::new();
        state.process_event(AppEvent::ButtonPressed(Button::Left));
        assert!(state.mouse.left);
        state.process_event(AppEvent::ButtonReleased(Button::Left));
        assert!(!state.mouse.left);
    }

    #[test]
    fn test_mouse_move() {
        let mut state = AppState::new();
        state.process_event(AppEvent::MouseMoved(100.0, 200.0));
        assert_eq!(state.mouse.x, 100.0);
        assert_eq!(state.mouse.y, 200.0);
    }

    #[test]
    fn test_log_capped() {
        let mut state = AppState::new();
        for _ in 0..30 {
            state.process_event(AppEvent::KeyPressed(Key::KeyA));
            state.process_event(AppEvent::KeyReleased(Key::KeyA));
        }
        assert!(state.event_log.len() <= 20);
    }
}
```

- [ ] **Step 5: Run tests to verify**

Run: `cargo test`
Expected: 4 tests pass

- [ ] **Step 6: Commit**

```bash
git add -A && git commit -m "feat: add core state model and event processing"
```

---

### Task 3: Input Listener

**Files:**
- Modify: `src/input.rs`

**Interfaces:**
- Consumes: `Sender<AppEvent>` from `std::sync::mpsc`
- Produces: `start_listener(Sender<AppEvent>)` function that spawns rdev thread
- Calls: `AppEvent::KeyPressed`, `AppEvent::KeyReleased`, `AppEvent::MouseMoved`, `AppEvent::ButtonPressed`, `AppEvent::ButtonReleased`, `AppEvent::WheelScrolled`

- [ ] **Step 1: Write input listener**

```rust
use std::sync::mpsc::Sender;
use std::thread;
use crate::event::AppEvent;

pub fn start_listener(tx: Sender<AppEvent>) {
    thread::spawn(move || {
        if let Err(e) = rdev::listen(move |event| {
            let app_event = match event.event_type {
                rdev::EventType::KeyPress(key) => Some(AppEvent::KeyPressed(key)),
                rdev::EventType::KeyRelease(key) => Some(AppEvent::KeyReleased(key)),
                rdev::EventType::ButtonPress(button) => Some(AppEvent::ButtonPressed(button)),
                rdev::EventType::ButtonRelease(button) => Some(AppEvent::ButtonReleased(button)),
                rdev::EventType::MouseMove { x, y } => {
                    Some(AppEvent::MouseMoved(x, y))
                }
                rdev::EventType::Scroll { delta_x, delta_y } => {
                    Some(AppEvent::WheelScrolled(delta_x, delta_y))
                }
                _ => None,
            };
            if let Some(ev) = app_event {
                tx.send(ev).ok();
            }
        }) {
            eprintln!("rdev listen error: {:?}", e);
        }
    });
}
```

- [ ] **Step 2: Verify it compiles**

Run: `cargo check`
Expected: Build succeeds

- [ ] **Step 3: Commit**

```bash
git add -A && git commit -m "feat: add rdev input listener thread"
```

---

### Task 4: Keyboard Layout + Drawing

**Files:**
- Modify: `src/keyboard.rs`

**Interfaces:**
- Consumes: `&AppState` (reads `keys` HashMap)
- Produces: `draw_keyboard(ui: &mut egui::Ui, state: &AppState)` function

- [ ] **Step 1: Define key layout data structure and constants**

```rust
use eframe::egui::{self, Color32, Rect, Rounding, Vec2};
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
        KeyDef { row: 1, col: 13.0, width: 2.0, label: "Back", key: Key::BackSpace },

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
        KeyDef { row: 5, col: 12.5, width: 1.25, label: "Menu", key: Key::ContextMenu },
        KeyDef { row: 5, col: 13.75, width: 1.25, label: "Ctrl", key: Key::ControlRight },
    ]
}
```

- [ ] **Step 2: Write keyboard drawing function**

Append to `src/keyboard.rs`:
```rust
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
            Rounding::same(4.0),
            bg,
            egui::Stroke::new(1.0, border),
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
```

- [ ] **Step 3: Verify it compiles**

Run: `cargo check`
Expected: Build succeeds

- [ ] **Step 4: Commit**

```bash
git add -A && git commit -m "feat: add keyboard layout and drawing"
```

---

### Task 5: Mouse Drawing + Event Log

**Files:**
- Modify: `src/mouse.rs`

**Interfaces:**
- Consumes: `&AppState` (reads `mouse` and `event_log`)
- Produces: `draw_mouse_area(ui: &mut egui::Ui, state: &AppState)` function

- [ ] **Step 1: Write mouse area drawing function**

```rust
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
    painter.rect_filled(rect, Rounding::same(6.0), bg);
    painter.text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        label,
        egui::FontId::proportional(16.0),
        Color32::WHITE,
    );
}
```

- [ ] **Step 2: Verify it compiles**

Run: `cargo check`
Expected: Build succeeds

- [ ] **Step 3: Commit**

```bash
git add -A && git commit -m "feat: add mouse visual and event log"
```

---

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
