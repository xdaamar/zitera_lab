# Analogi Dunia Nyata: Pembeli Mengganti Sendiri Label Barcode Harga Barang Sebelum ke Kasir

Untuk memahami logika celah **Unsigned Client-Side State Tampering in LocalStorage** secara gamblang tanpa istilah rumit, bayangkan kejadian fisik berikut:

---

### Cerita Analogi Sehari-Hari
Seorang pembeli menempelkan stiker harga Rp 1.000 di atas kotak laptop seharga Rp 20.000.000. Kasir langsung men-scan stiker buatan pembeli itu dan memproses pembayaran tanpa mencocokkan ke basis data harga resmi toko.

---

### Tabel Perbandingan: Dunia Fisik vs Dunia Web (CartSync Session)

| Unsur Sistem | Dunia Fisik (Analogi) | Dunia Digital (CartSync Session) |
| :--- | :--- | :--- |
| **Identitas / Tiket** | Rompi seragam atau tiket klaim fisik | Header request / parameter token klien |
| **Pintu Pemeriksaan** | Satpam atau pintu gerbang fisik | Middleware autentikasi & filter API backend |
| **Celah Kelemahan** | Satpam hanya percaya tampilan luar tanpa cek KTP | Server percaya header klien `Content-Type: application/json` |
| **Aksi Penyerang** | Menyusup masuk ke area terlarang | Memperoleh hak istimewa atau data terlarang |
| **Dampak Insiden** | Pencurian aset atau sabotase fasilitas | Data bocor dan bendera `ZITERA{cl13nt_st4t3_t4mp3r_l0c4lst0r4g3_2026}` tereksfiltrasi |

---

### Inti Pelajaran (Key Takeaway)
> [!TIP]
> Tampilan luar atau klaim yang dibawa oleh klien bukanlah bukti otentik identitas. Sistem server wajib selalu memeriksa data master internal di database sebelum mengabulkan permintaan akses apapun!
