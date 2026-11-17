# A04: Cryptographic Failures — Kelemahan Sandi & Kriptografi Usang

Cryptographic Failures menempati peringkat ke-4 (**#4**) dalam daftar risiko **OWASP Top 10:2025**. Kerentanan ini sebelumnya dikenal sebagai *Sensitive Data Exposure*, berfokus pada kegagalan sistem dalam melindungi kerahasiaan (*confidentiality*) dan integritas (*integrity*) data sensitif menggunakan algoritma kriptografi yang kuat dan modern.

---

### 🔴 1. Kenapa Ini Sangat Berbahaya? (Business & Financial Impact)
Kriptografi adalah benteng pertahanan terakhir data sensitif saat database berhasil dibobol penyerang. Jika algoritma kriptografinya lemah:

- **Kebocoran Kredensial Pengguna Massal**:
  Jika penyerang berhasil mencuri tabel pengguna yang memuat hash password MD5 atau SHA-1 tanpa salt, jutaan kata sandi nasabah dapat didekripsi dalam hitungan detik menggunakan tabel pencarian siap pakai (*Rainbow Tables*).
  - Kasus nyata kebocoran data raksasa seperti insiden **LinkedIn (2012)**: 6,5 juta hash SHA-1 tanpa salt bocor ke forum hacker, di mana lebih dari 90% password berhasil dibongkar dalam waktu kurang dari seminggu.
- **Sanksi Kepatuhan Hukum Berat**:
  Standar industri perbankan (**PCI-DSS v4.0**), regulasi privasi internasional (**GDPR**), dan undang-undang nasional (**UU PDP No. 27/2022**) secara eksplisit melarang penggunaan MD5 dan SHA-1 untuk melindungi kredensial atau nomor kartu kredit. Pelanggaran mengakibatkan pencabutan lisensi operasional sistem pembayaran dan denda puluhan miliar rupiah.

---

### 🔍 2. Bagaimana Cara Mencari & Mengujinya? (Recon & Vulnerability Indicators)
Metodologi audit kriptografi profesional:

1. **Apa yang Dicari (What to Search)**:
   - Endpoint audit, telemetri, atau ekspor data kredensial: `/api/audit/hashes`, `/api/v1/users/export`.
   - String hash berukuran 32 karakter heksadesimal (MD5) atau 40 karakter heksadesimal (SHA-1).
   - Indikator ketiadaan salt (karakter acak tambahan yang unik per pengguna).
2. **Apa Temuan yang Menandakan Celah (Vulnerability Indicators)**:
   - Endpoint audit mengembalikan hash bertipe `MD5 (Unsalted)` atau `Legacy Migration Hash`.
   - Hash `21232f297a57a5a743894a0e4a801fc3` untuk pengguna `admin` langsung cocok dengan kamus kata sandi standar (`admin`).
3. **Eksekusi Eksploitasi Terukur (Strategic Execution)**:
   - Lakukan pembalikan hash (*hash reversal / lookup*) terhadap hash administrator.
   - Lakukan autentikasi sah pada form login `/login` menggunakan kredensial hasil pemecahan.
   - Akses enklave brankas master `/vault` untuk mengekstraksi flag autoritatif `ZITERA{cryp70_f41lur35_w34k_k3y_2026}`.

---

### 🛡️ 3. Bagaimana Cara Memperbaikinya? (Secure Coding & Remediation)
Panduan pengerasan kriptografi standar:
- **Tinggalkan MD5 dan SHA-1**: Gunakan fungsi hash adaptif yang sengaja dibuat lambat dan tahan serangan GPU: **Argon2id** (standar emas saat ini), **bcrypt** (work factor $\ge 12$), atau **PBKDF2** (iterasi $\ge 600.000$).
- **Gunakan Salt Unik Kriptografis Secara Otomatis**: Setiap password wajib diberi salt acak unik sebelum di-hash agar nilai hash dari kata sandi yang sama menjadi berbeda bagi tiap pengguna.
- **Lindungi Data Bergerak & Diam**: Gunakan TLS 1.3 untuk komunikasi jaringan, dan enkripsi data tersimpan (*data-at-rest*) menggunakan AES-256-GCM.

---

### Gambaran Sistem yang Dihadapi (Target Commercial Architecture)
Dalam laboratorium ini, kamu menghadapi brankas kredensial perbankan: **SafeKey Corporate Vault / CryptoVault KMS (Banking Key Management & Credential Vault)** yang beroperasi pada port `8014`.

Komponen arsitektur:
- **Legacy Audit Endpoint**: Jalur `/api/audit/hashes` yang mengekspos hash kata sandi staf perbendaharaan tanpa salt.
- **Authentication Gateway**: Endpoint `/login` yang memvalidasi kecocokan hash masukan pengguna dengan hash database.
- **Master Cryptographic Vault**: Enklave tertutup `/vault` yang hanya dapat diakses setelah login sukses sebagai Chief Information Security Officer (`admin`).
