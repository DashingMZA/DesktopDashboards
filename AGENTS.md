# AGENTS.md - DesktopDashboards Development and Operations Rules

This file defines the project-specific rules that AI agents must always follow when doing development, build, and release work in this repository (`DesktopDashboards`).

---

## 1. Rules to Prevent Leftover Junk Files (Unneeded Artifacts and Temporary Files)

When doing builds, packaging, documentation updates, and GitHub Release work, thoroughly follow the rules below so that **no old files or temporary junk files are ever left in the workspace**:

1. **Organizing distribution ZIP archives (root directory)**
   - When creating or placing a distribution ZIP (`desktop-dashboards-x64.zip`), always delete or overwrite old versioned ZIP files (`*-v*.zip`) and any existing ZIP with the same name immediately beforehand.
2. **Keeping the `dist/` folder up to date or cleaning it up**
   - If a `dist/` folder exists, always overwrite and update `dist/desktop-dashboards.exe` and `dist/README.md` with the latest build artifacts when a release build completes (do not leave an old version of the `.exe` behind).
3. **Deleting unneeded files when replacing docs and images (`docs/`, etc.)**
   - When adding or replacing screenshots or assets in `README.md` or elsewhere under a new filename, promptly `git rm` / delete old image files that are no longer referenced (for example, screenshots of the old UI).
4. **Deleting temporary files (JSON payloads, etc.) immediately**
   - Create UTF-8 JSON files for sending to the GitHub REST API (`issue_payload.json`, `release_payload.json`, etc.) and temporary staging folders for packaging in a temporary directory (a scratch area or `$env:TEMP`), not in the repository root, and delete them promptly after use.

---

## 2. Git / GitHub Issue / Release Operations Rules

- Before starting work, output a message to the user explaining what you are about to do, then begin.
- When changing code or documentation in the Git repository, first create a GitHub Issue and get the user's confirmation before proceeding. (Issues are enabled in this repo.)
- Make Git commits only after the user has verified the behavior and approved the review, and include `(Closes #<Issue number>)` in an English commit message.
- When making a change related to a feature, screen, or setting described in `README.md`, always update `README.md` (and `dist/README.md`) in sync.
- When writing a file path in public documentation or source code, use a path relative to the repository root.
- **Workflow files cannot be pushed via the GitHub App.** A push containing `.github/workflows/*.yml` fails with 403 (the App lacks workflows permission). Workflow files go via the owner's GitHub web-UI upload; verify byte-identical afterwards. The same applies to binary files: push-files is UTF-8 text only, so PNGs go via web UI too.

---

## 3. Build and Verification Rules (Debug vs Release)

- **During development and when verifying each Issue (on `dev/*` branches, etc.)**:
  - To shorten build time, use a **Debug build (`cargo build`)**, place the generated `src-tauri/target/debug/desktop-dashboards.exe` at `dist/desktop-dashboards.exe`, and verify behavior there (skip creating a distribution ZIP or installer).
- **At release time (just before merging into the `main` branch)**:
  - Only at the point just before the `main` merge, when all features for the version are complete and the user has instructed "Release it!", do a **Release build and create the installer (`pnpm run build:installer` / `pnpm run build`)**, overwrite `dist/` with the optimized `.exe`, and create the distribution ZIP archive and NSIS setup installer and attach them to the Release. Do not create an installer during normal development.
  - **Release asset names:** tauri-action attaches the installer under its versioned Tauri name (`DesktopDashboards_<version>_x64-setup.exe`); the portable ZIP is `desktop-dashboards-x64.zip` (fixed name, built by `release.yml`).
    - A fixed-name installer download link does NOT resolve today. Renaming the installer to a fixed name is an open decision — do not promise fixed-name links, and do not add a rename step without the user's go-ahead, because the updater's `latest.json` points at the artifact name.
  - **Pre-release documentation sync checklist** (always check this before building and tagging):
    1. List all changes (Issues) for the target version with `git log <previous tag>..HEAD --oneline`.
    2. Update the version badge (`version-vX.Y.Z`) and contents of `README.md`, and sync them to `dist/README.md`.
    3. Add an entry for the version to `CHANGELOG.md`.
    4. If settings, tray menu items, or UI operations changed, update `docs/CUSTOMIZE.md`.

---

## 4. Regression Prevention and Test Operations Rules

To prevent regressions that come with feature additions and bug fixes, the
following automated test suite is in place.

- **Rust backend unit tests (`cargo test`)**
  - Target: `src-tauri/src/cli.rs`, `src-tauri/src/config.rs`,
    `src-tauri/src/lib.rs`, `src-tauri/src/widget.rs` (CLI parsing,
    config load/save, widget placement guards). 7 tests.
  - Run: `cargo test --locked --manifest-path src-tauri/Cargo.toml`
    (CI runs this on every push/PR to `main`).
- **There is no JavaScript test suite.** No Vitest, no Playwright, no
  `tests/` directory, no `test:unit` / `test:e2e` / `pnpm test` scripts.
  Do not cite them in docs or CI. If a JS suite is added one day, this
  section must be updated with it.

### Mandatory operations rules
1. **Running the tests before asking for verification is mandatory**
   - When you have changed code (`.rs`, `.html`, `.css`), **always run
     `cargo test` and confirm that all tests pass** before asking the user
     to verify behavior. Rust tests cannot run in the sandbox (no cargo)
     — say so and rely on CI instead of claiming them.
2. **Adding a regression-prevention test when fixing a bug**
   - When fixing a new defect or regression, as a rule add a test case
     that reproduces and verifies that defect to `src-tauri/src/`.
