# Analogi Dunia Nyata: Kartu Member Toko yang Dihubungkan ke Rekening Orang Lain Tanpa Tiket Konfirmasi

Untuk memahami logika celah **OAuth 2.0 Account Linking Hijacking via Missing State** secara gamblang tanpa istilah rumit, bayangkan kejadian fisik berikut:

---

### Cerita Analogi Sehari-Hari
Saat kamu hendak menyambungkan akun toko online ke akun bank, kasir tidak memberikan kode unik konfirmasi. Penipu yang berdiri di antrean menyisipkan nomor rekeningnya ke layar saat kamu berpaling sejenak.

---

### Tabel Perbandingan: Dunia Fisik vs Dunia Web (SocialConnect Auth)

| Unsur Sistem | Dunia Fisik (Analogi) | Dunia Digital (SocialConnect Auth) |
| :--- | :--- | :--- |
| **Identitas / Tiket** | Rompi seragam atau tiket klaim fisik | Header request / parameter token klien |
| **Pintu Pemeriksaan** | Satpam atau pintu gerbang fisik | Middleware autentikasi & filter API backend |
| **Celah Kelemahan** | Satpam hanya percaya tampilan luar tanpa cek KTP | Server percaya header klien `Accept: text/html` |
| **Aksi Penyerang** | Menyusup masuk ke area terlarang | Memperoleh hak istimewa atau data terlarang |
| **Dampak Insiden** | Pencurian aset atau sabotase fasilitas | Data bocor dan bendera `ZITERA{04uth_m1ss1ng_st4t3_4cc0unt_l1nk_2026}` tereksfiltrasi |

---

### Inti Pelajaran (Key Takeaway)
> [!TIP]
> Tampilan luar atau klaim yang dibawa oleh klien bukanlah bukti otentik identitas. Sistem server wajib selalu memeriksa data master internal di database sebelum mengabulkan permintaan akses apapun!
