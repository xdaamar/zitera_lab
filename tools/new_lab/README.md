# ZITERA_LAB // New Lab Template & Developer Workflow

This directory provides the canonical scaffolding and contract requirements for authoring new laboratory modules for ZITERA_LAB compliant with **OWASP Top 10:2025**.

## Fast Track: CLI Scaffolding

To create a new lab in seconds without copying template files manually:

```bash
# Syntax: zitera lab create <ID> [Title] [OWASP_Category]
zitera lab create A11 "Server-Side Request Forgery" "A10"
```

Then immediately validate compliance:

```bash
zitera lab validate A11
```

---

## Lab Directory Structure

Every lab must strictly follow this isolated structure under `labs/<LAB_ID>/`:

```text
labs/A11/
├── manifest.json            # Canonical curriculum & runtime metadata contract
├── lesson/                  # Markdown-based educational instructional modules
│   ├── 01_overview.md       # High-level vulnerability introduction & analogy
│   ├── 02_concept.md        # Technical breakdown & attack surface details
│   └── 03_remediation.md    # Defensive countermeasures & code fixes
├── challenge/               # CTF Challenge environment specifications
│   ├── challenge.md         # Objective instructions (no solutions leaked)
│   ├── challenge.json       # Challenge scoring & authoritative verification flag
│   └── hints.json           # Progressive tiered hints (tiers 1..3)
├── practice/                # Guided practice steps & verification definitions
│   └── verify.json          # Practice health and verification specification
├── bin/                     # Native compiled binary or script entrypoint
│   └── app.py (or .exe)     # Zero-docker native sandboxed process
└── README.md                # Developer & documentation notes
```

---

## Curriculum Metadata Contract (`manifest.json`)

The manifest is the single authoritative source of truth:

```json
{
  "schema_version": 2,
  "id": "A11",
  "package_id": "zitera-lab-a11",
  "slug": "a11",
  "title": "Server-Side Request Forgery",
  "owasp": "A10",
  "category_id": "A10",
  "category_name": "Mishandling of Exceptional Conditions",
  "standard": "owasp-top10",
  "standard_version": "2025",
  "short_description": "Hands-on investigation of SSRF vulnerabilities in cloud metadata endpoints.",
  "difficulty": "Intermediate",
  "estimated_minutes": 35,
  "estimated_time": 35,
  "modes": ["learn", "practice", "challenge"],
  "skills": ["Network Reconnaissance", "SSRF Exploitation", "Egress Filtering"],
  "learning_objectives": [
    "Understand how unsanitized user URLs enable internal intranet traversal",
    "Identify vulnerable HTTP client patterns in application code",
    "Implement URL allowlists and loopback IP blocking"
  ],
  "prerequisites": ["Basic HTTP protocol knowledge", "IP addressing basics"],
  "default_port": 8091,
  "runtime": "native_sandboxed",
  "entrypoint": "bin/a11-lab.exe",
  "version": "1.0.0",
  "security_version": 1,
  "minimum_core_version": "2.0.0"
}
```

---

## Validation & Packaging Checklist

Before submitting a pull request or distributing a package:

1. **Contract Validation**:
   ```bash
   zitera lab validate A11
   ```
2. **Package Build & Ed25519 Signing**:
   ```bash
   zitera package build labs/A11 dist/zitera-lab-a11-v1.0.0.zlab
   ```
3. **Cryptographic Verification**:
   ```bash
   zitera package verify dist/zitera-lab-a11-v1.0.0.zlab
   ```
