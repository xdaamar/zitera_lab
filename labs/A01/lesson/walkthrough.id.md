# Panduan Praktik Terbimbing: Investigasi & Pembuktian IDOR pada ApexFin Treasury

Dalam latihan terarah ini, kamu akan mempraktikkan proses audit keamanan nyata terhadap portal perbendaharaan korporat **ApexFin Corporate Treasury** yang berjalan di `http://127.0.0.1:8011`. Latihan ini disusun mengikuti alur investigasi bug bounty profesional: **Observasi $\rightarrow$ Hipotesis $\rightarrow$ Pembuktian PoC $\rightarrow$ Eksfiltrasi Flag**.

---

### Tahap 1: Observasi & Pemetaan Sistem (Reconnaissance)
Langkah pertama dalam pengujian penetrasi adalah memetakan perilaku normal aplikasi.

1. Buka peramban web dan akses portal ApexFin Treasury:
   `http://127.0.0.1:8011`
2. Lakukan autentikasi menggunakan akun analis junior:
   - **Username:** `alice`
   - **Password:** `password123`
3. Amati **Dashboard Operasional**:
   - Alice hanya melihat 1 transaksi kawat resmi di buku besar: **`TRX-APEX-1001`** (Cloud Compute Retainer - $4,950.00).
4. Klik tombol **"View Wire Slip"** pada transaksi kawat tersebut.
5. Perhatikan bilah alamat (*address bar*) peramban:
   `http://127.0.0.1:8011/wire/1`
   - Parameter angka `1` adalah referensi objek langsung (*Direct Object Reference*) ke baris database.

---

### Tahap 2: Pengujian Hipotesis & Parameter Tampering
Sekarang ujilah hipotesis: *Apakah server memvalidasi kepemilikan voucher transfer terhadap sesi login Alice?*

1. Tetap berada di halaman peramban, lalu klik bilah alamat (*address bar*).
2. Ubah URL dari:
   `http://127.0.0.1:8011/wire/1`
   menjadi:
   `http://127.0.0.1:8011/wire/2`
3. Tekan **Enter** dan amati hasilnya:
   - **Status Respon:** Berhasil dimuat dengan status `200 OK`.
   - **Temuan Bukti:** Data yang tampil di layar adalah voucher kawat **`TRX-APEX-1002`** bernilai $1,450.00 milik analis lain (**Robert Sterling / Bob**)!
   - **Analisis Kritis:** Cookie sesi yang dikirimkan tetap milik Alice (`user_sess_1`), namun server mengembalikan voucher milik Bob tanpa penolakan HTTP 403 Forbidden. Ini membuktikan adanya kerentanan **Horizontal IDOR**!

---

### Tahap 3: Eksfiltrasi Dokumen Berstatus Rahasia (Sovereign Wire #42)
Sebagai bug hunter, temuan horizontal IDOR biasa mungkin bernilai $1,500. Namun, jika kamu berhasil mengakses dokumen bernilai tinggi atau data pengawas regulasi (Sovereign/Executive Record), dampaknya melonjak menjadi **Critical ($5,000+)**!

1. Coba lakukan penelusuran terhadap ID khusus yang mencurigakan di URL, misalnya nomor referensi audit eksekutif:
   `http://127.0.0.1:8011/wire/42`
2. Halaman akan merender **Official Wire Settlement Voucher** berstatus **RESTRICTED**:
   - **Kode Referensi:** `TRX-APEX-9901-SOVEREIGN`
   - **Nilai Transaksi:** `$99,999.00`
   - **Signatory:** Marcus Reid (VP Corporate Controller & Auditor)
   - **Compliance Memo:** Memuat bukti validasi autoritatif berupa CTF Flag!
3. Perhatikan pita ambient di bagian atas: tombol **`[📋 COPY FLAG]`** aktif dan kamu dapat menyalin flag langsung ke clipboard untuk disubmit ke Zitera Lab desktop.

---

### Tahap 4: Verifikasi Uji Mandiri via Terminal (cURL)
Kamu juga dapat melakukan pembuktian tanpa browser menggunakan perintah cURL di terminal Zitera Lab:

```bash
# Uji akses voucher #42 langsung menggunakan cookie sesi Alice
curl -s -i "http://127.0.0.1:8011/wire/42" -H "Cookie: session_user=user_sess_1"
```
Perhatikan bahwa server mengembalikan payload lengkap dokumen rahasia tanpa meminta login ulang sebagai admin.
