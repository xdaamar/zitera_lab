# Panduan Praktik Terbimbing: Eksploitasi SQLi UNION

### Tujuan Investigasi
Dalam latihan ini, kamu akan menguji fitur pencarian produk pada katalog di `http://127.0.0.1:8015` dan membongkar tabel rahasia `vault_secrets`.

### Langkah 1: Uji Karakter Khusus
- Buka `http://127.0.0.1:8015` pada browser.
- Pada kotak pencarian produk, ketik tanda petik tunggal: `'` lalu tekan Enter.
- Jika halaman menampilkan pesan error database (seperti syntax error), ini menandakan input pengguna diproses langsung oleh database engine.

### Langkah 2: Menentukan Jumlah Kolom
- Masukkan payload untuk menebak jumlah kolom:
  `' UNION SELECT 1, 2, 3 --`
- Jika halaman merespons tanpa error dan menampilkan angka 1, 2, 3 di tabel, berarti query asli memiliki tepat 3 kolom.

### Langkah 3: Ekstraksi Data dari Tabel Rahasia
- Ganti kolom tampilan dengan data dari tabel `vault_secrets`:
  `' UNION SELECT secret_name, secret_data, 0 FROM vault_secrets --`
- Amati tabel hasil di layar untuk membaca flag rahasia yang tersimpan di dalam database.
