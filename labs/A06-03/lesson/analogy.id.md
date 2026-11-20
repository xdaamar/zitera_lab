# Analogi Dunia Nyata: Melompat dari Pintu Masuk Langsung ke Kabin Pesawat Tanpa Melewati Pemeriksaan Bagasi

Untuk memahami logika celah **Multi-Step Workflow Step Skipping** secara gamblang tanpa istilah rumit, bayangkan kejadian fisik berikut:

---

### Cerita Analogi Sehari-Hari
Alur pendaftaran memberitahukan: Langkah 1 Isi Form, Langkah 2 Upload KTP, Langkah 3 Verifikasi Wajah, Langkah 4 Ambil Uang Pinjaman. Pengguna langsung membuka link Langkah 4 di browser, dan sistem langsung mencairkan pinjaman tanpa memeriksa apakah KTP sudah diunggah.

---

### Tabel Perbandingan: Dunia Fisik vs Dunia Web (KYC Onboarding Portal)

| Unsur Sistem | Dunia Fisik (Analogi) | Dunia Digital (KYC Onboarding Portal) |
| :--- | :--- | :--- |
| **Identitas / Tiket** | Rompi seragam atau tiket klaim fisik | Header request / parameter token klien |
| **Pintu Pemeriksaan** | Satpam atau pintu gerbang fisik | Middleware autentikasi & filter API backend |
| **Celah Kelemahan** | Satpam hanya percaya tampilan luar tanpa cek KTP | Server percaya header klien `Content-Type: application/json` |
| **Aksi Penyerang** | Menyusup masuk ke area terlarang | Memperoleh hak istimewa atau data terlarang |
| **Dampak Insiden** | Pencurian aset atau sabotase fasilitas | Data bocor dan bendera `ZITERA{w0rkfl0w_st3p_sk1pp1ng_kyc_2026}` tereksfiltrasi |

---

### Inti Pelajaran (Key Takeaway)
> [!TIP]
> Tampilan luar atau klaim yang dibawa oleh klien bukanlah bukti otentik identitas. Sistem server wajib selalu memeriksa data master internal di database sebelum mengabulkan permintaan akses apapun!
