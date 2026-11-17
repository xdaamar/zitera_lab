# Panduan Praktik Terbimbing: Investigasi & Pembuktian IDOR pada VaultPay Enterprise

Dalam latihan terarah ini, kamu akan mempraktikkan proses audit keamanan profesional terhadap portal perbendaharaan institusional **VaultPay Enterprise (Fintech B2B Treasury & Wire Transfer Portal)** yang berjalan di `http://127.0.0.1:8011`. Metodologi pengujian mengikuti tahapan standar industri: **Observasi $\rightarrow$ Hipotesis $\rightarrow$ Pembuktian PoC $\rightarrow$ Eksfiltrasi Bukti**.

---

### Tahap 1: Observasi & Pemetaan Sistem (Reconnaissance)
1. Buka peramban web dan akses portal VaultPay Enterprise:
   `http://127.0.0.1:8011`
2. Lakukan autentikasi menggunakan akun staf analis likuiditas junior:
   - **Username:** `alice`
   - **Password:** `password123`
3. Amati antarmuka **Dispatched Wire Settlement Ledger**:
   - Alice hanya ditugaskan 1 transaksi kawat resmi pada siklus fiskal ini: **`TRX-VLT-1001`** (Cloud Infrastructure Compute Retainer - $4,950.00).
4. Klik tombol **"View Voucher #1 &rarr;"** pada baris transaksi tersebut.
5. Perhatikan bilah alamat (*address bar*) peramban:
   `http://127.0.0.1:8011/wire/1`
   - Angka `1` merupakan referensi objek langsung (*Direct Object Reference*) yang diteruskan langsung ke query database backend.

---

### Tahap 2: Pengujian Hipotesis & Parameter Tampering
Ujilah hipotesis keamanan: *Apakah backend memvalidasi bahwa voucher nomor 2 benar-benar dimiliki oleh sesi login Alice?*

1. Tetap berada di halaman peramban, lalu klik bilah alamat (*address bar*).
2. Ubah URL dari:
   `http://127.0.0.1:8011/wire/1`
   menjadi:
   `http://127.0.0.1:8011/wire/2`
3. Tekan **Enter** dan perhatikan respons sistem:
   - **Status Respon:** Berhasil dimuat dengan status `200 OK`.
   - **Temuan Bukti:** Voucher yang ditampilkan adalah **`TRX-VLT-1002`** bernilai $1,450.00 milik analis senior (**Robert Sterling / Bob**)!
   - **Analisis Kritis:** Cookie sesi otentikasi Alice tetap aktif, namun server mengizinkan akses ke catatan finansial Bob tanpa verifikasi hak kepemilikan objek (*Horizontal Privilege Escalation*).

---

### Tahap 3: Eksfiltrasi Dokumen Berstatus Rahasia (Voucher #42)
Dalam program bug bounty institusional, eksfiltrasi catatan transaksi eksekutif berstatus rahasia (*Restricted Sovereign Ledger*) merupakan bukti dampak kritis (*Critical Severity*).

1. Ujilah ID khusus yang dialokasikan untuk transaksi audit otoritas moneter / eksekutif:
   `http://127.0.0.1:8011/wire/42`
2. Sistem akan merender **Official Wire Settlement Voucher** berstatus **RESTRICTED**:
   - **Kode Referensi:** `TRX-VLT-9901-SOVEREIGN`
   - **Nilai Transaksi:** `$99,999.00`
   - **Originator:** VaultPay Sovereign Escrow Reserve (Marcus Reid - VP Corporate Controller)
   - **Clearing Record & Compliance Memo:** Memuat segel validasi autoritatif audit kepatuhan SOX-404 dalam format `ZITERA{...}`!
3. Salin flag validasi tersebut untuk dikirimkan (*submit*) ke Zitera Lab desktop.

---

### Tahap 4: Verifikasi Bukti Menggunakan cURL Terminal
Kamu juga dapat membuktikan celah ini secara programatik melalui terminal:

```bash
# Permintaan langsung ke endpoint JSON API untuk voucher #42
curl -s -i "http://127.0.0.1:8011/wire/42?format=json"
```
Amati bahwa objek JSON dikembalikan secara transparan oleh backend, membuktikan bahwa lapisan API tidak memiliki gerbang otorisasi objek sama sekali.
