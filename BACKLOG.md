# CareerOS Feature Backlog

_Last updated: 2026-05-27. Reflects post-v1.0.4 state._

---

## ✅ Shipped (v1.0.0 – v1.0.4)

| Feature | Module | Notes |
|---|---|---|
| CV/JD draft auto-persistence | CV Engine | localStorage on every keystroke |
| Job Match Score badges | Scanner | Gemini async batch, color-coded |
| Skills Gap Radar (per-job) | Scanner | missing_skills[] tags per card |
| Tracker notes + follow-up date | Tracker | Overdue highlighting in red |
| Company Intel panel | Scanner | Click company → Tavily + Gemini brief |
| Interview Prep panel | Tracker | Slide-in, Interview/Offer status only |
| Practice Mode in Interview Prep | Tracker | Toggle, per-Q textarea, Gemini scoring 1-5 |
| Follow-up email generator | Tracker | ✉ button, Gemini-drafted |
| Outreach Follow-Up Sequences | Outreach | Day 5 + Day 14, save all 3 to Tracker |
| Salary Benchmarking | Tracker | $ button on Offer/Interview → p25/median/p75 |
| Generate with Gemini (in-app) | CV Engine | SSE streaming, never leave app |
| Resume Section Rewriter | CV Engine | Third tab, side-by-side diff, Gemini |
| Saved Jobs panel + Use for CV | Scanner | Persistent, syncs to CV Engine |
| Apply Later bookmarks | Scanner | localStorage, toggle Save button |
| Skill Gap Dashboard | Module 6 | Aggregated missing skills, learning resources, status tracking |
| Referral Network Graph | Module 7 | D3 force-directed, click node → outreach |
| LinkedIn removed from scraping | Scanner | ATS-direct only |
| User Profile settings panel | Settings | Name, location, roles, skills, markets |
| Personalized header + onboarding | App shell | First-run toast, sidebar identity |

---

## P2 — Moonshots (remaining)

### 1. Auto-Fill Bookmarklet v2
**Why**: The bookmarklet captures jobs but doesn't fill ATS forms. 10-20 min per application is the biggest time sink.
**How**: Detect Greenhouse/Lever/Ashby form fields, fill from `user_profile`. Generate cover letter via Gemini inline. Requires Tauri `inject_js` or Chrome extension manifest — architecturally complex.

### 2. Live Job Stream (WebSocket polling)
**Why**: Jobs die in 48h. The current scan is manual. A live stream sorted by match score would be transformative.
**How**: Background Tauri task polling Greenhouse/Ashby APIs every 30 min. Emit new jobs to frontend. Show live counter in sidebar tab badge. Requires Rust changes.
