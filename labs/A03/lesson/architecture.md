# Target Lab Architecture

## System Diagram

```
[Developer / CI Environment]
            │
            ▼
[Package Registry Audit Engine] ─── (Serves http://127.0.0.1:8013)
            │
            ├── /packages/audit/package_lock_audit.json (Leaked metadata)
            └── /api/supply-chain/verify (Authorization checkpoint)
```

## Vulnerable Component
The target web service simulates an internal package registry proxy that caches dependency lockfiles. The build pipeline failed to verify GPG signatures and allowed an untrusted package `apex-internal-telemetry` into the lockfile. The metadata file exposes build environment secrets exfiltrated during the simulated `postinstall` step.
