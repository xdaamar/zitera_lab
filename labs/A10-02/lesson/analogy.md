# Analogi Dunia Nyata: Palang Pintu Tol Otomatis Terbuka Sendiri Saat Sinyal Satelit Internet Terputus

Untuk memahami logika celah **Fail-Open Exception Handling in Auth Microservice** secara gamblang tanpa istilah rumit, bayangkan kejadian fisik berikut:

---

### Cerita Analogi Sehari-Hari
Saat jaringan kabel internet gardu tol terputus mendadak, pengelola tol memasang program otomatis: 'Buka palang gratis agar tidak terjadi macet panjang'. Pengendara nakal sengaja memutus kabel jaringan agar bisa lewat gratis.

---

### Tabel Perbandingan: Dunia Fisik vs Dunia Web (GatewayGuard Failover)

| Unsur Sistem | Dunia Fisik (Analogi) | Dunia Digital (GatewayGuard Failover) |
| :--- | :--- | :--- |
| **Identitas / Tiket** | Rompi seragam atau tiket klaim fisik | Header request / parameter token klien |
| **Pintu Pemeriksaan** | Satpam atau pintu gerbang fisik | Middleware autentikasi & filter API backend |
| **Celah Kelemahan** | Satpam hanya percaya tampilan luar tanpa cek KTP | Server percaya header klien `X-Simulate-Error: TIMEOUT` |
| **Aksi Penyerang** | Menyusup masuk ke area terlarang | Memperoleh hak istimewa atau data terlarang |
| **Dampak Insiden** | Pencurian aset atau sabotase fasilitas | Data bocor dan bendera `ZITERA{f41l_0p3n_m1cr0s3rv1c3_byp4ss_2026}` tereksfiltrasi |

---

### Inti Pelajaran (Key Takeaway)
> [!TIP]
> Tampilan luar atau klaim yang dibawa oleh klien bukanlah bukti otentik identitas. Sistem server wajib selalu memeriksa data master internal di database sebelum mengabulkan permintaan akses apapun!
