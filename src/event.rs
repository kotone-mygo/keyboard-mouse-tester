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
