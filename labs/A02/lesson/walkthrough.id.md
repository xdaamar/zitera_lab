# Panduan Praktik Terbimbing: Eksplorasi Celah Konfigurasi

Dalam latihan terarah ini, kamu akan mengaudit aplikasi portal internal yang berjalan pada alamat `http://127.0.0.1:8012`. Latihan ini disusun mengikuti rasio **60% teknik analisis manual** dan **40% penggunaan tools terminal**.

---

### Tahap 1: Observasi & Pemetaan Response Header (15% â€” Manual)
Langkah awal dalam audit konfigurasi adalah memeriksa header HTTP dan jejak informasi bawaan server (*information disclosure*).

1. Buka peramban web dan arahkan ke alamat target:
   `http://127.0.0.1:8012`
2. Buka Developer Tools (tekan **F12** atau `Ctrl + Shift + I`), lalu pilih tab **Network**.
3. Muat ulang halaman (*refresh*) dan klik request pertama (`/`):
   - **Periksa Response Headers:**
     - Apakah server membocorkan versi teknologi (misalnya `Server: Werkzeug/...` atau `Server: nginx/...`)?
     - Apakah header keamanan penting seperti `Content-Security-Policy`, `X-Frame-Options`, dan `X-Content-Type-Options` hilang?
4. Uji form login administratif:
   - Coba masukkan kredensial bawaan yang lazim: `admin` / `admin`. Amati bagaimana sistem menolak atau menerima autentikasi.

---

### Tahap 2: Pengujian Hipotesis & Penemuan Jalur Terbuka (25% â€” Manual)
Ujilah hipotesis: *Apakah terdapat folder statis atau direktori pemeliharaan yang lupa ditutup oleh administrator?*

1. Administrator sistem sering menempatkan file cadangan pada rute umum seperti `/backup/`, `/backups/`, `/data/`, atau `/debug/`.
2. Pada bilah alamat peramban, ketikkan URL:
   `http://127.0.0.1:8012/backups/`
3. Tekan **Enter** dan amati hasilnya:
   - **Observasi:** Server tidak menampilkan pesan error `403 Forbidden` atau `404 Not Found`, melainkan merender halaman direktori lengkap berjudul **"Index of /backups/"**.
   - **Temuan Bukti:** Terdapat file arsip cadangan bernama:
     `backup_config.json.bak`
   - **Analisis Kritis:** Fitur *autoindex* dibiarkan menyala, sehingga seluruh file cadangan yang ditinggalkan administrator dapat dilihat oleh publik tanpa memerlukan login apa pun.

---

### Tahap 3: Ekstraksi Rahasia & Weaponization (35% â€” Tools: cURL)
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

---

### Tahap 4: Remediasi & Verifikasi Hasil (25% â€” Manual & Tools)
Terapkan perbaikan pengerasan konfigurasi dan buktikan bahwa celah telah tertutup secara terverifikasi.

1. Matikan fitur directory browsing pada konfigurasi server:
   - Pastikan konfigurasi rute `/backups/` menerapkan aturan `autoindex off;` dan mengembalikan respons `404` atau `403`.
   - Pindahkan seluruh file backup ke penyimpanan terenkripsi di luar direktori web publik (*outside web root*).
2. Lakukan uji verifikasi ulang menggunakan cURL dari terminal:
   ```bash
   curl -s -i "http://127.0.0.1:8012/backups/"
   ```
3. **Kriteria Keberhasilan:** Server tidak lagi merender daftar isi file dan wajib mengembalikan kode status **`403 Forbidden`** atau **`404 Not Found`**.
