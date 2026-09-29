# Remediation and Defense Strategies

## 1. Pin Exact Dependency Versions
Never use open-ended version ranges in production manifests. Use strict lockfiles (`package-lock.json`, `poetry.lock`, `Cargo.lock`) and verify their cryptographic hashes.

## 2. Software Bill of Materials (SBOM)
Generate and continuously audit an SBOM (using tools like CycloneDX or Syft) for every build artifact.

## 3. Disallow Arbitrary Post-Install Scripts
Disable lifecycle scripts during automated package installation:
```bash
npm install --ignore-scripts
```

## 4. Private Artifact Registries & Namespaces
Use scoped namespaces (`@mycompany/package`) and configure repository scoping to prevent dependency confusion attacks against public mirrors.

## 5. Automated Vulnerability Scanning
Integrate dependency scanners such as `npm audit`, `pip-audit`, `cargo-audit`, or Snyk into CI/CD pipelines to block builds containing known CVEs.
