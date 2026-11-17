# Panduan Praktik Terbimbing: Audit Rantai Pasok pada PackNexus Supply Hub

Dalam panduan terarah ini, Anda akan mempraktikkan proses audit forensik terhadap sistem registri paket dan manifest build **PackNexus Supply Hub (B2B Package Registry & Build Pipeline Hub)** pada `http://127.0.0.1:8013`.

---

### Tahap 1: Observasi Registri & Manifest Dependensi
1. Buka peramban web dan akses portal PackNexus Supply Hub:
   `http://127.0.0.1:8013`
2. Perhatikan ringkasan dependensi aktif pada tabel antarmuka:
   - Paket `express` (v4.19.2) - Terverifikasi Sigstore.
   - Paket `lodash` (v4.17.21) - Terverifikasi Sigstore.
   - Paket `apex-internal-telemetry` (v0.1.0-alpha.build.88) - Berstatus **Untrusted / Unpinned Origin**.
3. Klik tombol **"Download package_lock_audit.json &rarr;"** atau akses endpoint:
   `http://127.0.0.1:8013/packages/audit/package_lock_audit.json`

---

### Tahap 2: Analisis Forensik File Lockfile
1. Unduh atau tampilkan manifest audit menggunakan terminal:
   ```bash
   curl -s http://127.0.0.1:8013/packages/audit/package_lock_audit.json
   ```
2. Telusuri objek dependensi `apex-internal-telemetry`:
   - **Resolved Origin:** URL lokal tidak resmi alih-alih repositori resmi npmjs.
   - **Hook Eksekusi:** Menjalankan perintah `node ./scripts/send_env.js` pada siklus `postinstall_script`.
   - **Temuan Bukti:** Ditemukan token rahasia pipeline yang tereksfiltrasi pada field `exfiltrated_pipeline_token`: `ZITERA{...}`.

---

### Tahap 3: Verifikasi Attestation Token & Penyelesaian Misi
1. Ujilah token yang ditemukan pada formulir **Pipeline Attestation Verifier** di halaman utama portal web.
2. Atau lakukan pengujian langsung via cURL:
   ```bash
   curl -X POST http://127.0.0.1:8013/api/supply-chain/verify \
     -H "Content-Type: application/x-www-form-urlencoded" \
     -d "token=ZITERA{...}"
   ```
3. Server akan mengembalikan konfirmasi keberhasilan: `{"status": "passed", "message": "Token verified!..."}`.
4. Kirimkan (*submit*) flag berformat `ZITERA{...}` ke Zitera Lab desktop untuk merekam skor Anda.
