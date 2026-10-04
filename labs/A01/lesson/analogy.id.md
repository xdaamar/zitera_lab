# Analogi Kunci Kamar Hotel (The Hotel Keycard Analogy)

Untuk memahami konsep ini dengan mudah, bayangkan proses saat kamu menginap di sebuah hotel:

1. **Authentication (Proses Masuk):** Kamu menunjukkan KTP di meja resepsionis. Resepsionis memverifikasi pesanan kamarmu dan memberikan kartu kunci untuk kamar `#302`.
2. **Access Control Ideal (Kondisi Aman):** Kartu kuncimu hanya bisa membuka pintu kamar `#302`. Jika kamu mencoba menempelkan kartu tersebut ke pintu kamar `#303`, sensor kunci akan berkedip merah dan pintu tetap terkunci rapat.
3. **Broken Access Control (Celah Keamanan):** Bayangkan jika kunci hotel tersebut menggunakan sistem digital yang tidak aman, di mana saat kartu ditempelkan, aplikasi di ponselmu bertanya: *"Pintu kamar nomor berapa yang ingin kamu buka?"* Jika kamu mengganti input di ponsel menjadi `303`, pintu kamar milik tamu lain langsung terbuka!

Dalam skenario ini:
Resepsionis sudah berhasil memverifikasi identitasmu (Authentication sukses), namun sistem kunci pintu kamar percaya begitu saja pada input dari sisi pengguna tanpa memeriksa apakah kamar 303 memang disewakan kepadamu (Authorization gagal).
