# ZITERA_LAB — MASTER LONG-RUN IMPLEMENTATION SPRINT

## Full Foundation → Flutter → Rust Engine → Smart Setup → Tool Manager → Lab Runtime → A01 + A05 → Catalog → Update System → Audit → Release

---

# 0. ROLE & EXECUTION MODE

You are the primary implementation agent for the ZITERA_LAB project.

Your job is not to produce a prototype quickly.

Your job is to build the project foundation correctly enough that future sprints can safely extend it without turning the codebase into AI-generated spaghetti.

This is intentionally a LONG-RUN SPRINT.

You are expected to work continuously through the phases below.

Do NOT prematurely stop after creating scaffolding.

Do NOT ask for confirmation between phases unless a genuinely unrecoverable external blocker makes implementation impossible.

When a reasonable implementation decision is not explicitly specified, choose the simplest maintainable option consistent with the PRD and architecture documents, record the decision in the final report, and continue.

Never fabricate successful results.

Never mark a phase complete merely because files were created.

Every phase must be actually built, tested, audited, debugged, and verified.

---

# 1. PRIMARY OBJECTIVE

Build the first serious working foundation of:

```text
ZITERA_LAB
```

Target:

```text
Windows
x64
```

Core architecture:

```text
Flutter Desktop
      │
      │ local process / JSON
      ▼
Rust Engine / CLI
      │
      ├── System Detection
      ├── Smart Setup
      ├── Tool Manager
      ├── Git Manager
      ├── Docker Manager
      ├── WSL Manager
      ├── Lab Manager
      ├── Catalog Manager
      └── Update Manager
               │
               ▼
       Docker + WSL2
               │
               ▼
         Local Labs
```

The first implementation milestone must prove that this architecture works end-to-end.

The first two labs are:

```text
A01:2025 — Broken Access Control
A05:2025 — Injection
```

A01 should serve as the primary reference implementation.

A05 should prove that the lab contract is reusable for a second security category.

---

# 2. ABSOLUTE SOURCE OF TRUTH

Before changing anything, locate and read:

```text
/prd/PRD.md
/prd/architecture.md
/prd/lab_specification.md
/prd/security.md
```

If filenames differ slightly, locate the correct documents inside `/prd`.

Do not blindly overwrite existing documentation.

These documents define the permanent product boundaries.

The hierarchy is:

```text
Security requirements
        ↓
Architecture specification
        ↓
PRD
        ↓
Lab specification
        ↓
Sprint requirements
        ↓
Developer convenience
```

Developer convenience NEVER overrides the first five.

---

# 3. GIT / BACKUP CONTRACT

The primary repository is:

```text
https://github.com/xdaamar/zitera_lab.git
```

Expected core repository:

```text
github.com/xdaamar/zitera_lab
```

Use the existing repository.

Do not create a second core repository.

The core project must remain one repository.

Only individual labs will eventually live in separate repositories.

Expected conceptual ecosystem:

```text
xdaamar/zitera_lab
xdaamar/zitera_lab_a01
xdaamar/zitera_lab_a05
```

Check:

```bash
git status
git remote -v
git branch
git log --oneline --decorate -10
```

If the repository has existing user work, PRESERVE it.

Never destroy existing work simply to make scaffolding easier.

---

# 4. GIT CHECKPOINT STRATEGY

The `main` branch is the stable checkpoint branch for this project.

Do not continuously push half-broken work.

After each successful major gate:

```text
implement
↓
audit
↓
fix
↓
re-audit
↓
test
↓
commit
↓
push
```

Use clear commits such as:

```text
chore: establish project foundation
feat: bootstrap flutter windows application
feat: establish rust engine foundation
feat: implement engine ipc contract
feat: implement smart environment diagnostics
feat: implement tool manager foundation
feat: implement lab catalog
feat: implement docker lab lifecycle
feat: implement a01 reference lab
feat: implement a05 reference lab
feat: implement lab update and reset flows
feat: implement production ui foundation
chore: establish release pipeline
```

Do not use meaningless messages like:

```text
update
fix
changes
final
done
```

Push every stable checkpoint.

If a checkpoint fails audit, DO NOT push it as a stable checkpoint until fixed.

Before every push:

```text
git diff
git status
git diff --check
```

Ensure no secrets, personal credentials, API keys, tokens, `.env`, private certificates, local machine files, build artifacts, or unrelated files are committed.

---

# 5. SKILL DISCOVERY — MANDATORY

Before implementation begins, inspect ALL currently available Antigravity skills.

Specifically look for and use the appropriate skills for:

```text
design-system
ui-styling
software-craftsmanship
ponytail-audit
Ponytail code-quality / efficiency skills
Ponytail performance skills
Ponytail architecture skills
Ponytail Flutter skills
Ponytail dependency skills
Ponytail security skills
Ponytail refactoring skills
testing skills
Git / repository skills
documentation skills
```

