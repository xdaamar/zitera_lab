# A04: Cryptographic Failures — Kelemahan Enkripsi & Pembongkaran Hash

Cryptographic Failures menempati peringkat ke-4 (**#4**) dalam daftar risiko keamanan web **OWASP Top 10:2025** (sebelumnya dikenal sebagai *Sensitive Data Exposure*). Kerentanan ini terjadi ketika sistem menggunakan algoritma kriptografi usang yang telah terbukti rusak secara matematis, mengabaikan penggunaan *salt* acak pada penyimpanan kata sandi, atau gagal melindungi data sensitif saat transit dan saat tersimpan (*at rest*).

---

### 1. Alasan Kuat / Mengapa Ini Sangat Penting (Why It Matters)
Banyak organisasi perbankan dan enterprise keliru berasumsi bahwa "karena sudah di-hash, data pasti aman". Faktanya:

- **Bencana Sejarah Kebocoran Kredensial**:
  - **Kebocoran LinkedIn (117 Juta Akun)**: LinkedIn menyimpan password pengguna menggunakan SHA-1 tanpa *salt*. Akibatnya, lebih dari 90% password berhasil dibongkar penyerang dalam hitungan hari menggunakan tabel pencarian (*rainbow tables*).
  - **Insiden Adobe (153 Juta Akun)**: Menggunakan enkripsi simetris mode ECB (*Electronic Codebook*) tanpa IV acak, sehingga pola data identik menghasilkan ciphertext identik yang memudahkan peretas merekonstruksi password asli.
- **Sanksi Kepatuhan Standar Perbankan (PCI-DSS & ISO 27001)**:
  Standar keamanan industri kartu pembayaran (PCI-DSS v4.0) secara tegas melarang penggunaan MD5 dan SHA-1 untuk fungsi hashing password atau integritas data finansial. Pelanggaran standar ini berakibat pencabutan lisensi pemrosesan kartu kredit dan denda berat.

---

### 2. Gambaran Sistem yang Dihadapi (Target Commercial Architecture)
Dalam praktikum ini, Anda mengaudit gerbang pengelola kunci perbankan: **SafeKey Corporate Vault (Banking Encryption & Credential Vault)** yang beroperasi pada port `8014`.

Arsitektur sistem SafeKey Vault mencakup:
- **Banking HSM Appliance**: Mensimulasikan modul keamanan perangkat keras (Hardware Security Module) FIPS 140-2 Level 3 yang mengelola sertifikat interbank SWIFT dan sertifikat otoritas akar (Root CA).
- **Legacy Migration Audit Endpoint**: Rute `/api/audit/hashes` yang mengekspos tabel audit migrasi kredensial lama, membocorkan hash password operator perbankan tanpa *salt*.
- **Vault Authentication Gate**: Formulir masuk `/login` yang memverifikasi kata sandi pengguna dengan menghitung nilai hash MD5 sederhana dan mencocokkannya ke database.
- **Master Cryptographic Enclave**: Halaman rahasia `/vault` yang hanya dapat diakses oleh Security Officer (`admin`) dan menyimpan Master Banking Key Authorization Seal.

---

### 3. Sasaran, Temuan & Eksekusi (Search, Find, and Execute)
Metodologi audit kriptografi terstruktur:

1. **Apa yang Dicari (What to Search)**:
   - Endpoint audit kredensial atau ekspor data pengguna: `/api/audit/hashes`.
   - String hash heksadesimal dengan panjang 32 karakter (karakteristik MD5: 128-bit heksadesimal).
2. **Apa Temuan yang Menandakan Celah (Vulnerability Indicators)**:
   - Hash password disimpan dalam bentuk `MD5 (Unsalted)`.
   - Hash akun administrator (`admin`) adalah `21232f297a57a5a743894a0e4a801fc3`.
3. **Eksekusi Eksploitasi Terukur (Strategic Execution)**:
   - Gunakan tabel pencarian (*lookup table* / rainbow table) atau kalkulator hash untuk merekonstruksi teks asli (*plaintext*) dari hash tersebut (dihasilkan dari kata sandi sederhana `admin`).
   - Lakukan autentikasi pada formulir `/login` menggunakan akun `admin` dan kata sandi yang telah dipecahkan.
   - Buka konsol Master Key Vault di `/vault` dan eksfiltrasi segel enkripsi autoritatif `ZITERA{...}`.

---

### 4. Pondasi Kuat: Hashing vs Enkripsi vs Encoding
Pahami perbedaan mendasar agar tidak terjadi kerancuan terminologi:

| Konsep | Karakteristik | Sifat Pembalikan | Contoh Penggunaan Tepat |
| :--- | :--- | :--- | :--- |
| **Encoding** | Representasi format data (Base64, Hex) | Dapat dibalik tanpa kunci rahasia | Transmisi data biner via HTTP |
| **Encryption** | Pengacakan data menggunakan kunci kriptografis | Dapat didekripsi jika memiliki kunci | Kerahasiaan data tersimpan (AES-256-GCM) |
| **Hashing** | Fungsi matematika satu arah (*one-way*) | Tidak dapat didekripsi secara matematis | Verifikasi integritas & kata sandi |

> **Prinsip Pertahanan Modern:** Kata sandi **HARUS** disimpan menggunakan fungsi hash yang lambat (*memory-hard slow hashing functions*) seperti **Argon2id**, **bcrypt**, atau **PBKDF2** dengan *salt* acak unik per pengguna minimal 16-byte untuk menggagalkan komputasi GPU dan rainbow table.
