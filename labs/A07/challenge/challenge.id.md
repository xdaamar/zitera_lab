# Simulasi Audit: NexHealth Clinical Authentication Bypass

### MISI BRIEFING
Rumah Sakit Regional **NexHealth Systems Inc.** menugaskan Anda mengaudit keamanan gerbang masuk rekam medis klinis mereka di `http://127.0.0.1:8017`.

Portal staf medis diduga tidak menerapkan proteksi *rate-limiting* maupun mekanisme *account lockout*, serta membiarkan akun Chief Medical Officer (`admin`) dilindungi oleh PIN 4-digit yang lemah.

---

### RUANG LINGKUP & TARGET
- **Target URL:** `http://127.0.0.1:8017`
- **Tingkat Keparahan:** High (CVSS 7.8)
- **Vektor Kerentanan:** CWE-307 (Improper Restriction of Excessive Authentication Attempts), CWE-521 (Weak Password Requirements)

---

### OBJEKTIF PENELUSURAN
1. Buktikan ketiadaan pembatasan percobaan login pada `/login` dan `/api/login`.
2. Lakukan pengujian kredensial terarah terhadap PIN 4-digit akun `admin`.
3. Dapatkan akses ke dashboard rekam medis klinis `/admin`.
4. Salin dan kirimkan flag autoritatif berformat `ZITERA{...}` ke Zitera Lab desktop!

---

### FORMAT SUBMISSION
`ZITERA{...}`
