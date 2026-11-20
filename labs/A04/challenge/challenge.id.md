# Simulasi Audit: SafeKey Corporate Vault Weak Hashing

### MISI BRIEFING
Lembaga perbankan **SafeKey Technologies Inc.** meminta Anda melakukan uji penetrasi terhadap modul penyimpanan kredensial **SafeKey Corporate Vault** pada `http://127.0.0.1:8014`.

Modul ini diduga masih menyimpan jejak kredensial migrasi menggunakan algoritma MD5 tanpa garam kriptografis (*unsalted MD5*), memungkinkan penyerang merekonstruksi kata sandi Chief Information Security Officer (CISO).

---

### RUANG LINGKUP & TARGET
- **Target URL:** `http://127.0.0.1:8014`
- **Tingkat Keparahan:** High (CVSS 7.4)
- **Vektor Kerentanan:** CWE-328 (Use of Weak Hash), CWE-916 (Use of Password Hash With Insufficient Computational Effort)

---

### OBJEKTIF PENELUSURAN
1. Akses tabel migrasi kredensial di `/api/audit/hashes`.
2. Identifikasi dan pecahkan nilai hash MD5 akun `admin`.
3. Lakukan login ke portal `/login` untuk membuka Master Key Vault di `/vault`.
4. Ambil dan kirimkan flag berformat `ZITERA{...}` ke Zitera Lab desktop!

---

### FORMAT SUBMISSION
`ZITERA{...}`
