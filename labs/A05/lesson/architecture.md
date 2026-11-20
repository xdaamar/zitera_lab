# Arsitektur Sistem & Pemisahan Jalur Kontrol-Data

## 1. Topologi Arsitektur Lab A05 (Katalog Logistik Central)

Aplikasi target lab A05 mensimulasikan layanan portal inventaris perangkat keras jaringan (*Central Logistics Hardware Catalog*) yang berjalan pada port 8015:

```text
[ Browser / Klien Penyerang ]
             │
             ▼  HTTP GET /?q=<payload> (Port 8015)
┌──────────────────────────────────────────────────────────┐
│              FLASK WEB RUNTIME CONTAINER                 │
│                                                          │
│  ┌────────────────────────────────────────────────────┐  │
│  │ HTTP Request Handler (app.py)                      │  │
│  │ • Membaca parameter q dari query string            │  │
│  │ • Melakukan f-string concatenation ke SQL mentah   │  │
│  └────────────────────────┬───────────────────────────┘  │
│                           │                              │
│                           ▼ (SQL String Mentah)          │
│  ┌────────────────────────────────────────────────────┐  │
│  │ SQLite Database Engine (inventory.db)              │  │
│  │                                                    │  │
│  │  ┌──────────────────────┐  ┌────────────────────┐  │  │
│  │  │ Tabel products       │  │ Tabel vault_secrets│  │  │
│  │  │ • id, name           │  │ • id, secret_name  │  │  │
│  │  │ • category, price    │  │ • secret_data      │  │  │
│  │  └──────────────────────┘  └────────────────────┘  │  │
│  └────────────────────────┬───────────────────────────┘  │
│                           │                              │
│                           ▼                              │
│  ┌────────────────────────────────────────────────────┐  │
│  │ HTML Jinja2 Template Renderer                      │  │
│  │ • Menampilkan baris hasil produk / rahasia         │  │
│  │ • Menampilkan pesan error SQLite secara mentah     │  │
│  └────────────────────────────────────────────────────┘  │
└──────────────────────────────────────────────────────────┘
```

---

## 2. Analisis Titik Lemah Arsitektural (Architectural Flaws)
Aplikasi target memiliki tiga kegagalan arsitektur struktural yang saling memperparah:
1. **Pencampuran Jalur Kontrol dan Jalur Data**: Tidak ada lapisan abstraksi antara instruksi query yang disusun developer dan data yang dimasukkan pengguna. Penggunaan f-string (`f"SELECT ... '{query}%'"`) membuat sintaks SQL dan data pengguna berada pada satu saluran teks yang sama.
2. **Ketiadaan Prinsip Hak Akses Terkecil (*Least Privilege*)**: Layanan web menggunakan satu koneksi database bersama dengan izin penuh untuk membaca seluruh tabel, termasuk tabel sensitif `vault_secrets`.
3. **Penyebaran Informasi Diagnostik Internal (*Verbose Error Leakage*)**: Tangkapan pengecualian `except Exception as e: error = str(e)` dikirim langsung ke frontend HTML, memberikan petunjuk sintaks internal SQLite kepada penyerang.

---

## 3. Arsitektur Pertahanan Berlapis (Defense-in-Depth Architecture)

Sistem berdaya tahan tinggi memisahkan secara ketat siklus kompilasi query dan siklus transmisi data:

```text
[ User Input ] ──▶ [ Validation & Sanitization Gate ]
                                │ (Type & Range Check)
                                ▼
                   [ AST Query Compiler ] ◀── [ Predefined SQL Template ]
                                │
                                ▼ (Compiled AST Bytecode)
                   [ Database Execution Engine ]
                                ▲
                                │ (Inert Data Channel)
                   [ Parameter Binding Stream ] ◀── [ Sanitized Values ]
```

### Pilar Arsitektur yang Aman:
- **Precompiled Abstract Syntax Tree (AST)**: Mesin database mengompilasi struktur query SQL terlebih dahulu sebelum menerima data. Data pengguna dikirimkan melalui jalur pipa terpisah (*out-of-band parameter stream*) sehingga karakter kutip tunggal tidak akan pernah dievaluasi sebagai pemutus string.
- **Segmentasi Skema & Least Privilege**: Akun database yang digunakan oleh portal publik hanya diberikan izin `SELECT` pada tabel katalog publik (`products`), dan dilarang keras mengakses tabel kredensial (`vault_secrets`) maupun metadata skema.
- **Lapisan WAF & Error Suppression**: Menangkap seluruh error database pada middleware internal dan hanya menampilkan pesan ramah umum kepada pengguna, sekaligus mencatat rincian trace ke server log terenkripsi.
