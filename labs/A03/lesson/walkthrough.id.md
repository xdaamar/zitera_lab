# Panduan Praktik Terbimbing: Investigasi Lockfile Audit & Eksfiltrasi Dependensi

Panduan ini disusun menggunakan metodologi 4-Tier Mission Progression dengan proporsi **60% observasi manual** dan **40% eksekusi perintah alat**:

---

## Tier 1: Observasi Permukaan Serangan (Bobot: 15%)

1. **Pastikan Kontainer Lab Aktif**:
   Buka peramban web dan arahkan ke alamat target:
   `http://127.0.0.1:8013`

2. **Analisis Halaman Utama Registry**:
   Amati antarmuka web *ApexCorp // Internal Package Registry*:
   - Terdapat tabel dependensi aktif (*Active Registry Dependencies*).
   - Perhatikan baris ketiga pada tabel: paket `apex-internal-telemetry` versi `0.1.0-alpha.build.88` memiliki status berwarna kuning dengan tanda `UNTRUSTED / UNPINNED`.
   - Sumber unduhan paket mengarah ke alamat lokal (`http://127.0.0.1:8013/packages/download/`) bukan registry resmi npm.
   - Di bagian atas tabel terdapat tautan langsung ke laporan audit: `/packages/audit/package_lock_audit.json`.

---

## Tier 2: Penemuan Endpoint & Injeksi Permintaan (Bobot: 25%)

1. **Uji Health Endpoint**:
   Verifikasi kesiapan layanan menggunakan terminal atau peramban:
   ```bash
   curl -s http://127.0.0.1:8013/health
   ```
   *Respon*: `{"lab":"A03","port":8013,"status":"ok"}`

2. **Inspeksi Manual Laporan Lockfile**:
   Buka tautan laporan audit pada peramban web atau jalankan curl untuk mengunduh manifest mentah:
   ```bash
   curl -s http://127.0.0.1:8013/packages/audit/package_lock_audit.json
   ```

3. **Analisis Struktur Dokumen JSON**:
   Perhatikan bahwa dokumen tersebut memiliki versi schema lockfile npm (`lockfileVersion: 3`) dan memuat objek `dependencies`:
   - Paket `express` (versi `4.19.2`) diunduh dari `https://registry.npmjs.org/` dengan hash integritas sha512.
   - Paket `lodash` (versi `4.17.21`) terverifikasi dengan hash integritas resmi.
   - Paket `apex-internal-telemetry` memiliki atribut yang tidak wajar.

---

## Tier 3: Eksploitasi & Ekstraksi Token Eksfiltrasi (Bobot: 35%)

1. **Bedah Metadata Paket Mencurigakan**:
   Tinjau blok JSON untuk `apex-internal-telemetry`:
   ```json
   "apex-internal-telemetry": {
     "version": "0.1.0-alpha.build.88",
     "resolved": "http://127.0.0.1:8013/packages/download/apex-internal-telemetry.tgz",
     "author": "shadow-developer-anonymous",
     "integrity": "sha512-poisoned_build_hash_xyz",
     "postinstall_script": "node ./scripts/send_env.js",
     "exfiltrated_pipeline_token": "ZITERA{...}",
     "warning": "CRITICAL: Package not signed with corporate GPG key; untrusted origin."
   }
   ```

2. **Identifikasi Muatan Berbahaya**:
   - `author`: `shadow-developer-anonymous` (pengembang anonim yang tidak dikenal).
   - `postinstall_script`: `node ./scripts/send_env.js` (hook eksekusi otomatis yang membaca variabel rahasia lingkungan CI/CD).
   - `exfiltrated_pipeline_token`: Token otorisasi build pipeline yang berhasil dicuri oleh skrip tersebut.

3. **Verifikasi Flag**:
   Kirimkan token yang ditemukan melalui form web pada halaman utama (`/api/supply-chain/verify`) atau menggunakan perintah curl:
   ```bash
   curl -s -X POST http://127.0.0.1:8013/api/supply-chain/verify -d "token=<TOKEN_DARI_LOCKFILE>"
   ```
   *Respon*:
   ```json
   {
     "status": "passed",
     "message": "Flag verified! Supply chain backdoored dependency successfully audited.",
     "flag": "ZITERA{...}"
   }
   ```

---

## Tier 4: Validasi Perbaikan & Hardening Defensif (Bobot: 25%)

1. **Nonaktifkan Eksekusi Lifecycle Hook**:
   Pastikan setiap instalasi paket dalam pipeline CI/CD menggunakan argumen proteksi:
   ```bash
   npm ci --ignore-scripts
   ```

2. **Audit Integritas Lockfile**:
   Jalankan pemeriksaan dependensi otomatis sebelum proses kompilasi:
   ```bash
   npm audit --audit-level=high
   ```

3. **Verifikasi Scope Paket Internal**:
   Pastikan seluruh paket internal diberi namespace terdaftar (misal `@apexcorp/`) dan registry publik dikonfigurasi untuk menolak namespace internal tersebut.
