# Prinsip Arsitektur Mishandling of Exceptional Conditions

Aplikasi target berjalan sebagai micro-service native lokal yang terisolasi pada `127.0.0.1:8020`.

```text
[Klien / Pembelajar]
       │
       ▼ (Request Memicu Kondisi Galat)
[127.0.0.1:8020] ──► [Upstream Micro-Service Gateway]
                             │
                             ▼ (Exception / Timeout Terjadi)
                    [Exception Handler Block]
                             │
                             ▼ (Pelanggaran Fail-Open: Gerbang Terbuka)
                    [Bypass Otorisasi & Ekstraksi Flag]
```

### Batas Kepercayaan Arsitektur (*Architectural Trust Boundary*)
Kegagalan mendasar pada desain ini adalah implementasi penanganan galat yang bersifat *fail-open*. Ketika layanan pemverifikasi izin eksternal mengalami timeout atau melempar eksepsi 500, blok penangkap galat secara keliru memperlakukan kegagalan tersebut sebagai izin akses (*allow*), bukan penolakan (*deny*).

Mitigasi arsitektural mewajibkan seluruh kontrol keamanan beroperasi di bawah prinsip *fail-closed / deny-by-default*, di mana setiap kesalahan teknis harus mengunci akses secara aman.