Do not assume only the explicitly named skills exist.

Search the currently available skill catalog.

For every major phase:

1. identify the relevant skills,
2. use them,
3. inspect their output,
4. implement improvements,
5. re-run audits where relevant.

If a relevant skill is not installed and the environment provides a way to install/search for it, install or enable the appropriate skill before that phase.

Do not install irrelevant skills merely for quantity.

Skill usage must produce tangible engineering value.

---

# 6. PONYTAIL GATE POLICY

Ponytail is NOT a final cosmetic audit.

Every major section must have:

```text
IMPLEMENT
↓
PONYTAIL AUDIT
↓
ANALYZE FINDINGS
↓
FIX FINDINGS
↓
PONYTAIL RE-AUDIT
↓
TEST
↓
CHECKPOINT
```

Minimum audit gates:

```text
GATE 0 — repository/foundation
GATE 1 — Flutter bootstrap
GATE 2 — UI architecture/design system
GATE 3 — Rust engine
GATE 4 — IPC
GATE 5 — Smart Setup
GATE 6 — Tool Manager
GATE 7 — Catalog/Lab contract
GATE 8 — Docker/WSL lab lifecycle
GATE 9 — A01
GATE 10 — A05
GATE 11 — Update/reset
GATE 12 — Performance/polish
GATE 13 — release readiness
```

If Ponytail finds unnecessary complexity:

simplify it.

If Ponytail finds duplicate code:

refactor it where safe.

If Ponytail finds expensive/redundant work:

optimize it.

If Ponytail identifies a genuine architectural issue:

fix it before continuing.

Do not ignore an issue simply because the application "works".

---

# 7. ANTI-OVERENGINEERING CONTRACT

This is critical.

Do NOT introduce:

```text
microservices
unnecessary daemons
heavy backend frameworks
remote databases
cloud infrastructure
authentication servers
plugin systems
event buses
complex dependency injection containers
massive state-management architecture
custom virtualization
custom container runtime
custom Git implementation
custom package manager
```

unless a real requirement proves one is necessary.

Prefer:

```text
small modules
clear contracts
explicit data
simple process execution
predictable errors
```

The goal is a maintainable desktop application.

---

# 8. FINAL TARGET REPOSITORY STRUCTURE

