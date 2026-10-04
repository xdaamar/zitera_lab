# Pembahasan Teknis: Vektor Serangan Supply Chain

## 1. Typosquatting
Penyerang mendaftarkan nama paket yang sangat mirip dengan pustaka populer pada registry publik:
- Nama asli: `cross-env`
- Nama tiruan berbahaya: `crossenv` atau `cross_env`

Developer yang terburu-buru atau salah ketik satu huruf saat menjalankan `npm install` akan tanpa sadar mengunduh pustaka palsu yang berisi kode pencuri data (*stealer malware*).

## 2. Dependency Confusion
Banyak perusahaan menggunakan paket internal khusus (misal `@corp/internal-auth`). Bila registry internal tidak dikonfigurasi dengan prioritas yang benar, penyerang dapat mempublikasikan paket dengan nama identik namun nomor versi lebih tinggi (misal `v99.0.0`) di registry publik (seperti npmjs.com), memicu build pipeline untuk otomatis mengunduh versi berbahaya milik penyerang.

## 3. Kompromi Akun Maintainer & Hijacking
Penyerang mencuri kredensial pengembang open-source melalui phishing atau credential stuffing, lalu merilis pembaruan resmi (*patch update*) yang diam-diam menyisipkan payload trojan.
