# ZITERA_LAB: LEARNING FLOW & CURRICULUM ARCHITECTURE (MULTI-TIER PEDAGOGY & BOUNTY EXPANSION)
**Document Reference:** `PRD/LEARNING_FLOW_AND_CURRICULUM_ARCHITECTURE.md`  
**Author:** researcher-curriculum (Agentic Swarm)  
**Status:** CANONICAL RESEARCH SPECIFICATION & CURRICULUM BLUEPRINT  
**Target Platform:** Windows x64 Native + Flutter Desktop UI  
**Distribution Model:** Lightweight Core + Atomic Dynamic Package Updates (`.zlab`)

---

## 1. Executive Summary & Educational Vision

Zitera Lab didesain untuk menjembatani kesenjangan (*skill gap*) antara pemula (siswa SMK, mahasiswa, junior developer) dengan realitas industri keamanan siber profesional (*bug bounty hunting* dan *application security engineering*). 

Kebanyakan platform pembelajaran keamanan siber saat ini menderita salah satu dari dua kelemahan ekstrem:
1. **Terlalu Teoretis / Pasif:** Memberikan teks panjang tanpa interaksi bermakna, membuat pembelajar cepat bosan dan gagal membangun intuisi teknis (*mindset attacker*).
2. **Terlalu Langsung / "Drop into Deep Water":** Melempar pengguna langsung ke lab CTF kompleks tanpa fondasi mental atau analogi, sehingga pemula frustrasi, menyalin *writeup*, dan tidak memahami akar permasalahan kode (*root cause*).

Kurikulum Zitera Lab memecahkan masalah ini melalui **Arsitektur Pembelajaran Multi-Tier Berjenjang (*3-Tier Progressive Disclosure*)** yang dipadukan dengan **Dynamic Staging** dan kurikulum berbasis kerentanan bernilai *bounty* tinggi (*High-Bounty Vulnerability Curriculum*).

```text
┌──────────────────────────────────────────────────────────────────────────────┐
│                    ZITERA LAB 3-TIER PROGRESSION MODEL                       │
├──────────────────────────────────────────────────────────────────────────────┤
│  TIER 1: NEWBIE / MINDSET & CONCEPTUAL                                      │
│  "Pahami Logika & Mengapa Bernilai Mahal"                                     │
│  • Analogi Kehidupan Nyata (Mental Model)                                    │
│  • Logika Celah & Mengapa Developer Melakukan Kesalahan Ini                   │
│  • Attacker Mindset (Bagaimana Peretas Menemukan Celah Ini)                  │
│  • Bug Bounty Value (Dampak Bisnis, CVSS, $500 - $15,000+ Valuasi)          │
├──────────────────────────────────────────────────────────────────────────────┤
│                                      ▼                                       │
│  TIER 2: GUIDED HANDS-ON PRACTICE                                            │
│  "Verifikasi Konsep Dasar dengan Probe Interaktif"                           │
│  • Scaffolded Step-by-Step Walkthrough                                       │
│  • Endpoint Probe Interaktif (/practice/verify)                              │
│  • Eksperimen Perbedaan Respons Rentan vs Kebal                              │
│  • Telemetri Langsung Tanpa Rasa Takut Rusak / Reset Instan                  │
├──────────────────────────────────────────────────────────────────────────────┤
│                                      ▼                                       │
│  TIER 3: REALISTIC BOUNTY CHALLENGE                                          │
│  "Simulasi Skenario Dunia Nyata Tanpa Bantuan"                               │
│  • Skenario Produksi Realistis (SaaS Multi-Tenant, FinTech, E-Commerce)      │
│  • Blackbox / Greybox Security Testing                                       │
│  • Multi-Step Attack Chain & Defensif Decoys                                 │
│  • Server-Side Grader & Flag Verification ke Live Scoreboard (Turso DB)      │
└──────────────────────────────────────────────────────────────────────────────┘
```

---

## 2. Multi-Tier Learning Flow per Vulnerability Category

Setiap laboratorium di Zitera Lab tidak sekadar berupa target eksploitasi statis, melainkan paket pembelajaran komprehensif yang mengalir dari pemahaman intuitif hingga pembuktian mandiri.

### 2.1 Tier 1: Newbie / Mindset & Conceptual
Tujuan dari Tier 1 adalah membangun fondasi intuitif sebelum baris kode atau terminal dibuka. 

