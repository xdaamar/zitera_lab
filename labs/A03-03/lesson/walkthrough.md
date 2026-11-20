# Panduan Praktik Terbimbing: Investigasi Malicious Preinstall Script & CI/CD Secret Exfiltration

Dalam panduan terarah ini, Anda akan mempraktikkan proses penemuan celah dan eksploitasi terarah terhadap **NodeBuild Engine** pada `http://127.0.0.1:8046`.

---

### Langkah 1: Observasi Ketersediaan Layanan Target
1. Uji ketersediaan layanan melalui terminal menggunakan cURL:
   ```bash
   curl -s http://127.0.0.1:8046/health
   ```
   *Respons yang diharapkan:*
   ```json
   {"status":"UP","lab_id":"A03-03","port":8046,"flag_configured":true}
   ```
2. Buka peramban web dan navigasikan ke:
   `http://127.0.0.1:8046/`
3. Amati antarmuka operasional **NodeBuild Engine** dan perhatikan elemen kontrol yang disediakan.

---

### Langkah 2: Analisis & Pengujian Celah (Malicious Preinstall Script & CI/CD Secret Exfiltration)
1. Ujilah endpoint dengan mengirimkan parameter modifikasi:
   ```bash
   curl -s http://127.0.0.1:8046/api/pipeline/audit-hooks
   ```
2. Perhatikan respon yang dikembalikan oleh server.

> [!NOTE]
> [!NOTE]
> **Di Balik Layar: Apa yang terjadi di server?**  
> Sistem target menerima input `X-Pipeline-Agent: GitHub-Runner-01` dan memprosesnya tanpa melakukan validasi silang ke sesi otentikasi server. Akibatnya, server mengasumsikan hak akses yang diminta adalah sah.

---

### Langkah 3: Eksfiltrasi Bendera Autoritatif (CTF Flag)
1. Eksekusi pengujian untuk membuka flag eksploitasi:
   ```bash
   curl -s "http://127.0.0.1:8046/?solve=1"
   ```
   Atau panggil API exploit langsung:
   ```bash
   curl -s http://127.0.0.1:8046/api/exploit
   ```
2. **Hasil Respon Konfirmasi:**
   ```json
   {"success":true,"lab_id":"A03-03","flag":"ZITERA{pr31nst4ll_h00k_c1cd_s3cr3t_l34k_2026}"}
   ```
3. Salin bendera resmi:
   `ZITERA{pr31nst4ll_h00k_c1cd_s3cr3t_l34k_2026}`
4. Kirimkan (*submit*) bukti flag ke formulir tantangan Zitera Lab desktop untuk menyelesaikan modul ini!
