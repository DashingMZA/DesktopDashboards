# Changelog

All notable changes to this project are documented here. The format is
based on Keep a Changelog. All entries are dated 2026-11-16, when this
repo was created and built out in a single day. Early commit messages
are in Japanese (from the project's origin scaffold); the app and docs
are English.

## Unreleased

### Added

- Project docs set: `MEMORY.md`, `CLAUDE.md`, `release.md`,
  `CONTRIBUTING.md`, `SECURITY.md`, `CODE_OF_CONDUCT.md`,
  `ACCESSIBILITY.md`, `CHANGELOG.md` (this file) — rewritten for
  DesktopDashboards from direct repo inspection.
- GitHub automation: CI, CodeQL, Labeler, Dependabot, and a Windows-only
  release workflow (NSIS installer + portable ZIP, draft release on
  `v*` tags).
- GitHub issue templates (bug report / feature request); Issues are
  enabled in this repo.
- Labels: `dependencies`, `workflows`, `frontend`, `backend`, `tests`.

### Fixed

- `ci.yml` called non-existent `pnpm run test:unit` / `test:rust`
  scripts — now runs `cargo test` directly after installing Tauri's
  Linux system dependencies.
- `release.yml` copied mascot exe/asset paths that do not exist in
  this repo — now builds `desktop-dashboards.exe` and the portable
  ZIP `desktop-dashboards-x64.zip` (exe + README only).
- `dependabot.yml` / `codeql.yml` header names corrected to
  DesktopDashboards.

## 0.1.0 — 2026-11-16

The first version. Phases 1–3 implemented (see `docs/SPEC.md`):

### Added

- **Phase 1 (MVP):** Tauri 2 + WebView2 app shell; CLI
  (`--add-url`, `--manager`, `--silent`); single-instance protection;
  atomic `config.json` persistence; pinned-to-bottom desktop mode;
  navigation control (same-origin in-gadget, external links in the
  default browser); manager window (add / delete / toggle gadgets);
  system tray menu with auto-launch.
- **Phase 2:** per-gadget lock (hover action bar), opacity (20–100%),
  click-through, no-focus-steal, zoom (50–200%), periodic auto-reload
  with input guard, hide scrollbar, multi-monitor DPI support with
  off-screen rescue.
- **Phase 3:** `WorkerW` wallpaper-layer embedding with automatic
  re-attachment after Explorer restarts / wallpaper changes; custom
  CSS injection per gadget.
- Dedicated WebView2 profile (`%LOCALAPPDATA%\DesktopDashboards\webview`)
  shared across gadgets, so logins persist.
- NSIS installer (per-user), portable ZIP, and in-app updater via
  GitHub releases (`latest.json`).
- pnpm 12.19.0 + Node ≥ 24; Rust edition 2024.
