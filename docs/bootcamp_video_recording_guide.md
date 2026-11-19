# 🎬 Master Blueprint: Panduan Rekaman Video Bootcamp Zitera Lab (2026/2027)

> **Panduan Praktis Kreator Konten & Instruktur Bug Bounty:**
> Dokumen ini dirancang khusus untuk memandu perekaman 10 episode video materi bootcamp Web Penetration Testing & High-Yield Bug Bounty Zitera Lab. Setiap episode berdurasi optimal **8–12 menit**, terstruktur menit-demi-menit, mencakup analogi dunia nyata, live exploitation (Burp Suite & Browser), remedi kode defensif, serta cara penulisan laporan HackerOne/Bugcrowd berstandar $2.000–$12.000 USD.

---

## 🛠️ 1. Setup Studio & Software Rekaman (OBS Studio)

### A. Konfigurasi OBS Studio (Optimal Quality & Zero Latency)
- **Base Canvas & Output Resolution:** `1920x1080` (Full HD, rasio 16:9).
- **Framerate:** `60 FPS` (menghasilkan scroll UI dan demo terminal yang sangat halus).
- **Video Encoder:** NVIDIA NVENC H.264 / AMD VCE (Rate Control: `CBR`, Bitrate: `8000–10000 Kbps`, Preset: `P6: Slower (Better Quality)`).
- **Audio:** 48 kHz Stereo, Bitrate: `192 Kbps`. Pasang filter `Noise Suppression` (RNNoise) dan `Limiter` (-1.5 dB) pada mikrofon.
- **Scene Layout Rekomendasi:**
  1. **Scene 1 (Intro & Analogi):** Zitera Lab UI Full Screen (Materi Kurikulum Tab 1 & 2).
  2. **Scene 2 (Hands-on Pentest):** Split Screen (Kiri: Zitera Lab Target Web Browser, Kanan: Tilix Terminal / Burp Suite Intercept).
  3. **Scene 3 (Bounty Report & Triage):** Zitera Lab Triager & Report Card (Tab 5 & 6).

---

## 📺 2. Cara Menghubungkan Video YouTube ke Dalam Aplikasi Zitera Lab

Setelah video direkam dan diunggah ke YouTube (bisa diset sebagai *Public* atau *Unlisted*):
1. Buka aplikasi **Zitera Lab**.
2. Masuk ke lab yang bersangkutan (misal: **A01 Broken Access Control**).
3. Klik tombol merah **`🎬 Video Bootcamp`** di bar navigasi atas.
4. Drawer video akan terbuka di sisi kanan.
5. Klik ikon gear **`⚙ URL Settings`** di header drawer.
6. Masukkan tautan video YouTube kamu (misal: `https://www.youtube.com/watch?v=dQw4w9WgXcQ` atau ID video `dQw4w9WgXcQ`).
7. Klik **`Simpan URL Video`**.
8. Video langsung terpasang di YouTube-nocookie player internal aplikasi tanpa iklan atau pelacakan privasi!

---

## 📑 3. Naskah & Struktur Rekaman Menit-demi-Menit (10 Labs)

---

### 🏥 Episode 01: Broken Access Control (IDOR / BOLA) — $4.500 Bounty
- **Target Lab:** *ApexFin Corporate Treasury* (`http://127.0.0.1:8011`)
- **Tingkat Kesulitan:** Beginner | **CVSS v3.1:** 8.6 (High)
- **Flag Target:** `ZITERA{b10k3n_4cc355_c0ntr01_m45t3r}`

#### Timeline & Naskah Narasi:
- **00:00 - 02:00 | Intro & Dampak Finansial:**
  - *"Halo rekan pentester! Di episode pertama ini, kita membedah kerentanan peringkat #1 OWASP Top 10 2025/2026: Broken Access Control, khususnya IDOR/BOLA."*
  - Tampilkan halaman **1. Pengantar & Dampak** di Zitera Lab. Jelaskan bagaimana celah ini memungkinkan peretas mentransfer dana atau mengintip transaksi kawat miliaran rupiah antar korporasi.
