# Remediasi & Panduan Pengerasan Sistem (Security Hardening)

Untuk mengeliminasi celah Security Misconfiguration secara menyeluruh, tim rekayasa perangkat lunak dan DevOps wajib menerapkan daftar periksa pengerasan (*hardening checklist*) pada seluruh tumpukan teknologi.

---

### 1. Nonaktifkan Directory Browsing & Autoindex
Pastikan server web tidak pernah merender daftar isi folder jika file indeks tidak ditemukan:

#### Pada Nginx:
```nginx
# Nonaktifkan autoindex di tingkat http, server, atau location
http {
    autoindex off;
    server_tokens off; # Sembunyikan versi Nginx dari banner HTTP
}
```

#### Pada Apache (`httpd.conf` atau `.htaccess`):
```apache
<Directory /var/www/html>
    Options -Indexes
</Directory>
```

---

### 2. Matikan Mode Debug di Lingkungan Produksi
Mode debugging mengekspos interpreter interaktif dan variabel lingkungan rahasia. Pastikan konfigurasi aplikasi membaca flag dari variabel lingkungan dengan nilai default yang aman:

```python
import os

# Konfigurasi Aman Python / Flask:
DEBUG_MODE = os.environ.get('FLASK_DEBUG', 'False').lower() in ('true', '1')
app.config['DEBUG'] = DEBUG_MODE
app.config['ENV'] = 'production' if not DEBUG_MODE else 'development'
```

---

### 3. Singkirkan File Cadangan dari Web Root Publik
- Jangan pernah menyimpan file `.bak`, `.sql`, `.tar.gz`, atau `.env` di dalam folder dokumen web publik (`public_html`, `/var/www/html`, atau `static/`).
- Tambahkan aturan pemblokiran otomatis pada Nginx untuk seluruh file sensitif:
  ```nginx
  location ~* \.(bak|config|sql|fla|psd|ini|log|sh|inc|swp|dist|env)$ {
      deny all;
      return 404;
  }
  ```

---

### 4. Terapkan HTTP Security Headers Standar Industri
Tambahkan header keamanan berikut ke seluruh respons HTTP untuk menginstruksikan peramban agar memblokir eksploitasi sisi klien:

```python
@app.after_request
def apply_security_headers(response):
    # Cegah serangan Clickjacking
    response.headers['X-Frame-Options'] = 'DENY'
    # Cegah peramban menebak tipe MIME yang keliru
    response.headers['X-Content-Type-Options'] = 'nosniff'
    # Batasi asal muasal konten eksternal
    response.headers['Content-Security-Policy'] = "default-src 'self';"
    # Paksa penggunaan enkripsi HTTPS
    response.headers['Strict-Transport-Security'] = 'max-age=31536000; includeSubDomains'
    return response
```

---

### 5. Penanganan Error yang Aman & Sanitasi Respons
Gantikan halaman error bawaan framework dengan template kustom yang tidak membocorkan stack trace:

```python
@app.errorhandler(500)
def internal_server_error(e):
    # Catat error detail ke sistem log internal yang terproteksi
    logger.error(f"Internal Server Error: {str(e)}", exc_info=True)
    # Tampilkan pesan ramah yang aman kepada pengguna publik
    return jsonify({
        "status": "error",
        "message": "Terjadi kendala pada sistem. Tim kami sedang menanganinya."
    }), 500
```
