# 02 // PENDING TASKS & PRE-PUBLICATION ROADMAP

This document lists the remaining work agreed upon in strategy discussions that has **not yet been fully implemented in code** before public launch.

---

## Priority 1: Two-Tier PoC Development Modules & Ethical Guardrails (Sprint 3 Remaining)

Currently, `UpgradeModal.tsx` advertises these features for PRO and VIP, so the in-app experience must match the promise 100%:

1. **PRO Tier — Standard Industry PoC Module:**
   - **Target files:** `ZetaMentorModal.tsx`, `zetaSocraticData.ts`, `LessonCards.tsx` (or a new `PocBuilderCard.tsx`).
   - Add an interactive or structured **Standard PoC Generator / Template** inside solved/active labs for PRO users:
     - Clean reproducible `curl` command (with exact headers, method, and cookie placeholders).
     - Raw HTTP Request / Response capture block (Burp Repeater format).
     - Basic Markdown vulnerability summary + CVSS 3.1 impact note.

2. **VIP Tier — Elite Bug Bounty Automated PoC Suite & AI Mentor Guidance:**
   - **Target files:** `ZetaMentorModal.tsx`, `ZiteraAiOverview.tsx`, `zetaSocraticData.ts`.
   - Upgrade VIP Clue Level 3 / PoC section beyond static snippets to include:
     - **Custom Python / Go Automated Exploit Script** walkthroughs (e.g., `requests` session + dynamic CSRF token parsing, `concurrent.futures` / `asyncio` race condition harness, Blind OOB DNS/HTTP exfiltration examples).
     - **Triage-Ready HackerOne / Bugcrowd Report Skeleton** (Executive Summary, Attack Narrative, CVSS 3.1 vector, Non-Destructive Proof, ISO 27001 / OWASP ASVS Remediation).
     - **VIP Mentor Guidance Mode:** Specialized tactical Q&A nodes in `zetaSocraticData.ts` explaining *why* exploit scripts are structured a certain way, how WAF filters reject payloads, and how to refine PoCs so triagers do not mark them `Informative` or `Needs More Info`.

3. **Mandatory Ethical Hacking & Rules of Engagement (RoE) Guardrails:**
   - Embed explicit ethical warnings inside all VIP advanced exploit modules:
     - Legal scope boundaries (UU ITE Pasal 30/31, CFAA, explicit written authorization requirement).
     - Safe non-destructive PoC verification standards (using `whoami`/`id` or benign proof instead of destructive queries).
     - Direct orientation toward legitimate Bug Bounty platforms (HackerOne, Bugcrowd, Intigriti, official VDPs).

---

## Priority 2: Floating Workspace & UX Polish

1. **Compact Responsive Layout Inside `ZetaMentorModal.tsx`:**
   - When floating at default width (`min(640px, 94vw)`), verify that the 2-column grid (`lg:grid-cols-12`) does not look cramped. Adding a clean tab switcher (`[ Chat Mentor ] | [ Clue Ladder & PoC ]`) when width is narrow (`< 800px`) will make floating multitasking even smoother.
2. **Global Keyboard Hotkeys (`LabDetailView.tsx`):**
   - Implement `Ctrl + M` to toggle Tanya Mentor floating window.
   - Implement `Ctrl + B` to toggle Burp Lite floating window.
   - Implement `Esc` to close the topmost active floating window.
3. **Operative Scratchpad / Quick Notes (Optional High-Value UX):**
   - A small collapsible tab or drawer in `LabDetailView.tsx` / `BurpLiteModal.tsx` allowing students to paste JWTs, discovered IDs, or flags during an investigation without opening external Notepad.

---

## Priority 3: Bespoke Mentor Artwork & Branding Cleanup

1. **Unique Artwork for Chiyo & Valerie:**
   - During the last session, the cloud image generator returned HTTP 503 capacity unavailable.
   - Currently:
     - `kaede_*` uses the user's exact uploaded cute pouty anime reference image.
     - `chiyo_*` reuses `girl_cheerful.png` / `aoi_*.jpg`.
     - `valerie_*` reuses `elysia_*.jpg`.
   - When image generation is available (or via external art), generate bespoke cohesive portraits and teaching/praise poses for Chiyo and Valerie (and additional poses for Kaede) in the Zitera cyber uniform.
2. **Optional Component Rename:**
   - `ZetaMentorModal.tsx` and `zetaSocraticData.ts` still carry `Zeta` in their filenames/props internally (`zetaMentorOpen`), though user-facing text now says `Tanya Mentor`. Refactor filenames only if convenient, or leave as-is to avoid churn.

---

## Priority 4: Pre-Publication Hardening & Release Packaging

1. **Repository & Log Hygiene:**
   - Review untracked files before release: `docs/WhatsApp Image 2026-11-21 at 8.44.02 PM.jpeg`, root `*.log` files (`engine_err.log`, `engine_out.log`, `stderr.log`, `stdout.log`, `test_err.log`, `test_out.log`), and ensure `ui/tauri/run/app.exe` and `zitera-engine.exe` are both freshly built from latest commits.
2. **Documentation Alignment:**
   - Update `docs/architecture.md` to replace legacy "Flutter Desktop UI" references with "Tauri v2 + React Desktop UI" if distributing docs publicly.
3. **End-to-End Verification & Installer Build:**
   - Run `dev_internal/scripts/verify_all.ps1`, `test_phase2_hardening.ps1`, `test_phase3_phase4.ps1`.
   - Build final installer (`npm --prefix "ui/tauri" run tauri -- build`) and perform code-signing/release steps per `docs/code-signing.md` and `docs/release.md`.
