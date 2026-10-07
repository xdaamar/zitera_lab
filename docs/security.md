# ZITERA_LAB Security Architecture & Threat Model

## 1. Security Philosophy

ZITERA_LAB delivers authentic, exploit-vulnerable scenarios while guaranteeing absolute safety for the learner's host workstation. Security is enforced through defense-in-depth isolation layers.

---

## 2. Core Security Controls

### A. Non-Elevated Execution (Zero Admin Rights)
- The application executes strictly in standard user context.
- No UAC elevation prompts, no driver installations, and no system file modifications are permitted.

### B. Windows Job Object Containment
- Every laboratory runtime process tree is bound to a dedicated Windows Job Object.
- **`JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE`**: Ensures that when the lab is stopped or the engine terminates, every child, descendant, and background thread is instantly terminated by the Windows kernel.
- **Resource Limits**: Strict memory limits (default 512 MB) and CPU rate controls prevent resource starvation and runaway processes.

### C. Low-Privilege Token & AppContainer Isolation
- Processes run with restricted token privileges, stripped of administrative sids and write access to Windows system directories (`C:\Windows`, `C:\Program Files`).
- Temporary sandbox directories are assigned restrictive ACLs granted only to the specific runtime identity.

### D. Cryptographic Package Verification (Ed25519)
- Laboratory archives (`.zlab`) and catalog feeds are verified against authoritative Ed25519 public keys before installation or extraction.
- **Anti-Downgrade Protection**: The engine enforces monotonic security version counters (`security_version`). Installing a package with a lower security version than currently active is rejected.
- **Path Traversal Protection**: Archive extraction validates all relative filenames, strictly aborting on directory traversal sequences (`..`), drive prefixes, or UNC paths.

### E. Security Reverse Proxy & Broker Isolation
- Lab target servers bind exclusively to localhost (`127.0.0.1`) on ephemeral ports.
- The built-in Rust Broker intercepts incoming traffic, enforcing:
  - High-entropy cryptographic token validation (`/session/<token>/`).
  - Strict `Host` header whitelisting.
  - Outbound SSRF destination filtering (loopback, link-local, and cloud metadata blocking).

### F. Terminal VFS Isolation
- The educational terminal operates entirely inside an in-memory Virtual Filesystem.
- Shell metacharacters (`;`, `&&`, `||`, `|`, `&`, `` ` ``, `$()`) are detected and rejected by the lexer before command dispatch.
- Prohibited host binaries (`cmd.exe`, `powershell.exe`, `wsl.exe`, `taskkill.exe`, `reg.exe`) are blocked by static allowlists.
