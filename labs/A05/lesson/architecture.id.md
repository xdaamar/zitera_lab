# Prinsip Arsitektur Injection

Aplikasi target berjalan sebagai micro-service native lokal yang terisolasi pada `127.0.0.1:8015`.

```text
[Klien / Pembelajar]
       │
       ▼ (HTTP Request membawa Payload)
[127.0.0.1:8015] ──► [HTTP Query Interpreter]
                             │
                             ▼ (Pelanggaran Konkatenasi String)
                    [Database / Engine Eksekusi]
                             │
                             ▼ (Kebocoran Data Tak Terotorisasi)
                    [Respon / Data Terpapar]
```

### Batas Kepercayaan Arsitektur (*Architectural Trust Boundary*)
Kelemahan mendasar pada arsitektur ini adalah ketiadaan lapisan pemisah antara data masukan pengguna dan sintaks eksekusi database. Ketika input pengguna digabungkan langsung melalui konkatenasi string ke dalam query SQL, mesin parser database tidak mampu membedakan mana perintah yang dimaksud oleh developer dan mana data masukan dari pengguna luar.

Mitigasi arsitektural wajib memisahkan jalur data dan jalur instruksi melalui *Parameterized Queries / Prepared Statements*.
