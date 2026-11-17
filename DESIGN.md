# DesktopDashboards Design Document (v0.1 / Design Phase)

- Repository: https://github.com/DashingMZA/DesktopDashboards

## Concept

A lightweight Windows desktop gadget app that keeps multiple arbitrary
web apps, AI artifacts (such as Muse Spaces), and local HTML files
resident on the desktop — pinned to the very back / wallpaper layer —
instead of living in browser tabs.

## Design intent (archive)

This document is the archive of the initial concept. The implementation
source of truth is the **[Feature Specification](SPEC.md)** (Draft v0.3),
which takes precedence whenever the two disagree.

- **Standalone first:** the app is not tied to any specific web service.
  Register a URL or a local HTML file and it becomes a desktop gadget.
- **Tray-resident, single process:** one process manages the tray icon,
  one manager window, and 0–N frameless gadget windows (WebView2).
- **Desktop pinning:** gadgets sit at the bottom of the Z-order
  (Mode A, default) or embedded in the wallpaper layer via `WorkerW`
  (Mode B, Phase 3) so they survive `Win + D`.
- **Session isolation:** a dedicated WebView2 `user_data_dir`
  (`%LOCALAPPDATA%\DesktopDashboards\webview`) avoids lock conflicts
  with other apps while sharing login state across gadgets.
- **Safe navigation:** same-origin navigation stays inside the gadget;
  external links open in the OS default browser.
- **Per-gadget control:** lock, opacity (20–100%), click-through,
  no-focus-steal, zoom (50–200%), auto-reload with an input guard,
  hide scrollbar, custom CSS injection.
- **External kick-off:** other apps can add a gadget via CLI
  (`DesktopDashboards.exe --add-url <URL> [--title <NAME>]`);
  `SPEC.md` §6 also sketches bundling with a host app as a possible
  future integration.

## Phases (as built)

- **Phase 1 (MVP):** CLI + single-instance protection, atomic
  `config.json` persistence, pinned-to-bottom mode, navigation control,
  manager window + tray menu.
- **Phase 2:** lock, opacity / click-through / no-focus-steal, zoom,
  auto-reload with input guard, multi-monitor DPI support.
- **Phase 3:** `WorkerW` wallpaper embedding with automatic
  re-attachment, custom CSS injection.
