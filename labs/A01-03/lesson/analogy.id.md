# Analogi Dunia Nyata: Kunci Duplikat Lemari Loker Tetangga yang Bisa Dipakai Membuang Barang

Untuk memahami logika celah **Multi-Tenant Boundary Escape & Insecure Direct Object Deletion** secara gamblang tanpa istilah rumit, bayangkan kejadian fisik berikut:

---

### Cerita Analogi Sehari-Hari
Bayangkan di gedung perkantoran bersama, nomor loker kamu adalah 101. Ketika hendak membuang berkas lama, kamu menyebut ke petugas kebersihan: 'Tolong kosongkan loker nomor 202'. Petugas kebersihan langsung membakar seluruh berkas di loker 202 tanpa mengecek apakah kamu pemilik sah loker tersebut.

---

### Tabel Perbandingan: Dunia Fisik vs Dunia Web (TenantCloud Workspaces)

| Unsur Sistem | Dunia Fisik (Analogi) | Dunia Digital (TenantCloud Workspaces) |
| :--- | :--- | :--- |
| **Identitas / Tiket** | Rompi seragam atau tiket klaim fisik | Header request / parameter token klien |
| **Pintu Pemeriksaan** | Satpam atau pintu gerbang fisik | Middleware autentikasi & filter API backend |
| **Celah Kelemahan** | Satpam hanya percaya tampilan luar tanpa cek KTP | Server percaya header klien `Authorization: Bearer <tenant_a_token>` |
| **Aksi Penyerang** | Menyusup masuk ke area terlarang | Memperoleh hak istimewa atau data terlarang |
| **Dampak Insiden** | Pencurian aset atau sabotase fasilitas | Data bocor dan bendera `ZITERA{mult1_t3n4nt_3sc4p3_d3l3t3_b0l4_2026}` tereksfiltrasi |

---

### Inti Pelajaran (Key Takeaway)
> [!TIP]
> Tampilan luar atau klaim yang dibawa oleh klien bukanlah bukti otentik identitas. Sistem server wajib selalu memeriksa data master internal di database sebelum mengabulkan permintaan akses apapun!
