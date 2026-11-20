# A07: Authentication Failures — Jebolnya Gerbang Autentikasi Pengguna

Identification and Authentication Failures menempati peringkat ke-7 (**#7**) dalam daftar risiko **OWASP Top 10:2025**. Kerentanan ini terjadi ketika mekanisme verifikasi identitas pengguna, pengelolaan sesi, atau perlindungan kata sandi diimplementasikan secara lemah, sehingga memungkinkan penyerang menebak, mencuri, atau memalsukan identitas pengguna lain.

---

### 1. Kenapa Ini Sangat Berbahaya? (Business & Financial Impact)
Autentikasi adalah gerbang utama pembuktian identitas (*identity assertion*). Jika gerbang ini tidak memiliki pembatasan:

- **Pengambilalihan Akun Eksekutif Massal (*Account Takeover / ATO*)**:
  Bot peretas dapat menembakkan puluhan ribu kombinasi kata sandi atau PIN per detik (*brute-force* atau *credential stuffing*) ke endpoint login. Tanpa adanya *rate-limiting*, akun direktur atau dokter rumah sakit dapat dijebol dalam hitungan menit.
  - Insiden nyata kebocoran rekam medis rumah sakit dan portal kesehatan: Pelanggaran privasi data pasien di bawah standar **HIPAA** internasional dan **UU PDP No. 27/2022** berisiko denda miliaran rupiah dan gugatan hukum perdata atas kebocoran riwayat penyakit rahasia pasien.
- **Biaya Pemulihan Kepercayaan Finansial**:
  Serangan pengambilalihan akun yang sukses merusak integritas seluruh sistem transaksi digital, memicu sanksi pencabutan sertifikasi kepatuhan ISO 27001 dan pemutusan kerja sama asuransi kesehatan pihak ketiga.

---

### 2. Bagaimana Cara Mencari & Mengujinya? (Recon & Vulnerability Indicators)
Metodologi pengujian ketahanan autentikasi:

1. **Apa yang Dicari (What to Search)**:
   - Formulir masuk sistem: `/login`, `/api/login`, `/auth/verify`.
   - Panjang dan kompleksitas kata sandi/PIN yang diizinkan (misal PIN numerik 4-digit yang ruang kombinasinya hanya $10.000$ kemungkinan: `0000` - `9999`).
2. **Apa Temuan yang Menandakan Celah (Vulnerability Indicators)**:
   - Mengirimkan 5, 10, hingga puluhan percobaan login yang salah berturut-turut tidak memicu penguncian akun (*no account lockout*), tidak meminta CAPTCHA, dan tidak memperlambat waktu respons (*zero rate-limiting / no throttling*).
3. **Eksekusi Eksploitasi Terukur (Strategic Execution)**:
   - Lakukan pengujian terarah (*targeted brute-force*) terhadap PIN umum 4-digit akun administrator (`admin`).
   - Gunakan PIN siklus rilis sistem (`2026`).
   - Masuk ke konsol klinis `/admin` untuk mengekstrak bendera autoritatif `ZITERA{4uth_f41lur3_brut3_f0rc3_2026}`.

---

### 3. Bagaimana Cara Memperbaikinya? (Secure Coding & Remediation)
Pertahanan autentikasi standar industri:
- **Terapkan Rate Limiting & Slow Down**: Batasi jumlah percobaan login maksimal 5 kali per menit per IP atau per akun (misal menggunakan token bucket algoritma Redis).
- **Terapkan Kunci Akun Progresif (*Progressive Account Lockout*)**: Kunci sementara akun selama 15 menit jika terjadi 5 kali kegagalan berurutan.
- **Wajibkan Multi-Factor Authentication (MFA)**: Tambahkan lapisan kedua (TOTP via Google Authenticator atau WebAuthn FIDO2).
- **Tolak PIN / Password Terlalu Pendek**: Larang penggunaan kata sandi umum atau PIN 4-digit sederhana untuk akun dengan hak akses tinggi.

---

### Gambaran Sistem yang Dihadapi (Target Commercial Architecture)
Dalam praktikum ini, kamu mengaudit sistem informasi rekam medis rumah sakit: **NexHealth Portal / OmniAuth Admin Gateway (Hospital EMR & Inpatient Management System)** yang beroperasi pada port `8017`.

Komponen arsitektur:
- **Clinical Staff Portal**: Melayani login dokter dan administrator medis via `/login`.
- **Celah Tanpa Pembatas Frekuensi**: Handler login backend menerima request tak terbatas tanpa middleware rate-limiting.
- **PIN Administratif Lemah**: Akun `admin` menggunakan PIN 4-digit sederhana (`2026`).
- **Konsol Supervisor Klinis**: Halaman `/admin` yang memuat bukti kepatuhan medis dan flag CTF.
