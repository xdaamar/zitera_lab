# ZITERA_LAB: CURRICULUM UX, PROGRESS & RESOURCE EFFICIENCY SPECIFICATION
**Document Reference:** `PRD/CURRICULUM_UX_AND_RESOURCE_EFFICIENCY.md`  
**Phase:** 24 (CP11 — Checkpoint Push #12)  
**Author:** AI Pair Programmer (Software Craftsmanship Active)  
**Status:** CANONICAL UX & PERFORMANCE SPECIFICATION  
**Target Platform:** Windows x64  
**Primary Language:** Bahasa Indonesia (Secondary: English-ready architecture)  

---

## 1. Executive Summary & UI Scope

Dokumen ini mendefinisikan arsitektur antarmuka pengguna (*learner experience*), model siklus hidup progres belajar (*progress lifecycle*), dan efisiensi alokasi sumber daya (*resource efficiency*) pada aplikasi ZITERA_LAB.

Sesuai arahan Section 14 pada `dev_internal/sprints/master_pormt.md`, perbaikan berfokus pada **perubahan minimal dan terarah (*minimal targeted enhancements*)** pada model rendering konten Flutter yang ada, tanpa merombak arsitektur antarmuka secara keseluruhan.

---

## 2. Pengalaman Belajar pada Detail Lab (*Lab Detail Learner Experience*)

Halaman detail laboratorium (`LabDetailView`) harus menyajikan informasi yang transparan dan terstruktur sebelum pembelajar menekan tombol "Start":

```text
┌────────────────────────────────────────────────────────────────────────┐
│ [← Kembali]    A01: BROKEN ACCESS CONTROL                [Terpasang]   │
├────────────────────────────────────────────────────────────────────────┤
│ • Apa yang Dipelajari : IDOR, Horizontal Privilege Escalation, Otorisasi│
│ • Prasyarat           : Dasar Protokol HTTP, Konsep Sesi & Cookie      │
│ • Estimasi Durasi     : 35 Menit                                       │
│ • Status Paket        : Siap Luring (Installed: 1.0.1, Port: 8011)      │
│ • Kebutuhan Sumber Daya: 256 MB RAM, Loopback Port 8011                │
├────────────────────────────────────────────────────────────────────────┤
│ [ Tab 1: Lesson ]   [ Tab 2: Practice ]   [ Tab 3: CTF Challenge ]    │
│  6 Bagian Modular    Simulator Invoice     Ekstraksi Invoice ID 42     │
│  (Teori & Mitigasi)  Lokal Port 8011       Penilaian Otomatis Grader   │
└────────────────────────────────────────────────────────────────────────┘
```

### Komponen Wajib Halaman Detail
1. **Ringkasan Kompetensi (*What will be learned*):** Menampilkan daftar butir `learning_objectives` dan `skills` secara eksplisit.
2. **Prasyarat Pembelajaran (*Prerequisites*):** Menyebutkan topik dasar yang harus dikuasai (misal: "Disarankan menyelesaikan modul Fondasi F01 terlebih dahulu").
3. **Anggaran Waktu (*Time Budget*):** Durasi realistis (contoh: 35 menit) agar pembelajar dapat mengalokasikan waktu fokus dengan baik.
4. **Status Siklus Hidup Paket (*Package Lifecycle State*):** Menampilkan dengan jelas apakah paket sudah terpasang, perlu diunduh, atau memiliki pembaruan.
5. **Konektivitas Edukasi (Lesson -> Practice -> Challenge -> Remediation):** Tab navigasi yang secara visual menunjukkan tahapan progres belajar pembelajar.

---

## 3. Efisiensi Sumber Daya & Kinerja Memori (*Resource Efficiency*)

Aplikasi ZITERA_LAB didesain untuk berjalan lancar di komputer siswa SMK atau laptop standar tanpa menguras memori:

1. **Pemuatan Konten Sesuai Kebutuhan (*On-Demand Lazy Loading*):**
   - Berkas Markdown lesson hanya dibaca ke memori saat pengguna membuka halaman detail lab tertentu (`get_lab_content(labId)`).
   - Seluruh konten Markdown 10 lab tidak boleh dimuat ke memori saat startup aplikasi.
2. **Katalog Metadata Ringan (*Lightweight Catalog Index*):**
   - Katalog berukuran < 512 KB, hanya memuat ringkasan metadata yang dibutuhkan untuk kartu daftar lab.
3. **Pemisahan Aset Inti dari Paket Unduhan:**
   - Konten paket `.zlab` tidak boleh ditanamkan (*embedded*) ke dalam berkas asset biner Flutter (`pubspec.yaml assets/`).
4. **Log Diagnostik Terbatas (*Bounded Diagnostics*):**
   - Log konsol edukasi dan riwayat diagnostik engine dibatasi maksimal 1.000 baris melingkar (*circular buffer*) untuk mencegah lonjakan RAM.
5. **Kerahasiaan Progres Lokal (*Sanitized Local State*):**
   - Berkas `progress.json` hanya menyimpan ID lab yang telah selesai dan timestamp. Nilai secret flag tidak boleh disimpan di `progress.json`.

---

## 4. Semantik Progres & Status Siklus Hidup Paket

Untuk mencegah kerancuan antara **status file di disk** dan **prestasi belajar pengguna**, status dibagi menjadi dua model terpisah:

### 4.1 Status Siklus Hidup Paket (*Package Lifecycle States*)
```text
NOT_DOWNLOADED     ──► Paket terdaftar di katalog tetapi belum ada di disk lokal.
DOWNLOADING        ──► Paket sedang diunduh ke staging sementara.
INSTALLED          ──► Paket diverifikasi kriptografis dan siap dijalankan.
AVAILABLE_OFFLINE  ──► Paket terpasang dan dapat dijalankan tanpa internet.
UPDATE_AVAILABLE   ──► Versi rilis katalog lebih baru dari versi terpasang lokal.
REPAIR_REQUIRED    ──► Berkas biner atau manifest terkorupsi / checksum mismatch.
```

### 4.2 Status Prestasi Belajar (*Learner Progress States*)
```text
NOT_STARTED        ──► Pembelajar belum membuka lab ini.
IN_PROGRESS        ──► Pembelajar telah membuka lesson dan membaca teori.
PRACTICE_COMPLETED ──► Latihan simulator lokal berhasil diverifikasi.
CHALLENGE_SOLVED   ──► Flag resmi CTF berhasil diverifikasi oleh grader.
COMPLETED          ──► Seluruh lesson dibaca, latihan selesai, dan flag terverifikasi.
```

*Prinsip Integritas Data:* Menghapus paket dari disk (*Uninstall Lab*) tidak boleh menghapus catatan kemajuan belajar pembelajar (*Progress Retention Guarantee*).
