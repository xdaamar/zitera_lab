# Introduction to Software Supply Chain Failures

## What is a Software Supply Chain Failure?
In modern software engineering, developers rarely write all code from scratch. Over 80% to 90% of a typical application's codebase consists of open-source libraries, package managers (npm, PyPI, Cargo, Maven), CI/CD pipelines, and base container images.

**OWASP A03:2025: Software Supply Chain Failures** addresses vulnerabilities introduced into an application not by the team's custom application code, but through third-party dependencies, malicious build plugins, or compromised upstream package repositories.

## Why This Matters
When an upstream library with millions of downloads is compromised (e.g. `event-stream`, `solarwinds`, or `xz-utils`), every downstream organization that builds or deploys software using that library inherits the vulnerability automatically.
