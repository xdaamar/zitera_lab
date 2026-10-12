# ZITERA_LAB: EXTENDED CYBERSECURITY LAB CATALOG (BEYOND OWASP TOP 10)
**Document Reference:** `PRD/EXTENDED_CYBERSECURITY_LAB_CATALOG.md`  
**Phase:** 24 (CP03 — Checkpoint Push #4)  
**Author:** AI Pair Programmer (Software Craftsmanship Active)  
**Status:** EXPANSION CATALOG SPECIFICATION  
**Target Platform:** Windows x64  
**Primary Language:** Bahasa Indonesia (Secondary: English-ready architecture)  
**Distribution Model:** Lightweight Core + Optional Downloadable Lab Packages  

---

## 1. Executive Summary & Modular Expansion Philosophy

ZITERA_LAB dirancang untuk dapat diskalakan secara berkelanjutan dari 10 laboratorium awal menjadi puluhan hingga ratusan modul praktis tanpa perlu mengubah arsitektur aplikasi inti (*core application*). Penambahan modul baru cukup didaftarkan pada katalog terpusat (*signed catalog*), dikemas ke dalam arsip biner mandiri (`.zlab`), dan diunduh sesuai kebutuhan pengguna.

Prinsip utama ekspansi:
- **Kualitas Lebih Utama daripada Kuantitas:** Lebih baik menyediakan 15–20 lab yang stabil, realistis, dan memiliki validasi pengujian otomatis daripada 60 lab dengan materi dangkal atau tantangan yang rusak.
- **Isolasi Keamanan Ketat:** Seluruh modul baru tetap tunduk pada *sandbox* eksekusi lokal (Windows Job Objects, loopback port binding, tanpa akses shell host liar).
- **Format Paket Terstandarisasi:** Menggunakan struktur universal yang sama (`manifest.json`, `lesson/`, `challenge/`, `hints/`, dan target biner lokal).

---

## 2. Definisi 10 Domain Koleksi Spesialisasi (Collections A–J)

### Collection A — Web Application Security (`web-app`)
Fokus pada eksploitasi dan pertahanan aplikasi web modern di luar kategori standar OWASP:
- **CSRF-01:** Cross-Site Request Forgery & mitigasi SameSite cookie / anti-CSRF token.
- **SSRF-01:** Server-Side Request Forgery pada layanan fetch webhook lokal terisolasi.
- **UPL-01:** File Upload Vulnerabilities (bypass ekstensi ganda, MIME sniffing, eksekusi tak terduga).
- **TRAV-01:** Path Traversal & Local File Inclusion (LFI) pada endpoint pembaca dokumen.
- **CORS-01:** Exploiting Insecure Cross-Origin Resource Sharing dengan origin reflektif.

### Collection B — API Security (`api-sec`)
Fokus pada kerentanan REST, GraphQL, dan antarmuka JSON:
- **API-BOLA:** Broken Object Level Authorization pada endpoint transaksi modern `/api/v2/orders/{id}`.
- **API-MASS:** Mass Assignment vulnerability pada pendaftaran profil pengguna (`role: "admin"`).
- **API-RATE:** Missing Rate Limiting pada endpoint pengiriman OTP / verifikasi SMS.
- **API-LEAK:** Excessive Data Exposure di mana objek sensitif database dikembalikan mentah ke klien.

### Collection C — Identity & Session Security (`identity`)
Fokus pada siklus hidup akun dan manajemen token:
- **AUTH-MFA:** Kelemahan implementasi pemulihan MFA dan *bypass* kode cadangan.
- **AUTH-RESET:** Insecure Password Reset Token Generation (prediktabilitas timestamp / math random).
- **AUTH-OAUTH:** OAuth 2.0 State Parameter Misconfiguration & Account Linking Hijacking.
- **AUTH-JWT:** JSON Web Token attacks (algoritma `none`, manipulasi secret lemah, kid injection).

### Collection D — Secure Coding & Defensive Engineering (`secure-code`)
Fokus pada praktik penulisan kode aman bagi software engineer:
- **CODE-SQL:** Refactoring kode query rentan ke Prepared Statements & ORM aman.
- **CODE-XSS:** Penerapan Content Security Policy (CSP) dan kontekstual sanitasi string.
- **CODE-MEM:** Mencegah integer overflow, unhandled null pointers, dan memory safety bugs.
- **CODE-TEST:** Menulis *Automated Security Regression Unit Tests* pada pipeline CI/CD.

### Collection E — Linux & Terminal Fundamentals (`terminal-foundations`)
Menggunakan interpreter terminal edukatif ZITERA yang terkontrol (bukan raw shell host):
- **TERM-NAV:** Navigasi sistem berkas, struktur direktori Linux, dan pencarian berkas (`find`, `grep`).
- **TERM-PERM:** Hak akses berkas (`chmod`, `chown`), bit SUID/SGID, dan prinsip *least privilege*.
- **TERM-PROC:** Manajemen proses, inspeksi variabel lingkungan, dan pembacaan berkas log sistem.
- **TERM-PIPE:** Redirection stream (`|`, `>`, `>>`) dan teknik investigasi output biner.

### Collection F — Networking Fundamentals (`network-foundations`)
Memahami cara kerja lalu lintas data dan protokol komunikasi:
- **NET-TCP:** Anatomi TCP 3-Way Handshake, nomor port, dan perbedaan karakteristik TCP vs UDP.
- **NET-DNS:** Alur resolusi DNS, record A/CNAME/MX, dan mitigasi DNS poisoning konsep.
- **NET-TLS:** Cara kerja enkripsi TLS, sertifikat X.509, proses handshaking, dan cipher suite modern.
- **NET-SEG:** Konsep segmentasi jaringan, subnetting IPv4, dan aturan firewall dasar.

### Collection G — Defensive Security & Incident Analysis (`blue-team`)
Fokus pada analisis log, deteksi ancaman, dan rekonstruksi insiden:
- **BLUE-LOG:** Analisis log web server untuk mengidentifikasi jejak scanning dan upaya serangan.
- **BLUE-TIME:** Rekonstruksi garis waktu (*incident timeline*) dari kumpulan artefak forensik sintetis.
- **BLUE-TRIAGE:** Triage peringatan keamanan, pembedaan false positive vs true positive.
- **BLUE-HUNT:** Berburu ancaman (*threat hunting*) berbasis indikator kompromi (IoC).

### Collection H — Software Supply Chain & DevSecOps (`devsecops`)
Keamanan jalur rilis dan verifikasi komponen perangkat lunak:
- **SPLY-SBOM:** Membaca dan mengaudit Software Bill of Materials (CycloneDX / SPDX).
- **SPLY-PIN:** Kebijakan *hash pinning* dependensi dan deteksi dependensi palsu (*typosquatting*).
- **SPLY-SIG:** Penandatanganan biner otomatis di pipeline CI dan verifikasi Cosign / Sigstore.
- **SPLY-SECR:** Deteksi kebocoran kredensial dan API keys pada riwayat git commit (*trufflehog style*).

### Collection I — Forensics & Data Integrity (`forensics`)
Integritas bukti digital dan verifikasi data:
- **FOR-HASH:** Verifikasi hash kriptografis berkas (MD5 vs SHA256 vs BLAKE3) dan deteksi kolisi.
- **FOR-META:** Ekstraksi metadata berkas (EXIF, PDF metadata, timestomping concepts).
- **FOR-INTEG:** Verifikasi rantai kepemilikan (*chain-of-custody*) bukti digital.

### Collection J — Secure Architecture & Threat Modeling (`architecture`)
Perancangan sistem tahan banting tingkat makro:
- **ARCH-THREAT:** Penerapan metodologi STRIDE dalam pemodelan ancaman arsitektur web microservices.
- **ARCH-BOUND:** Penetapan batas kepercayaan (*trust boundary*) antara frontend, backend, dan DB.
- **ARCH-FAIL:** Desain arsitektur *graceful degradation* dan *circuit breaker* saat layanan down.

---

## 3. Struktur 6 Kelompok Rilis Katalog (*Catalog Sizing*) (Section 6.1)

Untuk pengelolaan rilis yang terukur, seluruh katalog ZITERA_LAB dikelompokkan ke dalam 6 koleksi rilis:

```text
┌─────────────────────────────────────────────────────────────────────────────┐
│                       ZITERA_LAB MODULAR CATALOG                            │
├────────────────────────────────┬────────────────────────────────────────────┤
│ 1. Core Foundation Collection  │ 2. OWASP Top 10:2025 Collection            │
│    (F01–F06 Micro-Lessons)     │    (A01–A10 Full Native Modules)           │
├────────────────────────────────┼────────────────────────────────────────────┤
│ 3. Web & API Security          │ 4. Defensive Security Collection           │
│    (SSRF, CSRF, File Upload,   │    (Log Analysis, Incident Timeline,       │
│     BOLA, Mass Assignment)     │     Threat Triage, IoC Hunting)            │
├────────────────────────────────┼────────────────────────────────────────────┤
│ 5. Systems & Networking        │ 6. Advanced Topics & Architecture          │
│    (Terminal Basics, TLS,      │    (Threat Modeling STRIDE, DevSecOps,     │
│     TCP Handshake, DNS)        │     Cryptographic Signature Verification)  │
└────────────────────────────────┴────────────────────────────────────────────┘
```

### 3.1 Lab Prioritas Rilis Pertama (Wave 1: Baseline Launch)
Target prioritas rilis awal difokuskan pada pemantapan konten berkualitas tinggi:
1. **Existing 10 OWASP Labs (A01 s/d A10):** Audit lengkap, standarisasi durasi realistis, dan perbaikan bilingual.
2. **Shared Foundation (F01–F04):** 4 micro-labs dasar (HTTP, Trust Boundaries, Sessions, Hashing/Crypto).
3. **Web & API Expansion Prioritas (W01–W03):**
   - `W01` (`zitera-lab-csrf`): CSRF & Token Defense.
   - `W02` (`zitera-lab-ssrf`): SSRF dalam simulator lokal terisolasi.
   - `W03` (`zitera-lab-bola`): Broken Object Level Authorization (API Security).
4. **Defensive Foundation Prioritas (D01–D02):**
   - `D01` (`zitera-lab-log-audit`): Analisis Log Web & Deteksi Jejak Eksploitasi.
   - `D02` (`zitera-lab-integ-check`): Verifikasi Integritas Berkas & Deteksi Tampering.

**Total Wave 1 Launch:** 10 OWASP Labs + 4 Foundation Micro-Lessons + 5 Expansion Labs = **19 Modul Teruji**.

---

## 4. Backlog Pengembangan Jangka Panjang (Wave 2 & Wave 3)

| Kode Usulan | Kategori Domain | Judul Laboratorium | Perkiraan Kompleksitas |
|---|---|---|---|
| `W04` | Web Security | Insecure File Upload & Content Sniffing | Intermediate |
| `W05` | Web Security | Path Traversal & Document Leakage | Beginner |
| `A02-API` | API Security | API Mass Assignment in User Registration | Intermediate |
| `A03-API` | API Security | Excessive Data Exposure & Sensitive Scrubbing | Beginner |
| `I01` | Identity | Password Reset Race Conditions & Timestamp Guessing | Intermediate |
| `I02` | Identity | JWT None Algorithm & Key Confusion | Advanced |
| `T01` | Terminal | Linux File Permissions & Least Privilege | Beginner |
| `T02` | Terminal | Linux Process Management & Environmental Audit | Beginner |
| `N01` | Networking | TCP Handshake, Packet Structures & Wire Interpretation | Beginner |
| `N02` | Networking | TLS 1.3 Handshake & Certificate Validation | Intermediate |
| `S01` | Supply Chain | Dependency Lockfile Tampering & Hash Pinning | Intermediate |
| `M01` | Architecture | STRIDE Threat Modeling for Microservices | Advanced |

---

## 5. Konvensi Penamaan Paket Baru (*Naming & Namespace Policy*)

Agar penambahan lab baru tidak bertabrakan dengan ID kanonikal yang ada:
- **Namespace OWASP Top 10 Resmi:** Dibatasi ketat hanya untuk `A01` hingga `A10`. Modul tambahan tidak boleh diberi kode `A11`, `A12`, dsb.
- **Namespace Modul Fondasi:** Menggunakan format `F01`, `F02`, dst. (contoh paket: `zitera-lab-f01`).
- **Namespace Web & API Security:** Menggunakan format `W01`, `W02`, dst. (contoh paket: `zitera-lab-w01`).
- **Namespace Defensive & Incident:** Menggunakan format `D01`, `D02`, dst. (contoh paket: `zitera-lab-d01`).
- **Namespace Sistem & Jaringan:** Menggunakan format `S01`, `S02`, dst. (contoh paket: `zitera-lab-s01`).
- Setiap paket tetap mematuhi schema manifest `schema_version: 1` dengan port default unik pada rentang `8021`–`8099`.