1. **Analogi Kehidupan Nyata (*Real-World Analogy*):**
   - Menggunakan objek fisik sehari-hari yang sudah dipahami semua orang (kunci kamar hotel, nota pesanan restoran, kurir pengantar paket, stempel kasir).
   - Analogi memetakan prinsip abstraksi keamanan menjadi fenomena fisik yang dapat divisualisasikan.
2. **Logika Celah (*Vulnerability Logic & Root Cause*):**
   - Membedakan antara *Autentikasi* ("Siapa kamu?") dan *Otorisasi* ("Apa yang boleh kamu buka?").
   - Menjelaskan pelanggaran batas kepercayaan (*Trust Boundary Violation*): data dari klien dipercaya secara buta oleh server.
3. **Mengapa Developer Melakukan Kesalahan Ini? (*Developer Cognitive Blindspots*):**
   - *UI-Centric Bias:* Developer berasumsi karena UI tidak menampilkan tombol edit milik user lain, maka request API tidak akan pernah dikirim.
   - *Framework Defaults:* Default framework (seperti Mass Assignment di Rails/Express/Spring) mengikat seluruh payload JSON langsung ke database model.
   - *Deadline & Complexity Pressure:* Menulis ownership check di setiap handler endpoint dianggap repetitif dan mudah terlewat dalam sprint yang terburu-buru.
   - *Pemisahan Middleware:* Adanya `@login_required` menciptakan rasa aman palsu bahwa endpoint sudah terlindungi, padahal otorisasi level objek (*object-level ownership*) belum diperiksa.
4. **Attacker Mindset (*How Bounty Hunters Think*):**
   - Bagaimana *bug hunter* melihat sebuah aplikasi web: bukan sebagai tombol dan warna, melainkan sebagai aliran data parameter (`id`, `user_id`, `role`, `url`, `file`).
   - Pola investigasi: Mengamati request di proxy/DevTools, mencari pola ID sekuensial atau UUID, melakukan *parameter tampering*, dan mengamati diff respons HTTP.
5. **Mengapa Bernilai Bounty Tinggi di Dunia Nyata? (*Real-World Bounty Value*):**
   - Menjelaskan dampak finansial dan hukum (UU Perlindungan Data Pribadi / GDPR).
   - Rating tingkat keparahan (*Severity*): P1 (Critical) / P2 (High) di HackerOne & Bugcrowd.
   - Alasan perusahaan rela membayar $1,000 – $10,000+: satu celah IDOR atau SSRF dapat membocorkan seluruh database pelanggan atau kunci infrastruktur cloud AWS/GCP.

---

### 2.2 Tier 2: Guided Hands-on Practice (Probe Interaktif)
Tujuan dari Tier 2 adalah memverifikasi konsep secara langsung dengan bimbingan scaffolded dan umpan balik otomatis.

1. **Scaffolded Step-by-Step Walkthrough:**
   - Pembelajar dipandu langkah demi langkah mengirimkan probe awal (misalnya via konsol terminal Zitera atau tombol probe cepat).
   - Diberikan contoh request curl spesifik dengan penjelasan setiap header dan parameternya.
2. **Authoritative Interactive Probe Endpoint (`/practice/verify`):**
   - Runtime lokal native lab mengekspos endpoint telemetri `/practice/verify` yang memeriksa kondisi lab secara real-time.
   - Mesin Rust mengeksekusi probe deterministik dan mengembalikan JSON status:
     ```json
     {
       "status": "passed",
       "tier": 2,
       "step_completed": "idor_horizontal_probed",
       "message": "Probe berhasil! Anda mendeteksi bahwa server mengembalikan data milik User #2 tanpa memeriksa session."
     }
     ```
3. **Komparasi Respon Rentan vs Respon Aman:**
   - Menampilkan *diff view* interaktif antara respons sebelum dan sesudah perbaikan.
   - Pembelajar dapat melihat perbandingan kode (*Vulnerable Pattern* vs *Defensive Pattern*) dan menguji efeknya secara langsung.

---

### 2.3 Tier 3: Realistic Bounty Challenge (Simulasi Skenario Nyata)
Tujuan dari Tier 3 adalah menguji transfer kemampuan (*knowledge transfer*) pada skenario tanpa panduan, persis seperti pengujian penetrasi atau berburu *bug bounty* nyata.

