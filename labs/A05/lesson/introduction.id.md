# A05: Injection (SQL Injection) — Pembobolan Logika Basis Data

Injection menempati peringkat ke-5 (**#5**) dalam daftar risiko web **OWASP Top 10:2025**. Kerentanan SQL Injection (SQLi) terjadi ketika data masukan yang dikirimkan oleh pengguna disatukan langsung secara mentah (*concatenation*) ke dalam perintah query database tanpa melalui parameterisasi atau sanitasi ketat. Akibatnya, mesin basis data salah menafsirkan data pengguna sebagai instruksi eksekusi kode SQL baru.

---

### 1. Alasan Kuat / Mengapa Ini Sangat Penting (Why It Matters)
Meskipun SQL Injection telah dikenal selama lebih dari dua dekade, kerentanan ini tetap menjadi salah satu vektor serangan paling destruktif di industri perangkat lunak:

- **Bencana Industri & Kerugian Finansial Dahsyat**:
  - **Kasus Heartland Payment Systems**: Penyerang menyusupkan malware melalui SQL Injection pada aplikasi web pengolah pembayaran, mencuri lebih dari 130 juta nomor kartu kredit dan merugikan perusahaan lebih dari $140 juta USD dalam bentuk denda dan kompensasi hukum.
  - **Serangan TalkTalk**: Serangan SQLi sederhana pada situs web telekomunikasi Inggris menyebabkan pencurian data 156.000 pelanggan, denda ratusan ribu poundsterling oleh ICO, dan penurunan nilai pasar perusahaan hingga jutaan pound.
- **Dampak Teknis Kritis**:
  Eksploitasi SQLi yang berhasil memungkinkan penyerang melewati autentikasi login, mengekstrak seluruh isi database (data pribadi, nomor kartu kredit, hash password), mengubah saldo rekening, hingga menulis file shell pada sistem operasi server (*RCE via SQL*).

---

### 2. Gambaran Sistem yang Dihadapi (Target Commercial Architecture)
Dalam laboratorium ini, Anda mengaudit portal katalog grosir enterprise: **MegaStore Global (Commercial E-Commerce & Retail Inventory Portal)** yang beroperasi pada port `8015`.

Arsitektur aplikasi MegaStore Global meliputi:
- **E-Commerce Web Portal**: Antarmuka katalog produk perangkat keras enterprise (Server, Switch, Firewall, Kabel) dengan fitur pencarian teks dinamis.
- **Search Controller**: Menerima parameter GET query `?q=` dari pengguna untuk menyaring inventaris.
- **Relational Database Tier (SQLite)**:
  - Tabel publik `products`: Menyimpan informasi SKU, nama produk, kategori, harga, dan stok.
  - Tabel terisolasi `vault_secrets`: Menyimpan kredensial rahasia inti korporasi `PROJECT_ZITERA_CORE_FLAG`.
- **Cacat Arsitektur Injeksi**: Controller menyusun string query SQL secara langsung menggunakan format string Python:
  `raw_sql = f"SELECT name, category, price FROM products WHERE name LIKE '%{query}%'"`
  Hal ini memungkinkan penyerang memecah sintaks string query dan menyisipkan klausul `UNION SELECT`!

---

### 3. Sasaran, Temuan & Eksekusi (Search, Find, and Execute)
Metodologi eksploitasi SQL Injection terarah:

1. **Apa yang Dicari (What to Search)**:
   - Form pencarian, filter kategori, parameter GET/POST: `?q=`, `?search=`, `?category=`.
2. **Apa Temuan yang Menandakan Celah (Vulnerability Indicators)**:
   - Memasukkan karakter kutip tunggal (`'`) memicu pesan kesalahan basis data mentah di antarmuka web:
     `Internal Database Exception: unrecognized token: "'%'"`
   - Ini membuktikan bahwa input pengguna masuk langsung ke mesin parser SQL SQLite!
3. **Eksekusi Eksploitasi Terukur (Strategic Execution)**:
   - Lakukan penyeimbangan kutip dan komentar query: `' OR 1=1 --`.
   - Hitung jumlah kolom target (`SELECT name, category, price` memiliki 3 kolom).
   - Eksekusi klausul `UNION SELECT` untuk mengekstrak data dari tabel rahasia:
     `' UNION SELECT secret_name, secret_data, 0 FROM vault_secrets --`
   - Ambil flag validasi autoritatif berformat `ZITERA{...}` yang tertampil pada tabel produk!

---

### 4. Pondasi Kuat: Pemisahan Mutlak Data vs Kode
Akar masalah SQL Injection adalah tercampurnya **Data** dengan **Instruksi Eksekusi**:
- **Pendekatan Rentan**: Menyambung string (`query = "SELECT * FROM ... " + user_input`).
- **Pendekatan Defensif Mutlak (Prepared Statements)**: Mengirimkan template query SQL ke mesin database terlebih dahulu untuk dikompilasi ke dalam struktur pohon sintaks (*AST*), baru kemudian mengirimkan parameter data secara terpisah. Dengan prepared statements, input apapun (bahkan yang memuat `' OR 1=1 --`) akan selalu diperlakukan murni sebagai nilai teks, bukan instruksi perintah.
