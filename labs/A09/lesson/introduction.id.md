# A09: Security Logging & Alerting Failures — Kebutaan Telemetri & Insiden Senyap

Security Logging and Alerting Failures menempati peringkat ke-9 (**#9**) dalam daftar risiko web **OWASP Top 10:2025** (sebelumnya dikenal sebagai *Security Logging and Monitoring Failures*). Kerentanan ini terjadi ketika sistem tidak mencatat kejadian keamanan kritis (*security-relevant events*), tidak menghasilkan peringatan waktu-nyata (*real-time alerting*), atau memiliki jejak audit (*audit trail*) yang tidak memadai, sehingga aktivitas musuh berlangsung tanpa terdeteksi dalam waktu yang lama.

---

### 1. Alasan Kuat / Mengapa Ini Sangat Penting (Why It Matters)
Berdasarkan laporan tahunan *IBM Cost of a Data Breach Report*, rata-rata waktu yang dibutuhkan organisasi untuk mendeteksi penyusupan siber (*dwell time*) mencapai **lebih dari 200 hari**:

- **Bahaya Insiden Senyap (*Silent Breaches*)**:
  - Penyerang yang menyusup ke sistem sering kali menghabiskan waktu berbulan-bulan menjelajahi jaringan internal, mencuri data secara bertahap, dan memanipulasi catatan keuangan tanpa ada alarm yang berbunyi di Security Operations Center (SOC).
  - Ketika investigasi forensik akhirnya dilakukan, penyelidik tidak dapat merekonstruksi apa yang terjadi karena log akses tidak mencatat kegagalan login, eskalasi wewenang, atau perubahan izin administrator.
- **Kepatuhan Regulasi & Tanggung Jawab Hukum**:
  Standar kepatuhan internasional (seperti ISO/IEC 27001, SOC 2, dan UU PDP) mewajibkan penyimpanan jejak audit yang tidak dapat diubah (*immutable audit logs*) untuk setiap peristiwa autentikasi dan eskalasi hak istimewa. Kelalaian ini berakibat denda besar dan gugatan kelalaian berat (*gross negligence*).

---

### 2. Gambaran Sistem yang Dihadapi (Target Commercial Architecture)
Dalam praktikum ini, Anda mengaudit pusat komando telematika komersial: **FleetTrack Logistics (Commercial Telematics & IoT Fleet Management)** yang beroperasi pada port `8019`.

Arsitektur sistem FleetTrack Logistics meliputi:
- **Telematics Command Console**: Memonitor status 340 kendaraan armada komersial (truk kontainer, van logistik) dan melacak koordinat GPS.
- **Audit Log Stream (`/api/audit/logs`)**: Endpoint yang menampilkan aliran catatan aktivitas sistem.
- **Dispatcher Authentication Gateway (`/api/auth/login`)**:
  - **Cacat Logging 1**: Percobaan login yang gagal dengan password salah sama sekali **tidak dicatat** ke dalam stream audit dan tidak memicu peringatan serangan brute force.
- **Role Escalation API (`/api/admin/role_escalate`)**:
  - **Cacat Logging 2**: Endpoint yang mengesahkan kenaikan hak akses pengguna menjadi `super_admin` berlangsung secara diam-diam (*silent escalation*) tanpa menghasilkan satu pun baris log audit keamanan!

---

### 3. Sasaran, Temuan & Eksekusi (Search, Find, and Execute)
Metodologi pembuktian kegagalan logging dan telemetri:

1. **Apa yang Dicari (What to Search)**:
   - Endpoint autentikasi dan otorisasi hak istimewa: `/api/auth/login`, `/api/admin/role_escalate`.
   - Aliran log keamanan di `/api/audit/logs`.
2. **Apa Temuan yang Menandakan Celah (Vulnerability Indicators)**:
   - Melakukan berkali-kali percobaan login gagal pada `/api/auth/login` tidak menambah jumlah log pada `/api/audit/logs` dan `alerts_fired` tetap bernilai `0`.
   - Melakukan eskalasi peran pada `/api/admin/role_escalate` berhasil memberikan wewenang administratif baru tanpa meninggalkan jejak audit (*audit_recorded: false*).
3. **Eksekusi Eksploitasi Terukur (Strategic Execution)**:
   - Lakukan minimal 5 kali percobaan login gagal dengan password acak untuk mendemonstrasikan serangan tebakan kredensial tanpa alarm.
   - Panggil endpoint eskalasi wewenang diam-diam `/api/admin/role_escalate` untuk akun `learner`.
   - Ambil bukti flag autoritatif berformat `ZITERA{...}` yang dikembalikan sistem atas keberhasilan membuktikan kebutaan telemetri SOC!

---

### 4. Pondasi Kuat: Tiga Pilar Audit Log Keamanan
Sistem audit yang memenuhi standar industri wajib mencakup:
1. **Peristiwa Kritis yang Wajib Dicatat**:
   - Setiap kegagalan dan keberhasilan login.
   - Setiap eskalasi hak akses (*privilege changes*).
   - Setiap akses ke data sensitif atau perubahan konfigurasi keamanan.
2. **Konteks Lengkap Catatan Log**:
   - Waktu dengan presisi UTC (`timestamp`).
   - Alamat IP sumber dan identitas pengguna pemrakarsa (`actor`).
   - Hasil tindakan (`outcome: SUCCESS / FAILED`).
3. **Penyimpanan Terpusat & Anti-Tamper**:
   - Log harus segera dialirkan ke sistem SIEM eksternal (Splunk, Elastic, Sentinel) dengan perizinan *Append-Only* agar tidak dapat dihapus oleh penyerang yang berhasil masuk ke server.
