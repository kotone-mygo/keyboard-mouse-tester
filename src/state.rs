use eframe::egui::Key;
use std::collections::{HashMap, VecDeque};

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
    pub scroll_delta: (f64, f64),
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
                scroll_delta: (0.0, 0.0),
            },
            event_log: VecDeque::new(),
            total_presses: 0,
        }
    }

    pub fn process_key(&mut self, key: Key, pressed: bool) {
        if pressed {
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
        } else if let Some(state) = self.keys.get_mut(&key) {
            state.pressed = false;
        }
    }

    pub fn process_mouse_move(&mut self, x: f64, y: f64) {
        self.mouse.x = x;
        self.mouse.y = y;
    }

    pub fn process_pointer_button(&mut self, button: eframe::egui::PointerButton, pressed: bool) {
        match button {
            eframe::egui::PointerButton::Primary => self.mouse.left = pressed,
            eframe::egui::PointerButton::Middle => self.mouse.middle = pressed,
            eframe::egui::PointerButton::Secondary => self.mouse.right = pressed,
            _ => {}
        }
        if pressed {
            self.log(format!("Mouse {:?}", button));
        }
    }

    pub fn process_scroll(&mut self, dx: f64, dy: f64) {
        self.mouse.scroll_delta.0 += dx;
        self.mouse.scroll_delta.1 += dy;
        self.log(format!("Scroll ({:.0}, {:.0})", dx, dy));
    }

    fn log(&mut self, msg: String) {
        self.event_log.push_back(msg);
        while self.event_log.len() > 20 {
            self.event_log.pop_front();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use eframe::egui::Key;

    #[test]
    fn test_key_press_release() {
        let mut state = AppState::new();
        state.process_key(Key::A, true);
        assert!(state.keys[&Key::A].pressed);
        assert_eq!(state.keys[&Key::A].press_count, 1);

        state.process_key(Key::A, true);
        assert_eq!(state.keys[&Key::A].press_count, 1);

        state.process_key(Key::A, false);
        assert!(!state.keys[&Key::A].pressed);
    }

    #[test]
    fn test_mouse_click() {
        let mut state = AppState::new();
        state.process_pointer_button(eframe::egui::PointerButton::Primary, true);
        assert!(state.mouse.left);
        state.process_pointer_button(eframe::egui::PointerButton::Primary, false);
        assert!(!state.mouse.left);
    }

    #[test]
    fn test_mouse_move() {
        let mut state = AppState::new();
        state.process_mouse_move(100.0, 200.0);
        assert_eq!(state.mouse.x, 100.0);
        assert_eq!(state.mouse.y, 200.0);
    }

    #[test]
    fn test_scroll_delta() {
        let mut state = AppState::new();
        state.process_scroll(10.0, 20.0);
        assert_eq!(state.mouse.scroll_delta, (10.0, 20.0));
        state.process_scroll(5.0, -3.0);
        assert_eq!(state.mouse.scroll_delta, (15.0, 17.0));
    }

    #[test]
    fn test_log_capped() {
        let mut state = AppState::new();
        for _ in 0..30 {
            state.process_key(Key::A, true);
            state.process_key(Key::A, false);
        }
        assert!(state.event_log.len() <= 20);
    }
}
