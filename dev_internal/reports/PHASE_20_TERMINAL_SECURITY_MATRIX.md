# PHASE 20 // TERMINAL & NETWORK BOUNDARY SECURITY REGRESSION MATRIX

**Document ID:** `PHASE_20_TERMINAL_SECURITY_MATRIX.md`
**Program:** ZITERA 2.0 Core Modernization & Productization
**Sprint Phase:** Phase 20 (Release Engineering, Distribution & Field Readiness)
**Date:** October 7, 2026
**Target Platform:** Windows x64 (Native Sandboxed Runtime / In-Process VFS)
**Baseline:** Checkpoint 02 (Terminal Security & Local HTTP Boundary Hardening)

---

## 1. Executive Summary

Phase 19 introduced an in-process educational terminal featuring process table inspection (`ps`), contextual manual pages (`help <cmd>`), a built-in safe `curl` HTTP proxy, and a cybernetic Flutter console interface (`TerminalConsoleWidget`).

To ensure that this educational terminal cannot be leveraged as an attack vector for host enumeration, arbitrary shell execution, or outbound network pivoting, Checkpoint 02 establishes an exhaustive regression matrix. All terminal commands execute strictly within an in-memory Virtual Filesystem (VFS) with zero host process passthrough, strict command allowlisting, deterministic argument bounds, and enforced loopback-only network constraints.

Every test case in the regression matrix (21/21 terminal tests) passed with 100% compliance.

---

## 2. Threat Vector & Mitigation Matrix

| Threat Category | Attack Vector | Security Mechanism | Validation Status |
| :--- | :--- | :--- | :---: |
| **Host Process Leakage** | Learner executes `ps` or `ps -ef` to enumerate host Windows processes (`explorer.exe`, `lsass.exe`). | Synthetic process table reporting only isolated lab daemons (`init`, `lab-daemon`, `zitera-term`). Zero host NT process querying. | **PASS** (Zero Host Leakage) |
| **Arbitrary Shell Execution** | Learner attempts to invoke `cmd.exe`, `powershell`, `pwsh`, `bash`, `wsl`, or `runas`. | Static allowlist filter + explicit forbidden command list rejecting host executables with exit code 127. | **PASS** (Execution Blocked) |
| **Command Injection / Chaining** | Metacharacters injected into command string: `;`, `&&`, `\|\|`, `\|`, `&`, `` ` ``, `$()`. | Lexical tokenizer rejects non-allowlisted operators before command dispatch with syntax error (exit code 2). | **PASS** (Syntax Error Raised) |
| **Argument Overflow DoS** | Command line injected with > 128 arguments or single token > 4096 bytes. | Tokenizer enforces strict bounds: max 128 tokens and 4096 bytes per token. | **PASS** (Memory Bounded) |
| **Unmatched Quote Trapping** | Malformed quotation marks (`'foo"`, `"bar'`). | Finite-state quote parser rejects unbalanced delimiters with clean error. | **PASS** (Cleanly Handled) |
| **Host Filesystem Escape (Path)** | Path containing Windows drive letters (`C:\Windows`), backslashes (`..\..`), or UNC paths (`//`). | VFS path resolver intercepts backslashes and drive letters with access denied error. | **PASS** (Sandbox Escape Blocked) |
| **Virtual Root Traversal** | Command `cat ../../etc/passwd` attempted from root. | Path normalizer clamps parent traversal at logical root `/`. Access never escapes VFS boundary. | **PASS** (Confined to Sandbox) |
| **SSRF / External Pivot via curl** | Learner executes `curl http://evil.com` or `curl 192.168.1.1`. | Strict URL validator requires target host to resolve strictly to loopback (`127.0.0.1`, `localhost`, `::1`). | **PASS** (Outbound Denied) |
| **Cloud Metadata Pivot** | Learner targets AWS/GCP/Azure link-local address (`http://169.254.169.254`). | Target validator immediately rejects non-loopback IPs with sandbox policy error. | **PASS** (SSRF Blocked) |
| **Userinfo Credential Smuggling** | URL with embedded credentials: `http://user:pass@127.0.0.1`. | URL parser rejects `@` userinfo syntax in target URLs. | **PASS** (Smuggling Prevented) |
| **Local File Exfiltration via curl** | Argument `-o <file>` or `--output <file>` or `file://` protocol. | Tool executor restricts file output redirection in terminal mode. | **PASS** (Exfiltration Blocked) |
| **Protocol Confusion / HTTPS** | Requesting `https://127.0.0.1` against plaintext lab server. | HTTPS protocol restricted for local sandboxed lab endpoints. | **PASS** (Blocked with Actionable Msg) |
| **Invalid Port Specification** | Malformed port specification: `http://127.0.0.1:99999` or non-numeric port. | Port validator parses port as `u16` > 0. Out-of-range ports rejected. | **PASS** (Rejected) |
| **IPv6 Loopback Boundary** | Legitimate IPv6 loopback query: `http://[::1]:8080`. | Bracketed IPv6 address parser safely extracts host and port, permitting only `::1`. | **PASS** (Safely Handled) |
| **Unknown Command Guidance** | Learner runs unknown command `help evil_tool`. | Contextual help handler returns informative message with command suggestions (exit code 1). | **PASS** (Actionable Feedback) |

---

## 3. Test Execution Summary

- **Test Suite:** `terminal::tests` & `terminal::parser::tests` & `terminal::commands::tests`
- **Total Tests:** 21
- **Passed:** 21
- **Failed:** 0
- **Execution Time:** < 0.05 seconds
- **Sandbox Boundary:** 100% verified against host escape and external pivoting.
