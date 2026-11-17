# Panduan Praktik Terbimbing: Investigasi & Audit CloudCore OpsGateway

Dalam latihan ini, Anda akan mempraktikkan proses penemuan dan pemanfaatan kesalahan konfigurasi keamanan pada portal manajemen infrastruktur **CloudCore OpsGateway (Enterprise Cloud Management & Microservices Monitor)** pada `http://127.0.0.1:8012`.

---

### Tahap 1: Rekonaisans & Fingerprinting Endpoint Diagnostik
1. Akses halaman beranda sistem pada peramban web:
   `http://127.0.0.1:8012/`
2. Lakukan inspeksi terhadap antarmuka operasional:
   - Amati bahwa portal menyediakan navigasi ke sumber daya diagnostik internal.
3. Lakukan uji coba terhadap endpoint variabel runtime sistem:
   `http://127.0.0.1:8012/debug/vars`
4. **Analisis Temuan:**
   - Server mengembalikan objek JSON berisi informasi lingkungan internal:
     - `debug_mode: true`
     - `default_credentials_enabled: true`
     - `backup_storage_path: "/app/backups/"`
     - Daftar rute sistem yang aktif.

---

### Tahap 2: Pengujian Kerentanan Directory Indexing
1. Akses jalur penyimpanan cadangan yang terungkap di endpoint diagnostik:
   `http://127.0.0.1:8012/backups/`
2. **Temuan Kritis:**
   - Server mengembalikan indeks direktori terbuka (*Directory Browsing / Autoindex Enabled*).
   - Terlihat file artefak cadangan sensitif: `backup_config.json.bak`.

---

### Tahap 3: Eksfiltrasi Arsip Cadangan & Kunci Sistem
1. Unduh file cadangan tersebut secara langsung dari browser atau gunakan terminal:
   ```bash
   curl -s "http://127.0.0.1:8012/backups/backup_config.json.bak" -o backup_config.json.bak
   ```
2. Buka dan periksa isi file JSON tersebut:
   ```bash
   cat backup_config.json.bak
   ```
3. **Analisis Bukti:**
   - File cadangan memuat konfigurasi produksi, nama database, pengguna administratif, dan properti `admin_system_key` berformat flag `ZITERA{...}`.
4. Uji login administratif ke konsol supervisor:
   - Kunjungi `http://127.0.0.1:8012/login`.
   - Masukkan kredensial default pabrik: Username `admin` dan Password `admin`.
   - Anda berhasil masuk ke **Root Cluster Administration Console** (`/admin`).
5. Salin flag validasi `ZITERA{...}` yang ditemukan di file backup untuk disubmit ke Zitera Lab desktop.

---

### Tahap 4: Remediasi & Verifikasi Hasil
Terapkan perbaikan pengerasan (*hardening*) konfigurasi pada server produksi:
1. Matikan opsi `directory_browsing` / nonaktifkan autoindex.
2. Cabut endpoint `/debug/vars` dari eksposur publik.
3. Paksa penggantian kredensial default pabrik pada instalasi pertama.
