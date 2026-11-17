# Simulasi Audit Keamanan: CloudCore OpsGateway Misconfiguration

### 🎯 MISI BRIEFING
Perusahaan cloud infrastructure **CloudCore Technologies Inc.** menugaskan Anda untuk mengaudit gerbang operasional kluster mereka: **CloudCore OpsGateway** yang berjalan pada `http://127.0.0.1:8012`.

Tim developer diduga meninggalkan mode debug aktif, membuka direktori cadangan otomatis tanpa proteksi autentikasi, serta membiarkan akun pengelola menggunakan kredensial bawaan pabrik.

---

### 🏢 RUANG LINGKUP & TARGET
- **Target URL:** `http://127.0.0.1:8012`
- **Tingkat Keparahan (Severity):** High (CVSS 7.5)
- **Vektor Kerentanan:** CWE-16 (Configuration), CWE-548 (Directory Listing), CWE-1188 (Default Insecure Credentials)

---

### 🚩 OBJEKTIF PENELUSURAN
1. Lakukan audit pengintaian pada endpoint diagnostik publik `/debug/vars`.
2. Eksploitasi indeks direktori terbuka pada `/backups/` untuk mengunduh arsip cadangan konfigurasi sistem.
3. Analisis isi file cadangan untuk mengekstrak token autoritatif `admin_system_key`.
4. Kirimkan (*submit*) flag berformat `ZITERA{...}` ke Zitera Lab desktop untuk memvalidasi penyelesaian misi!

---

### ⚠️ ATURAN PENGUJIAN
- Fokuskan pengujian pada service lokal `127.0.0.1:8012`.
- Hindari serangan denial-of-service (DoS).

---

### 🔑 FORMAT SUBMISSION
`ZITERA{...}`
