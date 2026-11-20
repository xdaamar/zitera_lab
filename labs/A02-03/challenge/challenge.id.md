# Tantangan CTF Mandiri: Exposed Git Directory & Object Carving Extraction

### MISI BRIEFING
Kamu ditugaskan untuk melakukan penetration testing terotorisasi terhadap sistem **DevBucket Storage** yang beroperasi pada port `8044`.
Sistem ini mengimplementasikan fungsionalitas tingkat lanjut namun terindikasi memiliki kelemahan arsitektur pada modul **Exposed Git Directory & Object Carving Extraction**.

---

### RUANG LINGKUP & TARGET
- **Target URL:** `http://127.0.0.1:8044`
- **Kategori OWASP:** A02 — Security Misconfiguration
- **Tingkat Kesulitan:** Advanced
- **Estimasi Pengerjaan:** 40 Menit

---

### OBJEKTIF PENELUSURAN
1. Lakukan pemetaan endpoint pada target `DevBucket Storage`.
2. Identifikasi parameter atau header yang dapat dimanipulasi: `Accept: */*`.
3. Buktikan dampak eksploitasi dan ambil token bendera validasi autoritatif.
4. Kirimkan (*submit*) flag berformat `ZITERA{...}` ke tab Challenge untuk menyelesaikan modul!

---

### [KEY] FORMAT FLAG
`ZITERA{3xp0s3d_g1t_d1r_c0mm1t_l34k_2026}`
