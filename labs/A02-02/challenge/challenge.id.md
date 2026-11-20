# Tantangan CTF Mandiri: Insecure CORS Misconfiguration & Origin Reflection

### 🎯 MISI BRIEFING
Kamu ditugaskan untuk melakukan penetration testing terotorisasi terhadap sistem **APINexus Gateway** yang beroperasi pada port `8043`.
Sistem ini mengimplementasikan fungsionalitas tingkat lanjut namun terindikasi memiliki kelemahan arsitektur pada modul **Insecure CORS Misconfiguration & Origin Reflection**.

---

### 🏢 RUANG LINGKUP & TARGET
- **Target URL:** `http://127.0.0.1:8043`
- **Kategori OWASP:** A02 — Security Misconfiguration
- **Tingkat Kesulitan:** Intermediate
- **Estimasi Pengerjaan:** 40 Menit

---

### 🚩 OBJEKTIF PENELUSURAN
1. Lakukan pemetaan endpoint pada target `APINexus Gateway`.
2. Identifikasi parameter atau header yang dapat dimanipulasi: `Origin: https://attacker-website.com`.
3. Buktikan dampak eksploitasi dan ambil token bendera validasi autoritatif.
4. Kirimkan (*submit*) flag berformat `ZITERA{...}` ke tab Challenge untuk menyelesaikan modul!

---

### 🔑 FORMAT FLAG
`ZITERA{c0r5_0r1g1n_r3fl3ct10n_cr3d3nt14l_2026}`
