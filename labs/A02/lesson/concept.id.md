# Pembahasan Teknis: Vektor Serangan Misconfiguration

## 1. Directory Browsing / Indexing Terbuka
Ketika web server (seperti Nginx atau Apache) menerima request ke sebuah folder (misalnya `/backups/`) tanpa adanya file indeks (`index.html` atau `index.php`), dan konfigurasi `autoindex` bernilai aktif (`on`), server akan otomatis menghasilkan daftar isi folder berupa tautan HTML lengkap.

Penyerang dapat memanfaatkan hal ini untuk mengunduh:
- File cadangan basis data (`database.sql`, `backup.tar.gz`).
- File konfigurasi rahasia (`.env`, `config.json.bak`, `settings.py`).
- Source code atau private certificate.

## 2. Kredensial Default pada Panel Admin
Banyak framework atau alat pemantauan sistem menyertakan akun default untuk mempermudah instalasi awal. Bila tidak diganti sebelum naik ke tahap produksi, penyerang dapat langsung mengambil alih kontrol penuh dengan kredensial umum seperti:
- `admin` / `admin`
- `root` / `root`
- `admin` / `password`

## 3. Verbose Error Pages & Debug Mode
Menyalakan mode debugging (`DEBUG = True` pada Django/Flask atau `APP_DEBUG=true` pada Laravel) di server publik akan menampilkan cuplikan kode sumber, kunci enkripsi rahasia (`SECRET_KEY`), serta kredensial database setiap kali terjadi error.
