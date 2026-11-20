# Tantangan CTF Mandiri: Multi-Step Workflow Step Skipping

### 🎯 MISI BRIEFING
Kamu ditugaskan untuk melakukan penetration testing terotorisasi terhadap sistem **KYC Onboarding Portal** yang beroperasi pada port `8052`.
Sistem ini mengimplementasikan fungsionalitas tingkat lanjut namun terindikasi memiliki kelemahan arsitektur pada modul **Multi-Step Workflow Step Skipping**.

---

### 🏢 RUANG LINGKUP & TARGET
- **Target URL:** `http://127.0.0.1:8052`
- **Kategori OWASP:** A06 — Insecure Design
- **Tingkat Kesulitan:** Advanced
- **Estimasi Pengerjaan:** 40 Menit

---

### 🚩 OBJEKTIF PENELUSURAN
1. Lakukan pemetaan endpoint pada target `KYC Onboarding Portal`.
2. Identifikasi parameter atau header yang dapat dimanipulasi: `Content-Type: application/json`.
3. Buktikan dampak eksploitasi dan ambil token bendera validasi autoritatif.
4. Kirimkan (*submit*) flag berformat `ZITERA{...}` ke tab Challenge untuk menyelesaikan modul!

---

### 🔑 FORMAT FLAG
`ZITERA{w0rkfl0w_st3p_sk1pp1ng_kyc_2026}`
