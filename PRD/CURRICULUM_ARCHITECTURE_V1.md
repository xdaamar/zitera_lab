# ZITERA_LAB: CANONICAL CURRICULUM ARCHITECTURE (V1)
**Document Reference:** `PRD/CURRICULUM_ARCHITECTURE_V1.md`  
**Phase:** 24 (CP01 — Checkpoint Push #2)  
**Author:** AI Pair Programmer (Software Craftsmanship Active)  
**Status:** CANONICAL ARCHITECTURE SPECIFICATION  
**Target Platform:** Windows x64  
**Primary Language:** Bahasa Indonesia (Secondary: English-ready architecture)  
**Distribution Model:** Lightweight Core + Optional Downloadable Lab Packages  

---

## 1. Executive Summary & Educational Vision

ZITERA_LAB bertujuan menjadi platform edukasi keamanan siber (*cybersecurity learning platform*) yang praktis, ramah pemula, terstruktur secara pedagogis, dan aman untuk siswa SMK, mahasiswa, *software developer*, serta pembelajar mandiri.

Pendekatan pembelajaran ZITERA_LAB bertumpu pada **Prinsip Siklus Belajar 5 Tahap (*5-Stage Learning Cycle*)**:
1. **Teori Konseptual (*Understand*):** Penjelasan prinsip keamanan, terminologi, dan analogi mental yang jelas dalam Bahasa Indonesia.
2. **Eksplorasi Perilaku (*Observe*):** Mengamati perbedaan perilaku aplikasi yang rentan (*vulnerable*) dibandingkan perilaku yang aman (*secure*) pada simulasi interaktif.
3. **Eksperimen Mandiri (*Practice*):** Menjalankan payload atau input terkontrol pada lingkungan latihan lokal yang terisolasi (*sandboxed*).
4. **Tantangan CTF (*Solve Challenge*):** Menyelesaikan misi mandiri berbasis skenario untuk menemukan dan mengekstraksi flag yang divalidasi oleh sistem penilaian (*grader*).
5. **Remediasi & Pertahanan (*Remediate*):** Memahami akar penyebab teknis, menerapkan perbaikan kode (*patch*), dan memvalidasi kebenaran pertahanan menggunakan *security regression tests*.

---

## 2. Prinsip Fondasi Kurikulum (Section 4.1)

Setiap modul laboratorium di ZITERA_LAB wajib memenuhi standar rekayasa kurikulum berikut:

1. **Fokus Konsep Tunggal (*Concept Isolation*):** Setiap lab mengajarkan satu konsep kerentanan utama atau satu klaster konsep terkait yang kohesif.
2. **Tujuan Belajar Terukur (*Measurable Objectives*):** Memiliki tujuan pembelajaran yang dapat dibuktikan secara empiris melalui observasi atau pengujian.
3. **Prasyarat Eksplisit (*Explicit Prerequisites*):** Menyatakan secara jujur pengetahuan awal yang harus dipahami sebelum memulai lab.
4. **Keterkaitan Nyata (*Observable Reality*):** Teori dihubungkan langsung dengan respons target interaktif (HTTP code, JSON payload, log audit, atau error message).
5. **Prioritas Remediasi (*Remediation-First*):** Lab tidak boleh berakhir hanya pada eksploitasi; pencegahan dan mitigasi defensif adalah tujuan akhir pembelajaran.
6. **Tantangan Mandiri (*Autonomous Challenge*):** Tantangan tidak boleh sekadar mengulang *walkthrough*, melainkan menguji transfer pemahaman ke variasi skenario.
7. **Verifikasi Penyelesaian Teruji (*Deterministic Verification*):** Flag dan penyelesaian diverifikasi oleh *grader* terpercaya tanpa bergantung pada asumsi UI klien.
8. **Paket Modular Mandiri (*Modular Distribution*):** Setiap lab dapat diunduh, diverifikasi integritasnya, dan dijalankan secara terpisah tanpa mewajibkan unduhan seluruh katalog.
9. **Desain Utamakan Luring (*Offline-First*):** Setelah paket terpasang, seluruh lesson, practice target, dan challenge dapat dijalankan tanpa koneksi internet.
10. **Kompatibilitas Runtime Terisolasi (*Strict Runtime Sandbox*):** Lab berjalan sebagai proses lokal native berkecepatan tinggi di dalam batas Windows Job Objects tanpa membutuhkan instalasi dependency pengembang eksternal (Docker, Rust, Node.js, atau Python di komputer pengguna).
11. **Bebas Ketergantungan Workspace (*Zero Workspace Coupling*):** Lab runtime terinstal di `%LOCALAPPDATA%\Programs\ZiteraLab\packages` dan tidak mengandalkan berkas kode sumber *development repository*.

---

## 3. Standar Metadata & Pemetaan Manifest (Section 4.2)

Untuk memastikan interoperabilitas antara katalog, runtime engine Rust, dan UI Flutter, berikut pemetaan standar metadata terhadap skema `LabManifest` v1:

| Field Konseptual | Tipe Data | Field Manifest Eksisting | Deskripsi & Aturan Validasi |
|---|---|---|---|
| `lab_id` | String | `id` | Identifier kanonikal (contoh: `A01`, `A02`, `A05`). Wajib unik. |
| `package_id` | String | `package_id` | Nama paket distribusi (`zitera-lab-a01`). |
| `title` | String | `title` | Judul resmi lab dalam Bahasa Indonesia / bilingual (1–128 karakter). |
| `short_description` | String | `short_description` | Ringkasan 1–2 kalimat untuk kartu katalog (maks 256 karakter). |
| `owasp_category` | String | `owasp` | Kategori resmi OWASP Top 10:2025 (contoh: `A01:2025`). |
| `curriculum_domain` | String | `standard` | Standar kurikulum (`owasp-top10`, `web-security`, `systems-security`). |
| `track_ids` | Array[String] | `modes` / `track_ids` | Jalur belajar terkait (`track-a`, `track-b`, `shared-foundation`). |
| `difficulty` | Enum | `difficulty` | Tingkat kesulitan: `Beginner`, `Intermediate`, `Advanced`. |
| `estimated_duration_minutes` | Integer | `estimated_minutes` | Estimasi durasi realistis berdasarkan uji coba aktual (30–75 menit). |
| `prerequisites` | Array[String] | `prerequisites` | Daftar topik prasyarat yang harus dikuasai sebelumnya. |
| `learning_objectives` | Array[String] | `learning_objectives` | 3–5 butir hasil belajar terukur. |
| `skills` | Array[String] | `skills` | Keterampilan spesifik yang dilatih (contoh: `IDOR Audit`, `Header Hardening`). |
| `lesson_references` | Object | Dinamis | File section markdown dalam direktori `labs/A*/lesson/`. |
| `practice_references` | String | `runtime` + `default_port` | Target lokal practice engine native (`127.0.0.1:<default_port>`). |
| `challenge_references` | String | `entrypoint` | Spesifikasi misi di `labs/A*/challenge/challenge.md`. |
| `hints_reference` | Object | `hints` | Skema petunjuk berjenjang di `labs/A*/challenge/hints.json`. |
| `remediation_reference` | String | `remediation.md` | Petunjuk penulisan kode aman dan uji regresi. |
| `tags` | Array[String] | `skills` / `tags` | Tag pencarian katalog (contoh: `sql`, `idor`, `cors`, `crypto`). |
| `content_version` | String | `version` | Semantic version konten materi (contoh: `1.0.1`). |
| `minimum_core_version` | String | `minimum_core_version` | Versi minimum mesin inti ZITERA yang didukung (`2.0.0`). |
| `security_version` | Integer | `security_version` | Anti-rollback counter untuk pencegahan downgrade paket. |
| `locale_availability` | Array[String] | Bahasa | Daftar bahasa yang didukung (`id`, `en`). |
| `runtime_requirements` | String | `runtime` | Tipe runtime: `native_sandboxed`, `mock`. |
| `test_requirements` | Array[String] | Automated Tests | Persyaratan uji verifikasi konten dan integritas challenge. |

---

## 4. Dua Jalur Belajar Utama (*Learning Pathways*) (Section 4.3)

Untuk memberikan navigasi yang terstruktur bagi pembelajar dengan latar belakang berbeda, kurikulum ZITERA_LAB dibagi menjadi dua jalur utama dengan satu fondasi bersama:

```text
               ┌────────────────────────────────────────────────────────┐
               │              SHARED FOUNDATION MICRO-LESSONS           │
               │  HTTP Basics • Client/Server Boundaries • Sessions •   │
               │  Encoding vs Crypto • DevTools • Challenge Mechanics   │
               └───────────────────────────┬────────────────────────────┘
                                           │
                    ┌──────────────────────┴──────────────────────┐
                    ▼                                             ▼
       ┌─────────────────────────┐                   ┌─────────────────────────┐
       │         TRACK A         │                   │         TRACK B         │
       │ Web Security            │                   │ Secure Systems &        │
       │ Fundamentals            │                   │ Application Architecture│
       ├─────────────────────────┤                   ├─────────────────────────┤
       │ 1. A01: Broken Access   │                   │ 1. A03: Supply Chain    │
       │ 2. A07: Authentication  │                   │ 2. A06: Insecure Design │
       │ 3. A02: Misconfiguration│                   │ 3. A08: Data Integrity  │
       │ 4. A05: Injection       │                   │ 4. A09: Logging/Alerting│
       │ 5. A04: Cryptography    │                   │ 5. A10: Exception Handle│
       └─────────────────────────┘                   └─────────────────────────┘
```

---

### 4.1 Shared Foundation Micro-Lessons
Modul fondasi bersama dirancang ringan sebagai micro-lessons interaktif yang tidak membutuhkan executable binary berat:

1. **F01: Anatomi Request & Response HTTP:**
   - Memahami method (`GET`, `POST`, `PUT`, `DELETE`), status code (`200`, `301`, `401`, `403`, `404`, `500`), headers, dan request body.
2. **F02: Batas Kepercayaan Klien vs Server (*Trust Boundaries*):**
   - Mengapa input dari peramban/klien tidak boleh dipercaya secara membabi buta, dan konsep otorisasi di sisi server.
3. **F03: Cookie, Token, dan Manajemen Sesi:**
   - Bagaimana HTTP yang *stateless* mempertahankan sesi login menggunakan cookie, flag `HttpOnly`, `Secure`, dan token JWT.
4. **F04: Autentikasi vs Otorisasi:**
   - Membedakan pembuktian identitas (*siapa kamu*) dengan penentuan hak akses (*apa yang boleh kamu lakukan*).
5. **F05: Encoding vs Hashing vs Enkripsi:**
   - Menghentikan kebingungan umum antara Base64 (representasi data), SHA-256 (ringkasan satu arah), dan AES (kerahasiaan dua arah).
6. **F06: Cara Kerja Simulator & Lab Terminal ZITERA:**
   - Panduan menggunakan konsol edukasi lokal, memeriksa status readiness, membaca clue tantangan, dan submit flag CTF.

---

### 4.2 Track A — Web Security Fundamentals

**Profil Target:** Pemula, siswa SMK RPL/TKJ, mahasiswa informatika tahun pertama, dan web developer pemula.  
**Tujuan Pedagogis:** Memahami cara kerja kerentanan web yang paling umum dieksploitasi dan menerapkan validasi serta otorisasi yang kokoh.

| No | Modul Lab | Judul & Fokus | Target Pembelajaran | Durasi Realistis |
|---|---|---|---|---|
| 1 | **A01** | **Broken Access Control** | IDOR (*Insecure Direct Object Reference*), privilege escalation horizontal, pemeriksaan kepemilikan data. | 35 Menit |
| 2 | **A07** | **Authentication Failures** | Kelemahan alur login, serangan tebakan otomatis (*brute force*), proteksi rate-limiting, session fixation. | 40 Menit |
| 3 | **A02** | **Security Misconfiguration** | Bahaya kredensial default, antarmuka diagnostik/debug terbuka, CORS longgar, *directory listing*. | 35 Menit |
| 4 | **A05** | **Injection** | SQL Injection, pemisahan data vs perintah eksekusi, parameterized queries, context-aware output encoding. | 50 Menit |
| 5 | **A04** | **Cryptographic Failures** | Data sensitif terbuka, kesalahan hashing password polos, hardcoded secret tokens, pemilihan algoritma aman. | 45 Menit |

---

### 4.3 Track B — Secure Systems & Application Architecture

**Profil Target:** Pengembang backend, software engineer intermediate, DevOps/SecOps, dan calon application security engineer.  
**Tujuan Pedagogis:** Membangun arsitektur perangkat lunak yang tangguh terhadap kegagalan komponen pihak ketiga, desain logika bisnis yang cacat, manipulasi integritas, serta kegagalan penanganan kondisi darurat.

| No | Modul Lab | Judul & Fokus | Target Pembelajaran | Durasi Realistis |
|---|---|---|---|---|
| 1 | **A03** | **Software Supply Chain Failures** | Analisis dependency lockfile, package provenance, manipulasi versi artefak, typosquatting simulasi. | 45 Menit |
| 2 | **A06** | **Insecure Design** | Threat modeling, business logic bypass, invariant state machine, rate-limit bypassing, failure mode analysis. | 65 Menit |
| 3 | **A08** | **Software or Data Integrity Failures** | Verifikasi tanda tangan digital, deserialisasi data tak terpercaya, proteksi anti-tampering, verifikasi payload. | 60 Menit |
| 4 | **A09** | **Security Logging & Alerting Failures** | Audit trail gaps, deteksi aktivitas mencurigakan, redaksi data sensitif dari log, log injection defense. | 50 Menit |
| 5 | **A10** | **Mishandling of Exceptional Conditions** | Fail-open vs fail-closed, degradasi layanan parsial, race conditions dalam error handling, boundary rollback. | 65 Menit |

---

## 5. Navigasi Lintas Jalur (*Cross-Track Navigation*)

Sistem katalog ZITERA_LAB dirancang fleksibel sehingga pengguna tidak dipaksa menyelesaikan modul secara kaku:
- Pengguna dapat memfilter katalog berdasarkan:
  - **Learning Pathway:** Track A, Track B, atau Foundation.
  - **Kategori OWASP:** A01 s/d A10.
  - **Tingkat Kesulitan:** Beginner, Intermediate, Advanced.
  - **Durasi Waktu:** < 45 Menit, 45–60 Menit, > 60 Menit.
  - **Status Paket:** Terpasang (*Installed*), Belum Diunduh (*Not Downloaded*), Pembaruan Tersedia (*Update Available*).
- Setiap lab mencantumkan prasyarat (*prerequisites*), namun pembelajar berpengalaman bebas melompat langsung ke modul yang diminati.
- Kemajuan (*progress*) dicatat per unit (Lesson Read, Practice Verified, Challenge Solved) dan disimpan secara persisten di penyimpanan lokal pengguna.
