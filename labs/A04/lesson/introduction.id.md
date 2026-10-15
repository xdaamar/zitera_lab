# A04: Cryptographic Failures (Kegagalan Kriptografi)

## 1. Apa itu Cryptographic Failures?
Dalam klasifikasi OWASP Top 10:2025, **Cryptographic Failures** (sebelumnya dikenal sebagai *Sensitive Data Exposure*) berfokus pada kegagalan yang berkaitan dengan kriptografi—atau ketiadaan penerapannya—yang mengakibatkan kebocoran data sensitif atau kompromi total terhadap sistem.

Kriptografi adalah benteng pertahanan terakhir: ketika lapisan pertahanan perimeter (seperti firewall atau kontrol akses jaringan) berhasil ditembus penyerang, enkripsi dan hashing yang kuat menjamin bahwa data yang dicuri tetap tidak dapat dibaca (*unreadable*) dan tidak dapat dimanipulasi (*tamper-proof*).

## 2. Mengapa Kegagalan Kriptografi Terjadi?
Sebagian besar insiden keamanan kriptografi bukan diakibatkan oleh penyerang yang berhasil memecahkan matematika di balik algoritma modern, melainkan oleh:
- **Penggunaan Algoritma yang Sudah Usang & Rusak**: Menggunakan fungsi hash seperti MD5 atau SHA-1 untuk kata sandi, atau sandi simetris usang seperti DES dan RC4.
- **Penyimpanan Hash Tanpa Salt (*Unsalted Hashes*)**: Menyimpan hash deterministik yang rentan terhadap basis data pencarian instan (*Rainbow Tables*).
- **Manajemen Kunci yang Buruk**: Menyimpan kunci enkripsi (*hardcoded keys*) langsung di dalam kode sumber aplikasi atau repositori publik.
- **Ketiadaan Enkripsi Data Saat Transit atau Saat Istirahat**: Mengirimkan kredensial melalui protokol teks biasa (HTTP, FTP, Telnet) atau menyimpan basis data pengguna tanpa enkripsi storage.

## 3. Dampak Serangan Kriptografi
Ketika kriptografi gagal:
1. **Eksfiltrasi Identitas & Kredensial**: Penyerang merekonstruksi kata sandi pengguna atau administrator dari hash yang bocor, membuka akses pintu depan ke seluruh sistem.
2. **Kompromi Integritas Data**: Penyerang memodifikasi data transaksi di tengah jaringan (*tampering*) karena ketiadaan tanda tangan digital terautentikasi (seperti HMAC atau mode enkripsi AEAD).
3. **Pelanggaran Kepatuhan Regulasi**: Sanksi hukum berat di bawah GDPR, HIPAA, PCI-DSS, atau UU Perlindungan Data Pribadi (UU PDP).

## 4. Sasaran Pembelajaran di Lab A04
Melalui lab *CryptoVault* pada port 8014, Anda akan:
1. Menginspeksi endpoint audit publik yang membocorkan hash kata sandi pengguna.
2. Mempelajari struktur karakteristik hash MD5 128-bit (string hex 32 karakter).
3. Memecahkan hash MD5 tanpa salt milik akun administrator menggunakan analisis kamus deterministik.
4. Masuk ke brankas rahasia (*Master Vault*) dan mengambil activation key berformat `ZITERA{...}`.
5. Menerapkan skema password hashing adaptif modern berbasis `Argon2id` dan `bcrypt`.
