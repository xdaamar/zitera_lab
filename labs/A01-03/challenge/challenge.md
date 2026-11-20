# Tantangan CTF Mandiri: Multi-Tenant Boundary Escape & Insecure Direct Object Deletion

### MISI BRIEFING
Kamu ditugaskan untuk melakukan penetration testing terotorisasi terhadap sistem **TenantCloud Workspaces** yang beroperasi pada port `8042`.
Sistem ini mengimplementasikan fungsionalitas tingkat lanjut namun terindikasi memiliki kelemahan arsitektur pada modul **Multi-Tenant Boundary Escape & Insecure Direct Object Deletion**.

---

### RUANG LINGKUP & TARGET
- **Target URL:** `http://127.0.0.1:8042`
- **Kategori OWASP:** A01 — Broken Access Control
- **Tingkat Kesulitan:** Advanced
- **Estimasi Pengerjaan:** 40 Menit

---

### OBJEKTIF PENELUSURAN
1. Lakukan pemetaan endpoint pada target `TenantCloud Workspaces`.
2. Identifikasi parameter atau header yang dapat dimanipulasi: `Authorization: Bearer <tenant_a_token>`.
3. Buktikan dampak eksploitasi dan ambil token bendera validasi autoritatif.
4. Kirimkan (*submit*) flag berformat `ZITERA{...}` ke tab Challenge untuk menyelesaikan modul!

---

### [KEY] FORMAT FLAG
`ZITERA{mult1_t3n4nt_3sc4p3_d3l3t3_b0l4_2026}`
