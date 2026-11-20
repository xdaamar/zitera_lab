# A05: Injection (SQL Injection) — Pembobolan Database Lewat Kotak Input

Injection menempati peringkat ke-5 (**#5**) dalam daftar risiko **OWASP Top 10:2025**. Kerentanan ini terjadi ketika data yang dimasukkan oleh pengguna (*untrusted user input*) disisipkan langsung ke dalam perintah atau query basis data (SQL, NoSQL, OS Command, LDAP) tanpa validasi atau pemisahan antara instruksi kode dan data masukan.

---

### 1. Kenapa Ini Sangat Berbahaya? (Business & Financial Impact)
SQL Injection (SQLi) adalah salah satu kerentanan web paling legendaris dan berbahaya dalam sejarah internet:

- **Pengambilalihan Seluruh Basis Data Perusahaan**:
  Dengan satu baris karakter jahat pada kotak pencarian produk atau form login, penyerang dapat membaca seluruh isi database (daftar nasabah, saldo rekening, kartu kredit, riwayat belanja), mengubah data secara sepihak, hingga menghapus seluruh tabel (*DROP TABLE*).
  - Kasus nyata insiden **Heartland Payment Systems**: Penyerang menggunakan SQL Injection untuk menembus jaringan pembayaran dan mencuri lebih dari 130 juta nomor kartu kredit, menimbulkan kerugian lebih dari $140 juta USD.
- **Sanksi Regulasi Finansial & Pidana**:
  Kebocoran data akibat SQL Injection merupakan bentuk kelalaian fatal pengembang perangkat lunak (*gross negligence*) di bawah **UU PDP No. 27/2022** dan standar perbankan **PCI-DSS**, berisiko denda hingga puluhan miliar rupiah serta tuntutan ganti rugi perdata dari jutaan konsumen yang dirugikan.

---

### 2. Bagaimana Cara Mencari & Mengujinya? (Recon & Vulnerability Indicators)
Metodologi investigasi SQL Injection profesional:

1. **Apa yang Dicari (What to Search)**:
   - Formulir pencarian, filter produk, parameter URL (seperti `?q=`, `?id=`, `?category=`), atau form input login.
2. **Apa Temuan yang Menandakan Celah (Vulnerability Indicators)**:
   - Memasukkan karakter kutip tunggal (`'`) memicu pesan kesalahan database (*SQL syntax error / database exception*):  
     `unrecognized token: "'%'"`
   - Memasukkan payload logika boolean `' OR 1=1 --` menampilkan seluruh rekaman tanpa filter.
3. **Eksekusi Eksploitasi Terukur (Strategic Execution)**:
   - Identifikasi jumlah kolom query asli (3 kolom: `name`, `category`, `price`).
   - Gunakan teknik **UNION-Based Injection** untuk menggabungkan hasil pencarian dengan data dari tabel rahasia `vault_secrets`.
   - Ekstraksi entri rahasia `PROJECT_ZITERA_CORE_FLAG` untuk mengungkap flag autoritatif `ZITERA{5q1_1nj3ct10n_m45t3r_2026}`.

---

### 3. Bagaimana Cara Memperbaikinya? (Secure Coding & Remediation)
Pertahanan mutlak terhadap SQL Injection:
- **Wajib Gunakan Parameterized Queries (Prepared Statements)**: Pisahkan jalur instruksi SQL dengan data input. Mesin database tidak akan pernah mengeksekusi data masukan sebagai sintaks perintah.
- **Gunakan ORM Terpercaya**: Gunakan Object-Relational Mapper (seperti SQLAlchemy, Prisma, Hibernate) dengan aman, dan hindari menyusun query mentah (*raw string concatenation*).
- **Terapkan Prinsip Hak Akses Terkecil (Least Privilege)**: Akun database yang digunakan aplikasi web tidak boleh memiliki hak akses administratif atau hak membaca tabel rahasia sistem jika tidak diperlukan.

---

### Gambaran Sistem yang Dihadapi (Target Commercial Architecture)
Dalam praktikum ini, kamu menghadapi portal inventaris e-commerce komersial: **MegaStore Global / Central Logistics Hardware (Commercial E-Commerce & Retail Inventory Portal)** yang beroperasi pada port `8015`.

Komponen arsitektur:
- **Katalog Inventaris Produk**: Endpoint pencarian `GET /?q=<search_query>` yang mengambil data barang dari tabel `products`.
- **Celah Penggabungan String Mentah**: Handler Python menyusun query SQL menggunakan format string mentah:  
  `SELECT name, category, price FROM products WHERE name LIKE '%{q}%'`
- **Tabel Brankas Tersembunyi**: Tabel database SQLite internal `vault_secrets` yang menyimpan token rahasia sistem dan bendera CTF.
