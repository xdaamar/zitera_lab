# Tantangan CTF Mandiri: Dependency Confusion & Internal Namespace Hijacking

### MISI BRIEFING
Kamu ditugaskan untuk melakukan penetration testing terotorisasi terhadap sistem **Enterprise Artifactory** yang beroperasi pada port `8045`.
Sistem ini mengimplementasikan fungsionalitas tingkat lanjut namun terindikasi memiliki kelemahan arsitektur pada modul **Dependency Confusion & Internal Namespace Hijacking**.

---

### RUANG LINGKUP & TARGET
- **Target URL:** `http://127.0.0.1:8045`
- **Kategori OWASP:** A03 — Software Supply Chain Failures
- **Tingkat Kesulitan:** Intermediate
- **Estimasi Pengerjaan:** 40 Menit

---

### OBJEKTIF PENELUSURAN
1. Lakukan pemetaan endpoint pada target `Enterprise Artifactory`.
2. Identifikasi parameter atau header yang dapat dimanipulasi: `Content-Type: application/json`.
3. Buktikan dampak eksploitasi dan ambil token bendera validasi autoritatif.
4. Kirimkan (*submit*) flag berformat `ZITERA{...}` ke tab Challenge untuk menyelesaikan modul!

---

### [KEY] FORMAT FLAG
`ZITERA{d3p_c0nfus10n_n4m3sp4c3_h1j4ck_2026}`
