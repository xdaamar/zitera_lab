# Prinsip Arsitektur Security Logging and Alerting Failures

Aplikasi target berjalan sebagai micro-service native lokal yang terisolasi pada `127.0.0.1:8019`.

```text
[Klien / Pembelajar]
       │
       ▼ (Operasi Sensitif / Akses Admin)
[127.0.0.1:8019] ──► [Audit Logger Component]
                             │
                             ▼ (Blind Spot / Data Hilang Tanpa Alert)
                    [Log Sink / Perekam Acara]
                             │
                             ▼ (Aktivitas Tak Terdeteksi)
                    [Penyusupan Tanpa Jejak Audit]
```

### Batas Kepercayaan Arsitektur (*Architectural Trust Boundary*)
Kegagalan arsitektur ini terletak pada ketidakmampuan sistem membedakan kejadian biasa dari kejadian berisiko tinggi. Kurangnya pencatatan peristiwa autentikasi, tidak adanya redaksi data rahasia (sehingga token/kata sandi bocor ke berkas log), serta ketiadaan alarm peringatan dini membuka ruang bagi penyerang untuk bergerak tanpa terdeteksi.

Mitigasi arsitektural mewajibkan pencatatan peristiwa audit secara terstruktur (JSON audit format), redaksi otomatis data rahasia, serta pencegahan injeksi log (*log injection defense*).
