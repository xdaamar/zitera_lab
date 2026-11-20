# A02: Security Misconfiguration — Kesalahan Konfigurasi Berujung Malapetaka

Security Misconfiguration menempati peringkat ke-2 (**#2**) dalam daftar risiko keamanan web **OWASP Top 10:2025**. Kerentanan ini merupakan salah satu pintu masuk penyerang (*initial access*) paling umum di seluruh ekosistem cloud dan enterprise, karena sering kali tidak membutuhkan eksploitasi kode yang rumit, melainkan cukup memanfaatkan kelalaian admin atau developer yang lupa mengubah pengaturan bawaan pabrik (*default settings*).

---

### 1. Kenapa Ini Sangat Berbahaya? (Business & Financial Impact)
Banyak korporasi menghabiskan miliaran rupiah membeli firewall tercanggih, namun jebol seketika hanya karena satu opsi konfigurasi dibiarkan terbuka:

- **Insiden Nyata Berdampak Finansial Raksasa**:
  - **Kebocoran Cloud Storage Bucket (Accenture, Booz Allen, GoDaddy)**: Ratusan juta rekaman rahasia, kunci privat SSH, dan basis data pelanggan bocor ke internet publik hanya karena perizinan bucket dibiarkan `Public Read` secara default.
  - **Denda Regulasi Masif**: Otoritas privasi global (GDPR) dan nasional (**UU PDP No. 27/2022**) menjatuhkan sanksi administratif dan denda hingga puluhan miliar rupiah atas kelalaian membiarkan password default (`admin:admin`) dan endpoint diagnostik terbuka di lingkungan produksi.
- **Waktu Penetrasi Sangat Cepat**:
  Peretas tidak perlu meretas algoritma kriptografi jika folder `/backups/` dapat diakses bebas atau akun admin masih menggunakan kata sandi bawaan. Bot scanner internet (seperti Shodan dan Censys) menemukan server salah konfigurasi dalam hitungan detik.

---

### 2. Bagaimana Cara Mencari & Mengujinya? (Recon & Vulnerability Indicators)
Metodologi investigasi terstruktur pada target:

1. **Apa yang Dicari (The Target)**:
   - Header HTTP respons yang membocorkan teknologi server (*fingerprinting*).
   - Rute diagnostik dan debug standar: `/debug/vars`, `/server-status`, `/actuator`, `/env`.
   - File cadangan (*backup artifacts*) dengan ekstensi berbahaya: `.bak`, `.old`, `.json.bak`, `.sql.gz`.
2. **Apa Temuan yang Menandakan Celah (Vulnerability Indicators)**:
   - Server mengembalikan status `200 OK` dengan daftar file pada direktori `/backups/` (*Directory Indexing / Autoindex Enabled*).
   - Endpoint `/debug/vars` membocorkan variabel sistem: `debug_mode: true`, `backup_storage_path: "/app/backups/"`.
3. **Eksekusi Eksploitasi Terukur (The Strategic Execution)**:
   - Unduh file konfigurasi cadangan `backup_config.json.bak` dari folder terbuka.
   - Dapatkan kunci sistem autoritatif `admin_system_key` berformat `ZITERA{53cur1ty_m15c0nf1g_b4ckup_134k}`.
   - Masuk ke konsol supervisor `/admin` menggunakan kredensial default pabrik `admin` / `admin`.

---

### 3. Bagaimana Cara Memperbaikinya? (Secure Coding & Remediation)
Langkah pengerasan (*system hardening*) standar industri:
- **Matikan Directory Browsing**: Nonaktifkan opsi `autoindex` pada server web (Nginx/Apache).
- **Nonaktifkan Endpoint Debug di Produksi**: Pastikan `DEBUG = False` dan blokir seluruh rute diagnostik `/debug/*` dari akses luar.
- **Wajibkan Ganti Kredensial Default**: Sistem harus memaksa pergantian password pada saat booting/inisialisasi pertama kali.
- **Pindahkan Direktori Backup ke Luar Web Root**: Jangan pernah menyimpan file cadangan di dalam folder yang dapat diakses melalui HTTP.

---

### Gambaran Sistem yang Dihadapi (Target Commercial Architecture)
Dalam laboratorium ini, kamu menghadapi platform orkestrasi infrastruktur cloud enterprise: **CloudCore OpsGateway (Enterprise Cloud Management & Microservices Monitor)** yang beroperasi pada port `8012`.

Komponen arsitektur yang kamu hadapi:
- **Operations Gateway**: Mengatur lalu lintas manajemen antar-microservices internal.
- **Telemetry & Diagnostic Endpoints**: Endpoint `/debug/vars` yang tidak sengaja dibiarkan aktif di lingkungan produksi, mengekspos variabel runtime dan path direktori.
- **Snapshot Storage Volume**: Direktori statis `/backups/` yang membuka direktori file secara publik (*autoindex enabled*).
- **Administrative Control Plane**: Portal login `/login` dan konsol supervisor `/admin` yang masih menggunakan kredensial bawaan pabrik `admin:admin`.
