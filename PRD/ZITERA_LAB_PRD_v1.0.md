# ZITERA_LAB — Product Requirements Document (PRD) v1.0

> **Document status:** Foundation / Source of Truth  
> **Product:** ZITERA_LAB  
> **Primary platform:** Windows Desktop  
> **Primary audience:** Beginner–intermediate learners of web application security  
> **Core curriculum baseline:** OWASP Top 10:2025  
> **Architecture principle:** Flutter UI + lightweight Rust engine + isolated lab runtime  
> **Rule:** All future implementation sprints must follow this document unless an explicit Architecture Decision Record (ADR) changes it.

---

## 1. Product Vision

ZITERA_LAB is a Windows desktop application designed as a practical cyber security learning laboratory.

The product combines:

- structured cyber security learning materials,
- interactive vulnerable web application labs,
- guided practice,
- CTF-style assessments,
- local lab environments,
- a smart system/tool setup assistant,
- tool detection and version checking,
- GitHub-based lab distribution,
- local progress tracking,
- and a lightweight built-in CLI/engine.

The product should feel like a real **personal cyber security laboratory**, not like an ebook, terminal wrapper, or generic admin dashboard.

### Core experience

The intended learning flow is:

```text
Learn
  ↓
Understand
  ↓
Practice
  ↓
Experiment
  ↓
Test / CTF
  ↓
Review
  ↓
Progress
```

---

# 2. Product Goals

## 2.1 Primary Goals

1. Make web security concepts understandable to learners without requiring advanced security knowledge.
2. Turn each security topic into an isolated, reproducible local lab.
3. Provide guided tutorials without forcing the user to manually configure every dependency.
4. Provide a separate challenge/CTF mode where the solution is not directly explained.
5. Make system setup transparent, diagnosable, and reversible where possible.
6. Keep the desktop application lightweight and maintainable.
7. Allow new labs and lab updates to be delivered without rebuilding the entire application.
8. Keep the architecture modular so the number of labs can grow substantially.
9. Keep vulnerable environments isolated from the host as much as reasonably possible.
10. Prevent future AI-generated implementation work from drifting into unnecessary complexity.

## 2.2 Secondary Goals

- Provide a polished cyber-security identity.
- Make the application useful for classroom/self-study scenarios.
- Allow future expansion beyond OWASP Top 10:2025.
- Allow future lab categories such as API security, authentication, JWT, SSRF, client-side security, and secure coding.

---

# 3. Explicit Non-Goals for Version 1

The following are NOT part of the initial product unless explicitly added through an ADR:

- Cloud-hosted attack infrastructure.
- Remote penetration-testing targets.
- Publicly exposed vulnerable applications.
- Full enterprise LMS functionality.
- Social network features.
- Online multiplayer CTF infrastructure.
- Real-world target scanning.
- Automatic exploitation against arbitrary Internet targets.
- Silent installation of every security tool on the user's system.
- A massive plugin framework.
- Microservice architecture.
- Mandatory database server.
- Account/login system for the local application.
- Complex cloud synchronization.
- Native Linux desktop application as a first-release requirement.
- A custom browser engine.
- A custom virtualization engine.
- Reimplementing Docker/WSL/Git functionality.

Version 1 should remain focused on **local educational labs**.

---

# 4. Current OWASP Curriculum Baseline

ZITERA_LAB v1 uses **OWASP Top 10:2025** as its primary web application security curriculum baseline.

The ten categories are:

1. A01:2025 — Broken Access Control
2. A02:2025 — Security Misconfiguration
3. A03:2025 — Software Supply Chain Failures
4. A04:2025 — Cryptographic Failures
5. A05:2025 — Injection
6. A06:2025 — Insecure Design
7. A07:2025 — Authentication Failures
8. A08:2025 — Software or Data Integrity Failures
9. A09:2025 — Security Logging and Alerting Failures
10. A10:2025 — Mishandling of Exceptional Conditions

The application must not hard-code the curriculum architecture around exactly ten categories. OWASP categories are content, not the application's fundamental architecture.

The catalog must support adding future topics without requiring a core application rewrite.

---

# 5. Target Users

## 5.1 Beginner Learner

Knows basic programming/web concepts but has limited security knowledge.

Needs:

- simple explanations,
- analogies,
- visual examples,
- guided steps,
- safe experiments,
- clear feedback.

