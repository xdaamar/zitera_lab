# Analogi Pintu Rumah dengan Percobaan Kunci Tanpa Batas

Bayangkan sebuah rumah dengan kunci pintu berputar (kombinasi 4 angka 0000 - 9999):

1. **Sistem Pengamanan Aman (Modern ATM):** Jika seseorang salah memasukkan PIN sebanyak 3 kali berturut-turut, mesin ATM akan memblokir kartu secara otomatis dan membunyikan alarm.
2. **Sistem Rentan (Authentication Failure):** Pintu rumah mengizinkan siapa pun mencoba kombinasi angka sebanyak jutaan kali tanpa jeda waktu, tanpa alarm, dan tanpa penguncian otomatis.
3. **Eksploitasi:** Pencuri menyewa robot kecil untuk memutar kombinasi angka dari `0000` hingga `9999` dengan kecepatan 50 tebakan per detik. Dalam waktu kurang dari 3 menit, pintu rumah pasti terbuka!

Inilah bahaya ketiadaan *Rate Limiting*: kata sandi apa pun yang pendek atau dapat ditebak pasti akan jebol jika penyerang diberi kesempatan mencoba tanpa henti.
