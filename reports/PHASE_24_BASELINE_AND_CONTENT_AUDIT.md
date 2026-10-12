# PHASE 24: BASELINE, REPOSITORY CONTRACT & CURRENT CONTENT AUDIT
**Document Reference:** `reports/PHASE_24_BASELINE_AND_CONTENT_AUDIT.md`  
**Phase:** 24 (CP00 — Checkpoint Push #1)  
**Execution Date:** 2026-10-12  
**Target Platform:** Windows x64  
**Author:** AI Pair Programmer (Software Craftsmanship Active)  
**Branch:** `migration/phase-24-curriculum-redesign-modular-architecture`  
**Baseline Git SHA:** `9a8eb08` (`docs(phase23): complete Phase 23 final release report and static verification`)  
**Repository Build Status:** `NOT REBUILT — AWAITING OWNER CONTENT APPROVAL`  

---

## 1. Executive Summary & Verification Baseline

Phase 24 establishes a comprehensive **Curriculum Redesign, Hands-on Cybersecurity Lab Architecture, and Modular Learning Contract** for ZITERA_LAB. This checkpoint (CP00) establishes the verifiable baseline, audits the actual current repository state following the Phase 23 artifact cleanup, verifies architectural contracts across Rust and Flutter codebases, and maps the foundation for subsequent checkpoints.

### 1.1 Repository State Verification
| Item | Inspected Value | Status |
|---|---|---|
| Repository Root | `c:\Users\Damar\Documents\project_pribadi\zitera_lab` | Verified |
| Active Branch | `migration/phase-24-curriculum-redesign-modular-architecture` | Clean & Dedicated |
| Remote Origin | `https://github.com/xdaamar/zitera_lab.git` | Synced |
| HEAD SHA | `9a8eb08` | Phase 23 Final Report Baseline |
| Working Tree | Clean (no untracked binaries or uncommitted modifications) | PASS |
| Binary Audit | 0 `.exe`, 0 `.dll`, 0 `.zip` in `dist/` or workspace root | PASS |
| Mandatory Stop-Build Rule | No compilation (`cargo build`, `flutter build`, release packaging) | STRICTLY ENFORCED |

---

## 2. Stable Identifiers & Contracts to Preserve

In strict accordance with Section 1.2 of the Phase 24 specification, existing stable IDs, port assignments, and package namespace conventions must be strictly preserved across all curriculum updates:

| Lab ID | Stable Package ID | OWASP Category | Default Port | Target Entrypoint | Runtime Type |
|---|---|---|---|---|---|
| `A01` | `zitera-lab-a01` | `A01:2025` | `8011` | `bin/a01-lab.exe` | `native_sandboxed` |
| `A02` | `zitera-lab-a02` | `A02:2025` | `8012` | `bin/a02-lab.exe` | `native_sandboxed` |
| `A03` | `zitera-lab-a03` | `A03:2025` | `8013` | `bin/a03-lab.exe` | `native_sandboxed` |
| `A04` | `zitera-lab-a04` | `A04:2025` | `8014` | `bin/a04-lab.exe` | `native_sandboxed` |
| `A05` | `zitera-lab-a05` | `A05:2025` | `8015` | `bin/a05-lab.exe` | `native_sandboxed` |
| `A06` | `zitera-lab-a06` | `A06:2025` | `8016` | `bin/a06-lab.exe` | `native_sandboxed` |
| `A07` | `zitera-lab-a07` | `A07:2025` | `8017` | `bin/a07-lab.exe` | `native_sandboxed` |
| `A08` | `zitera-lab-a08` | `A08:2025` | `8018` | `bin/a08-lab.exe` | `native_sandboxed` |
| `A09` | `zitera-lab-a09` | `A09:2025` | `8019` | `bin/a09-lab.exe` | `native_sandboxed` |
| `A10` | `zitera-lab-a10` | `A10:2025` | `8020` | `bin/a10-lab.exe` | `native_sandboxed` |

---

## 3. Actual Schema & Content Format Audit

### 3.1 Actual Manifest Schema (`LabManifest`)
Defined in `engine/rust/src/models.rs` and mirrored in `ui/flutter/lib/core/ipc/models.dart`:
```json
{
  "schema_version": 1,
  "id": "A01",
  "package_id": "zitera-lab-a01",
  "slug": "broken-access-control",
  "title": "Broken Access Control",
  "owasp": "A01:2025",
  "standard": "owasp-top10",
  "standard_version": "2025",
  "category_id": "A01",
  "category_name": "Broken Access Control",
  "version": "1.0.1",
  "security_version": 1,
  "minimum_core_version": "2.0.0",
  "difficulty": "Beginner",
  "runtime": "native_sandboxed",
  "entrypoint": "bin/a01-lab.exe",
  "default_port": 8011,
  "estimated_minutes": 45,
  "modes": ["learn", "practice", "challenge"],
  "short_description": "Insecure Direct Object References and multi-tenant boundary bypass.",
  "skills": ["Authorization Matrix", "IDOR Detection", "Parameter Tampering"],
  "learning_objectives": [
    "Identify flawed object-level access controls",
    "Audit multi-tenant boundary isolation",
    "Implement server-side authorization enforcement"
  ],
  "prerequisites": ["HTTP basics", "User session concepts"]
}
```
**Evaluation:** The schema is highly extensible and supports optional fields (`track_ids`, `tags`, `requirements`, `architecture`) without breaking backwards compatibility.

### 3.2 Actual Lesson Structure
Each lab directory (`labs/A01` through `labs/A10`) organizes educational text under `labs/A*/lesson/`:
- `introduction.md` / `introduction.id.md`: Lab orientation, real-world context, learning goals.
- `concept.md` / `concept.id.md`: Core technical vulnerability mechanics and definitions.
- `analogy.md` / `analogy.id.md`: Intuitive mental models for beginners and vocational students.
- `architecture.md` / `architecture.id.md`: System components, boundaries, and data flow.
- `walkthrough.md` / `walkthrough.id.md`: Guided step-by-step reproduction and observation.
- `remediation.md` / `remediation.id.md`: Secure implementation, defensive code patterns, and unit tests.

The engine's `labs::get_lab_content` reads all `*.md` files dynamically, stripping unwanted path separators, and returns them as a key-value dictionary to the Flutter frontend (`LabDetailView`).

### 3.3 Actual Challenge & Hint Schema
Located in `labs/A*/challenge/`:
- `challenge.md` / `challenge.id.md`: Mission briefing, scenario description, instructions.
- `hints.json` / `hints.id.json`:
```json
{
  "flag": "ZITERA{b10k3n_4cc355_c0ntr01_m45t3r}",
  "hints": [
    { "tier": 1, "type": "conceptual", "hint": "Consider how the server decides which invoice to render..." },
    { "tier": 2, "type": "technical_area", "hint": "Look at the numerical identifier in the invoice URL..." },
    { "tier": 3, "type": "investigation_direction", "hint": "Standard user invoices start at ID 1 and 2..." },
    { "tier": 4, "type": "near_solution", "hint": "Try accessing /invoice/42 while logged in as Alice." }
  ]
}
```

---

## 4. Security & Distribution Lifecycle Contracts

### 4.1 Flag Grader & Validation Contract
- **Engine Implementation (`engine/rust/src/labs.rs`):**
  - Flags submitted via `ZiteraEngineClient.verifyFlag(labId, flag)` are validated server-side by the Rust engine.
  - Flag comparison uses constant-time `timing_safe_compare` to prevent side-channel timing attacks.
  - In `labs::get_lab_content`, the engine explicitly parses `hints.json` and **strips the secret `"flag"` field**, ensuring only tier/type/hint text reaches the Flutter client.
- **Architectural Risk / Issue Noted:**
  - `hints.json` stores the raw plaintext flag on disk inside the lab bundle directory (`challenge/hints.json`). A student who explores the local filesystem can discover the flag without solving the challenge.
  - *Recommendation:* Transition flag validation to use salted SHA-256 digests (`flag_hash: "..."`) or challenge-derived tokens in future core iterations.

### 4.2 Package Verification & Download Contract
- **Verifier (`engine/rust/src/package/verifier.rs`):** Enforces an 11-point security gate before extracting any `.zlab` package:
  1. Package size boundary (`MAX_PACKAGE_SIZE = 100 MB`).
  2. ZIP archive central directory structure.
  3. Manifest schema validity.
  4. Stable lab ID validation (`A01`–`A10`).
  5. Allowed runtime (`native_sandboxed`, `mock`, `docker`).
  6. Architecture matching (`x86_64` or `any`).
  7. Maximum entry count limit (`MAX_ARCHIVE_ENTRIES = 1,000`).
  8. Path traversal and Zip-Slip prevention (no `..`, leading slashes, or drive letters).
  9. Deterministic content digest calculation.
  10. Package SHA-256 hash validation.
  11. Ed25519 asymmetric cryptographic signature verification with trusted release and dev public keys.
- **Installer (`engine/rust/src/package/installer.rs`):**
  - Atomic installation using temporary staging directories (`.staging_<uuid>`).
  - Automatic rollback on validation failure.
  - Path collision protection.

### 4.3 Catalog Contract (`catalog/catalog.json`)
- Maximum catalog size: 512 KB.
- Strict HTTPS enforcement for remote repository URLs.
- Dual-key Ed25519 signature verification (`catalog.signature`).
- Built-in `default_catalog()` fallback in Rust ensures offline first-run operation without requiring network connectivity.

### 4.4 Runtime Lifecycle & Isolation
- Each lab process is launched as an isolated native executable under Windows Job Objects:
  - `JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE`: Ensures lab processes are killed immediately when the engine closes.
  - `JOB_OBJECT_LIMIT_DIE_ON_UNHANDLED_EXCEPTION`: Prevents unhandled crash dialogs.
  - Process memory limit and active process limits enforced.
  - Execution uses `CREATE_NO_WINDOW | DETACHED_PROCESS`.
  - Process listens strictly on localhost (`127.0.0.1:<default_port>`).

---

## 5. Curriculum Gaps Identified from Phase 23 Audit

1. **Missing Documentation:**
   - Lab A05 (Injection) lacks `architecture.md` and `architecture.id.md`.
   - Labs A08, A09, and A10 lack Indonesian translations for `architecture.id.md`.
2. **Time Budget Inconsistency:**
   - All 10 manifests currently declare `estimated_minutes: 45`.
   - Fundamental labs (A01, A02) require ~30–40 minutes.
   - Complex architectural labs (A06 Insecure Design, A08 Deserialization, A10 Mishandling) require ~60–75 minutes.
3. **Cognitive Leap (Difficulty Spike):**
   - Sharp jump between A05 (direct SQL/XSS input injection) and A06 (complex multi-state invariant logic), as well as A08 (binary serialization tamper-proofing).
   - Solution: Establish the **Shared Foundation** modules and structured **Two-Track Learning Flow** (Track A: Web Security Fundamentals; Track B: Secure Systems Architecture).

---

## 6. Core Change Requests (CCR) Register

To avoid scope creep, any core engine or UI modifications requiring code refactoring outside curriculum authoring are logged here:

| CCR ID | Target Component | Proposed Change | Justification | Priority |
|---|---|---|---|---|
| `CCR-01` | `engine/rust/src/labs.rs` | Salted flag hashing in `hints.json` | Prevents local filesystem inspection of plaintext flags. | Medium (Post-Phase 24) |
| `CCR-02` | `ui/flutter/lib/core/i18n/` | Dynamic `.id.md` fallback preference | Automatically selects Indonesian markdown when app language is `id`. | High (Phase 24 CP06) |
| `CCR-03` | `catalog/catalog.json` | Add `track_ids` array to catalog schema | Allows filtering catalog by Track A vs Track B. | High (Phase 24 CP01) |

---

## 7. Next Actions (CP01)
- Author `PRD/CURRICULUM_ARCHITECTURE_V1.md` establishing the canonical two-track curriculum architecture, shared foundation micro-lessons, and learning objectives.
- Commit: `docs(phase24): define canonical curriculum architecture and learning paths` (Push #2).
