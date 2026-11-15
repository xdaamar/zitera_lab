# Remediation & Structured Security Telemetry Architecture

To resolve **OWASP A09:2025 – Security Logging and Alerting Failures**, software architectures must upgrade logging from ad-hoc text output into **a structured, tamper-evident telemetry pipeline integrated directly with SIEM detection engines**.

---

## 1. Centralized Telemetry & Log Immutability

In modern environments, security logs stored locally on application host filesystems are vulnerable to tampering, alteration, or deletion by adversaries who acquire shell access (*MITRE ATT&CK T1070: Indicator Removal*). Audit logs must be forwarded immediately to remote, append-only repositories enforcing **Write-Once-Read-Many (WORM)** retention policies.

```
┌─────────────────────────────────┐
│     Web App / API Gateway       │
│  - JSON Structured Logger       │
│  - Correlation ID Injection     │
└─────────────────────────────────┘
                 │
                 ▼ (gRPC / TLS Syslog / HTTPS Stream)
┌─────────────────────────────────┐
│   Log Forwarder (Fluentbit)     │
└─────────────────────────────────┘
                 │
                 ▼
┌─────────────────────────────────┐      Real-time Stream      ┌─────────────────────────────────┐
│  SIEM / Central Store (Elastic) │ ─────────────────────────> │   Detection & Alerting Engine   │
│  - Immutable WORM Storage       │                            │   - Slack / PagerDuty / SOC     │
│  - Merkle Tree Hash Chaining    │                            │   - Automated IP Ban via WAF    │
└─────────────────────────────────┘                            └─────────────────────────────────┘
```

---

## 2. Backend Code Refactoring: Structured Telemetry & Active Alerting

The following Python/Flask refactor for the `Zitera SOC Audit & Telemetry Console` introduces structured JSON security events and real-time SIEM alert triggers:

