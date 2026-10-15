# Panduan Praktik Terbimbing: Investigasi & Pembuktian IDOR

Dalam latihan terarah ini, kamu akan mempraktikkan proses audit keamanan menyeluruh terhadap sistem penagihan internal yang berjalan di `http://127.0.0.1:8011`. Latihan ini disusun mengikuti rasio **60% teknik analisis manual** dan **40% penggunaan tools terminal**.

---

### Tahap 1: Observasi & Pemetaan Sistem (15% â€” Manual)
Langkah pertama dalam pengujian penetrasi adalah memetakan perilaku normal aplikasi.

1. Buka peramban web dan akses portal penagihan lokal:
   `http://127.0.0.1:8011`
2. Lakukan autentikasi menggunakan akun standar:
   - **Username:** `alice`
   - **Password:** `password123`
3. Masuk ke menu **"My Invoices"**. Perhatikan bahwa antarmuka web hanya menampilkan daftar faktur milik Alice.
4. Buka Developer Tools peramban (tekan **F12** atau `Ctrl + Shift + I`), lalu pilih tab **Network**.
5. Klik salah satu faktur Alice dan amati detail request HTTP:
   - **Request URL:** `http://127.0.0.1:8011/invoice/1`
   - **Method:** `GET`
   - **Headers:** Perhatikan adanya cookie sesi `session_id=...` yang menandai kamu sebagai Alice.

---

### Tahap 2: Pengujian Hipotesis & Fault Injection (25% â€” Manual)
Sekarang ujilah hipotesis: *Apakah server memvalidasi hak kepemilikan faktur terhadap sesi Alice?*

1. Tetap berada di halaman peramban, lalu klik bilah alamat (*address bar*).
2. Ubah URL dari:
   `http://127.0.0.1:8011/invoice/1`
   menjadi:
   `http://127.0.0.1:8011/invoice/2`
3. Tekan **Enter** dan amati hasilnya:
   - **Observasi:** Halaman berhasil dimuat dengan status `200 OK`.
   - **Temuan Bukti:** Data yang tampil di layar adalah faktur milik pengguna **Bob**!
   - **Analisis Kritis:** Cookie yang dikirimkan tetap milik Alice, namun server mengembalikan data rahasia milik Bob tanpa penolakan otorisasi. Ini adalah bukti nyata kerentanan IDOR.

---

### Tahap 3: Eksploitasi Terarah & Weaponization (35% â€” Tools: cURL)
Gunakan cURL pada terminal lokal untuk mengevaluasi dampak kebocoran data secara terstruktur dan mencari faktur bernilai tinggi (*master invoice*).

1. Buka terminal di dalam workspace ZITERA_LAB.
2. Salin nilai cookie sesi Alice dari tab Network Developer Tools.
3. Jalankan perintah manual cURL untuk memeriksa faktur lain:
   ```bash
   curl -s -i "http://127.0.0.1:8011/invoice/2" -H "Cookie: session_id=YOUR_ALICE_SESSION"
   ```
4. Lakukan penelusuran sistematis terhadap ID bernilai khusus (misalnya `id=0`, `id=3`, atau ID administratif):
   ```bash
   # Uji keberadaan entri indeks awal atau catatan khusus sistem
   curl -s "http://127.0.0.1:8011/invoice/0" -H "Cookie: session_id=YOUR_ALICE_SESSION"
   ```
5. Perhatikan bagaimana response body membocorkan catatan penagihan master yang tidak pernah ditampilkan pada menu web biasa.

---

### Tahap 4: Remediasi & Verifikasi Hasil (25% â€” Manual & Tools)
Tahap penutup adalah memastikan bahwa perbaikan telah efektif dan dapat dibuktikan.

1. Terapkan aturan otorisasi server-side pada controller backend:
   ```python
   # Verifikasi bahwa invoice.owner_id == session['user_id']
   ```
2. Ulangi request cURL terhadap faktur nomor 2 menggunakan akun Alice:
   ```bash
   curl -s -i "http://127.0.0.1:8011/invoice/2" -H "Cookie: session_id=YOUR_ALICE_SESSION"
   ```
3. **Kriteria Keberhasilan:** Server wajib mengembalikan kode status **`403 Forbidden`** atau **`404 Not Found`**, membuktikan bahwa kebocoran data lintas pengguna telah tertutup sempurna.
