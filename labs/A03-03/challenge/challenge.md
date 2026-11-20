# Tantangan CTF Mandiri: Malicious Preinstall Script & CI/CD Secret Exfiltration

### 🎯 MISI BRIEFING
Kamu ditugaskan untuk melakukan penetration testing terotorisasi terhadap sistem **NodeBuild Engine** yang beroperasi pada port `8046`.
Sistem ini mengimplementasikan fungsionalitas tingkat lanjut namun terindikasi memiliki kelemahan arsitektur pada modul **Malicious Preinstall Script & CI/CD Secret Exfiltration**.

---

### 🏢 RUANG LINGKUP & TARGET
- **Target URL:** `http://127.0.0.1:8046`
- **Kategori OWASP:** A03 — Software Supply Chain Failures
- **Tingkat Kesulitan:** Advanced
- **Estimasi Pengerjaan:** 40 Menit

---

### 🚩 OBJEKTIF PENELUSURAN
1. Lakukan pemetaan endpoint pada target `NodeBuild Engine`.
2. Identifikasi parameter atau header yang dapat dimanipulasi: `X-Pipeline-Agent: GitHub-Runner-01`.
3. Buktikan dampak eksploitasi dan ambil token bendera validasi autoritatif.
4. Kirimkan (*submit*) flag berformat `ZITERA{...}` ke tab Challenge untuk menyelesaikan modul!

---

### 🔑 FORMAT FLAG
`ZITERA{pr31nst4ll_h00k_c1cd_s3cr3t_l34k_2026}`
