# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What This Is

CareerOS is a single-file job application toolkit for freshers applying to AI/SaaS companies in India and ASEAN. The entire frontend is `app/index.html` — one ~7300-line HTML file containing all CSS, JavaScript, and markup. This is intentional. The Tauri wrapper (`src-tauri/`) packages it as a desktop app.

## Commands

**Run locally (no build step):** Open `app/index.html` directly in a browser. All features work without the Tauri shell — the only things that require Tauri are `window.show()` focus fix on startup and the auto-updater (`window.__TAURI__` is gated everywhere).

**Build desktop app:**
```
cd src-tauri
cargo tauri build
```

**Run desktop app in dev mode:**
```
cd src-tauri
cargo tauri dev
```

**Regenerate signing keypair** (only needed once per machine):
```
cd gen_key
cargo run
# Writes keys to C:\Users\me\.tauri\careeros.key and careeros.pub.txt
```

## Shipping a Release

1. Bump `version` in both `src-tauri/tauri.conf.json` and `src-tauri/Cargo.toml`
2. Commit and push to `main`
3. Push a version tag: `git tag v1.x.x && git push origin v1.x.x`

The CI workflow (`.github/workflows/release.yml`) triggers on version tags only. It builds the NSIS installer, signs it, uploads it, and generates `latest.json` — which is what installed apps poll to detect updates.

**CI secrets required:** `TAURI_SIGNING_PRIVATE_KEY` (base64 of the `.key` file) and `TAURI_SIGNING_PRIVATE_KEY_PASSWORD` (empty string for this project).

## Architecture

### Frontend (`app/index.html`)

Everything lives in one file. The structure within the file:
- **Lines ~1–220**: CSS custom properties (theme, colors, typography)
- **Lines ~220–2000**: Component CSS (panels, cards, tracker table, modals, mascot, PWA banners)
- **Lines ~2000–3060**: HTML body — sidebar, desktop header, module tab nav, all 7 module views, modals (settings, add-entry), mascot SVG
- **Lines ~3060–3090**: `<script type="text/markdown">` — skill file context (not executed; read by `getElementById`)
- **Lines ~3098–6838**: Main `<script>` — all application logic
- **Lines ~6882–7052**: Second `<script>` — interview prep panel and salary panel logic
- **Lines ~7085–7270**: Third/fourth `<script>` — network graph (D3) and PWA service worker

### Module System

Seven modules share a single `<main>` area. `switchModule(name)` shows/hides `.module-view` divs and updates `.module-tab.active`. Module IDs: `prompt-engine`, `referral-outreach`, `app-tracker`, `job-search`, `job-scanner`, `skill-gaps`, `network-graph`.

### Data Flow

All persistence is client-side only:
- **localStorage keys**: `gemini_key`, `tavily_key`, `user_profile`, `applications_tracker` (JSON array), `bookmarked_contacts` (JSON array), `js_saved_jobs`, `skill_gap_status`, `draft_cv_text`, `draft_jd_text`, `draft_role_title`
- **IndexedDB**: Mirror backup via `idbSet`/`idbGet` (keys match localStorage). `restoreFromBackup()` runs at startup to recover from localStorage clears.

### Roles Database

`rolesData` (line ~3100) is a 17-entry array of target roles across three categories: `Product`, `GTM`, `Operations`. Each role has `id`, `title`, `category`, `catClass`, `suitability` (small/mid/large company fit with `yes`/`partial`/`no`), and `context` (passed verbatim into Gemini prompts). The currently selected role is tracked in `selectedRoleId`.

### API Calls

- **Gemini** (`geminiApiKey`): CV generation, rewriting, outreach drafting, job scoring, interview prep, salary intel, practice answer scoring. Model: `gemini-2.0-flash`.
- **Tavily** (`tavilyApiKey`): Contact/referral search, company intel, job search.
- **ATS direct**: `jsFetchGreenhouse()`, `jsFetchAshby()`, `jsFetchInternshala()` — no auth, public APIs, called from the browser.

### Tauri Backend (`src-tauri/src/lib.rs`)

Minimal Rust. Two responsibilities:
1. On `setup`: show/focus the window, then spawn a background task that calls `check_for_update()` and emits `"update-available"` with the version string if found.
2. `install_update` command: downloads and installs the update, then restarts.

The frontend listens for `"update-available"` via `window.__TAURI__.event.listen` and shows a toast with an "Update & Restart" button that invokes `install_update`.

### Auto-Updater

The updater polls `https://github.com/itsyashvardhan/CareerOS/releases/latest/download/latest.json`. The CI generates this file from the `.nsis.zip.sig` signature — **use `Where-Object { $_.Name -like "*.nsis.zip" }` not `-Filter "*.nsis.zip"`** in PowerShell; the `-Filter` API silently fails on double-extension filenames on Windows.

### Mascot

The pixel octopus (`#pixel-octopus-companion`) is an ambient AI companion. State machine: `idle → crawl → search/read/type`. `octopusActive` must be `true` and `startOctopusEngine()` must have been called for it to function. Both happen at `DOMContentLoaded`.

### Known Patterns

- **JS syntax in template literals**: Avoid literal newlines inside strings within template literals (e.g., in `split()` calls). Use `\n` escape sequences.
- **`<script type="text/markdown">`**: Used to embed the skill file context as a non-executed block; read via `document.getElementById("skill-file-content").textContent`.
- **`_ric(fn)`**: Wrapper for `requestIdleCallback` with a setTimeout fallback — used to defer non-critical initialization after first paint.
