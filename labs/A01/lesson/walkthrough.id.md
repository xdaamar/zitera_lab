# Panduan Praktik Terbimbing: Investigasi Celah IDOR

### Tujuan Investigasi
Dalam latihan terarah ini, kamu akan mempelajari bagaimana parameter data dikirimkan antara browser dan server target pada alamat `http://127.0.0.1:8011`, serta menemukan letak kegagalan otorisasi.

### Langkah 1: Masuk Menggunakan Akun Uji Coba
- Buka alamat `http://127.0.0.1:8011` pada peramban web (browser).
- Lakukan login menggunakan kredensial standar:
  - Username: `alice`
  - Password: `password123`

### Langkah 2: Amati Struktur URL dan Parameter
- Masuk ke menu *"My Invoices"* (Faktur Saya).
- Perhatikan struktur URL di address bar browser:
  `http://127.0.0.1:8011/invoice/1`

### Langkah 3: Periksa Lalu Lintas Jaringan (Network Inspection)
- Buka menu Developer Tools di browser (tekan tombol F12 atau Ctrl + Shift + I).
- Pilih tab Network, lalu refresh halaman.
- Amati response data dan perhatikan bahwa nilai `id: 1` adalah representasi akun milik Alice.

### Langkah 4: Uji Manipulasi Objek (Parameter Tampering)
- Ubah nilai parameter pada URL browser dari `/invoice/1` menjadi `/invoice/2`.
- Hasil Pengamatan: Data faktur tagihan rahasia milik pengguna Bob langsung muncul di layar!
- Akar Masalah (Root Cause): Backend langsung menjalankan query berdasarkan ID dari URL tanpa memverifikasi apakah akun pengguna yang sedang login (`session['user_id']`) adalah pemilik sah dari faktur nomor 2 tersebut.
