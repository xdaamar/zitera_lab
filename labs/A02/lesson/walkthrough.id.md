# Panduan Praktik Terbimbing: Menemukan Celah Misconfiguration

### Tujuan Investigasi
Dalam latihan ini, kamu akan mempraktikkan proses recon dan inspeksi konfigurasi pada server gateway operasi internal di `http://127.0.0.1:8012`.

### Langkah 1: Kunjungi Aplikasi Target
- Buka alamat `http://127.0.0.1:8012` pada peramban web.
- Amati antarmuka halaman login dan footer informasi server.

### Langkah 2: Uji Kredensial Default
- Masukkan username `admin` dan password `admin`.
- Jika berhasil masuk, perhatikan menu administratif yang terbuka.

### Langkah 3: Periksa Endpoint Direktori Cadangan
- Arahkan URL peramban ke `http://127.0.0.1:8012/backups/`.
- Perhatikan bahwa fitur *directory listing* aktif dan menampilkan file backup konfigurasi.

### Langkah 4: Analisis File Cadangan
- Unduh file konfigurasi bernama `backup_config.json.bak`.
- Buka file tersebut dengan text editor dan perhatikan variabel rahasia `admin_system_key` yang tidak dienkripsi.
