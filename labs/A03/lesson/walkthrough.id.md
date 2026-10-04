# Panduan Praktik Terbimbing: Investigasi Lockfile Audit

### Tujuan Investigasi
Dalam latihan ini, kamu akan mengaudit laporan build dan lockfile paket dependensi pada sistem yang berjalan di `http://127.0.0.1:8013` untuk mendeteksi paket mencurigakan.

### Langkah 1: Akses Sistem CI/CD Audit
- Buka `http://127.0.0.1:8013` pada peramban web.
- Amati status build pipeline dan laporan dependensi proyek.

### Langkah 2: Temukan Endpoint Laporan Audit Lockfile
- Periksa route `/packages/audit/package_lock_audit.json` menggunakan browser atau curl:
  `curl http://127.0.0.1:8013/packages/audit/package_lock_audit.json`

### Langkah 3: Identifikasi Paket Asing
- Cari objek dependensi di dalam struktur JSON yang memiliki metadata mencurigakan atau nama paket aneh yang disisipkan tanpa tanda tangan digital resmi.
- Perhatikan paket bernama `apex-internal-telemetry`.

### Langkah 4: Ekstraksi Token Eksfiltrasi
- Periksa properti `exfiltrated_pipeline_token` di dalam paket tersebut untuk mendapatkan flag verifikasi.
