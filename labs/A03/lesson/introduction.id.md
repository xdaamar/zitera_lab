# A03: Software Supply Chain Failures — Ancaman Senyap Rantai Pasok

Software Supply Chain Failures menempati peringkat ke-3 (**#3**) dalam daftar risiko **OWASP Top 10:2025**. Kerentanan ini menyoroti bahaya ketika perangkat lunak enterprise disusupi bukan karena kesalahan penulisan kode internal oleh pengembang, melainkan akibat kompromi pada ekosistem rantai pasok hulu (*upstream third-party dependencies, package registries, and CI/CD pipelines*).

---

### 1. Alasan Kuat / Mengapa Ini Sangat Penting (Why It Matters)
Dalam rekayasa perangkat lunak modern, lebih dari 85% basis kode aplikasi terdiri dari pustaka pihak ketiga (npm, PyPI, Cargo, NuGet). Ketika satu dependensi dipercaya disusupi, ribuan korporasi yang mengimpor pustaka tersebut otomatis terinfeksi:

- **Dampak Berantai Skala Global (*Massive Blast Radius*)**:
  - **Insiden SolarWinds Orion (2020)**: Penyerang menyusupkan trojan ke dalam siklus kompilasi build resmi sebelum rilis ditandatangani, menginfeksi lebih dari 18.000 lembaga pemerintah dan perusahaan Fortune 500.
  - **Backdoor xz-utils (CVE-2024-3094)**: Penyerang menghabiskan waktu bertahun-tahun membangun reputasi kontributor tepercaya sebelum menyisipkan muatan berbahaya bertahap pada pustaka kompresi Linux guna membajak autentikasi SSH daemon global.
  - **Kasus `event-stream` (npm)**: Pustaka populer yang diunduh jutaan kali per minggu dialihkan kepemilikannya ke akun peretas, lalu disisipi skrip pencuri dompet mata uang kripto.
- **Kerugian Finansial & Hukum**:
  Kegagalan verifikasi rantai pasok dapat memicu kebocoran token repositori internal, kunci API cloud AWS/GCP produksi, dan hilangnya hak operasional perusahaan akibat pelanggaran standar keamanan seperti NIST SP 800-161 dan regulasi Executive Order 14028.

---

### 2. Gambaran Sistem yang Dihadapi (Target Commercial Architecture)
Dalam praktikum ini, Anda mengaudit infrastruktur build enterprise: **PackNexus Supply Hub (B2B Package Registry & Build Pipeline Hub)** yang beroperasi pada port `8013`.

Sistem ini merepresentasikan hub manajemen dependensi dan registri privat korporasi:
- **Private Artifact Registry**: Menyimpan dan mendistribusikan pustaka software terverifikasi serta mirror upstream (npmjs, PyPI).
- **Lockfile & Build Auditor**: Mempublikasikan manifest hasil audit otomatis dependensi pada endpoint `/packages/audit/package_lock_audit.json`.
- **Automated Pipeline Worker**: Lingkungan build otomatis yang mengeksekusi skrip instalasi paket (`postinstall_script`) dengan hak akses token produksi.
- **Pipeline Attestation Gate**: Endpoint verifikasi `/api/supply-chain/verify` untuk mengesahkan integritas rahasia pipeline sebelum artefak dipublikasikan ke server produksi.

---

### 3. Sasaran, Temuan & Eksekusi (Search, Find, and Execute)
Metodologi audit rantai pasok profesional:

1. **Apa yang Dicari (What to Search)**:
   - File manifest dan lockfile: `package.json`, `package-lock.json`, `Cargo.lock`.
   - Dependensi tanpa tanda tangan GPG/Sigstore, berasal dari URL tidak terverifikasi (*unpinned registry source*).
   - Hook siklus hidup paket yang mencurigakan: `postinstall`, `preinstall`, skrip eksternal `send_env.js`.
2. **Apa Temuan yang Menandakan Celah (Vulnerability Indicators)**:
   - Manifest `package_lock_audit.json` memuat paket asing (`apex-internal-telemetry`) yang tidak ditandatangani kunci perusahaan (*untrusted origin*).
   - Ditemukan metadata `postinstall_script` yang secara diam-diam mencuri variabel lingkungan dan mencatat `exfiltrated_pipeline_token`.
3. **Eksekusi Eksploitasi Terukur (Strategic Execution)**:
   - Ekstrak token rahasia pipeline berformat `ZITERA{...}` dari properti artefak yang disusupi.
   - Kirimkan token ke terminal verifikasi `/api/supply-chain/verify` untuk membuktikan dampak kompromi dan memvalidasi penyelesaian audit.

---

### 4. Pondasi Kuat: Mekanisme Integritas Lockfile & Attestation
Sebuah sistem rantai pasok yang aman mengandalkan verifikasi hash kriptografis:
- **Cryptographic Subresource Integrity (SRI)**: Setiap entri pustaka harus memiliki hash `sha512-...` yang cocok persis dengan repositori hulu.
- **Dependency Pinning**: Melarang penggunaan wildcard versi (`*` atau `^`) pada dependensi kritis guna mencegah penarikan paket versi baru yang belum diaudit.
- **Software Bill of Materials (SBOM)**: Inventaris lengkap seluruh komponen, pustaka, dan lisensi yang digunakan dalam aplikasi.
