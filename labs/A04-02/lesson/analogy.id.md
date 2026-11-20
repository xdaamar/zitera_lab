# Analogi Dunia Nyata: Karcis Antrean Bank yang Nomornya Menggunakan Jam dan Menit Tepat

Untuk memahami logika celah **Predictable Password Reset Token via Insecure PRNG** secara gamblang tanpa istilah rumit, bayangkan kejadian fisik berikut:

---

### Cerita Analogi Sehari-Hari
Jika nomor antrean bank selalu dibuat berdasarkan detik kedatangan (misal: 142055 untuk pukul 14:20:55), orang lain di luar pintu bisa menebak dengan mudah nomor antreanmu tanpa perlu melihat kertas karcismu.

---

### Tabel Perbandingan: Dunia Fisik vs Dunia Web (AuthKey Recovery)

| Unsur Sistem | Dunia Fisik (Analogi) | Dunia Digital (AuthKey Recovery) |
| :--- | :--- | :--- |
| **Identitas / Tiket** | Rompi seragam atau tiket klaim fisik | Header request / parameter token klien |
| **Pintu Pemeriksaan** | Satpam atau pintu gerbang fisik | Middleware autentikasi & filter API backend |
| **Celah Kelemahan** | Satpam hanya percaya tampilan luar tanpa cek KTP | Server percaya header klien `Content-Type: application/json` |
| **Aksi Penyerang** | Menyusup masuk ke area terlarang | Memperoleh hak istimewa atau data terlarang |
| **Dampak Insiden** | Pencurian aset atau sabotase fasilitas | Data bocor dan bendera `ZITERA{prng_pr3d1ct4bl3_r3s3t_t0k3n_2026}` tereksfiltrasi |

---

### Inti Pelajaran (Key Takeaway)
> [!TIP]
> Tampilan luar atau klaim yang dibawa oleh klien bukanlah bukti otentik identitas. Sistem server wajib selalu memeriksa data master internal di database sebelum mengabulkan permintaan akses apapun!
