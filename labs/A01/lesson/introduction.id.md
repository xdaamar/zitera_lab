# A01: Broken Access Control & IDOR — Primadona Bug Bounty Hunter

Broken Access Control menempati peringkat teratas (**#1**) dalam daftar risiko keamanan web **OWASP Top 10:2025**. Kerentanan ini terjadi ketika sistem aplikasi web atau API gagal membatasi hak akses pengguna secara ketat di sisi server (*server-side authorization failure*), sehingga pengguna yang telah terautentikasi dapat mengakses, memanipulasi, atau menghapus data milik pengguna lain di luar batas izin sahnya.

---

### 1. Alasan Kuat / Mengapa Ini Sangat Penting (Why It Matters)
Di dunia nyata, Broken Access Control bukan sekadar kesalahan kode sepele, melainkan pintu gerbang kerugian finansial masif dan kehancuran reputasi bisnis:

- **Kerugian Finansial & Denda Regulasi Masif**:
  Di bawah regulasi privasi modern seperti **UU Perlindungan Data Pribadi No. 27/2022** di Indonesia atau **GDPR** internasional, kegagalan mengamankan data pengguna berisiko denda hingga puluhan miliar rupiah atau 4% dari perputaran uang global tahunan. Dalam sektor perbankan dan fintech, kebocoran data transaksi dapat memicu pencabutan izin kliring oleh Bank Sentral dan OJK.
- **Kehancuran Kepercayaan & Saham Korporasi**:
  Ketika platform keuangan atau e-commerce membocorkan data nasabah, kepercayaan publik lenyap seketika. Kasus nyata seperti kebocoran First American Financial Corporation (mengakibatkan 885 juta rekaman dokumen hipotek dan transfer bank terbuka akibat ketiadaan kontrol otorisasi pada URL dokumen) menjadi bukti nyata malapetaka bisnis.
- **Tambang Emas Bug Bounty ($1,500 – $8,000+ USD)**:
  Di platform bounty internasional seperti HackerOne dan Bugcrowd, laporan **IDOR (Insecure Direct Object Reference)** selalu digolongkan sebagai **Severity P1/P2 (Critical/High)** karena memungkinkan penyerang mengeksfiltrasi database ribuan nasabah secara massal hanya dengan sedikit iterasi script.

---

### 2. Gambaran Sistem yang Dihadapi (Target Commercial Architecture)
Dalam praktikum ini, kamu tidak menghadapi form latihan buatan yang kaku, melainkan sebuah portal perbankan institusional komersial nyata: **VaultPay Enterprise (Fintech B2B Treasury & Wire Transfer Portal)** yang beroperasi pada port `8011`.

Arsitektur sistem yang kamu hadapi mencakup:
- **Web Application Tier**: Portal web Flask/Python yang melayani antarmuka operator likuiditas dan analis perbendaharaan institusional.
- **API Gateway & Routing Layer**: Endpoint RESTful `/wire/<int:wire_id>` dan `/invoice/<int:wire_id>` yang melayani peramban web maupun integrasi sistem SWIFT ERP via JSON.
- **Identity & Session Management**: Sesi login berbasis cookie HTTP terenkripsi yang mencatat identitas analis (`alice`, `bob`, `admin`).
- **Data Persistence Layer (SQLite / RDBMS)**: Tabel buku besar `wires` yang memisahkan transaksi antar-entitas berdasarkan `user_id`. Cacat terjadi karena handler rute mengambil data langsung berdasarkan parameter ID yang dikirim klien (`wire_id`) tanpa memverifikasi relasi kepemilikan dengan pengguna aktif di sesi!

---

### 3. Sasaran, Temuan & Eksekusi (Search, Find, and Execute)
Sebelum kamu menyentuh sasaran pengujian, kamu harus memahami metodologi pengujian terstruktur:

1. **Apa yang Dicari (The Target)**:
   - Parameter referensi langsung pada URL atau body request HTTP: e.g. `wire_id`, `invoice_id`, `doc_id`, `account_no`.
   - Pola penomoran berurutan (*sequential integer IDs*) atau UUID yang dapat ditebak.
2. **Apa Temuan yang Menandakan Celah (The Vulnerability Indicator)**:
   - Ketika kamu login sebagai analis biasa (`alice`), kamu melihat voucher transaksi milikmu (`wire/1`).
   - Saat parameter URL diubah menjadi `wire/2` (milik analis lain `bob`), server mengembalikan HTTP status `200 OK` dengan data Bob lengkap tanpa penolakan `403 Forbidden`!
3. **Eksekusi Eksploitasi Terukur (The Strategic Execution)**:
   - Lakukan pemetaan terhadap transaksi bernilai tinggi atau dokumen kepatuhan tingkat eksekutif (`wire/42`).
   - Buktikan kegagalan isolasi batas multi-tenant (*tenant boundary bypass*) dan eksfiltrasi segel kepatuhan SOX-404 yang memuat bukti validasi autoritatif (`ZITERA{...}`).

---

### 4. Pondasi Kuat: Perbedaan Kritis AuthN vs AuthZ
Banyak insinyur perangkat lunak pemula keliru mengira sistemnya aman hanya karena telah memasang login password dan token sesi:

| Dimensi Keamanan | Pertanyaan Kunci | Contoh Mekanisme | Titik Kegagalan Umum |
| :--- | :--- | :--- | :--- |
| **Authentication (AuthN)** | *"Siapa kamu?"* | Password, SSO SAML, Multi-Factor Auth (MFA) | Brute-force, password lemah, session hijacking |
| **Authorization (AuthZ)** | *"Apa saja hak aksesmu?"* | Role-Based Access Control (RBAC), Object-Level Ownership | **Broken Access Control**, IDOR, Privilege Escalation |

> **Prinsip Fundamental:** Broken Access Control adalah kegagalan lapisan **Authorization**. Penyerang masuk secara sah menggunakan akun berhak akses rendah, lalu memanipulasi parameter untuk membaca atau mengubah objek milik pihak lain.
