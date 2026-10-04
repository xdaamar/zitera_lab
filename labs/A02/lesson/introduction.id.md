# A02: Security Misconfiguration (Kesalahan Konfigurasi Keamanan)

Security Misconfiguration menempati peringkat ke-2 (**#2**) dalam daftar risiko keamanan web OWASP Top 10:2025. Kerentanan ini merupakan salah satu jenis celah keamanan paling umum dan paling sering ditemukan di berbagai infrastruktur teknologi modern.

## Apa yang Dimaksud dengan Security Misconfiguration?
Security Misconfiguration terjadi ketika kontrol keamanan tidak dikonfigurasi secara tepat, dibiarkan menggunakan nilai bawaan (*default*) yang tidak aman, tidak lengkap, atau tidak dirawat dengan baik pada lapisan sistem aplikasi.

Contoh umum dari celah ini meliputi:
- Menggunakan kredensial bawaan yang tidak diubah (seperti username `admin` dengan password `admin` atau `password`).
- Pengaturan izin akses cloud atau penyimpanan data (*cloud storage buckets*) yang terbuka untuk publik.
- Mengaktifkan fitur, port, atau service yang tidak diperlukan di lingkungan produksi (misalnya endpoint debug atau halaman contoh instalasi).
- Pesan error server yang terlalu detail sehingga membocorkan *stack trace*, variabel lingkungan (*environment variables*), atau path direktori internal.
- Tidak memasang atau keliru mengatur header keamanan web penting (seperti CORS, Content-Security-Policy, HSTS).
- Mengaktifkan fitur *directory indexing* sehingga siapa pun bisa melihat dan mengunduh seluruh isi folder server.

## Mengapa Penyerang Sangat Menyukai Celah Ini?
Penyerang sering menggunakan pemindai otomatis (*automated scanners*) untuk mencari server yang masih menggunakan halaman default atau port administrasi terbuka. Karena celah ini tidak membutuhkan eksploitasi logika pemrograman yang rumit, Security Misconfiguration sering menjadi pintu masuk termudah bagi peretas.
