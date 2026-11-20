# Analogi Dunia Nyata: Menebak Isi Kotak Rahasia dengan Mengetuk dan Mengamati Respon Waktu

Untuk memahami logika celah **Blind Boolean & Time-Based SQL Injection** secara gamblang tanpa istilah rumit, bayangkan kejadian fisik berikut:

---

### Cerita Analogi Sehari-Hari
Kamu tidak bisa melihat isi brankas terkunci. Namun, kamu bisa memasukkan perintah: 'Jika huruf pertama sandi adalah A, ketuk dinding dalam 5 detik. Jika bukan, diam'. Dari jeda waktu respon, kamu bisa merekonstruksi seluruh isi sandi karakter demi karakter.

---

### Tabel Perbandingan: Dunia Fisik vs Dunia Web (CustPortal Database)

| Unsur Sistem | Dunia Fisik (Analogi) | Dunia Digital (CustPortal Database) |
| :--- | :--- | :--- |
| **Identitas / Tiket** | Rompi seragam atau tiket klaim fisik | Header request / parameter token klien |
| **Pintu Pemeriksaan** | Satpam atau pintu gerbang fisik | Middleware autentikasi & filter API backend |
| **Celah Kelemahan** | Satpam hanya percaya tampilan luar tanpa cek KTP | Server percaya header klien `Accept: application/json` |
| **Aksi Penyerang** | Menyusup masuk ke area terlarang | Memperoleh hak istimewa atau data terlarang |
| **Dampak Insiden** | Pencurian aset atau sabotase fasilitas | Data bocor dan bendera `ZITERA{bl1nd_b00l34n_t1m3_sq1_1nj3ct_2026}` tereksfiltrasi |

---

### Inti Pelajaran (Key Takeaway)
> [!TIP]
> Tampilan luar atau klaim yang dibawa oleh klien bukanlah bukti otentik identitas. Sistem server wajib selalu memeriksa data master internal di database sebelum mengabulkan permintaan akses apapun!