## 5.2 Intermediate Learner

Already understands HTTP, web applications, and basic Linux/tool usage.

Needs:

- less hand-holding,
- realistic scenarios,
- more investigation,
- harder challenges,
- fewer hints.

## 5.3 Instructor / Mentor

Needs:

- reproducible labs,
- deterministic reset,
- clear learning objectives,
- challenge mode,
- student progress visibility in future versions.

---

# 6. Product Personality

ZITERA_LAB should feel:

- technical,
- mysterious,
- serious,
- cyber-security oriented,
- modern,
- educational,
- polished,
- slightly intimidating,
- but still approachable.

It must NOT feel:

- excessively dark,
- neon cyberpunk,
- overloaded with green terminal effects,
- “AI-generated futuristic dashboard,”
- gamer UI,
- childish,
- or visually noisy.

---

# 7. Visual Design Direction

## 7.1 General UI

Use:

- clean layouts,
- generous whitespace,
- strong typography,
- restrained cyber-security accents,
- subtle grids/lines,
- small technical labels,
- controlled contrast,
- concise cards,
- clear status indicators.

The cyber identity should come from:

- typography,
- micro-details,
- iconography,
- patterns,
- technical metadata,
- subtle borders,
- terminal-inspired elements,

not from making the entire interface black and glowing.

## 7.2 Lab Interface

Lab pages should have an aesthetic educational feel.

Suggested visual structure:

```text
┌──────────────────────────────────────────────┐
│ A01 / BROKEN ACCESS CONTROL                  │
│                                              │
│ "Who are you allowed to access?"             │
│                                              │
│ ┌────────────┐  ┌─────────────────────────┐ │
│ │ Concept    │  │ Interactive Scenario    │ │
│ └────────────┘  └─────────────────────────┘ │
│                                              │
│ OBJECTIVE                                    │
│ ...                                          │
│                                              │
│ [Start Lab] [Open Target]                    │
└──────────────────────────────────────────────┘
```

Do not turn every lab into a large terminal window.

---

# 8. ZITERA_LAB Branding and Logo Requirements

The application name is:

**ZITERA_LAB**

## 8.1 Logo Direction

The logo must be primarily a **text-based wordmark**.

Visual inspiration:

- ASCII typography,
- distorted monospace,
- terminal/banner typography,
- horror/cyber text treatment,
- controlled glitch,
- technical warning-label character,
- mysterious security-tool identity.

The design should feel intimidating without becoming unreadable.

## 8.2 Restrictions

Do NOT use:

- generic shield logos,
- stock hacker silhouettes,
- padlocks as the main logo,
- generic skull icons,
- generic AI circuit brains,
- excessive gradients,
- 3D chrome gaming logos.

The identity should be recognizable primarily through the **ZITERA_LAB text treatment**.

## 8.3 Required Assets

Antigravity must prepare a reusable vector-first logo system:

```text
branding/
├── zitera_logo.svg
├── zitera_logo_dark.svg
├── zitera_logo_light.svg
├── zitera_wordmark.svg
├── zitera_wordmark.png
├── zitera_icon.png
├── zitera_splash.png
└── source/
```

If an ASCII-style appearance is used, the primary source should remain editable/vector-based where practical.

Required outputs:

- transparent background,
- light background version,
- dark background version,
- horizontal version,
- compact version,
- application icon version,
- installer/splash version.

---

# 9. High-Level System Architecture

The system is divided into four major layers:

```text
┌──────────────────────────────────────────┐
│              ZITERA_LAB UI               │
│                 Flutter                  │
└───────────────────┬──────────────────────┘
                    │
                    │ local process / JSON
                    ▼
┌──────────────────────────────────────────┐
│          ZITERA ENGINE / CLI             │
│                   Rust                   │
└───────────────────┬──────────────────────┘
                    │
       ┌────────────┼─────────────┐
       ▼            ▼             ▼
      Git          Docker        WSL2
       │            │             │
       └────────────┼─────────────┘
                    ▼
             Local Lab Runtime
                    │
                    ▼
              Browser / Tools
```

## 9.1 Flutter Responsibilities

Flutter is responsible for:

- UI,
- navigation,
- lab catalog presentation,
- lesson rendering,
- challenge interface,
- setup wizard,
- tool status,
- progress UI,
- settings,
- user feedback,
- logs visualization.

