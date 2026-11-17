# Customization Guide

A reference covering detailed customization methods for DesktopDashboards.
For basic usage, see the [README](../README.md); for internal details and the authoritative specification (Single Source of Truth), see the **[Feature Specification](SPEC.md)** (Draft v0.3).

All settings below are per gadget unless stated otherwise, and are saved
to `%APPDATA%\DesktopDashboards\config.json` (atomic writes with a
one-generation `.bak` fallback — see "Config file" below). The manager
window applies most changes immediately; no restart is needed.

## Adding a gadget

Manager window → **Type** (URL or local HTML file) → **Title (optional)**
→ **URL or HTML file path** → **Add & Show**.

- **URL:** any web app or page, e.g. a Muse Space, a timetable, a
  dashboard. Same-origin navigation stays inside the gadget; external
  links open in the OS default browser.
- **Local HTML file:** pick a `.html` file on disk; it is served to the
  gadget through a Tauri custom protocol (`widgetfile`).
- **From the command line:** `DesktopDashboards.exe --add-url <URL>
  [--title <NAME>]` adds the URL as a gadget and shows it. If the URL
  already exists, it is enabled and brought forward instead.

## Gadget card settings

Open a gadget's settings in the manager window. Every gadget has:

| Setting | What it does |
| --- | --- |
| **Lock** | Hides the hover action bar and resize borders so the gadget cannot be moved or resized accidentally. Unlock to move (drag the bar), resize, or reach Home / Reload / Close. |
| **Click-through** | Mouse events pass through the gadget to the desktop behind it. Turn it off from the manager window or tray menu if you need to interact with the gadget again. |
| **No focus steal** | Clicking the gadget does not take keyboard focus away from other apps (good for view-only gadgets). |
| **Opacity** | Slider, 20–100%. Controls the whole window's transparency. |
| **Zoom** | Slider, 50–200%. Scales the gadget's page content. |
| **Auto-reload** | 0 (disabled), 1, 5, 15, 30, or 60 minutes. Reloads the page on schedule — but never while an input field is focused or right after you clicked/typed (the reload guard postpones it). |
| **Hide scrollbar** | Hides the page scrollbar via CSS injection for a cleaner look. |
| **Custom CSS** | A text editor for arbitrary CSS applied to the gadget's page (e.g. hiding page elements, restyling). Applied immediately; the button shows "Custom CSS (set)" once non-empty. |
| **Placement mode** | `bottom` (pinned to the very back, default) or `worker_w` (Mode B: embedded in wallpaper via `WorkerW` — survives `Win + D`). |
| **Reset position to center** | Rescues a gadget lost off-screen (e.g. after disconnecting a monitor). |

## Moving and resizing

- **Unlocked gadget:** hover over its top edge — the action bar appears
  (drag to move, title, Home, Reload, Lock, Close). Drag window edges
  to resize.
- **Position/size are saved automatically** (debounced) and restored on
  launch. Coordinates are physical pixels; on a different-DPI monitor
  the size is recalculated so the gadget looks the same size.
- **Off-screen rescue:** if a gadget ends up outside all connected
  monitors, the app moves it into the primary monitor automatically;
  the manager's "Reset position to center" does the same on demand.

## Tray menu

Right-click the tray icon (or double-click to open the manager):

- **Open Manager** — opens the manager window.
- **Gadget list** — checkboxes instantly show/hide each gadget.
- **Lock All / Unlock All** — locks or unlocks every gadget at once.
- **Reload All** — reloads all gadgets (subject to the reload guard).
- **Launch at Windows startup** — registers the app to start with
  `--silent` (tray only, no manager window).
- **Quit** — closes the app and all gadgets.

## Startup and CLI

| Command | Behavior |
| --- | --- |
| `DesktopDashboards.exe` | Restores enabled gadgets; if already running, brings the manager window forward. |
| `DesktopDashboards.exe --manager` | Opens the manager window. |
| `DesktopDashboards.exe --add-url <URL> [--title <NAME>]` | Adds and shows the URL as a gadget. |
| `DesktopDashboards.exe --silent` | Starts quietly in the tray with only enabled gadgets (use this for auto-launch). |

A second launch while the app is running passes its arguments to the
running instance and exits (single-instance protection).

## Config file

- **Location:** `%APPDATA%\DesktopDashboards\config.json`
- **Backup:** `config.json.bak` (one generation). If `config.json` is
  corrupt, the app falls back to the backup automatically.
- **Editing by hand:** close the app first, then edit. The file holds
  `version`, `placement_mode`, `autostart`, and the `widgets` array
  (`id`, `title`, `source_type`, `source`, `enabled`, `x`, `y`,
  `width`, `height`, `dpi_scale`, `opacity`, `zoom`, `locked`,
  `click_through`, `no_activate`, `auto_reload_minutes`,
  `hide_scrollbar`, `custom_css`, `placement_mode`). Bad values are
  guarded on load (e.g. off-screen gadgets are moved into view).

## WebView profile and logins

- **Profile:** `%LOCALAPPDATA%\DesktopDashboards\webview` — the
  WebView2 profile shared by all gadgets. Log in once (e.g. to
  `muse.ai`) and every gadget shares that login.
- **Clearing logins:** delete the `webview` folder while the app is
  closed. This logs you out of every gadget at once.
