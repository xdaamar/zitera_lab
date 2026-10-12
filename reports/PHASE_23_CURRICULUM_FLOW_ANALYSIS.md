# PHASE 23: LEARNING FLOW ANALYSIS & CURRICULUM ISSUE REGISTER
**Document Reference:** `reports/PHASE_23_CURRICULUM_FLOW_ANALYSIS.md`  
**Phase:** 23 (CP04 — Learning Flow Map & Curriculum Issue Register)  
**Target Audience:** Project Architect & Curriculum Owner  
**Author:** AI Pair Programmer (Software Craftsmanship Active)  
**Date:** 2026-10-12  
**Status:** COMPLETE (AWAITING OWNER REVIEW & DECISION)

---

## 1. Executive Summary

This report analyzes the pedagogical coherence, cognitive load progression, and practical mechanics of the 10 curriculum labs in `ZITERA_LAB`. Based on empirical findings from CP03, this analysis isolates curriculum defects, identifies prerequisite gaps, and proposes a restructured learner roadmap.

> **CRITICAL ARCHITECTURAL BOUNDARY:**  
> In strict compliance with Phase 23 mandates, **no curriculum files have been modified or reordered**. Stable package IDs (`zitera-lab-a01` .. `a10`), challenge validation logic, and Ed25519 signatures remain intact. The recommendations below represent decision options submitted for owner approval before any content restructuring in Phase 24.

---

## 2. Current User Learning Flow (Observed Implementation)

In the current desktop application (`ui/flutter/lib/features/labs/lab_detail_view.dart`), learner progression follows a linear 3-stage model per lab:

```text
┌─────────────────────────────────────────────────────────────┐
│                      1. LEARN MODE                          │
│  Overview -> Objectives -> Analogy -> Concept ->            │
│  Architecture -> Walkthrough -> Remediation                 │
└──────────────────────────────┬──────────────────────────────┘
                               │
                               ▼
┌─────────────────────────────────────────────────────────────┐
│                     2. PRACTICE MODE                        │
│  Interactive Embedded Sandbox (Native Port 8011..8020)      │
│  Live Vulnerable Target -> Exploit Exercise -> Defend Patch │
└──────────────────────────────┬──────────────────────────────┘
                               │
                               ▼
┌─────────────────────────────────────────────────────────────┐
│                    3. CHALLENGE MODE                        │
│  Capture The Flag Goal -> Tiered Hints (1, 2, 3) ->         │
│  Exploit Execution -> Flag Capture -> Completion Record     │
└─────────────────────────────────────────────────────────────┘
```

### Flow Strengths
- **Clear Modality Separation:** Distinct separation between passive conceptual learning (Learn), hands-on guided interaction (Practice), and unguided assessment (Challenge).
- **Progressive Hinting:** 3-tier hint unlocks (`hints.json`) prevent learners from getting completely blocked while discouraging instant spoiler consumption.
- **Immediate Local Feedback:** Practice modes run directly against daemonless sandboxed native processes without external server latency.

---

## 3. Educational & Structural Issue Register

| Issue ID | Affected Lab(s) | Observed Educational Condition | Cognitive Impact on Learner | Root Cause & Evidence | Priority |
|---|---|---|---|---|---|
| **CURR-23-01** | **A05** | Missing Architecture chapter in `labs/A05/lesson/` | Learner lacks defensive system diagrams explaining SQL parameterization boundary. | Only 10 files present (missing `architecture.md` / `architecture.id.md`). | **HIGH** |
| **CURR-23-02** | **A08, A09, A10** | Missing Indonesian translation for `architecture.md` | Non-English speakers experience sudden language fallback or broken layout in Architecture section. | Files `architecture.id.md` are absent in `labs/A08`, `A09`, `A10`. | **HIGH** |
| **CURR-23-03** | **Global** | Uniform 45-minute estimated duration across all labs | Inaccurate time expectations; A01 (IDOR) takes ~25 mins, whereas A06 (State Logic) & A10 (Fail-Open) require 60–75 mins. | Hardcoded `"estimated_minutes": 45` in all 10 manifests. | **MEDIUM** |
| **CURR-23-04** | **A03** vs **A05** | Premature placement of Software Supply Chain (A03) | Beginners are asked to parse AST scripts and package lockfiles before mastering simpler injection attacks. | A03 is marked "Beginner" and placed 3rd, before SQLi (A05) and Auth (A07). | **MEDIUM** |
| **CURR-23-05** | **A08** | Cryptographic prerequisite gap | A08 requires understanding asymmetric digital signatures and public keys without explicitly requiring A04 (Crypto). | Manifest prerequisites only declare `"Asymmetric cryptography concepts"` without formal module gating. | **MEDIUM** |
| **CURR-23-06** | **A01, A05** | Tier 3 hints are overly explicit | Reduces learner satisfaction; hints provide near-verbatim exploit strings rather than guiding methodology. | `hints.json` reveals exact parameter injection syntax in Tier 3. | **LOW** |

---

## 4. Lab-by-Lab Analysis & Recommendations

### Lab A01: Broken Access Control
- **Current Structure:** Invoicing portal with IDOR on `/invoice/:id`.
- **Observed Issue:** Excellent beginner introduction to authorization, but conceptual overlap with multi-tenant SaaS boundaries is introduced abruptly.
- **Recommended Remediation:** Keep as flagship beginner module; expand remediation section on attribute-based access control (ABAC).

