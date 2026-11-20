# Tantangan CTF Mandiri: CRLF Log Injection & Log Forgery

### 🎯 MISI BRIEFING
Kamu ditugaskan untuk melakukan penetration testing terotorisasi terhadap sistem **AuditSys Central** yang beroperasi pada port `8057`.
Sistem ini mengimplementasikan fungsionalitas tingkat lanjut namun terindikasi memiliki kelemahan arsitektur pada modul **CRLF Log Injection & Log Forgery**.

---

### 🏢 RUANG LINGKUP & TARGET
- **Target URL:** `http://127.0.0.1:8057`
- **Kategori OWASP:** A09 — Security Logging & Alerting Failures
- **Tingkat Kesulitan:** Intermediate
- **Estimasi Pengerjaan:** 40 Menit

---

### 🚩 OBJEKTIF PENELUSURAN
1. Lakukan pemetaan endpoint pada target `AuditSys Central`.
2. Identifikasi parameter atau header yang dapat dimanipulasi: `Content-Type: application/json`.
3. Buktikan dampak eksploitasi dan ambil token bendera validasi autoritatif.
4. Kirimkan (*submit*) flag berformat `ZITERA{...}` ke tab Challenge untuk menyelesaikan modul!

---

### 🔑 FORMAT FLAG
`ZITERA{cr1f_l0g_1nj3ct10n_f0rg3ry_2026}`
