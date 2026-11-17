# Simulasi Audit: FleetTrack Telematics Audit Trail Blind Spot

### 🎯 MISI BRIEFING
Tim Keamanan Cyber **FleetTrack Logistics Inc.** menugaskan Anda mengaudit ketahanan sistem telemetri pemantauan armada di `http://127.0.0.1:8019`.

Diduga terdapat kelemahan fatal pada sistem pencatatan log keamanan (*Security Logging Blind Spot*), di mana serangan tebakan kata sandi beruntun dan eskalasi wewenang administratif tidak menghasilkan peringatan maupun entri jejak audit.

---

### 🏢 RUANG LINGKUP & TARGET
- **Target URL:** `http://127.0.0.1:8019`
- **Tingkat Keparahan:** Medium / High (CVSS 7.3)
- **Vektor Kerentanan:** CWE-778 (Insufficient Logging), CWE-392 (Failure to Log Suspicious Events)

---

### 🚩 OBJEKTIF PENELUSURAN
1. Lakukan minimal 5 kali percobaan login gagal pada `/api/auth/login`.
2. Lakukan eskalasi peran pada `/api/admin/role_escalate` untuk akun `learner`.
3. Buktikan ketiadaan entri alarm pada `/api/audit/logs`.
4. Dapatkan dan kirimkan flag berformat `ZITERA{...}` ke Zitera Lab desktop!

---

### 🔑 FORMAT SUBMISSION
`ZITERA{...}`
