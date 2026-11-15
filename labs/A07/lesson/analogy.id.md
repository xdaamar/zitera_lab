# Analogi Dunia Nyata: Mesin ATM vs Kunci Koper Putar Tanpa Pengawas

Untuk memahami mengapa ketiadaan pembatasan tebakan (*rate limiting*) dapat menghancurkan keamanan autentikasi, bayangkan perbandingan antara dua mekanisme pengamanan berikut:

---

### 1. Mesin ATM Bank Modern (Autentikasi Aman dengan Rate Limiting)
Ketika Anda memasukkan kartu debit ke mesin ATM untuk menarik uang tunai:
- Anda diminta memasukkan nomor PIN 4 angka (misalnya `2026`).
- **Aturan Pembatasan Ketat**:
  - Jika Anda salah memasukkan PIN 1 kali: mesin memberi peringatan.
  - Jika Anda salah memasukkan PIN 3 kali berturut-turut: mesin ATM **otomatis memblokir kartu Anda**, menahan fisik kartu di dalam mesin, membunyikan alarm peringatan, dan kamera keamanan merekam wajah orang di depan mesin.
- **Hasil**: Bahkan jika PIN ATM hanya terdiri dari 4 angka sederhana, tidak ada pencuri yang bisa membobolnya dengan tebakan acak karena mereka hanya diberi kesempatan maksimal 3 kali sebelum sistem terkunci total.

---

### 2. Kunci Koper Putar di Meja Terbuka (Authentication Failure)
Sekarang bayangkan seseorang meletakkan koper berisi dokumen rahasia perusahaan di atas meja lobi tanpa ada satpam yang mengawasi:
- Koper tersebut dikunci dengan gembok kombinasi putar 4 roda angka (`0000` hingga `9999`).
- **Ketiadaan Pembatasan**:
  - Gembok tidak memiliki alarm.
  - Roda angka dapat diputar bebas jutaan kali tanpa macet atau terkunci.
- **Eksploitasi**: Pencuri membawa alat pemutar bermotor kecil (*robotic spinner*) yang dapat memutar dan menguji 50 kombinasi angka setiap detik.
  - Dalam waktu 10 detik, pemutar telah menguji 500 angka.
  - Dalam waktu kurang dari 2 menit, seluruh 10.000 kombinasi pasti telah dicoba dan koper terbuka lebar!

---

### Pesan Utama Keamanan
Dalam aplikasi web:
- Formulir login tanpa *Rate Limiting*, penundaan (*cooldown*), dan penguncian akun (*Lockout*) sama persis dengan koper putar tanpa pengawas.
- Betapapun sulitnya kata sandi yang Anda buat, membiarkan penyerang mencoba miliaran kali tanpa batasan waktu akan selalu berakhir pada kegagalan autentikasi.
