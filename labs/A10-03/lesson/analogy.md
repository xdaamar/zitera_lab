# Analogi Dunia Nyata: Memperdaya Kurir Internal Kantor untuk Mengambil Berkas Rahasia di Ruang Direktur

Untuk memahami logika celah **Full-Chain Blind SSRF to Cloud Metadata** secara gamblang tanpa istilah rumit, bayangkan kejadian fisik berikut:

---

### Cerita Analogi Sehari-Hari
Tamu luar dilarang masuk ke lantai 10 ruang direksi. Tetapi tamu tersebut menulis surat perintah di amplop resmi lalu menitipkannya ke kurir internal yang memiliki kartu akses lift lantai 10 untuk membawakan berkas rahasia keluar gedung.

---

### Tabel Perbandingan: Dunia Fisik vs Dunia Web (WebFetch Microservice)

| Unsur Sistem | Dunia Fisik (Analogi) | Dunia Digital (WebFetch Microservice) |
| :--- | :--- | :--- |
| **Identitas / Tiket** | Rompi seragam atau tiket klaim fisik | Header request / parameter token klien |
| **Pintu Pemeriksaan** | Satpam atau pintu gerbang fisik | Middleware autentikasi & filter API backend |
| **Celah Kelemahan** | Satpam hanya percaya tampilan luar tanpa cek KTP | Server percaya header klien `Content-Type: application/json` |
| **Aksi Penyerang** | Menyusup masuk ke area terlarang | Memperoleh hak istimewa atau data terlarang |
| **Dampak Insiden** | Pencurian aset atau sabotase fasilitas | Data bocor dan bendera `ZITERA{ssrf_cl0ud_m3t4d4t4_3xf1ltr4t10n_2026}` tereksfiltrasi |

---

### Inti Pelajaran (Key Takeaway)
> [!TIP]
> Tampilan luar atau klaim yang dibawa oleh klien bukanlah bukti otentik identitas. Sistem server wajib selalu memeriksa data master internal di database sebelum mengabulkan permintaan akses apapun!
