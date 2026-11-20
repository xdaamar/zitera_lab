# Analogi Dunia Nyata: Surat Kuasa yang Boleh Tanpa Tanda Tangan Asalkan Formatnya Rapi

Untuk memahami logika celah **JWT Algorithm None & HMAC Weak Key Cracking** secara gamblang tanpa istilah rumit, bayangkan kejadian fisik berikut:

---

### Cerita Analogi Sehari-Hari
Di sebuah kantor kelurahan, ada aturan bahwa surat pengantar resmi harus bertanda tangan. Tetapi jika pemohon menulis jenis tinta 'TIDAK_ADA' di bagian atas surat, petugas menerima surat itu tanpa memeriksa stempel apapun.

---

### Tabel Perbandingan: Dunia Fisik vs Dunia Web (TokenShield SSO)

| Unsur Sistem | Dunia Fisik (Analogi) | Dunia Digital (TokenShield SSO) |
| :--- | :--- | :--- |
| **Identitas / Tiket** | Rompi seragam atau tiket klaim fisik | Header request / parameter token klien |
| **Pintu Pemeriksaan** | Satpam atau pintu gerbang fisik | Middleware autentikasi & filter API backend |
| **Celah Kelemahan** | Satpam hanya percaya tampilan luar tanpa cek KTP | Server percaya header klien `Authorization: Bearer <tampered_jwt>` |
| **Aksi Penyerang** | Menyusup masuk ke area terlarang | Memperoleh hak istimewa atau data terlarang |
| **Dampak Insiden** | Pencurian aset atau sabotase fasilitas | Data bocor dan bendera `ZITERA{jwt_n0n3_4lg0r1thm_f0rg3ry_2026}` tereksfiltrasi |

---

### Inti Pelajaran (Key Takeaway)
> [!TIP]
> Tampilan luar atau klaim yang dibawa oleh klien bukanlah bukti otentik identitas. Sistem server wajib selalu memeriksa data master internal di database sebelum mengabulkan permintaan akses apapun!
