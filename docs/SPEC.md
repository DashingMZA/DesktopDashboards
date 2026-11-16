# DesktopDashboards Specification (SPEC, Draft v0.3)

- **Repository name**: `DesktopDashboards`
- **App nickname**: TBD (working title: `DesktopDashboards`)
- **Target OS**: Windows 10 / 11 (x64)
- **Tech stack**: Tauri v2 (Rust) + WebView2 + HTML/CSS/JS

---

## 1. Concept & Purpose

A lightweight desktop gadget app that lets you keep multiple arbitrary web apps, AI artifacts (such as Muse Spaces), and local HTML files resident on your Windows desktop (wallpaper / very back layer).

- **Versatility as a standalone product**: Not tied to any specific web service. Simply register a URL or local HTML to turn any web screen into a desktop widget.
- **Kick-off integration from external apps**: Besides running standalone, it can be bundled with external apps such as `Muse Desktop Mascot` and invoked with CLI arguments for seamless integration.

---

## 2. System Architecture

### 2.1 Process & Window Structure (Single Process, Multiple Windows)

Runs as a tray-resident **single process**, which creates and manages multiple windows internally.

1. **System tray (resident core)**: Stays resident in the background even when all windows are closed.
2. **Manager window (Manager UI / one)**: A regular window for adding gadgets, managing the list, and configuring settings.
3. **Gadget windows (Widget WebView / 0 to N)**: Frameless desktop gadget windows that display registered and enabled content.
   - **Memory management policy**: Since WebView2 launches a renderer process per window, a comfortable number of simultaneously displayed gadgets is assumed to be **around 5**. Gadgets with `enabled: false` (hidden) are **destroyed (closed)** rather than merely hidden, in order to free memory.

### 2.2 WebView2 Session Isolation

- A dedicated `user_data_dir` is used explicitly to prevent file-lock conflicts with other Tauri / WebView2 apps.
  - Example: `%LOCALAPPDATA%\DesktopDashboards\webview`
- Because multiple gadget windows in the same process share the same `user_data_dir`, **the authentication state of a service you have logged in to once (e.g. `muse.ai`) is shared across all gadgets**.

### 2.3 Single-Instance Protection & CLI Integration

- `tauri-plugin-single-instance` is adopted to prevent multiple instances of the app.
- If the app is run again while already running, the second process passes only its arguments to the existing process and exits immediately.

#### CLI Argument Specification (for external app integration)

| Example command | Behavior |
| :--- | :--- |
| `DesktopDashboards.exe` (no arguments) | On first launch, restores registered gadgets. If already running, brings the manager window to the front. |
| `DesktopDashboards.exe --manager` | Opens the manager window and brings it to the front. |
| `DesktopDashboards.exe --add-url <URL> [--title <NAME>]` | Adds the specified URL as a new gadget and shows it. If the same URL already exists, enables it, temporarily raises its Z-order to make it visible (or highlights it in the manager), then places it in its designated position. |
| `DesktopDashboards.exe --silent` | Does not open the manager window; starts quietly in the tray with only enabled gadgets (**when registered for OS auto-launch, this flag must always be included**). |

---

## 3. Wallpaper Pinning (Desktop Fixing) Specification

To achieve the Windows behavior of "not getting in the way of other windows and sticking to the desktop like wallpaper," two pinning modes are provided.  
`placement_mode` is a **global app-wide setting** applied uniformly to all gadgets (to avoid the complexity of coordinate management caused by mixing modes).

### 3.1 Mode A: Pinned-to-Bottom Mode (default; stability-focused)

- `decorations: false` (no title bar or frame), `skip_taskbar: true` (hidden from the taskbar).
- In addition to `always_on_bottom: true`, the window is kept behind all other windows using Win32 APIs (applying `WS_EX_TOOLWINDOW` to exclude it from `Alt+Tab`, and `WM_WINDOWPOSCHANGING` / `SetWindowPos(HWND_BOTTOM)`).
- **Advantages**: Works stably regardless of Windows 10/11 version differences, and clicking and scrolling behave naturally.
- **Known limitation**: Because it is treated as a normal desktop window, **pressing `Win + D` (Show desktop) minimizes (temporarily hides) it like any other window**.

### 3.2 Mode B: Wallpaper-Layer Embedding Mode (`WorkerW` mode, Phase 3)

