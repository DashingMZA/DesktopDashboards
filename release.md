# Release Guide — DesktopDashboards

This document covers the release process: the Windows desktop installer
(NSIS) and portable ZIP, built by GitHub Actions. This app is
Windows-only — there is no web deployment and no other platform.

## 1. Versioning

Keep these in sync before tagging a release (see `AGENTS.md` §4):

| File                        | Field                                      |
| --------------------------- | ------------------------------------------ |
| `package.json`              | `"version"`                                |
| `src-tauri/tauri.conf.json` | `"version"`                                |
| `src-tauri/Cargo.toml`      | `version` under `[package]`                |
| `README.md`                 | version badge (`vX.Y.Z`)                   |
| `CHANGELOG.md`              | entry for the version                      |

Current version is **0.1.0** and no release has been published yet —
the first release tags `v0.1.0`. (`docs/SPEC.md` has no version-history
section; do not cite a "SPEC §18".)

## 2. Desktop releases (GitHub Actions)

### How it works

`.github/workflows/release.yml` runs when you push a git tag starting with
`v` (e.g. `v0.1.0`), or manually via **Actions → Release → Run workflow**.
It builds on a single Windows runner:

- Windows (`x86_64-pc-windows-msvc`) → NSIS installer (`.exe`) + portable ZIP

The result is a **draft** GitHub Release containing the installer, the
portable ZIP, and the auto-updater artifacts (`latest.json` + signature
files).

Release assets:

- Installer: `DesktopDashboards_<version>_x64-setup.exe` (the versioned
  name Tauri produces, e.g. `DesktopDashboards_0.1.0_x64-setup.exe`)
- Portable ZIP: `desktop-dashboards-x64.zip`

The portable ZIP contains the built `.exe` plus `README.md` — nothing
else (this repo has no bundled asset files).

The in-app updater (`src-tauri/tauri.conf.json` → `plugins.updater`)
polls
`https://github.com/DashingMZA/DesktopDashboards/releases/latest/download/latest.json`,
so publishing the draft release is what ships the update to existing
desktop installs.

> **Note:** a fixed-name installer link (e.g.
> `releases/latest/download/DesktopDashboards_x64-setup.exe`) does NOT
> resolve for these releases, because the attached installer carries
> the version in its name. Renaming it to a fixed name is an open
> decision (see `AGENTS.md` §4) — do not promise fixed-name links
> until a rename step exists in `release.yml`.

### One-time setup: signing keys

The updater artifacts must be signed. Generate a keypair once on your own
machine (never commit these files):

```bash
pnpm tauri signer generate -w ~/.tauri/DesktopDashboards.key
```

This prints the **public key** — it must match the `pubkey` already stored
in `src-tauri/tauri.conf.json` (`plugins.updater.pubkey`). If you generate
a fresh keypair, update that field and commit it. The key files
(`DesktopDashboards.key`, `DesktopDashboards.key.pub`) are gitignored.

For local signed builds, export these as **real OS environment variables**
in the shell that runs the build (the Tauri CLI does not read `.env`
files). PowerShell example:

```powershell
$env:TAURI_SIGNING_PRIVATE_KEY="C:/path/to/your/DesktopDashboards.key"
$env:TAURI_SIGNING_PRIVATE_KEY_PASSWORD=""  # only if you set one
pnpm run build
```

(`TAURI_SIGNING_PRIVATE_KEY` accepts the key file path or the raw key
content.)

### One-time setup: GitHub Secrets

Repository → **Settings → Secrets and variables → Actions → New repository secret**:

| Secret                               | Value                                                  |
| ------------------------------------ | ------------------------------------------------------ |
| `TAURI_SIGNING_PRIVATE_KEY`          | Contents (or path contents) of the private `.key` file |
| `TAURI_SIGNING_PRIVATE_KEY_PASSWORD` | The password set at generation time (empty if none)    |

`GITHUB_TOKEN` is provided automatically; the workflow already requests
`permissions: contents: write` so it can create the release.

### Triggering a release

```bash
# 1. Make sure the working tree is clean and versions are bumped (see §1)
# 2. Commit and push to main
git push origin main

# 3. Tag and push the tag — this starts the workflow
git tag v0.1.0
git push origin v0.1.0
```

Then:

1. Watch progress under **Actions → Release**.
2. When the job finishes, open **Releases** — you'll find a **draft**
   release named `DesktopDashboards v0.1.0` with the installer and
   portable ZIP attached.
3. Review the artifacts, edit the release notes if needed, and click
   **Publish release**. Publishing makes `latest.json` live, which is what
   the auto-updater reads.

> **Note:** `pnpm install --frozen-lockfile` runs in the workflow, so
> `pnpm-lock.yaml` must be committed and in sync with `package.json`,
> otherwise the workflow fails before building.

## 3. Local build before tagging

Build locally and smoke-test before tagging (see `AGENTS.md` §4):

1. `pnpm run build` — produces the NSIS installer.
2. Verify the installer runs and the app launches, the tray icon
   appears, and the manager window opens.
3. Add a gadget (a URL and a local HTML file), toggle it on/off,
   and confirm it shows on the desktop, locks/unlocks, and survives
   a reload.
4. Confirm the gadget's external links open in the default browser,
   not inside the gadget.

## 4. Checklist before tagging

- [ ] Versions bumped in all four places (§1) + `CHANGELOG.md` entry
- [ ] `pnpm install` clean, `pnpm-lock.yaml` committed
- [ ] Rust tests pass (`cargo test`, also green in CI)
- [ ] Desktop smoke test via `pnpm dev` (if a Rust toolchain is available)
- [ ] Release notes drafted (what's new / fixed)
