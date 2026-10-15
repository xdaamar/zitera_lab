# A03: Software Supply Chain Failures

## 1. What is a Software Supply Chain Failure?
In modern software engineering, developers rarely construct complete codebases from scratch. Contemporary enterprise applications typically consist of 80% to 90% third-party components: open-source libraries, package manager registries (such as npm, PyPI, Cargo, Maven, NuGet), base container images, build pipeline plugins, and CI/CD automation workflows.

OWASP Top 10:2025 A03 categorizes vulnerabilities introduced into an application not through internal implementation bugs written by the development team, but through upstream supply chain compromises. When a trusted dependency is poisoned, backdoored, or spoofed, every downstream software package consuming that library inherits the risk automatically.

## 2. Why Supply Chain Security is Critical in 2025/2026
Supply chain attacks have expanded dramatically due to disproportionate attacker leverage:
- **Massive Blast Radius**: Compromising a single open-source package with millions of weekly downloads allows threat actors to compromise thousands of organizations simultaneously without penetrating individual corporate perimeters.
- **Implicit Automation Trust**: Automated CI/CD build environments frequently run with elevated credentials, holding access to container registries, cloud deployments, and production secrets.
- **Transitive Dependency Blind Spots**: A top-level dependency may resolve dozens or hundreds of indirect, transitive dependencies. Teams often lack complete visibility into the code executed during automated build phases.

## 3. Notable Real-World Supply Chain Incidents
- **The `event-stream` Compromise (npm)**: A popular utility package was transferred to a malicious maintainer via social engineering, subsequently injecting targeted cryptocurrency wallet-stealing payloads into downstream builds.
- **SolarWinds Orion Breach**: Threat actors compromised the build pipeline itself, inserting the Sunburst backdoor into authentic, cryptographically signed enterprise monitoring updates distributed to thousands of global organizations.
- **The `xz-utils` Backdoor (CVE-2024-3094)**: A state-sponsored persona invested years of patient open-source contributions before subtly injecting a multi-stage backdoor into liblzma tarballs targeting Linux OpenSSH authentication.

## 4. Learning Objectives in Lab A03
Throughout this hands-on lab, you will:
1. Understand lockfile mechanics (`package-lock.json`) and cryptographic integrity attributes.
2. Detect rogue dependency registrations and hazardous installation lifecycle hooks (`postinstall_script`).
3. Analyze pipeline secret exfiltration originating from unpinned, untrusted third-party packages.
4. Implement defense-in-depth mitigations utilizing Software Bill of Materials (SBOM) generation and lifecycle execution restrictions.
