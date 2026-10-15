# A02: Security Misconfiguration (Kesalahan Konfigurasi Keamanan)

Security Misconfiguration menempati peringkat ke-2 (**#2**) dalam daftar risiko keamanan web OWASP Top 10:2025. Kerentanan ini merupakan salah satu vektor serangan paling umum, paling sering ditemukan di seluruh spektrum teknologi, dan sering menjadi titik masuk awal paling mudah bagi penyerang siber.

---

### Tujuan Pembelajaran
Setelah menyelesaikan modul ini, kamu diharapkan mampu:
1. Mengidentifikasi berbagai bentuk kesalahan konfigurasi keamanan pada lapisan web server, aplikasi, dan kontainer.
2. Mendeteksi paparan direktori tidak terproteksi (*unauthenticated directory indexing*) dan file artefak cadangan (*backup file leaks*).
3. Melakukan audit manual dan inspeksi header keamanan HTTP menggunakan peramban serta cURL.
4. Menerapkan konfigurasi defensif otomatis (*Infrastructure as Code hardening*, penutupan autoindex, dan sanitasi pesan error).

---

### Spektrum Kerentanan Security Misconfiguration
Berbeda dengan kerentanan logika kode seperti injeksi, Security Misconfiguration dapat terjadi di setiap lapisan tumpukan teknologi (*technology stack*):

| Lapisan Sistem | Contoh Kesalahan Konfigurasi | Dampak Risiko |
| :--- | :--- | :--- |
| **Aplikasi Web** | Mode debug aktif (`DEBUG=True`), stack trace verbose | Kebocoran rahasia `SECRET_KEY`, path file, kredensial DB |
| **Web Server (Nginx/Apache)** | `autoindex on`, halaman status server `/server-status` terbuka | Pengunjung dapat menjelajahi dan mengunduh seluruh isi direktori |
| **Manajemen Kredensial** | Akun default pabrik (`admin` / `admin`, `root` / `root`) | Pengambilalihan akun administratif secara instan |
| **Header HTTP** | Hilangnya header `Content-Security-Policy`, `X-Frame-Options` | Rentan terhadap serangan Clickjacking dan Cross-Site Scripting |
| **Infrastruktur Cloud / Kontainer** | File `.env` atau `.bak` tersimpan di web root publik | Ekstraksi rahasia API key, token cloud, dan koneksi database |

---

### Mengapa Penyerang Mengincar Celah Konfigurasi?
Penyerang tidak perlu menghabiskan waktu berminggu-minggu membedah algoritma enkripsi atau logika transaksi jika sistem administrator membiarkan folder `/backups/` dapat diakses bebas atau menggunakan kata sandi bawaan pabrik. Dengan pemindai otomatis (*automated web crawlers*), penyerang dapat menemukan ribuan server yang salah konfigurasi dalam hitungan menit.
