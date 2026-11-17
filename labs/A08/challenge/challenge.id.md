# Simulasi Audit: DataFlow Analytics Unsigned Configuration Injection

### 🎯 MISI BRIEFING
Tim Arsitektur Big Data **DataFlow Analytics Inc.** meminta Anda melakukan pengujian penetrasi terhadap mesin pemrosesan data mereka di `http://127.0.0.1:8018`.

Endpoint pembaruan konfigurasi kluster diduga menerima paket JSON secara mentah tanpa memverifikasi tanda tangan digital otoritas sertifikasi, memungkinkan pemuatan konfigurasi ilegal ke dalam memori eksekusi.

---

### 🏢 RUANG LINGKUP & TARGET
- **Target URL:** `http://127.0.0.1:8018`
- **Tingkat Keparahan:** High (CVSS 8.4)
- **Vektor Kerentanan:** CWE-494 (Download of Code Without Integrity Check), CWE-345 (Insufficient Verification of Data Authenticity)

---

### 🚩 OBJEKTIF PENELUSURAN
1. Periksa konfigurasi aktif kluster di `/api/config`.
2. Kirimkan paket pembaruan yang dimodifikasi tanpa tanda tangan valid ke `/api/deploy_update`.
3. Buktikan keberhasilan pemuatan konfigurasi berbahaya ke memori kluster.
4. Dapatkan dan kirimkan flag autoritatif berformat `ZITERA{...}` ke Zitera Lab desktop!

---

### 🔑 FORMAT SUBMISSION
`ZITERA{...}`
