# Tantangan CTF Mandiri: OAuth 2.0 Account Linking Hijacking via Missing State

### MISI BRIEFING
Kamu ditugaskan untuk melakukan penetration testing terotorisasi terhadap sistem **SocialConnect Auth** yang beroperasi pada port `8053`.
Sistem ini mengimplementasikan fungsionalitas tingkat lanjut namun terindikasi memiliki kelemahan arsitektur pada modul **OAuth 2.0 Account Linking Hijacking via Missing State**.

---

### RUANG LINGKUP & TARGET
- **Target URL:** `http://127.0.0.1:8053`
- **Kategori OWASP:** A07 — Authentication Failures
- **Tingkat Kesulitan:** Intermediate
- **Estimasi Pengerjaan:** 40 Menit

---

### OBJEKTIF PENELUSURAN
1. Lakukan pemetaan endpoint pada target `SocialConnect Auth`.
2. Identifikasi parameter atau header yang dapat dimanipulasi: `Accept: text/html`.
3. Buktikan dampak eksploitasi dan ambil token bendera validasi autoritatif.
4. Kirimkan (*submit*) flag berformat `ZITERA{...}` ke tab Challenge untuk menyelesaikan modul!

---

### [KEY] FORMAT FLAG
`ZITERA{04uth_m1ss1ng_st4t3_4cc0unt_l1nk_2026}`