Establish this conceptual structure:

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
├── catalog/
│
├── branding/
│
├── installer/
│
├── scripts/
│
├── .github/
│
├── README.md
├── .gitignore
└── LICENSE
```

Do not create dozens of speculative folders.

Only create additional folders when implementation requires them.

---

# PHASE 0 — REPOSITORY + ENVIRONMENT FORENSICS

Before implementing anything:

Inspect:

```text
OS
Windows version
architecture
CPU
RAM
disk
virtualization
Git
GitHub CLI if available
Flutter
Dart
Visual Studio
C++ workload
CMake
Ninja
Rust
Cargo
Docker
Docker Compose
WSL
WSL distributions
PowerShell
```

Produce a private implementation checklist.

Do NOT modify the machine recklessly.

Do not uninstall working tools.

Do not upgrade unrelated software merely because a newer version exists.

Prefer stable toolchains.

---

## PHASE 0 AUDIT

Run:

```text
Ponytail architecture audit
Ponytail dependency audit
Ponytail environment/codebase audit
```

Fix all actionable findings related to the repository foundation.

Create the first checkpoint:

```text
chore: establish project foundation
```

Push to:

```text
origin/main
```

---

# PHASE 1 — DEVELOPMENT TOOLCHAIN FROM ZERO

The development environment must be reproducible on a fresh Windows x64 development machine.

Establish:

```text
Flutter stable
Dart
Visual Studio Windows desktop C/C++ workload
Windows SDK
CMake
Ninja
Rust stable
Cargo
Git
Docker Desktop
WSL2
```

Follow current official installation requirements instead of hardcoding outdated versions.

IMPORTANT:

This phase is about the DEVELOPER environment.

Do not confuse it with the end-user Smart Setup system.

---

## Flutter Windows Bootstrap

Initialize the Flutter project from the correct location:

```text
/ui/flutter
```

Validate:

```bash
flutter doctor -v
flutter devices
```

The Windows desktop target must be recognized.

Create a minimal application.

Run:

```bash
flutter run -d windows
```

Then:

```bash
flutter build windows --release
```

If required system software is missing:

* identify the exact missing dependency,
* use the appropriate official/manual setup path,
* never fake success,
* continue with independent work where possible.

---

## Rust Bootstrap

Create:

```text
/engine/rust
```

Initialize a Rust project.

Validate:

```bash
cargo check
cargo test
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo build --release
```

Keep the initial binary lightweight.

---

## PHASE 1 PONYTAIL GATE

Use:

```text
software-craftsmanship
dependency audit
Ponytail performance audit
Ponytail architecture audit
```

Fix findings.

Run the Flutter and Rust test/build cycle again.

Commit:

```text
feat: bootstrap flutter and rust development environments
```

Push.

---

# PHASE 2 — CORE UI SHELL

Now create the first real ZITERA_LAB application shell.

DO NOT create every final screen in detail yet.

Build the structural shell:

```text
App
│
├── Navigation
├── Dashboard
├── Labs
├── Lab Detail
├── Challenge
├── Environment
├── Tools
└── Settings
```

The UI must already feel intentional.

No placeholder "AI dashboard" styling.

No excessive glow.

No huge gradients.

No black neon cyberpunk overload.

---

# PHASE 3 — ZITERA DESIGN SYSTEM

Use the installed:

```text
design-system
ui-styling
```

skills.

Create a centralized design system.

At minimum define:

```text
Color tokens
Typography
Spacing
Radius
Borders
Elevation
Icon sizing
Status styles
Button variants
Input variants
Card variants
Navigation styles
Lab status styles
```

Visual direction:

```text
clean
cyber security
technical
slightly intimidating
premium
mostly light / neutral
strong typography
restrained accent color
subtle technical patterns
minimal glitch details
```

Avoid:

```text
full-screen black
neon green everywhere
hacker cliché
generic shield icon
AI-glow cards
overloaded dashboards
```

Create reusable widgets.

Do not repeatedly hand-code identical cards/buttons.

---

# PHASE 4 — ZITERA_LAB BRANDING

Create the ZITERA_LAB visual identity.

Primary identity:

```text
ZITERA_LAB
```

Logo requirement:

TEXT ONLY.

Direction:

```text
ASCII inspired
distorted monospace
horror/cyber
technical
slightly disturbing
recognizable
```

The logo should look intimidating but remain legible.

Do NOT create:

```text
generic shield
generic lock
generic skull
generic hacker silhouette
AI brain icon
```

Use the design skills to produce the identity.

Prefer an editable/vector-first source.

Create:

```text
/branding/
```

with approximately:

```text
zitera_wordmark.svg
zitera_wordmark_light.svg
zitera_wordmark_dark.svg
zitera_logo_compact.svg
zitera_logo.png
zitera_logo_transparent.png
zitera_icon.png
zitera_splash.png
```

Use appropriate formats based on actual platform requirements.

The main logo must remain usable without an external font dependency when rendered as an asset.

If a generated visual reference is used, convert/finalize the actual product logo into a deterministic editable/vector asset rather than relying on an AI raster image.

---

# PHASE 5 — FLUTTER APP ARCHITECTURE

Structure the Flutter codebase cleanly.

Suggested conceptual structure:

```text
lib/
├── app/
├── core/
│   ├── theme/
│   ├── routing/
│   ├── errors/
│   ├── utils/
│   └── widgets/
├── features/
│   ├── dashboard/
│   ├── labs/
│   ├── challenge/
│   ├── environment/
│   ├── tools/
│   └── settings/
└── main.dart
```

Do not copy this blindly.

Use the simplest maintainable equivalent.

Important:

Flutter widgets must NOT contain:

```text
Docker logic
Git logic
PowerShell logic
WSL logic
package manager logic
tool installation logic
lab orchestration logic
```

The UI talks to the engine.

---

# PHASE 6 — RUST ENGINE FOUNDATION

Build:

```text
zitera-engine
```

The engine must work independently from Flutter.

Establish modules for:

```text
system
process
git
docker
wsl
tools
labs
catalog
updates
logging
cli
```

Keep interfaces small.

The engine should not become one giant `main.rs`.

---

# PHASE 7 — CLI

Implement:

```bash
zitera doctor
zitera lab list
zitera lab status <id>
zitera lab install <id>
zitera lab start <id>
zitera lab stop <id>
zitera lab reset <id>
zitera lab update <id>
zitera lab remove <id>
zitera tool list
zitera tool status
```

Support JSON:

```bash
zitera --json doctor
zitera --json lab status A01
```

Human output:

```text
ZITERA_LAB
Environment Diagnostics

Git       READY
WSL2      READY
Docker    READY
```

JSON output should use stable machine-readable structures.

Do not make Flutter scrape terminal formatting.

---

# PHASE 8 — FLUTTER ↔ RUST IPC

Use a simple process boundary.

Concept:

```text
Flutter
   ↓
launch engine
   ↓
arguments/input
   ↓
Rust engine
   ↓
JSON response
   ↓
