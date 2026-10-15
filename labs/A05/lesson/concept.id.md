# Pembahasan Teknis: Anatomi dan Taksonomi Serangan SQL Injection

Kerentanan SQL Injection berakar pada satu kegagalan fundamental: **pencampuran jalur instruksi kode dan jalur data masukan (*conflation of control and data planes*)**.

---

## 1. Akar Masalah: Konkatenasi String Dinamis
Perhatikan kode Python yang rentan di bawah ini:
```python
search_term = request.args.get('q', '')
# KESALAHAN FATAL: Menempelkan input pengguna langsung ke sintaks SQL
query = f"SELECT name, category, price FROM products WHERE name LIKE '%{search_term}%'"
cursor.execute(query)
```

Jika pengguna memasukkan kata kunci normal `Server`, mesin database mengeksekusi:
```sql
SELECT name, category, price FROM products WHERE name LIKE '%Server%'
```

Namun, jika pengguna memasukkan karakter petik tunggal (`'`), batas string SQL terputus. Input pengguna kini dianggap sebagai bagian dari struktur bahasa SQL itu sendiri!

---

## 2. Tiga Kategori Utama SQL Injection

### A. In-Band SQLi (Classic SQLi)
Penyerang menggunakan saluran komunikasi yang sama untuk menyuntikkan payload dan membaca hasil eksfiltrasi data secara langsung pada halaman web:
1. **Error-Based SQLi**: Memanipulasi query agar sengaja memicu pesan kesalahan internal database yang mencantumkan potongan data sensitif (misal: pesan error konversi tipe data).
2. **UNION-Based SQLi**: Memanfaatkan operator `UNION` untuk menggabungkan hasil query sah aplikasi dengan hasil query kustom penyerang.

### B. Inferential SQLi (Blind SQLi)
Aplikasi tidak menampilkan data database atau pesan error pada layar. Penyerang harus merekonstruksi data karakter demi karakter dengan mengajukan pertanyaan bernilai True/False:
1. **Boolean-Based Blind**: Mengamati perbedaan konten halaman web ketika kondisi boolean benar (`AND 1=1`) dibandingkan saat salah (`AND 1=2`).
2. **Time-Based Blind**: Memaksa database menunda respons jaringan selama durasi tertentu (misal via fungsi `pg_sleep(5)` atau `WAITFOR DELAY '0:0:5'`) jika tebakan karakter benar.

### C. Out-of-Band SQLi (OOB)
Digunakan ketika penyerang tidak bisa melihat respons langsung maupun mengandalkan latensi waktu. Penyerang memicu server database untuk mengirimkan paket DNS lookup atau HTTP request ke server Command & Control milik penyerang (misal melalui fitur `xp_dirtree` di MS SQL Server atau `UTL_HTTP` di Oracle).

---

## 3. Anatomi Serangan UNION-Based SQL Injection
Operator `UNION` menggabungkan baris hasil dari dua query `SELECT` yang terpisah. Agar mesin database bersedia mengeksekusi perintah `UNION`, dua syarat mutlak harus terpenuhi:

### Syarat 1: Jumlah Kolom Wajib Sama Persis
Query yang diinjeksikan harus mengembalikan jumlah kolom yang identik dengan query asli aplikasi. Penyerang menentukan jumlah kolom menggunakan teknik inkremental:
```sql
' ORDER BY 1 --  (Sukses)
' ORDER BY 2 --  (Sukses)
' ORDER BY 3 --  (Sukses)
' ORDER BY 4 --  (Error: database melaporkan kolom 4 tidak ada -> Jumlah kolom adalah 3!)
```
Atau menggunakan teknik NULL injection:
```sql
' UNION SELECT NULL, NULL, NULL --
```

### Syarat 2: Tipe Data Kolom Harus Kompatibel
Tipe data setiap kolom pada query kedua harus dapat dikonversikan ke tipe data kolom yang bersesuaian pada query pertama (misalnya teks string ke kolom bertipe string).

---

## 4. Ekstraksi Skema pada Database SQLite
Pada mesin database SQLite (seperti yang digunakan di Lab A05), seluruh metadata skema, nama tabel, dan definisi kolom disimpan dalam tabel internal bernama `sqlite_master`:
```sql
-- Membaca seluruh nama tabel yang ada di dalam database
' UNION SELECT tbl_name, sql, 0 FROM sqlite_master WHERE type='table' --
```
Begitu tabel target tersembunyi teridentifikasi (misal `vault_secrets`), penyerang menyusun query akhir untuk mengekstrak data rahasia:
```sql
' UNION SELECT secret_name, secret_data, 0 FROM vault_secrets --
```
Karakter komentar ganda `--` di akhir payload berfungsi memerintahkan parser database untuk mengabaikan seluruh sisa string query bawaan aplikasi.
