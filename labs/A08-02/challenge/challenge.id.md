# Tantangan CTF Mandiri: Unsigned Client-Side State Tampering in LocalStorage

### MISI BRIEFING
Kamu ditugaskan untuk melakukan penetration testing terotorisasi terhadap sistem **CartSync Session** yang beroperasi pada port `8055`.
Sistem ini mengimplementasikan fungsionalitas tingkat lanjut namun terindikasi memiliki kelemahan arsitektur pada modul **Unsigned Client-Side State Tampering in LocalStorage**.

---

### RUANG LINGKUP & TARGET
- **Target URL:** `http://127.0.0.1:8055`
- **Kategori OWASP:** A08 — Software or Data Integrity Failures
- **Tingkat Kesulitan:** Intermediate
- **Estimasi Pengerjaan:** 40 Menit

---

### OBJEKTIF PENELUSURAN
1. Lakukan pemetaan endpoint pada target `CartSync Session`.
2. Identifikasi parameter atau header yang dapat dimanipulasi: `Content-Type: application/json`.
3. Buktikan dampak eksploitasi dan ambil token bendera validasi autoritatif.
4. Kirimkan (*submit*) flag berformat `ZITERA{...}` ke tab Challenge untuk menyelesaikan modul!

---

### [KEY] FORMAT FLAG
`ZITERA{cl13nt_st4t3_t4mp3r_l0c4lst0r4g3_2026}`
