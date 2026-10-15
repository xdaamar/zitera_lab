# Technical Analysis: Software Supply Chain Attack Vectors

Supply chain threats exploit structural trust assumptions within automated engineering workflows. Below is an in-depth examination of the primary threat vectors:

---

## 1. Typosquatting (Deceptive Package Registration)
Attackers register malicious packages on public registries (such as npmjs, PyPI, or crates.io) using names that intentionally mimic widely used libraries with millions of downloads:
- **Legitimate Packages**: `cross-env`, `requests`, `lodash`, `chalk`
- **Typosquatted Lookalikes**: `crossenv`, `requsts`, `loadash`, `chalk-js`

When developers make a typographical error during command-line installation (`npm install crossenv`) or within project configuration files, the package manager installs the rogue library. Once unpacked, embedded trojan payloads execute immediately on developer workstations or automated build agents.

---

## 2. Dependency Confusion (Public-Private Namespace Substitution)
Many organizations develop private in-house packages (such as `@acme-corp/financial-core` or `internal-auth-token`). When build pipeline package managers query both private and public registries simultaneously without strict namespace pinning:
1. Threat actors identify unreserved internal package names leaked through public source code, client-side bundles, or repository commit histories.
2. The attacker registers the identical package name on the public registry (npmjs.org or PyPI) with an artificially high semver version (e.g. `v99.0.0`).
3. Standard dependency resolution logic prioritizes the higher version number, silently fetching the attacker's package from the internet rather than the internal corporate repository.

---

## 3. Automated Lifecycle Hook Exploitation
Package ecosystems (notably npm and yarn) provide automated lifecycle hooks including `preinstall`, `install`, and `postinstall` intended for local C++ native compilation or environment configuration.

Attackers abuse these hooks to run arbitrary commands upon installation:
```json
{
  "name": "apex-internal-telemetry",
  "version": "0.1.0-alpha.build.88",
  "scripts": {
    "postinstall": "node ./scripts/send_env.js"
  }
}
```
During automated `npm install` execution in CI/CD runners, `send_env.js` executes with the runner's full ambient permissions, scraping environment secrets (`AWS_SECRET_ACCESS_KEY`, `DOCKER_PASSWORD`, API tokens) and transmitting them to a remote Command & Control (C2) server.

---

## 4. Maintainer Account Takeover & Repository Hijacking
Rather than publishing new packages, attackers frequently compromise established open-source projects:
- **Credential Stuffing / Phishing**: Stealing registry credentials from project maintainers who have not enforced Multi-Factor Authentication (MFA).
- **Social Engineering**: Infiltrating dormant repositories by volunteering as maintainers, then slipping backdoors into subsequent official release patches.
- **Domain Hijacking**: Claiming expired maintainer email domains to trigger password resets on package registries.

---

## 5. Lockfile Mechanics & Cryptographic Integrity
Deterministic lockfiles (`package-lock.json`, `poetry.lock`, `Cargo.lock`) preserve the entire resolved dependency graph alongside tamper-evident hashes:
- `resolved`: The exact absolute download URL of the archive tarball.
- `integrity`: Cryptographic checksum (e.g., Subresource Integrity SHA-512) validating payload authenticity.

If lockfiles are omitted from version control or altered without rigorous automated auditing, build runners risk fetching modified packages across the network.
