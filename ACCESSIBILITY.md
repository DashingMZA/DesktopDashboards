# Accessibility

This document describes what DesktopDashboards actually does for
accessibility, what it deliberately does not do, and how to verify it.
It is written against the code in this repo — every claim below names
the file that implements it.

## What is in place

### Keyboard operation

- **Real buttons and labels.** The manager window (`src/index.html`)
  uses real `<button>` elements for its main actions (Reload All, Add
  &amp; Show, Browse) and real `<label>` elements for the add-gadget
  form (Type, Title, URL or HTML file path), plus a real checkbox for
  "Launch at Windows startup" — all keyboard-operable by default.
  Each gadget card's toggles (Lock, Click-through, No focus steal),
  sliders (Opacity, Zoom), placement selector, and Custom CSS editor
  are built as real form controls (`input`, `select`, `textarea`) in
  the same file, so they are reachable by Tab and operable by keyboard.
- **Tray menu.** The tray menu (`src-tauri/src/tray.rs`) is a native
  Windows menu: Open Manager, per-gadget show/hide checkboxes, Lock
  All / Unlock All, Reload All, Launch at Windows startup, Quit — all
  operable by keyboard once the menu is open.

### Display

- **Opacity and zoom are adjustable per gadget.** Opacity runs
  20–100% and zoom 50–200% (sliders in the gadget card), so content
  can be made larger or more transparent to taste.
- **Settings persist.** Gadget position, size, opacity, zoom, and
  placement mode are saved to `config.json` and restored between
  launches, so a user's layout does not reset.

## Known limitations (honest list)

- **No ARIA attributes.** The manager window uses no `aria-*`
  attributes, landmarks, or `role` attributes, and no `tabindex`
  management. Screen-reader support is unmeasured.
- **No focus styling.** There is no `:focus` / `:focus-visible` rule
  in `src/index.html` — keyboard focus on controls has no visible
  outline beyond the browser/WebView default.
- **No reduced-motion setting.** There is no `prefers-reduced-motion`
  handling.
- **No text alternative for gadget content.** A gadget is a live web
  page in a WebView; its accessibility depends entirely on the page
  it loads.
- **Gadget windows are frameless.** They have no title bar or standard
  window chrome; they are moved by dragging (the hover action bar when
  unlocked) and controlled through the manager window and tray menu.
  A locked gadget's hover bar is hidden by design.
- **Click-through and no-focus-steal modes hide the gadget from
  input.** When click-through is on, mouse events pass through the
  gadget to the desktop behind it; when no-focus-steal is on, clicks
  do not take keyboard focus. Both are per-gadget opt-ins, and both
  can be turned off from the manager window or tray menu.
- **No automated accessibility audit has been run.** No axe / Lighthouse /
  screen-reader pass exists. The claims above are code-verified, not
  tool-verified.

## How to verify

1. `pnpm dev`, then Tab through the manager window with a keyboard:
   every control is reachable and operable.
2. Add a gadget, open its settings, and change opacity and zoom with
   the keyboard alone.
3. Turn on Lock for a gadget and confirm its hover bar disappears;
   turn it off from the manager window without touching the gadget.
4. Run a screen-reader pass (e.g. NVDA on Windows) against the built app
   before claiming screen-reader support.
