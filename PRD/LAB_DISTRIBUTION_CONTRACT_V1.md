# ZITERA_LAB: MODULAR PACKAGE, DOWNLOAD & OFFLINE CONTRACT (V1)
**Document Reference:** `PRD/LAB_DISTRIBUTION_CONTRACT_V1.md`  
**Phase:** 24 (CP08 — Checkpoint Push #9)  
**Author:** AI Pair Programmer (Software Craftsmanship Active)  
**Status:** CANONICAL DISTRIBUTION & OFFLINE SPECIFICATION  
**Target Platform:** Windows x64  
**Primary Language:** Bahasa Indonesia (Secondary: English-ready architecture)  

---

## 1. Executive Summary & Distribution Architecture

ZITERA_LAB mengadopsi model distribusi **Aplikasi Inti Ringan (*Lightweight Core*) + Paket Laboratorium Modular Opsional (*Optional Downloadable Lab Packages*)**. 

Prinsip dasar distribusi:
- **Ukuran Awal Ringan:** Installer dan bundle inti ZITERA_LAB tidak membebani perangkat pengguna dengan seluruh biner laboratorium sekaligus. Pengguna hanya mengunduh modul yang ingin dipelajari.
- **Kemandirian Paket (.zlab):** Setiap laboratorium dikemas ke dalam arsip biner mandiri berformat `.zlab` yang memuat seluruh aset pelajaran, latihan, tantangan, dan biner eksekusi lokal yang relevan.
- **Keamanan Kriptografis Tanpa Kompromi:** Seluruh paket diverifikasi menggunakan tanda tangan digital asimetris (Ed25519) sebelum diekstraksi ke sistem operasi.
- **Desain Utamakan Luring (*Offline-First*):** Setelah sebuah paket terpasang, seluruh interaksi pembelajaran dapat dilakukan secara mandiri tanpa memerlukan koneksi internet.

---

## 2. Pemisahan Tanggung Jawab (*Separation of Concerns*) (Section 11.1)

```text
┌─────────────────────────────────────────────────────────────────────────────┐
│                       CORE APPLICATION (APLIKASI INTI)                      │
│                                                                             │
│ • Flutter GUI Antarmuka Pengguna      • Ed25519 Trusted Public Keys         │
│ • Rust Background Engine Service      • Catalog, Download & Package Manager │
│ • Sandboxed Lab Process Broker        • Local Progress & State Manager      │
│ • Dynamic Markdown Lesson Renderer    • Component Readiness & Diagnostics   │
│ • Windows Job Object Lifecycle        • Minimal First-Run Assets & Branding │
└──────────────────────────────────────┬──────────────────────────────────────┘
                                       │ Mengelola & Menjalankan
                                       ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│                 OPTIONAL DOWNLOADABLE LAB PACKAGE (.zlab)                   │
│                                                                             │
│ • manifest.json (Spesifikasi Lab)     • challenge/ (Misi & Hints JSON)      │
│ • lesson/ (Markdown Modular ID/EN)    • bin/ (Target Biner Native Terisolasi)│
│ • metadata/ (Keahlian & Rubrik)       • Local Assets (Gambar / Fixture Data)│
└─────────────────────────────────────────────────────────────────────────────┘
```

*Dilarang keras mengemas seluruh berkas lesson dan biner lab ke dalam bundle asset Flutter bawaan.* Asset Flutter dibatasi hanya untuk UI chrome, ikon branding, dan font.

---

## 3. Perilaku Katalog (*Catalog Behavior*) (Section 11.2)

Katalog ZITERA_LAB (`catalog/catalog.json`) dirancang sebagai indeks metadata ringan (< 512 KB):
- **Metadata Lengkap per Lab:**
  - `id`: Kode kanonikal lab (`A01`–`A10`, `W01`, dst.).
  - `package_id`: Nama paket arsip (`zitera-lab-a01`).
  - `title`: Judul resmi laboratorium.
  - `short_description`: Ringkasan 1–2 kalimat.
  - `category_name` & `owasp`: Standar klasifikasi keamanan.
  - `difficulty`: Tingkat kesulitan (`Beginner`, `Intermediate`, `Advanced`).
  - `estimated_time`: Durasi pengerjaan realistis dalam menit.
  - `package_size`: Ukuran berkas unduhan yang terkompresi.
  - `installed_version` vs `available_version`: Status versi paket di disk lokal vs rilis terbaru.
  - `minimum_core_version`: Persyaratan kompatibilitas mesin inti ZITERA.
  - `offline_available`: Ketersediaan penuh saat tidak terhubung ke jaringan.
  - `update_available`: Indikator jika ada pembaruan konten atau keamanan.
- **Efisiensi Memori:** Membuka layar katalog hanya memuat daftar metadata JSON tanpa mengunduh arsip `.zlab` di latar belakang.

---

## 4. Siklus Hidup Unduhan & Pemasangan (*Download Lifecycle*) (Section 11.3)

Alur pemasangan paket laboratorium diatur secara deterministik dengan jaminan integritas:

```text
       ┌────────────────────────┐
       │      Buka Katalog      │ Pengguna menelusuri katalog lab.
       └───────────┬────────────┘
                   ▼
       ┌────────────────────────┐
       │       Pilih Lab        │ Melihat ringkasan, durasi, dan ukuran.
       └───────────┬────────────┘
                   ▼
       ┌────────────────────────┐
       │     Klik Download      │ Pengguna meminta unduhan secara eksplisit.
       └───────────┬────────────┘
                   ▼
       ┌────────────────────────┐
       │   Unduh ke Staging     │ Berkas disimpan ke folder .staging_<uuid>.
       └───────────┬────────────┘
                   ▼
       ┌────────────────────────┐
       │ Validasi Hash SHA-256  │ Memverifikasi integritas checksum berkas.
       └───────────┬────────────┘
                   ▼
       ┌────────────────────────┐
       │  Verifikasi Ed25519    │ Menguji tanda tangan asimetris rilis resmi.
       └───────────┬────────────┘
                   ▼
       ┌────────────────────────┐
       │ Validasi Manifest/Core │ Memeriksa kompatibilitas versi minimum core.
       └───────────┬────────────┘
                   ▼
       ┌────────────────────────┐
       │ Audit Keamanan Arsip   │ Zip-Slip & Path Traversal Check (no '..').
       └───────────┬────────────┘
                   ▼
       ┌────────────────────────┐
       │   Instalasi Atomik     │ Mengganti/memindahkan staging ke paket aktif.
       └───────────┬────────────┘
                   ▼
       ┌────────────────────────┐
       │    Uji Health Check    │ Menjalankan verifikasi kesiapan komponen.
       └───────────┬────────────┘
                   ▼
       ┌────────────────────────┐
       │    Tandai Terpasang    │ Status lab diperbarui menjadi INSTALLED.
       └────────────────────────┘
```

Jika terjadi kegagalan pada salah satu tahap:
- Paket tidak boleh ditandai sebagai *installed*.
- Versi lab lama yang sedang aktif tidak boleh dirusak.
- Direktori *staging* sementara wajib dibersihkan (*clean rollback*).
- Respons HTTP 200 tidak boleh dianggap sebagai jaminan paket valid sebelum uji kriptografi lolos.

---

## 5. Kepercayaan Kriptografis & Keamanan Paket (*Package Trust*) (Section 11.4)

Pemeriksaan keamanan paket mengimplementasikan 11 lapisan verifikasi di `engine/rust/src/package/verifier.rs`:
1. **Batas Ukuran Paket:** Maksimal 100 MB per paket.
2. **Struktur ZIP Standar:** Header End of Central Directory (EOCD) valid.
3. **Validasi Schema Manifest:** `manifest.json` mematuhi `schema_version: 1`.
4. **Validasi Lab ID:** Hanya menerima ID alfabetik-numerik resmi yang terdaftar.
5. **Runtime yang Diizinkan:** Wajib salah satu dari `native_sandboxed`, `mock`.
6. **Kesesuaian Arsitektur:** Wajib `x86_64` atau `any`.
7. **Batas Jumlah Berkas:** Maksimal 1.000 entri per arsip.
8. **Pencegahan Path Traversal:** Menolak entri dengan path `..`, slash absolut (`/` atau `\`), maupun huruf drive Windows (`C:`).
9. **Ringkasan Konten Deterministik (*Deterministic Digest*):** Hash dihitung berurutan berdasarkan nama entri leksikografis.
10. **Integritas Hash Paket:** SHA-256 seluruh arsip dicocokkan dengan katalog.
11. **Tanda Tangan Asimetris Ed25519:** Diverifikasi menggunakan `RELEASE_PUBLIC_KEY` atau `DEV_PUBLIC_KEY`. Kunci privat penandatanganan tidak boleh disertakan pada biner klien.

---

## 6. Perilaku Berorientasi Luring (*Offline-First Behavior*) (Section 11.5)

Ketika paket telah terpasang di komputer pengguna:
- Seluruh teks lesson (Introduction, Concept, Analogy, Walkthrough, Remediation) dapat dibuka secara offline.
- Target latihan dan tantangan CTF berjalan pada loopback lokal (`127.0.0.1:<port>`) tanpa membutuhkan gateway eksternal.
- Kemajuan belajar (*progress record*) disimpan di penyimpanan lokal `%LOCALAPPDATA%\ZiteraLab\storage\progress.json`.
- Ketiadaan koneksi internet tidak boleh menghambat peluncuran lab yang sudah terinstal.
- Mode offline tidak boleh menonaktifkan verifikasi integritas tanda tangan kriptografis paket.

---

## 7. Matriks Uji Unduhan, Pembaruan & Penghapusan (Section 11.6)

18 skenario uji terstandarisasi untuk menjamin kehandalan modul distribusi:

```text
 1. [TEST_DL_SUCCESS]       : Unduhan paket resmi utuh berhasil diverifikasi dan dipasang atomik.
 2. [TEST_DL_INTERRUPT]     : Koneksi terputus di tengah unduhan membersihkan staging tanpa merusak paket aktif.
 3. [TEST_HASH_MISMATCH]    : Perubahan 1 bit data pada berkas unduhan ditolak dengan galat SHA256 mismatch.
 4. [TEST_SIG_MISMATCH]     : Tanda tangan digital yang tidak cocok dengan kunci publik resmi ditolak.
 5. [TEST_MANIFEST_INVALID] : Manifest dengan sintaks JSON rusak atau atribut wajib kosong ditolak.
 6. [TEST_IDENTITY_MISMATCH]: Arsip yang mengaku ID A01 tetapi memuat manifest A02 ditolak.
 7. [TEST_CORE_INCOMPAT]    : Paket yang mewajibkan minimum_core_version > versi lokal ditolak dengan peringatan.
 8. [TEST_DOWNGRADE_ATTACK] : Paket dengan security_version lebih rendah dari yang terpasang ditolak (anti-rollback).
 9. [TEST_ZIP_SLIP_ATTACK]  : Arsip bermuatan path traversal ('../../windows/system32') ditolak keras.
10. [TEST_DISK_FULL_ABORT]  : Simulasi kehabisan ruang disk membatalkan instalasi dan mengembalikan status aman.
11. [TEST_UPDATE_SUCCESS]   : Pembaruan versi (1.0.0 -> 1.0.1) menggantikan berkas lama dengan mulus.
12. [TEST_ROLLBACK_FAIL]    : Kegagalan saat menimpa paket versi baru mengembalikan paket versi lama secara utuh.
13. [TEST_LAB_REMOVAL]      : Fitur hapus lab membersihkan berkas paket dari disk lokal dan membebaskan port.
14. [TEST_PROGRESS_RETAIN]  : Penghapusan paket tidak menghapus riwayat skor/tantangan di progress.json.
15. [TEST_OFFLINE_LAUNCH]   : Lab terpasang dapat diluncurkan saat jaringan dimatikan tanpa galat timeout.
16. [TEST_CATALOG_OFFLINE]  : Saat remote katalog down, katalog cache lokal atau default_catalog tetap aktif.
17. [TEST_DUPLICATE_REQ]    : Mengklik tombol unduh berulang kali tidak memicu dobel download paralel.
18. [TEST_CONCURRENT_LOCK]  : Permintaan instalasi konkuren dikunci secara serial (*mutex queue*).
```
