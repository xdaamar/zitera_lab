# A08: Software and Data Integrity Failures — Bahaya Menerima Data Tanpa Segel

Software and Data Integrity Failures menempati peringkat ke-8 (**#8**) dalam daftar risiko **OWASP Top 10:2025**. Kerentanan ini berfokus pada asumsi keliru bahwa kode, berkas pembaruan (*firmware/software updates*), atau data serialisasi yang diterima oleh aplikasi selalu murni dan tidak pernah dimanipulasi di tengah jalan tanpa memverifikasi tanda tangan digital integritasnya (*cryptographic integrity verification*).

---

### 🔴 1. Kenapa Ini Sangat Berbahaya? (Business & Financial Impact)
Ketika aplikasi mengeksekusi atau menerapkan data tanpa memvalidasi integritasnya, sistem menyerahkan kendali sepenuhnya kepada penyerang:

- **Penyusupan Pintu Belakang (*Backdoor Injection*) pada Sistem Kritis**:
  - Penyerang dapat menyusupkan pembaruan konfigurasi atau kode firmware palsu yang mematikan fitur keamanan, menyalakan mode debug berbahaya, atau mengalihkan transaksi keuangan ke server peretas.
  - Insiden nyata seperti serangan pembajakan pembaruan perangkat lunak jaringan perbankan dan industri utilitas, di mana kegagalan verifikasi tanda tangan digital sertifikat mengakibatkan terkomprominya ribuan perangkat edge dan pusat data telekomunikasi.
- **Sanksi Kepatuhan Hukum Berat**:
  Standar keamanan internasional seperti **ISO/IEC 27001**, regulasi perbankan **PCI-DSS**, dan **UU PDP No. 27/2022** mewajibkan mekanisme *code signing* dan validasi integritas terhadap setiap artefak yang diunduh atau dipasang pada infrastruktur komersial.

---

### 🔍 2. Bagaimana Cara Mencari & Mengujinya? (Recon & Vulnerability Indicators)
Metodologi audit integritas data dan perangkat lunak:

1. **Apa yang Dicari (What to Search)**:
   - Endpoint penerima berkas konfigurasi, plugin, atau firmware: `/api/deploy_update`, `/api/config/apply`, `/update/firmware`.
   - Parameter integritas: `signature`, `signature_verified`, `checksum`, `hash`, `signer`.
2. **Apa Temuan yang Menandakan Celah (Vulnerability Indicators)**:
   - Endpoint `POST /api/deploy_update` menerima paket pembaruan yang memiliki `signature_verified: false` atau tanda tangan kosong, dan tetap menerapkannya ke memori aktif kluster (*Cluster Hot-Reload*).
3. **Eksekusi Eksploitasi Terukur (Strategic Execution)**:
   - Amati konfigurasi aktif yang sah via `GET /api/config`.
   - Buat paket payload pembaruan modifikasi yang menyalakan opsi `debug_backdoor: true` dengan `signature_verified: false`.
   - Kirimkan paket ke server dan amati respons yang mengonfirmasi bahwa pembaruan diterima tanpa penolakan, mengungkap bendera autoritatif `ZITERA{1nt3gr1ty_f41lur3_un51gn3d_p4ck4g3}`.

---

### 🛡️ 3. Bagaimana Cara Memperbaikinya? (Secure Coding & Remediation)
Pertahanan integritas software mutlak:
- **Wajibkan Verifikasi Kriptografis Asimetris (Cryptographic Code Signing)**: Server hanya boleh menerima pembaruan yang memiliki tanda tangan digital valid dari otoritas kunci privat resmi (menggunakan algoritma RSA-4096 atau ECDSA Ed25519).
- **Tolak Keras Paket yang Gagal Verifikasi (*Fail-Closed Signature Validation*)**: Jika `signature_verified == false` atau hash tidak cocok, batalkan proses dan isolasi paket seketika.
- **Gunakan Saluran Distribusi Tepercaya dengan TLS Mutual Authentication (mTLS)**: Pastikan pengirim paket telah terotentikasi secara timbal balik di tingkat jaringan sebelum payload diterima.

---

### Gambaran Sistem yang Dihadapi (Target Commercial Architecture)
Dalam laboratorium ini, kamu menghadapi mesin analitik bisnis: **DataFlow Analytics / Zitera Edge Config Ingestion (Enterprise BI & Stream Processing Engine)** yang beroperasi pada port `8018`.

Komponen arsitektur:
- **Active Streaming Engine Configuration**: Endpoint `GET /api/config` yang menampilkan konfigurasi runtime kluster saat ini.
- **Ingestion Update Gateway**: Endpoint `POST /api/deploy_update` yang menerima berkas konfigurasi pembaruan.
- **Celah Integritas**: Server menerima paket pembaruan dari pihak asing tanpa memverifikasi tanda tangan digital otoritas resmi (*untrusted deserialization / unverified deployment*).
