# A07: Identification and Authentication Failures (Kegagalan Autentikasi)

## 1. Apa itu Authentication Failures?
Dalam standar OWASP Top 10:2025, **Authentication Failures** (sebelumnya dikenal sebagai *Identification and Authentication Failures*) mencakup seluruh spektrum kerentanan yang berkaitan dengan kegagalan sistem dalam memverifikasi identitas pengguna, mengelola sesi autentikasi (*session management*), dan melindungi batas akses akun dari serangan otomatisasi.

Autentikasi adalah pintu gerbang utama aplikasi: kegagalan pada lapisan ini memungkinkan penyerang menyamar sebagai pengguna yang sah, mengambil alih akun administratif (*account takeover*), dan mengendalikan seluruh sistem tanpa perlu meretas basis data secara langsung.

## 2. Mengapa Kegagalan Autentikasi Sering Terjadi di 2025/2026?
Serangan terhadap sistem autentikasi meningkat secara masif karena tersedianya komputasi awan berbiaya rendah dan kebocoran miliaran kredensial di dark web:
- **Ketiadaan Pembatasan Frekuensi (*Missing Rate Limiting & Lockout*)**: Sistem login mengizinkan ribuan percobaan tebakan kata sandi per detik tanpa jeda waktu (*cooldown delay*), blokir IP, atau CAPTCHA.
- **Kebijakan Kredensial Lemah (*Weak Credential Policies*)**: Mengizinkan PIN pendek (seperti 4 digit numerik) atau kata sandi default pabrik yang mudah ditebak menggunakan kamus kata (*wordlists*).
- **Credential Stuffing Otomatis**: Penyerang menggunakan botnet untuk menguji jutaan kombinasi email dan password yang bocor dari situs pihak ketiga lain, mengeksploitasi kebiasaan buruk pengguna yang menggunakan kembali kata sandi (*password reuse*).
- **Pengelolaan Sesi yang Rapuh (*Insecure Session Lifecycle*)**: Session ID tidak diganti setelah login berhasil (*Session Fixation*), tidak memiliki masa kedaluwarsa (*idle timeout*), atau dikirimkan melalui saluran tidak terenkripsi.

## 3. Insiden Bersejarah Dunia Industri
- **Serangan Brute-Force Massal Reddit (2018)**: Penyerang membobol akun karyawan Reddit melalui serangan credential stuffing pada sistem autentikasi berbasis SMS-2FA yang lemah.
- **Kasus Zoom Credential Stuffing (2020)**: Lebih dari 500.000 kredensial akun Zoom dijual di forum peretas karena endpoint login publik tidak memiliki pembatasan laju tebakan yang memadai.
- **Kebocoran Kredensial IoT Mirai Botnet**: Ratusan ribu perangkat router dan kamera pintar dibajak dalam hitungan jam karena menggunakan kombinasi username dan password bawaan pabrik (`admin:admin`, `root:123456`).

## 4. Sasaran Pembelajaran di Lab A07
Melalui laboratorium portal *OmniAuth Admin Gateway* pada port 8017, Anda akan:
1. Mengidentifikasi tidak adanya mekanisme perlindungan rate-limiting pada endpoint formulir web dan API JSON.
2. Mempelajari efisiensi serangan brute-force terhadap ruang pencarian kata sandi terbatas (PIN 4-digit).
3. Melakukan serangan tebakan terarah menggunakan skrip cURL/terminal untuk menemukan PIN otentik akun administrator.
4. Masuk ke portal manajemen dan mengambil flag otentikasi berformat `ZITERA{...}`.
5. Merancang arsitektur pertahanan autentikasi modern berbasis *Exponential Backoff*, *Account Lockout*, dan *Multi-Factor Authentication (MFA)*.
