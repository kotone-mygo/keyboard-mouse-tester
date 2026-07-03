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
