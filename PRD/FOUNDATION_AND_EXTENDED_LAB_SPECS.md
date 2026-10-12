# ZITERA_LAB: SHARED FOUNDATION & EXTENDED LAB SPECIFICATIONS
**Document Reference:** `PRD/FOUNDATION_AND_EXTENDED_LAB_SPECS.md`  
**Phase:** 24 (CP07 — Checkpoint Push #8)  
**Author:** AI Pair Programmer (Software Craftsmanship Active)  
**Status:** CANONICAL EDUCATIONAL SPECIFICATION  
**Target Platform:** Windows x64  
**Primary Language:** Bahasa Indonesia (Secondary: English-ready architecture)  

---

## 1. Executive Summary

Dokumen ini mendefinisikan spesifikasi kurikulum lengkap untuk dua komponen ekspansi penting di ZITERA_LAB:
1. **Shared Foundation Micro-Lessons (`F01`–`F06`):** Modul pengantar yang menjembatani kesenjangan pengetahuan pemula sebelum mempelajari laboratorium OWASP Top 10. Modul ini dirancang ringan tanpa membutuhkan executable binary berat.
2. **Extended Cybersecurity Labs (Wave 1 Candidates):** 5 laboratorium spesialisasi prioritas di luar OWASP Top 10 (`W01`, `W02`, `W03`, `D01`, `D02`) yang dirancang modular sebagai paket mandiri (`.zlab`).

---

## 2. Spesifikasi Shared Foundation Micro-Lessons (Section 10.1)

Modul fondasi ini disajikan langsung di antarmuka ZITERA_LAB sebagai materi interaktif berbasis data *fixture* dan visualizer UI, tanpa ketergantungan pada port lokal atau binary eksternal:

### F01 — Dasar Protokol HTTP & Web Architecture
- **ID / Paket:** `F01` / `zitera-foundations-http`
- **Tujuan Belajar:**
  1. Membedakan method HTTP (`GET`, `POST`, `PUT`, `DELETE`).
  2. Menganalisis status code (`200`, `301`, `401`, `403`, `404`, `500`).
  3. Membaca struktur header (`Host`, `User-Agent`, `Content-Type`, `Authorization`).
- **Bentuk Praktik:** HTTP Request Inspector di UI untuk menyusun dan mengirim request simulasi.
- **Durasi:** 20 Menit.

### F02 — Cookies, Token, dan Manajemen Sesi
- **ID / Paket:** `F02` / `zitera-foundations-session`
- **Tujuan Belajar:**
  1. Memahami sifat stateless HTTP dan bagaimana sesi dipertahankan.
  2. Memeriksa atribut keamanan cookie (`HttpOnly`, `Secure`, `SameSite`).
  3. Memahami siklus hidup token JWT dan bahaya penyimpanan di LocalStorage tanpa proteksi.
- **Bentuk Praktik:** Visualizer cookie dan simulasi session hijacking via inspeksi token.
- **Durasi:** 25 Menit.

### F03 — Autentikasi vs Otorisasi & Batas Kepercayaan
- **ID / Paket:** `F03` / `zitera-foundations-auth`
- **Tujuan Belajar:**
  1. Menegaskan perbedaan mendasar antara "siapa Anda" (Autentikasi) dan "apa izin Anda" (Otorisasi).
  2. Mengidentifikasi batas kepercayaan (*trust boundary*) antara klien peramban dan server.
  3. Memahami mengapa validasi sisi klien (*client-side validation*) tidak pernah cukup untuk keamanan.
- **Bentuk Praktik:** Diagram interaktif interseptor request yang mendemonstrasikan bypass kontrol klien.
- **Durasi:** 20 Menit.

### F04 — Kriptografi Esensial: Encoding vs Hashing vs Enkripsi
- **ID / Paket:** `F04` / `zitera-foundations-crypto`
- **Tujuan Belajar:**
  1. Menghilangkan kerancuan umum antara Base64, SHA-256, dan AES.
  2. Memahami sifat satu arah (*one-way property*) dari cryptographic hash functions.
  3. Memahami peran salt dalam mencegah serangan tabel pelangi (*rainbow table attack*).
- **Bentuk Praktik:** Sandbox konversi teks instan ke Base64, MD5, SHA-256, dan AES-GCM dengan visualisasi matematis.
- **Durasi:** 25 Menit.

### F05 — Dasar Query Database & Pemisahan Data/Perintah
- **ID / Paket:** `F05` / `zitera-foundations-database`
- **Tujuan Belajar:**
  1. Memahami sintaks dasar SQL (`SELECT`, `INSERT`, `UPDATE`, `DELETE`, `WHERE`).
  2. Memahami bagaimana parser database mengompilasi string query menjadi pohon eksekusi.
  3. Mengamati titik di mana masukan data berbahaya mengubah struktur sintaks query.
- **Bentuk Praktik:** Parser visual interaktif yang menunjukkan perbedaan query terkompilasi vs konkatenasi string.
- **Durasi:** 25 Menit.

### F06 — Mekanisme Lab ZITERA & Pengiriman Flag CTF
- **ID / Paket:** `F06` / `zitera-foundations-onboarding`
- **Tujuan Belajar:**
  1. Memahami alur kerja lab ZITERA (Status Readiness, Practice Verification, Challenge Solved).
  2. Menggunakan terminal edukasi lokal yang aman.
  3. Mengidentifikasi format flag `ZITERA{...}` dan memverifikasi pengiriman flag ke grader.
- **Bentuk Praktik:** Lab orientasi langsung dengan challenge latihan pertama untuk klaim flag orientasi.
- **Durasi:** 15 Menit.

---

## 3. Spesifikasi Kandidat Extended Labs (Wave 1 Launch) (Section 10.2)

Setiap laboratorium baru berikut dirancang mematuhi 17 standar rekayasa kurikulum kanonikal:

---

### Modul W01 — Cross-Site Request Forgery (CSRF) & Token Defense
1. **Stable Proposed ID:** `W01`
2. **Proposed Package ID:** `zitera-lab-w01`
3. **Collection:** Collection A (Web Application Security)
4. **Learning Objectives:**
   - Memahami bagaimana browser secara otomatis menyertakan kredensial/cookie sesi pada request lintas origin.
   - Mereproduksi serangan transfer dana lintas origin tanpa persetujuan pengguna.
   - Mengimplementasikan mitigasi anti-CSRF token sinkron dan atribut `SameSite=Strict` pada cookie.
5. **Prerequisites:** `F01` (HTTP Basics), `F02` (Cookies & Sessions).
6. **Estimated Duration:** 45 Menit.
7. **Lesson Sequence:**
   - Bagian 1: Karakteristik bawaan browser terhadap cookie sesi.
   - Bagian 2: Anatomi serangan formulir tersembunyi (*hidden form auto-submit*).
   - Bagian 3: Dampak pengubahan status aplikasi secara diam-diam.
   - Bagian 4: Cara kerja perlindungan Anti-CSRF Synchronizer Token.
   - Bagian 5: Batasan dan implementasi atribut cookie `SameSite`.
8. **Practice Mechanic:**
   - Menjalankan simulator perbankan lokal pada `127.0.0.1:8021`. Pengguna login sebagai `victim`, lalu membuka halaman pancingan lokal yang mengirimkan request `POST /transfer` tanpa token validasi.
9. **CTF Objective:**
   - Menyusun payload HTML/JavaScript simulasi untuk memicu aksi transfer dana administratif pada akun korban tiruan dan mengekstrak konfirmasi flag.
10. **Flag / Grader Concept:**
    - Flag dihasilkan saat transaksi gelap berhasil dieksekusi: `ZITERA{csrf_anti_token_guard_bypassed}`. Grader memvalidasi mutasi saldo server.
11. **Remediation Exercise:**
    - Menambahkan validasi header `X-CSRF-Token` pada handler backend Rust dan menguji bahwa request lintas origin tanpa token ditolak dengan HTTP 403.
12. **Runtime Requirement:** Native micro-service `8021` (`native_sandboxed`).
13. **Estimated Package Footprint:** ~3.2 MB (biner lab + konten markdown).
14. **Test Matrix:** Valid token diterima, missing token ditolak, cross-origin request terblokir saat `SameSite=Strict`.
15. **Safety Boundary:** Berjalan strictly pada localhost port `8021`, dilarang memicu koneksi internet eksternal.
16. **Dependency on Core:** Menggunakan runtime broker dan Windows Job Object existing.
17. **Implementation Priority:** Prioritas Tinggi (Wave 1).

---

### Modul W02 — Server-Side Request Forgery (SSRF) in Isolated Environment
1. **Stable Proposed ID:** `W02`
2. **Proposed Package ID:** `zitera-lab-w02`
3. **Collection:** Collection A (Web Application Security)
4. **Learning Objectives:**
   - Memahami risiko ketika server mengambil resource eksternal berdasarkan URL yang ditentukan pengguna.
   - Mengeksploitasi fitur fetch URL untuk memindai port internal lokal yang tidak tereskpos ke publik.
   - Menerapkan daftar putih IP (*IP allowlisting*) dan menolak IP loopback/private (RFC 1918).
5. **Prerequisites:** `F01` (HTTP), `A01` (Access Control).
6. **Estimated Duration:** 50 Menit.
7. **Lesson Sequence:**
   - Bagian 1: Konsep fitur webhook, proxy pratinjau tautan, dan pengunduh gambar.
   - Bagian 2: Batas kepercayaan antara antarmuka publik dan jaringan internal privat.
   - Bagian 3: Bahaya akses ke metadata service lokal (`127.0.0.1:8022/internal/admin`).
   - Bagian 4: Mitigasi validasi URL, pemblokiran DNS rebinding, dan isolasi jaringan.
8. **Practice Mechanic:**
   - Simulator webhook fetcher di `127.0.0.1:8022`. Pengguna meminta server mengunduh URL `http://127.0.0.1:8022/internal/vault` yang secara normal ditolak jika diakses langsung oleh peramban luar.
9. **CTF Objective:**
   - Menemukan skema URL atau parameter loopback bypass untuk mengakses endpoint vault internal rahasia server.
10. **Flag / Grader Concept:**
    - Server internal memvalidasi bahwa pemanggil adalah localhost loopback internal dan mengembalikan flag: `ZITERA{ssrf_internal_metadata_breach}`.
11. **Remediation Exercise:**
    - Mengonfigurasi parser URL backend untuk menolak skema `file://` dan blok subnet privat (`127.0.0.0/8`, `10.0.0.0/8`, `192.168.0.0/16`).
12. **Runtime Requirement:** Native micro-service `8022` (`native_sandboxed`).
13. **Estimated Package Footprint:** ~3.4 MB.
14. **Test Matrix:** External URL lolos, private IP terblokir, loopback bypass terdeteksi.
15. **Safety Boundary:** Semua target request diarahkan ke internal loopback port `8022` saja. Dilarang mengirim paket ke LAN atau WAN pengguna.
16. **Dependency on Core:** Zero core modification.
17. **Implementation Priority:** Prioritas Tinggi (Wave 1).

---

### Modul W03 — Broken Object Level Authorization (API BOLA)
1. **Stable Proposed ID:** `W03`
2. **Proposed Package ID:** `zitera-lab-w03`
3. **Collection:** Collection B (API Security)
4. **Learning Objectives:**
   - Mengidentifikasi kelemahan otorisasi berbasis objek pada REST API modern (`/api/v1/orders/{uuid}`).
   - Memahami mengapa API sering kali mengandalkan token bearer tanpa memvalidasi kepemilikan record spesifik.
   - Menerapkan kontrol otorisasi tingkat data (*record-level authorization check*).
5. **Prerequisites:** `F01` (HTTP REST), `A01` (Broken Access Control Basics).
6. **Estimated Duration:** 40 Menit.
7. **Lesson Sequence:**
   - Bagian 1: Desain RESTful API modern dan penggunaan identifier objek pada URL.
   - Bagian 2: Autentikasi vs Otorisasi pada arsitektur API microservices.
   - Bagian 3: Skenario manipulasi ID pesanan atau ID rekam medis.
   - Bagian 4: Implementasi policy engine otorisasi di tingkat repository/service layer.
8. **Practice Mechanic:**
   - API e-commerce lokal di `127.0.0.1:8023`. Pengguna login sebagai `user_alpha` dan memanggil `/api/orders/order_1001`. Pengguna memanipulasi request menjadi `/api/orders/order_1002`.
9. **CTF Objective:**
   - Menemukan nomor order milik pengguna VIP yang memuat item transaksi khusus berisikan flag.
10. **Flag / Grader Concept:**
    - Flag diekstraksi dari payload JSON order 1002: `ZITERA{api_bola_object_level_breached}`.
11. **Remediation Exercise:**
    - Menulis middleware otorisasi: `SELECT * FROM orders WHERE id = :order_id AND user_id = :auth_user_id`.
12. **Runtime Requirement:** Native micro-service `8023` (`native_sandboxed`).
13. **Estimated Package Footprint:** ~3.1 MB.
14. **Test Matrix:** Akses order sendiri berhasil (200), akses order orang lain gagal (403/404), token invalid ditolak (401).
15. **Safety Boundary:** Berjalan strictly lokal pada localhost `8023`.
16. **Dependency on Core:** Standar package contract.
17. **Implementation Priority:** Prioritas Tinggi (Wave 1).

---

### Modul D01 — Web Log Security Audit & Threat Detection
1. **Stable Proposed ID:** `D01`
2. **Proposed Package ID:** `zitera-lab-d01`
3. **Collection:** Collection G (Defensive Security & Incident Analysis)
4. **Learning Objectives:**
   - Menganalisis berkas log web server (format Nginx/Apache Combined Log).
   - Mengidentifikasi pola pemindaian otomatis, serangan injeksi SQL, dan path traversal dari rekaman log.
   - Menghitung frekuensi serangan dan merekonstruksi garis waktu insiden (*incident timeline*).
5. **Prerequisites:** `F01` (HTTP Basics), `A09` (Security Logging).
6. **Estimated Duration:** 45 Menit.
7. **Lesson Sequence:**
   - Bagian 1: Anatomi satu baris catatan log web (IP, Timestamp, Method, URI, Status, User-Agent).
   - Bagian 2: Ciri-ciri pola request normal vs pola scanning botnet / scanner kerentanan.
   - Bagian 3: Teknik investigasi log menggunakan perintah terminal filter (`grep`, `awk`, `cut`, `sort`, `uniq`).
   - Bagian 4: Menentukan dampak insiden: membedakan percobaan yang gagal (HTTP 404/403) dari eksploitasi yang berhasil (HTTP 200).
8. **Practice Mechanic:**
   - Simulator log analyzer lokal di `127.0.0.1:8024` yang menyajikan dataset log insiden sintetis (5.000 entri).
9. **CTF Objective:**
   - Mengidentifikasi alamat IP penyerang dan parameter endpoint rahasia yang berhasil dibobol oleh penyerang untuk menemukan bukti flag.
10. **Flag / Grader Concept:**
    - Flag tersimpan pada parameter respons dari entri log serangan yang berhasil: `ZITERA{log_forensics_analyst_mastery}`.
11. **Remediation Exercise:**
    - Merumuskan aturan deteksi (misal: alert jika IP yang sama memicu > 20 error 404 dalam 1 menit).
12. **Runtime Requirement:** Native micro-service `8024` (`native_sandboxed`).
13. **Estimated Package Footprint:** ~3.0 MB.
14. **Test Matrix:** Parsing log valid, pencarian regex aman, tidak ada kebocoran memory buffer.
15. **Safety Boundary:** Log bersifat sintetis tanpa data pengguna asli.
16. **Dependency on Core:** Terminal emulator widget integrasi.
17. **Implementation Priority:** Prioritas Sedang (Wave 1).

---

### Modul D02 — File & Package Cryptographic Integrity Verification
1. **Stable Proposed ID:** `D02`
2. **Proposed Package ID:** `zitera-lab-d02`
3. **Collection:** Collection I (Forensics & Data Integrity)
4. **Learning Objectives:**
   - Memahami mengapa ukuran berkas (*file size*) saja tidak menjamin keaslian program.
   - Memverifikasi ringkasan hash SHA-256 dan mendeteksi perubahan sekecil 1 bit (*avalanche effect*).
   - Memverifikasi tanda tangan digital asimetris (Ed25519) menggunakan kunci publik resmi.
5. **Prerequisites:** `F04` (Hashing & Crypto Basics), `A08` (Data Integrity).
6. **Estimated Duration:** 45 Menit.
7. **Lesson Sequence:**
   - Bagian 1: Prinsip integritas berkas dan ancaman *man-in-the-middle* (MitM) saat unduhan.
   - Bagian 2: Cara menghitung dan membandingkan checksum SHA-256 berkas biner.
   - Bagian 3: Kelemahan checksum murni jika file checksum ikut dipalsukan penyerang.
   - Bagian 4: Mengapa tanda tangan digital kunci asimetris adalah solusi definitif otentisitas.
8. **Practice Mechanic:**
   - Simulator package manager lokal di `127.0.0.1:8025`. Pengguna diberikan 3 berkas paket pembaruan: Paket Asli, Paket Rusak, dan Paket Dimanipulasi Penyerang.
9. **CTF Objective:**
   - Mengaudit ketiga paket menggunakan perintah verifikasi kriptografis, menolak paket palsu, dan menginstal hanya paket otentik yang divalidasi kunci publik resmi untuk membuka flag.
10. **Flag / Grader Concept:**
    - Paket otentik yang berhasil diverifikasi dan diekstraksi mengeluarkan flag: `ZITERA{cryptographic_authenticity_proven}`.
11. **Remediation Exercise:**
    - Menulis fungsi verifikasi otomatis: `verify_signature(data, signature, public_key) == true`.
12. **Runtime Requirement:** Native micro-service `8025` (`native_sandboxed`).
13. **Estimated Package Footprint:** ~3.2 MB.
14. **Test Matrix:** Paket valid lolos, corrupt checksum gagal, tanda tangan palsu ditolak.
15. **Safety Boundary:** Eksekusi strictly di dalam sandbox memori terisolasi.
16. **Dependency on Core:** Menggunakan pustaka kriptografi `ed25519_dalek` existing di engine.
17. **Implementation Priority:** Prioritas Sedang (Wave 1).
