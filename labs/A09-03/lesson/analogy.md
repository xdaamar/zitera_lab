# Analogi Dunia Nyata: Pencuri Mengambil Satu Butir Permata Setiap Minggu Agar Neraca Toko Tidak Curiga

Untuk memahami logika celah **SIEM Threshold Evasion via Slow-and-Low Scanning** secara gamblang tanpa istilah rumit, bayangkan kejadian fisik berikut:

---

### Cerita Analogi Sehari-Hari
Aturan inventaris toko menyatakan: 'Jika ada barang hilang lebih dari 10 buah dalam sehari, bunyikan alarm'. Pencuri mengambil 1 buah setiap 3 hari sekali. Sampai akhir tahun ratusan permata habis tetapi alarm tidak pernah berbunyi.

---

### Tabel Perbandingan: Dunia Fisik vs Dunia Web (StealthDetect Gateway)

| Unsur Sistem | Dunia Fisik (Analogi) | Dunia Digital (StealthDetect Gateway) |
| :--- | :--- | :--- |
| **Identitas / Tiket** | Rompi seragam atau tiket klaim fisik | Header request / parameter token klien |
| **Pintu Pemeriksaan** | Satpam atau pintu gerbang fisik | Middleware autentikasi & filter API backend |
| **Celah Kelemahan** | Satpam hanya percaya tampilan luar tanpa cek KTP | Server percaya header klien `X-Client-Probe: stealth` |
| **Aksi Penyerang** | Menyusup masuk ke area terlarang | Memperoleh hak istimewa atau data terlarang |
| **Dampak Insiden** | Pencurian aset atau sabotase fasilitas | Data bocor dan bendera `ZITERA{s13m_thr3sh0ld_3v4s10n_st34lth_2026}` tereksfiltrasi |

---

### Inti Pelajaran (Key Takeaway)
> [!TIP]
> Tampilan luar atau klaim yang dibawa oleh klien bukanlah bukti otentik identitas. Sistem server wajib selalu memeriksa data master internal di database sebelum mengabulkan permintaan akses apapun!
