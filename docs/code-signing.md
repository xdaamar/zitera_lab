# Windows Release Code Signing Architecture & Workflow

**Document ID:** SEC-SIGN-001  
**Status:** APPROVED  
**Date:** 2026-10-07  
**Scope:** Phase 20 (Release Engineering, Distribution & Field Readiness)  

---

## 1. Dual Trust Model Architecture

ZITERA_LAB enforces a strict separation between two independent layers of cryptographic trust:

```text
┌────────────────────────────────────────────────────────────────────────┐
│                   LAYER 1: WINDOWS APPLICATION TRUST                   │
│                       (Windows Authenticode)                           │
│  • Target: zitera-engine.exe, zitera.exe, setup installers             │
│  • Algorithm: SHA-256 Authenticode (PKCS#7 / CMS)                      │
│  • Timestamp: RFC 3161 Timestamping Authority                          │
│  • Boundary: Operating System, Windows SmartScreen, Device Guard, WDAC │
└────────────────────────────────────────────────────────────────────────┘
                                    ≠
┌────────────────────────────────────────────────────────────────────────┐
│                    LAYER 2: ZITERA PACKAGE TRUST                       │
│                        (Contract V3 Ed25519)                           │
│  • Target: .zlab curriculum modules and lab packages                   │
│  • Algorithm: Ed25519 asymmetric signature over canonical payload      │
│  • Content Digest: Deterministic sorted entry SHA-256                  │
│  • Boundary: zitera-engine package verifier, zero external network     │
└────────────────────────────────────────────────────────────────────────┘
```

These two trust models serve distinct security boundaries:
- **Authenticode** ensures that the Windows kernel, SmartScreen, and enterprise Device Guard allow the application binaries and installer to launch without untrusted binary warnings or execution blocks.
- **Ed25519 Lab Trust** ensures that educational lab exercises downloaded or distributed dynamically cannot tamper with student machines, violate security versions, or inject unverified payloads.

---

## 2. Authenticode Specification

All shipped Windows PE executables (`.exe`, `.dll`) must conform to the following signing standard:

| Parameter | Standard | Rationale |
| :--- | :--- | :--- |
| **Digest Algorithm** | SHA-256 (`/fd SHA256`) | SHA-1 is cryptographically broken and rejected by modern Windows. |
| **Timestamp Protocol** | RFC 3161 (`/tr`) | Ensures binaries remain valid after certificate expiry. |
| **Timestamp Digest** | SHA-256 (`/td SHA256`) | Matches signature strength; prevents weak timestamp collisions. |
| **Primary TSA URL** | `http://timestamp.digicert.com` | High-availability enterprise timestamp authority. |
| **Secondary TSA URL** | `http://timestamp.sectigo.com` | Automated fallback for transient network failures. |
| **Description** | `ZITERA_LAB Enterprise Core` | Displayed on UAC and SmartScreen dialogs. |
| **Information URL** | `https://github.com/xdaamar/zitera_lab` | Canonical source repository. |

---

## 3. Key Management & Repository Isolation

### Strict Invariant: Zero Private Keys in Repository

Under no circumstances may private signing certificates (`.pfx`, `.p12`, `.key`, `.pem` private blocks) or certificate passwords be committed to the source control repository.

```text
PROHIBITED:
  • Storing .pfx / .p12 files in git
  • Hardcoding certificate passwords in scripts or source code
  • Committing unencrypted signing keys to CI/CD repositories
```

### Production Signing Pipeline

In production CI/CD builds, signing credentials must be provided externally via one of the following secure mechanisms:

1. **Hardware Security Module (HSM):**
   - FIPS 140-2 Level 2 EV Code Signing token (e.g., SafeNet, YubiKey) accessed via Windows Certificate Store with hardware PIN injection.
2. **Cloud Key Management Service:**
   - Azure Trusted Signing (formerly Microsoft Identity Verification) or AWS CloudHSM via CI/CD OIDC authentication.
3. **Environment Variable Injection (Protected Runners):**
   - Secure base64-encoded certificate injected via ephemeral memory variables (`ZITERA_SIGN_PFX_BASE64`, `ZITERA_SIGN_PWD`) on isolated, non-public runners.

### Development & Test Signing Fixture

For local pipeline testing and regression verification on development workstations:
- An ephemeral self-signed certificate fixture is generated on-demand in memory using `New-SelfSignedCertificate`.
- The fixture is used strictly for format and pipeline verification and is isolated from the Windows Root Trusted Authority store.

---

## 4. Signing Procedure

The automated signing helper script (`scripts/release/sign_release.ps1`) executes the following workflow:

```powershell
# Production Signing Example (Using External PFX):
.\scripts\release\sign_release.ps1 `
    -ArtifactPath "dist\bin\zitera-engine.exe" `
    -CertPath "$env:SIGNING_CERT_PATH" `
    -CertPassword "$env:SIGNING_CERT_PASSWORD"

# Development / Verification Example (Ephemeral Test Fixture):
.\scripts\release\sign_release.ps1 `
    -ArtifactPath "dist\bin\zitera-engine.exe" `
    -UseTestFixture
```

### Direct SignTool Invocation Reference

```cmd
signtool.exe sign ^
    /fd SHA256 ^
    /td SHA256 ^
    /tr http://timestamp.digicert.com ^
    /d "ZITERA_LAB Enterprise Core" ^
    /du "https://github.com/xdaamar/zitera_lab" ^
    /f "%SIGNING_CERT_PATH%" ^
    /p "%SIGNING_CERT_PASSWORD%" ^
    "dist\bin\zitera-engine.exe"
```

---

## 5. Verification & Validation Gate

Before any binary is promoted to release status, it must pass automated signature verification:

```powershell
# Verify Authenticode Signature
$sig = Get-AuthenticodeSignature -FilePath "dist\bin\zitera-engine.exe"

if ($sig.Status -notin @("Valid", "UnknownError")) {
    throw "Authenticode signature validation failed: $($sig.StatusMessage)"
}
```

### Verification Matrix

| Check | Expected Value | Enforcement |
| :--- | :--- | :--- |
| **Signature Status** | `Valid` (or `UnknownError` for self-signed test fixtures) | Automated release gate |
| **Signature Algorithm** | `SHA256` | Reject legacy SHA1 |
| **Signer Certificate** | Matches authorized Organization CN | Production gate |
| **Timestamp Counter** | Present and valid RFC 3161 token | Mandatory for production |
| **Binary Integrity** | PE checksum matches content | Rejects modified binaries |

---

## 6. Device Guard & WDAC Compatibility

Enterprises and universities deploying ZITERA_LAB under Windows Defender Application Control (WDAC) require signed binaries:
1. **Publisher Rule:** WDAC policies configured to trust the ZITERA_LAB code-signing certificate allow application updates seamlessly without requiring per-hash administrative approval.
2. **File Reputation:** Authenticode signatures combined with SmartScreen reputation accumulation ensure non-developer students experience zero installation blocking.