### Lab A02: Security Misconfiguration
- **Current Structure:** Unauthenticated `/server-status`, directory browsing at `/backups/`, `.env.bak` exposure.
- **Observed Issue:** Highly intuitive and visual; would serve as a gentler first lab than A01 for absolute novices.
- **Recommended Remediation:** Suggest positioning as either introductory Lab #1 or maintaining current stable position with cross-links.

### Lab A03: Software Supply Chain Failures
- **Current Structure:** Package lockfile audit (`package_lock_audit.json`), tracking malicious dependency hooks.
- **Observed Issue:** High prerequisite burden (npm/cargo ecosystem, lockfile hashes, build lifecycle scripts).
- **Recommended Remediation:** Reclassify difficulty from "Beginner" to "Intermediate" (Est. 60 min).

### Lab A04: Cryptographic Failures
- **Current Structure:** Cracking unsalted MD5/SHA-1 hashes with dictionary lookup, migrating to Argon2/bcrypt.
- **Observed Issue:** Fundamental prerequisite for A08; highly educational and engaging.
- **Recommended Remediation:** Keep beginner classification; add visual explanation of cryptographic work factors.

### Lab A05: Injection
- **Current Structure:** SQL Injection in search inputs, `UNION SELECT` database exfiltration.
- **Observed Issue:** Missing `architecture.md` / `.id.md` files.
- **Recommended Remediation:** Author defensive Architecture chapter detailing SQL parsing engine boundaries and prepared statement AST compilation.

### Lab A06: Insecure Design
- **Current Structure:** Multi-step business logic and state machine bypass.
- **Observed Issue:** Complex workflow; learners often confuse design flaws with coding bugs.
- **Recommended Remediation:** Reclassify duration to 60 minutes; add state-diagram visual aid.

### Lab A07: Authentication Failures
- **Current Structure:** Login endpoint brute forcing without rate limiting.
- **Observed Issue:** Very accessible; pairs naturally with A02 and A01.
- **Recommended Remediation:** Ensure progressive delay calculations are illustrated with clear graphs.

### Lab A08: Software or Data Integrity Failures
- **Current Structure:** Ingestion of unsigned firmware/payloads.
- **Observed Issue:** Missing `architecture.id.md`; requires prior understanding of digital signatures from A04.
- **Recommended Remediation:** Author `architecture.id.md`; add cross-reference to A04.

### Lab A09: Security Logging & Alerting Failures
- **Current Structure:** Privilege elevation without audit log generation.
- **Observed Issue:** Missing `architecture.id.md`. Logging concepts feel dry unless connected to SIEM detection.
- **Recommended Remediation:** Author `architecture.id.md`; provide mock SIEM alert timeline in practice mode.

### Lab A10: Mishandling of Exceptional Conditions
- **Current Structure:** Fail-open exception handler in financial authorization guard.
- **Observed Issue:** Missing `architecture.id.md`. Learners struggle to induce the specific error trigger without clues.
- **Recommended Remediation:** Author `architecture.id.md`; clarify error injection vectors in walkthrough.

---

## 5. Recommended Pedagogical Flow (Curriculum Roadmap)

Without altering underlying stable package IDs (`zitera-lab-a01` .. `a10`), we recommend introducing **Two Guided Learning Tracks** in the Flutter user interface:

```text
================================================================================
  TRACK 1: DEFENSIVE FOUNDATIONS (BEGINNER)
================================================================================
  Step 1: Lab A02 — Security Misconfiguration      (Exposed endpoints, defaults)
  Step 2: Lab A07 — Authentication Failures        (Brute-force, lockout guards)
  Step 3: Lab A01 — Broken Access Control          (IDOR, object-level auth)
  Step 4: Lab A05 — Injection                      (SQLi syntax vs data bounds)
  Step 5: Lab A04 — Cryptographic Failures         (Hashing, salts, Argon2)

================================================================================
  TRACK 2: ARCHITECTURAL SECURITY & SYSTEM DEFENSE (INTERMEDIATE)
================================================================================
  Step 6: Lab A03 — Software Supply Chain Failures (Lockfiles, dependencies)
  Step 7: Lab A06 — Insecure Design                (State machine invariants)
  Step 8: Lab A08 — Software/Data Integrity        (Signatures, deserialization)
  Step 9: Lab A09 — Security Logging & Alerting    (Audit trails, detection)
  Step 10: Lab A10 — Mishandling of Exceptions     (Fail-closed boundary guards)
================================================================================
```

---

## 6. Owner Decision Boundary & Action Choices

Before proceeding with curriculum rewriting or future release builds, the project owner should review and select one of the following strategic paths:

- **OPTION A (Conservative / Minimal):**
  - Keep current A01–A10 sequential numbering exactly as-is.
  - Fix missing files in Phase 24: Author A05 `architecture.md` / `.id.md` and missing `architecture.id.md` in A08, A09, A10.
  - Adjust estimated durations in manifests to reflect realistic effort.
  
- **OPTION B (Curriculum Paths in UI - RECOMMENDED):**
  - Maintain underlying stable package IDs and signatures (`A01` .. `A10`).
  - Introduce "Track 1: Foundations" and "Track 2: Architecture" in the Flutter dashboard navigation.
  - Fill all missing architecture chapters and Indonesian translations.
  
- **OPTION C (Full Sequential Re-indexing):**
  - Re-number packages to match difficulty ordering (requires rebuilding catalog, resignings `.zlab` packages, and updating Rust tests).

> **Awaiting Owner Approval:** No files will be modified until the owner makes a decision.