Flutter
```

No permanent daemon unless required by a real use case.

Create a stable command/result contract.

Examples:

```json
{
  "success": true,
  "action": "doctor",
  "data": {
    "git": {
      "status": "ready",
      "version": "..."
    }
  }
}
```

Errors:

```json
{
  "success": false,
  "action": "lab.start",
  "error": {
    "code": "DOCKER_NOT_READY",
    "message": "Docker Desktop is installed but the Docker engine is not ready.",
    "recoverable": true
  }
}
```

---

# PHASE 9 — SMART SETUP ENGINE

This is one of the most important components.

The UI should present:

```text
SYSTEM READINESS
```

not raw shell commands.

Workflow:

```text
detect
↓
classify
↓
explain
↓
recommend
↓
install / guide
↓
recheck
↓
verify
```

Supported states:

```text
READY
MISSING
OUTDATED
WARNING
BLOCKED
MANUAL ACTION REQUIRED
```

Checks:

```text
Windows
architecture
virtualization
RAM
disk
internet
PowerShell
Git
WSL
WSL version
WSL distributions
Docker
Docker daemon
required ports
permissions
```

---

# PHASE 10 — SETUP MODES

Every dependency should support the appropriate mode:

```text
AUTO INSTALL
GUIDED INSTALL
MANUAL INSTALL
```

Do not force automatic installation when inappropriate.

Example:

```text
Git

[Auto Install]
[Manual Guide]
[Recheck]
```

Another:

```text
Docker Desktop

Installation requires user interaction.

[Open Official Guide]
[Check Again]
```

Another:

```text
Burp Suite

Recommended:
Manual installation.

[Official Installation Guide]
[Check Again]
```

The system must verify after installation.

Never assume success.

---

# PHASE 11 — COMMON TOOL MANAGER

Do not bundle heavy GUI tools into the main installer.

Do NOT embed:

```text
Burp Suite
OWASP ZAP
other large GUI security suites
```

as mandatory application payloads.

Instead provide:

```text
manual official installation guidance
detect if installed
version status
recheck
```

For lightweight/common CLI security tooling, create an app-managed tool profile system.

Initial candidates:

```text
Nmap
sqlmap
ffuf
gobuster / dirsearch
LinPEAS
```

Use the smallest sensible initial set.

Do not duplicate functionality unnecessarily.

IMPORTANT:

"Bundled" means the ZITERA_LAB ecosystem can provide and manage these tools.

It does NOT necessarily mean every third-party binary must be embedded inside the installer.

For lightweight third-party tools, prefer:

```text
detect
download official release
verify
store in ZITERA-managed tools directory
expose through controlled execution
```

This keeps the core installer lighter.

Respect each tool's license.

Never redistribute a third-party binary when its license or distribution terms do not permit it.

For each tool define:

```text
id
name
category
platform
environment
minimum version
detection command
official source
update source
installation method
verification method
license metadata if necessary
```

---

# PHASE 12 — TOOL EXECUTION BOUNDARY

Do not directly concatenate arbitrary user input into shell commands.

The Rust engine should use process APIs and explicit argument arrays.

Dangerous example to avoid:

```text
shell("tool " + userInput)
```

Prefer:

```text
command = toolExecutable
args = [validatedArgument1, validatedArgument2]
```

Validate:

```text
tool ID
lab ID
paths
ports
versions
repository identifiers
```

---

# PHASE 13 — WSL MANAGER

Implement detection first.

Detect:

```text
WSL installed
WSL version
default distribution
available distributions
distribution version
kernel information
```

Do not automatically create multiple distributions.

Start with:

```text
detect existing WSL
```

and guide users when a distro is required.

Docker Desktop's WSL2 integration remains the primary container runtime.

---

# PHASE 14 — DOCKER MANAGER

Implement:

```text
Docker detected
Docker daemon status
Docker version
Compose availability
container list
lab container identification
lab start
lab stop
lab reset
```

Use unique ZITERA naming.

Do not kill unrelated containers.

Before starting:

```text
check port
check image
check dependencies
check runtime
```

---

# PHASE 15 — LAB CONTRACT IMPLEMENTATION

Implement the lab contract defined in:

```text
/prd/lab_specification.md
```

Each lab needs:

```text
manifest.json
lesson/
challenge/
docker/
assets/
```

The engine must understand the machine-readable manifest.

Do not hardcode A01 logic into the engine when the manifest can describe it.

---

# PHASE 16 — CENTRAL CATALOG

Create:

```text
/catalog/
```

with a central catalog representation.

Example:

```json
{
  "schemaVersion": 1,
  "labs": [
    {
      "id": "A01",
      "title": "Broken Access Control",
      "repository": "xdaamar/zitera_lab_a01",
      "version": "1.0.0"
    },
    {
      "id": "A05",
      "title": "Injection",
      "repository": "xdaamar/zitera_lab_a05",
      "version": "1.0.0"
    }
  ]
}
```

The application must not hard-code every lab repository throughout UI code.

Catalog should be the source of lab discovery.

---

# PHASE 17 — CATALOG UPDATE

Implement:

```text
refresh catalog
compare versions
detect new lab
detect lab update
```

Example:

```text
New lab available

