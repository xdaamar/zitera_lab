# Analogi Dunia Nyata: Kamera CCTV Bank Dimatikan Demi Hemat Listrik

Untuk memahami apa itu **Security Logging and Monitoring Failures**, bayangkan sebuah kantor cabang bank penyimpanan emas:

---

### Cerita: Gedung Bank Penyimpanan Emas Mewah
Bayangkan sebuah bank memiliki brankas baja tebal dan pintu berlapis untuk melindungi emas batangan:

1. **Keputusan Konyol Manajemen Gedung**:
   Suatu hari, kepala operasional gedung melihat tagihan listrik bulanan yang membengkak. Demi menghemat anggaran listrik, ia mengambil keputusan:
   - **Mematikan seluruh kamera CCTV** di lorong brankas.
   - **Mencabut kabel sirine alarm kebakaran dan alarm pencuri**.
   - **Membuang buku tamu pos satpam** agar satpam tidak repot mencatat siapa saja orang yang masuk keluar gedung.
2. **Maling Masuk Menginap di Brankas**:
   Malam harinya, seorang pencuri masuk melalui pintu ventilasi. Pencuri mencoba membongkar gembok brankas berkali-kali menggunakan linggis.
   Karena alarm dimatikan, tidak ada satupun sirine yang berbunyi.
   Karena kamera CCTV dimatikan, tidak ada rekaman video wajah pencuri.
   Pencuri itu bahkan leluasa membawa kasur lipat dan menginap di samping brankas selama berminggu-minggu sampai berhasil membobol pintu baja tersebut!
3. **Penyelidikan yang Buta Total**:
   Tiga bulan kemudian, pihak bank baru tersadar bahwa seluruh emas di dalam brankas telah raib. Saat polisi datang untuk menyelidiki:
   - Tidak ada rekaman CCTV.
   - Tidak ada catatan buku tamu siapa yang datang.
   - Tidak ada log alarm yang mencatat jam berapa pintu brankas dibuka paksa.
   Polisi tidak bisa melacak siapa pelakunya karena **gedung tersebut mengalami kebutaan total**!

---

### Pelajaran Keamanan
Pencurian terjadi bukan hanya karena gemboknya berhasil dibuka, melainkan karena **tidak ada sistem pengawasan yang mengawasi dan membunyikan alarm saat kejahatan sedang berlangsung**.

Itulah yang terjadi pada sistem komputer yang rentan terhadap **Logging and Monitoring Failures**:
Aplikasi tidak mencatat ketika ada orang yang salah mengetik password berkali-kali, dan tidak membunyikan notifikasi ketika ada pengguna biasa yang mengangkat dirinya sendiri menjadi Admin Eksekutif!

---

### Tabel Perbandingan: Dunia Fisik vs Dunia Web (FleetTrack)

| Komponen | Dunia Fisik (Gedung Bank) | Dunia Web (FleetTrack Logistics) |
| :--- | :--- | :--- |
| **Kamera Pengawas** | Kamera CCTV di setiap sudut lorong | Log aktivitas terpusat (*Centralized Audit Logging*) |
| **Sirine Alarm** | Sirine darurat yang berbunyi saat pintu dijebol | Sistem peringatan waktu nyata (*Real-time SIEM Alerting*) |
| **Aksi Mencurigakan** | Orang asing mencoba 5 kunci berbeda | 5 kali percobaan login gagal berturut-turut |
| **Eskalasi Ilegal** | Orang biasa masuk ruang direktur & ganti plang nama | Panggilan ke `/api/admin/role_escalate` |
| **Celah Telemetri** | CCTV dan sirine alarm dicabut kabelnya | Backend tidak mencatat log kegagalan atau eskalasi |
| **Dampak** | Maling bebas menginap di brankas | Penyerang menjadi super admin tanpa jejak di SOC |

---

### Inti Pelajaran (Key Takeaway)
> [!TIP]
> Pertahanan siber tidak ada artinya jika kamu buta terhadap apa yang sedang terjadi di dalam sistemmu. Pencatatan log yang lengkap dan sistem alarm waktu nyata (*real-time alerting*) adalah mata dan telinga para pembela keamanan (*Defenders / SOC*) untuk menggagalkan serangan sebelum kerugian terjadi!
