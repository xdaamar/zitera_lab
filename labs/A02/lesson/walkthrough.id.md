# Panduan Praktik Terbimbing: Eksplorasi Celah Konfigurasi

Dalam latihan terarah ini, kamu akan mengaudit aplikasi portal cloud internal **OpsGateway** yang berjalan pada alamat `http://127.0.0.1:8012`. Latihan ini disusun mengikuti rasio **60% teknik analisis manual** dan **40% penggunaan tools terminal**.

---

### Tahap 1: Observasi & Pemetaan Response Header (15% — Manual)
Langkah awal dalam audit konfigurasi adalah memeriksa header HTTP dan jejak informasi bawaan server (*information disclosure*).

1. Buka peramban web dan arahkan ke alamat target:
   `http://127.0.0.1:8012`
2. Buka Developer Tools (tekan **F12** atau `Ctrl + Shift + I`), lalu pilih tab **Network**.
3. Muat ulang halaman (*refresh* / F5) dan klik request pertama (`/`):
   - **Periksa Request Name:** Request pertama tercatat sebagai `/` dengan kode status `200 OK` (langsung diproxy oleh engine tanpa pengalihan URL acak).
   - **Periksa Response Headers:**
     - Server membocorkan versi teknologi: `Server: OpsGateway/2.1.4 (Ubuntu)`.
     - Server mengaktifkan flag operasional berbahaya: `X-Debug-Mode: enabled` dan `X-Gateway-Cluster: prod-east-gateway-01`.
4. Amati antarmuka web OpsGateway:
   - Terdapat kartu telemetri *Storage Archival* yang mengindikasikan direktori `/backups/`.
   - Terdapat ringkasan audit log dan inventaris node cloud.
   - Klik menu navigasi atau tombol *Sign In* (`/login`) untuk mengamati portal supervisor autentikasi.

---

### Tahap 2: Pengujian Hipotesis & Penemuan Jalur Terbuka (25% — Manual)
Ujilah hipotesis: *Apakah terdapat folder statis atau direktori pemeliharaan yang lupa ditutup oleh administrator?*

1. Administrator sistem sering menempatkan file cadangan pada rute umum seperti `/backup/`, `/backups/`, `/data/`, atau `/debug/`.
2. Pada bilah alamat peramban, ketikkan URL:
   `http://127.0.0.1:8012/backups/`
3. Tekan **Enter** dan amati hasilnya:
   - **Observasi:** Server tidak menampilkan pesan error `403 Forbidden` atau `404 Not Found`, melainkan merender halaman direktori lengkap berjudul **"Index of /backups/"**.
   - **Temuan Bukti:** Terdapat file arsip cadangan penting:
     - `backup_config.json.bak`
     - `ops_keys_old.txt`
     - `database_schema.sql.bak`
   - **Analisis Kritis:** Fitur *autoindex* dibiarkan aktif pada web server Nginx/OpsGateway, sehingga seluruh file cadangan yang ditinggalkan administrator dapat diunduh bebas oleh publik tanpa autentikasi.

---

### Tahap 3: Ekstraksi Rahasia & Weaponization (35% — Tools: cURL)
Gunakan tools terminal cURL untuk mengunduh artefak cadangan dan mengurai kredensial rahasia yang terkandung di dalamnya.

1. Buka terminal di dalam workspace ZITERA_LAB.
2. Unduh file cadangan tersebut secara langsung menggunakan cURL:
   ```bash
   curl -s "http://127.0.0.1:8012/backups/backup_config.json.bak" -o backup_config.json.bak
   ```
3. Periksa isi file cadangan yang baru saja diunduh:
   ```bash
   # Tampilkan isi payload JSON konfigurasi cadangan
   cat backup_config.json.bak
   ```
4. **Analisis Payload:** Amati properti konfigurasi sensitif seperti `admin_system_key`, koneksi database internal, dan flag aktivasi sistem yang bocor di dalam dokumen teks tersebut.
5. Akses portal supervisor `/admin` menggunakan kunci token rahasia yang ditemukan untuk mendapatkan konfirmasi flag `ZITERA{53cur1ty_m15c0nf1g_b4ckup_134k}`.

---

### Tahap 4: Remediasi & Verifikasi Hasil (25% — Manual & Tools)
Terapkan perbaikan pengerasan konfigurasi dan buktikan bahwa celah telah tertutup secara terverifikasi.

1. Matikan fitur directory browsing pada konfigurasi server:
   - Pastikan konfigurasi rute `/backups/` menerapkan aturan `autoindex off;` dan mengembalikan respons `404` atau `403`.
   - Pindahkan seluruh file backup ke penyimpanan terenkripsi di luar direktori web publik (*outside web root*).
2. Lakukan uji verifikasi ulang menggunakan cURL dari terminal:
   ```bash
   curl -s -i "http://127.0.0.1:8012/backups/"
   ```
3. **Kriteria Keberhasilan:** Server tidak lagi merender daftar isi file dan wajib mengembalikan kode status **`403 Forbidden`** atau **`404 Not Found`**.
