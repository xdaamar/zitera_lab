# ZITERA CYBER_LAB // AI AGENT HANDOFF BRIEF (START HERE)

> **Purpose:** This folder (`handoff/`) contains the complete, token-efficient state of **ZITERA CYBER_LAB** up to commit `e989dff` (2026-11-25) so any successor AI agent (e.g., Claude Code) can resume work immediately without re-scanning the repository.

---

## 1. Foundational Documentation Reference (`docs/`)

Do **NOT** duplicate or ignore the canonical project documentation already located in [`docs/`](../docs/):
- [`docs/architecture.md`](../docs/architecture.md) — Core layer separation, Rust IPC envelope, and Windows native sandboxed runtime (Job Objects, low-privilege token, loopback-only sockets; zero Docker/WSL2).
  - *Important Delta:* `docs/architecture.md` mentions Flutter Desktop UI from early Phase 1. **Production UI is now Tauri v2 + React 19 + TypeScript + Vite + Tailwind CSS v4** located in `ui/tauri/`.
- [`docs/build.md`](../docs/build.md) — Prerequisites (Windows 10/11 x64, Rust MSVC, Node 18+, VS 2022 Build Tools) and build steps for `zitera-engine` and lab binaries (`A01`–`A10`, etc.).
- [`docs/security.md`](../docs/security.md) — Security boundaries, SSRF guards, Ed25519 verification, and sandbox guarantees.
- [`docs/storage-boundaries.md`](../docs/storage-boundaries.md) — Local vs. cloud state isolation.
- [`docs/lab-development.md`](../docs/lab-development.md) — Adding and structuring lab binaries and `catalog/catalog.json` / `manifest.json`.
- [`docs/code-signing.md`](../docs/code-signing.md), [`docs/distribution.md`](../docs/distribution.md), [`docs/release.md`](../docs/release.md) — Pre-publication packaging, signing, and release checklist.

---

## 2. Project Identity & Tech Stack

- **Product:** ZITERA CYBER_LAB — Offline-first, serverless-first Windows desktop cybersecurity training platform with native sandboxed lab binaries (`< 120 MB` RAM, `< 2s` startup).
- **Frontend (`ui/tauri/src/`):** React 19, TypeScript, Vite 8, Tailwind CSS v4, `lucide-react` icons, Tauri v2 APIs.
- **Desktop Shell (`ui/tauri/src-tauri/`):** Tauri v2 (`app.exe`), compiled and deployed to `ui/tauri/run/app.exe`.
- **Orchestration Engine (`engine/rust/`):** Native Rust CLI/IPC (`zitera-engine.exe`), also copied to `ui/tauri/run/zitera-engine.exe`.
- **Labs (`labs/` & `catalog/catalog.json`):** Precompiled native Windows lab executables (`F01`–`F06`, `A01`–`A12`, `W01`–`W05`, `D01`–`D02`).
- **Admin Portal:** `admin_portal.html` (manages user accounts, license tiers, and active durations).

---

## 3. Non-Negotiable Product & Engineering Constraints

1. **Zero Emojis:** Never use Unicode emojis anywhere in UI components, labels, logs, or user-facing strings. Use `lucide-react` vector icons exclusively.
2. **Provider Privacy:** Never mention internal cloud database provider names (specifically never expose `"TURSO"` in UI, telemetry, error messages, or docs). Use generic terms such as *"cloud database"* or *"secure cloud enclave"*.
3. **Pure Monthly Pricing (No Annual Plans):**
   - **Foundation Edition:** `Rp 70.000 / bln`
   - **PRO Specialist:** `Rp 270.000 / bln` (Credit Roll-Over from Foundation: `+Rp 200.000`)
   - **Enterprise VIP:** `Rp 470.000 / bln` (Credit Roll-Over from PRO: `+Rp 200.000`)
   - License active durations are managed from `admin_portal.html`, NOT via annual billing toggles in the app.
4. **All-in-One Tier Bundling:** Clue Ladder (Level 1–3), Tactical Mentor Chat, Labs, and Tools (Burp Lite / PoC modules) are bundled per tier — never sold as separate micro-transactions.
5. **Character Identity Separation:**
   - **Zeta** is strictly the **player/operative protagonist icon** (`AvatarRegistry.ts`). NEVER use Zeta as a mentor.
   - **Mentors** are **Chiyo** (Foundation/Socratic), **Kaede** (PRO/Exploit), and **Valerie** (VIP/White-Box & Bug Bounty), defined in `ui/tauri/src/mentorRegistry.ts`.
   - All UI labels must say **"Tanya Mentor"** (not "Tanya Zeta").
6. **No "Live CVE" Promises in VIP:** VIP must NOT promise monthly live zero-day CVEs. Instead, VIP delivers **Advanced Exploit Engineering (Python/Go scripts)**, **Elite Bug Bounty Automated PoC Suite**, **White-Box Code Audit & ISO 27001 Patching**, and **Ethical Rules of Engagement (RoE) / Safe Harbor**.
7. **No Freeform Chatbot Input:** `ZetaMentorModal.tsx` uses a **Finite Tactical Question Tree** (curated scrollable questions + variable-cadence typewriter + 2–3 branching layers + reset button), NOT a freeform text input that produces mismatched template replies.

---

## 4. Handoff Folder Index

1. [`00_START_HERE_AGENT_BRIEF.md`](./00_START_HERE_AGENT_BRIEF.md) — This brief, constraints, and `docs/` pointers.
2. [`01_CURRENT_STATE_AND_FILE_MAP.md`](./01_CURRENT_STATE_AND_FILE_MAP.md) — Exact file map and summary of all completed features up to `e989dff`.
3. [`02_PENDING_TASKS_AND_PRE_PUBLISH_ROADMAP.md`](./02_PENDING_TASKS_AND_PRE_PUBLISH_ROADMAP.md) — Remaining implementation tasks & pre-release polish checklist.
4. [`03_BUILD_TEST_AND_WORKFLOW_COMMANDS.md`](./03_BUILD_TEST_AND_WORKFLOW_COMMANDS.md) — Copy-pasteable PowerShell build, test, and verification commands.
