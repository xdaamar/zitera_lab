# Misi Forensik: Audit Software Supply Chain PackNexus

### 🎯 MISI BRIEFING
Tim DevOps **PackNexus Inc.** mendeteksi anomali pada log pipeline build CI/CD otomatis (`http://127.0.0.1:8013`). Diduga salah satu pengembang eksternal menyisipkan pustaka telemetri yang belum diaudit dan tidak ditandatangani kunci GPG perusahaan.

Tugas Anda adalah melakukan audit mendalam terhadap artefak lockfile, mendeteksi skrip hook instalasi berbahaya, dan merekonstruksi token rahasia pipeline yang dicuri.

---

### 🏢 RUANG LINGKUP & TARGET
- **Target URL:** `http://127.0.0.1:8013`
- **Tingkat Keparahan:** High (CVSS 8.1)
- **Vektor Kerentanan:** CWE-1357 (Reliance on Uncontrolled Component), CWE-506 (Embedded Malicious Code)

---

### 🚩 OBJEKTIF PENELUSURAN
1. Akses manifest audit dependensi pada `/packages/audit/package_lock_audit.json`.
2. Temukan paket mencurigakan yang memiliki skrip `postinstall` berbahaya.
3. Ekstrak token penyusupan rahasia pipeline berformat `ZITERA{...}`.
4. Lakukan verifikasi di `/api/supply-chain/verify` dan kirimkan bukti flag ke Zitera Lab desktop!

---

### 🔑 FORMAT SUBMISSION
`ZITERA{...}`