A05 — Injection

[Install]
```

or:

```text
Update available

A01
Installed: 1.0.0
Available: 1.1.0

[Update]
```

Catalog/content updates must NOT require rebuilding the Flutter application.

---

# PHASE 18 — INDIVIDUAL LAB REPOSITORIES

If GitHub authentication/permissions are available through the environment:

Check:

```bash
gh auth status
```

Then create/use:

```text
xdaamar/zitera_lab_a01
xdaamar/zitera_lab_a05
```

If repository creation cannot be performed because GitHub credentials are unavailable:

DO NOT invent credentials.

Continue local implementation and provide the exact remaining repository operation in the final report.

Do not let a GitHub authentication blocker prevent development of the local lab architecture.

---

# PHASE 19 — A01 REFERENCE LAB

Build:

```text
A01:2025 — Broken Access Control
```

The lab MUST be intentionally vulnerable.

It MUST run locally.

Prefer a small self-contained web application.

Keep dependencies controlled.

A small Flask/SQLite-style application is acceptable if it simplifies the runtime and keeps the lab easy to reproduce.

Do not over-engineer the vulnerable application.

The lab should demonstrate:

```text
authentication ≠ authorization
```

and a controlled broken-access-control scenario.

The target must contain only synthetic local data.

No real credentials.

No real third-party data.

No public targets.

---

# PHASE 20 — A01 LEARN MODE

Create a polished lesson.

Structure:

```text
What is Broken Access Control?
↓
Everyday analogy
↓
Authentication vs Authorization
↓
What the vulnerability looks like
↓
Why developers accidentally create it
↓
Impact
↓
How to identify it
↓
How to fix it
```

The analogy should be memorable but technically accurate.

Do not turn the lesson into a generic cybersecurity article.

Tie every concept to the actual lab.

---

# PHASE 21 — A01 PRACTICE MODE

Create guided tasks.

Each significant task should explain:

```text
What are we doing?
Why?
What should we observe?
What does the result mean?
```

The practice mode should progressively reduce hand-holding.

Do not encourage blindly copying long commands.

---

# PHASE 22 — A01 CHALLENGE MODE

Create a separate CTF-style challenge.

No direct walkthrough.

Provide:

```text
Scenario
Objective
Target
Clues
Optional progressive hints
Success condition
Flag/validation
```

Challenge data must use synthetic local content.

Use an isolated flag.

Example conceptual structure:

```text
MISSION

A test application appears to allow users to access resources they should not own.

OBJECTIVE

Find the access-control weakness and retrieve the challenge flag.

CLUES

1. Multiple user accounts exist.
2. Authorization behavior differs between endpoints.
3. User-controlled identifiers may influence resource selection.
```

Do not include the full solution in the default challenge presentation.

---

# PHASE 23 — A01 RESET

Implement:

```text
Start
Stop
Reset
```

Reset must recreate deterministic initial state.

Test:

```text
modify lab
solve challenge
reset
verify clean state
```

---

# PHASE 24 — A05 REFERENCE LAB

Build:

```text
A05:2025 — Injection
```

Use a separate repository.

A small local application with a synthetic database is acceptable.

The goal is to demonstrate a controlled SQL injection learning scenario.

Keep the application intentionally vulnerable ONLY inside the lab.

No remote targets.

No third-party data.

---

# PHASE 25 — A05 LEARN MODE

Structure:

```text
What is Injection?
↓
Analogy
↓
How application/data boundaries fail
↓
SQL injection concept
↓
Why string concatenation is dangerous
↓
Impact
↓
Detection concepts
↓
Parameterized queries / remediation
```

Keep the explanation understandable to beginners.

---

# PHASE 26 — A05 PRACTICE MODE

Build a guided investigation.

Include:

```text
observe request
understand input
observe application behavior
understand root cause
understand mitigation
```

Do not turn the whole lesson into a copy/paste exploit tutorial.

The lab itself remains intentionally vulnerable for learning.

---

# PHASE 27 — A05 CHALLENGE MODE

Create a distinct challenge.

Provide:

```text
mission
objective
clues
progressive hints
validation
flag
reset
```

Do not reveal the full exploit path immediately.

The challenge should test understanding rather than command memorization.

---

# PHASE 28 — LAB INSTALLATION FLOW

From the application:

```text
Labs
 ↓
A01
 ↓
Install
 ↓
Dependency check
 ↓
Repository retrieval
 ↓
Manifest validation
 ↓
Runtime preparation
 ↓
Health check
 ↓