1. **Skenario Bisnis Realistis (*Real-World Context*):**
   - Target disajikan sebagai portal bisnis yang masuk akal (misalnya: *FinTech Billing Portal*, *Healthcare Telemedicine API*, *E-Commerce Flash Sale Platform*).
   - Terdapat peran pengguna (*roles*) nyata: Tamu, Karyawan Standar, Manajer Finansial, dan Administrator.
2. **Blackbox / Greybox Engagement:**
   - Pengguna hanya diberikan *Scope of Assessment*, *Target URL*, dan satu set kredensial akun awal (*standard user*).
   - Pengguna harus melakukan *recon*, pemetaan endpoint, identifikasi anomali, hingga eksfiltrasi flag rahasia.
3. **Multi-Step Attack Chain & Decoys:**
   - Tantangan tingkat lanjut menggabungkan rantai eksploitasi (contoh: IDOR membocorkan reset token -> login sebagai admin -> eksploitasi SSRF di webhook internal).
   - Disediakan endpoint jebakan (*decoys*) yang aman untuk melatih ketajaman eliminasi false positive.
4. **Deterministic Grader & Scoreboard Sync:**
   - Flag rahasia divalidasi oleh mesin inti Rust (`validate_challenge`).
   - Ketika flag valid, skor otomatis disinkronkan ke live CTF scoreboard di cloud Turso DB, menampilkan avatar, pangkat, dan lencana penyelesaian.

---

### 2.4 Matriks Contoh Implementasi Konkret: IDOR (A01)

| Elemen Pembelajaran | Implementasi pada Modul IDOR (A01) |
|---|---|
| **Tier 1: Analogi** | **Kartu Kunci Kamar Hotel & Penitipan Koper:** Tamu memegang kartu sah untuk Kamar 302, tetapi aplikasi ponsel ceroboh mengizinkan tamu mengetik nomor kamar 303 dan pintu tetap terbuka karena kunci tidak mencocokkan data reservasi pusat. |
| **Tier 1: Mengapa Terjadi?** | Developer hanya menerapkan `@login_required` (memastikan pengguna sudah login), tetapi lupa menulis klausul `WHERE owner_id = current_user.id` pada kueri database. |
| **Tier 1: Attacker Mindset** | Bug hunter memperhatikan parameter `GET /api/invoices/1042`. Mereka segera menguji `1043`, `1041`, dan `1` menggunakan repeater untuk melihat apakah ada data entitas lain yang bocor. |
| **Tier 1: Bounty Value** | **Tingkat Keparahan:** High / Critical (CVSS 7.5–8.8). **Bounty:** $2,000 - $10,000 pada program private fintech karena membuka data keuangan seluruh nasabah tanpa izin. |
| **Tier 2: Guided Practice** | User membuka tab Practice, menjalankan request `GET /invoice/1` (milik Alice), lalu mengirim manipulasi `GET /invoice/2`. Klik tombol *Verify Local Service Health* memicu probe yang mengonfirmasi anomali respon HTTP 200 dengan data user lain. |
| **Tier 3: Bounty Challenge** | **Skenario Misi:** Portal finansial internal perusahaan menyimpan "Master System Activation Invoice" dengan ID tersembunyi. User harus menganalisis skema penomoran ID, menemukan master invoice rahasia milik departemen audit, dan mengambil flag `ZITERA{b10k3n_4cc355_c0ntr01_m45t3r}`. |

---

## 3. Content Expansion Roadmap: Menuju Puluhan Vulnerability Bernilai Tinggi

Untuk memperluas katalog dari 10 laboratorium OWASP awal menjadi puluhan modul spesialisasi, Zitera Lab menetapkan taksonomi kurikulum berbasis **High-Bounty Vulnerability Categories**.

### 3.1 Taksonomi 10 Kategori Kerentanan Berbounty Tinggi

