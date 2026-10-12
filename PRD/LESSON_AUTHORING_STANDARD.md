# ZITERA_LAB: LESSON AUTHORING & REMEDIATION STANDARD
**Document Reference:** `PRD/LESSON_AUTHORING_STANDARD.md`  
**Phase:** 24 (CP04 — Checkpoint Push #5)  
**Author:** AI Pair Programmer (Software Craftsmanship Active)  
**Status:** EDUCATIONAL AUTHORING STANDARD  
**Target Platform:** Windows x64  
**Primary Language:** Bahasa Indonesia (Secondary: English-ready architecture)  

---

## 1. Executive Summary & Pedagogical Principles

Dokumen ini menetapkan standar kanonikal penulisan materi pembelajaran (*lesson authoring standard*) untuk seluruh laboratorium di ZITERA_LAB. Standar ini memastikan bahwa materi tidak hanya mengajarkan "bagaimana cara meretas" (*exploit-only*), tetapi berfokus pada **rekayasa perangkat lunak defensif (*remediation-first education*)**.

Setiap materi dirancang agar ramah bagi siswa SMK, mahasiswa, dan pemula, menggunakan analogi konkret, contoh masukan/keluaran nyata, serta tahapan verifikasi perbaikan yang teruji secara otomatis.

---

## 2. Struktur Kanonikal Materi Laboratorium (21 Sections) (Section 7.1)

Setiap modul laboratorium lengkap mengikuti 21 struktur bagian berikut (modul *micro-lesson* dapat memadatkan bagian teoritis namun wajib mempertahankan tujuan, praktik, observasi, dan mitigasi):

```text
┌────────────────────────────────────────────────────────────────────────┐
│                        21-SECTION LESSON SCHEMA                        │
├────────────────────────────────┬───────────────────────────────────────┤
│  1. Lab Overview               │ 12. Why the Behavior is Vulnerable    │
│  2. Why this Topic Matters     │ 13. Secure Implementation (Code Fix)  │
│  3. Learning Objectives        │ 14. How to Verify the Fix (Test Case) │
│  4. Prerequisites              │ 15. Common Mistakes & Anti-Patterns   │
│  5. Estimated Duration         │ 16. Challenge Briefing                │
│  6. Required Tools / Runtime   │ 17. Progressive Hints (Tiers 1–3)     │
│  7. Core Concepts              │ 18. Reflection & Knowledge Check      │
│  8. How Vulnerable Logic Works │ 19. Key Takeaways                     │
│  9. Step-by-Step Guided Lesson │ 20. References & Standards            │
│ 10. Hands-on Practice Exercise │ 21. Completion Criteria               │
│ 11. Expected Observations      │                                       │
└────────────────────────────────┴───────────────────────────────────────┘
```

### Rincian Setiap Bagian
1. **Lab Overview:** Ringkasan singkat modul dan skenario aplikasi yang dihadapi.
2. **Why this Topic Matters:** Dampak nyata celah keamanan terhadap kerugian finansial, reputasi bisnis, atau kebocoran privasi pengguna di dunia nyata.
3. **Learning Objectives:** 3–5 butir hasil belajar terukur yang akan dicapai pembelajar.
4. **Prerequisites:** Pengetahuan dasar prasyarat yang wajib dipahami sebelum memulai lab ini.
5. **Estimated Duration:** Perkiraan waktu penyelesaian realistis (30–75 menit) berdasarkan data pengujian.
6. **Required Tools or Environment:** Kebutuhan antarmuka (konsol edukasi, browser bawaan, port loopback).
7. **Core Concepts:** Definisi istilah, diagram arsitektur komponen, dan analogi mental sehari-hari.
8. **How the Vulnerable Behavior Works:** Bedah alur logika atau cuplikan kode yang menyebabkan celah keamanan terjadi.
9. **Step-by-Step Guided Lesson:** Panduan langkah-demi-langkah interaktif untuk mengeksplorasi aplikasi target.
10. **Hands-on Practice:** Instruksi tindakan konkret pada simulator lokal terisolasi.
11. **Expected Observations:** Apa yang harus dilihat oleh pembelajar (kode status HTTP, respons JSON, pesan galat).
12. **Why the Behavior is Vulnerable:** Analisis mendalam mengapa sistem gagal menegakkan batas keamanan (*trust boundary breach*).
13. **Secure Implementation:** Cuplikan kode perbaikan (*clean code patch*) yang menerapkan pertahanan yang kokoh.
14. **How to Verify the Fix:** Cara menulis *automated security test* untuk membuktikan celah tertutup.
15. **Common Mistakes:** Kesalahan fatal pemula (contoh: hanya memfilter di JavaScript klien tanpa validasi server).
16. **Challenge Briefing:** Misi mandiri CTF dengan skenario variasi untuk menguji pemahaman konsep.
17. **Hints, Progressively Revealed:** 3 tingkat petunjuk bantuan terstruktur tanpa membocorkan jawaban langsung.
18. **Reflection and Knowledge Check:** 3 pertanyaan reflektif / kuis konsep untuk menguji pemahaman mendalam.
19. **Key Takeaways:** 3 poin kesimpulan utama yang harus selalu diingat saat membangun aplikasi produksi.
20. **References:** Tautan ke standar resmi OWASP, CWE, NIST, atau dokumentasi bahasa pemrograman terkait.
21. **Completion Criteria:** Syarat kelulusan lab (Lesson dibaca, Practice terverifikasi, Challenge flag diterima).

---

## 3. Pedoman Bahasa & Terminologi Teknis (Section 7.2)

Bahasa Indonesia adalah bahasa pengantar utama ZITERA_LAB dengan ketentuan ketat:
- **Kejelasan & Kesederhanaan:** Gunakan kalimat aktif, lugas, dan instruktif. Hindari gaya bahasa akademis kaku yang membingungkan pemula.
- **Istilah Teknis yang Dipertahankan:** Jangan memaksakan terjemahan harfiah jika justru mengaburkan pemahaman industri:
  - *Gunakan:* HTTP, SQL Injection, Authentication, Authorization, Payload, Hash, Salt, Token, Cookie, Middleware, Endpoint.
  - *Hindari terjemahan rancu seperti:* "Suntikan SQL" (gunakan: *SQL Injection*), "Pemberian Kuasa" (gunakan: *Otorisasi / Authorization*).
- **Koreksi Distingsi Konseptual Kritis:**
  - **Autentikasi vs Otorisasi:** Autentikasi adalah pembuktian identitas (*siapa Anda*); Otorisasi adalah pemeriksaan hak akses (*apa yang boleh Anda lakukan*).
  - **Validasi Input vs Output Encoding:** Validasi input memastikan data sesuai tipe yang diharapkan sebelum diproses; *Output encoding* memastikan data yang dicetak ke antarmuka tidak dieksekusi sebagai kode HTML/JS.
  - **Hashing vs Enkripsi vs Tanda Tangan:** Hashing adalah fungsi satu arah (*one-way fingerprint*); Enkripsi adalah transformasi dua arah dengan kunci rahasia (*two-way confidentiality*); Tanda tangan digital adalah pembuktian keaslian dan integritas menggunakan kunci asimetris (*authenticity & non-repudiation*).

---

## 4. Standar Mutu Latihan Terpandu (*Guided Lesson Quality*) (Section 7.3)

Setiap langkah latihan terpandu (*guided step*) wajib memuat format standar:

```text
[Langkah N: Judul Langkah]
- Tujuan Langkah: Apa yang ingin dibuktikan oleh pembelajar.
- Target & Lokasi: URL endpoint, antarmuka, atau komponen lokal (misal: http://127.0.0.1:8011/invoice/1).
- Masukan / Tindakan: Nilai input spesifik atau tombol yang diklik pengguna.
- Respon yang Diharapkan: Respon sistem aktual (misal: HTTP 200 OK dengan data invoice milik Alice).
- Penjelasan Hasil: Mengapa sistem memberikan respon tersebut dan apa artinya bagi keamanan aplikasi.
- Cara Reset: Langkah untuk mengembalikan simulator ke kondisi awal jika terjadi kesalahan masukan.
```

---

## 5. Pendekatan Edukasi Berbasis Remediasi (*Remediation-First*) (Section 7.4)

Pendidikan keamanan yang bertanggung jawab wajib menanamkan pola pikir pertahanan (*defensive mindset*):
1. **Pengenalan Gejala (*Identify*):** Mengetahui pola kode rentan melalui inspeksi statis atau respons dinamis.
2. **Reproduksi Aman (*Safely Reproduce*):** Membuktikan kelemahan hanya pada *fixture* simulasi lokal yang terisolasi.
3. **Analisis Akar Masalah (*Root Cause*):** Mengidentifikasi apakah kegagalan terjadi di layer arsitektur, konfigurasi, atau implementasi kode.
4. **Implementasi Perbaikan (*Patch*):** Menyediakan contoh kode perbaikan nyata yang sesuai dengan *best practices* industri:
   - Contoh: Mengganti string formatting query menjadi `db.prepare("SELECT * FROM users WHERE id = ?").bind(id)`.
5. **Uji Regresi Otomatis (*Automated Regression Test*):** Menulis unit test atau integration test untuk membuktikan bahwa payload serangan yang sama kini ditolak secara konsisten (contoh: menghasilkan HTTP 403 Forbidden).
