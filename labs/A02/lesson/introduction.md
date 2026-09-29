# A02: Security Misconfiguration

Security Misconfiguration is ranked **#2 in the OWASP Top 10:2025**. It represents the most common and pervasive class of vulnerabilities found across modern web stacks.

## What is Security Misconfiguration?
Security misconfiguration occurs when security controls are inaccurately defined, configured with insecure defaults, left incomplete, or poorly maintained across any part of the application stack.

This includes:
- Unhardened default configurations (e.g. default usernames and passwords).
- Improperly configured cloud permissions or storage buckets.
- Unnecessary features, ports, services, or pages enabled (e.g. debug endpoints or sample applications).
- Overly verbose error messages disclosing stack traces or internal environment variables.
- Missing or misconfigured security headers (CORS, CSP, HSTS).
- Directory browsing / indexing left enabled on static content directories.

## Why Attackers Target Misconfigurations
Attackers frequently run automated scanners to detect default pages, unpatched services, and unprotected administrative ports. Because misconfigurations require zero exploitation of complex business logic, they represent the lowest-friction entry point for adversaries.
