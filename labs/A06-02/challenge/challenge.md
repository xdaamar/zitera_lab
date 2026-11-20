# Tantangan CTF Mandiri: Concurrency Race Condition & Double Spending

### MISI BRIEFING
Kamu ditugaskan untuk melakukan penetration testing terotorisasi terhadap sistem **FlashSale Pay** yang beroperasi pada port `8051`.
Sistem ini mengimplementasikan fungsionalitas tingkat lanjut namun terindikasi memiliki kelemahan arsitektur pada modul **Concurrency Race Condition & Double Spending**.

---

### RUANG LINGKUP & TARGET
- **Target URL:** `http://127.0.0.1:8051`
- **Kategori OWASP:** A06 — Insecure Design
- **Tingkat Kesulitan:** Intermediate
- **Estimasi Pengerjaan:** 40 Menit

---

### OBJEKTIF PENELUSURAN
1. Lakukan pemetaan endpoint pada target `FlashSale Pay`.
2. Identifikasi parameter atau header yang dapat dimanipulasi: `Content-Type: application/json`.
3. Buktikan dampak eksploitasi dan ambil token bendera validasi autoritatif.
4. Kirimkan (*submit*) flag berformat `ZITERA{...}` ke tab Challenge untuk menyelesaikan modul!

---

### [KEY] FORMAT FLAG
`ZITERA{r4c3_c0nd1t10n_d0ubl3_sp3nd_2026}`
