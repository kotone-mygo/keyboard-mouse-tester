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

