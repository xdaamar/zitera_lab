# A02: Security Misconfiguration — Kesalahan Konfigurasi Berujung Malapetaka

Security Misconfiguration menempati peringkat ke-2 (**#2**) dalam daftar risiko keamanan web **OWASP Top 10:2025**. Kerentanan ini merupakan salah satu vektor serangan paling umum di seluruh ekosistem cloud dan enterprise, sering kali menjadi pintu masuk awal (*initial access*) paling mudah bagi penyerang siber karena tidak memerlukan eksploitasi logika kode rumit.

---

### 1. Alasan Kuat / Mengapa Ini Sangat Penting (Why It Matters)
Banyak organisasi menghabiskan miliaran rupiah untuk membeli firewall generasi terbaru dan enkripsi tercanggih, namun roboh hanya karena satu opsi konfigurasi default tidak dimatikan:

- **Insiden Nyata Berdampak Finansial Raksasa**:
  - **Kebocoran AWS S3 Bucket (Accenture, Booz Allen, GoDaddy)**: Ratusan juta rekaman rahasia, kunci privat SSH, dan basis data pelanggan bocor ke internet publik hanya karena opsi perizinan bucket dibiarkan `Public Read` secara default.
  - **Denda Regulasi Global**: Komisi Perdagangan Federal AS (FTC) dan otoritas kepatuhan privasi UE (GDPR) secara konsisten menjatuhkan denda puluhan hingga ratusan juta dolar atas kelalaian membiarkan password default (`admin:admin`) dan endpoint diagnostik terbuka.
- **Waktu Penetrasi Eksponensial**:
  Penyerang tidak perlu membongkar algoritma kriptografi jika folder `/backups/` dapat diakses bebas atau akun pengelola sistem masih menggunakan kata sandi bawaan vendor. Bot scanner otomatis di internet (seperti Shodan dan Censys) menemukan server salah konfigurasi dalam hitungan detik.

---

### 2. Gambaran Sistem yang Dihadapi (Target Commercial Architecture)
Dalam laboratorium ini, Anda menghadapi platform orkestrasi infrastruktur cloud enterprise: **CloudCore OpsGateway (Enterprise Cloud Management & Microservices Monitor)** yang berjalan pada port `8012`.

Sistem ini merepresentasikan gerbang ingress kontrol kluster Kubernetes komersial:
- **Operations Gateway**: Mengatur lalu lintas manajemen antar microservices internal (`auth-proxy`, `telemetry-collector`, `backup-manager`, `billing-engine`).
- **Telemetry & Diagnostic Endpoints**: Endpoint `/debug/vars` yang tidak sengaja dibiarkan aktif di lingkungan produksi, mengekspos variabel runtime, versi kernel, path direktori server internal, dan daftar rute sensitif.
- **Snapshot Storage Volume**: Direktori statis `/backups/` di mana server web Nginx/Werkzeug menjalankan fitur *directory listing / autoindex*, memungkinkan publik menjelajahi dan mengunduh seluruh arsip konfigurasi cadangan (`backup_config.json.bak`).
- **Administrative Control Plane**: Portal login `/login` dan konsol supervisor `/admin` yang masih menggunakan kredensial bawaan pabrik.

---

### 3. Sasaran, Temuan & Eksekusi (Search, Find, and Execute)
Metodologi investigasi terstruktur yang harus Anda kuasai:

1. **Apa yang Dicari (Reconnaissance Targets)**:
   - Header respons HTTP yang membocorkan teknologi server (*fingerprinting*).
   - Rute diagnostik standar industri: `/debug/vars`, `/health`, `/server-status`, `/actuator`.
   - File cadangan dengan ekstensi berbahaya: `.bak`, `.old`, `.json.bak`, `.sql.gz`, `.env`.
2. **Apa Temuan yang Menandakan Celah (Vulnerability Indicators)**:
   - Direktori `/backups/` mengembalikan daftar file alih-alih `403 Forbidden` (*Unauthenticated Directory Browsing*).
   - Variabel runtime `/debug/vars` mengonfirmasi `default_credentials_enabled: true` dan `debug_mode: true`.
3. **Eksekusi Eksploitasi Terukur (Strategic Execution)**:
   - Unduh arsip konfigurasi cadangan `backup_config.json.bak`.
   - Lakukan inspeksi statis terhadap properti `admin_system_key` dan kredensial database internal.
   - Autentikasi ke konsol supervisor `/admin` menggunakan akun default `admin` / `admin` untuk mengonfirmasi pengambilalihan hak akses penuh.

---

### 4. Pondasi Kuat: Spektrum Kesalahan Konfigurasi
Security Misconfiguration dapat terjadi di setiap lapisan tumpukan teknologi:

| Lapisan Sistem | Contoh Cacat Konfigurasi | Dampak Risiko |
| :--- | :--- | :--- |
| **Aplikasi Web** | `DEBUG=True`, stack trace verbose di browser | Bocornya path file sistem, kunci `SECRET_KEY`, dan snippet kode |
| **Web Server** | Opsi `autoindex on` aktif pada web server | Pengguna dapat menjelajahi dan menyedot seluruh file di direktori |
| **Manajemen Akun** | Password default pabrik (`admin:admin`, `root:root`) | Pengambilalihan akun administratif secara instan |
| **Penyimpanan Cloud** | Bucket backup publik tanpa autentikasi token | Ekstraksi dump database dan kunci sertifikat SSL/TLS |
