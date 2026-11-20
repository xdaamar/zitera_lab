# Panduan Praktik Terbimbing: Investigasi Dependency Confusion & Internal Namespace Hijacking

Dalam panduan terarah ini, Anda akan mempraktikkan proses penemuan celah dan eksploitasi terarah terhadap **Enterprise Artifactory** pada `http://127.0.0.1:8045`.

---

### Langkah 1: Observasi Ketersediaan Layanan Target
1. Uji ketersediaan layanan melalui terminal menggunakan cURL:
   ```bash
   curl -s http://127.0.0.1:8045/health
   ```
   *Respons yang diharapkan:*
   ```json
   {"status":"UP","lab_id":"A03-02","port":8045,"flag_configured":true}
   ```
2. Buka peramban web dan navigasikan ke:
   `http://127.0.0.1:8045/`
3. Amati antarmuka operasional **Enterprise Artifactory** dan perhatikan elemen kontrol yang disediakan.

---

### Langkah 2: Analisis & Pengujian Celah (Dependency Confusion & Internal Namespace Hijacking)
1. Ujilah endpoint dengan mengirimkan parameter modifikasi:
   ```bash
   curl -s http://127.0.0.1:8045/api/packages/resolve?package=internal-analytics-core
   ```
2. Perhatikan respon yang dikembalikan oleh server.

> [!NOTE]
> **💡 Di Balik Layar: Apa yang terjadi di server?**  
> Sistem target menerima input `Content-Type: application/json` dan memprosesnya tanpa melakukan validasi silang ke sesi otentikasi server. Akibatnya, server mengasumsikan hak akses yang diminta adalah sah.

---

### Langkah 3: Eksfiltrasi Bendera Autoritatif (CTF Flag)
1. Eksekusi pengujian untuk membuka flag eksploitasi:
   ```bash
   curl -s "http://127.0.0.1:8045/?solve=1"
   ```
   Atau panggil API exploit langsung:
   ```bash
   curl -s http://127.0.0.1:8045/api/exploit
   ```
2. **Hasil Respon Konfirmasi:**
   ```json
   {"success":true,"lab_id":"A03-02","flag":"ZITERA{d3p_c0nfus10n_n4m3sp4c3_h1j4ck_2026}"}
   ```
3. Salin bendera resmi:
   `ZITERA{d3p_c0nfus10n_n4m3sp4c3_h1j4ck_2026}`
4. Kirimkan (*submit*) bukti flag ke formulir tantangan Zitera Lab desktop untuk menyelesaikan modul ini!
