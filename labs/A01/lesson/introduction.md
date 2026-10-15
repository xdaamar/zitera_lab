# A01: Broken Access Control

Broken Access Control occupies the top spot (**#1**) in the OWASP Top 10:2025 security risks. It occurs when a web application or API fails to strictly enforce restrictions on what authenticated users are permitted to do, allowing users to access data, functions, or assets belonging to other entities beyond their legitimate authorization boundaries.

---

### Learning Objectives
Upon completing this module, you will be able to:
1. Explain the fundamental difference between **Authentication (AuthN)** and **Authorization (AuthZ)**.
2. Identify vulnerable patterns of **Insecure Direct Object References (IDOR)** and **Broken Object Level Authorization (BOLA)** across HTTP requests.
3. Conduct manual audits of multi-tenant boundary isolation using browser developer tools and cURL.
4. Implement defensive coding patterns (*server-side ownership verification*) to permanently eliminate access control flaws.

---

### The Critical Distinction: AuthN vs AuthZ
Developers often mistakenly assume an application is secure simply because it requires a strong login (authentication) workflow:

| Security Concept | Core Question | Typical Mechanisms | Common Failure Modes |
| :--- | :--- | :--- | :--- |
| **Authentication (AuthN)** | *"Who are you?"* | Password, MFA, JWT signature, biometrics | Default credentials, credential stuffing, token forgery |
| **Authorization (AuthZ)** | *"What are you allowed to do?"* | Role-based ACL, ownership checks, tenant filtering | **Broken Access Control**, IDOR, privilege escalation |

> **Key Rule:** Broken Access Control is almost always a failure of the **Authorization** layer, not Authentication. The attacker has typically logged in legally using a low-privilege account, then manipulated request parameters to access resources owned by other users.

---

### Why Is Broken Access Control So Dangerous?
When authorization enforcement is omitted on the server:
- **Multi-Tenant Boundary Violations:** A user belonging to Tenant A can view invoices, statements, and confidential records of Tenant B.
- **Mass Data Exfiltration:** Attackers can perform trivial parameter iteration (e.g. `id=1, 2, 3...`) to systematically harvest entire customer databases.
- **Privilege Escalation:** Standard users can invoke administrative endpoints simply by tampering with URL paths or HTTP verbs.
- **Regulatory Non-Compliance:** Directly triggers major non-compliance penalties under data protection regulations (such as GDPR, HIPAA, and Indonesia's PDP Law No. 27/2022).
