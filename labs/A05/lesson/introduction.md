# A05: Injection

Injection flaws occur when untrusted user data is sent to an interpreter as part of a command or query.

The interpreter is tricked into executing unintended commands or accessing data without proper authorization.

## Common Types of Injection
1. **SQL Injection (SQLi):** Malicious inputs alter database queries.
2. **Command Injection (OS Command Injection):** User input executed directly in the host OS shell.
3. **NoSQL Injection:** Manipulating MongoDB / JSON query objects.
4. **LDAP / XPath Injection:** Manipulating directory services or XML trees.

In OWASP Top 10:2025, Injection remains one of the most destructive vulnerability classes because successful exploitation often leads to full database takeover, data exfiltration, or remote code execution.