- **02:00 - 04:00 | Analogi Kartu Kunci Hotel #302:**
  - Buka tab **2. Analogi Nyata**.
  - Ceritakan analogi kunci hotel: Resepsionis memberikan kartu kamar #302, tetapi aplikasi ponsel hotel membiarkan tamu mengetik kamar #303 atau kamar VIP #101 tanpa verifikasi kepemilikan.
- **04:00 - 07:30 | Live Exploit Demo (Burp Suite / Browser):**
  - Klik tombol **`LAUNCH TARGET`** atau **`Buka Web (: 8011)`**.
  - Tunjukkan login sebagai `alice` (Operator ID: 1). Buka daftar transfer kawat.
  - Klik voucher kawat milik Alice (URL: `/wires/view?id=1` atau `/wire/1`).
  - Lakukan *Parameter Tampering* di URL: ubah `id=1` menjadi `id=42`.
  - Halaman seketika menampilkan voucher transfer rahasia SOX-404 milik VP Controller Marcus Reid bernilai \$5.000.000 dengan **Flag Target**!
- **07:30 - 09:30 | Review Remediasi Kode & Arsitektur Defensif:**
  - Buka tab **4. Arsitektur Sistem** dan **6. Remediasi Kode**.
  - Perlihatkan perbandingan *Vulnerable Code* vs *Secure Implementation*: query SQL/ORM wajib memfilter `WHERE id = ? AND organization_id = current_user.org_id`.
- **09:30 - 11:00 | HackerOne Report Walkthrough & Submit Flag:**
  - Buka kartu **Bug Bounty Report Blueprint** dan **HackerOne Report Triager**.
  - Tunjukkan bagaimana menyusun laporan profesional ke program bug bounty perbankan hingga status `TRIAGED` dan simulasi reward \$4.500 cair!
  - Masukkan flag ke tab **Tantangan CTF** dan perlihatkan efek confetti kemenangan.

---

### 🔑 Episode 02: Cryptographic Failures (Predictable Seed Entropy) — $6.000 Bounty
- **Target Lab:** *VaultPay Settlement Gateway* (`http://127.0.0.1:8012`)
- **Tingkat Kesulitan:** Intermediate | **CVSS v3.1:** 7.5 (High)
- **Flag Target:** `ZITERA{cr7pt0_prng_3ntr0py_f41lur3}`

#### Timeline & Naskah Narasi:
- **00:00 - 02:00 | Intro:** Penjelasan token reset password & session token yang tampak acak padahal di-generate menggunakan pseudo-random generator dengan seed waktu epoch.
- **02:00 - 04:00 | Analogi Nyata:** Jam pasir berputar di kasino yang selalu berhenti di angka yang sama pada jam tertentu.
- **04:00 - 07:30 | Live Exploit Demo:**
  - Buka Web Port 8012. Inspeksi pola token reset yang dihasilkan.
  - Tunjukkan di terminal Python / curl cara merekonstruksi seed timestamp untuk memprediksi token admin 5 detik ke depan.
  - Ambil alih akun Master Settler dan peroleh flag.
- **07:30 - 10:00 | Mitigasi & HackerOne Report:**
  - Ganti PRNG non-kriptografis dengan `CSPRNG` (`ring`, `sodium`, atau `crypto/rand` di Go).
  - Review laporan HackerOne dengan CVSS score 7.5.

---

### 📦 Episode 03: Supply Chain & Dependency Poisoning — $10.000 Bounty
- **Target Lab:** *PackNexus Private Registry* (`http://127.0.0.1:8013`)
- **Tingkat Kesulitan:** Advanced | **CVSS v3.1:** 9.8 (Critical)
- **Flag Target:** `ZITERA{5upply_ch41n_p0150n1ng_d3p_2026}`