Flutter must not become the primary systems-management layer.

## 9.2 Rust Engine Responsibilities

Rust is responsible for:

- system checks,
- environment detection,
- Git operations,
- Docker operations,
- WSL inspection,
- process execution,
- tool version detection,
- lab lifecycle,
- lab installation,
- lab updates,
- lab reset,
- status checks,
- validation helpers,
- machine-readable CLI output.

The Rust engine must remain lightweight and modular.

## 9.3 Communication

Version 1 should prefer a simple process boundary:

```text
Flutter
  ↓
launch zitera-engine.exe
  ↓
command + JSON arguments
  ↓
JSON result
```

A permanent daemon is NOT required unless later evidence shows it is needed.

This keeps debugging and packaging simpler.

---

# 10. Recommended Technology Stack

| Layer | Technology |
|---|---|
| Desktop UI | Flutter / Dart |
| Core Engine | Rust |
| Desktop Target | Windows |
| Linux Environment | WSL2 |
| Lab Runtime | Docker |
| Lab Source | GitHub repositories |
| Lab Metadata | JSON |
| Content | Markdown/JSON + assets |
| Build/Release | GitHub Actions |
| Local progress | Lightweight local persistence |
| App communication | JSON over local process boundary |

The first release should target Windows. Linux desktop support can be considered later.

Docker Desktop on Windows can use the WSL2 backend, and WSL2 provides a full Linux kernel environment for running Linux distributions. This architecture is therefore aligned with the intended Windows-first + Linux-lab workflow.

---

# 11. Smart Setup System

The Setup system is a first-class feature.

It must not simply show:

> Install everything.

Instead it must perform:

```text
Detect
  ↓
Classify
  ↓
Explain
  ↓
Recommend
  ↓
Install / Manual Setup
  ↓
Recheck
  ↓
Validate
```

## 11.1 Environment Checks

At minimum:

- Windows version,
- architecture,
- CPU virtualization availability,
- RAM,
- disk space,
- internet connectivity,
- PowerShell,
- Git,
- WSL,
- WSL version,
- detected Linux distributions,
- Docker,
- Docker daemon,
- required ports,
- required folders/permissions.

## 11.2 Status Classes

Use:

```text
READY
WARNING
MISSING
OUTDATED
BLOCKED
MANUAL ACTION REQUIRED
```

Example:

```text
Docker
Installed: YES
Version: 4.x
Engine: RUNNING

Status: READY
```

Or:

```text
WSL2
Installed: YES
Version: 1.x

Status: OUTDATED
Recommended: update WSL
```

## 11.3 Installation Policy

ZITERA_LAB must prioritize:

1. Detect first.
2. Prefer official package manager or official installer.
3. Provide manual installation instructions.
4. Re-run checks after installation.
5. Never assume installation succeeded.
6. Never silently run arbitrary remote scripts.
7. Never install security tools without clear user visibility.

## 11.4 Tool Profiles

Tools are not one giant dependency list.

Each tool has:

- ID,
- name,
- category,
- supported platform,
- execution environment,
- minimum version,
- detection command,
- update method,
- official source,
- privilege requirement,
- installation guidance.

The engine should be able to say:

```text
Nmap
Windows → installed
WSL     → installed
Docker  → not required
```

---

# 12. Tool Manager

The Tool Manager must support:

```text
detect
version
compare
health check
install guidance
update guidance
recheck
```

Example status:

```text
Nmap
Installed: YES
Version: 7.xx
Latest: 7.yy
Status: UPDATE AVAILABLE

[View Update Guide]
```

The application should NOT force users to install every pentesting tool globally.

Tools should be assigned to a lab or environment where practical.

---

# 13. Lab System

Every lab is independently versioned.

Recommended conceptual repository layout:

```text
github.com/<owner>/zitera-lab-a01
github.com/<owner>/zitera-lab-a02
github.com/<owner>/zitera-lab-a03
...
```

Each lab repository follows a predictable contract.

## 13.1 Standard Lab Repository

```text
zitera-lab-a01/
│
├── manifest.json
├── lesson/
│   ├── introduction.md
│   ├── concept.md
│   ├── analogy.md
│   ├── walkthrough.md
│   └── remediation.md
│
├── challenge/
│   ├── challenge.md
│   ├── hints.json
│   └── validator/
│
├── docker/
│   ├── Dockerfile
│   ├── compose.yml
│   └── config/
│
├── assets/
│   ├── images/
│   └── diagrams/
│
└── README.md
```

