# Prinsip Arsitektur Software and Data Integrity Failures

Aplikasi target berjalan sebagai micro-service native lokal yang terisolasi pada `127.0.0.1:8018`.

```text
[Klien / Pembelajar]
       │
       ▼ (HTTP Request / File Config Manipulasi)
[127.0.0.1:8018] ──► [Parser Konfigurasi / Lisensi]
                             │
                             ▼ (Kegagalan Verifikasi Integritas)
                    [State & Validation Engine]
                             │
                             ▼ (Hak Akses Tak Terotorisasi Diberikan)
                    [Flag Vault Terbuka]
```

### Batas Kepercayaan Arsitektur (*Architectural Trust Boundary*)
Lapisan arsitektur ini gagal karena mempercayai berkas konfigurasi atau muatan serialisasi data dari disk tanpa memverifikasi tanda tangan digital asimetris. Tanpa tanda tangan kriptografis (seperti Ed25519), penyerang yang memiliki akses lokal ke disk dapat mengubah parameter hak akses secara bebas.

Mitigasi arsitektural mewajibkan seluruh berkas kritis diverifikasi integritas hash dan tanda tangan digitalnya sebelum dibaca oleh mesin validasi aplikasi.
