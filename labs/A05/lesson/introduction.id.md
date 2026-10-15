# A05: Injection (Injeksi Perintah & Query)

## 1. Apa itu Kerentanan Injection?
Dalam klasifikasi OWASP Top 10:2025, **Injection** mencakup seluruh kelas kerentanan di mana data masukan yang tidak tepercaya (*untrusted user input*) dikirimkan ke interpreter perintah atau mesin pemroses query sebagai bagian dari instruksi eksekusi.

Interpreter yang naif tidak memiliki kecerdasan untuk membedakan batas antara **kode instruksi resmi pengembang** (*control plane*) dan **data murni dari pengguna** (*data plane*). Akibatnya, ketika penyerang memasukkan karakter kontrol khusus (seperti tanda petik `'`, operator boolean `OR`, titik koma `;`, atau pipa `|`), interpreter mengeksekusi data tersebut sebagai sintaks perintah tambahan.

## 2. Mengapa Injection Tetap Bertengger di OWASP Top 10?
Meskipun konsep pencegahan injeksi telah dikenal luas selama beberapa dekade, kerentanan ini tetap mendominasi laporan insiden keamanan global karena:
- **Warisan Kode Monolitik (*Legacy Codebases*)**: Ribuan aplikasi enterprise masih mengandalkan konkatenasi string SQL mentah yang ditulis sebelum adopsi ORM modern.
- **Kompleksitas Query Dinamis**: Fitur pencarian tingkat lanjut, filter multi-kriteria, dan pelaporan kustom sering kali menggoda pengembang untuk menyusun query secara ad-hoc menggunakan format string.
- **Keberagaman Interpreter Modern**: Vektor injeksi tidak hanya terbatas pada SQL, tetapi juga merambah ke Command Injection (OS Shell), NoSQL Injection (MongoDB/CouchDB), LDAP Injection, XML/XPath Injection, hingga Server-Side Template Injection (SSTI) dan LLM Prompt Injection.

## 3. Insiden Bersejarah Akibat Serangan Injection
- **Kebocoran Data Equifax (2017)**: Kerentanan Command Injection pada parser Apache Struts OGNL (CVE-2017-5638) memungkinkan penyerang mengeksekusi perintah shell sistem operasi secara remote, membocorkan data pribadi lebih dari 147 juta konsumen.
- **Peretasan Sony Pictures (2011)**: Serangan SQL Injection sederhana pada formulir web publik berhasil mengekstraksi seluruh kredensial pengguna, kata sandi teks terbuka, dan informasi rahasia produksi film.
- **Kasus TalkTalk Telecom (2015)**: Pelajar berusia 15 tahun menggunakan alat otomatis SQLi untuk membobol database pelanggan karena parameter pencarian web tidak menggunakan prepared statements.

## 4. Sasaran Pembelajaran di Lab A05
Melalui lab katalog inventaris logistik pada port 8015, Anda akan:
1. Mengidentifikasi perilaku anomali interpreter database saat menerima karakter penguji injeksi (`'`).
2. Menentukan jumlah kolom yang dikembalikan oleh query asli menggunakan teknik probe UNION.
3. Menelusuri metadata skema database internal (`sqlite_master`) untuk memetakan tabel rahasia.
4. Mengekstrak data kredensial dari tabel `vault_secrets` dan mengambil master flag berformat `ZITERA{...}`.
5. Mempelajari arsitektur pertahanan berbasis *Prepared Statements / Parameterized Queries*.
