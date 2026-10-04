# Panduan Praktik Terbimbing: Membongkar Hash Kata Sandi

### Tujuan Investigasi
Dalam latihan ini, kamu akan mengaudit vault manajemen kunci di `http://127.0.0.1:8014` dan memulihkan kata sandi akun administrator dari log hash yang tidak aman.

### Langkah 1: Akses Sistem Target
- Buka alamat `http://127.0.0.1:8014` pada peramban web.
- Periksa menu login vault dan navigasi ke halaman dokumentasi audit.

### Langkah 2: Temukan Log Audit Hash
- Akses endpoint `/api/audit/hashes` atau periksa tab log enkripsi.
- Temukan catatan akun `admin` yang menyimpan hash password berbentuk heksadesimal 32 karakter (MD5).

### Langkah 3: Identifikasi dan Pecahkan Hash
- Perhatikan nilai hash akun administrator (misal: `5f4dcc3b5aa765d61d8327deb882cf99`).
- Gunakan database kamus hash publik atau alat decoder lokal untuk mengembalikan nilai aslinya.

### Langkah 4: Autentikasi dan Ambil Flag
- Kembali ke halaman login utama dan masuk menggunakan username `admin` serta kata sandi hasil dekripsi yang kamu dapatkan.
- Akses brankas data untuk mengambil master flag.