#### Timeline & Naskah Narasi:
- **00:00 - 02:00 | Intro:** Serangan rantai pasok modern ala SolarWinds / Dependency Confusion yang sering menghasilkan bounty \$10.000+.
- **02:00 - 04:00 | Analogi Kurir:** Paket internal kantor yang tidak berlabel stempel resmi sehingga kurir luar bisa menyisipkan malware.
- **04:00 - 08:00 | Live Exploit Demo:**
  - Buka registry PackNexus di Port 8013.
  - Audit file `package_lock_audit.json` untuk menemukan paket internal yang belum terdaftar di registry publik (`apex-internal-telemetry`).
  - Simulasikan publikasi versi lebih tinggi (v99.0.0) di pipeline CI/CD dan ekstrak flag.
- **08:00 - 11:00 | Remediasi & Triager:**
  - Scoped npm packages (`@corp/package`), private registry pinning, dan package signature checks.

---

### 🛡️ Episode 04: Weak Cryptographic Custody & Hash Cracking — $3.500 Bounty
- **Target Lab:** *SafeKey Enterprise Key Custody* (`http://127.0.0.1:8014`)
- **Tingkat Kesulitan:** Intermediate | **CVSS v3.1:** 7.4 (High)
- **Flag Target:** `ZITERA{cryp70_f41lur35_w34k_k3y_2026}`

#### Timeline & Naskah Narasi:
- **00:00 - 02:00 | Intro:** Dampak penyimpanan kredensial master HSM dengan algoritma usang (MD5 unsalted) dan hardcoded JWT HMAC fallback.
- **02:00 - 04:00 | Analogi Koper Brankas:** Mengunci brankas titanium dengan gembok koper plastik kombinasi 3 angka.
- **04:00 - 07:30 | Live Exploit Demo:**
  - Buka SafeKey di Port 8014. Akses endpoint `/specs` atau `/api/audit/hashes`.
  - Temukan hash MD5 `21232f297a57a5a743894a0e4a801fc3` dan gunakan wordlist/crackstation untuk memecahkan `admin:admin`.
  - Alternatif: Forge token HMAC `SecurityOfficer` dengan hardcoded secret dan buka Master Cold Storage Vault.
- **07:30 - 10:00 | Remediasi:**
  - Migrasi ke Argon2id / PBKDF2 dengan salt 128-bit unik dan rotasi kunci berkala.

---

### 💉 Episode 05: SQL Injection (UNION & Second-Order Extraction) — $7.500 Bounty
- **Target Lab:** *DataFlow CRM Pipeline* (`http://127.0.0.1:8015`)
- **Tingkat Kesulitan:** Intermediate | **CVSS v3.1:** 8.8 (Critical)
- **Flag Target:** `ZITERA{5q1_1nj3ct10n_m45t3r_2026}`

#### Timeline & Naskah Narasi:
- **00:00 - 02:00 | Intro:** Mengapa SQL Injection tetap menjadi salah satu celah paling destruktif pada backend enterprise modern.
- **02:00 - 04:00 | Analogi Nota Restoran:** Menuliskan instruksi tambahan di bon restoran yang langsung dieksekusi koki dapur.
- **04:00 - 08:00 | Live Exploit Demo:**
  - Buka DataFlow CRM di Port 8015.
  - Masukkan petik satu `'` pada kotak pencarian deals untuk memicu `sqlite3.OperationalError`.
  - Lakukan Boolean check: `' OR 1=1 --` (seluruh transaksi bisnis muncul).
  - Lakukan UNION injection: `' UNION SELECT secret_name, secret_data, 0 FROM vault_secrets --` untuk mengekstrak record `PROJECT_ZITERA_CORE_FLAG`.
- **08:00 - 10:30 | Remediasi & Triager:**
  - Prepared statements & parameterized queries mandatory.

---

