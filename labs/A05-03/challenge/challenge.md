# Tantangan CTF Mandiri: OS Command Injection via Unsanitized File Utilities

### 🎯 MISI BRIEFING
Kamu ditugaskan untuk melakukan penetration testing terotorisasi terhadap sistem **CloudPing Network Tool** yang beroperasi pada port `8050`.
Sistem ini mengimplementasikan fungsionalitas tingkat lanjut namun terindikasi memiliki kelemahan arsitektur pada modul **OS Command Injection via Unsanitized File Utilities**.

---

### 🏢 RUANG LINGKUP & TARGET
- **Target URL:** `http://127.0.0.1:8050`
- **Kategori OWASP:** A05 — Injection
- **Tingkat Kesulitan:** Advanced
- **Estimasi Pengerjaan:** 40 Menit

---

### 🚩 OBJEKTIF PENELUSURAN
1. Lakukan pemetaan endpoint pada target `CloudPing Network Tool`.
2. Identifikasi parameter atau header yang dapat dimanipulasi: `Content-Type: application/x-www-form-urlencoded`.
3. Buktikan dampak eksploitasi dan ambil token bendera validasi autoritatif.
4. Kirimkan (*submit*) flag berformat `ZITERA{...}` ke tab Challenge untuk menyelesaikan modul!

---

### 🔑 FORMAT FLAG
`ZITERA{c0mm4nd_1nj3ct10n_sh3ll_3x3c_2026}`
