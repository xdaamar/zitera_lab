# A03: Software Supply Chain Failures — Ancaman Senyap Rantai Pasok

Software Supply Chain Failures menempati peringkat ke-3 (**#3**) dalam daftar risiko **OWASP Top 10:2025**. Kerentanan ini menyoroti bahaya ketika perangkat lunak enterprise disusupi bukan karena kesalahan penulisan kode internal oleh pengembang, melainkan akibat kompromi pada ekosistem rantai pasok hulu (*upstream third-party dependencies, package registries, and CI/CD pipelines*).

---

### 🔴 1. Kenapa Ini Sangat Berbahaya? (Business & Financial Impact)
Dalam rekayasa perangkat lunak modern, lebih dari 85% basis kode aplikasi terdiri dari pustaka pihak ketiga (npm, PyPI, Cargo, NuGet). Ketika satu dependensi dipercaya disusupi, ribuan korporasi yang mengimpor pustaka tersebut otomatis terinfeksi:

- **Dampak Berantai Skala Global (*Massive Blast Radius*)**:
  - **Insiden SolarWinds Orion (2020)**: Penyerang menyusupkan trojan ke dalam siklus kompilasi build resmi sebelum rilis ditandatangani, menginfeksi lebih dari 18.000 lembaga pemerintah dan perusahaan Fortune 500 dunia.
  - **Backdoor xz-utils (CVE-2024-3094)**: Penyerang menyisipkan muatan bertahap pada pustaka kompresi Linux guna membajak autentikasi SSH daemon global.
  - **Kasus `event-stream` (npm)**: Pustaka populer yang diunduh jutaan kali per minggu dialihkan kepemilikannya ke akun peretas, lalu disisipi skrip pencuri dompet mata uang kripto.
- **Kerugian Finansial & Hukum**:
  Kegagalan verifikasi rantai pasok dapat memicu kebocoran token repositori internal, kunci API cloud AWS/GCP produksi, dan hilangnya hak operasional perusahaan akibat pelanggaran standar keamanan seperti **NIST SP 800-161** dan **UU PDP No. 27/2022**.

---

### 🔍 2. Bagaimana Cara Mencari & Mengujinya? (Recon & Vulnerability Indicators)
Metodologi audit rantai pasok profesional:

1. **Apa yang Dicari (What to Search)**:
   - File manifest dan lockfile: `package.json`, `package-lock.json`, `Cargo.lock`.
   - Dependensi tanpa tanda tangan GPG/Sigstore, berasal dari URL tidak terverifikasi (*unpinned registry source*).
   - Hook siklus hidup paket yang mencurigakan: `postinstall`, `preinstall`, skrip eksternal seperti `send_env.js`.
2. **Apa Temuan yang Menandakan Celah (Vulnerability Indicators)**:
   - Manifest `package_lock_audit.json` memuat paket asing (`apex-internal-telemetry`) yang berstatus **UNTRUSTED / UNPINNED**.
   - Ditemukan metadata `postinstall_script` yang secara diam-diam mencuri variabel lingkungan dan mencatat `exfiltrated_pipeline_token`.
3. **Eksekusi Eksploitasi Terukur (Strategic Execution)**:
   - Ekstrak token rahasia pipeline berformat `ZITERA{5upply_ch41n_p0150n1ng_d3p_2026}` dari properti artefak yang disusupi.
   - Kirimkan token ke terminal verifikasi `/api/supply-chain/verify` untuk membuktikan dampak kompromi dan memvalidasi penyelesaian audit.

---

### 🛡️ 3. Bagaimana Cara Memperbaikinya? (Secure Coding & Remediation)
Strategi pertahanan rantai pasok mutlak:
- **Kunci Versi dan Hash Eksplisit (Lockfile & Subresource Integrity)**: Selalu gunakan lockfile dan pastikan integrity hash (`sha512-...`) diverifikasi pada setiap proses build.
- **Matikan Eksekusi Skrip Siklus Hidup Otomatis**: Gunakan flag `--ignore-scripts` saat menjalankan instalasi dependensi (`npm install --ignore-scripts`).
- **Terapkan SBOM & Automated Dependency Scanning**: Gunakan Software Bill of Materials (SBOM) dan alat pemindai kerentanan otomatis (misal: OWASP Dependency-Check, Snyk, Trivy).
- **Gunakan Registri Privat dengan Kebijakan Quarantine**: Pustaka pihak ketiga wajib melalui kurasi dan pengujian keamanan sebelum diizinkan masuk ke repositori internal korporasi.

---

### Gambaran Sistem yang Dihadapi (Target Commercial Architecture)
Dalam praktikum ini, kamu mengaudit infrastruktur build enterprise: **PackNexus Supply Hub (B2B Package Registry & Build Pipeline Hub)** yang beroperasi pada port `8013`.

Komponen arsitektur:
- **Private Artifact Registry**: Menyimpan dan mendistribusikan pustaka software terverifikasi serta mirror upstream.
- **Lockfile & Build Auditor**: Mempublikasikan manifest hasil audit otomatis dependensi pada endpoint `/packages/audit/package_lock_audit.json`.
- **Automated Pipeline Worker**: Lingkungan build otomatis yang mengeksekusi skrip instalasi paket (`postinstall_script`).
- **Pipeline Attestation Gate**: Endpoint verifikasi `/api/supply-chain/verify` untuk mengesahkan integritas rahasia pipeline.
