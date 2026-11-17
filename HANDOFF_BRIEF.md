# Handoff Brief (for Pasting to Another AI)

Paste the text between the "---" lines below as-is to the coding AI. Also pass along the feature specification (docs/SPEC.md) and the design document (DESIGN.md).

---
You are responsible for continuing a Windows desktop gadget app. The design is complete and Phases 1–3 are already implemented (v0.1.0). The feature specification (docs/SPEC.md, Draft v0.3) is authoritative. Do not change the requirements on your own; ask about anything unclear before proceeding.

## What to build
A lightweight, standalone Windows desktop gadget app. It keeps multiple arbitrary web apps, AI artifacts (such as Muse Spaces), and local HTML files resident on the desktop — pinned to the very back / wallpaper layer — instead of living in browser tabs. It is not tied to any specific web service: register a URL or a local HTML file and it becomes a desktop gadget.

## Repository
- Implementation target: https://github.com/DashingMZA/DesktopDashboards (default branch `main`)
- Stack: Tauri 2 + Rust (edition 2024), WebView2, vanilla HTML/CSS/JS — the frontend is a single static `src/index.html`, no framework. Package manager: pnpm 12.19.0, Node.js 24+.

## Current state (verified)
- Phases 1–3 implemented; version 0.1.0 (package.json, Cargo.toml, tauri.conf.json, README badge in sync).
- No release has been published yet — the first release will tag `v0.1.0`.
- CI is green (GitHub Actions: `cargo test` + CodeQL). Rust unit tests: 7 (`cargo test --locked --manifest-path src-tauri/Cargo.toml`). There is no JavaScript test suite: no Vitest, no Playwright, no `tests/` directory, no `test:unit` / `test:e2e` / `pnpm test` scripts — do not cite them.
- GitHub Issues are ENABLED. Labels: `dependencies`, `workflows`, `frontend`, `backend`, `tests`.

## Requirements (as built, Phases 1–3)
- R1: Display web URLs and local HTML files (via the `widgetfile` custom protocol) as desktop gadgets; 0–N gadget windows, around 5 shown at once for memory (hidden gadgets are destroyed, not merely hidden)
- R2: Desktop pinning — Mode A, pinned to the very back (`HWND_BOTTOM` + `WS_EX_TOOLWINDOW`, hidden from taskbar and `Alt+Tab`); Mode B, embedded in the wallpaper layer via `Progman` / `WorkerW` so gadgets survive `Win + D`, with automatic re-attachment after Explorer restarts and wallpaper changes. `placement_mode` is a global app-wide setting (`bottom` / `worker_w`)
- R3: Manager window + system tray residency — add gadgets, toggle visibility, adjust position and size, configure detailed settings, delete gadgets, "Lock All / Unlock All", auto-launch at Windows startup (`--silent`)
- R4: Per-gadget controls — lock (hover action bar with drag/home/reload/lock/close), opacity 20–100%, click-through (`WS_EX_TRANSPARENT`), no-focus-steal (`WS_EX_NOACTIVATE`), zoom 50–200%, periodic auto-reload (0 / 1 / 5 / 15 / 30 / 60 minutes) with an input guard that postpones reloads while a field is focused or right after user interaction, hide scrollbar, custom CSS injection
- R5: CLI + single-instance protection (`tauri-plugin-single-instance`) — `DesktopDashboards.exe` restores gadgets, `--manager` brings the manager forward, `--add-url <URL> [--title <NAME>]` adds/enables a gadget, `--silent` starts quietly in the tray
- R6: Navigation control — same-origin navigation stays inside the gadget; external links open in the OS default browser
- R7: Session persistence & isolation — a dedicated WebView2 `user_data_dir` (`%LOCALAPPDATA%\DesktopDashboards\webview`) avoids lock conflicts with other apps while sharing login state across gadgets
- R8: Atomic settings persistence — `%APPDATA%\DesktopDashboards\config.json` written via `.tmp` + flush + rename, with a one-generation `.bak` fallback
- R9: Multi-monitor DPI support and an abnormal-coordinate guard

## Architecture (SPEC.md §2 is authoritative)
- Single tray-resident process managing three window kinds: system tray (resident core), one manager window (`src/index.html`), 0–N frameless gadget WebView windows
- The manager window calls Tauri commands in `src-tauri/src/lib.rs` (`list_widgets`, `add_widget`, `toggle_widget`, `update_widget_settings`, …) → `src-tauri/src/widget.rs` creates/destroys gadget windows → `src-tauri/src/config.rs` persists them to `config.json`
- `src-tauri/src/cli.rs` — CLI parsing (`--manager`, `--add-url`, `--silent`) + unit tests; `src-tauri/src/tray.rs` — tray menu; `src-tauri/src/win32.rs` — Win32 pinning (`HWND_BOTTOM` / `WorkerW`, layered windows), `#[cfg(windows)]` only
- Tauri config: `src-tauri/tauri.conf.json` (productName `DesktopDashboards`, NSIS installer, updater via GitHub releases `latest.json`); capabilities: `src-tauri/capabilities/default.json` (manager window) and `widgets.json` (gadget windows) — keep grants minimal

## Absolute security requirements
- Keep Tauri capabilities minimal — grant only what the manager and gadget windows need; never widen them for convenience
- No backend, no accounts, no telemetry. Gadgets load only URLs the user registered, in their own WebViews
- Do not store passwords or other credentials in the app. Login state lives in the WebView2 profile and belongs to the user

## How to proceed
1. **First verify the current behavior on a real Windows machine** (do not start changing code before this check): `pnpm dev`, add a gadget, toggle lock / opacity / zoom, test both placement modes (including `Win + D` in Mode B), test `--add-url` from another process, confirm the config file round-trips. Report anything that does not match SPEC.md
2. Fix or build only what the owner asked for, in small units; commit in small units and report "what works now" for each change
3. Run `cargo test --locked --manifest-path src-tauri/Cargo.toml` (or let CI run it — the sandbox has no Rust toolchain) and confirm it passes before asking the owner to verify behavior. Never claim a Rust build or test passed without running it
4. When a change touches a feature, screen, or setting described in `README.md`, update `README.md` in the same change. Version bumps must sync package.json, Cargo.toml, tauri.conf.json, the README badge, and CHANGELOG.md together

## Environment
- Windows 10 / 11, Node.js 24+, pnpm 12.19.0, Rust (stable, MSVC toolchain), Visual Studio C++ Build Tools
- The developer speaks English. Report and ask questions in English

## Open items (ask about these instead of deciding them yourself)
- The fixed-name installer decision: tauri-action attaches the versioned name (`DesktopDashboards_<version>_x64-setup.exe`); a fixed-name download link does not resolve today — do not add a rename step without the owner's go-ahead
- The first release (v0.1.0): signing secrets (`TAURI_SIGNING_PRIVATE_KEY`) still need confirming in repo Settings before the first `v*` tag
- The owner still needs to smoke-test the app on Windows (gadget add, tray, lock/opacity/zoom, auto-reload, WorkerW re-attachment after an Explorer restart)
---
