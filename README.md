# CareerOS

A single-file job application toolkit built for people applying to AI/SaaS companies in India, Singapore, Malaysia, and Indonesia with 0–1 year of experience.

## What it does

**CV Engine** — takes your base resume and a job description, generates a tailored CV prompt and cold-email draft using Gemini. No upload, no account, no tracking.

**Outreach Finder** — finds LinkedIn contacts at a target company (hiring managers, CSMs, team leads) and drafts a personalised referral message.

**Tracker Board** — kanban-style application tracker stored entirely in localStorage. One-click add from the Job Scanner.

**Job Scanner** — live scraper that hits Greenhouse, AshbyHQ, and LinkedIn's guest API directly from the browser. Filters to: last 24 hours, target geographies, entry-level titles only, no senior/lead/manager roles. No backend.

## Why this exists

Job boards are noise. Filtering for 0-exp roles at AI companies across four countries, posted in the last day, that actually match a specific set of titles — that's not something any single portal does well. This app does it in one click, then gives you everything you need to apply: a tailored CV prompt, a cold email, and a contact to send it to.

## Setup

Open `app/index.html` in a browser — or install the desktop/mobile app from [Releases](https://github.com/itsyashvardhan/careeros/releases).

Add your API keys via the settings icon (top right):
- **Gemini** — [aistudio.google.com](https://aistudio.google.com/) — free tier works
- **Tavily** — [app.tavily.com](https://app.tavily.com/) — free tier works

Keys are stored in `localStorage` only. Nothing is sent anywhere except the respective API endpoints.

## Auto-update

The desktop app checks for new releases on every launch and updates silently. To ship a new version: bump `version` in `src-tauri/tauri.conf.json` and push to `main`. GitHub Actions builds Windows, macOS (ARM + Intel), Linux, Android APK, and iOS IPA, publishes them as a GitHub Release, and all installed apps pick up the update automatically.

## Stack

- Frontend: single HTML file, vanilla JS, Tailwind CDN
- Desktop/mobile: Tauri v2 (Rust)
- AI: Gemini API (outreach generation)
- Search: Tavily API (contact discovery), Greenhouse/AshbyHQ public APIs, LinkedIn guest API
- Storage: localStorage only