This structure is a contract, not a reason to create unnecessary files.

---

# 14. Lab Manifest

Every lab MUST contain machine-readable metadata.

Example:

```json
{
  "id": "A01",
  "slug": "broken-access-control",
  "title": "Broken Access Control",
  "owasp": "A01:2025",
  "version": "1.0.0",
  "difficulty": "beginner",
  "runtime": "docker",
  "entrypoint": "docker/compose.yml",
  "defaultPort": 8011,
  "estimatedMinutes": 45,
  "modes": [
    "guided",
    "challenge"
  ]
}
```

The exact schema may evolve, but the separation between machine metadata and human content must remain.

---

# 15. Lab Lifecycle

The Rust engine must expose a consistent lifecycle:

```text
discover
install
verify
start
status
stop
reset
update
remove
```

From the user's perspective:

```text
Not Installed
     ↓
Install
     ↓
Installed
     ↓
Start
     ↓
Running
     ↓
Stop
     ↓
Reset / Update
```

The UI must not expose low-level Git/Docker complexity unless the user opens an advanced/debug view.

---

# 16. One-Click Lab Setup

The user should be able to open:

```text
A01 — Broken Access Control
```

and see:

```text
ENVIRONMENT

Git       ✓
Docker    ✓
WSL2      ✓

LAB

Not installed

[ SET UP LAB ]
```

The button performs:

```text
check dependencies
↓
download lab
↓
verify manifest
↓
prepare runtime
↓
start lab if requested
↓
health check
↓
READY
```

The user should not have to manually run Git commands for normal usage.

---

# 17. GitHub Distribution Strategy

GitHub remains the source of truth for lab repositories.

Version 1 should support shallow/tagged cloning where practical:

```text
git clone --depth 1 --branch <version> ...
```

This reduces unnecessary download size compared with full repository history.

The implementation should abstract the distribution method so future releases can switch or supplement Git cloning with release artifacts or container registries without changing the Flutter UI contract.

The UI should always expose the concept:

```text
Install Lab
Update Lab
```

rather than:

```text
git clone
git pull
checkout branch
```

---

# 18. Central Lab Catalog

A central catalog repository should exist:

```text
zitera-lab-catalog/
└── catalog.json
```

Example:

```json
{
  "schemaVersion": 1,
  "labs": [
    {
      "id": "A01",
      "title": "Broken Access Control",
      "repository": "owner/zitera-lab-a01",
      "version": "1.0.0"
    }
  ]
}
```

Purpose:

- add new labs without rebuilding the app,
- update lab versions,
- change repository locations,
- enable remote catalog refresh,
- support future categories.

The Flutter app should not hard-code every lab URL.

---

# 19. Learning Experience

Each lab should have three primary modes.

## Mode A — Learn

Contains:

- introduction,
- analogy,
- concept,
- vulnerable behavior,
- real-world impact,
- how to identify it,
- how to fix it.

## Mode B — Practice

Contains:

- guided scenario,
- clear objective,
- environment,
- tasks,
- observations,
- commands/tools where relevant,
- progressive explanation.

The learner should understand **why** an action works, not merely copy commands.

## Mode C — Challenge / CTF

No full tutorial.

Provide:

- scenario,
- objective,
- target,
- constraints,
- bug category,
- clues,
- optional progressive hints,
- flag or validation target.

Do not reveal the exact exploit path by default.

---

# 20. Analogy System

Every major concept should have a memorable analogy.

Example pattern:

```text
Technical concept
        ↓
Everyday analogy
        ↓
What goes wrong
        ↓
How the lab demonstrates it
        ↓
How developers prevent it
```

Analogies must simplify the idea without teaching an incorrect mental model.

Avoid childish or overly comedic analogies.

---

# 21. Challenge System

Each challenge has:

```text
ID
Title
Difficulty
Objective
Scenario
Target
Clues
Hints
Validation method
Reset behavior
Completion state
```

Example:

```text
MISSION

A test account can access resources it should not own.

OBJECTIVE

Find the security weakness and retrieve the challenge flag.

CLUES

• Multiple user accounts exist.
• Requests contain object identifiers.
• The application may trust user-controlled identifiers too much.

[Launch Lab]
[Submit Flag]
[Hint 1]
```

