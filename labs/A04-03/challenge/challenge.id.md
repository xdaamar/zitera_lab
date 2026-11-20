# Tantangan CTF Mandiri: JWT Algorithm None & HMAC Weak Key Cracking

### 🎯 MISI BRIEFING
Kamu ditugaskan untuk melakukan penetration testing terotorisasi terhadap sistem **TokenShield SSO** yang beroperasi pada port `8048`.
Sistem ini mengimplementasikan fungsionalitas tingkat lanjut namun terindikasi memiliki kelemahan arsitektur pada modul **JWT Algorithm None & HMAC Weak Key Cracking**.

---

### 🏢 RUANG LINGKUP & TARGET
- **Target URL:** `http://127.0.0.1:8048`
- **Kategori OWASP:** A04 — Cryptographic Failures
- **Tingkat Kesulitan:** Advanced
- **Estimasi Pengerjaan:** 40 Menit

---

### 🚩 OBJEKTIF PENELUSURAN
1. Lakukan pemetaan endpoint pada target `TokenShield SSO`.
2. Identifikasi parameter atau header yang dapat dimanipulasi: `Authorization: Bearer <tampered_jwt>`.
3. Buktikan dampak eksploitasi dan ambil token bendera validasi autoritatif.
4. Kirimkan (*submit*) flag berformat `ZITERA{...}` ke tab Challenge untuk menyelesaikan modul!

---

### 🔑 FORMAT FLAG
`ZITERA{jwt_n0n3_4lg0r1thm_f0rg3ry_2026}`
