# A02: Security Misconfiguration

Security Misconfiguration is ranked **#2** in the OWASP Top 10:2025. It represents the single most pervasive and easily exploitable class of security vulnerabilities across modern enterprise architectures, cloud environments, and containerized deployments.

---

### Learning Objectives
Upon completing this module, you will be able to:
1. Identify common security misconfigurations across web servers, application frameworks, and containerized runtimes.
2. Detect exposed static directory browsing (*unauthenticated directory indexing*) and abandoned backup archives (*backup file leaks*).
3. Conduct manual audits and HTTP security header inspections using browser developer tools and cURL.
4. Implement automated defensive configuration controls (*Infrastructure as Code hardening*, disabling directory listings, and error message sanitization).

---

### The Architecture Spectrum of Misconfiguration
Unlike localized code injection flaws, security misconfigurations can manifest at any layer of the technology stack:

| Stack Layer | Common Misconfiguration Example | Potential Impact |
| :--- | :--- | :--- |
| **Web Framework** | Active debug mode (`DEBUG=True`), verbose stack traces | Exposure of `SECRET_KEY`, filesystem paths, DB credentials |
| **Web Server (Nginx/Apache)** | `autoindex on`, exposed `/server-status` diagnostic portals | Unauthenticated visitors can browse and download all assets |
| **Credential Management** | Unchanged vendor defaults (`admin` / `admin`, `root` / `root`) | Trivial, immediate full takeover of administrative interfaces |
| **HTTP Headers** | Missing `Content-Security-Policy`, `X-Frame-Options` headers | Heightened susceptibility to Clickjacking, XSS, and MIME sniffing |
| **Cloud & Containers** | Leaving `.env`, `.git/`, or `.bak` archives in public web roots | Direct exfiltration of production API keys and database strings |

---

### Why Attackers Prioritize Misconfigurations
Adversaries do not need to spend weeks analyzing complex application logic if a server administrator inadvertently exposes `/backups/` or retains default credentials. Using lightweight automated scanners, attackers can scan thousands of endpoints and seize control within minutes.
