# A07: Authentication Failures — Kegagalan Mekanisme Autentikasi Rumah Sakit

Authentication Failures menempati peringkat ke-7 (**#7**) dalam daftar risiko keamanan web **OWASP Top 10:2025** (sebelumnya dikenal sebagai *Identification and Authentication Failures*). Kerentanan ini terjadi ketika aplikasi gagal memverifikasi identitas pengguna secara andal, tidak menerapkan pembatasan laju percobaan (*rate limiting*), mengizinkan kredensial lemah yang mudah ditebak, atau gagal memberlakukan penguncian akun (*account lockout*) setelah serangkaian kegagalan masuk.

---

### 1. Alasan Kuat / Mengapa Ini Sangat Penting (Why It Matters)
Dalam sektor kesehatan (*healthcare*), pertahanan autentikasi bukan sekadar pelindung data finansial, melainkan benteng keselamatan nyawa pasien:

- **Dampak Kritis terhadap Pasien & Regulasi HIPAA**:
  - Pelanggaran terhadap catatan medis elektronik (*Electronic Health Records / EHR*) di bawah **HIPAA** dan regulasi privasi data medis global dapat menimbulkan sanksi denda perdata hingga jutaan dolar serta pencabutan akreditasi rumah sakit.
  - Jika akun dokter atau staf anestesi dibajak akibat password lemah tanpa MFA, penyerang dapat mengubah dosis obat pasien, mengakses rekam medis rahasia, atau menyandera sistem klinik (*ransomware deployment*).
- **Serangan Otomatis Skala Masif (*Automated Credential Stuffing*)**:
  Penyerang menggunakan daftar password hasil kebocoran publik (*credential dumps*) dan botnet otomatis untuk menguji jutaan kombinasi username/password per detik. Tanpa adanya *rate limiting* dan CAPTCHA, akun pengguna dengan PIN sederhana akan jebol dalam hitungan detik.

---

### 2. Gambaran Sistem yang Dihadapi (Target Commercial Architecture)
Dalam praktikum ini, Anda mengaudit portal rekam medis klinis rumah sakit: **NexHealth Portal (Hospital Electronic Medical Records & Patient Portal)** yang beroperasi pada port `8017`.

Komponen arsitektur NexHealth Portal:
- **Clinical Practitioner Sign In**: Antarmuka masuk staf medis (`/login` dan `/api/login`) yang meminta Staff Identifier (`admin` sebagai Chief Medical Officer) dan Security PIN 4-digit.
- **Cacat Autentikasi Fatal**:
  - Ketiadaan mekanisme *rate limiting* (pembatasan request per IP/per akun).
  - Ketiadaan *lockout policy* setelah 3-5 kali salah input.
  - Penggunaan PIN 4-digit yang dapat ditebak (menggunakan tahun rilis sistem saat ini: `2026`).
- **Clinical Records Dashboard (`/admin`)**: Dashboard rekam medis pasien terproteksi yang menampilkan data triage, bangsal ICU, dan catatan audit autoritatif Chief Medical Officer.

---

### 3. Sasaran, Temuan & Eksekusi (Search, Find, and Execute)
Metodologi pengujian kelemahan autentikasi terstruktur:

1. **Apa yang Dicari (What to Search)**:
   - Endpoint login web dan REST API: `/login`, `/api/login`.
   - Header respons HTTP yang menunjukkan ketiadaan kontrol rate-limit (misalnya tidak ada header `Retry-After` atau `X-RateLimit-*`).
2. **Apa Temuan yang Menandakan Celah (Vulnerability Indicators)**:
   - Mengirimkan 10 kali percobaan login yang salah secara berurutan tidak menghasilkan penolakan status `429 Too Many Requests` maupun penundaan respons (*progressive delay*).
   - Form mengizinkan PIN pendek 4-angka (`0000` - `9999`, hanya 10.000 kemungkinan kombinasi).
3. **Eksekusi Eksploitasi Terukur (Strategic Execution)**:
   - Lakukan pengujian terarah terhadap PIN 4-digit tahun rilis aktif (`2026`) untuk akun `admin`.
   - Masuk ke sesi Chief Medical Officer di `/admin` dan eksfiltrasi segel validasi kepatuhan HIPAA berformat `ZITERA{...}`.

---

### 4. Pondasi Kuat: Tiga Lapisan Pertahanan Autentikasi
Sistem autentikasi enterprise yang tangguh wajib menerapkan pertahanan berlapis (*Defense-in-Depth*):
1. **Multi-Factor Authentication (MFA)**: Menggabungkan sesuatu yang Anda ketahui (*knowledge: password*), sesuatu yang Anda miliki (*possession: token hardware/TOTP*), atau karakteristik fisik (*biometrics: sidik jari/wajah*).
2. **Exponential Backoff & Rate Limiting**: Menambah waktu tunggu secara eksponensial (1 detik, 2 detik, 4 detik, 8 detik...) setiap kali terjadi kegagalan masuk guna menggagalkan serangan otomatis.
3. **Password & PIN Complexity Guardrails**: Melarang keras PIN 4-angka sederhana untuk akun berprivelege tinggi seperti direksi atau dokter rumah sakit.
