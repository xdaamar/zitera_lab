# 01 // CURRENT STATE & CODEBASE FILE MAP

**Git Head:** `e989dff` (`main`)
**Latest Release Binary:** `ui/tauri/run/app.exe` (built 2026-11-24 21:08 WIB via `cargo build --release`, 34,370,560 bytes)
**Build Status:** `tsc -b && vite build` (0 errors) | `scratch/test_ai_overview_paywall.ts` (79/79 passed)

---

## 1. Completed Milestones (Recent History)

### A. Detached Feature Windows (`featureWindows.ts`, `FeatureWindowRoot.tsx`, `LabDetailView.tsx`, `LabsView.tsx`)
- **Tanya Mentor** and **Burp Suite Lite** now open as real OS windows (`zitera-mentor`, `zitera-burp`) via `WebviewWindow`, not overlays inside the main UI. Each window has native decorations, its own taskbar entry, and can be dragged, resized, and moved to another monitor or virtual desktop.
- Window creation/focus/close is centralized in `ui/tauri/src/featureWindows.ts`; `FeatureWindowRoot.tsx` boots the correct panel by reading the current window label. `main.tsx` branches between the main app and feature windows.
- Lab context (lab id, title, target port, tier) is stored in Rust (`set_feature_window_context` / `get_feature_window_context`, `FeatureWindowsState`) and pushed to open windows through the `zitera:feature-context` event.
- Window lifecycle is reported to the main window through `zitera:feature-window` (open/close). The Rust builder listens for `WindowEvent::Destroyed`, so closing with the native X or `Esc` resets the toolbar `(opened)` state.
- **Cross-window payload injection:** "Kirim ke Burp Repeater" emits `zitera:inject-repeater` app-wide, clears the pending payload from `localStorage`, and opens/focuses the Burp window. A pending payload survives the window boot so injection is not lost when Burp is closed.
- Toolbar buttons in `LabDetailView.tsx` still show `(opened)` with pulsing dots and close the detached window on click; toast copy now says the window opens outside the main UI.
- In-app floating modals remain only as a fallback when native window creation is unavailable (browser preview / webview failure).

### B. Mentor Roster Overhaul & Zeta Separation (`mentorRegistry.ts`, `LabsView.tsx`, `ZiteraAiOverview.tsx`)
- **Zeta** is reserved exclusively as the player avatar (`AvatarRegistry.ts`).
- New cute anime mentor trio registered in `mentorRegistry.ts` (with backwards-compatible ID mapping `'zeta' -> 'chiyo'`, `'aoi' -> 'kaede'`, `'elysia' -> 'valerie'`):
  1. `chiyo` — Socratic Cyber Intel Mentor (Foundation / All Tiers)
  2. `kaede` — Offensive Web Exploitation Specialist (PRO / VIP; uses user-supplied cute pouty anime reference)
  3. `valerie` — Secure Architecture & Defensive Code Auditor (VIP)
- Assets located in `ui/tauri/public/assets/mentors/` (`{name}_avatar.png`, `{name}_teaching.jpg`, `{name}_praise.jpg`).
- UI labels standardized from "Tanya Zeta" to **"Tanya Mentor"**.

### C. Finite Tactical Question Tree & Clue Level 3 (`ZetaMentorModal.tsx`, `ZiteraAiOverview.tsx`)
- Eliminated freeform `<input type="text">` and form submission in mentor chat.
- Implemented scrollable curated tactical questions (`activeQuestions`), variable-cadence typewriter streaming (`18-32ms` + punctuation pauses + `[ Lewati Animasi ]`), 2–3 layer branching paths, and `[ Kembali ke Awal Investigasi ]` reset button.
- Replaced Clue Level 3 blurred payload teaser / P2W cheat with career assets: White-Box Source Code Anatomy, DevSecOps ISO 27001 Remediation Patch, and HackerOne Report Draft card.

