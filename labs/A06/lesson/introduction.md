# A06: Insecure Design — Cacat Desain Arsitektur & Logika Bisnis

Insecure Design menempati peringkat ke-6 (**#6**) dalam daftar risiko **OWASP Top 10:2025**. Kategori ini merepresentasikan pergeseran paradigma penting dalam keamanan siber modern: membedakan secara tegas antara *kesalahan penulisan kode* (*implementation bug*) dan *cacat desain arsitektur* (*design flaw*). Suatu aplikasi dapat ditulis tanpa ada satu pun bug memori atau celah injeksi, namun tetap mudah dibobol karena model bisnis atau arsitektur dasarnya cacat sejak awal.

---

### 1. Kenapa Ini Sangat Berbahaya? (Business & Financial Impact)
Cacat desain adalah kerentanan paling mahal dan paling sulit diperbaiki dalam siklus pengembangan perangkat lunak:

- **Kebocoran Finansial Langsung Lewat Manipulasi Alur Bisnis**:
  - Penyerang tidak perlu membobol server jika mereka bisa memanipulasi alur transaksi bisnis, seperti mengubah status pesanan barang senilai puluhan ribu dolar dari "Belum Bayar" (*DRAFT*) langsung melompat menjadi "Terkirim" (*DISPATCHED*) tanpa persetujuan tim keuangan.
  - Insiden nyata seperti kasus pembobolan sistem pengadaan korporasi dan platform e-commerce tiket konser, di mana celah logika alur (*race conditions* dan *state machine bypass*) mengakibatkan kerugian finansial jutaan dolar tanpa memicu peringatan firewall.
- **Biaya Perbaikan Sangat Tinggi (*Cost to Fix*)**:
  Memperbaiki bug implementasi (seperti SQL injection) cukup dengan mengganti satu baris query. Namun memperbaiki Insecure Design sering kali memerlukan perombakan total arsitektur database, desain antarmuka, dan alur otorisasi multi-divisi yang memakan waktu berbulan-bulan.

---

### 2. Bagaimana Cara Mencari & Mengujinya? (Recon & Vulnerability Indicators)
Metodologi pengujian logika alur bisnis profesional:

1. **Apa yang Dicari (What to Search)**:
   - Mesin status (*State Machines*) pada alur bertahap: pendaftaran akun, checkout keranjang belanja, persetujuan pinjaman, atau pengadaan barang.
   - Endpoint API yang menerima status berikutnya dari sisi klien: e.g. `POST /api/orders/<id>/transition` dengan parameter `target_state` atau `status`.
2. **Apa Temuan yang Menandakan Celah (Vulnerability Indicators)**:
   - Klien diperbolehkan menentukan sendiri status target (`target_state: "DISPATCHED"`).
   - Server tidak memvalidasi syarat prasyarat (*invariants*), seperti ketiadaan tanda tangan manajer atau ketiadaan bukti kliring pembayaran bank.
3. **Eksekusi Eksploitasi Terukur (Strategic Execution)**:
   - Identifikasi pesanan bernilai tinggi `#9999` ($50,000) yang masih berstatus `DRAFT`.
   - Kirimkan instruksi lompatan status langsung ke `DISPATCHED` melalui panggilan REST API.
   - Amati respons server yang mengesahkan pengiriman dan membuka flag autoritatif `ZITERA{1n53cur3_d351gn_fl4w3d_w0rkfl0w}`.

---

### 3. Bagaimana Cara Memperbaikinya? (Secure Coding & Remediation)
Prinsip rekayasa perangkat lunak defensif:
- **Terapkan State Machine yang Ketat di Sisi Server (Strict Invariants)**: Status hanya boleh berpindah ke tahap berikutnya jika dan hanya jika seluruh kondisi prasyarat terpenuhi (misal: `DRAFT` hanya boleh pindah ke `PENDING_REVIEW`, bukan langsung ke `DISPATCHED`).
- **Larang Klien Mengirimkan Nama Status**: Klien hanya boleh mengirimkan *aksi* (misal: "ajukan_tinjauan", "setujui"), sedangkan server yang menentukan status berikutnya berdasarkan peran pengguna di sesi.
- **Terapkan Pemodelan Ancaman (Threat Modeling)**: Gunakan kerangka kerja seperti STRIDE sejak fase desain arsitektur sebelum satu baris kode pun ditulis.

---

### Gambaran Sistem yang Dihadapi (Target Commercial Architecture)
Dalam praktikum ini, kamu mengaudit sistem pengadaan barang korporasi: **Zitera Procurement Workflow / AuthShield Identity (Corporate Enterprise Procurement)** yang beroperasi pada port `8016`.

Komponen arsitektur:
- **Sistem Pengadaan Korporasi**: Mengelola permohonan belanja infrastruktur dari staf (`learner`).
- **Aturan Bisnis Nominal**: Permohonan di bawah $500 disetujui otomatis, sedangkan pesanan bernilai $\ge \$500$ wajib melalui tinjauan manajer eksekutif.
- **Cacat Desain Mesin Status**: Endpoint `POST /api/orders/<id>/transition` menerima input bebas `target_state` dari browser klien tanpa memeriksa apakah tahapan persetujuan manajer telah dilewati.
