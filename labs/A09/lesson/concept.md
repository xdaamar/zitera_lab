# Konsep Mendalam: Telemetri Keamanan, Audit, dan Alerting

Sistem pemantauan keamanan modern beroperasi di atas tiga pilar yang saling melengkapi: **Logging**, **Auditing**, dan **Alerting**. Kegagalan pada salah satu pilar ini melemahkan seluruh postur deteksi insiden (*incident detection & response*).

```
┌────────────────────────────────────────────────────────┐
│                      TELEMETRI SISTEM                   │
├────────────────────┬───────────────────┬───────────────┤
│      LOGGING       │     AUDITING      │   ALERTING    │
├────────────────────┼───────────────────┼───────────────┤
│ Mencatat seluruh   │ Jejak bukti tak   │ Peringatan    │
│ aliran event untuk │ terbantahkan      │ real-time ke  │
│ debugging dan      │ terkait aksi      │ tim SOC saat  │
│ operasional umum   │ pengguna kritis   │ ambang bahaya │
│                    │ (compliance)      │ terlampaui    │
└────────────────────┴───────────────────┴───────────────┘
```

---

## 1. Anatomi Peristiwa Keamanan Kritis (*Security Event Schema*)

Pencatatan log keamanan yang efektif tidak cukup hanya berupa pesan teks sembarang seperti `print("Login gagal")`. Log harus mengikuti skema terstruktur (seperti JSON) yang dapat di-ingest dan diindeks secara otomatis oleh SIEM (Security Information and Event Management) seperti Splunk, Elastic SIEM, atau Wazuh.

### Atribut Wajib dalam Security Event:
1. **Timestamp**: Format UTC ISO 8601 dengan presisi milidetik (`2026-11-15T12:00:00.123Z`).
2. **Event Identifier & Category**: Klasifikasi baku, misal `auth.attempt.failed` atau `iam.privilege.escalated`.
3. **Actor Context**: Pengenal unik pengguna (`user_id`, username, atau token ID parsial).
4. **Network Origin**: IP sumber (`client_ip`), port, dan User-Agent klien.
5. **Target Resource**: Objek atau entitas yang menjadi sasaran aksi (`resource_id`, `target_role`).
6. **Outcome**: Status keberhasilan aksi (`SUCCESS`, `FAILURE`, `DENIED`).
7. **Severity**: Klasifikasi keparahan (`DEBUG`, `INFO`, `WARN`, `HIGH`, `CRITICAL`).

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

## 2. Larangan Informasi Sensitif dalam Log (Data Privacy & Compliance)

Mencatat terlalu sedikit informasi menciptakan titik buta, namun mencatat data yang salah dapat menyebabkan kebocoran privasi (*Data Leakage via Logs*). Standar industri seperti PCI-DSS, GDPR, dan HIPAA melarang keras pencatatan:
- Kata sandi, PIN, atau *passphrase* dalam bentuk plaintext.
- Nomor kartu kredit lengkap (PAN) dan kode keamanan CVV/CVC.
- Kunci privat kriptografis atau token sesi otentikasi utuh.
- Data kesehatan pribadi (*Protected Health Information* - PHI).

---

## 3. Bahaya Log Injection (CWE-117: CRLF Injection)

Jika aplikasi mencatat input pengguna langsung ke dalam file teks log tanpa sanitasi:
```python
# RENTAN: CRLF Injection
app.logger.info(f"Login failed for user: {username}")
```
Penyerang dapat mengirimkan karakter baris baru (`\r\n` atau `%0d%0a`) dalam username:
```
admin\r\n2026-11-15 12:00:00 [INFO] User admin logged in successfully.
```
Tindakan ini menyisipkan baris palsu ke dalam berkas log, menipu analis keamanan yang mengira pengguna telah berhasil login secara sah.

**Mitigasi**: Selalu gunakan format terstruktur (JSON) di mana pemisah baris baru secara otomatis di-escape menjadi `\n` di dalam string JSON.

---

## 4. Ambang Batas Peringatan (*Alerting Thresholds*) dan Deteksi SIEM

Sistem deteksi modern tidak boleh menunggu penyerang berhasil membobol sistem baru mengeluarkan peringatan. Aturan deteksi anomali harus memicu alert secara proaktif:
- **Rule 1 (Brute-Force Detection)**: $\ge 5$ percobaan login gagal dari IP yang sama dalam interval 5 menit $\rightarrow$ Severity: HIGH, Trigger IP Throttling.
- **Rule 2 (Silent Privilege Escalation)**: Setiap perubahan peran ke `super_admin` atau penambahan hak `root` $\rightarrow$ Severity: CRITICAL, Notifikasi instan ke Channel Keamanan SOC.
- **Rule 3 (Distributed Probing)**: 1 akun mengalami kegagalan login dari $\ge 3$ negara/subnet berbeda dalam 10 menit $\rightarrow$ Severity: HIGH, Tantang Step-up MFA.
