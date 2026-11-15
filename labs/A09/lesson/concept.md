# Core Concepts: Security Telemetry, Auditing & SIEM Alerting

Modern security operations depend upon three mutually reinforcing pillars: **Logging**, **Auditing**, and **Alerting**. A defect or omission in any of these components degrades the organization's overall Incident Detection and Response posture.

```
┌────────────────────────────────────────────────────────┐
│                   SYSTEM SECURITY TELEMETRY            │
├────────────────────┬───────────────────┬───────────────┤
│      LOGGING       │     AUDITING      │   ALERTING    │
├────────────────────┼───────────────────┼───────────────┤
│ Continuous stream  │ Tamper-evident,   │ Real-time     │
│ of system events   │ non-repudiable    │ notifications │
│ for diagnostics    │ proof of critical │ to SOC when   │
│ and troubleshooting│ user transactions │ anomaly lines │
│                    │ (compliance)      │ are crossed   │
└────────────────────┴───────────────────┴───────────────┘
```

---

## 1. Anatomy of a High-Fidelity Security Event Schema

Effective security logging goes beyond unstructured free-form text strings like `print("Login failed")`. Security events must adhere to a standardized, machine-parsable schema (such as JSON or CEF/CECS) that can be ingested, indexed, and analyzed automatically by SIEM systems (e.g., Splunk, Elastic SIEM, or Wazuh).

### Mandatory Attributes in Security Events:
1. **Timestamp**: UTC ISO 8601 with millisecond precision (`2026-11-15T12:00:00.123Z`).
2. **Event Identifier & Category**: Standardized taxonomy (e.g., `auth.attempt.failed` or `iam.privilege.escalated`).
3. **Actor Context**: Unique user identity (`user_id`, username, or hashed session ID).
4. **Network Origin**: Remote client IP address (`client_ip`), port, and client User-Agent string.
5. **Target Resource**: The entity or asset being manipulated (`resource_id`, `target_role`).
6. **Outcome**: Discrete result of the action (`SUCCESS`, `FAILURE`, `DENIED`).
7. **Severity Level**: Standardized severity rating (`DEBUG`, `INFO`, `WARN`, `HIGH`, `CRITICAL`).

```json
{
  "timestamp": "2026-11-15T12:00:00.123Z",
  "event_id": "SEC-EVT-8019-04",
  "category": "AUTHENTICATION",
  "action": "auth.login.failed",
  "severity": "WARN",
  "actor": {
    "username": "admin",
    "ip_address": "192.168.1.105",
    "user_agent": "curl/8.4.0"
  },
  "outcome": "FAILURE",
  "details": {
    "reason": "invalid_credentials",
    "consecutive_failures": 5
  }
}
```

---

## 2. Sensitive Data Prohibitions in Audit Streams

While logging too little creates detection blind spots, logging forbidden data creates severe privacy and regulatory liabilities (*Data Leakage via Logging*). Regulatory standards (PCI-DSS 4.0, GDPR, HIPAA) strictly forbid storing:
- Passwords, PINs, or passphrase secrets in cleartext.
- Full Primary Account Numbers (PANs) and CVV/CVC verification codes.
- Cryptographic private keys, seed phrases, or full session bearer tokens.
- Protected Health Information (PHI) and raw personally identifiable information (PII).

---

## 3. Log Injection Hazards (CWE-117: CRLF Injection)

When an application concatenates untrusted user inputs directly into plaintext log files without sanitization:
```python
# VULNERABLE: Direct string interpolation enables CRLF injection
app.logger.info(f"Failed login attempt for user: {username}")
```
An attacker can supply carriage return and line feed characters (`\r\n` or `%0d%0a`) inside the username parameter:
```
admin\r\n2026-11-15 12:00:00 [INFO] User admin logged in successfully.
```
This inserts a forged line into the log file, deceptive to security analysts conducting an incident review.

**Mitigation**: Always employ structured JSON logging where newline characters are automatically escaped to literal `\n` within valid JSON string properties.

---

## 4. SIEM Alerting Rules & Proactive Thresholds

Modern detection engineering does not wait for a breach to conclude before firing an alert. Automated rules must detect anomalies proactively:
- **Rule 1 (Brute-Force Attack)**: $\ge 5$ failed login attempts from a single IP address within a 5-minute window $\rightarrow$ Severity: HIGH, Trigger IP-level throttle.
- **Rule 2 (Stealth Privilege Escalation)**: Any role escalation event to `super_admin` or granting of root privileges $\rightarrow$ Severity: CRITICAL, Real-time dispatch to SOC incident channel.
- **Rule 3 (Distributed Identity Attack)**: Single user identity failing logins from $\ge 3$ distinct geographic subnets within 10 minutes $\rightarrow$ Severity: HIGH, Trigger mandatory MFA challenge.
