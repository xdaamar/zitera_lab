# Laboratory Developer Guide: Adding a New Lab

This guide explains how to create, build, test, package, sign, register, and update a new laboratory module for ZITERA_LAB.

---

## 1. How do I create a lab?

Use the built-in CLI scaffolding tool:

```bash
zitera lab create <LAB_ID> "<TITLE>" "<OWASP_CATEGORY>"
```

**Example**:
```bash
zitera lab create A11 "Server-Side Request Forgery" "A10"
```

This generates a structured folder under `labs/A11/`:
- `manifest.json`: Canonical metadata adhering to the curriculum contract.
- `lesson/`: Educational markdown files (`01_overview.md`, `02_concept.md`, `03_remediation.md`).
- `challenge/`: CTF challenge scenario (`challenge.md`), scoring verification (`challenge.json`), and progressive hints (`hints.json`).
- `practice/`: Practice verification steps (`verify.json`).
- `bin/`: Target executable or entrypoint script.

---

## 2. How do I build it?

Laboratory applications run natively within the sandboxed runtime without Docker.
1. Place the executable entrypoint inside `labs/<LAB_ID>/bin/` (e.g. `labs/A11/bin/a11-lab.exe` or `labs/A11/bin/app.py`).
2. Ensure the entrypoint binds to the port specified in `manifest.json` (`default_port`) or listens on `127.0.0.1:$PORT`.
3. Verify that the entrypoint path in `manifest.json` correctly points to `bin/<entrypoint>`.

---

## 3. How do I test it?

### Step A: Automated Contract & Content Validation
Run the built-in lab validator:

```bash
zitera lab validate A11
```

The validator verifies 8 critical compliance checks:
- Lab directory exists and is accessible.
- `manifest.json` parses and adheres to the `LabManifest` schema.
- Curriculum metadata contract complies with OWASP Top 10:2025.
- Educational markdown lesson modules exist.
- Challenge specification and flag verification exist.
- Progressive tiered hints are configured.
- Target entrypoint exists and is executable.
- All paths are contained within the sandbox boundary (zero path traversal).

### Step B: Interactive Runtime Verification
Test the lifecycle locally:

```bash
# 1. Start lab in native sandbox
zitera lab start A11

# 2. Check runtime status & assigned port
zitera lab status A11

# 3. Test interaction via educational terminal
zitera terminal --lab A11 curl http://localhost:8090/

# 4. Validate practice verification
zitera lab practice-verify A11

# 5. Test challenge flag submission
zitera lab validate-challenge A11 "ZITERA{flag_here}"

# 6. Stop lab cleanly
zitera lab stop A11
```

---

## 4. How do I package it?

Compile the laboratory directory into a deterministic `.zlab` package:

```bash
zitera package build labs/A11 dist/zitera-lab-a11-v1.0.0.zlab
```

The package builder creates a standardized compressed archive containing all lesson materials, challenges, hints, binaries, and manifest.

---

## 5. How do I sign it?

Packages are signed with authoritative Ed25519 asymmetric keys:

```bash
zitera package sign dist/zitera-lab-a11-v1.0.0.zlab --key release_private.key
```

Verify signature and archive integrity:

```bash
zitera package verify dist/zitera-lab-a11-v1.0.0.zlab
```

---

## 6. How do I register it?

Add the lab metadata entry into `catalog/catalog.json`:

```json
{
  "id": "A11",
  "package_id": "zitera-lab-a11",
  "title": "Server-Side Request Forgery",
  "category_id": "A10",
  "category_name": "Mishandling of Exceptional Conditions",
  "standard": "owasp-top10",
  "standard_version": "2025",
  "difficulty": "Intermediate",
  "version": "1.0.0",
  "security_version": 1,
  "minimum_core_version": "2.0.0",
  "estimated_time": 30,
  "skills": ["SSRF Analysis", "Intranet Traversal", "URL Allowlisting"],
  "learning_objectives": [
    "Identify vulnerable HTTP request handlers",
    "Prevent unauthorized metadata endpoint queries"
  ],
  "download_url": "https://github.com/xdaamar/zitera_lab/releases/download/v1.0.0/zitera-lab-a11.zlab",
  "sha256": "<calculated_sha256_hash>"
}
```

---

## 7. How do I update it?

When updating an existing laboratory:
1. Increment the semantic version in `manifest.json` (e.g. `1.0.0` -> `1.1.0`).
2. If this update addresses a security flaw or patch, increment `security_version` (e.g. `1` -> `2`).
3. Re-run `zitera lab validate <LAB_ID>`.
4. Re-build and re-sign the package.
5. Update `catalog/catalog.json` with the new version, package hash, and download URL.
6. The client's Update Center will notify users of the update, display the changelog, and perform an in-place atomic update while preserving all learner progress.
