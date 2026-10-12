# PHASE 24: FINAL CURRICULUM REVIEW PACKAGE
**Document Reference:** `reports/PHASE_24_CURRICULUM_REVIEW.md`  
**Phase:** 24 (CP12 — Checkpoint Push #13)  
**Execution Date:** 2026-10-12  
**Target Platform:** Windows x64  
**Author:** AI Pair Programmer (Software Craftsmanship Active)  
**Branch:** `migration/phase-24-curriculum-redesign-modular-architecture`  
**Status:** CURRICULUM REVIEW PACKAGE COMPLETE  

---

## 1. Executive Summary & Review Scope

Dokumen ini menyajikan tinjauan kurikulum komprehensif (*Comprehensive Curriculum Review*) untuk ditinjau dan disetujui oleh Repository Owner sebelum ZITERA_LAB melangkah ke tahap implementasi dan rilis biner berikutnya.

Seluruh cetak biru (*blueprints*), standar kepenulisan, spesifikasi tantangan CTF, kontrak distribusi modular, dan pengujian mutu telah diselesaikan secara konsisten tanpa melanggar **MANDATORY STOP-BUILD RULE**.

---

## 2. 20 Poin Tinjauan Kurikulum Kanonikal (Section 15)

### 1. Existing Content yang Dipertahankan
- Seluruh 10 modul laboratorium native (`A01`–`A10`) tetap dipertahankan dengan ID kanonikal, package ID (`zitera-lab-a01` s/d `a10`), port default (`8011`–`8020`), dan runtime `native_sandboxed`.
- Seluruh kode backend simulasi Rust di `engine/rust/src/bin/a01_lab.rs` s/d `a10_lab.rs` tetap dipertahankan.
- Skema `manifest.json` v1 dan arsitektur verifikasi Ed25519 tetap menjadi standar acuan.

### 2. Content yang Diperbaiki
- Penyesuaian durasi pengerjaan realistis (*realistic duration budget*) pada seluruh 10 manifest JSON (menggantikan angka default seragam 45m yang tidak akurat).
- Pembersihan dan sanitasi kebocoran flag publik pada `labs/A03/lesson/walkthrough.md` dan `labs/A07/lesson/walkthrough.md`.

### 3. Content Baru yang Ditulis di Phase 24
- `labs/A05/lesson/architecture.md` & `labs/A05/lesson/architecture.id.md` (Arsitektur Injection).
- `labs/A08/lesson/architecture.id.md` (Arsitektur Software/Data Integrity Failures berbahasa Indonesia).
- `labs/A09/lesson/architecture.id.md` (Arsitektur Security Logging & Alerting berbahasa Indonesia).
- `labs/A10/lesson/architecture.id.md` (Arsitektur Mishandling of Exceptional Conditions berbahasa Indonesia).
- 8 Dokumen Spesifikasi PRD Baru (`PRD/CURRICULUM_ARCHITECTURE_V1.md`, `PRD/OWASP_TOP10_2025_CURRICULUM_BLUEPRINT.md`, `PRD/EXTENDED_CYBERSECURITY_LAB_CATALOG.md`, `PRD/LESSON_AUTHORING_STANDARD.md`, `PRD/CTF_CHALLENGE_STANDARD.md`, `PRD/FOUNDATION_AND_EXTENDED_LAB_SPECS.md`, `PRD/LAB_DISTRIBUTION_CONTRACT_V1.md`, `PRD/CURRICULUM_QUALITY_GATE.md`, `PRD/CURRICULUM_UX_AND_RESOURCE_EFFICIENCY.md`).
- Skrip validasi otomatis `scripts/validate_curriculum.ps1`.

### 4. Cakupan OWASP Top 10:2025 (A01–A10)
- 10 dari 10 kategori resmi OWASP Top 10:2025 tercakup 100% dengan teks modular lengkap (Introduction, Concept, Analogy, Architecture, Walkthrough, Remediation) dalam Bahasa Indonesia dan Inggris.

### 5. Cakupan Fondasi Bersama (*Shared Foundation*)
- Didefinisikan 6 modul mikro interaktif (`F01`–`F06`) yang mencakup dasar HTTP, batas kepercayaan klien/server, manajemen cookie/token sesi, autentikasi vs otorisasi, dasar database, dan alur penggunaan lab ZITERA.

### 6. Backlog Katalog Tambahan (*Extended Catalog*)
- Dipetakan 10 domain spesialisasi (Collections A–J) dengan 5 modul Wave 1 peluncuran awal:
  - `W01`: CSRF & Anti-Token Defense (Web Security).
  - `W02`: SSRF in Isolated Environment (Web Security).
  - `W03`: Broken Object Level Authorization / BOLA (API Security).
  - `D01`: Web Log Security Audit & Threat Detection (Defensive Security).
  - `D02`: Cryptographic File & Package Integrity Verification (Forensics & Data Integrity).

### 7. Dua Jalur Pembelajaran (*Two Learning Pathways*)
- **Track A — Web Security Fundamentals:** `A01` -> `A07` -> `A02` -> `A05` -> `A04`.
- **Track B — Secure Systems & Architecture:** `A03` -> `A06` -> `A08` -> `A09` -> `A10`.

### 8. Urutan Rekomendasi Belajar
- Pemula disarankan menyelesaikan modul Fondasi `F01`–`F04` terlebih dahulu, lalu menuntaskan Track A sebelum beralih ke Track B.

### 9. Estimasi Durasi Realistis
- Divalidasi dan ditetapkan: A01 (35m), A02 (35m), A03 (45m), A04 (45m), A05 (50m), A06 (65m), A07 (40m), A08 (60m), A09 (50m), A10 (65m).

### 10. Keselarasan Tujuan Belajar (*Learning Objectives*)
- Setiap lab memiliki minimal 3 butir tujuan belajar terukur yang selaras langsung dengan latihan praktik dan tantangan CTF.

### 11. Desain CTF & Validasi Flag
- Format standar: `ZITERA{...}`.
- Validasi waktu konstan (*timing-safe compare*) di backend Rust engine.
- 3 tingkat petunjuk bertahap (Tier 1 Orientasi, Tier 2 Teknis, Tier 3 Sintaks/Arah).
- Kerahasiaan flag terjaga (0 kebocoran di dokumen publik).

### 12. Hasil Uji Per-Lab
- Seluruh 10 lab lolos uji Layer 1 (Content & Structure), Layer 2 (Practice Mechanics via Source Inspection), Layer 3 (Challenge Contracts), dan Layer 4 (Security Boundaries).

### 13. Kontrak Unduhan & Modus Luring (*Offline Contract*)
- Model Aplikasi Inti Ringan + Paket `.zlab` opsional.
- Verifikasi tanda tangan digital Ed25519 dan checksum SHA-256 sebelum ekstraksi.
- Seluruh materi dan target lab dapat dijalankan 100% luring setelah paket terpasang.

### 14. Persyaratan Kompatibilitas
- Kompatibel dengan Windows x64 (Windows 10 / 11), arsitektur zero-developer toolchain dependency (tidak mewajibkan pengguna menginstal Rust, Docker, Node.js, atau Python).

### 15. Batasan Keamanan (*Security Constraints*)
- Eksekusi native diatur oleh Windows Job Objects dengan isolasi memori dan proses.
- Loopback port binding only (`127.0.0.1:<port>`).
- Dilarang keras memberikan akses passthrough ke shell host (`cmd.exe /c` atau PowerShell liar).

### 16. Keterbatasan yang Diketahui (*Known Limitations*)
- Flag disimpan dalam bentuk teks polos pada berkas `hints.json` lokal di dalam paket.
- Validasi runtime biner ditangguhkan (*runtime deferred*) karena larangan build pada Phase 24.

### 17. Usulan Perubahan Mesin Inti (*Core Change Requests*)
- `CCR-01`: Migrasi penyimpanan flag ke salted hash di `hints.json`.
- `CCR-02`: Preferensi otomatis file markdown `.id.md` pada Flutter localization controller.
- `CCR-03`: Pendaftaran array `track_ids` pada schema katalog.

### 18. Hambatan Rilis (*Release Blockers*)
- Penandatanganan biner produksi membutuhkan EV Authenticode Code Signing Certificate milik owner untuk menghindari SmartScreen warning pada mesin pengguna luar.
- Ketiadaan URL donasi resmi dari owner (`DONATION_URL_REQUIRED`).

### 19. Keputusan yang Wajib Dibuat Owner (*Owner Approval Gate*)
1. Menyetujui struktur kurikulum Two-Track (Track A dan Track B).
2. Menyetujui daftar 5 lab ekspansi prioritas Wave 1 (`W01`, `W02`, `W03`, `D01`, `D02`).
3. Menyetujui durasi realistis yang telah diperbarui pada manifest.
4. Memberikan URL donasi resmi untuk `AppConfig.donationUrl`.
5. Memberikan otorisasi untuk memulai kompilasi rilis dan packaging installer pada Phase 25.

### 20. Usulan Rencana Implementasi Phase 25
- Mengimplementasikan 5 modul paket biner Wave 1 (`W01`–`W03`, `D01`–`D02`).
- Melakukan kompilasi bersih (*clean build*) engine Rust dan Flutter GUI bundle.
- Mengompilasi installer `Setup.exe` menggunakan Inno Setup V4.
- Melakukan pengujian instalasi end-to-end pada lingkungan Windows bersih (*clean machine*).
