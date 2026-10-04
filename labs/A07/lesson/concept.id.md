# Pembahasan Teknis: Brute-Force & Session Fixation

## 1. Serangan Tebak Kata Sandi (Brute-Force & Credential Stuffing)
- **Brute-Force:** Mencoba seluruh kemungkinan kombinasi karakter atau daftar kata sandi populer (*wordlist* seperti RockYou) pada satu akun target (misal `admin`).
- **Credential Stuffing:** Menggunakan miliaran pasangan email dan password yang bocor dari insiden peretasan situs lain, lalu mengujinya secara otomatis ke aplikasi target dengan asumsi pengguna sering menggunakan password yang sama.

## 2. Ketiadaan Rate Limiting
Bila server merespons request login gagal dalam hitungan milidetik tanpa menambahkan jeda (*delay*), penyerang dapat mengirimkan ratusan request per detik secara paralel menggunakan script otomatis.
