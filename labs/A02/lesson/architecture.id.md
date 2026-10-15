# Arsitektur Keamanan: Penguatan Sistem Berbasis DevSecOps & IaC

Dalam arsitektur modern yang mengandalkan infrastruktur komputasi awan dan kontainer mikroservis, konfigurasi keamanan tidak boleh lagi dikelola secara manual oleh manusia melalui remote shell (SSH). Mengelola server secara manual selalu berujung pada inkonsistensi (*configuration drift*) dan kelalaian manusia.

---

### Tiga Fondasi Arsitektur Konfigurasi Aman

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚ 1. Infrastructure as Code (IaC) & Baseline Hardening    â”‚
â”‚    - Didefinisikan dalam kode (Terraform / Ansible)     â”‚
â”‚    - Mengacu pada standar CIS Benchmarks                â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
                            â”‚
                            â–¼
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚ 2. Pipeline Otomatisasi CI/CD & Static Auditing         â”‚
â”‚    - Linter konfigurasi (Checkov / Trivy / Hadolint)    â”‚
â”‚    - Blokir build jika autoindex aktif / port debug ada â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
                            â”‚
                            â–¼
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚ 3. Runtime Immutability & Secret Management             â”‚
â”‚    - Kontainer berjalan sebagai 'non-root user'         â”‚
â”‚    - Root filesystem bersifat 'read-only'               â”‚
â”‚    - Rahasia dipasok via Vault / AWS Secrets Manager    â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

---

### Prinsip Utama Arsitektur Hardening

#### 1. Server & Container Immutability (Infrastruktur Abadi)
Jangan pernah melakukan debugging atau membuat file backup darurat langsung di dalam server produksi yang sedang berjalan:
- Jika server mengalami masalah, ganti dengan instance kontainer baru yang bersih.
- Terapkan opsi sistem berkas hanya-baca (*read-only root filesystem*):
  ```dockerfile
  # Dockerfile Defensif
  FROM python:3.12-alpine
  RUN addgroup -S appgroup && adduser -S appuser -G appgroup
  USER appuser
  WORKDIR /home/appuser
  # Jalankan tanpa izin root!
  ```

#### 2. Pemisahan Total Antara Kode dan Rahasia (*Secret Management*)
- Kunci enkripsi, password database, dan token API **tidak boleh berada di file konfigurasi statis** di dalam direktori web.
- Gunakan variabel lingkungan yang disuntikkan secara dinamis pada saat booting sistem dari manajemen rahasia terenkripsi (seperti HashiCorp Vault atau AWS Secrets Manager).

#### 3. Penanganan Error Terpusat & Sanitasi Output
Setiap aplikasi wajib mengarahkan seluruh eksepsi yang tidak tertangani (*unhandled exceptions*) ke pengendali error terpusat yang aman:
- Klien publik hanya menerima kode status generik dan nomor ID insiden (*tracking ID*):
  `{"status": "error", "message": "Terjadi kesalahan internal. Hubungi tim dukungan dengan ID: REF-98124"}`
- Rincian *stack trace*, nama modul, path direktori server, dan query SQL hanya dicatat ke file log internal yang terproteksi, bukan ke browser pengguna.
