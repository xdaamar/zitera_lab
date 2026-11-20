# Tantangan CTF Mandiri: MFA Bypass via Direct Endpoint Traversal

### 🎯 MISI BRIEFING
Kamu ditugaskan untuk melakukan penetration testing terotorisasi terhadap sistem **TwoFactor SecureNet** yang beroperasi pada port `8054`.
Sistem ini mengimplementasikan fungsionalitas tingkat lanjut namun terindikasi memiliki kelemahan arsitektur pada modul **MFA Bypass via Direct Endpoint Traversal**.

---

### 🏢 RUANG LINGKUP & TARGET
- **Target URL:** `http://127.0.0.1:8054`
- **Kategori OWASP:** A07 — Authentication Failures
- **Tingkat Kesulitan:** Advanced
- **Estimasi Pengerjaan:** 40 Menit

---

### 🚩 OBJEKTIF PENELUSURAN
1. Lakukan pemetaan endpoint pada target `TwoFactor SecureNet`.
2. Identifikasi parameter atau header yang dapat dimanipulasi: `Cookie: session_user=operator`.
3. Buktikan dampak eksploitasi dan ambil token bendera validasi autoritatif.
4. Kirimkan (*submit*) flag berformat `ZITERA{...}` ke tab Challenge untuk menyelesaikan modul!

---

### 🔑 FORMAT FLAG
`ZITERA{mf4_byp4ss_d1r3ct_r0ut3_2026}`