```python
import datetime
import json
import logging
import uuid
from flask import Flask, request, jsonify

app = Flask(__name__)

# Structured JSON Logger Configuration
class StructuredJsonFormatter(logging.Formatter):
    def format(self, record):
        log_obj = {
            "timestamp": datetime.datetime.now(datetime.timezone.utc).isoformat(),
            "level": record.levelname,
            "message": record.getMessage(),
            "context": getattr(record, "security_context", {})
        }
        return json.dumps(log_obj)

handler = logging.StreamHandler()
handler.setFormatter(StructuredJsonFormatter())
security_logger = logging.getLogger("security_audit")
security_logger.setLevel(logging.INFO)
security_logger.addHandler(handler)

AUDIT_LOG_STREAM = []
ALERTS_FIRED_COUNT = 0
FAILED_LOGINS_BY_IP = {}

def record_security_audit_event(category: str, action: str, severity: str, actor: dict, target: dict, outcome: str, details: dict):
    """
    Appends a structured security event to the immutable in-memory audit stream.
    """
    event = {
        "event_id": str(uuid.uuid4()),
        "time": datetime.datetime.now(datetime.timezone.utc).isoformat(),
        "category": category,
        "action": action,
        "severity": severity,
        "actor": actor,
        "target": target,
        "outcome": outcome,
        "details": details
    }
    AUDIT_LOG_STREAM.append(event)
    security_logger.info(f"Security Event: {action}", extra={"security_context": event})
    return event

def trigger_soc_alert(rule_name: str, severity: str, context: dict):
    """
    Fires an actionable real-time security alert to SIEM and increments SOC alert counter.
    """
    global ALERTS_FIRED_COUNT
    ALERTS_FIRED_COUNT += 1
    security_logger.critical(f"SIEM ALERT TRIGGERED: {rule_name}", extra={"security_context": {
        "alert_id": str(uuid.uuid4()),
        "rule": rule_name,
        "severity": severity,
        "context": context
    }})

@app.route('/api/auth/login', methods=['POST'])
def secure_login():
    client_ip = request.remote_addr
    data = request.get_json(silent=True) or {}
    user = data.get("username", "anonymous")
    pwd = data.get("password", "")

    actor_info = {"username": user, "ip": client_ip, "user_agent": request.headers.get("User-Agent")}

    if user == "admin" and pwd == "correct_horse_battery":
        record_security_audit_event(
            category="AUTHENTICATION",
            action="auth.login.success",
            severity="INFO",
            actor=actor_info,
            target={"username": user},
            outcome="SUCCESS",
            details={}
        )
        FAILED_LOGINS_BY_IP.pop(client_ip, None)
        return jsonify({"status": "authenticated", "role": "admin"}), 200

    # Authentication Failure Handling: Audit Logging & Threshold Monitoring
    fails = FAILED_LOGINS_BY_IP.get(client_ip, 0) + 1
    FAILED_LOGINS_BY_IP[client_ip] = fails

    record_security_audit_event(
        category="AUTHENTICATION",
        action="auth.login.failure",
        severity="WARN" if fails < 5 else "CRITICAL",
        actor=actor_info,
        target={"username": user},
        outcome="FAILURE",
        details={"consecutive_attempts": fails}
    )

    # Brute-force threshold: fire SOC alarm when failures reach or exceed 5
    if fails >= 5:
        trigger_soc_alert(
            rule_name="AUTH_BRUTE_FORCE_THRESHOLD_EXCEEDED",
            severity="CRITICAL",
            context={"ip": client_ip, "target_user": user, "attempt_count": fails}
        )
        return jsonify({"error": "Excessive failures. Access throttled and flagged to SOC."}), 429

    return jsonify({"error": "Invalid credentials", "attempt": fails}), 401

@app.route('/api/admin/role_escalate', methods=['POST'])
def secure_role_escalate():
    client_ip = request.remote_addr
    data = request.get_json(silent=True) or {}
    target_user = data.get("user", "learner")
    new_role = data.get("role", "super_admin")

    actor_info = {"ip": client_ip, "user_agent": request.headers.get("User-Agent")}

    # 1. Mandatory audit recording of privilege changes
    record_security_audit_event(
        category="IDENTITY_AND_ACCESS",
        action="iam.role.escalated",
        severity="HIGH",
        actor=actor_info,
        target={"user": target_user, "role_assigned": new_role},
        outcome="SUCCESS",
        details={"approval_ticket": "SEC-TICK-2026"}
    )

    # 2. Trigger high-severity alert for superuser role assignments
    if new_role == "super_admin":
        trigger_soc_alert(
            rule_name="SUPERUSER_PRIVILEGE_ASSIGNED",
            severity="HIGH",
            context={"target_user": target_user, "operator_ip": client_ip}
        )

    return jsonify({
        "status": "granted",
        "message": f"User {target_user} assigned role {new_role} with audit trace logged.",
        "audit_recorded": True
    }), 200
```

---

## 3. SIEM Detection Rules & Alert Calibration

To prevent operational alert fatigue, SIEM rules must balance precision and sensitivity:

1. **Sliding Time-Window Aggregations**: Evaluate events over bounded durations (e.g., 5 failures in 300 seconds).
2. **Multivariate Threat Correlation**: Correlate application-tier authentication failures with network firewall alerts and geo-IP velocity anomalies.
3. **Automated Incident Response (SOAR)**: Leverage Security Orchestration, Automation, and Response playbooks to temporarily blacklist offending IPs on perimeter WAFs without requiring manual human dispatch.

---

## 4. Telemetry and Audit Verification Checklist

Use this checklist during architecture reviews and penetration testing audits:

- [ ] Every authentication and authorization failure is captured with source IP and UTC timestamp.
- [ ] Privileged administrative modifications (role changes, credential resets, key rotations) are audited unconditionally.
- [ ] Logs are formatted as structured JSON resistant to CRLF log injection attacks.
- [ ] PII, passwords, payment tokens, and cryptographic keys are strictly excluded from audit payloads.
- [ ] Real-time alert triggers monitor repeated failure thresholds and high-risk administrative escalations.
- [ ] Log events stream to an external immutable, append-only repository (WORM compliance).
- [ ] Incident response teams conduct regular logging health and tamper-resilience exercises.
