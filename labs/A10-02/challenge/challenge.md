# Tantangan CTF Mandiri: Fail-Open Exception Handling in Auth Microservice

### MISI BRIEFING
Kamu ditugaskan untuk melakukan penetration testing terotorisasi terhadap sistem **GatewayGuard Failover** yang beroperasi pada port `8059`.
Sistem ini mengimplementasikan fungsionalitas tingkat lanjut namun terindikasi memiliki kelemahan arsitektur pada modul **Fail-Open Exception Handling in Auth Microservice**.

---

### RUANG LINGKUP & TARGET
- **Target URL:** `http://127.0.0.1:8059`
- **Kategori OWASP:** A10 — Mishandling of Exceptional Conditions
- **Tingkat Kesulitan:** Intermediate
- **Estimasi Pengerjaan:** 40 Menit

---

### OBJEKTIF PENELUSURAN
1. Lakukan pemetaan endpoint pada target `GatewayGuard Failover`.
2. Identifikasi parameter atau header yang dapat dimanipulasi: `X-Simulate-Error: TIMEOUT`.
3. Buktikan dampak eksploitasi dan ambil token bendera validasi autoritatif.
4. Kirimkan (*submit*) flag berformat `ZITERA{...}` ke tab Challenge untuk menyelesaikan modul!

---

### [KEY] FORMAT FLAG
`ZITERA{f41l_0p3n_m1cr0s3rv1c3_byp4ss_2026}`
