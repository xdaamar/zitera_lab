# Tantangan CTF Mandiri: Blind Boolean & Time-Based SQL Injection

### 🎯 MISI BRIEFING
Kamu ditugaskan untuk melakukan penetration testing terotorisasi terhadap sistem **CustPortal Database** yang beroperasi pada port `8049`.
Sistem ini mengimplementasikan fungsionalitas tingkat lanjut namun terindikasi memiliki kelemahan arsitektur pada modul **Blind Boolean & Time-Based SQL Injection**.

---

### 🏢 RUANG LINGKUP & TARGET
- **Target URL:** `http://127.0.0.1:8049`
- **Kategori OWASP:** A05 — Injection
- **Tingkat Kesulitan:** Intermediate
- **Estimasi Pengerjaan:** 40 Menit

---

### 🚩 OBJEKTIF PENELUSURAN
1. Lakukan pemetaan endpoint pada target `CustPortal Database`.
2. Identifikasi parameter atau header yang dapat dimanipulasi: `Accept: application/json`.
3. Buktikan dampak eksploitasi dan ambil token bendera validasi autoritatif.
4. Kirimkan (*submit*) flag berformat `ZITERA{...}` ke tab Challenge untuk menyelesaikan modul!

---

### 🔑 FORMAT FLAG
`ZITERA{bl1nd_b00l34n_t1m3_sq1_1nj3ct_2026}`
