# Analogi Dunia Nyata: Montir Servis yang Memasang GPS Pelacak Rahasia di Bawah Jok Mobil

Untuk memahami logika celah **Malicious Preinstall Script & CI/CD Secret Exfiltration** secara gamblang tanpa istilah rumit, bayangkan kejadian fisik berikut:

---

### Cerita Analogi Sehari-Hari
Saat mobil dibawa ganti oli rutin, montir nakal memanfaatkan instruksi otomatis pembongkaran mesin untuk menanamkan alat penyadap rahasia yang mencatat rute perjalanan pemilik mobil.

---

### Tabel Perbandingan: Dunia Fisik vs Dunia Web (NodeBuild Engine)

| Unsur Sistem | Dunia Fisik (Analogi) | Dunia Digital (NodeBuild Engine) |
| :--- | :--- | :--- |
| **Identitas / Tiket** | Rompi seragam atau tiket klaim fisik | Header request / parameter token klien |
| **Pintu Pemeriksaan** | Satpam atau pintu gerbang fisik | Middleware autentikasi & filter API backend |
| **Celah Kelemahan** | Satpam hanya percaya tampilan luar tanpa cek KTP | Server percaya header klien `X-Pipeline-Agent: GitHub-Runner-01` |
| **Aksi Penyerang** | Menyusup masuk ke area terlarang | Memperoleh hak istimewa atau data terlarang |
| **Dampak Insiden** | Pencurian aset atau sabotase fasilitas | Data bocor dan bendera `ZITERA{pr31nst4ll_h00k_c1cd_s3cr3t_l34k_2026}` tereksfiltrasi |

---

### Inti Pelajaran (Key Takeaway)
> [!TIP]
> Tampilan luar atau klaim yang dibawa oleh klien bukanlah bukti otentik identitas. Sistem server wajib selalu memeriksa data master internal di database sebelum mengabulkan permintaan akses apapun!