Installed
```

User should NOT need to know Git commands.

The engine handles them.

Prefer shallow retrieval where practical.

Do not download unnecessary Git history.

---

# PHASE 29 — LAB START FLOW

User:

```text
[Start Lab]
```

System:

```text
check Docker
check image/runtime
check port
prepare container
start
health check
return target URL
```

UI:

```text
LAB READY

Target:
http://127.0.0.1:<port>

[Open Target]
[Stop Lab]
[Reset Lab]
```

---

# PHASE 30 — LAB UPDATE FLOW

Do NOT blindly overwrite a functioning lab.

Flow:

```text
check catalog
↓
new version?
↓
download/prepare new version
↓
validate manifest
↓
verify compatibility
↓
stop old version if needed
↓
activate new version
↓
health check
↓
mark updated
```

If update fails:

```text
do not falsely mark success
do not silently delete working state
report error
```

---

# PHASE 31 — LAB REMOVE FLOW

Implement:

```text
Remove Lab
```

But remove ONLY resources owned by ZITERA_LAB.

Do not delete unrelated:

```text
containers
volumes
images
folders
ports
```

Use explicit naming/prefixing.

---

# PHASE 32 — PROGRESS SYSTEM

Implement lightweight local progress.

Track approximately:

```text
lesson started
lesson completed
practice completed
challenge completed
lab installed
last opened
```

Do not add:

```text
remote account
cloud database
social features
```

yet.

Use local persistence.

Keep it replaceable later.

---

# PHASE 33 — ERROR SYSTEM

Create structured errors.

Examples:

```text
DOCKER_NOT_INSTALLED
DOCKER_NOT_READY
WSL_NOT_AVAILABLE
GIT_NOT_FOUND
LAB_REPOSITORY_UNAVAILABLE
LAB_MANIFEST_INVALID
PORT_IN_USE
LAB_START_FAILED
LAB_HEALTHCHECK_FAILED
LAB_UPDATE_FAILED
UNSUPPORTED_ENVIRONMENT
```

Every error should have:

```text
code
human message
technical details
recoverable flag
suggested action
```

The UI must present useful recovery guidance.

Bad:

```text
Process exited with code 1.
```

Good:

```text
Docker Desktop is installed, but its engine is not ready.

Open Docker Desktop and wait until the engine reports Ready.

[Retry]
[View Details]
```

---

# PHASE 34 — PERFORMANCE PASS

Now explicitly use the installed Ponytail efficiency/performance skills.

Audit:

```text
Flutter rebuilds
unnecessary widget work
asset loading
navigation
startup
memory
large dependencies
duplicate utilities
Rust allocations where relevant
process spawning
Git operations
Docker polling
catalog fetching
logging overhead
```

Implement:

```text
lazy loading
reasonable caching
no busy loops
no unnecessary polling
small payloads
no duplicate downloads
```

Do NOT optimize imaginary bottlenecks.

Use evidence.

---

# PHASE 35 — UI QUALITY PASS

Run:

```text
design-system audit
ui-styling audit
accessibility review
responsive desktop layout review
typography consistency review
spacing consistency review
empty/error/loading states
```

Verify the UI remains:

```text
clean
premium
cyber
not overly dark
not over-glowing
not generic
```

Every important page should have:

```text
loading
success
empty
error
disabled
```

states where applicable.

---

# PHASE 36 — FULL CROSS-LAYER TEST

Test the entire chain:

```text
Flutter
 ↓
Rust
 ↓
Git
 ↓
Catalog
 ↓
Docker
 ↓
A01
```

Then:

```text
Flutter
 ↓
Rust
 ↓
Catalog
 ↓
Docker
 ↓
