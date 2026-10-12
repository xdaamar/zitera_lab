# ZITERA_LAB: OWASP TOP 10:2025 CURRICULUM BLUEPRINT
**Document Reference:** `PRD/OWASP_TOP10_2025_CURRICULUM_BLUEPRINT.md`  
**Phase:** 24 (CP02 — Checkpoint Push #3)  
**Author:** AI Pair Programmer (Software Craftsmanship Active)  
**Status:** CURRICULUM BLUEPRINT SPECIFICATION  
**Target Platform:** Windows x64  
**Primary Language:** Bahasa Indonesia (Secondary: English-ready architecture)  
**Official Reference:** [OWASP Top 10:2025 Standard](https://top10.owasp.org/2025/0x00_2025-Introduction/)  

---

## 1. Executive Summary

Dokumen ini mendefinisikan cetak biru (*blueprint*) kurikulum ZITERA_LAB untuk koleksi **OWASP Top 10:2025**. Koleksi ini menjadi salah satu pilar inti pembelajaran keamanan aplikasi web dan sistem, yang dipetakan secara terstandarisasi ke dalam 10 modul laboratorium native (`A01` hingga `A10`).

Setiap modul dihubungkan dengan simulasi lokal native terisolasi, latihan interaktif (*guided practice*), tantangan mandiri berbasis CTF (*capture-the-flag*), petunjuk progresif (*progressive hints*), serta teknik remediasi dan pengujian regresi (*security regression testing*).

---

## 2. Cakupan Mendalam 10 Kategori OWASP Top 10:2025 (Section 5.1)

### A01:2025 — Broken Access Control
- **Konsep Utama:** Kegagalan penegakan aturan otorisasi di sisi server yang memungkinkan pengguna mengakses, memodifikasi, atau menghapus data yang bukan haknya.
- **Topik Pembahasan:**
  - Perbedaan mendasar antara Autentikasi (*Authentication*) dan Otorisasi (*Authorization*).
  - Eskalasi hak akses horizontal (*Horizontal Privilege Escalation* / IDOR / BOLA).
  - Eskalasi hak akses vertikal (*Vertical Privilege Escalation* ke peran Administrator).
  - Melewati pemeriksaan kontrol akses melalui manipulasi parameter URL, *path traversal*, atau header HTTP.
  - Batas kepercayaan (*trust boundary*) antara antarmuka klien dan backend.
- **Strategi Mitigasi:** Otorisasi wajib di sisi server, prinsip *deny-by-default*, pemeriksaan kepemilikan objek (*record ownership check*), dan validasi peran (*role-based access control / RBAC*).

### A02:2025 — Security Misconfiguration
- **Konsep Utama:** Pengaturan konfigurasi server, aplikasi, atau middleware yang tidak aman atau dibiarkan pada setelan *default* pabrik.
- **Topik Pembahasan:**
  - Penggunaan kredensial bawaan (*default credentials*) pada akun administratif.
  - Kebocoran informasi sensitif melalui pesan galat (*stack trace / debug leaks*).
  - Konfigurasi CORS (*Cross-Origin Resource Sharing*) yang terlalu permisif (`*` dengan credentials).
  - Tidak adanya header keamanan penting (`Content-Security-Policy`, `X-Content-Type-Options`, `Strict-Transport-Security`).
  - *Directory listing* yang aktif sehingga mengekspos berkas cadangan (*backup.sql*, *.env*).
- **Strategi Mitigasi:** Penerapan *secure defaults*, penonaktifan mode debug di produksi, penutupan *directory browsing*, dan audit konfigurasi berkala.

### A03:2025 — Software Supply Chain Failures
- **Konsep Utama:** Kerentanan yang masuk melalui pustaka (*libraries*), ketergantungan pihak ketiga (*dependencies*), atau manipulasi rantai pasok perangkat lunak.
- **Topik Pembahasan:**
  - Batas kepercayaan pada repositori publik (npm, crates.io, PyPI).
  - Risiko ketergantungan usang atau yang memiliki CVE kritis yang diketahui publik.
  - Konsep *typosquatting* dan *dependency confusion*.
  - Peran penting berkas *lockfile* (`Cargo.lock`, `package-lock.json`, `pubspec.lock`) dan *version pinning*.
  - Integritas artefak biner dan *Software Bill of Materials* (SBOM).
- **Strategi Mitigasi:** Pemeriksaan integritas hash artefak, verifikasi tanda tangan digital paket, pembaruan otomatis yang terkontrol, dan audit kerentanan berkala.

### A04:2025 — Cryptographic Failures
- **Konsep Utama:** Kegagalan dalam melindungi kerahasiaan dan integritas data sensitif melalui penggunaan kriptografi yang salah atau tidak memadai.
- **Topik Pembahasan:**
  - Klasifikasi data sensitif (data dalam transit vs data saat istirahat / *at rest*).
  - Perbedaan esensial antara Hashing, Enkripsi Simetris/Asimetris, dan Tanda Tangan Digital.
  - Bahaya penyimpanan kata sandi teks polos atau menggunakan algoritma usang (MD5, SHA1 tanpa salt).
  - Penggunaan kunci rahasia yang dikodekan secara kaku (*hardcoded secrets*) di dalam repositori kode.
  - Pembangkit bilangan acak yang tidak aman (*insecure pseudo-random number generators*).
- **Strategi Mitigasi:** Penggunaan fungsi derivasi kunci modern (Argon2id, bcrypt), manajemen rahasia terisolasi (*environment variables / secret manager*), dan protokol TLS 1.3.

### A05:2025 — Injection
- **Konsep Utama:** Penerjemahan data yang dikendalikan oleh penyerang sebagai perintah eksekusi atau struktur sintaks oleh mesin penerjemah.
- **Topik Pembahasan:**
  - Prinsip pemisahan data murni vs struktur perintah eksekusi (*data vs code separation*).
  - SQL Injection klasik (bypass login `' OR '1'='1`) dan ekstraksi data berbasis UNION.
  - Cross-Site Scripting (Reflected & Stored XSS) melalui kelalaian *output encoding*.
  - Command Injection konsep dalam target simulasi lokal terisolasi.
  - Template Injection pada mesin rendering data.
- **Strategi Mitigasi:** Penggunaan *Parameterized Queries / Prepared Statements*, validasi tipe data input, dan *context-aware output encoding* (HTML entity, JavaScript escaping).

### A06:2025 — Insecure Design
- **Konsep Utama:** Cacat mendasar pada arsitektur perangkat lunak dan desain logika bisnis yang tidak dapat diperbaiki hanya dengan penulisan kode sintaks yang sempurna.
- **Topik Pembahasan:**
  - Pemodelan ancaman (*threat modeling*) dan identifikasi skenario penyalahgunaan (*abuse cases*).
  - Cacat logika transaksi (contoh: nilai transfer negatif, manipulasi urutan checkout *e-commerce*).
  - Ketiadaan batasan frekuensi operasi (*missing rate-limiting design*).
  - Kegagalan memvalidasi prasyarat status (*state machine invariant bypass*).
  - Prinsip pertahanan berlapis (*defense-in-depth*).
- **Strategi Mitigasi:** Perancangan kontrol keamanan sebelum pengkodean, penegakan invarian bisnis di tingkat domain model, dan pengujian berbasis kasus penyalahgunaan.

### A07:2025 — Authentication Failures
- **Konsep Utama:** Kelemahan pada proses verifikasi identitas pengguna yang memungkinkan penyerang mengambil alih akun (*account takeover*).
- **Topik Pembahasan:**
  - Penyusupan sesi (*session hijacking*) dan fiksasi sesi (*session fixation*).
  - Ketiadaan perlindungan terhadap tebakan kata sandi otomatis (*credential stuffing & brute-force*).
  - Kebocoran keberadaan akun melalui pesan galat (*account enumeration*).
  - Kelemahan pada alur pemulihan kata sandi (*insecure password reset workflow*).
  - Konsep Multi-Factor Authentication (MFA) dan siklus hidup masa berlaku token sesi.
- **Strategi Mitigasi:** Kebijakan penguncian akun progresif (*progressive delays*), pembuatan ulang session ID setelah login, penyeragaman pesan galat autentikasi, dan MFA.

### A08:2025 — Software or Data Integrity Failures
- **Konsep Utama:** Asumsi yang salah bahwa kode atau data terstruktur yang dibaca dari disk atau jaringan dapat dipercaya tanpa verifikasi kriptografis.
- **Topik Pembahasan:**
  - Perbedaan antara integritas (hash) dan keaslian/otensitas (tanda tangan digital).
  - Bahaya menerima data serialisasi objek tak terpercaya (*insecure deserialization*).
  - Risiko pembaruan otomatis aplikasi tanpa verifikasi tanda tangan digital biner.
  - Manipulasi berkas konfigurasi firmware atau lisensi offline di disk.
- **Strategi Mitigasi:** Verifikasi tanda tangan digital asimetris (Ed25519), penggunaan format serialisasi data aman (JSON murni tanpa instantiasi objek dinamis), dan *strict cryptographic checksum verification*.

### A09:2025 — Security Logging & Alerting Failures
- **Konsep Utama:** Ketidakmampuan mendeteksi, merespons, dan menginvestigasi insiden keamanan akibat ketiadaan catatan audit (*audit trail*) atau peringatan dini.
- **Topik Pembahasan:**
  - Kegagalan mencatat peristiwa penting (login gagal berulang, perubahan hak akses admin, transaksi bernilai besar).
  - Redaksi data sensitif: mencegah kata sandi, token, atau nomor kartu kredit tercatat di log publik.
  - Serangan injeksi log (*log injection / CRLF injection*) yang merusak integritas rekaman.
  - Ambang batas peringatan (*alerting thresholds*) dan investigasi garis waktu insiden (*incident timeline*).
- **Strategi Mitigasi:** Pencatatan log terstruktur (JSON format), sanitasi karakter newline pada input log, redaksi otomatis data rahasia, dan pemantauan aktif.

### A10:2025 — Mishandling of Exceptional Conditions
- **Konsep Utama:** Penanganan galat (*exception handling*) yang tidak konsisten yang menyebabkan sistem beralih ke status tidak aman (*fail-open*) atau membocorkan kondisi internal.
- **Topik Pembahasan:**
  - Paradigma *Fail-Open* (sistem membuka akses saat terjadi error) vs *Fail-Closed* (sistem mengunci akses saat terjadi error).
  - Penanganan batas waktu (*timeout handling*) dan kegagalan parsial pada panggilan layanan mikro.
  - Kondisi inkonsistensi data ketika operasi gagal di tengah jalan tanpa *rollback* transaksi.
  - Pemulihan sumber daya (*resource exhaustion & memory leaks*) saat menghadapi masukan tak terduga.
- **Strategi Mitigasi:** Penegakan prinsip *fail-safe defaults*, transaksi atomik dengan *rollback*, penanganan timeout eksplisit, dan pengujian ketahanan terhadap kegagalan komponen (*chaos engineering basics*).

---

## 3. Matriks Laboratorium Lengkap (Section 5.2)

Berikut matriks kurikulum detail untuk 10 laboratorium OWASP Top 10:2025:

| ID | Paket ID | Konsep Utama & Target | Profil & Prasyarat | Tujuan Belajar Terukur | Praktik Terpandu | Tantangan CTF | Remediasi | Durasi | Port & Runtime |
|---|---|---|---|---|---|---|---|---|---|
| **A01** | `zitera-lab-a01` | Broken Access Control (IDOR, Horiz. Escalation) | Pemula. Prasyarat: HTTP basics, cookies. | 1. Deteksi IDOR.<br>2. Audit batas tenant.<br>3. Terapkan otorisasi server. | Eksplorasi endpoint tagihan `/invoice/1` vs `/invoice/2` via manipulasi parameter ID. | Akses dokumen administratif rahasia milik pengguna lain untuk menemukan flag. | Tambahkan verifikasi `session.user_id == record.owner_id` dan tes regresi unit. | 35m | 8011 (`native_sandboxed`) |
| **A02** | `zitera-lab-a02` | Security Misconfiguration (Exposed Endpoints & Defaults) | Pemula. Prasyarat: Konsep web server & routing. | 1. Audit endpoint diagnostik.<br>2. Identifikasi kebocoran berkas cadangan.<br>3. Amankan setelan default. | Akses antarmuka `/debug/vars`, `/admin` dengan kredensial default, dan direktori cadangan. | Temukan kredensial database tersembunyi pada berkas cadangan publik untuk ekstrak flag. | Nonaktifkan mode debug, tutup *directory listing*, dan ganti kredensial default. | 35m | 8012 (`native_sandboxed`) |
| **A03** | `zitera-lab-a03` | Software Supply Chain Failures (Dependency Tampering) | Pemula–Menengah. Prasyarat: Lockfiles, paket dependensi. | 1. Analisis integritas lockfile.<br>2. Deteksi versi paket palsu.<br>3. Terapkan validasi checksum. | Analisis manifest dependensi tiruan yang memuat pustaka tidak terverifikasi. | Identifikasi paket pihak ketiga yang disusupi dan buktikan ketidaksesuaian hash. | Terapkan *hash pinning* dan validasi tanda tangan asal usul paket (*provenance*). | 45m | 8013 (`native_sandboxed`) |
| **A04** | `zitera-lab-a04` | Cryptographic Failures (Plaintext & Weak Hashes) | Pemula. Prasyarat: Hashing vs Enkripsi. | 1. Klasifikasi data sensitif.<br>2. Audit penyimpanan password.<br>3. Pilih algoritma modern. | Inspeksi tabel pengguna tiruan yang menyimpan password dengan hashing lemah tanpa salt. | Pecahkan hash kata sandi administratif lemah untuk login dan klaim flag. | Migrasikan algoritma ke Argon2id/bcrypt dengan salt acak unik per akun. | 45m | 8014 (`native_sandboxed`) |
| **A05** | `zitera-lab-a05` | Injection (SQL Injection & Query Logic Flaw) | Pemula–Menengah. Prasyarat: Sintaks query database dasar. | 1. Bedakan data vs kode.<br>2. Identifikasi query concatenation.<br>3. Terapkan prepared statements. | Masukkan payload kutip tunggal `'` pada form pencarian dan amati pesan galat sintaks SQL. | Manipulasi klausul WHERE autentikasi menggunakan payload SQL injection untuk login admin. | Ganti query konkatenasi string dengan *Parameterized Queries* / *Prepared Statements*. | 50m | 8015 (`native_sandboxed`) |
| **A06** | `zitera-lab-a06` | Insecure Design (Business Logic & State Invariant) | Menengah. Prasyarat: State machine, logika transaksi. | 1. Susun threat model logika.<br>2. Temukan bypass urutan state.<br>3. Buat invarian transaksi aman. | Analisis alur checkout multi-tahap di mana diskon kupon dapat diterapkan berulang kali. | Picu status inkonsistensi saldo kredit dompet untuk membeli item berharga tanpa saldo cukup. | Terapkan validasi invarian pada model domain dan proteksi atomic commit. | 65m | 8016 (`native_sandboxed`) |
| **A07** | `zitera-lab-a07` | Authentication Failures (Brute Force & Session Fixation) | Pemula. Prasyarat: Form login, cookie session. | 1. Uji ketahanan brute force.<br>2. Deteksi fiksasi sesi.<br>3. Terapkan rate limiting. | Jalankan tebakan berulang pada endpoint login dan amati ketiadaan jeda/blokir. | Manfaatkan ketiadaan batasan login untuk menemukan PIN rahasia 4 digit dan raih flag. | Terapkan delay progresif, regenerasi session ID saat login, dan pesan galat netral. | 40m | 8017 (`native_sandboxed`) |
| **A08** | `zitera-lab-a08` | Software or Data Integrity (Digital Signatures & Tampering) | Menengah. Prasyarat: Asymmetric signing, serialisasi. | 1. Verifikasi tanda tangan digital.<br>2. Cegah modifikasi biner/konfigurasi.<br>3. Tolak payload tak bertanda tangan. | Ubah parameter lisensi di disk dan amati apakah simulator menerima berkas tanpa validasi tanda tangan. | Manipulasi berkas konfigurasi hak akses offline untuk memicu pembukaan brankas flag. | Terapkan verifikasi tanda tangan kriptografis Ed25519 sebelum membaca berkas konfigurasi. | 60m | 8018 (`native_sandboxed`) |
| **A09** | `zitera-lab-a09` | Security Logging & Alerting (Blind Spots & Log Injection) | Menengah. Prasyarat: Manajemen log & audit event. | 1. Identifikasi blind spot audit.<br>2. Redaksikan data rahasia.<br>3. Cegah injeksi pemalsuan log. | Simulasikan operasi administratif sensitif dan periksa apakah tercatat di log audit. | Temukan aktivitas eksfiltrasi tersembunyi yang sengaja tidak dicatat oleh audit trail simulator. | Implementasikan event audit komprehensif, sanitasi karakter newline, dan filter redaksi. | 50m | 8019 (`native_sandboxed`) |
| **A10** | `zitera-lab-a10` | Mishandling Exceptional Conditions (Fail-Open Bypass) | Menengah. Prasyarat: Penanganan exception & timeout. | 1. Bedakan fail-open vs fail-closed.<br>2. Uji kondisi timeout gateway.<br>3. Amankan alur fallback. | Picu error 500 pada auth gate tiruan dan amati bahwa sistem secara keliru meloloskan request. | Simulasikan kondisi network timeout pada layanan otorisasi eksternal untuk lewati gerbang flag. | Rancang alur penanganan error agar selalu *deny-by-default* (*fail-closed*) saat upstream error. | 65m | 8020 (`native_sandboxed`) |

---

## 4. Rencana Validasi Pengujian Terotomasi (Automated Validation)

Setiap modul wajib divalidasi melalui 3 lapisan pengujian statis dan kontrak:
1. **Manifest Integrity Test:** Memverifikasi seluruh atribut JSON manifest valid, field tidak kosong, port unik, dan range durasi realistis (30–75m).
2. **Content Coverage Test:** Memverifikasi ketersediaan seluruh section materi (`introduction`, `concept`, `analogy`, `architecture`, `walkthrough`, `remediation`) dalam Bahasa Indonesia.
3. **Grader Contract Test:** Memverifikasi format flag `ZITERA{...}`, ketiadaan kebocoran flag pada UI `hints.json` klien, dan berfungsinya `timing_safe_compare` di backend engine.
