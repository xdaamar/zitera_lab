# System Architecture & Resilient Supply Chain Design

## 1. Lab A03 Architecture Topology (ApexCorp Package Registry)

The A03 target environment models an internal enterprise artifact cache and dependency audit gateway (*ApexCorp Dependency Inspector*) hosting build manifests on port 8013:

```text
[ Developer Workstation / CI Runner ]
                 │
                 ▼  (HTTP GET / POST port 8013)
┌──────────────────────────────────────────────────────────┐
│             APEXCORP REGISTRY PROXY GATEWAY              │
│                                                          │
│  ┌──────────────────────┐      ┌──────────────────────┐  │
│  │   Verified Packages  │      │  Compromised Module  │  │
│  │   • express (4.19.2) │      │  • apex-internal-    │  │
│  │   • lodash (4.17.21) │      │    telemetry         │  │
│  └──────────┬───────────┘      └──────────┬───────────┘  │
│             │                             │              │
│             ▼                             ▼              │
│  ┌────────────────────────────────────────────────────┐  │
│  │      Lockfile Audit Manifest Storage Engine        │  │
│  │      Path: /packages/audit/package_lock_audit.json │  │
│  └────────────────────────┬───────────────────────────┘  │
│                           │                              │
│                           ▼                              │
│  ┌────────────────────────────────────────────────────┐  │
│  │   Pipeline Token Verification Endpoint             │  │
│  │   Route: /api/supply-chain/verify                  │  │
│  └────────────────────────────────────────────────────┘  │
└──────────────────────────────────────────────────────────┘
```

---

## 2. Vulnerable Component Analysis
The target application exhibits multiple structural architectural flaws:
1. **Unscoped Package Ingestion**: The proxy accepted an unpinned, non-namespaced dependency `apex-internal-telemetry` (`v0.1.0-alpha.build.88`) lacking corporate GPG signing keys.
2. **Unconfined Installation Lifecycle Hooks**: The malicious package configured a `postinstall_script: node ./scripts/send_env.js` hook that simulated pipeline credential harvesting (`exfiltrated_pipeline_token`).
3. **Public Exposure of Build Telemetry**: The raw dependency audit manifest (`package_lock_audit.json`) was left publicly accessible over HTTP without authentication at `/packages/audit/package_lock_audit.json`.

---

## 3. High-Assurance Supply Chain Architecture (SLSA Framework)
To eliminate these vectors, modern architectures implement the **SLSA (Supply-chain Levels for Software Artifacts)** Level 3/4 framework:

```text
[ Git Source Code ] ──▶ [ Isolated Build Runner ] ──▶ [ Cryptographic Attestation ]
 (Signed Commits)       (Hermetic Container,           (Sigstore Cosign Provenance)
                         Zero Ambient Internet)                  │
                                                                 ▼
                                                    [ Quarantined Artifact Repo ]
                                                                 │
                                                    (Policy & Vulnerability Gate)
                                                                 │
                                                                 ▼
                                                    [ Production Deployment ]
```

### Core Architecture Pillars:
- **Hermetic Build Isolation**: Build runners execute inside isolated, ephemeral containers denied general internet access. Dependencies are supplied strictly through verified local proxy caches.
- **Cryptographic Provenance Attestation**: Every generated binary or container image is cryptographically signed (e.g. via Cosign/Sigstore) with tamper-proof claims specifying source repository, commit SHA, and builder identities.
- **Quarantined Ingestion Gateways**: External packages are quarantined and subjected to automated SCA scanners, behavioral sandboxing, and manual sign-off before entering internal enterprise registries.
