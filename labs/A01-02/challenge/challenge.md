# Tantangan CTF Mandiri: Vertical Privilege Escalation & Header Role Manipulation

### MISI BRIEFING
Kamu ditugaskan untuk melakukan penetration testing terotorisasi terhadap sistem **OpsAdmin Portal** yang beroperasi pada port `8041`.
Sistem ini mengimplementasikan fungsionalitas tingkat lanjut namun terindikasi memiliki kelemahan arsitektur pada modul **Vertical Privilege Escalation & Header Role Manipulation**.

---

### RUANG LINGKUP & TARGET
- **Target URL:** `http://127.0.0.1:8041`
- **Kategori OWASP:** A01 — Broken Access Control
- **Tingkat Kesulitan:** Intermediate
- **Estimasi Pengerjaan:** 40 Menit

---

### OBJEKTIF PENELUSURAN
1. Lakukan pemetaan endpoint pada target `OpsAdmin Portal`.
2. Identifikasi parameter atau header yang dapat dimanipulasi: `X-User-Role: admin`.
3. Buktikan dampak eksploitasi dan ambil token bendera validasi autoritatif.
4. Kirimkan (*submit*) flag berformat `ZITERA{...}` ke tab Challenge untuk menyelesaikan modul!

---

### [KEY] FORMAT FLAG
`ZITERA{v3rt1c4l_pr1v_3sc4l4t10n_h34d3r_2026}`
