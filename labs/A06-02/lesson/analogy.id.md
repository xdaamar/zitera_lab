# Analogi Dunia Nyata: Mengambil Uang Bersamaan di Dua Mesin ATM Menggunakan Rekening yang Sama

Untuk memahami logika celah **Concurrency Race Condition & Double Spending** secara gamblang tanpa istilah rumit, bayangkan kejadian fisik berikut:

---

### Cerita Analogi Sehari-Hari
Saldo rekeningmu hanya tersisa 100 ribu rupiah. Kamu menekan tombol tarik tunai 100 ribu di ATM A dan ATM B pada mikrodetik yang sama persis. Karena kedua mesin membaca saldo sebelum menguranginya, kedua ATM sama-sama mengeluarkan uang tunai 100 ribu.

---

### Tabel Perbandingan: Dunia Fisik vs Dunia Web (FlashSale Pay)

| Unsur Sistem | Dunia Fisik (Analogi) | Dunia Digital (FlashSale Pay) |
| :--- | :--- | :--- |
| **Identitas / Tiket** | Rompi seragam atau tiket klaim fisik | Header request / parameter token klien |
| **Pintu Pemeriksaan** | Satpam atau pintu gerbang fisik | Middleware autentikasi & filter API backend |
| **Celah Kelemahan** | Satpam hanya percaya tampilan luar tanpa cek KTP | Server percaya header klien `Content-Type: application/json` |
| **Aksi Penyerang** | Menyusup masuk ke area terlarang | Memperoleh hak istimewa atau data terlarang |
| **Dampak Insiden** | Pencurian aset atau sabotase fasilitas | Data bocor dan bendera `ZITERA{r4c3_c0nd1t10n_d0ubl3_sp3nd_2026}` tereksfiltrasi |

---

### Inti Pelajaran (Key Takeaway)
> [!TIP]
> Tampilan luar atau klaim yang dibawa oleh klien bukanlah bukti otentik identitas. Sistem server wajib selalu memeriksa data master internal di database sebelum mengabulkan permintaan akses apapun!
