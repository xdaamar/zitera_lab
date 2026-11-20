# Tantangan CTF Mandiri: Predictable Password Reset Token via Insecure PRNG

### 🎯 MISI BRIEFING
Kamu ditugaskan untuk melakukan penetration testing terotorisasi terhadap sistem **AuthKey Recovery** yang beroperasi pada port `8047`.
Sistem ini mengimplementasikan fungsionalitas tingkat lanjut namun terindikasi memiliki kelemahan arsitektur pada modul **Predictable Password Reset Token via Insecure PRNG**.

---

### 🏢 RUANG LINGKUP & TARGET
- **Target URL:** `http://127.0.0.1:8047`
- **Kategori OWASP:** A04 — Cryptographic Failures
- **Tingkat Kesulitan:** Intermediate
- **Estimasi Pengerjaan:** 40 Menit

---

### 🚩 OBJEKTIF PENELUSURAN
1. Lakukan pemetaan endpoint pada target `AuthKey Recovery`.
2. Identifikasi parameter atau header yang dapat dimanipulasi: `Content-Type: application/json`.
3. Buktikan dampak eksploitasi dan ambil token bendera validasi autoritatif.
4. Kirimkan (*submit*) flag berformat `ZITERA{...}` ke tab Challenge untuk menyelesaikan modul!

---

### 🔑 FORMAT FLAG
`ZITERA{prng_pr3d1ct4bl3_r3s3t_t0k3n_2026}`
