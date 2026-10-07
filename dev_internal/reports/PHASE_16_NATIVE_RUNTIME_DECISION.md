# ARCHITECTURE DECISION RECORD (ADR)
## ADR-016: ZITERA 2.0 Native Security Runtime Architecture Feasibility Gate

**Status:** **APPROVED (GO)**  
**Date:** October 4, 2026  
**Deciders:** Senior Systems Architecture & Core Engineering Team  
**Consulted:** ZITERA_LAB Product & Security Specification  
**Informed:** All Engineering Contributors  

---

### 1. Context and Problem Statement

ZITERA_LAB originally relied on Docker Desktop and WSL2 to execute vulnerable cybersecurity labs. While containers provided isolation, this architecture suffered from critical liabilities:
1. Massive system prerequisites: Docker Desktop requires Administrator privileges, Hyper-V, and multi-gigabyte background services.
2. Incompatible user experience: Standard Windows users frequently encountered WSL startup failures, engine connection timeouts, and system resource exhaustion (> 2 GB RAM idle).
3. Port collisions and firewall issues: Docker container port bindings conflicted with existing developer environments and triggered global Windows Firewall prompts.

In Phase 15, the legacy Docker and WSL dependencies were purged. Phase 16 was commissioned as the **Feasibility Gate** to prove whether Windows-native security labs can run isolated from the host machine using **AppContainer profiles + Job Objects + Process Containment + Network Proof**, without Administrator rights, without WSL, without Docker, and without system-wide registry/firewall modifications.

---

### 2. Decision

The Engineering Team unanimously declares a **GO** decision for the **ZITERA 2.0 Native Security Runtime**.

The native security runtime has been proven viable, safe, controllable, and performant. All 16 checkpoints of the Phase 16 specification have passed empirical validation.

---

### 3. Architecture Specification

The production architecture for ZITERA 2.0 native security runtime is defined as follows:

```
┌─────────────────────────────────────────────────────────────┐
│                 ZITERA 2.0 DESKTOP CLIENT                  │
│       (Flutter 3.x UI / Windows x64 Native Desktop)         │
└──────────────────────────────┬──────────────────────────────┘
                               │ JSON-RPC / CLI IPC
┌──────────────────────────────▼──────────────────────────────┐
│                    ZITERA CORE ENGINE                       │
│      (Rust Native Systems Engine / Non-Elevated User)       │
├─────────────────────────────────────────────────────────────┤
│  AppContainer Lifecycle Manager                             │
│  - Deterministic per-user naming: ZiteraLab_<ID>           │
│  - Token SID: S-1-15-2-...                                  │
│  - Zero Network Capabilities                                │
├─────────────────────────────────────────────────────────────┤
│  Win32 Job Object Controller                                │
│  - Process Tree Containment (Parent + Child + Grandchild)   │
│  - Hard Lifecycle Enforcement (Kill-on-Close)              │
│  - Memory & Process Count Limits                            │
├─────────────────────────────────────────────────────────────┤
│  IPC & Stdio Redirection Engine                             │
│  - Bidirectional Anonymous Stdio Pipes (CreatePipe)         │
│  - Zero Port Binding, Zero Firewall Exemptions              │
│  - Sanitized Unicode Environment Blocks                     │
└──────────────────────────────┬──────────────────────────────┘
                               │ Spawns into Job Object
┌──────────────────────────────▼──────────────────────────────┐
│                  ISOLATED LAB PROCESS                       │
│               (Vulnerable Native Target)                   │
│                                                             │
│  SECURITY BOUNDARY (Kernel Enforced):                       │
│  - Host filesystem: BLOCKED (Access Denied)                 │
│  - System directory write: BLOCKED                          │
│  - SAM / Host Credentials: BLOCKED                          │
│  - Outbound Internet / LAN: BLOCKED                         │
│  - Process Escape / Breakaway: BLOCKED                      │
│  - Startup Latency: < 100 ms                                │
│  - Memory Overhead: < 15 MB                                 │
└─────────────────────────────────────────────────────────────┘
```

---

### 4. Empirical Evaluation Summary

| Requirement | Standard | Empirical Result | Evaluation |
|-------------|----------|------------------|:----------:|
| **Zero Elevation** | Must not prompt UAC or require Admin | 100% standard user tokens across all operations | **MET** |
| **No Virtualization** | No Hyper-V, no WSL, no Docker | 100% pure Windows NT kernel security primitives | **MET** |
| **Filesystem Safety** | Host files, credentials, and system directories protected | Read of SAM and writes to Windows directories denied by OS | **MET** |
| **Process Tree Safety**| Child and grandchild processes cannot escape | Job Object terminates entire process hierarchy on close | **MET** |
| **Network Boundary**| Outbound Internet and LAN blocked by default | TCP connections to 1.1.1.1 and 192.168.1.1 rejected | **MET** |
| **IPC Viability** | Clean communication without firewall rules | Anonymous Stdio pipes provide robust microsecond IPC | **MET** |
| **Lifecycle Reliability**| Repeated cycles without resource leaks | 10x consecutive create/run/stop/clean completed cleanly | **MET** |
| **Performance** | Startup < 500 ms, RAM < 50 MB | Cold startup < 100 ms, RAM < 15 MB | **EXCEEDED** |
| **Release Stability** | Release binaries functional | Rust engine and Flutter Windows release binaries pass | **MET** |

---

### 5. Consequences & Architectural Rules for Phase 17

1. **Curriculum Migration Authorized:**
   - Migration of Lab A01 (Broken Access Control) and Lab A06 (Vulnerable Components) to Windows-native binaries is authorized.
2. **Standard User Invariant:**
   - No feature in Phase 17 or beyond may introduce administrative requirements.
3. **Zitera Terminal Stream Architecture:**
   - The interactive Zitera Terminal must attach directly to the sandboxed process's anonymous stdio pipes (`stdin`, `stdout`, `stderr`).
   - Raw user input must never be passed to `cmd.exe` or `powershell.exe`. Commands must be executed directly via structured argument vectors.
4. **Tool Packaging Standard:**
   - All tools (nmap, curl, etc.) must be bundled as portable, self-contained binaries within the lab package, executable under AppContainer tokens without host installation.

---

### 6. Sign-off

- **Architecture Gate:** **PASSED**
- **Feasibility Recommendation:** **PROCEED TO PHASE 17**
