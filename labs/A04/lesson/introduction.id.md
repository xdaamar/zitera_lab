# A04: Cryptographic Failures (Kegagalan Kriptografi)

Cryptographic Failures menempati peringkat ke-4 (`#4`) dalam OWASP Top 10:2025 (sebelumnya dikenal sebagai *Sensitive Data Exposure*). Kerentanan ini berfokus pada kelemahan implementasi algoritma enkripsi dan hashing yang berujung pada kebocoran data rahasia.

## Akar Permasalahan
Kriptografi secara teoritis adalah matematika yang solid, tetapi keamanannya sangat bergantung pada cara penerapannya di dalam kode (*implementation*). Kesalahan umum yang sering dilakukan pengembang meliputi:
- Mengirimkan data sensitif melalui protokol teks biasa tanpa enkripsi (HTTP, FTP, Telnet).
- Menggunakan algoritma kriptografi lawas yang sudah usang dan terbukti rusak (MD5, SHA-1, DES, RC4).
- Menyimpan kunci enkripsi rahasia (*hardcoded secret keys*) secara terang-terangan di dalam source code aplikasi.
- Menyimpan kata sandi pengguna hanya dengan fungsi hash cepat tanpa bumbu pengacak (*cryptographic salt*), sehingga mudah dibongkar menggunakan *rainbow table*.
- Menggunakan ulang nilai acak *Initialization Vector (IV)* atau nonce pada algoritma stream cipher.

## Dampak Nyata di Dunia Nyata
Saat kriptografi gagal ditegakkan, penyerang dapat mendekripsi data pribadi pelanggan, memalsukan token sesi login (*session tokens*), membongkar seluruh database kata sandi, dan menyamar sebagai pihak yang berwenang.
