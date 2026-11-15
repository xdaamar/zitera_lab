# Pengenalan: Mishandling of Exceptional Conditions (OWASP A10:2025)

Dalam rekayasa perangkat lunak, kondisi tak terduga (*exceptional conditions*) adalah keniscayaan: koneksi database terputus di tengah transaksi, server verifikasi hulu (*upstream*) mengalami *timeout*, payload JSON memiliki tipe data yang rusak (*malformed*), atau terjadi kondisi kekurangan memori (*out of memory*). Namun, bagaimana sistem menangani kondisi luar biasa ini memisahkan arsitektur yang aman dari arsitektur yang rapuh. **OWASP A10:2025 – Mishandling of Exceptional Conditions** menyoroti kerentanan fatal di mana penanganan eksepsi yang keliru menyebabkan runtuhnya batas keamanan aplikasi.

Kesalahan paling berbahaya dalam kategori ini adalah **Fail-Open (Gagal-Terbuka)**: ketika terjadi kesalahan atau timeout pada komponen evaluasi keamanan, sistem memilih untuk memberikan izin akses (*grant access*) dengan dalih menjaga ketersediaan layanan (*availability*) atau kenyamanan pengguna (*user experience*).

---

## Mengapa Kategori Ini Sangat Penting dalam OWASP Top 10 2025?

Ketika sistem terdistribusi semakin kompleks dengan puluhan microservices yang saling bergantung, kegagalan parsial (*partial failures*) menjadi hal yang lazim terjadi:

1. **Perbedaan Fail-Closed vs Fail-Open**: Dalam konteks keamanan, prinsip dasar yang wajib dipatuhi adalah **Fail-Closed (Gagal-Terkunci)**: jika verifikasi keamanan tidak dapat diselesaikan dengan sempurna, status akses secara default harus DITOLAK (*DENY*). Memilih Fail-Open membuka celah jalan pintas otorisasi.
2. **Eksploitasi Sengaja melalui Fault Injection**: Penyerang dapat sengaja membanjiri server KYC eksternal atau mengirim karakter unicode yang tidak valid untuk memaksa modul autentikasi melempar eksepsi, sehingga mengaktifkan blok penanganan kesalahan yang tidak aman.
3. **Penyembunyian Exception (*Exception Swallowing*)**: Blok `try...catch (Exception e)` generik yang menangkap seluruh jenis kesalahan tanpa logging atau penanganan spesifik sering kali menutupi kegagalan kritis dari tim operasi.
4. **Kebocoran Informasi Sensitif**: Eksepsi yang tidak tertangani dengan benar sering kali menampilkan stack trace internal, kredensial koneksi, atau versi dependensi ke browser pengguna, memberikan peta jalan kepada penyerang.

---

## Skenario Lab: Zitera Payment & Security Gate

Dalam lab ini, Anda akan berhadapan dengan **Zitera Payment & Security Gate (Port 8020)**. Layanan ini merepresentasikan gerbang otorisasi pembayaran transaksi keuangan dan verifikasi akses identitas:

```
┌─────────────────────────┐          HTTP POST          ┌─────────────────────────┐
│     Untrusted Client    │  ─────────────────────────> │   Zitera Security Gate  │
│  (Attacker / Tester)    │   /api/gate/verify          │       (Port 8020)       │
│                         │   {"simulate_exception":    │                         │
│                         │    "UPSTREAM_TIMEOUT"}      │ ❌ FAIL-OPEN TRIGGERED! │
└─────────────────────────┘                             │  authorized = True      │
                                                        └─────────────────────────┘
```

Secara fatal, pengembang gateway menambahkan blok penanganan pengecualian generik: ketika terjadi kesalahan pemrosesan token atau timeout dari verifikator hulu, sistem secara otomatis mengeksekusi `authorized = True` dan memberikan akses transaksi penuh kepada pengguna tanpa izin!

---

## Sasaran Pembelajaran

Melalui modul ini, Anda akan:
- **Mengidentifikasi Logika Fail-Open**: Mengaudit blok penanganan eksepsi untuk menemukan asumsi default yang melemahkan keamanan.
- **Mendemonstrasikan Fault Injection**: Memicu kondisi eksepsi yang disengaja (timeout hulu, nilai null, error parsing) untuk menerobos gerbang keamanan.
- **Memahami Pola Desain Fail-Safe Defaults**: Mengimplementasikan prinsip keamanan bahwa otorisasi harus berstatus `False` kecuali terbukti valid secara definitif.
- **Membangun Circuit Breaker & Fallback Aman**: Merancang pola penanganan kegagalan hulu yang menjaga ketersediaan sistem tanpa mengorbankan integritas keamanan.
