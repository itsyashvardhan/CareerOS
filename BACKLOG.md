# CareerOS Feature Backlog

_Last updated: 2026-05-27. Reflects post-v1.0.2 state._

---

## ✅ Shipped (this sprint)

| Feature | Module | Notes |
|---|---|---|
| CV/JD draft auto-persistence | CV Engine | localStorage on every keystroke |
| Job Match Score badges | Scanner | Gemini async batch, color-coded |
| Tracker notes + follow-up date | Tracker | Overdue highlighting in red |
| Company Intel panel | Scanner | Click company → Tavily + Gemini brief |
| Interview Prep panel | Tracker | Slide-in, Interview/Offer status only |
| Follow-up email generator | Tracker | ✉ button, Gemini-drafted |
| Generate with Gemini (in-app) | CV Engine | SSE streaming, never leave app |
| Saved Jobs panel + Use for CV | Scanner | Persistent, syncs to CV Engine |
| Apply Later bookmarks | Scanner | localStorage, toggle Save button |
| LinkedIn removed from scraping | Scanner | ATS-direct only |
| User Profile settings panel | Settings | Name, location, roles, skills, markets |
| Personalized header + onboarding | App shell | First-run toast, sidebar identity |

---

## P0 — Build Next (highest impact, low effort)

### 1. Salary Benchmarking on Offer rows
**Why**: Freshers accept below-market offers because they have no reference. The offer stage is the highest-stakes decision in the whole flow.
**How**: On Tracker rows with "Offer" or "Interview" status, add a "Salary Intel" button. Tavily search for role + country + salary 2025, feed snippets to Gemini, return p25/median/p75. Cache by `role|country` 7 days. Show negotiation nudge if offer is below median.

### 2. Skills Gap Radar (per-job)
**Why**: The CV Engine shows gaps as raw text in a prompt — there's no persistent, structured view of what the user is missing across all their applications.
**How**: After a job match score runs, also extract a `missing_skills[]` array from Gemini. Show it as compact tags below the match score on each card: `Missing: Salesforce · SQL · stakeholder mgmt`. One extra field in the score response JSON, ~10 lines of render code.

### 3. Outreach Follow-Up Sequences
**Why**: Module 2 writes one message and stops. Most replies come from follow-ups. The feature is half-built — tracker has follow-up dates now, it just needs the message sequence.
**How**: After outreach generation in Module 2, show "Generate Day 5 + Day 14 Follow-ups" button. Gemini writes two variants. "Save to Tracker" button creates a row with all three messages in the notes field and follow-up date = today + 5.

---

## P1 — Next Sprint

### 4. Practice Mode in Interview Prep
**Why**: Reading questions isn't prep. Answering them is. The panel generates Q&A but doesn't let you practice.
**How**: Add a "Practice" toggle to the prep panel. Hide model answers, show a textarea per question. On "Reveal", show the model answer. Add a "Score my answer" button that sends user answer + model answer to Gemini and rates it 1-5.

### 5. Skill Gap Dashboard (Module 6)
**Why**: Fragmented skills intel per-job isn't enough — the user needs to see their top 3 gaps across all target roles to know what to study.
**How**: Aggregate `missing_skills[]` from all scored jobs. Frequency-rank them. Show top 10 as a ranked list with curated free resources (Coursera, freeCodeCamp, Google certs). Mark skills as "learning" / "done" — persisted to localStorage.

### 6. Resume Section Rewriter
**Why**: The CV Engine generates a master prompt to paste elsewhere. Users want the rewrite to happen in-app.
**How**: Third tab in Module 1 — "Rewrite". User highlights a bullet or section, Gemini rewrites it for the selected role. Side-by-side diff view. Already have Gemini key + streaming working.

---

## P2 — Moonshots

### 7. Auto-Fill Bookmarklet v2
**Why**: The bookmarklet captures jobs but doesn't fill ATS forms. 10-20 min per application is the biggest time sink.
**How**: Detect Greenhouse/Lever/Ashby form fields, fill from `user_profile`. Generate cover letter via Gemini inline. Requires Tauri `inject_js` or Chrome extension manifest — architecturally complex.

### 8. Live Job Stream (WebSocket polling)
**Why**: Jobs die in 72h. The current scan is manual. A live stream sorted by match score would be transformative.
**How**: Background Tauri task polling Greenhouse/Ashby APIs every 30 min. Emit new jobs to frontend. Show live counter in sidebar tab badge.

### 9. Referral Network Graph
**Why**: Referrals convert 4× better than cold applications. The outreach finder finds contacts but doesn't show the network structure.
**How**: Visualize bookmarked contacts as a force-directed graph (D3 or canvas). Group by company. Show connection strength (1st degree = solid, cold = dashed). Click node → open outreach panel.