```text
┌─────────────────────────────────────────────────────────────────────────────┐
│                   ZITERA LAB HIGH-BOUNTY EXPANSION TAXONOMY                 │
├───────────────────────────────────┬─────────────────────────────────────────┤
│ 1. IDOR & BOLA (Access Control)   │ 6. Auth Bypass & Account Takeover (ATO) │
│    Horizontal/Vertical, UUID Fuzz │    MFA Bypass, Password Reset Poison    │
├───────────────────────────────────┼─────────────────────────────────────────┤
│ 2. SQL Injection (SQLi)           │ 7. Mass Assignment & Data Overposting   │
│    Union-Based, Blind, Second-Ord │    Role Injection, Nested Parameter Bind│
├───────────────────────────────────┼─────────────────────────────────────────┤
│ 3. Server-Side Request Forgery    │ 8. CSRF & Cross-Origin Sharing (CORS)   │
│    Cloud Metadata, Internal Mesh  │    S-Cookie Bypass, Reflective Origin   │
├───────────────────────────────────┼─────────────────────────────────────────┤
│ 4. Cross-Site Scripting (XSS)     │ 9. Remote Code Execution (RCE)          │
│    Stored Feed, DOM Sink, CSP-Byp │    Command Injection, Deserialization   │
├───────────────────────────────────┼─────────────────────────────────────────┤
│ 5. Business Logic & Race Flaws    │ 10. Insecure File Upload                │
│    Coupon Races, State Machine Byp│     MIME Sniffing, Path Traversal Filenm│
└───────────────────────────────────┴─────────────────────────────────────────┘
```

---

### 3.2 Matriks Spesifikasi Detail 10 Kategori Kerentanan Utama

