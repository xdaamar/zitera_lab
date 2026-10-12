# ZITERA_LAB: CURRICULUM QUALITY GATE & CONTENT VALIDATION FRAMEWORK
**Document Reference:** `PRD/CURRICULUM_QUALITY_GATE.md`  
**Phase:** 24 (CP09 — Checkpoint Push #10)  
**Author:** AI Pair Programmer (Software Craftsmanship Active)  
**Status:** CANONICAL QUALITY GATE SPECIFICATION  
**Target Platform:** Windows x64  
**Primary Language:** Bahasa Indonesia (Secondary: English-ready architecture)  

---

## 1. Executive Summary & Purpose

`CURRICULUM_QUALITY_GATE` menetapkan standar dan kerangka kerja otomatisasi validasi mutu untuk seluruh konten edukasi di ZITERA_LAB. Setiap modul laboratorium wajib melewati gerbang kualitas ini sebelum didaftarkan ke katalog resmi atau dikemas ke dalam rilis paket `.zlab`.

Gerbang kualitas terdiri dari tiga pilar pengujian:
1. **Validasi Struktural (*Structural Validation*):** Integritas skema, pemetaan referensi berkas, dan keunikan identitas.
2. **Validasi Pedagogis (*Educational Validation*):** Keselarasan tujuan belajar, kejelasan instruksi, progresi kesulitan, dan konsistensi Bahasa Indonesia.
3. **Validasi Keamanan Konten (*Security Validation*):** Pencegahan kebocoran flag publik, ketiadaan kredensial produksi nyata, penolakan akses host shell liar, dan isolasi lingkungan target.

---

## 2. Kriteria Validasi Terperinci (Section 12)

### 2.1 Validasi Struktural (*Structural Validation*)
- **Skema Manifest:** Berkas `manifest.json` harus valid JSON dan memenuhi `schema_version: 1`.
- **Identitas Kanonikal (Stable IDs):** ID lab wajib mengikuti format resmi (`A01`–`A10`, `W01`–`W03`, `D01`–`D02`).
- **Keunikan ID & Port:** Tidak boleh ada duplikasi `id`, `package_id`, atau `default_port`.
- **Kelengkapan Section Wajib:** Memiliki minimal 5 berkas materi inti dalam `labs/A*/lesson/`:
  - `introduction.id.md`
  - `concept.id.md`
  - `analogy.id.md`
  - `walkthrough.id.md`
  - `remediation.id.md`
- **Keberadaan Spesifikasi Misi & Hints:** Direktori `labs/A*/challenge/` wajib memuat `challenge.id.md` dan `hints.json`.
- **Rentang Durasi Realistis:** Nilai `estimated_minutes` wajib berada pada rentang logis 20–90 menit.

### 2.2 Validasi Pedagogis (*Educational Validation*)
- **Tujuan Belajar Terukur:** Minimal 3 butir `learning_objectives` yang jelas dan dapat diuji.
- **Prasyarat Masuk Akal:** `prerequisites` menyebutkan topik dasar yang relevan dari shared foundation.
- **Konsistensi Bahasa Indonesia:** Teks instruksi utama disajikan dalam Bahasa Indonesia yang ramah pemula dengan istilah teknis industri yang tepat.
- **Progresi Petunjuk (Tiered Hints):** `hints.json` memuat minimal 3 tingkat petunjuk bantuan bertahap (Tier 1: Orientasi Konsep, Tier 2: Area Teknis, Tier 3: Arah Sintaks/Investigasi).
- **Remediasi Wajib:** Setiap kelemahan keamanan wajib disertai penjelasan kode perbaikan defensif dan cara menulis uji regresi.

### 2.3 Validasi Keamanan Konten (*Content Security Gate*)
- **Pencegahan Kebocoran Flag Publik (*No Flag Leaks*):** Berkas lesson markdown dan petunjuk bantuan publik tidak boleh mengekspos string flag `ZITERA{...}`.
  - *Aturan Pelaporan:* Jika deteksi otomatis menemukan string flag pada dokumen publik, laporan hanya mencatat:
    ```text
    CHALLENGE_FLAG_LEAK_DETECTED: <lab_id>
    ```
    Dilarang mencetak nilai rahasia flag ke dalam log atau laporan.
- **Ketiadaan Kredensial Nyata (*No Real Secrets*):** Dilarang menyertakan token API asli, private key produksi, atau hash kata sandi pengguna nyata.
- **Ketiadaan Akses Shell Host Liar (*No Raw Host Passthrough*):** Lab tidak boleh mengeksekusi `cmd.exe /c` atau PowerShell liar di luar Job Object AppContainer yang dibatasi.
- **Zero Host File Coupling:** Lab tidak boleh mengandalkan path absolut di komputer pengembang (`C:\Users\...`).

---

## 3. Implementasi Skrip Validator Otomatis

Alat validator diimplementasikan pada skrip mandiri:
`scripts/validate_curriculum.ps1`

Skrip ini memindai seluruh direktori `labs/` secara rekursif, memvalidasi JSON schema, memeriksa keterkaitan berkas section, mengaudit durasi waktu, mendeteksi potensi kebocoran flag pada dokumen publik, dan menghasilkan laporan kepatuhan terstruktur (*Quality Audit Report*).
