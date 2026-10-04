# Panduan Praktik Terbimbing: Mengeksploitasi Cacat Alur Bisnis

### Tujuan Investigasi
Dalam latihan ini, kamu akan menguji sistem alur workflow pada portal gateway di `http://127.0.0.1:8016` untuk menemukan celah manipulasi status transaksi.

### Langkah 1: Kunjungi Dashboard Target
- Buka alamat `http://127.0.0.1:8016` pada peramban web.
- Amati alur proses transaksi formulir dari tahap inisiasi hingga verifikasi akhir.

### Langkah 2: Analisis Request API dengan DevTools
- Buka tab Network pada Developer Tools (F12).
- Amati payload JSON yang dikirimkan saat menekan tombol submit.

### Langkah 3: Uji Cacat Transisi Status
- Perhatikan parameter transisi atau exception condition yang dapat dipicu langsung via request POST tanpa otorisasi bertingkat.
- Ekstrak respons yang memuat flag konfirmasi otoritas.