| ID Lab | Kategori & Topik | Analogi Nyata (Tier 1) | Valuasi Bounty & CVSS | Tier 2 Guided Probe Objective | Tier 3 Realistic Bounty Challenge |
|---|---|---|---|---|---|
| **A01 / BOLA-01** | **IDOR / BOLA** (Broken Object Level Authorization) | Tamu hotel membuka pintu kamar tetangga dengan mengetik nomor kamar di aplikasi tanpa validasi kunci pusat. | **High / Critical** (CVSS 7.5 - 8.6)<br>**Bounty:** $1,500 - $8,000 | Menginspeksi payload JSON `/api/v1/orders/101` vs `102` menggunakan probe telemetri otomatis. | **FinTech Billing Portal:** Mengaudit endpoint pembayaran multi-tenant untuk mengekfiltrasi master billing invoice perusahaan rahasia. |
| **A05 / SQLI-01** | **SQL Injection** (Union-Based & Blind) | Pelanggan memesan makanan di kasir sambil menyelipkan catatan: `Nasi Goreng; Hapus Semua Catatan Hutang`. Kasir membacanya langsung ke dapur tanpa membedakan pesanan dan instruksi. | **Critical** (CVSS 8.5 - 9.8)<br>**Bounty:** $3,000 - $15,000+ | Menguji karakter `' OR '1'='1` pada filter katalog produk dan mengamati respons error database. | **E-Commerce Inventory Search:** Melakukan teknik UNION injection untuk mengekstrak tabel kredensial administrator tersembunyi. |
| **W02 / SSRF-01** | **SSRF** (Server-Side Request Forgery & Cloud Metadata) | Kantor memiliki kurir internal yang dipercaya satpam. Tamu meminta kurir mengambilkan berkas rahasia dari brankas internal berlabel `http://169.254.169.254`. | **High / Critical** (CVSS 8.6 - 9.8)<br>**Bounty:** $4,000 - $20,000+ | Mengirim URL loopback `http://127.0.0.1:8080/internal/status` pada fitur web scraper profil. | **Cloud Webhook Manager:** Memanfaatkan fitur export PDF / URL preview untuk membaca endpoint metadata cloud sintetis dan mengekstrak token AWS IAM. |
| **XSS-01** | **Stored & DOM XSS** | Orang jahat menempelkan kertas pengumuman bertuliskan mantra hipnotis di papan pengumuman desa. Siapa pun warga yang membaca kertas tersebut terhipnotis memberikan dompetnya. | **Medium / High** (CVSS 6.1 - 8.2)<br>**Bounty:** $500 - $4,000 | Menginjeksi payload sanitasi HTML sederhana `<img src=x onerror=...>` pada kolom bio dan memverifikasi rendering DOM. | **SaaS Support Helpdesk:** Menginjeksi payload stored XSS pada tiket bantuan yang dipicu saat agen admin membuka tiket, mengeksfiltrasi session admin cookie. |
| **LOGIC-01** | **Business Logic & Race Conditions** | Mesin penjual otomatis yang dipukul pada dua tombol secara serentak mengeluarkan dua kaleng minuman sementara saldo hanya terpotong satu koin. | **High** (CVSS 7.1 - 8.5)<br>**Bounty:** $2,000 - $10,000 | Mengirim 2 permintaan simultan menggunakan concurrency probe untuk klaim kupon diskon satu kali pakai. | **Crypto Exchange Wallet:** Memanfaatkan race condition limit transaksi penarikan saldo (*balance overdraft*) untuk menggandakan kuota kredit. |
| **AUTH-01** | **Authentication Bypass & ATO** | Resepsionis hotel menerima telepon yang mengaku pemilik kamar, lalu tanpa meminta nomor identitas langsung mereset kode kunci kamar. | **Critical** (CVSS 8.8 - 9.8)<br>**Bounty:** $3,000 - $15,000 | Memanipulasi respon verifikasi OTP dari `{"status":"fail"}` menjadi `{"status":"success"}` pada client state. | **Corporate Single-Sign-On:** Mengidentifikasi implementasi OAuth 2.0 tanpa validasi state parameter untuk melakukan pengambilalihan akun (*Account Takeover*). |
| **MASS-01** | **Mass Assignment / Parameter Binding** | Formulir biodata sekolah memiliki kolom tersembunyi "Tentukan Nilai Akhir". Siswa menambahkan tulisan tangan "Nilai: A+" dan staf tata usaha langsung menyalin semuanya ke raport. | **Medium / High** (CVSS 6.5 - 8.1)<br>**Bounty:** $1,000 - $5,000 | Menambahkan parameter JSON `"role": "admin"` pada payload update profil standar dan memvalidasi respon. | **B2B SaaS HRMS:** Memodifikasi endpoint pendaftaran karyawan untuk menginjeksikan properti `is_organization_owner: true` guna menguasai tenant. |
| **CORS-01** | **CSRF & Permissive CORS** | Surat kuasa palsu bertanda tangan asli yang dibawa orang tak dikenal ke bank; teller memproses penarikan uang karena surat itu tampak sah dari pemilik rekening. | **Medium / High** (CVSS 6.5 - 8.3)<br>**Bounty:** $750 - $4,000 | Mengonfigurasi probe origin `http://attacker.local` dan membaca header `Access-Control-Allow-Credentials: true`. | **Intranet Banking Portal:** Membuat halaman exploit CSRF untuk mentransfer saldo secara otomatis saat korban mengunjungi link berbahaya. |
| **RCE-01** | **Remote Code Execution & SSTI** | Surat pesanan koki di restoran ditempeli instruksi: "Tolong masak steak, dan setelah itu bakar gedung ini". Koki patuh membakar gedung karena instruksi digabung dengan resep. | **Critical** (CVSS 9.8 - 10.0)<br>**Bounty:** $5,000 - $25,000+ | Menguji ekspresi template engine `{{7*7}}` pada generator faktur otomatis dan memverifikasi evaluasi angka `49`. | **Enterprise Report Generator:** Memanfaatkan SSTI pada template email Jinja2/Twig untuk mengeksekusi perintah shell native dan mengambil flag server. |
| **UPL-01** | **Insecure File Upload** | Kurir memasukkan bom ke dalam kardus bertuliskan "Kado Boneka Lucu". Penjaga gerbang hanya membaca label stiker luar tanpa membuka dan memeriksa isi kardus. | **High / Critical** (CVSS 7.8 - 9.8)<br>**Bounty:** $2,500 - $12,000 | Mengunggah berkas `shell.php.jpg` dan memverifikasi apakah server mengeksekusinya sebagai skrip atau gambar pasif. | **Document Management System:** Mem-bypass filter ekstensi dan MIME validation menggunakan teknik double extension & null-byte untuk mengunggah web shell. |

---

### 3.3 Phased Expansion Roadmap (Gelombang Rilis Konten)

Ekspansi katalog Zitera Lab dilakukan dalam 3 gelombang terencana (*Phased Release Waves*), memastikan setiap paket lab memenuhi standar *Curriculum Quality Gate* (bilingual, native executable, deterministik, tanpa dependensi eksternal):