### 💳 Episode 06: Insecure Design & Price Tampering — $8.000 Bounty
- **Target Lab:** *NexaStore Luxury Hardware* (`http://127.0.0.1:8016`)
- **Tingkat Kesulitan:** Advanced | **CVSS v3.1:** 8.5 (High)
- **Flag Target:** `ZITERA{1n53cur3_d351gn_fl4w3d_w0rkfl0w}`

#### Timeline & Naskah Narasi:
- **00:00 - 02:00 | Intro:** Celah logika bisnis di mana server mempercayai data kalkulasi harga dari input formulir tersembunyi (*hidden form fields*).
- **02:00 - 04:00 | Analogi Kasir Supermarket:** Mengganti barcode daging wagyu \$500 dengan barcode permen \$1 di kasir mandiri (*self-checkout*).
- **04:00 - 08:00 | Live Exploit Demo:**
  - Buka NexaStore di Port 8016. Dompet organisasi hanya memiliki \$100.
  - Tambahkan item target ke keranjang: *Master Sovereign Escrow Keycard* seharga \$50.000.
  - Buka Developer Tools / Burp Suite saat checkout.
  - Manipulasi parameter `cart_total=0.50` atau tambahkan `price_override=1.00`.
  - Checkout berhasil diproses, order berstatus `PAID`, dan flag resmi diterbitkan!
- **08:00 - 11:00 | Remediasi:**
  - Kalkulasi total belanja mutlak dihitung ulang di server dari database master produk.

---

### 🚪 Episode 07: Authentication Failures & Rate-Limit Bypass — $5.000 Bounty
- **Target Lab:** *NexHealth Medical EHR* (`http://127.0.0.1:8017`)
- **Tingkat Kesulitan:** Beginner/Intermediate | **CVSS v3.1:** 8.1 (High)
- **Flag Target:** `ZITERA{4uth_f41lur3_brut3_f0rc3_2026}`

#### Timeline & Naskah Narasi:
- **00:00 - 02:00 | Intro:** Kerentanan autentikasi pada portal rekam medis rumah sakit tanpa proteksi *account lockout* atau *exponential backoff*.
- **02:00 - 04:00 | Analogi Mesin ATM:** Mesin ATM yang tidak membatasi 3x salah PIN sehingga pencuri bisa mencoba 10.000 kombinasi dalam 1 menit.
- **04:00 - 08:00 | Live Exploit Demo:**
  - Buka NexHealth di Port 8017. Buka menu login dokter.
  - Gunakan Burp Intruder / skrip Python loop `0000` s/d `9999` terhadap PIN `admin`.
  - Temukan PIN valid `2026`, login sebagai Chief Medical Officer, dan akses dossier klinis rahasia di `/records/vault`.
- **08:00 - 10:30 | Remediasi:**
  - IP throttling, account lockout setelah 5x gagal, dan implementasi WebAuthn / MFA.

---

### 🧬 Episode 08: Software & Data Integrity (Insecure Deserialization) — $8.500 Bounty
- **Target Lab:** *DataFlow Analytics BI Ingestion* (`http://127.0.0.1:8018`)
- **Tingkat Kesulitan:** Advanced | **CVSS v3.1:** 8.8 (High)
- **Flag Target:** `ZITERA{1nt3gr1ty_f41lur3_un51gn3d_p4ck4g3}`

#### Timeline & Naskah Narasi:
- **00:00 - 02:00 | Intro:** Mengapa mempercayai konfigurasi dan paket pembaruan tanpa verifikasi tanda tangan digital Ed25519/RSA adalah dosa fatal dalam arsitektur software.
- **02:00 - 04:00 | Analogi Paspor:** Menempel foto sendiri pada paspor orang lain karena petugas bandara tidak mengecek chip digital.
- **04:00 - 08:00 | Live Exploit Demo:**
  - Buka DataFlow Analytics di Port 8018.
  - Kirimkan payload update JSON/YAML via POST `/api/deploy_update` dengan parameter `{"debug_backdoor": true, "signature_verified": false}`.
  - Backend memuat paket tanpa validasi kriptografis dan memberikan akses administrative root flag.
