# WPMeter

A minimal, low-overhead menu bar application for **macOS** and **Linux** that displays your current typing speed in Words Per Minute (WPM).

## Features

- **Real-time WPM Display** — Shows your current WPM in the system tray / menu bar, updated every second.
- **Privacy Focused** — Does not log keystrokes. It only counts the *number* of key presses to calculate speed and immediately discards the events.
- **Idle Decay** — WPM resets to 0 after one second of inactivity.
- **Low Overhead** — Built with Rust for minimal resource usage.
- **Background Process** — Runs as a menu bar app (macOS) or system tray app (Linux) with no dock/taskbar presence.

## Prerequisites

- [Rust & Cargo](https://rustup.rs/) (1.85+ recommended, edition 2024)

## Running in Debug Mode

Debug mode compiles faster and includes debug symbols, useful during development:

```bash
# Clone the repository
cd WPMeter

# Run in debug mode
cargo run
```

You can also build the debug binary without running it:

```bash
cargo build
# Binary is located at: target/debug/WPMeter
```

> **Note:** Debug builds are not optimized and will be noticeably larger and slower than release builds.

## Building Release Binaries

Release mode enables compiler optimizations for a smaller, faster binary.

### Manual Build (any platform)

```bash
cargo build --release
# Binary is located at: target/release/WPMeter
```

### macOS — App Bundle

The bundler script builds a release binary and packages it into a `.app` bundle with `LSUIElement` set so it runs as a background-only app (no Dock icon):

```bash
chmod +x bundlers/macos.sh
./bundlers/macos.sh
```

**Output:** `target/release/os_bundle/WPMeter.app`

To run the app bundle:

```bash
open target/release/os_bundle/WPMeter.app
```

### Linux — Desktop Package

The bundler script builds a release binary and creates a FreeDesktop-compliant package with `.desktop` launcher and autostart entries:

```bash
chmod +x bundlers/linux.sh
./bundlers/linux.sh
```

**Output:** `target/release/os_bundle/WPMeter/`

**System-wide install:**

```bash
sudo cp target/release/os_bundle/WPMeter/usr/bin/WPMeter /usr/bin/
sudo cp target/release/os_bundle/WPMeter/usr/share/applications/com.jithin.wpmeter.desktop /usr/share/applications/
sudo cp target/release/os_bundle/WPMeter/etc/xdg/autostart/com.jithin.wpmeter.desktop /etc/xdg/autostart/
```

**Current-user install:**

```bash
cp target/release/os_bundle/WPMeter/usr/bin/WPMeter ~/.local/bin/
cp target/release/os_bundle/WPMeter/usr/share/applications/com.jithin.wpmeter.desktop ~/.local/share/applications/
mkdir -p ~/.config/autostart
cp target/release/os_bundle/WPMeter/etc/xdg/autostart/com.jithin.wpmeter.desktop ~/.config/autostart/
```

## Permissions

WPMeter listens for global key events to count keystrokes. This requires special permissions:

| Platform | Requirement |
|----------|-------------|
| **macOS** | Grant **Accessibility** permission: *System Settings → Privacy & Security → Accessibility → enable WPMeter* |
| **Linux** | User must have read access to input devices (typically via the `input` group), or run with elevated privileges |

## Usage

- The app runs in the background and sits in your system tray / menu bar.
- Start typing in any application and the WPM counter will update in real time.
- Click the tray icon to quit.

## Project Structure

```
WPMeter/
├── src/
│   ├── main.rs        # App entry point, tray icon, and event loop
│   └── wpm.rs         # WPM calculation engine
├── bundlers/
│   ├── macos.sh       # macOS .app bundle script
│   └── linux.sh       # Linux desktop package script
├── Cargo.toml         # Dependencies and project metadata
└── README.md
```
