# Analogi Dunia Nyata: Rompi Petugas Bandara Palsu untuk Menembus Pintu Apron Pesawat

Untuk memahami logika celah **Vertical Privilege Escalation & Header Role Manipulation** secara gamblang tanpa istilah rumit, bayangkan kejadian fisik berikut:

---

### Cerita Analogi Sehari-Hari
Bayangkan seseorang membeli rompi oranye bertuliskan 'STAF OPERASIONAL' di pasar loak. Saat berjalan ke pintu gerbang darurat apron pesawat, satpam bandara yang malas memeriksa kartu identitas dan langsung membukakan pintu hanya karena melihat tulisan di rompi tersebut.

---

### Tabel Perbandingan: Dunia Fisik vs Dunia Web (OpsAdmin Portal)

| Unsur Sistem | Dunia Fisik (Analogi) | Dunia Digital (OpsAdmin Portal) |
| :--- | :--- | :--- |
| **Identitas / Tiket** | Rompi seragam atau tiket klaim fisik | Header request / parameter token klien |
| **Pintu Pemeriksaan** | Satpam atau pintu gerbang fisik | Middleware autentikasi & filter API backend |
| **Celah Kelemahan** | Satpam hanya percaya tampilan luar tanpa cek KTP | Server percaya header klien `X-User-Role: admin` |
| **Aksi Penyerang** | Menyusup masuk ke area terlarang | Memperoleh hak istimewa atau data terlarang |
| **Dampak Insiden** | Pencurian aset atau sabotase fasilitas | Data bocor dan bendera `ZITERA{v3rt1c4l_pr1v_3sc4l4t10n_h34d3r_2026}` tereksfiltrasi |

---

### Inti Pelajaran (Key Takeaway)
> [!TIP]
> Tampilan luar atau klaim yang dibawa oleh klien bukanlah bukti otentik identitas. Sistem server wajib selalu memeriksa data master internal di database sebelum mengabulkan permintaan akses apapun!
