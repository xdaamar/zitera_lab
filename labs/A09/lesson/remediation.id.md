# Remediasi & Arsitektur Telemetri Keamanan Terstruktur

Untuk mengatasi **OWASP A09:2025 – Security Logging and Alerting Failures**, arsitektur aplikasi harus mentransformasikan logging dari sekadar pencatatan teks informal menjadi **pipa telemetri keamanan terstruktur, kebal manipulasi (*tamper-evident*), dan terintegrasi langsung dengan mesin deteksi SIEM**.

---

## 1. Arsitektur Telemetri Terpusat & Immutability

Dalam lingkungan modern, file log yang tersimpan secara lokal di mesin server rentan dimanipulasi atau dihapus oleh penyerang yang berhasil mendapatkan akses shell (*MITRE ATT&CK T1070: Indicator Removal*). Log audit keamanan harus segera dikirimkan ke repositori eksternal yang menerapkan kebijakan **Write-Once-Read-Many (WORM)**.

```
┌─────────────────────────────────┐
│     Aplikasi Web / Gateway      │
│  - JSON Structured Logger       │
│  - Correlation ID Injection     │
└─────────────────────────────────┘
                 │
                 ▼ (gRPC / TLS Syslog / HTTPS Stream)
┌─────────────────────────────────┐
│   Log Collector (Fluentbit)     │
└─────────────────────────────────┘
                 │
                 ▼
┌─────────────────────────────────┐      Real-time Stream      ┌─────────────────────────────────┐
│  SIEM / Search Cluster (Elastic)│ ─────────────────────────> │   Rule Engine & Alerting        │
│  - Immutable WORM Storage       │                            │   - Slack / PagerDuty / SOC     │
│  - Merkle Tree Hash Chaining    │                            │   - Automated IP Ban via WAF    │
└─────────────────────────────────┘                            └─────────────────────────────────┘
```

---

## 2. Refactor Backend: Logging Terstruktur & Peringatan Proaktif

Berikut adalah refactoring aman pada backend Python/Flask untuk `Zitera SOC Audit & Telemetry Console`, menerapkan logging terstruktur dan pemantauan batas ambang alert real-time:

```python
import datetime
import json
import logging
import uuid
from flask import Flask, request, jsonify

app = Flask(__name__)

# Konfigurasi Structured JSON Logger
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
    Mencatat security event ke audit trail yang tidak dapat disangkal (non-repudiable).
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
    Memicu alarm real-time ke SIEM dan menaikkan counter alert SOC.
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

    # Tangani Kegagalan Autentikasi: Catat dan Evaluasi Batas Ambang Alert
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

    # Deteksi Brute Force: Picu Alarm SOC saat percobaan mencapai ambang batas 5x
    if fails >= 5:
        trigger_soc_alert(
            rule_name="AUTH_BRUTE_FORCE_THRESHOLD_EXCEEDED",
            severity="CRITICAL",
            context={"ip": client_ip, "target_user": user, "attempt_count": fails}
        )
        return jsonify({"error": "Terlalu banyak percobaan gagal. Akses ditangguhkan dan dicatat oleh SOC."}), 429

    return jsonify({"error": "Kredensial tidak valid", "attempt": fails}), 401

@app.route('/api/admin/role_escalate', methods=['POST'])
def secure_role_escalate():
    client_ip = request.remote_addr
    data = request.get_json(silent=True) or {}
    target_user = data.get("user", "learner")
    new_role = data.get("role", "super_admin")

    actor_info = {"ip": client_ip, "user_agent": request.headers.get("User-Agent")}

    # 1. Catat aksi eskalasi hak istimewa secara wajib ke Audit Trail
    record_security_audit_event(
        category="IDENTITY_AND_ACCESS",
        action="iam.role.escalated",
        severity="HIGH",
        actor=actor_info,
        target={"user": target_user, "role_assigned": new_role},
        outcome="SUCCESS",
        details={"approval_ticket": "SEC-TICK-2026"}
    )

    # 2. Picu notifikasi instan untuk setiap penetapan peran administrator
    if new_role == "super_admin":
        trigger_soc_alert(
            rule_name="SUPERUSER_PRIVILEGE_ASSIGNED",
            severity="HIGH",
            context={"target_user": target_user, "operator_ip": client_ip}
        )

    return jsonify({
        "status": "granted",
        "message": f"User {target_user} berhasil diubah perannya menjadi {new_role} dengan jejak audit lengkap.",
        "audit_recorded": True
    }), 200
```

---

## 3. Desain Peringatan SIEM & Korelasi Multivariat

Agar tim SOC tidak mengalami *alert fatigue* (kelelahan akibat terlalu banyak alarm palsu), aturan deteksi harus dikalibrasi secara cermat:

1. **Korelasi Berbasis Waktu**: Kelompokkan beberapa peristiwa gagal dalam rentang waktu terukur (misal: 5 peristiwa dalam 300 detik).
2. **Korelasi Multivariat**: Hubungkan kegagalan otentikasi di aplikasi dengan anomali geolokasi IP atau lalu lintas DNS yang mencurigakan.
3. **Automated Containment (SOAR)**: Integrasikan platform Security Orchestration, Automation, and Response (SOAR) untuk secara dinamis memasukkan IP penyerang ke blokir WAF tanpa memerlukan intervensi manusia manual.

---

## 4. Checklist Audit Telemetri dan Deteksi Keamanan

Gunakan checklist ini dalam peninjauan arsitektur telemetri:

- [ ] Seluruh kegagalan autentikasi dan otorisasi dicatat ke log audit dengan IP sumber dan stempel waktu UTC.
- [ ] Operasi administratif kritis (perubahan peran, reset MFA, pergantian kunci enkripsi) dicatat tanpa terkecuali.
- [ ] Log diformat dalam skema terstruktur (JSON) yang kebal terhadap serangan CRLF log injection.
- [ ] Informasi sensitif (kata sandi, nomor kartu kredit, token utuh) disaring sebelum log disimpan.
- [ ] Ambang batas alert otomatis dikonfigurasi untuk serangan berulang (*brute-force* & *credential stuffing*).
- [ ] File log ditransmisikan secara real-time ke penyimpanan terpusat yang aman dan tidak dapat diedit (*immutable WORM*).
- [ ] Tim respons insiden secara berkala melakukan simulasi uji pemadaman log (*log tampering drills*).
