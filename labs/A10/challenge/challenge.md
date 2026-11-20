# Simulasi Audit: TemplateCloud Studio Fail-Open Exception Handling

### 🎯 MISI BRIEFING
Tim Arsitektur SaaS **TemplateCloud Studio Inc.** meminta Anda melakukan audit keamanan terhadap gerbang autorisasi perender dokumen di `http://127.0.0.1:8020`.

Diduga terdapat kelemahan penanganan kondisi luar biasa (*Exceptional Condition Mishandling*), di mana kegagalan atau timeout pada layanan hulu menyebabkan sistem secara keliru mengizinkan transaksi (*Fail-Open Authorization*).

---

### 🏢 RUANG LINGKUP & TARGET
- **Target URL:** `http://127.0.0.1:8020`
- **Tingkat Keparahan:** High (CVSS 7.9)
- **Vektor Kerentanan:** CWE-755 (Improper Handling of Exceptional Conditions), CWE-636 (Not Failing Securely - 'Failing Open')

---

### 🚩 OBJEKTIF PENELUSURAN
1. Kirimkan payload yang memicu exception hulu pada `/api/gate/verify`.
2. Amati terjadinya kondisi fail-open yang memberikan izin otorisasi.
3. Dapatkan dan kirimkan flag autoritatif berformat `ZITERA{...}` ke Zitera Lab desktop!

---

### 🔑 FORMAT SUBMISSION
`ZITERA{...}`
