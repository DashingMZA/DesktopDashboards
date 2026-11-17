# Contributing to DesktopDashboards

Thanks for helping improve DesktopDashboards — a lightweight Windows
desktop app that keeps web apps, AI artifacts (such as Muse Spaces),
and local HTML files resident on your desktop as gadgets. Please read
this guide before opening a pull request.

## Ground rules

- **One app, one repo.** This repo only — do not touch other projects.
- **No deploys.** Do not deploy to production, and do not add workflows
  that deploy on push. Releases are created only by pushing a `v*` tag
  (see `release.md`); PRs are checked by `ci.yml` and never deploy.
- **Follow the Code of Conduct** (`CODE_OF_CONDUCT.md`).
- **English only.** This project is English-only — code, comments, docs,
  commit messages, and PRs in English. (See `AGENTS.md`.)

## Setup

Requirements:

- **Windows 10 / 11** (the app is Windows-only; the installer targets
  Windows, and the Rust tests need Tauri's Linux system libraries only
  on the CI runner)
- **Node.js 24+** (`node --version`)
- **pnpm 12.19.0** (`pnpm --version`) — the repo pins `pnpm@12.19.0` in
  `package.json` (`packageManager`); use Corepack (`corepack enable`) or
  install pnpm 12 directly. Do not use npm or yarn for installs.
- **Rust toolchain** (stable) + **WebView2** (for `pnpm dev` and
  `pnpm run build`)

```bash
git clone https://github.com/DashingMZA/DesktopDashboards.git
cd DesktopDashboards
pnpm install --frozen-lockfile
pnpm dev        # Tauri dev — launches the app
```

## Commands

| Command | What it does |
| --- | --- |
| `pnpm dev` | Tauri dev — launches the app |
| `pnpm run build` | Production build (Tauri build, NSIS installer) |
| `cargo test --locked --manifest-path src-tauri/Cargo.toml` | Rust unit tests (7 tests: CLI parsing, config, widget placement) |

There is no JavaScript test suite in this repo: no Vitest, no
`tests/` directory, and no `test:unit` / `test:e2e` / `pnpm test`
scripts — the frontend is a single static `src/index.html`. Do not
add test-script references to CI or docs without adding the suite
they run. All checks must pass before a PR is ready: install, Rust
tests, and build.

## Branch and PR flow

1. Fork the repo and create a branch from `main`:
   `git checkout -b fix/short-description` (or `feature/...`, `docs/...`).
2. Make focused changes — one concern per PR.
3. In the PR description, include: summary, type of change, testing
   done (the commands above), and screenshots for any UI change.
4. Keep commits logical and reviewable; write clear commit messages in
   English.
5. CI (`.github/workflows/ci.yml`) runs on pull requests. A green CI run
   is required before merge.

## Code and docs conventions

- **Frontend** lives in `src/` (a single static `index.html` with
  inline CSS/JS, no framework). **Backend** lives in `src-tauri/`
  (Rust + Tauri 2). Keep the split clean.
- **Docs:** if your change affects behavior, config, or the release
  process, update the matching doc: `README.md`, `docs/SPEC.md`,
  `docs/CUSTOMIZE.md`, `release.md`, or `CHANGELOG.md`.
- **Versioning:** every release needs the version in sync across
  `package.json`, `src-tauri/Cargo.toml`, `src-tauri/tauri.conf.json`,
  and the README badge, plus a `CHANGELOG.md` entry (see `AGENTS.md` §4).
  `docs/SPEC.md` has no version-history section — do not cite a
  "SPEC §18".
- **Styling:** keep the design consistent with the existing manager
  window and gadget UI; avoid unnecessary animations and oversized UI.

## What not to do

- Don't commit `node_modules/`, `dist/`, `src-tauri/target/`, or `.env`
  (all gitignored).
- Don't change the Rust edition, Tauri version, or workflow Node versions
  without noting it — the project requires Node 24+ and Rust edition 2024.
- Don't add dependencies without justification; prefer the smallest change
  that solves the problem.
- Don't report a security issue as a public issue — see `SECURITY.md`.

## Questions

GitHub Issues are enabled in this repo. For questions, open an issue,
or open a pull request with your change.
