# Panduan Praktik Terbimbing: Eksploitasi Autentikasi Tanpa Rate Limit

### Tujuan Investigasi
Dalam latihan ini, kamu akan menguji halaman login administrasi pada sistem di `http://127.0.0.1:8017` dan menemukan kata sandi PIN akun admin yang rentan brute force.

### Langkah 1: Akses Halaman Login
- Buka alamat `http://127.0.0.1:8017` pada browser.
- Amati form login yang meminta username dan PIN numerik 4 digit.

### Langkah 2: Uji Respon Login Gagal
- Masukkan username `admin` dan PIN acak seperti `0000`.
- Perhatikan bahwa sistem merespons langsung tanpa memberi peringatan cooldown atau pembatasan percobaan.

### Langkah 3: Eksekusi Tebakan Terarah
- PIN admin merepresentasikan tahun rilis laboratorium Zitera (antara rentang 2020 - 2030).
- Uji PIN tahun saat ini (misal `2026`).

### Langkah 4: Autentikasi Sukses & Ambil Flag
- Saat PIN yang tepat dimasukkan, portal langsung mengalihkan sesi ke dashboard admin dan menampilkan flag tantangan.