A05
```

Test:

```text
discover
install
start
open
stop
reset
update
remove
```

for both labs.

---

# PHASE 37 — FAILURE TESTING

Intentionally test failures.

Examples:

```text
Docker closed
Git unavailable
internet unavailable
port occupied
invalid repository
invalid manifest
corrupt local lab
lab already running
lab already installed
catalog unavailable
update interrupted
Docker image unavailable
WSL unavailable
```

The system must fail gracefully.

Never silently report:

```text
READY
```

when the runtime is not ready.

---

# PHASE 38 — SECURITY AUDIT

Use all appropriate security/Ponytail/security craftsmanship skills.

Audit:

```text
command execution
argument handling
path traversal risks
repository input
host mount usage
Docker privileges
network exposure
port handling
download validation
tool installation
temporary files
logging
secrets
permissions
cleanup
```

Important:

Labs are intentionally vulnerable.

The HOST APPLICATION must not be unnecessarily vulnerable.

---

# PHASE 39 — HOST ISOLATION VALIDATION

Verify:

```text
lab binds locally by default
no unnecessary LAN exposure
no arbitrary host filesystem mount
no host credential access
no silent shell script execution
no unrelated container deletion
no unrelated volume deletion
```

If a lab genuinely requires something exceptional:

document it in its manifest and security documentation.

---

# PHASE 40 — DOCUMENTATION CONSISTENCY

Review:

```text
README
PRD
architecture
lab specification
security specification
CLI docs
setup docs
development docs
```

Do not create documentation merely for decoration.

Update existing documents only when implementation genuinely changes the established behavior.

If architecture must change:

create:

```text
/prd/decisions/
```

and write a concise ADR.

Never silently change the core architecture.

---

# PHASE 41 — GITHUB LAB REPOSITORY QUALITY

Each lab repository must have:

```text
README.md
manifest.json
lesson
challenge
docker
assets
```

README should explain:

```text
purpose
OWASP category
learning objectives
supported runtime
how to run independently if appropriate
reset behavior
security scope
```

Do not include real-world targets or credentials.

---

# PHASE 42 — RELEASE / CI FOUNDATION

Create `.github/workflows/`.

Core repository CI should validate at minimum:

Flutter:

```text
flutter pub get
flutter analyze
flutter test
flutter build windows --release
```

Rust:

```text
cargo fmt --check
cargo check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
cargo build --release
```

Use the project’s actual toolchain versions.

Do not blindly pin old versions.

---

# PHASE 43 — PACKAGE / INSTALLER DECISION

Do NOT lock the project to Inno Setup.

Evaluate the practical choices for the current Windows-first project:

```text
MSIX
Inno Setup
other appropriate Windows packaging approach
```

Evaluate based on:

```text
installation simplicity
update mechanism
offline usability
development complexity
distribution
security/signing implications
Flutter compatibility
Rust engine inclusion
```

Choose a V1 packaging strategy only after this comparison.

If the choice is architectural and likely to affect future release infrastructure, record an ADR.

For this sprint, at minimum produce a working release artifact that includes:

```text
Flutter app
Rust engine
required assets
CLI
configuration
```

---

# PHASE 44 — RELEASE CONTENT SEPARATION

Prove that:

```text
CORE APP
```

and:

```text
LAB CONTENT
```

are separate.

Demonstrate:

```text
A01 update
```

without rebuilding Flutter.

Demonstrate:

```text
catalog update
```

without rebuilding Flutter.

Demonstrate:

```text
new lab metadata
```

appearing without modifying core UI logic.

This is a core acceptance requirement.

---

# PHASE 45 — CLEAN MACHINE SIMULATION

Where practical, test the app in the closest possible clean environment.

At minimum verify that the built application does NOT assume:

```text
developer directory
developer PATH
IDE open
specific current working directory
```

All runtime paths must be resolved safely.

The application must locate its own resources correctly.

---

# PHASE 46 — FINAL PONYTAIL MASTER AUDIT

Now run the FULL available Ponytail audit suite.

Minimum areas:

```text
architecture
code quality
security
performance
dependencies
Flutter
Rust
UI
dead code
duplication
unnecessary abstraction
error handling
resource management
process execution
filesystem handling
Docker integration
```

Then:

```text
collect findings
↓
classify
↓
fix actionable findings
↓
re-audit
```

Do not stop after the first audit.

---

# PHASE 47 — FINAL BUILD

Run a complete build cycle.

Flutter:

```bash
flutter clean
flutter pub get
flutter analyze
flutter test
flutter build windows --release
```

Rust:

```bash
cargo fmt --check
cargo check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
cargo build --release
```

Run the actual built release application.

Do not rely only on debug mode.

---

# PHASE 48 — FINAL END-TO-END DEMONSTRATION

The final validation should reproduce this exact user journey:

```text
Launch ZITERA_LAB
        ↓
Dashboard
        ↓
Environment
        ↓
System diagnostics
        ↓
Labs
        ↓
A01
        ↓
Install
        ↓
Start
        ↓
Open target
        ↓
Learn
        ↓
Practice
        ↓
Challenge
        ↓
Complete / validate
        ↓
Reset
        ↓
A05
        ↓
Install
        ↓
Start
        ↓
Learn
        ↓
Practice
        ↓
Challenge
        ↓
Reset
        ↓
Catalog refresh
        ↓
