# Pembahasan Teknis: Hashing, Salting, & Enkripsi Modern

## 1. Perbedaan Mendasar Enkripsi vs Hashing
- Enkripsi (Two-Way): Mengubah data asli (*plaintext*) menjadi kode acak (*ciphertext*) menggunakan sebuah kunci. Tujuannya adalah data tersebut dapat dikembalikan ke bentuk semula (*decryption*) oleh penerima yang memiliki kunci sah (misalnya AES-256-GCM).
- Hashing (One-Way): Mengubah data dengan panjang berapa pun menjadi string berukuran tetap yang bersifat satu arah (*irreversible*). Sangat ideal untuk memverifikasi password tanpa perlu menyimpan teks password aslinya di database.

## 2. Mengapa MD5 dan SHA-1 Tidak Boleh Digunakan Lagi?
Algoritma seperti MD5 dan SHA-1 dirancang sangat cepat. Komputer modern dengan GPU dapat menghitung miliaran tebakan hash per detik. Selain itu, basis data kamus publik (*Rainbow Tables*) telah memetakan ratusan juta kata sandi populer langsung ke nilai hash MD5-nya secara instan.

## 3. Salting dan Password Hashing Modern
Untuk mengamankan kata sandi secara mutlak:
- Salt (Garam Pengacak): String acak unik yang digabungkan ke kata sandi sebelum proses hashing. Hal ini membuat rainbow table menjadi tidak berguna sama sekali.
- Adaptive Algorithms (Memory-Hard): Gunakan algoritma yang sengaja dibuat membutuhkan memori dan komputasi intensif seperti `Argon2id`, `bcrypt`, atau `PBKDF2`.
