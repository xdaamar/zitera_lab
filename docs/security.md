# ZITERA_LAB // Security Architecture & Threat Model

ZITERA_LAB is engineered to deliver authentic, exploit-vulnerable scenarios while guaranteeing absolute containment and protection for the student's Windows workstation. Security is enforced through kernel-level isolation, cryptographic trust chains, input sanitization, and strict data boundaries.

---

## 1. Core Defensive Principles

1. **Zero Administrator Privileges:** The platform operates strictly in the standard non-elevated user context. Zero UAC prompts, zero device drivers, zero system service registrations.
2. **Zero Virtualization & Daemonless:** Completely eliminates Docker, Docker Desktop, and WSL2. Eliminates container daemon attack surfaces and host bridging vulnerabilities.
3. **Defense-in-Depth:** Every lab process is subjected to dual-layer containment: Windows AppContainers and Windows Job Objects.
4. **Offline-First & Zero Telemetry:** Zero outbound telemetry, zero cloud tracking, and complete privacy sanitization on diagnostic bundles.

---

## 2. The 17 Security Controls Matrix

| Control ID | Security Control | Threat Mitigated | Defensive Mechanism |
| :---: | :--- | :--- | :--- |
| **01** | **Package Signature** | Forged or tampered `.zlab` packages | Asymmetric Ed25519 digital signature verified prior to extraction. |
| **02** | **Catalog Signature** | MITM catalog spoofing or rogue lab links | Signed catalog root key matching with canonical JSON verification. |
| **03** | **Anti-Downgrade** | Replay attacks installing known vulnerable builds | Monotonic `security_version` counter enforcement. Lower revisions rejected. |
| **04** | **Archive Traversal** | Zip slip attacks targeting host filesystem | Path canonicalization (`safe_subpath`) checking every archive entry name. |
| **05** | **Path Traversal** | Relative escape (`../../Windows`) via terminal | Logical VFS bounds confinement; commands restricted to virtual mounts. |
| **06** | **AppContainer Isolation** | Host OS file/registry tampering by lab | Low-integrity Windows AppContainer with `CapabilityCount = 0`. |
| **07** | **Job Object Containment** | Runaway processes, fork bombs & orphaned daemons | `JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE` and process count caps (16 processes). |
| **08** | **Broker SSRF Protection** | Cloud metadata probe (`169.254.169.254`) or LAN scan | In-process proxy restricts targets to loopback `127.0.0.1`; external IPs blocked. |
| **09** | **Terminal Injection** | Shell metacharacter injection (`;`, `&&`, `|`, `` ` ``) | Lexical tokenizer rejects operators before dispatch; zero raw shell invocation. |
| **10** | **Safe Curl Proxy** | Exfiltration or unauthorized outbound traffic | In-process curl client restricted to localhost lab port. Non-loopback blocked. |
| **11** | **Process Table Isolation** | Host process enumeration and reconnaissance | Built-in `ps` inspects only internal sandboxed process tables. |
| **12** | **Non-Elevated Isolation** | Privilege escalation to Administrator | Enforces standard user token; explicitly checks and refuses elevation. |
| **13** | **Progress Migration** | Plaintext flag leakage & progress corruption | Dual-key normalization (`A01` <-> `zitera-lab-a01`); plaintext flags purged. |
| **14** | **Atomic Update** | Corrupted in-place overwrite during update | Staged in `versions/<V>/` and activated via atomic `active_version.txt` pointer. |
| **15** | **Atomic Rollback** | Bricked installation on update interruption | Preserves previous active version directory and rolls back on error. |
| **16** | **Diagnostic Scrubbing** | Accidental exposure of PII, tokens, or credentials | Diagnostics exporter redacts usernames, machine names, paths, and secrets. |
| **17** | **Installer Behavior** | System-wide directory pollution | Installs strictly into `%LOCALAPPDATA%\Programs\ZiteraLab` with clean uninstaller. |

---

## 3. Storage Boundary Protection (6-Tier Hierarchy)

User and system data are partitioned into 6 distinct isolation tiers:

1. **Tier 1 (Immutable Application):** `%LOCALAPPDATA%\Programs\ZiteraLab\bin\`
2. **Tier 2 (Student Progress):** `%LOCALAPPDATA%\ZiteraLab\user\` (Preserved across upgrades and uninstalls)
3. **Tier 3 (Lab Sandboxes):** `%LOCALAPPDATA%\ZiteraLab\labs\` (Isolated per-lab runtime state)
4. **Tier 4 (Disposable Cache):** `%LOCALAPPDATA%\ZiteraLab\cache\` (Purged on maintenance)
5. **Tier 5 (Session Logs):** `%LOCALAPPDATA%\ZiteraLab\logs\` (Rotated and size-capped)
6. **Tier 6 (Diagnostics):** `%LOCALAPPDATA%\ZiteraLab\diagnostics\` (Privacy-scrubbed archives)

---

## 4. Verification & Audit

To execute the authoritative security release gate:
```powershell
.\scripts\verify_security_gate.ps1
```
All 17 security controls must evaluate to **PASS**. Zero `UNKNOWN` or ambiguous states are permitted.
