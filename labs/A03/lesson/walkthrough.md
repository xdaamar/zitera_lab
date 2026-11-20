# Panduan Praktik Terbimbing: Audit Rantai Pasok pada PackNexus Supply Hub

Dalam panduan terarah ini, kamu akan mempraktikkan proses audit forensik terhadap sistem registri paket dan manifest build **PackNexus Supply Hub (B2B Package Registry & Build Pipeline Hub)** yang beroperasi pada `http://127.0.0.1:8013`.

---

### Langkah 1: Observasi Registri & Pemeriksaan Kesehatan Layanan
1. Pastikan target lab telah berjalan dan uji ketersediaan layanan melalui terminal:
   ```bash
   curl -s http://127.0.0.1:8013/health
   ```
   *Respons yang diharapkan:*
   ```json
   {"lab":"A03","port":8013,"status":"ok"}
   ```
2. Buka peramban web dan akses portal PackNexus Supply Hub:
   `http://127.0.0.1:8013`
3. Perhatikan tabel **Active Registry Dependencies**:
   - Paket `express` (v4.19.2) - Berstatus resmi terverifikasi (*Sigstore Verified*).
   - Paket `lodash` (v4.17.21) - Berstatus resmi terverifikasi (*Sigstore Verified*).
   - Paket `apex-internal-telemetry` (v0.1.0-alpha.build.88) - Berstatus **UNTRUSTED / UNPINNED** dengan peringatan merah menyala!

---

### Langkah 2: Mengunduh & Menganalisis Manifest `package_lock_audit.json`
1. Buka tautan laporan audit di browser atau unduh file JSON audit menggunakan cURL:
   ```bash
   curl -s http://127.0.0.1:8013/packages/audit/package_lock_audit.json
   ```
2. **Analisis Struktur Dokumen JSON:**
   Perhatikan objek `dependencies` pada manifest tersebut. Paket `express` dan `lodash` memiliki asal repositori resmi npmjs serta checksum sha512 yang valid.
3. Namun, periksa entri paket mencurigakan `apex-internal-telemetry`:
   ```json
   "apex-internal-telemetry": {
     "version": "0.1.0-alpha.build.88",
     "resolved": "http://127.0.0.1:8013/packages/download/apex-internal-telemetry.tgz",
     "author": "shadow-developer-anonymous",
     "integrity": "sha512-poisoned_build_hash_xyz",
     "postinstall_script": "node ./scripts/send_env.js",
     "exfiltrated_pipeline_token": "ZITERA{5upply_ch41n_p0150n1ng_d3p_2026}",
     "warning": "CRITICAL: Package not signed with corporate GPG key; untrusted origin."
   }
   ```

> [!NOTE]
> **💡 Di Balik Layar: Mengapa skrip `postinstall` berbahaya?**  
> Manajer paket seperti `npm` secara default menjalankan skrip `postinstall` segera setelah paket selesai diunduh. Jika dependensi disusupi, skrip ini berjalan dengan hak akses penuh mesin pengembang atau server CI/CD, mampu membaca variabel lingkungan (`process.env`) dan mengirimkan token rahasia ke server penyerang!

---

### Langkah 3: Ekstraksi Token & Verifikasi Attestation Pipeline
1. Pada atribut `exfiltrated_pipeline_token`, temukan flag validasi kepatuhan:
   `ZITERA{5upply_ch41n_p0150n1ng_d3p_2026}`
2. Lakukan pengesahan token melalui formulir **Pipeline Attestation Verifier** di antarmuka web, atau kirimkan via cURL:
   ```bash
   curl -s -X POST http://127.0.0.1:8013/api/supply-chain/verify \
     -H "Content-Type: application/x-www-form-urlencoded" \
     -d "token=ZITERA{5upply_ch41n_p0150n1ng_d3p_2026}"
   ```
3. **Respons Konfirmasi Keberhasilan:**
   ```json
   {
     "status": "passed",
     "message": "Token verified! Supply chain audit completed successfully.",
     "verified_token": "ZITERA{5upply_ch41n_p0150n1ng_d3p_2026}"
   }
   ```
4. Salin flag `ZITERA{5upply_ch41n_p0150n1ng_d3p_2026}` dan kirimkan (*submit*) ke Zitera Lab desktop untuk menyelesaikan tantangan ini!
