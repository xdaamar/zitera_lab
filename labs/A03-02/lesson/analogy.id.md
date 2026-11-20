# Analogi Dunia Nyata: Restoran Membeli Bahan Baku Bermerek Sama dari Pedagang Pinggir Jalan

Untuk memahami logika celah **Dependency Confusion & Internal Namespace Hijacking** secara gamblang tanpa istilah rumit, bayangkan kejadian fisik berikut:

---

### Cerita Analogi Sehari-Hari
Dapur restoran membutuhkan kecap merek internal 'Kecap-Pusat'. Seorang pedagang pinggir jalan mendaftarkan merek dengan nama persis sama tetapi mencantumkan nomor versi 9.9.9. Sistem pemesanan otomatis memilih versi lebih tinggi dari pedagang luar tanpa memeriksa asal registri resmi.

---

### Tabel Perbandingan: Dunia Fisik vs Dunia Web (Enterprise Artifactory)

| Unsur Sistem | Dunia Fisik (Analogi) | Dunia Digital (Enterprise Artifactory) |
| :--- | :--- | :--- |
| **Identitas / Tiket** | Rompi seragam atau tiket klaim fisik | Header request / parameter token klien |
| **Pintu Pemeriksaan** | Satpam atau pintu gerbang fisik | Middleware autentikasi & filter API backend |
| **Celah Kelemahan** | Satpam hanya percaya tampilan luar tanpa cek KTP | Server percaya header klien `Content-Type: application/json` |
| **Aksi Penyerang** | Menyusup masuk ke area terlarang | Memperoleh hak istimewa atau data terlarang |
| **Dampak Insiden** | Pencurian aset atau sabotase fasilitas | Data bocor dan bendera `ZITERA{d3p_c0nfus10n_n4m3sp4c3_h1j4ck_2026}` tereksfiltrasi |

---

### Inti Pelajaran (Key Takeaway)
> [!TIP]
> Tampilan luar atau klaim yang dibawa oleh klien bukanlah bukti otentik identitas. Sistem server wajib selalu memeriksa data master internal di database sebelum mengabulkan permintaan akses apapun!