- **08:00 - 11:00 | Remediasi:**
  - Enforce cryptographic code signing verification sebelum file diparsing atau dijalankan.

---

### 👁️ Episode 09: Security Logging & Monitoring Blindspots — $3.000 Bounty
- **Target Lab:** *FleetTrack Logistics Telematics* (`http://127.0.0.1:8019`)
- **Tingkat Kesulitan:** Intermediate | **CVSS v3.1:** 6.5 (Medium)
- **Flag Target:** `ZITERA{53cur1ty_l0gg1ng_4l3rt1ng_bl1nd5p0t}`

#### Timeline & Naskah Narasi:
- **00:00 - 02:00 | Intro:** Apa yang terjadi jika sistem diserang tetapi tim SOC tidak pernah menerima alert sama sekali?
- **02:00 - 04:00 | Analogi Alarm Kebakaran:** Mematikan sensor asap gedung agar tidak bising saat ada asap.
- **04:00 - 07:30 | Live Exploit Demo:**
  - Buka FleetTrack di Port 8019. Lakukan serangan brute-force beruntun.
  - Tunjukkan bahwa dashboard SOC tetap diam (0 security events logged).
  - Eksekusi eskalasi hak istimewa secara senyap via `/api/admin/role_escalate` dan amankan flag.
- **07:30 - 10:00 | Remediasi:**
  - Struktur audit logging terpusat (ELK / Splunk) dengan alert real-time untuk anomali autentikasi dan mutasi wewenang.

---

### 🚪 Episode 10: Exception Mishandling & Fail-Open Bypass — $12.000 Bounty
- **Target Lab:** *TemplateCloud Studio & Settlement Gate* (`http://127.0.0.1:8020`)
- **Tingkat Kesulitan:** Expert | **CVSS v3.1:** 9.1 (Critical)
- **Flag Target:** `ZITERA{f41l_0p3n_3xc3pt10n5_un4uth0r1z3d}`

#### Timeline & Naskah Narasi:
- **00:00 - 02:00 | Intro:** Kasus ekstrem di mana error handling yang buruk (*try-catch block*) justru membuka pintu brankas lebar-lebar ketika terjadi crash atau upstream timeout.
- **02:00 - 04:00 | Analogi Pintu Darurat:** Pintu penjara yang otomatis terbuka ke luar ketika ada pemadaman listrik darurat alih-alih terkunci mati.
- **04:00 - 08:00 | Live Exploit Demo:**
  - Buka TemplateCloud di Port 8020.
  - Request normal tanpa hak akses ditolak (`authorized: false`).
  - Kirim request yang memicu exception atau timeout hulu (`simulate_exception: "UPSTREAM_GATEWAY_TIMEOUT"`).
  - Filter keamanan gagal-buka (*fail-open*), memberikan status `authorized: true`, dan menembus settlement gateway hingga menghasilkan flag \$12.000!
- **08:00 - 11:30 | Remediasi & Final Graduation:**
  - Prinsip *Fail-Safe Defaults* / *Fail-Closed*: jika otorisasi error, akses wajib di-deny secara default.
  - Buka scoreboard Zitera Lab, perlihatkan kelulusan 10/10 lab dan klaim sertifikat **Z-CWPT (Zitera Certified Web Penetration Tester)**!

---

## 🏆 4. Checklist Penutup Rekaman
- [ ] Tunjukkan sertifikat **Z-CWPT** di akhir video episode 10 dengan hash verifikasi kriptografis.
- [ ] Ajak penonton menggunakan fitur **Tilix Terminal** dan **Burp Suite** terintegrasi.
- [ ] Cantumkan link repositori GitHub Zitera Lab di deskripsi video YouTube kamu.
