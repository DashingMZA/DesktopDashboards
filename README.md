# DesktopDashboards

A lightweight desktop gadget app built with Tauri v2 + WebView2 that lets you keep multiple arbitrary web apps, AI artifacts (such as Muse Spaces), and local HTML files resident on your Windows desktop (at the very back / in the wallpaper layer).

## Key Features (Phases 1–3 implemented)

- **General-purpose web desktop gadgets**: Display web URLs and local HTML files (via the `widgetfile` custom protocol) as desktop widgets
- **Session persistence & isolation**: Uses a dedicated WebView2 `user_data_dir` (`%LOCALAPPDATA%\DesktopDashboards\webview`) to avoid lock conflicts with other apps while sharing login state across gadgets
- **External app integration & single-instance protection**: Seamless invocation via `tauri-plugin-single-instance` and CLI arguments
  - `DesktopDashboards.exe` (no arguments): Restores registered gadgets (brings the manager window to the front if already running)
  - `DesktopDashboards.exe --manager`: Brings the manager window to the front
  - `DesktopDashboards.exe --add-url <URL> [--title <NAME>]`: Adds and enables the specified URL and shows it
  - `DesktopDashboards.exe --silent`: Starts quietly in the tray without opening the manager window, launching only enabled gadgets
- **Desktop pinning & navigation control**:
  - **Mode A (pinned to bottom)**: Frameless, hidden from the taskbar, pinned to the very back (`HWND_BOTTOM` + `WS_EX_TOOLWINDOW` + `WM_WINDOWPOSCHANGING`)
  - **Mode B (embedded in wallpaper layer, Phase 3)**: Embeds into `Progman` / `WorkerW` to survive `Win + D` (switchable per gadget; supports automatic re-attachment after Explorer restarts and wallpaper changes)
  - **Navigation control**: Allows same-origin navigation and opens external links in the default browser
- **Gadget controls & customization (Phases 2–3)**:
  - **Lock feature**: When unlocked, hovering over the top edge shows an action bar (drag to move, home, reload, lock, close) and resizing is enabled; when locked, the bar and resizing are disabled (supports "Lock All / Unlock All" in the tray)
  - **Opacity + click-through**: `WS_EX_LAYERED` (20%–100%) and mouse pass-through via `WS_EX_TRANSPARENT`
  - **No focus stealing**: `WS_EX_NOACTIVATE` prevents clicks from taking focus away from other apps
  - **Zoom level**: Per-gadget zoom setting from 50% to 200%
  - **Periodic auto-reload + reload guard**: Auto-reload at 1–60 minute intervals, with protection that postpones reloads while an input field is focused or right after user interaction
  - **Custom CSS injection (Phase 3)**: Edit, instantly apply, and persist arbitrary CSS per gadget
- **Atomic settings persistence**: Atomic writes to `%APPDATA%\DesktopDashboards\config.json` (`.tmp` + flush + rename) with a one-generation backup (`.bak`) fallback
- **Manager window & system tray residency**: Add gadgets, toggle visibility, adjust position and size, configure detailed settings, delete gadgets, and toggle auto-launch at Windows startup (`--silent`)

## Tech Stack

- **Target OS**: Windows 10 / 11 (x64)
- **Framework**: Tauri v2 (Rust) + WebView2 + HTML/CSS/JS

## Development & Build

```bash
# Install dependencies
npm install

# Run in development mode
npm run dev

# Release build
npm run build
```

## Documentation

- [Specification (docs/SPEC.md)](docs/SPEC.md)
