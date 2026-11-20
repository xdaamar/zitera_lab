# Tantangan CTF Mandiri: SIEM Threshold Evasion via Slow-and-Low Scanning

### 🎯 MISI BRIEFING
Kamu ditugaskan untuk melakukan penetration testing terotorisasi terhadap sistem **StealthDetect Gateway** yang beroperasi pada port `8058`.
Sistem ini mengimplementasikan fungsionalitas tingkat lanjut namun terindikasi memiliki kelemahan arsitektur pada modul **SIEM Threshold Evasion via Slow-and-Low Scanning**.

---

### 🏢 RUANG LINGKUP & TARGET
- **Target URL:** `http://127.0.0.1:8058`
- **Kategori OWASP:** A09 — Security Logging & Alerting Failures
- **Tingkat Kesulitan:** Advanced
- **Estimasi Pengerjaan:** 40 Menit

---

### 🚩 OBJEKTIF PENELUSURAN
1. Lakukan pemetaan endpoint pada target `StealthDetect Gateway`.
2. Identifikasi parameter atau header yang dapat dimanipulasi: `X-Client-Probe: stealth`.
3. Buktikan dampak eksploitasi dan ambil token bendera validasi autoritatif.
4. Kirimkan (*submit*) flag berformat `ZITERA{...}` ke tab Challenge untuk menyelesaikan modul!

---

### 🔑 FORMAT FLAG
`ZITERA{s13m_thr3sh0ld_3v4s10n_st34lth_2026}`