The challenge system should support multiple validation strategies in the future.

---

# 22. Lab Reset

Every lab must have a deterministic reset strategy.

Reset should restore:

- container state,
- temporary data,
- seeded database,
- challenge state,
- required files,
- environment variables.

The user experience:

```text
[ RESET LAB ]

This will restore the lab to its initial state.
Your local challenge progress for this lab may be reset.

[Cancel] [Reset]
```

---

# 23. Local Security Boundary

ZITERA_LAB is an educational local-lab application.

Default lab behavior must prefer:

```text
localhost / 127.0.0.1
```

Lab services should not be exposed to the local network by default.

Lab containers must not receive unnecessary host filesystem access.

Avoid writable host bind mounts unless explicitly required.

Do not expose arbitrary vulnerable services publicly.

Do not allow a downloaded lab repository to silently modify unrelated host files.

Potentially destructive host operations require explicit user interaction.

---

# 24. Progress System

Version 1 requires only lightweight local progress.

Track:

```text
lab discovered
lab installed
lab completed
challenge completed
lesson completed
last opened
```

Optional later fields:

```text
time spent
attempt count
hints used
score
streak
badges
```

Do not introduce a remote account/database system in V1.

---

# 25. Core Application Screens

Minimum screen set:

### 1. Welcome / First Run

Shows:

- ZITERA_LAB identity,
- short description,
- system readiness,
- start setup.

### 2. Dashboard

Shows:

- learning progress,
- available labs,
- active lab,
- setup health,
- recent activity.

### 3. Labs

Shows:

- categories,
- difficulty,
- progress,
- installed state,
- update state.

### 4. Lab Detail

Shows:

- overview,
- objectives,
- requirements,
- learning modes,
- setup state,
- start button.

### 5. Learning

Shows:

- concept,
- analogy,
- visuals,
- interactive explanation,
- walkthrough.

### 6. Challenge

Shows:

- scenario,
- objective,
- clues,
- target,
- optional hints,
- flag submission.

### 7. Environment / Setup

Shows:

- system requirements,
- tools,
- WSL,
- Docker,
- Git,
- environment status.

### 8. Settings

Shows:

- lab storage location,
- update preferences,
- language later,
- logs/debug information,
- reset options.

---

# 26. Storage Model

Separate application data from lab data.

Conceptually:

```text
ZITERA_LAB
│
├── app
│   ├── config
│   ├── progress
│   └── logs
│
├── labs
│   ├── A01
│   ├── A02
│   └── ...
│
└── cache
```

The exact Windows directories should follow standard OS conventions and should not be hard-coded into the UI.

The lab storage path should eventually be configurable.

---

# 27. Update Architecture

There are two different update categories.

## 27.1 Core Application Update

Updates:

- Flutter application,
- Rust engine,
- core UI,
- internal contracts,
- setup engine.

This requires a new application build/package.

## 27.2 Content/Lab Update

Updates:

- lesson content,
- images,
- challenges,
- clues,
- lab code,
- Docker definitions,
- lab versions.

This MUST NOT require rebuilding the entire application.

Example:

```text
ZITERA_LAB 1.0.0
A01 1.0.0
A02 1.0.0
```

Later:

```text
ZITERA_LAB 1.0.0
A01 1.1.0  ← update only A01
A02 1.0.0
A03 1.0.0  ← newly added
```

---

# 28. Versioning Rules

Use semantic versions where practical:

```text
MAJOR.MINOR.PATCH
```

At minimum:

- application version,
- catalog schema version,
- lab version,
- manifest schema version.

A lab update must not silently break the engine contract.

The engine must validate compatibility.

Example:

```json
{
  "engine": {
    "minVersion": "1.0.0"
  }
}
```

---

# 29. CLI Requirements

The bundled CLI is called conceptually:

```text
zitera
```

It must support human-readable and machine-readable output.

Examples:

```text
zitera doctor
zitera lab list
zitera lab install A01
zitera lab start A01
zitera lab stop A01
zitera lab reset A01
zitera lab update A01
zitera tool status
```

Machine mode:

```text
zitera --json doctor
zitera --json lab status A01
```

The CLI is primarily a systems/control interface, not a replacement for the Flutter application.

---