### D. Monthly Pricing & Authoritative Lab Gating (`UpgradeModal.tsx`, `progressionHelper.ts`)
- Pure monthly pricing: Foundation (`Rp 70.000 / bln`), PRO Specialist (`Rp 270.000 / bln`), Enterprise VIP (`Rp 470.000 / bln`), plus `+Rp 200.000` Credit Roll-Over messaging.
-VIP bullets updated: removed "Live CVE", added Track 3 Advanced Exploit Engineering, Elite Bug Bounty Automated PoC Suite (Python/Go + AI guidance), and Ethical Safe Harbor / RoE.
- `canUserAccessLab` in `progressionHelper.ts`:
  - `trial` / `free`: F01–F06 + flagship A01 (IDOR)
  - `foundation`: 8 labs (F01–F06 + A01 + A02)
  - `pro`: All 40+ labs in Track 1, Track 2 (`A01`–`A10`), Track 3 (`W01`–`W05`, `D01`–`D02`, `A11`, `A12`)
  - `vip` / `enterprise`: All 60+ labs and advanced scenarios (`'vip'` included in `isProOrAbove`).

### E. Auto-Hiding Zoom Capsule & 12-Hour Persistent Auth (`ZoomControls.tsx`, `App.tsx`, `authService.ts`)
- Floating zoom pill (`[-] 130% [+]`) starts hidden on mount, appears only during zoom actions (`Ctrl + +/-/0/Wheel`), pauses hide timer on hover, and auto-hides after 2.5s. Duplicate `ZoomToast` removed from `App.tsx`.
- 12-hour persistent login sessions (`SESSION_MAX_INACTIVE_MS`, `SESSION_MAX_LIFETIME_MS`) implemented in `authService.ts`.

---

## 2. Key Frontend File Map (`ui/tauri/src/`)

| File | Responsibility | Notes |
| :--- | :--- | :--- |
| `App.tsx` | Root shell, routing, `useZoomManager`, `AuthDialog` | `ZoomToast` removed; uses `ZoomFloatingWidget` |
| `LabDetailView.tsx` | Main lab workspace, terminal, CTF submit, floating toolbar | Manages detached window state, `windowToast` |
| `featureWindows.ts` | Native window open/focus/close bridge, context + injection events | `zitera-mentor` / `zitera-burp` labels |
| `FeatureWindowRoot.tsx` | Boots Mentor/Burp panel inside its own OS window | Reads label, fetches lab context, `Esc` close |
| `useFeatureWindow.ts` | Parent hook for `(opened)` state, toggle, fallback | Reconciles on focus + lifecycle events |
| `LabsView.tsx` | Lab catalog browser, track filters, tier lock cards | Uses `progressionHelper.ts` |
| `ZetaMentorModal.tsx` | Tactical Mentor & Clue Ladder panel | `windowMode` fills its detached OS window; in-app drag only as fallback |
| `BurpLiteModal.tsx` | HTTP Repeater & History inspector | `windowMode` fills its detached OS window; listens to `zitera:inject-repeater` |
| `mentorRegistry.ts` | Mentor definitions (`chiyo`, `kaede`, `valerie`) | Legacy IDs auto-map safely |
| `zetaSocraticData.ts` | Socratic curriculum, clues, tactical question trees | Large data file (127 KB) |
| `fuzzySemanticEngine.ts` | Offline typo normalizer & AI Overview synthesizer | Used by `ZiteraAiOverview.tsx` |
| `ZiteraAiOverview.tsx` | Clue Level 1–3 structured overview & VIP career asset gate | Zero blurred cheats |
| `UpgradeModal.tsx` | 3-tier monthly pricing modal + WhatsApp concierge checkout | 70k / 270k / 470k + 200k roll-over |
| `progressionHelper.ts` | `canUserAccessLab`, `LAB_CATALOG_TITLES`, progression paths | Authoritative tier gating |
| `ZoomControls.tsx` | Native WebView2 zoom IPC + auto-hiding floating widget | 2.5s auto-dismiss + hover guard |
| `authService.ts` | Session management (12h expiry), tier & license checks | Syncs with cloud backend |
| `AvatarRegistry.ts` | 13 player operative avatars (includes `zeta`, `yuki`, etc.) | Strictly player icons only |
| `engineClient.ts` | IPC bridge to `zitera-engine.exe` | Handles lab start/stop/status/exec |
