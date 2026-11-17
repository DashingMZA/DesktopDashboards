# CLAUDE.md — DesktopDashboards (Claude Code operating manual)

Companion to `AGENTS.md` (shared rules) and `MEMORY.md` (project history).
Read all three before acting.

## A. Startup protocol

1. Read `AGENTS.md`, then `MEMORY.md` (check "Current Project State" and
   "In-Progress Work" — another session may have left work half-done).
2. Run `git status -sb` and `git log --oneline -5`. Note the branch
   (**`main`**) and any uncommitted changes — inspect them before editing
   anything nearby.
3. Check whether `MEMORY.md` already records the requested work (completed,
   in progress, or failed). Don't redo it blind.
4. Locate the implementation, configs, and any tests touching the task
   area (see directory map in `MEMORY.md` §3).
5. Confirm the package manager state: `pnpm --version` should be 12.19.0,
   `node --version` ≥ 24, `pnpm-lock.yaml` present and in sync
   (`pnpm install --frozen-lockfile` must succeed in CI).

## B. Project knowledge

- **What it is:** a lightweight Windows desktop gadget app. Tauri 2 +
  vanilla JS (no frontend framework; the frontend is a single static
  `src/index.html`). Windows-only. No backend, no DB, no auth of its own
  (gadgets share one WebView2 profile; logins live there).
- **Verified versions:** pnpm 12.19.0 · node ≥ 24 · Tauri 2 · Rust edition
  2024. Don't assume other versions; check `package.json` when in doubt.
- **Key files:** `package.json` (scripts),
  `src-tauri/tauri.conf.json` (installer metadata + updater),
  `src-tauri/src/main.rs` / `lib.rs` (entry point, windows, commands),
  `src-tauri/src/cli.rs` (CLI parsing), `src-tauri/src/config.rs`
  (config.json persistence), `src-tauri/src/widget.rs` (gadget windows),
  `src-tauri/src/tray.rs` (tray menu), `src-tauri/src/win32.rs`
  (Win32 pinning, Windows-only), `src/index.html` (manager window UI),
  `.github/workflows/release.yml` (tag-triggered Windows release),
  `docs/SPEC.md` (feature spec, Draft v0.3, single source of truth),
  `.env.example` (placeholders only).
- **Commands:**
  - `pnpm install` / `pnpm dev` (Tauri dev — launches the app) /
    `pnpm run build` (Tauri build, NSIS installer)
  - `cargo test --locked --manifest-path src-tauri/Cargo.toml`
    (Rust unit tests, 7 tests)
  - There is no JS test suite: no `test:unit`, no `test:e2e`, no
    `pnpm test`. No lint / typecheck / format scripts exist in this repo.
- **Environment:** copy `.env.example` → `.env` if needed (no app
  secrets). Tauri signing vars are local-only / GitHub Secrets, never
  committed.
- **Release:** push a `v*` tag (or `workflow_dispatch`) →
  `.github/workflows/release.yml` builds the Windows NSIS installer +
  portable ZIP + updater artifacts (`latest.json`) as a **draft** GitHub
  release. Requires `TAURI_SIGNING_PRIVATE_KEY` (+ optional password)
  secrets. See `release.md`.
- **Compatibility constraints:** pnpm strict resolution; Rust edition
  2024 (no `gen` identifiers, `unsafe extern "system"`); Win32 pinning
  code is `#[cfg(windows)]` and never compiles on Linux CI.

## C. Task execution protocol

1. Translate the request into concrete requirements + acceptance criteria.
2. Inspect current implementation and related configs/tests.
3. Identify risks, affected modules, and whether the change touches
   installer metadata, the release workflow, or the Win32 pinning code.
4. Plan proportionally to task size; for multi-step work, set milestones.
5. Implement with existing conventions (frontend in `src/`, backend in
   `src-tauri/`).
6. Review the full diff for regressions and unintended changes
   (`git diff --check` for whitespace).
7. Run relevant verification (`cargo test` at minimum for code changes;
   `pnpm run build` for release-affecting changes) and fix regressions
   you introduced. Note: Rust tests cannot run in the sandbox (no cargo)
   — rely on CI and say so instead of claiming them.
8. Update `MEMORY.md`: what changed, why, verification results, remaining
   risks/pending work. Checkpoint long tasks instead of waiting until the
   end.
9. Final report: files changed, behavior changes, exact commands run with
   outcomes, what was NOT verified, next recommended step.

## D. Truthful verification policy

Label every check exactly one of:

- **NOT RUN** — no verification performed (say why).
- **PASS** — command completed successfully (give the command).
- **FAIL** — command failed (give the command + error).
- **BLOCKED** — could not proceed; name the specific blocker.
- **PARTIAL** — only part of the required verification succeeded.

Rules: a running dev server ≠ a passing production build. A passing
CI run ≠ a verified installer — the installer is only proven by a
Windows build. `pnpm run build` failing means the installer was never
produced — say so. Never mark a task "fully verified" while required
checks are missing; list them as NOT RUN/BLOCKED instead.

## E. Memory maintenance policy

`MEMORY.md` is the durable record. At every meaningful checkpoint and on
task completion, record: objective, implementation + files changed,
decisions + rationale, exact commands run and outcomes, errors with
confirmed (or unconfirmed) root causes, unresolved issues/blockers,
pending tasks + next action, and current version/status changes. Append —
don't silently rewrite history; if an old entry is wrong, mark it
corrected and explain. Never invent timestamps, hashes, or results.

## F. Skills, subagents, and tool usage

- Do not assume any MCP server, plugin, or subagent exists — verify first.
- When delegating: hand over the task requirements, the relevant
  `MEMORY.md` entries, and require the delegate to report files
  inspected/changed, evidence-backed findings, checks actually run, and
  open issues. You reconcile and write the memory update.

## G. Git safety

- Branch is `main`. Never force-push or rewrite history without explicit
  user authorization. No `reset --hard`, no `clean -fdx` on the user's
  tree.
- Don't commit or push unless the user asked. If they did: single focused
  commits, and verify `git status` is clean of unintended files first
  (never `dist/`, `node_modules/`, `src-tauri/target/`, `.env`).
- **Workflow files cannot be pushed via the GitHub App** (403, no
  workflows permission) — they go via the owner's web-UI upload, verified
  byte-identical afterwards. Push-files is UTF-8 text only — no binaries.
- Record commit hashes only after they exist.
