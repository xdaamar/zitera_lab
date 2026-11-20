# Tantangan CTF Mandiri: Full-Chain Blind SSRF to Cloud Metadata

### MISI BRIEFING
Kamu ditugaskan untuk melakukan penetration testing terotorisasi terhadap sistem **WebFetch Microservice** yang beroperasi pada port `8060`.
Sistem ini mengimplementasikan fungsionalitas tingkat lanjut namun terindikasi memiliki kelemahan arsitektur pada modul **Full-Chain Blind SSRF to Cloud Metadata**.

---

### RUANG LINGKUP & TARGET
- **Target URL:** `http://127.0.0.1:8060`
- **Kategori OWASP:** A10 — Mishandling of Exceptional Conditions
- **Tingkat Kesulitan:** Advanced
- **Estimasi Pengerjaan:** 40 Menit

---

### OBJEKTIF PENELUSURAN
1. Lakukan pemetaan endpoint pada target `WebFetch Microservice`.
2. Identifikasi parameter atau header yang dapat dimanipulasi: `Content-Type: application/json`.
3. Buktikan dampak eksploitasi dan ambil token bendera validasi autoritatif.
4. Kirimkan (*submit*) flag berformat `ZITERA{...}` ke tab Challenge untuk menyelesaikan modul!

---

### [KEY] FORMAT FLAG
`ZITERA{ssrf_cl0ud_m3t4d4t4_3xf1ltr4t10n_2026}`