- Sends `0x052C` to `Progman` to create a `WorkerW`, then uses `SetParent` to embed the gadget behind the desktop icon layer (`SHELLDLL_DefView`) / into the wallpaper layer.
- **Advantages**: Gadgets do not disappear when `Win + D` (Show desktop) is pressed.
- **Coordinate conversion on mode switch**: Mode A (virtual-screen coordinates) and Mode B (client coordinates relative to the `WorkerW` parent window) use different coordinate spaces, so a coordinate conversion equivalent to `ScreenToClient` / `ClientToScreen` is performed when switching modes to prevent widgets from flying off-screen.

---

## 4. Gadget Feature Specification

### 4.1 Data Model & Persistence (`config.json`)

The configuration file is stored at `%APPDATA%\DesktopDashboards\config.json`.

#### Atomic Writes & Backup
- For debounced saves on position/size changes and for settings changes, `config.json` is not overwritten directly. Instead, an atomic write is performed: **write to a temporary file (`config.json.tmp`), flush, then replace via rename**.
- A valid configuration is kept as a one-generation backup (`config.json.bak`), and the app automatically falls back to it in case of corruption.

```json
{
  "version": 1,
  "placement_mode": "bottom",
  "autostart": false,
  "widgets": [
    {
      "id": "550e8400-e29b-41d4-a716-446655440000",
      "title": "Commuter Train Timetable",
      "source_type": "url",
      "source": "https://muse.ai/library/artifacts/tong-qin-dian-che-shi-ke-biao",
      "enabled": true,
      "x": 1400,
      "y": 40,
      "width": 480,
      "height": 760,
      "dpi_scale": 1.0,
      "opacity": 0.95,
      "zoom": 1.0,
      "locked": false,
      "click_through": false,
      "no_activate": false,
      "auto_reload_minutes": 0,
      "hide_scrollbar": true,
      "custom_css": ""
    }
  ]
}
```

#### Handling Local HTML (`source_type: "file"`)
- `source_type` is either `"url"` or `"file"`.
- For `"file"`, `source` holds the absolute path of the local HTML file, which is displayed within a safe scope via a Tauri custom protocol (or another permitted local-loading mechanism).

### 4.2 Coordinate System & Multi-Monitor DPI Support

- Coordinates (`x`, `y`) and size (`width`, `height`) are recorded in **physical pixels (virtual desktop coordinates)**, together with the `dpi_scale` of the monitor at save time (e.g. `1.0`, `1.25`, `1.5`).
- On restore, using the logical size at save time (`physical size / saved dpi_scale`) as the baseline, the **physical size is recalculated to match the DPI of the destination monitor**, so that the apparent size does not change drastically.
- If, at startup or restore, a gadget lies outside the area of the currently connected monitors (e.g. after an external monitor was disconnected), it is automatically corrected to a safe area within the primary monitor.

### 4.3 Navigation Policy on Link Clicks

Because frameless gadget windows have no browser "Back" button, navigation is controlled with the following rules:

1. **Same-origin navigation**: Allowed to proceed within the gadget.
2. **Navigation to external domains / new windows (`target="_blank"`, etc.)**: Not opened inside the gadget; opened in the OS default browser instead (via `tauri-plugin-opener` or similar).
3. **Hover bar actions**:
   - **Reload (↻)**: Reloads the currently displayed page.
   - **Home (⌂)**: Returns to the registered initial `source` (URL / file).

### 4.4 Per-Gadget Control & Display Features

1. **Lock feature (`locked`)**
   - **When unlocked**: A thin action bar (drag to move, title, home, reload, lock, close) appears only while hovering over the top edge of the gadget, and resizing from the window edges is also possible.
   - **When locked**: The action bar and resize borders are completely hidden to prevent accidental moves or resizes.
2. **Opacity (`opacity`) and click-through (`click_through`)**
   - Both are implemented on the premise of Win32 layered windows (`WS_EX_LAYERED`).
   - **Opacity (20% – 100%)**: Controls the opacity of the whole window via `SetLayeredWindowAttributes` (`LWA_ALPHA`).
   - **Click-through**: Applies `WS_EX_TRANSPARENT` so mouse events pass through to the desktop behind (it can be disabled from the tray menu or the manager window).
