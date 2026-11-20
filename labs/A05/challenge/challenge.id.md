# Simulasi Uji Penetrasi: MegaStore Global SQL Injection

### MISI BRIEFING
Departemen E-Commerce **MegaStore Global Retail Logistics Inc.** mengundang Anda untuk mengaudit portal pencarian inventaris B2B mereka di `http://127.0.0.1:8015`.

Fitur pencarian perangkat keras diduga rentan terhadap manipulasi parameter SQL langsung tanpa menggunakan *prepared statements*, berpotensi mengekspos tabel rahasia sistem.

---

### RUANG LINGKUP & TARGET
- **Target URL:** `http://127.0.0.1:8015`
- **Tingkat Keparahan:** High (CVSS 8.5)
- **Vektor Kerentanan:** CWE-89 (Improper Neutralization of Special Elements used in an SQL Command)

---

### OBJEKTIF PENELUSURAN
1. Buktikan keberadaan SQL Injection pada form pencarian `?q=`.
2. Gunakan teknik UNION SELECT untuk mengekstrak data dari tabel terisolasi `vault_secrets`.
3. Dapatkan flag berformat `ZITERA{...}` yang tersimpan di kolom rahasia.
4. Kirimkan (*submit*) bukti flag ke Zitera Lab desktop untuk merekam skor Anda!

---

### FORMAT SUBMISSION
`ZITERA{...}`