# 30. Logging

The engine must produce useful structured logs.

Minimum log levels:

```text
INFO
WARNING
ERROR
DEBUG
```

Logs should include:

- timestamp,
- action,
- component,
- result,
- error details.

User-facing errors must be understandable.

Bad:

```text
Process exited with code 128.
```

Better:

```text
Lab download failed.

Git could not reach the repository.

Check:
1. Internet connection
2. Repository availability
3. Git installation

[Retry]
[View Details]
```

---

# 31. Error Handling Principles

Every major operation needs:

- progress,
- success state,
- failure state,
- retry option,
- human-readable explanation,
- technical details in debug view.

Operations must fail safely.

Never leave half-installed labs silently marked as ready.

---

# 32. Performance Requirements

The application should prioritize:

- fast startup,
- low idle memory usage,
- lazy loading,
- minimal embedded assets,
- no unnecessary background services,
- no permanent Rust daemon unless required,
- no large bundled lab datasets,
- no full repository history for labs where shallow retrieval is sufficient.

Large lab assets should remain outside the core application package.

---

# 33. Maintainability Requirements

Core components must be separated:

```text
UI
Engine
Lab Contract
Tool Definitions
Catalog
Content
```

Avoid putting system logic into Flutter widgets.

Avoid giant engine files.

Avoid generic abstractions before there is a real use case.

Prefer:

```text
small modules
clear interfaces
predictable contracts
explicit errors
```

---

# 34. Recommended Repository Structure

The main ZITERA_LAB repository should initially look approximately like:

```text
zitera_lab/
│
├── ui/
│   └── flutter/
│
├── engine/
│   └── rust/
│
├── prd/
│   ├── PRD.md
│   ├── architecture.md
│   ├── lab_specification.md
│   ├── security.md
│   └── decisions/
│
├── scripts/
│
├── installer/
│
├── catalog/
│
├── branding/
│
└── README.md
```

Additional directories should only be created when justified by a real requirement.

---

# 35. Future Lab Repository Structure

Each external lab repository should follow the shared contract:

```text
zitera-lab-a01/
│
├── manifest.json
│
├── lesson/
│
├── challenge/
│
├── docker/
│
├── assets/
│
└── README.md
```

The exact contents may vary by lab, but the top-level contract must remain understandable.

---

# 36. Development Rules for Antigravity

These rules are mandatory for future implementation sprints.

## Rule 1 — Read the PRD First

Before modifying the project, read:

```text
/prd/PRD.md
```

and any relevant supporting architecture documents.

## Rule 2 — Do Not Invent Architecture

Do not introduce:

- frameworks,
- databases,
- services,
- daemons,
- state managers,
- libraries,
- cloud services,

unless the current requirement justifies them.

## Rule 3 — Preserve the Boundaries

Maintain:

```text
Flutter = UI
Rust = system/engine
Docker/WSL = lab runtime
GitHub = lab distribution/source
Catalog = lab discovery
```

## Rule 4 — No Feature Creep

Do not implement future ideas during a sprint unless they are explicitly included in that sprint.

## Rule 5 — Prefer Simplicity

When two solutions work:

choose the simpler one that remains maintainable.

## Rule 6 — Test Real Behavior

Do not consider a system complete because code compiles.

For relevant sprints:

```text
build
run
test
lint
verify
```

must be performed.

## Rule 7 — Preserve Existing Work

Do not rewrite working architecture simply because another implementation looks cleaner.

## Rule 8 — Security Is a Requirement

Do not weaken lab isolation to simplify development.

## Rule 9 — Do Not Hide Errors

Errors must be logged and surfaced appropriately.

## Rule 10 — Document Architectural Changes

If a sprint requires changing a core architectural decision, create/update an ADR before implementing it.

---

# 37. Definition of Done for Every Sprint

A sprint is complete only when:

```text
[ ] Requirement implemented
[ ] Existing functionality preserved
[ ] Project builds successfully
[ ] Relevant automated checks pass
[ ] Manual behavior verified
[ ] Error paths tested where relevant
[ ] Security implications reviewed
[ ] Unnecessary code removed
[ ] Documentation updated where necessary
[ ] Git diff reviewed
[ ] Final implementation report written
[ ] Suggested commit message provided
```

Screenshots are NOT required in sprint reports unless specifically requested.

---

