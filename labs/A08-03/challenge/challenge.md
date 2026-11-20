# Tantangan CTF Mandiri: Python Pickle & YAML Unsafe Object Deserialization

### MISI BRIEFING
Kamu ditugaskan untuk melakukan penetration testing terotorisasi terhadap sistem **PipelineSerializer** yang beroperasi pada port `8056`.
Sistem ini mengimplementasikan fungsionalitas tingkat lanjut namun terindikasi memiliki kelemahan arsitektur pada modul **Python Pickle & YAML Unsafe Object Deserialization**.

---

### RUANG LINGKUP & TARGET
- **Target URL:** `http://127.0.0.1:8056`
- **Kategori OWASP:** A08 — Software or Data Integrity Failures
- **Tingkat Kesulitan:** Advanced
- **Estimasi Pengerjaan:** 40 Menit

---

### OBJEKTIF PENELUSURAN
1. Lakukan pemetaan endpoint pada target `PipelineSerializer`.
2. Identifikasi parameter atau header yang dapat dimanipulasi: `Content-Type: application/x-python-pickle`.
3. Buktikan dampak eksploitasi dan ambil token bendera validasi autoritatif.
4. Kirimkan (*submit*) flag berformat `ZITERA{...}` ke tab Challenge untuk menyelesaikan modul!

---

### [KEY] FORMAT FLAG
`ZITERA{uns4f3_d3s3r14l1z4t10n_p1ckl3_2026}`
