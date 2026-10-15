# A05: Injection

## 1. What are Injection Vulnerabilities?
In the OWASP Top 10:2025 standard, **Injection** covers the entire family of security flaws where untrusted user-supplied input is relayed to a command interpreter or query processor as part of an active command or query string.

A naive interpreter cannot inherently distinguish between the **control plane** (authoritative instructions defined by software engineers) and the **data plane** (arbitrary values supplied by external users). When adversaries inject syntactic control delimiters (such as quotation marks `'`, boolean operators `OR`, semicolons `;`, or pipes `|`), the parser erroneously interprets attacker input as executable instructions.

## 2. Why Injection Remains in OWASP Top 10
Despite established architectural remediation patterns spanning decades, injection vulnerabilities persist across modern industries due to:
- **Legacy Monolithic Systems**: Massive enterprise architectures maintain millions of lines of legacy database drivers utilizing raw dynamic string concatenation.
- **Complex Ad-Hoc Query Assembly**: Dynamic filtering, multi-column search engines, and custom analytics reporting frequently lure engineers into assembling queries using string formatting.
- **Proliferation of Modern Interpreters**: Attack vectors extend far beyond relational SQL into OS Command Injection, NoSQL Injection (MongoDB query operators), LDAP/XPath queries, Server-Side Template Injection (SSTI), and LLM Prompt Injection.

## 3. Notable Real-World Injection Breaches
- **Equifax Data Breach (2017)**: An OGNL expression injection in Apache Struts (CVE-2017-5638) enabled remote OS shell command execution, leaking financial records of over 147 million consumers.
- **Sony Pictures Compromise (2011)**: Basic SQL injection attacks against public marketing portals permitted adversaries to dump complete user registries and cleartext administrator credentials.
- **TalkTalk Telecom Breach (2015)**: Automated SQL injection against legacy search parameters caused the exfiltration of sensitive banking data affecting over 150,000 subscribers.

## 4. Learning Objectives in Lab A05
Through the Central Logistics Warehouse Catalog on port 8015, you will:
1. Detect database parser anomalies triggered by injection test delimiters (`'`).
2. Enumerate query result column counts using systematic UNION operator probes.
3. Traverse internal SQLite schema metadata (`sqlite_master`) to map hidden storage structures.
4. Exfiltrate sensitive vault records from the `vault_secrets` table to recover the master flag in standard `ZITERA{...}` format.
5. Implement prepared statements and parameterized query architectures to definitively eliminate injection vectors.