Update detection
```

Everything must remain inside the intended local educational environment.

---

# 49. FINAL GIT CHECKPOINTS

Create stable checkpoint commits for each major completed layer.

At minimum:

```text
1. project foundation
2. flutter bootstrap
3. design system and branding
4. rust engine
5. IPC
6. smart setup
7. tools
8. catalog/lab contract
9. docker runtime
10. A01
11. A05
12. updates/progress
13. performance/security
14. release readiness
```

Push each stable checkpoint to:

```text
origin/main
```

At the end:

```bash
git status
git log --oneline --decorate -20
git diff --check
```

Repository must be clean except for intentionally ignored/generated files.

---

# 50. FINAL REPORT REQUIREMENT

When the entire run is complete, produce a structured report.

Do NOT use screenshots unless explicitly requested.

Report:

## A. Summary

What was actually implemented.

## B. Architecture

Final architecture:

```text
Flutter
Rust
IPC
GitHub
Catalog
Docker
WSL2
Labs
```

## C. Files Added

Important new files/directories.

## D. Major Features

List actual functionality.

## E. Skills Used

List the relevant installed skills used during implementation.

Especially:

```text
design-system
ui-styling
software-craftsmanship
Ponytail audits
Ponytail efficiency
Ponytail security
```

## F. Audit Results

For every major gate:

```text
audit
findings
fixes
re-audit result
```

## G. Tests

Provide exact validation performed.

Example:

```text
Flutter analyze      PASS
Flutter test         PASS
Windows build        PASS
Rust fmt             PASS
Rust test            PASS
Rust clippy          PASS
A01 install          PASS
A01 start            PASS
A01 reset            PASS
A05 install          PASS
A05 start            PASS
A05 reset            PASS
```

Never claim PASS when it was not actually executed.

## H. Git Checkpoints

List:

```text
commit hash
commit message
purpose
push status
```

## I. GitHub Repositories

Confirm:

```text
core repo
A01 repo
A05 repo
```

and their status.

## J. Known Limitations

Only actual remaining limitations.

## K. Recommended Next Sprint

Keep this concise.

Do not redesign the entire project.

## L. Suggested Final Commit

Provide one suggested final release/checkpoint commit message.

---

# 51. IMPORTANT BEHAVIORAL RULES FOR THIS ENTIRE SPRINT

Never:

```text
rewrite working code without reason
delete user work
skip tests
hide errors
fake successful installation
fake successful audit
fake successful Git push
invent GitHub credentials
hardcode secrets
silently install heavy security software
expose labs publicly
turn the UI into a generic cyberpunk dashboard
create unnecessary abstraction
create unnecessary documentation
```

Always:

```text
inspect first
use installed skills
follow the PRD
preserve boundaries
audit
debug
test
re-audit
checkpoint
push
continue
```

---

# 52. CRITICAL ARCHITECTURE BOUNDARIES

These must remain true:

```text
Flutter
    =
UI / Presentation / User Experience

Rust
    =
Engine / System Integration / CLI / Orchestration

Docker + WSL2
    =
Lab Runtime

GitHub
    =
Lab Distribution / Source

Catalog
    =
Lab Discovery / Version Metadata

Lab Repository
    =
Independent Educational Content + Runtime

Local Storage
    =
Progress / Config / Cache / Logs
```

Do not merge these responsibilities just because doing so is temporarily easier.

---

# 53. CORE PRODUCT EXPERIENCE

The final UI should communicate:

```text
ZITERA_LAB

Your local security laboratory.

Learn.
Experiment.
Investigate.
Solve.
```

The user should NOT feel like they are configuring infrastructure.

They should feel like they are entering a laboratory.

Infrastructure complexity should be hidden behind the platform while still available through diagnostics/advanced views for technical users.

---

# 54. MVP DEFINITION OF DONE

The sprint is successful ONLY if the following is true:

```text
[ ] Windows x64 Flutter app builds
[ ] Rust engine builds
[ ] Rust CLI works
[ ] Flutter can communicate with Rust
[ ] Doctor/system diagnostics work
[ ] Smart setup model works
[ ] Tool manager works
[ ] Git manager works
[ ] Catalog works
[ ] Docker manager works
[ ] WSL detection works
[ ] A01 is installable
[ ] A01 is runnable
[ ] A01 has Learn
[ ] A01 has Practice
[ ] A01 has Challenge
[ ] A01 resets
[ ] A05 is installable
[ ] A05 is runnable
[ ] A05 has Learn
[ ] A05 has Practice
[ ] A05 has Challenge
[ ] A05 resets
[ ] Lab update works
[ ] Catalog refresh works
[ ] Local progress works
[ ] Error states work
[ ] Security review completed
[ ] Ponytail audits completed
[ ] Performance pass completed
[ ] Release build completed
[ ] Git checkpoints pushed
[ ] Repository clean
```

If one item is incomplete, explicitly report it.

Do not pretend the MVP is finished.

---

# 55. FINAL INSTRUCTION

Start NOW.

Do not start by writing a huge amount of code.

First:

```text
inspect repository
inspect PRD
inspect architecture
inspect lab specification
inspect security specification
inspect available skills
inspect environment
```

Then execute the phases sequentially.

Use the shortest correct implementation.

Audit continuously.

Fix continuously.

Test continuously.

Commit and push stable checkpoints.

Protect the architecture.

Protect the lab isolation.

Keep ZITERA_LAB lightweight.

Keep the UI clean.

Keep the labs educational.

Do not drift from the product vision.

When the full sprint has genuinely reached a stable state, provide the final implementation report described above.
