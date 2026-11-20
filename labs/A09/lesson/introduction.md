# A09: Security Logging & Alerting Failures — Kebutaan Log & Kegagalan Alarm Sistem

Security Logging & Monitoring Failures menempati peringkat ke-9 (**#9**) dalam daftar risiko **OWASP Top 10:2025**. Kategori ini menyoroti bahaya ketika sebuah sistem aplikasi gagal mencatat aktivitas keamanan kritis (*critical security events*) dan tidak memiliki sistem pendeteksian dini (*real-time alerting*), sehingga penyerang dapat melakukan eksplorasi, brute-force, dan eskalasi hak akses secara bebas tanpa pernah disadari oleh tim operasional keamanan (*Security Operations Center / SOC*).

---

### 1. Kenapa Ini Sangat Berbahaya? (Business & Financial Impact)
Rata-rata waktu yang dibutuhkan perusahaan di seluruh dunia untuk mendeteksi bahwa sistem mereka telah dibobol (*Mean Time to Detect / MTTD*) adalah **lebih dari 200 hari**:

- **Penyerang Berdiam di Dalam Jaringan Berbulan-bulan (*Attacker Dwell Time*)**:
  Tanpa adanya pencatatan log percobaan login gagal dan eskalasi hak akses, penyerang leluasa memetakan arsitektur database, memasang pintu belakang cadangan, dan mencuri data rahasia korporasi sedikit demi sedikit (*slow data exfiltration*) tanpa memicu alarm keamanan.
- **Denda Regulasi Akibat Ketiadaan Audit Trail**:
  Standar kepatuhan perbankan internasional (**PCI-DSS Requirement 10**), regulasi kesehatan (**HIPAA**), dan **UU PDP No. 27/2022** mewajibkan penyimpanan jejak audit keamanan (*audit trail*) yang tidak dapat diubah selama minimal 1 tahun. Kegagalan mencatat insiden keamanan berisiko tuntutan pidana kelalaian manajemen dan denda finansial masif.

---

### 2. Bagaimana Cara Mencari & Mengujinya? (Recon & Vulnerability Indicators)
Metodologi pengujian integritas telemetri keamanan:

1. **Apa yang Dicari (What to Search)**:
   - Aliran log audit keamanan: `/api/audit/logs`, `/audit`, konsol SIEM.
   - Perilaku sistem terhadap aksi berisiko tinggi: rentetan login gagal (*credential brute-force*), eskalasi peran pengguna (*role escalation*), dan perubahan hak akses admin.
2. **Apa Temuan yang Menandakan Celah (Vulnerability Indicators)**:
   - Mengirimkan 5 kali percobaan login gagal berturut-turut ke `/api/auth/login` menghasilkan **0 entri log** pada tabel audit.
   - Panggilan eskalasi wewenang eksekutif ke `/api/admin/role_escalate` berhasil dijalankan secara diam-diam tanpa memicu satupun notifikasi alert keamanan (*audit_recorded: false*).
3. **Eksekusi Eksploitasi Terukur (Strategic Execution)**:
   - Kirimkan 5 percobaan login gagal berturut-turut ke endpoint autentikasi.
   - Lakukan panggilan eskalasi peran akun `learner` menjadi `super_admin`.
   - Buktikan bahwa seluruh aksi penyerangan tidak terdeteksi oleh SOC dan ekstraksi flag autoritatif `ZITERA{53cur1ty_l0gg1ng_4l3rt1ng_bl1nd5p0t}`.

---

### 3. Bagaimana Cara Memperbaikinya? (Secure Coding & Remediation)
Strategi pembangunan telemetri pertahanan mutlak:
- **Catat Seluruh Peristiwa Keamanan Kritis (Audit Event Logging)**: Setiap kegagalan login, percobaan bypass otorisasi, dan perubahan peran wewenang wajib dicatat dengan format terstruktur (JSON / CEF) mencakup timestamp UTC, IP sumber, username, dan jenis event.
- **Hubungkan ke Sistem SIEM & Real-Time Alerting**: Sambungkan aliran log ke platform monitoring (Splunk, Elastic SIEM, Wazuh) dan buat aturan alarm otomatis (misal: $> 5$ kegagalan login dalam 1 menit langsung memicu notifikasi darurat ke tim SOC).
- **Lindungi Integritas Log (*Append-Only & Write-Once*)**: Simpan log di server terisolasi dengan akses baca-saja (*read-only*) agar penyerang tidak dapat menghapus jejak kejahatan mereka setelah berhasil masuk.

---

### Gambaran Sistem yang Dihadapi (Target Commercial Architecture)
Dalam laboratorium ini, kamu mengaudit sistem pelacakan armada kendaraan komersial: **FleetTrack Logistics / Zitera SOC Telemetry Console (Commercial Telematics & IoT Fleet Hub)** yang beroperasi pada port `8019`.

Komponen arsitektur:
- **Operations Audit Trail**: Endpoint `GET /api/audit/logs` yang menampilkan catatan operasional armada harian.
- **Authentication Service**: Endpoint `POST /api/auth/login` yang memproses login operator.
- **Privilege Escalation Route**: Endpoint `POST /api/admin/role_escalate` yang mengubah wewenang pengguna.
- **Titik Buta Telemetri**: Sistem backend sama sekali tidak memiliki kode pencatatan log pada kegagalan autentikasi dan eskalasi wewenang.