# 38. Acceptance Criteria for the V1 Foundation

The core architecture is considered correctly implemented when:

### Application

- Flutter Windows application launches.
- Navigation is stable.
- UI does not directly contain OS orchestration logic.

### Engine

- Rust engine builds.
- CLI launches independently.
- JSON output mode works.
- Flutter can invoke the engine reliably.

### Setup

- System requirements can be detected.
- Missing dependencies are distinguishable from outdated dependencies.
- The user receives actionable setup guidance.
- Checks can be repeated after installation.

### Lab System

- A lab can be discovered from the central catalog.
- A lab can be installed from its GitHub repository.
- A lab has a manifest.
- A lab can be started.
- A lab can be stopped.
- A lab can be reset.
- Lab status can be reported.

### Content

- A lab can contain Learn mode.
- A lab can contain Practice mode.
- A lab can contain Challenge mode.

### Updates

- Catalog updates do not require a new application build.
- Individual lab updates do not require rebuilding Flutter.
- Application updates remain separate from lab updates.

### Security

- Lab network defaults to local-only operation where possible.
- No arbitrary repository scripts are executed implicitly.
- Host filesystem access is minimized.
- Destructive system actions require explicit user interaction.

---

# 39. MVP Scope

The MVP should prove the architecture with:

```text
1 Flutter shell
1 Rust engine
1 Smart Setup implementation
1 catalog
1 complete lab
1 guided lesson
1 challenge
1 Docker-based runtime
1 update flow
1 reset flow
1 bundled CLI
1 branding system
```

Do NOT build all OWASP Top 10 labs before the platform architecture is validated.

A single excellent end-to-end lab is more valuable than ten incomplete labs.

---

# 40. First Reference Lab

The first reference lab should be:

```text
A01:2025 — Broken Access Control
```

Reason:

- clearly understandable,
- easy to demonstrate with a web application scenario,
- suitable for guided practice,
- suitable for CTF-style assessment,
- can demonstrate the full ZITERA_LAB architecture.

The first lab should become the reference implementation for future lab repositories.

---

# 41. Quality Bar

The target is:

> “A student can install ZITERA_LAB on a clean Windows machine, follow the setup guidance, install one lab with one button, understand the vulnerability through a guided lesson, launch the vulnerable environment locally, and complete a challenge without needing to understand Git, Docker, or WSL internals.”

The product should hide unnecessary infrastructure complexity while still exposing enough technical information for advanced learners.

---

# 42. Architectural Principle

The most important architectural principle is:

> **ZITERA_LAB is a learning platform first and a systems orchestrator second.**

The user should experience:

```text
Learn → Practice → Investigate → Solve
```

not:

```text
Install Git → configure WSL → debug Docker → clone repo → edit compose → fix PATH
```

Infrastructure complexity should exist behind the scenes.

---

# 43. Future Expansion Path

The architecture should allow:

```text
OWASP Web
    ↓
API Security
    ↓
Authentication / JWT
    ↓
Cloud Security Labs
    ↓
Secure Coding Labs
    ↓
Blue Team / Defensive Labs
    ↓
CTF Tracks
```

But these are future content expansions, not V1 implementation requirements.

---

# 44. Source of Truth Hierarchy

When future implementation decisions conflict, use this order:

```text
1. Security requirements
2. Core architecture
3. This PRD
4. Relevant ADR
5. Lab specification
6. Sprint task
7. Developer convenience
```

Developer convenience must never override security or architecture.

---

# 45. Final Product Statement

ZITERA_LAB is a lightweight Windows cyber-security learning laboratory that combines a polished Flutter desktop interface, a Rust systems engine, isolated Docker/WSL-based lab environments, GitHub-distributed modular labs, guided learning, and CTF-style assessment.

Its architecture deliberately separates:

```text
Application
Content
Engine
Runtime
Distribution
```

so that the product can evolve without repeatedly rebuilding the entire system.

The goal is not to create the largest cyber-security platform.

The goal is to create a **clean, maintainable, extensible, safe, and genuinely enjoyable local cyber-security lab**.

---

# 46. Official Baseline References

- OWASP Top 10:2025 — official project documentation.
- Flutter Windows Desktop Deployment documentation.
- Docker Desktop WSL2 documentation.

These sources define the current external baseline and should be rechecked when implementation depends on version-specific behavior.
