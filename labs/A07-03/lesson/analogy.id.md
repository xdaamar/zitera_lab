# Analogi Dunia Nyata: Melewati Gerbang Kedua Bandara Lewat Tangga Darurat Tanpa Menunjukkan Paspor

Untuk memahami logika celah **MFA Bypass via Direct Endpoint Traversal** secara gamblang tanpa istilah rumit, bayangkan kejadian fisik berikut:

---

### Cerita Analogi Sehari-Hari
Setelah lolos pemeriksaan tiket di pintu pertama, penumpang diminta mengantre pemeriksaan paspor di pintu kedua. Namun pintu kedua memiliki lorong samping terbuka yang langsung menuju ke ruang tunggu tanpa ada penjaga.

---

### Tabel Perbandingan: Dunia Fisik vs Dunia Web (TwoFactor SecureNet)

| Unsur Sistem | Dunia Fisik (Analogi) | Dunia Digital (TwoFactor SecureNet) |
| :--- | :--- | :--- |
| **Identitas / Tiket** | Rompi seragam atau tiket klaim fisik | Header request / parameter token klien |
| **Pintu Pemeriksaan** | Satpam atau pintu gerbang fisik | Middleware autentikasi & filter API backend |
| **Celah Kelemahan** | Satpam hanya percaya tampilan luar tanpa cek KTP | Server percaya header klien `Cookie: session_user=operator` |
| **Aksi Penyerang** | Menyusup masuk ke area terlarang | Memperoleh hak istimewa atau data terlarang |
| **Dampak Insiden** | Pencurian aset atau sabotase fasilitas | Data bocor dan bendera `ZITERA{mf4_byp4ss_d1r3ct_r0ut3_2026}` tereksfiltrasi |

---

### Inti Pelajaran (Key Takeaway)
> [!TIP]
> Tampilan luar atau klaim yang dibawa oleh klien bukanlah bukti otentik identitas. Sistem server wajib selalu memeriksa data master internal di database sebelum mengabulkan permintaan akses apapun!
