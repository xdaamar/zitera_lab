# ZITERA_LAB — CURRICULUM ARCHITECTURE & LEARNING CONTRACT (V1)
**Document Reference:** `PRD/CURRICULUM_CONTRACT_V1.md`  
**Phase:** 1 (Checkpoint CP01)  
**Status:** CANONICAL EDUCATIONAL ARCHITECTURE SPECIFICATION  
**Target:** Windows x64 Native Sandboxed Runtime  
**Default Locale:** `id` (Bahasa Indonesia) | **Alternative Locale:** `en` (English)  
**Distribution:** Data-Driven Portable `.zlab` Packages (Ed25519 Signed)

---

## 1. Executive Summary & Educational Vision

ZITERA_LAB provides an offline-first, professional yet beginner-friendly cybersecurity laboratory environment on Windows.
The curriculum architecture is designed to teach real-world secure engineering, threat identification, and defensive remediation based on the **OWASP Top 10:2025** baseline.

The learning model enforces a **Dual-Track Pedagogy**:
1. **Track A — Guided Learning (Understand & Practice):** Designed for vocational students, junior software developers, and beginners. Guides learners step-by-step through real-world problems, intuitive analogies, controlled demonstrations, hands-on practice, and remediation.
2. **Track B — Independent Challenge (Investigate & Solve):** Designed for autonomous problem-solving and CTF-style assessment. Provides scenario context, scope boundaries, initial evidence, and tiered progressive hints without spoiling solutions or flags.

---

## 2. Core Hierarchy & Stable Identifiers

All educational content adheres to a strict, locale-invariant hierarchical identity model. Language selection (`id` or `en`) **never alters** content identity, progression state, validation logic, or flags.

```text
┌────────────────────────────────────────────────────────┐
│                        TRACK                           │
│     (TRACK-A: Guided Learning | TRACK-B: Challenge)    │
└──────────────────────────┬─────────────────────────────┘
                           │
                           ▼
┌────────────────────────────────────────────────────────┐
│                         LAB                            │
│           (A01, A02, A03, ..., A10)                    │
└──────────────────────────┬─────────────────────────────┘
                           │
                           ▼
┌────────────────────────────────────────────────────────┐
│                        LESSON                          │
│     (A01-L01: Overview & Concept, A01-L02: Attack)     │
└──────────────────────────┬─────────────────────────────┘
                           │
                           ▼
┌────────────────────────────────────────────────────────┐
│                        MISSION                         │
│  (A01-L01-M01: Inspect IDOR, A01-L01-M02: Fix Filter)  │
└────────────────────────────────────────────────────────┘
```

### 2.1 Identifier Grammar & Constraints
- **Track ID:** `TRACK-A`, `TRACK-B`, or `SHARED-FOUNDATION`.
- **Lab ID:** Canonical regex `^A(0[1-9]|10)$` (e.g. `A01`, `A02`, ..., `A10`).
- **Lesson ID:** `<LabID>-L<01..99>` (e.g. `A01-L01`, `A02-L03`).
- **Mission ID:** `<LessonID>-M<01..99>` (e.g. `A01-L01-M01`, `A05-L02-M03`).
- **Content Versioning:** Semantic versioning `MAJOR.MINOR.PATCH` (e.g. `1.0.0`).
- **Language Independence:** Identifiers are strictly ASCII alphanumeric. Neither the directory structure nor the code logic keys off translated terms.

---

## 3. Four-Tier Mission Progression Structure

Each lab module structures its missions across four clearly defined pedagogical tiers:

| Tier | Level Name | Focus & Description | Learner Deliverable |
| :--- | :--- | :--- | :--- |
| **Level 1** | **Foundation** | Concept recognition, mental analogy, real-world impact, and architectural boundaries. | Answering conceptual checks, recognizing vulnerable code patterns. |
| **Level 2** | **Guided Practice** | Step-by-step hands-on interaction on isolated loopback simulator (`127.0.0.1:<port>`). | Observing expected system outputs (HTTP status, error codes, response differences). |
| **Level 3** | **Applied Analysis** | Independent evidence inspection, log examination, payload adaptation, and root-cause analysis. | Identifying flaw mechanics, modifying inputs to confirm vulnerability hypotheses. |
| **Level 4** | **Advanced / Remediation** | Autonomous CTF scenario resolution and code-level remediation (patching + test verification). | Extracting dynamic completion flag and validating defensive code fix. |

