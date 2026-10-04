# Remediasi & Langkah Penguatan Sistem (Hardening)

Untuk mengatasi dan mencegah Security Misconfiguration:

## 1. Proses Penguatan Otomatis (Automated Hardening)
- Selalu matikan mode *autoindex* pada web server:
  - Pada Nginx: `autoindex off;`
  - Pada Apache: `Options -Indexes`
- Nonaktifkan mode debug (`DEBUG = False`) secara mutlak di lingkungan *staging* dan *production*.

## 2. Manajemen Kredensial yang Ketat
- Paksa penggantian password bawaan pada login pertama sistem atau instalasi layanan baru.
- Jangan pernah menyimpan kredensial atau kunci rahasia (*secret keys*) di dalam repositori source code atau folder web publik.

## 3. Minimalkan Layanan & Port (Attack Surface Reduction)
- Hapus semua modul, fitur, sampel aplikasi, dan dokumentasi bawaan yang tidak diperlukan untuk operasional bisnis.
- Terapkan pemindaian berkala (*automated configuration audits*) menggunakan alat seperti LinPEAS, Lynis, atau ScoutSuite.
