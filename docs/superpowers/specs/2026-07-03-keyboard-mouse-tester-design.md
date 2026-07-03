# Keyboard & Mouse Tester — Design Document

## Overview

A cross-platform GUI tool that displays a visual keyboard and mouse layout, highlighting keys/buttons in real-time as the user presses them. Built with `egui` (via `eframe`) for the GUI layer and `rdev` for global input event listening.

## Target Users

General end users who want to test whether their keyboard keys or mouse buttons are functioning correctly.

## Tech Stack

| Component | Library | Rationale |
|-----------|---------|-----------|
| GUI framework | `eframe` (egui) | Pure Rust, immediate mode, cross-platform, zero system dependencies, flexible custom painting |
| Input listening | `rdev` | Cross-platform global input events (no window focus required) |
| Inter-thread communication | `std::sync::mpsc` | Standard library channel |

## Architecture

```
┌─────────────────────────────────────┐
│          Background Thread           │
│  rdev listens for global events      │
│  (key down/up, mouse move/click/     │
│   scroll)                            │
└──────────────┬──────────────────────┘
               │ mpsc channel
               ▼
┌─────────────────────────────────────┐
│         Main Thread (egui)          │
│  Each frame: drain channel events   │
│  Update key/mouse state             │
│  Render visual layout               │
└─────────────────────────────────────┘
```

## Data Model

```rust
struct KeyState {
    pressed: bool,
    press_count: u64,
}

struct MouseState {
    x: f64,
    y: f64,
    left: bool,
    middle: bool,
    right: bool,
    scroll_delta: (f64, f64),
}

struct AppState {
    keys: HashMap<Key, KeyState>,
    mouse: MouseState,
    event_log: VecDeque<String>,  // last 20 events
}
```

## Visual Layout

Three regions on screen:

1. **Status bar** — Key press count, mouse coordinates (x, y)
2. **Keyboard area** — US ANSI 104-key layout drawn with egui painting
   - Row 1: Escape, function keys, number row
   - Row 2: QWERTY top row
   - Row 3: Home row + modifiers
   - Row 4: Bottom row with space bar, Alt, Ctrl, Win/Cmd
   - Pressed keys highlighted with a different color (blue/green background)
3. **Mouse area + Event log** (side panel)
   - Left / Middle / Right buttons visual
   - Scroll wheel indicator
   - Cursor coordinates
   - Recent event history (last 20 entries)

## Keyboard Layout Support

- Phase 1: US ANSI 104-key fixed layout
- Key sizes: space bar extra wide, Tab/Backspace/CapsLock/Shift medium, standard keys uniform

## Scope (Phase 1)

### Included
- Visual keyboard with real-time key highlighting
- Visual mouse buttons + coordinates + scroll
- Recent event log (textual, last 20 events)
- Cross-platform codebase (initial validation on Linux)
- Multiple simultaneous key press support

### Excluded (YAGNI)
- Custom key mapping / remapping
- Input recording and replay
- Configuration file
- User-defined keyboard layouts
- Custom themes / skinning

## Dependencies

```toml
[dependencies]
eframe = "0.31"
rdev = "0.5"
```

## State Diagram

Per key/button: `[Released] -- key down event --> [Pressed] -- key up event --> [Released]`

The event log is append-only FIFO (ring buffer of 20 entries).