3. **No focus stealing (`no_activate`)**
   - For "view-only / light-touch" gadgets, an option is provided to apply Win32 `WS_EX_NOACTIVATE`. Clicking does not take focus (IME or the window being worked in) away from other apps.
4. **Display customization**
   - **Zoom level (`zoom`)**: 50% – 200%
   - **Hide scrollbar (`hide_scrollbar`)**: Adjusts the look to be cleaner via CSS injection
5. **Periodic auto-reload and input protection (`auto_reload_minutes`)**
   - Auto-reloads at intervals such as 0 (disabled), 1 min, 5 min, 15 min, 30 min, and 60 min.
   - **Reload guard**: If an input element (`input` / `textarea` / `contenteditable`) in the WebView has focus, or if there was user interaction (click or key input) within the last few seconds, the reload is postponed and executed safely after the interaction ends. The tray menu's "Reload All" is also subject to the reload guard.

---

## 5. UI / Interaction Flow

### 5.1 System Tray Menu

- **Open Manager** (double-click does the same)
- **Gadget list** (checkboxes instantly toggle show/hide)
- **Lock All / Unlock All**
- **Reload All**
- **Launch at Windows startup** (toggle: when enabled, registered with `--silent`)
- **Quit**

### 5.2 Manager Window (Dashboard Manager)

- **Add new gadget**: Choose a URL or local HTML file, enter a title, and create it immediately.
- **Gadget card list**:
  - Show ON / OFF switch
  - Lock / click-through / no-focus-steal toggles
  - Detailed settings (opacity slider, zoom, auto-reload interval, hide scrollbar, custom CSS)
  - "Reset position to center" button (to rescue gadgets lost off-screen)
  - Delete button

---

## 6. External App Integration (Assuming Bundling with Muse Desktop Mascot)

Integration from the `Muse Desktop Mascot` side is envisioned as follows:

1. `DesktopDashboards.exe` is bundled in the build / distribution package of `Muse Desktop Mascot`.
2. From the `Muse Desktop Mascot` tray menu or a chat screen action, it launches `DesktopDashboards.exe --add-url <Space URL>` or runs it without arguments.
3. `DesktopDashboards` operates as an independent resident app, so gadgets on the desktop remain even when the mascot is closed.

---

## 7. Development Phase Plan

### Phase 1: MVP (Minimum Viable Configuration)
- **M0 (top-priority spike verification / decision gate)**: Verify on a real machine whether **logging in to `muse.ai` and viewing/operating Spaces (Artifacts) works correctly** inside a fresh Tauri v2 + WebView2 with an independent `user_data_dir`.
  - *Decision gate*: If login is rejected due to the authentication method or similar, reconsider before proceeding to the full MVP implementation whether to "limit to a public-URL mode" or "switch to default-browser authentication / a `Muse Desktop Mascot` integration approach".
- Initial Tauri v2 project setup (`DesktopDashboards`)
- `single-instance` + CLI arguments (`--add-url`, `--manager`, `--silent`)
- Pinned-to-bottom mode (`HWND_BOTTOM` + `skip_taskbar` + `WS_EX_TOOLWINDOW`)
- Navigation control (allow same-origin navigation, open external links in the default browser)
- Creation / destruction of multiple gadget windows and atomic `config.json` persistence (URL + local HTML)
- Basic manager window (add, delete, toggle visibility, reset position) + tray menu (including auto-launch with `--silent`)

### Phase 2: Polishing the Gadget Experience
- Lock feature (drag handle bar that appears only on hover + reload / home buttons)
- Opacity, click-through (`WS_EX_TRANSPARENT`), and `WS_EX_NOACTIVATE` via `WS_EX_LAYERED`
- Zoom level, hide scrollbar, periodic auto-reload (with input protection)
- Multi-monitor DPI scale support & abnormal-coordinate guard

### Phase 3: Advanced Pinning & Integration
- `WorkerW` wallpaper-embedding mode (`Win + D` resilience) and inter-mode coordinate conversion
  - Also verify **automatic re-attachment** when `WorkerW` is recreated by an Explorer restart or wallpaper slideshow switch (e.g. via `TaskbarCreated` detection).
- Custom CSS injection feature
- Bundling with `Muse Desktop Mascot` and implementing the kick-off feature on its side
