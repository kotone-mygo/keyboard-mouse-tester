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

