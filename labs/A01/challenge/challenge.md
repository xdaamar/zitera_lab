# Simulasi Bug Bounty: Operasi VaultPay Leak (Eksfiltrasi Sovereign Treasury Record)

### MISI BRIEFING
Perusahaan perbankan korporat **VaultPay Financial Technologies Inc.** mengundangmu dalam program *Private Bug Bounty Engagement*. Portal perbendaharaan dan kliring kawat likuiditas global mereka (`http://127.0.0.1:8011`) baru saja diluncurkan ke tahap pra-produksi.

Tim audit internal mencurigai bahwa meskipun halaman kontrol eksekutif telah diproteksi dengan verifikasi peran pengguna (*Role-Based Access Control*), mekanisme penarikan dokumen voucher transfer kawat di lapisan API tidak menerapkan validasi kepemilikan objek (*Object-Level Ownership Validation*).

---

### RUANG LINGKUP & TARGET
- **Target URL:** `http://127.0.0.1:8011`
- **Kredensial Pengujian (Scope):** `alice` / `password123` (Analis Junior - Divisi Likuiditas)
- **Tingkat Keparahan (Severity):** High / Critical (CVSS 8.6)
- **Estimasi Bounty:** $1,500 – $8,000 USD

---

### OBJEKTIF PENELUSURAN
1. Lakukan audit keamanan menggunakan akun sah `alice`.
2. Temukan dan eksfiltrasi voucher transfer bernilai tinggi berstatus **RESTRICTED** milik Controller Eksekutif (**Marcus Reid**).
3. Dapatkan segel validasi autoritatif berupa CTF Flag yang tersimpan di dalam catatan kepatuhan SOX-404 tersebut.
4. Kirimkan (*submit*) bukti flag ke Zitera Lab desktop untuk memvalidasi penyelesaian misi dan merekam skor ke live leaderboard!

---

### ATURAN PENGUJIAN (RULES OF ENGAGEMENT)
- Batasi pengujian hanya pada target sandbox lokal `127.0.0.1:8011`.
- Jangan melakukan serangan brute force terhadap password akun lain. Kerentanan murni terletak pada manipulasi parameter kontrol akses (*Parameter Tampering & IDOR*).

---

### FORMAT SUBMISSION
Kirimkan flag valid yang kamu temukan dengan format:
`ZITERA{...}`
