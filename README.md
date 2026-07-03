# Keyboard & Mouse Tester

A cross-platform GUI tool that visualizes keyboard and mouse input in real-time. Built with Rust, egui, and eframe.

## Features

- **104-key US ANSI keyboard layout** with real-time key highlighting
- **Mouse visual** with L/M/R button states, coordinate display, and scroll indicator
- **Event log** showing the last 20 input events
- **Total press counter** and **modifier indicators** (Shift/Ctrl/Alt/Cmd)
- **Cross-platform**: Linux, Windows, macOS

## Build & Run

```bash
cargo run
```

### Linux Dependencies

On Linux, you need X11 development libraries:

```bash
# Ubuntu/Debian
sudo apt install libx11-dev libxtst-dev libxi-dev libxext-dev pkg-config

# Fedora/RHEL
sudo dnf install libX11-devel libXtst-devel libXi-devel libXext-devel pkgconfig

# Arch Linux
sudo pacman -S libx11 libxtst libxi libxext pkgconf
```

### Windows Build

Cross-compile from Linux using MinGW:

```bash
sudo apt install mingw-w64
rustup target add x86_64-pc-windows-gnu
cargo build --target x86_64-pc-windows-gnu
```

## Input Source

This app captures input through the GUI window (egui/winit). The window must have focus to receive events. This approach works on all platforms including WSL2, unlike global input hooks (XRecord) which are unavailable in many environments.

## Limitations

The following keys are displayed in the layout but cannot light up because egui's key enum does not cover them:

- Modifier keys: Ctrl, Shift, Alt, Win, Menu
- Caps Lock, Num Lock, Scroll Lock, Print Screen, Pause
- Numpad keys (all)

Modifier states (Ctrl/Shift/Alt/Cmd) are shown as text indicators in the status bar. All letter keys, number keys, punctuation, F-keys, navigation keys, arrow keys, Space, Enter, Tab, Backspace, and Escape are fully supported.

## License

MIT