```text
┌─────────────────────────────────────────────────────────────────────────────┐
│                       ZITERA LAB CONTENT EXPANSION ROADMAP                  │
├─────────────────────────────────────────────────────────────────────────────┤
│  WAVE 1: OWASP TOP 10 CANONICAL BASELINE (CURRENT LAUNCH)                   │
│  Target: 10 Core Labs (A01 - A10) + Shared Foundation (F01 - F06)           │
│  Status: All 10 native binaries built, sequential ports 8011-8020,           │
│          Win32 Job Object sandbox verified, Turso live scoreboard active.   │
├─────────────────────────────────────────────────────────────────────────────┤
│                                      ▼                                       │
│  WAVE 2: HIGH-BOUNTY WEB & API EXPANSION (Q1 - Q2)                          │
│  Target: +10 Specialized Labs (Total: 20 Labs)                              │
│  Fokus: Kerentanan paling dicari di program bug bounty komersial:            │
│  • W01: Cross-Site Request Forgery (CSRF) & SameSite Cookie Bypass          │
│  • W02: Server-Side Request Forgery (SSRF) & Synthetic Cloud Metadata       │
│  • W03: API Broken Object Level Authorization (BOLA v2)                      │
│  • W04: Insecure File Upload & Polyglot Execution                            │
│  • W05: Business Logic: Coupon Race Condition & Flash Sale Tampering        │
│  • W06: Mass Assignment & Object Injection in REST/GraphQL                  │
│  • W07: Insecure CORS with Reflected Origin & Credential Leakage             │
│  • W08: Stored XSS & CSP Nonce Bypass                                       │
│  • W09: Path Traversal & Arbitrary Document Disclosure                       │
│  • W10: JWT Attack Suite (Algorithm None, Secret Brute-forcing)              │
├─────────────────────────────────────────────────────────────────────────────┤
│                                      ▼                                       │
│  WAVE 3: ADVANCED ATTACK CHAINS & CLOUD ARCHITECTURE (Q3 - Q4)              │
│  Target: +12 Enterprise Labs (Total: 32+ Labs)                              │
│  Fokus: Rantai serangan multi-tahap dan keamanan infrastruktur modern:      │
│  • C01: SSTI to Remote Code Execution Chain                                 │
│  • C02: OAuth 2.0 Account Hijacking via Pre-Account Takeover                 │
│  • C03: GraphQL Batching & Depth Query Denial of Service                    │
│  • C04: Deserialization of Untrusted Data (Native Windows Object RPC)       │
│  • C05: Webhook Signature Forgery & Replay Attacks                           │
│  • C06: AWS/GCP Synthetic Cloud Pivot via SSRF Instance Profile              │
│  • C07: Microservices Service-to-Service Trust Exploitation                  │
│  • C08: SQLi Second-Order Blind Timing Extraction                           │
│  • C09: Prototype Pollution in Node.js Microservices                        │
│  • C10: Subdomain Takeover & CNAME Dangling DNS Audit                        │
│  • C11: CI/CD Pipeline Poisoning & GitHub Actions Artifact Tampering         │
│  • C12: Defensive Threat Hunting & SIEM Log Correlation Challenge           │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## 4. Evaluasi Komparatif 3 Alternatif Arsitektur Kurikulum

Untuk menentukan arsitektur pembelajaran yang paling unggul, berikut perbandingan mendalam antara 3 alternatif arsitektur:

### 4.1 Definisi 3 Alternatif

1. **Alternatif 1: Flat Labs (Daftar Lab Lepas / CTF Monolitik Sederhana)**
   - *Konsep:* Seluruh tantangan disajikan sebagai daftar datar (*flat list* 1 s/d N), mirip dengan daftar challenge CTF klasik atau DVWA/WebGoat generasi pertama.
   - *Alur Pengguna:* Pengguna memilih lab apa pun, langsung dihadapkan pada satu halaman target dan satu kotak input flag.
2. **Alternatif 2: Monolithic Lessons (Dokumentasi Teori Raksasa + Satu Sandbox)**
   - *Konsep:* Satu materi panjang terpadu (mirip membaca bab buku digital / eBook tebal) yang diakhiri dengan tombol peluncuran satu sandbox target serba guna.
   - *Alur Pengguna:* Pengguna harus membaca dokumen panjang dari awal hingga akhir, kemudian membuka sandbox untuk mengeksplorasi target tanpa tahapan terisolasi.
3. **Alternatif 3: Multi-Tier Track Progression with Dynamic Staging (Direkomendasikan)**
   - *Konsep:* Pemisahan kurikulum ke dalam jalur terstruktur (*Tracks*), di mana setiap laboratorium memiliki 3 tier progresif (*Tier 1 Mindset -> Tier 2 Guided Probe -> Tier 3 Bounty CTF*). 
   - *Alur Pengguna:* Didukung oleh *dynamic staging* (lingkungan runtime lokal yang mengisolasi tahapan latihan terpandu dari skenario final CTF, dengan probe telemetri otomatis dan verifikasi server-side).

---

### 4.2 Matriks Perbandingan Komprehensif

| Kriteria Evaluasi | Alternatif 1: Flat Labs | Alternatif 2: Monolithic Lessons | Alternatif 3: Multi-Tier Track Progression (Rekomendasi) |
|---|---|---|---|
| **Efektivitas Pedagogis & Scaffolding** | **Rendah (D).** Tidak ada jembatan antara konsep awal dan eksploitasi nyata. Pemula mengalami *cognitive overload*. | **Sedang (C).** Terlalu pasif. Membaca teks panjang tanpa validasi interaktif menurunkan retensi ingatan (*text fatigue*). | **Sangat Tinggi (A+).** Scaffolding sempurna: Analogi -> Logika Celah -> Probe Praktis Terpandu -> Misi CTF Mandiri. |
| **User Experience (UX) & Drop-off Rate** | **Tinggi Drop-off.** Pemula sering menyerah di 10 menit pertama karena tidak tahu harus mulai dari mana. | **Sedang.** Pengguna cenderung melompati teks panjang (*skim reading*) dan bingung saat sandbox gagal. | **Rendah Drop-off (Gamified).** Micro-milestones memberikan rasa pencapaian berkala (*instant gratification*). |
| **Kesiapan Real-World Bug Bounty** | **Rendah-Sedang.** Pengguna hanya terlatih memasukkan payload statis tanpa memahami motif bisnis dan sudut pandang hunter. | **Rendah.** Berfokus pada teori akademis, minim keterampilan insting investigasi parameter. | **Sangat Tinggi.** Melatih *attacker mindset*, estimasi *bounty value*, dan identifikasi dampak bisnis sejak Tier 1. |
| **Kelayakan Teknis (*Feasibility*) di Zitera Lab** | **Tinggi (Mudah).** Sangat sederhana diimplementasikan, tetapi membuang potensi engine Rust & Flutter UI yang sudah canggih. | **Tinggi.** Hanya membutuhkan render markdown biasa. | **Sangat Tinggi (Sudah Tersedia Fondasi).** Zitera Lab sudah memiliki mode `learn`, `practice`, `challenge`, IPC Rust, dan Job Object sandbox. |
| **Isolasi State & Kebersihan Runtime** | **Buruk.** Percobaan manipulasi di tahap belajar dapat merusak target dan flag CTF, membutuhkan restart manual berkali-kali. | **Buruk.** Satu sandbox menanggung seluruh eksperimen pengguna, rentan kondisi inkonsisten. | **Sangat Baik.** Dynamic staging: endpoint `/practice/verify` terpisah secara logis dari state flag CTF `/validate`. |
| **Skalabilitas Konten (Ekspansi 10 -> 50+ Lab)** | **Kacau.** Daftar lab menjadi sangat panjang dan membingungkan tanpa struktur tema atau kurikulum. | **Kaku.** Setiap pembaruan memerlukan penulisan ulang dokumen masif. | **Sangat Skalabel.** Penambahan modul cukup mengikuti skema manifest standar dan didistribusikan via signed catalog. |
| **Retensi Pengguna & Tracking Progres** | **Biner (0 atau 100%).** Pengguna hanya tercatat "Solved" atau "Belum Solved", tidak ada progress granular. | **Statik.** Sulit mengukur seberapa jauh pengguna memahami materi. | **Granular & Terukur.** Tracking per tier (Tier 1 Read, Tier 2 Probed, Tier 3 Flag Conquered) tersinkronisasi ke Turso DB. |

---

### 4.3 Mengapa Alternatif 3 Adalah Pilihan Terbaik untuk Zitera Lab

1. **Memaksimalkan Aset Arsitektur yang Sudah Ada:**
   Zitera Lab telah memiliki:
   - File manifest dengan field `modes: ["learn", "practice", "challenge"]`.
   - Engine Rust dengan handler `verify_practice` dan `validate_challenge`.
   - UI Flutter dengan navigasi tab `MODE A: LEARN`, `MODE B: PRACTICE`, `MODE C: CHALLENGE / CTF`.
   - Win32 Job Object sandbox berkinerja tinggi pada port loopback native.
   - Live Scoreboard & User Authentication terintegrasi dengan Turso DB.
2. **Pedagogi Kognitif Modern (*Low Floor, High Ceiling*):**
   - *Low Floor:* Siapa pun tanpa latar belakang keamanan siber dapat membaca Tier 1 karena analoginya sangat manusiawi dan relevan.
   - *High Ceiling:* Praktisi berpengalaman dapat langsung melompat ke Tier 3 untuk menguji kemampuannya menyelesaikan tantangan realistis dan mengumpulkan poin di scoreboard.
3. **Menciptakan "Bug Bounty Hunter" yang Kompeten, Bukan Sekadar Script Kiddie:**
   Dengan mengajarkan *Attacker Mindset* dan *Root Cause Engineering*, pembelajar memahami alasan mendalam mengapa developer membuat kesalahan tersebut dan bagaimana memperbaikinya secara defensif (*remediation-first*).

---

## 5. Blueprint Arsitektur Implementasi pada Zitera Lab Engine & UI

Untuk merealisasikan Alternatif 3 secara penuh pada sistem yang ada:

### 5.1 Standarisasi Kontrak Berkas Laboratorium (`manifest.json` v2)
Setiap paket laboratorium wajib memuat metadata kurikulum berjenjang:

```json
{
  "schema_version": 2,
  "id": "A01",
  "package_id": "zitera-lab-a01",
  "slug": "broken-access-control",
  "title": "Broken Access Control",
  "owasp": "A01:2025",
  "category": "web-security",
  "version": "1.1.0",
  "difficulty": "Beginner",
  "bounty_profile": {
    "tier": "Critical",
    "typical_payout_range": "$1,500 - $8,000",
    "cvss_score": 8.5,
    "cwe_id": "CWE-639"
  },
  "runtime": "native_sandboxed",
  "entrypoint": "bin/a01-lab.exe",
  "default_port": 8011,
  "modes": ["learn", "practice", "challenge"],
  "tier_specs": {
    "tier_1_concept": {
      "analogy_title": "Kartu Kunci Hotel & Nomor Penitipan Koper",
      "reading_time_minutes": 10
    },
    "tier_2_practice": {
      "probe_endpoint": "/practice/verify",
      "expected_outcome": "IDOR parameter reflection detected"
    },
    "tier_3_challenge": {
      "mission_title": "Kebocoran Master Invoice Perusahaan",
      "points": 100
    }
  }
}
```

### 5.2 Alur Eksekusi & State Machine di Flutter UI
1. **Status Kesiapan (*Tier Completion State*):**
   - Tiap lab memiliki 3 status granular di `ProgressManager`:
     - `tier1_completed` (Lesson sections dibaca).
     - `tier2_completed` (Probe `/practice/verify` berhasil diverifikasi).
     - `tier3_completed` (Flag CTF tervalidasi dan tercatat di Turso DB).
2. **Visual Feedback & Gamifikasi:**
   - Tab Bar menampilkan ikon status real-time:
     - `MODE A: LEARN` -> Checkmark hijau saat seksi esensial selesai dibaca.
     - `MODE B: PRACTICE` -> Radar pulse animasi saat melakukan probe, badge hijau `PRACTICE VERIFIED`.
     - `MODE C: CHALLENGE` -> Tanda bintang emas / piala saat flag berhasil di-submit, dialog selebrasi dengan ringkasan poin dan avatar operative.

---

## 6. Rekomendasi Aksi Berikutnya (*Next Actionable Steps*)

1. **Standarisasi Konten 10 Lab A01–A10:**
   - Memastikan berkas `analogy.id.md` dan `concept.id.md` di seluruh 10 lab memuat bagian eksplisit:
     - *Mengapa developer melakukan kesalahan ini*.
     - *Attacker mindset & teknik pengintaian parameter*.
     - *Valuasi bug bounty di dunia nyata*.
2. **Ekspansi Wave 2 (W01–W10):**
   - Menginisialisasi paket lab ekspansi prioritas tinggi: SSRF (`W02`), CSRF (`W01`), BOLA API (`W03`), dan Insecure File Upload (`W04`).
3. **Penyempurnaan Endpoint Telemetri `/practice/verify`:**
   - Memperluas verifikasi practice di `engine/rust/src/labs.rs` agar mengembalikan detail granular langkah yang sedang diuji untuk memandu pengguna secara interaktif.

---
*Dokumen ini merupakan hasil riset resmi dan evaluasi arsitektur kurikulum Zitera Lab oleh researcher-curriculum.*
