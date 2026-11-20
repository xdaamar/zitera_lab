# Pembahasan Teknis: Anatomi Celah Kesalahan Konfigurasi

Security Misconfiguration bukanlah akibat dari bug penulisan fungsi matematika atau kesalahan sintaks kode, melainkan kegagalan dalam menegakkan kebijakan keamanan saat mengoperasikan perangkat lunak.

---

### Vektor 1: Directory Indexing & Autoindex Terbuka
Ketika web server menerima request ke path direktori (misalnya `GET /backups/`):
1. Web server mencari file indeks standar seperti `index.html`, `index.htm`, atau `index.php`.
2. Jika file indeks tersebut tidak ditemukan, server memeriksa opsi konfigurasi direktori.
3. Bila fitur **`autoindex on`** (pada Nginx) atau **`Options +Indexes`** (pada Apache) aktif, server secara otomatis menghasilkan dokumen HTML yang memuat daftar tautan seluruh file di dalam folder tersebut beserta ukuran dan tanggal perubahannya.

```http
HTTP/1.1 200 OK
Content-Type: text/html

<html>
<head><title>Index of /backups/</title></head>
<body>
<h1>Index of /backups/</h1>
<hr><pre>
<a href="../">../</a>
<a href="backup_config.json.bak">backup_config.json.bak</a>  15-Oct-2026 10:20  1485
<a href="database_dump.sql">database_dump.sql</a>            14-Oct-2026 23:45  4891024
</pre><hr>
</body>
</html>
```

---

### Vektor 2: Artefak Cadangan Terlantar (*Dangling Backup Files*)
Kebiasaan buruk administrator saat melakukan pemeliharaan darurat adalah membuat salinan langsung di folder publik:
- Mengganti ekstensi file dinamis menjadi file teks statis:
  - `config.php` dieksekusi oleh interpreter PHP (aman dari pembacaan source code).
  - Namun ketika disalin menjadi `config.php.bak` atau `config.php~`, web server tidak lagi mengeksekusinya sebagai skrip, melainkan mengirimkannya sebagai teks murni (*plain text*) kepada siapa pun yang meminta URL tersebut!

---

### Vektor 3: Kredensial Bawaan & Portal Administratif Tanpa Proteksi
Banyak sistem industri, panel kontrol IoT, router jaringan, dan framework web menyertakan akun pabrikan default:
- Username: `admin`, Password: `admin` atau `password`
- Port administratif seperti `8012`, `8080`, `9090` yang diekspos tanpa pembatasan IP (*IP allowlisting*).

---

### Perbandingan Konfigurasi: Nginx & Web Server

#### Konfigurasi Nginx yang Rentan (Vulnerable):
```nginx
server {
    listen 8012;
    server_name portal.local;
    root /var/www/html;

    # KESALAHAN 1: Autoindex aktif!
    location /backups/ {
        autoindex on;
    }

    # KESALAHAN 2: Menampilkan versi Nginx pada header server
    server_tokens on;

    # KESALAHAN 3: Tidak ada header keamanan dasar
}
```

#### Konfigurasi Nginx yang Diperkuat (Hardened & Secure):
```nginx
server {
    listen 8012;
    server_name portal.local;
    root /var/www/html;

    # PERBAIKAN 1: Matikan autoindex secara global dan spesifik
    location /backups/ {
        autoindex off;
        deny all; # Blokir total akses publik ke direktori backup!
        return 404;
    }

    # PERBAIKAN 2: Sembunyikan informasi versi server
    server_tokens off;

    # PERBAIKAN 3: Pasang HTTP Security Headers wajib
    add_header X-Frame-Options "DENY" always;
    add_header X-Content-Type-Options "nosniff" always;
    add_header Content-Security-Policy "default-src 'self';" always;
    add_header Strict-Transport-Security "max-age=31536000; includeSubDomains" always;
}
```