---

## 4. Content Composition Target: 60% Manual / 40% Tools

To cultivate robust defensive engineering instincts rather than "script-kiddie" button clicking, ZITERA_LAB balances practice modes:

- **60% Manual Techniques:**
  - Reading raw HTTP request and response headers (`curl -v`, in-app inspector).
  - Inspecting application state transitions, session tokens, and parameters.
  - Analyzing server-side source code snippets and identifying insecure assumptions.
  - Formulating hypotheses and crafting surgical inputs manually.
- **40% Assisted Tools:**
  - In-app HTTP request clients and parameter fuzzers.
  - Guided enumeration scripts and schema inspection tools.
  - Static security analysis linters and automated regression checks.
  - **Tool Rule:** Lessons must explain *what* the tool does under the hood, *why* its output is significant, and *how* to confirm findings manually.

---

## 5. Completion Flag & Verification Contract

1. **Flag Format:** Strict canonical regex `^ZITERA\{[a-zA-Z0-9_\-]{8,64}\}$` (e.g., `ZITERA{b10k3n_4cc355_c0ntr01_m45t3r}`).
2. **Determinism:** Each challenge evaluates against an unambiguous, reproducible objective state in the sandboxed target.
3. **Locale-Invariance:** The flag string is **identical** whether the learner operates in Indonesian (`id`) or English (`en`).
4. **Anti-Leak Assurance:**
   - Flags are NEVER hardcoded into public educational text (`lesson/*.md`), introductory guides, or early hint tiers (`hints.json` tier 1–2).
   - Lessons and UI progress records must NEVER persist the cleartext flag string.
5. **Grader Verification:** Flag evaluation is executed exclusively by `zitera-engine lab validate-challenge <id> <flag>` using constant-time string comparison (`timing_safe_compare`) to prevent timing side-channel leaks.

---

## 6. Engine Integration & Architectural Boundaries

ZITERA_LAB maintains a strict architectural separation of concerns:

```text
┌─────────────────────────────────────────────────────────────────┐
│                     PRESENTATION LAYER (UI)                     │
│  - Flutter Windows Desktop Runner                               │
│  - Locale Selection ('id' default, 'en' alternative)            │
│  - Markdown Curriculum Renderer & CuteAnimeCursor               │
│  - Interaction-based Progress Tracking (ProgressManager)        │
└────────────────────────────────┬────────────────────────────────┘
                                 │ JSON IPC (stdio --json)
                                 ▼
┌─────────────────────────────────────────────────────────────────┐
│                       CORE SYSTEMS ENGINE                       │
│  - Rust Binary (zitera-engine.exe)                              │
│  - Zero-Daemon Native Windows Sandboxing (AppContainer Profiles)│
│  - Job Object Lifecycle Management (Kill-on-Close)              │
│  - Broker & Reverse Proxy (SSRF Guard, Security Headers)        │
│  - Cryptographic Package Verifier (Ed25519 dalek 3.0.0)         │
└────────────────────────────────┬────────────────────────────────┘
                                 │ Process Spawning & I/O
                                 ▼
┌─────────────────────────────────────────────────────────────────┐
│                     LAB TARGET RUNTIMES                         │
│  - Isolated Executables (a01-lab.exe ... a10-lab.exe)           │
│  - Bound exclusively to 127.0.0.1 (Loopback only)               │
│  - Contained within AppContainer DACLs & Job Object RAM Caps    │
└─────────────────────────────────────────────────────────────────┘
```

- **Zero Engine Changes:** Curriculum authoring, multi-language support, and lesson schemas do NOT alter the Rust systems engine, sandbox token generation, broker rules, or package verification protocols.
- **Port Allocation Invariant:**
  - `A01`: 8011 | `A02`: 8012 | `A03`: 8013 | `A04`: 8014 | `A05`: 8015
  - `A06`: 8016 | `A07`: 8017 | `A08`: 8018 | `A09`: 8019 | `A10`: 8020
