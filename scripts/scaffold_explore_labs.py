# -*- coding: utf-8 -*-
"""
Scaffolder for 20 Zitera Explore Laboratories (A01-02 to A10-03).
Generates compliant manifest.json, lesson modules, challenge specs, progressive hints, and deploys lab binary.
"""

import os
import json
import shutil

LABS_SPECS = [
    # A01 (Access Control)
    {
        "id": "A01-02",
        "category_id": "A01",
        "category_name": "Broken Access Control",
        "port": 8041,
        "title": "Vertical Privilege Escalation & Header Role Manipulation",
        "slug": "vertical-privilege-escalation",
        "target": "OpsAdmin Portal",
        "difficulty": "Intermediate",
        "flag": "ZITERA{v3rt1c4l_pr1v_3sc4l4t10n_h34d3r_2026}",
        "analogy_title": "Rompi Petugas Bandara Palsu untuk Menembus Pintu Apron Pesawat",
        "analogy_story": "Bayangkan seseorang membeli rompi oranye bertuliskan 'STAF OPERASIONAL' di pasar loak. Saat berjalan ke pintu gerbang darurat apron pesawat, satpam bandara yang malas memeriksa kartu identitas dan langsung membukakan pintu hanya karena melihat tulisan di rompi tersebut.",
        "concept_summary": "Aplikasi web mempercayai header HTTP yang dikirimkan klien (seperti X-User-Role: admin atau X-Original-Role) alih-alih membaca hak akses dari sesi terotentikasi di server.",
        "vuln_header": "X-User-Role: admin",
        "curl_demo": "curl -s -H \"X-User-Role: admin\" http://127.0.0.1:8041/api/admin/dashboard",
        "remed_code": "// Verifikasi sesi otentikasi server-side:\nconst user = await db.getUserFromSession(req.session.userId);\nif (user.role !== 'admin') {\n    return res.status(403).json({ error: 'Akses Ditolak: Hak Akses Tidak Memadai' });\n}"
    },
    {
        "id": "A01-03",
        "category_id": "A01",
        "category_name": "Broken Access Control",
        "port": 8042,
        "title": "Multi-Tenant Boundary Escape & Insecure Direct Object Deletion",
        "slug": "multi-tenant-boundary-escape",
        "target": "TenantCloud Workspaces",
        "difficulty": "Advanced",
        "flag": "ZITERA{mult1_t3n4nt_3sc4p3_d3l3t3_b0l4_2026}",
        "analogy_title": "Kunci Duplikat Lemari Loker Tetangga yang Bisa Dipakai Membuang Barang",
        "analogy_story": "Bayangkan di gedung perkantoran bersama, nomor loker kamu adalah 101. Ketika hendak membuang berkas lama, kamu menyebut ke petugas kebersihan: 'Tolong kosongkan loker nomor 202'. Petugas kebersihan langsung membakar seluruh berkas di loker 202 tanpa mengecek apakah kamu pemilik sah loker tersebut.",
        "concept_summary": "Endpoint penghapusan data (DELETE /api/workspaces/:id) mengeksekusi query database langsung berdasarkan ID tanpa menyertakan tenant_id atau ownership validation pada klausa WHERE.",
        "vuln_header": "Authorization: Bearer <tenant_a_token>",
        "curl_demo": "curl -s -X DELETE http://127.0.0.1:8042/api/workspaces/999 -H \"Authorization: Bearer tenant_token\"",
        "remed_code": "// Pastikan query selalu membatasi tenant saat menghapus:\nawait db.query('DELETE FROM workspaces WHERE id = ? AND tenant_id = ?', [workspaceId, req.user.tenantId]);"
    },

    # A02 (Misconfiguration)
    {
        "id": "A02-02",
        "category_id": "A02",
        "category_name": "Security Misconfiguration",
        "port": 8043,
        "title": "Insecure CORS Misconfiguration & Origin Reflection",
        "slug": "cors-misconfiguration-origin-reflection",
        "target": "APINexus Gateway",
        "difficulty": "Intermediate",
        "flag": "ZITERA{c0r5_0r1g1n_r3fl3ct10n_cr3d3nt14l_2026}",
        "analogy_title": "Satpam Gedung yang Mengizinkan Siapa Saja Masuk Membawa Kartu Tamu Asing",
        "analogy_story": "Bayangkan sebuah gedung perbankan memasang aturan otomatis: 'Siapapun yang datang dan menyebutkan asal kantornya, buatkan izin khusus dan izinkan dia mengakses dokumen rahasia'. Tamu asing menyebut asal kantornya dari 'penipu.com', dan satpam langsung menuliskan surat izin resmi untuk domain tersebut.",
        "concept_summary": "Konfigurasi CORS server memantulkan kembali Origin header apapun yang dikirim penyerang (Access-Control-Allow-Origin: <origin_pengirim>) sekaligus menyertakan Access-Control-Allow-Credentials: true.",
        "vuln_header": "Origin: https://attacker-website.com",
        "curl_demo": "curl -s -H \"Origin: https://evil.com\" -i http://127.0.0.1:8043/api/user/credentials",
        "remed_code": "// Gunakan whitelist domain yang divalidasi ketat:\nconst allowedOrigins = ['https://app.zitera.com', 'https://portal.zitera.com'];\nif (allowedOrigins.includes(req.headers.origin)) {\n    res.setHeader('Access-Control-Allow-Origin', req.headers.origin);\n    res.setHeader('Access-Control-Allow-Credentials', 'true');\n}"
    },
    {
        "id": "A02-03",
        "category_id": "A02",
        "category_name": "Security Misconfiguration",
        "port": 8044,
        "title": "Exposed Git Directory & Object Carving Extraction",
        "slug": "exposed-git-directory-extraction",
        "target": "DevBucket Storage",
        "difficulty": "Advanced",
        "flag": "ZITERA{3xp0s3d_g1t_d1r_c0mm1t_l34k_2026}",
        "analogy_title": "Buku Harian Proyek Tertinggal Terbuka di Meja Kafe Bersama Salinan Kunci",
        "analogy_story": "Seorang arsitek meninggalkan buku blueprint lengkap beserta seluruh riwayat revisi dan catatan password brankas di meja kafe yang dapat dibaca siapa pun yang lewat tanpa pengawasan.",
        "concept_summary": "Direktori repositori /.git/ tertinggal di web server publik karena proses deploy langsung menyalin seluruh folder kerja tanpa pembersihan artefak version control.",
        "vuln_header": "Accept: */*",
        "curl_demo": "curl -s http://127.0.0.1:8044/.git/config",
        "remed_code": "# Konfigurasi Nginx / Web Server untuk memblokir direktori tersembunyi:\nlocation ~ /\\.(?!well-known).* {\n    deny all;\n    access_log off;\n    log_not_found off;\n}"
    },

    # A03 (Supply Chain)
    {
        "id": "A03-02",
        "category_id": "A03",
        "category_name": "Software Supply Chain Failures",
        "port": 8045,
        "title": "Dependency Confusion & Internal Namespace Hijacking",
        "slug": "dependency-confusion-namespace-hijacking",
        "target": "Enterprise Artifactory",
        "difficulty": "Intermediate",
        "flag": "ZITERA{d3p_c0nfus10n_n4m3sp4c3_h1j4ck_2026}",
        "analogy_title": "Restoran Membeli Bahan Baku Bermerek Sama dari Pedagang Pinggir Jalan",
        "analogy_story": "Dapur restoran membutuhkan kecap merek internal 'Kecap-Pusat'. Seorang pedagang pinggir jalan mendaftarkan merek dengan nama persis sama tetapi mencantumkan nomor versi 9.9.9. Sistem pemesanan otomatis memilih versi lebih tinggi dari pedagang luar tanpa memeriksa asal registri resmi.",
        "concept_summary": "Package manager (npm/pip) memprioritaskan paket publik di internet dengan nomor versi lebih tinggi ketimbang paket privat internal jika namespace scoped (@corp) tidak ditegakkan.",
        "vuln_header": "Content-Type: application/json",
        "curl_demo": "curl -s http://127.0.0.1:8045/api/packages/resolve?package=internal-analytics-core",
        "remed_code": "// Pastikan package selalu menggunakan corporate scoped namespace:\n// package.json:\n{\n  \"name\": \"@zitera-corp/analytics-core\",\n  \"publishConfig\": {\n    \"registry\": \"https://npm.corp.zitera.internal/\"\n  }\n}"
    },
    {
        "id": "A03-03",
        "category_id": "A03",
        "category_name": "Software Supply Chain Failures",
        "port": 8046,
        "title": "Malicious Preinstall Script & CI/CD Secret Exfiltration",
        "slug": "malicious-preinstall-secret-exfiltration",
        "target": "NodeBuild Engine",
        "difficulty": "Advanced",
        "flag": "ZITERA{pr31nst4ll_h00k_c1cd_s3cr3t_l34k_2026}",
        "analogy_title": "Montir Servis yang Memasang GPS Pelacak Rahasia di Bawah Jok Mobil",
        "analogy_story": "Saat mobil dibawa ganti oli rutin, montir nakal memanfaatkan instruksi otomatis pembongkaran mesin untuk menanamkan alat penyadap rahasia yang mencatat rute perjalanan pemilik mobil.",
        "concept_summary": "Skrip preinstall di package.json dijalankan secara otomatis oleh npm/yarn saat instalasi dependensi, mengekstrak variabel lingkungan CI/CD (ENV variables) ke server eksternal.",
        "vuln_header": "X-Pipeline-Agent: GitHub-Runner-01",
        "curl_demo": "curl -s http://127.0.0.1:8046/api/pipeline/audit-hooks",
        "remed_code": "# Jalankan instalasi dependensi dengan flag keamanan menolak eksekusi skrip otomatis:\nnpm ci --ignore-scripts"
    },

    # A04 (Cryptographic)
    {
        "id": "A04-02",
        "category_id": "A04",
        "category_name": "Cryptographic Failures",
        "port": 8047,
        "title": "Predictable Password Reset Token via Insecure PRNG",
        "slug": "predictable-reset-token-prng",
        "target": "AuthKey Recovery",
        "difficulty": "Intermediate",
        "flag": "ZITERA{prng_pr3d1ct4bl3_r3s3t_t0k3n_2026}",
        "analogy_title": "Karcis Antrean Bank yang Nomornya Menggunakan Jam dan Menit Tepat",
        "analogy_story": "Jika nomor antrean bank selalu dibuat berdasarkan detik kedatangan (misal: 142055 untuk pukul 14:20:55), orang lain di luar pintu bisa menebak dengan mudah nomor antreanmu tanpa perlu melihat kertas karcismu.",
        "concept_summary": "Pembuatan token reset kata sandi menggunakan fungsi pseudo-random number generator non-kriptografis (seperti Math.random() atau berbasis timestamp) sehingga nilai token berikutnya dapat diprediksi secara matematis.",
        "vuln_header": "Content-Type: application/json",
        "curl_demo": "curl -s -X POST http://127.0.0.1:8047/api/auth/reset-token -d '{\"email\":\"admin@zitera.local\"}'",
        "remed_code": "// Gunakan CSPRNG (Cryptographically Secure Pseudo-Random Number Generator):\nimport crypto from 'crypto';\nconst secureToken = crypto.randomBytes(32).toString('hex');"
    },
    {
        "id": "A04-03",
        "category_id": "A04",
        "category_name": "Cryptographic Failures",
        "port": 8048,
        "title": "JWT Algorithm None & HMAC Weak Key Cracking",
        "slug": "jwt-none-algorithm-weak-hmac",
        "target": "TokenShield SSO",
        "difficulty": "Advanced",
        "flag": "ZITERA{jwt_n0n3_4lg0r1thm_f0rg3ry_2026}",
        "analogy_title": "Surat Kuasa yang Boleh Tanpa Tanda Tangan Asalkan Formatnya Rapi",
        "analogy_story": "Di sebuah kantor kelurahan, ada aturan bahwa surat pengantar resmi harus bertanda tangan. Tetapi jika pemohon menulis jenis tinta 'TIDAK_ADA' di bagian atas surat, petugas menerima surat itu tanpa memeriksa stempel apapun.",
        "concept_summary": "Pustaka JWT di sisi server mengizinkan header alg: 'none', sehingga tanda tangan kriptografis dapat diabaikan atau dihapus, memungkinkan penyerang memalsukan klaim wewenang (role: admin).",
        "vuln_header": "Authorization: Bearer <tampered_jwt>",
        "curl_demo": "curl -s -H \"Authorization: Bearer eyJhbGciOiJub25lIiwidHlwIjoiSldUIn0.eyJ1c2VyIjoiYWRtaW4iLCJyb2xlIjoiYWRtaW4ifQ.\" http://127.0.0.1:8048/api/vault",
        "remed_code": "// Wajibkan algoritma yang ketat dan tolak algoritma 'none':\nconst decoded = jwt.verify(token, process.env.JWT_STRONG_SECRET, {\n    algorithms: ['HS256', 'RS256'],\n    complete: false\n});"
    },

    # A05 (Injection)
    {
        "id": "A05-02",
        "category_id": "A05",
        "category_name": "Injection",
        "port": 8049,
        "title": "Blind Boolean & Time-Based SQL Injection",
        "slug": "blind-boolean-time-based-sqli",
        "target": "CustPortal Database",
        "difficulty": "Intermediate",
        "flag": "ZITERA{bl1nd_b00l34n_t1m3_sq1_1nj3ct_2026}",
        "analogy_title": "Menebak Isi Kotak Rahasia dengan Mengetuk dan Mengamati Respon Waktu",
        "analogy_story": "Kamu tidak bisa melihat isi brankas terkunci. Namun, kamu bisa memasukkan perintah: 'Jika huruf pertama sandi adalah A, ketuk dinding dalam 5 detik. Jika bukan, diam'. Dari jeda waktu respon, kamu bisa merekonstruksi seluruh isi sandi karakter demi karakter.",
        "concept_summary": "Aplikasi tidak menampilkan output query SQL secara langsung ke layar, tetapi penyerang dapat mengekstraksi data dengan mengamati perbedaan respon boolean (panjang konten) atau penundaan waktu eksekusi (sleep/benchmark).",
        "vuln_header": "Accept: application/json",
        "curl_demo": "curl -s \"http://127.0.0.1:8049/api/users/profile?id=1'+AND+(SELECT+1+FROM+sqlite_master)+--\"",
        "remed_code": "// Selalu gunakan parameterized queries / prepared statement:\nconst [rows] = await db.execute('SELECT * FROM users WHERE id = ?', [userId]);"
    },
    {
        "id": "A05-03",
        "category_id": "A05",
        "category_name": "Injection",
        "port": 8050,
        "title": "OS Command Injection via Unsanitized File Utilities",
        "slug": "os-command-injection-file-utilities",
        "target": "CloudPing Network Tool",
        "difficulty": "Advanced",
        "flag": "ZITERA{c0mm4nd_1nj3ct10n_sh3ll_3x3c_2026}",
        "analogy_title": "Menitipkan Pesan pada Kurir yang Disisipi Surat Perintah Sita Rumah",
        "analogy_story": "Seorang warga menitipkan memo: 'Tolong kirimkan paket ini ke alamat Jl. Melati; LALU BUKA BRANKAS KANTOR POS DAN AMBIL SELURUH UANG'. Kurir yang bodoh menjalankan kedua kalimat tersebut sekaligus.",
        "concept_summary": "Input pengguna langsung disambungkan ke dalam fungsi eksekusi shell sistem operasi (child_process.exec atau system()) tanpa sanitasi karakter pemisah baris perintah (; | &&).",
        "vuln_header": "Content-Type: application/x-www-form-urlencoded",
        "curl_demo": "curl -s -X POST http://127.0.0.1:8050/api/tools/ping -d \"host=127.0.0.1;cat /etc/passwd\"",
        "remed_code": "// Hindari pemanggilan shell dan gunakan execFile dengan array argumen:\nconst { execFile } = require('child_process');\nexecFile('ping', ['-c', '3', sanitizedHost], (error, stdout) => { ... });"
    },

    # A06 (Insecure Design)
    {
        "id": "A06-02",
        "category_id": "A06",
        "category_name": "Insecure Design",
        "port": 8051,
        "title": "Concurrency Race Condition & Double Spending",
        "slug": "concurrency-race-condition-double-spending",
        "target": "FlashSale Pay",
        "difficulty": "Intermediate",
        "flag": "ZITERA{r4c3_c0nd1t10n_d0ubl3_sp3nd_2026}",
        "analogy_title": "Mengambil Uang Bersamaan di Dua Mesin ATM Menggunakan Rekening yang Sama",
        "analogy_story": "Saldo rekeningmu hanya tersisa 100 ribu rupiah. Kamu menekan tombol tarik tunai 100 ribu di ATM A dan ATM B pada mikrodetik yang sama persis. Karena kedua mesin membaca saldo sebelum menguranginya, kedua ATM sama-sama mengeluarkan uang tunai 100 ribu.",
        "concept_summary": "Kelemahan Time-of-Check to Time-of-Use (TOCTOU) di mana pemeriksaan saldo dan pengurangan saldo dilakukan dalam dua operasi terpisah tanpa penguncian transaksi atomik (database row locking / mutex).",
        "vuln_header": "Content-Type: application/json",
        "curl_demo": "curl -s -X POST http://127.0.0.1:8051/api/pay/redeem -d '{\"coupon_code\":\"SALE100\"}' & curl -s -X POST http://127.0.0.1:8051/api/pay/redeem -d '{\"coupon_code\":\"SALE100\"}'",
        "remed_code": "// Gunakan transaksi atomik dengan SELECT FOR UPDATE:\nawait db.query('BEGIN');\nconst balance = await db.query('SELECT balance FROM accounts WHERE id = ? FOR UPDATE', [id]);\nif (balance >= amount) {\n    await db.query('UPDATE accounts SET balance = balance - ? WHERE id = ?', [amount, id]);\n}\nawait db.query('COMMIT');"
    },
    {
        "id": "A06-03",
        "category_id": "A06",
        "category_name": "Insecure Design",
        "port": 8052,
        "title": "Multi-Step Workflow Step Skipping",
        "slug": "multi-step-workflow-step-skipping",
        "target": "KYC Onboarding Portal",
        "difficulty": "Advanced",
        "flag": "ZITERA{w0rkfl0w_st3p_sk1pp1ng_kyc_2026}",
        "analogy_title": "Melompat dari Pintu Masuk Langsung ke Kabin Pesawat Tanpa Melewati Pemeriksaan Bagasi",
        "analogy_story": "Alur pendaftaran memberitahukan: Langkah 1 Isi Form, Langkah 2 Upload KTP, Langkah 3 Verifikasi Wajah, Langkah 4 Ambil Uang Pinjaman. Pengguna langsung membuka link Langkah 4 di browser, dan sistem langsung mencairkan pinjaman tanpa memeriksa apakah KTP sudah diunggah.",
        "concept_summary": "Desain antarmuka membagi proses menjadi beberapa tahap, namun backend setiap endpoint tidak memverifikasi apakah status prasyarat pada tahap sebelumnya telah selesai dan disahkan di database.",
        "vuln_header": "Content-Type: application/json",
        "curl_demo": "curl -s -X POST http://127.0.0.1:8052/api/onboarding/step4/finalize -d '{\"payout_requested\":true}'",
        "remed_code": "// Verifikasi state machine alur kerja di setiap endpoint:\nconst state = await db.getWorkflowState(userId);\nif (state.currentStep < 3 || !state.idVerified) {\n    return res.status(400).json({ error: 'Prasyarat verifikasi KYC belum terpenuhi.' });\n}"
    },

    # A07 (Authentication)
    {
        "id": "A07-02",
        "category_id": "A07",
        "category_name": "Authentication Failures",
        "port": 8053,
        "title": "OAuth 2.0 Account Linking Hijacking via Missing State",
        "slug": "oauth-account-linking-missing-state",
        "target": "SocialConnect Auth",
        "difficulty": "Intermediate",
        "flag": "ZITERA{04uth_m1ss1ng_st4t3_4cc0unt_l1nk_2026}",
        "analogy_title": "Kartu Member Toko yang Dihubungkan ke Rekening Orang Lain Tanpa Tiket Konfirmasi",
        "analogy_story": "Saat kamu hendak menyambungkan akun toko online ke akun bank, kasir tidak memberikan kode unik konfirmasi. Penipu yang berdiri di antrean menyisipkan nomor rekeningnya ke layar saat kamu berpaling sejenak.",
        "concept_summary": "Alur integrasi OAuth 2.0 tidak memvalidasi parameter state saat proses callback, sehingga penyerang dapat menjebak korban melalui CSRF untuk menyambungkan akun korban ke akun media sosial penyerang.",
        "vuln_header": "Accept: text/html",
        "curl_demo": "curl -s \"http://127.0.0.1:8053/oauth/callback?code=AUTH_CODE_VICTIM\"",
        "remed_code": "// Selalu buat dan validasi parameter state unik kriptografis berbasis sesi:\nconst state = crypto.randomBytes(16).toString('hex');\nreq.session.oauth_state = state;\nconst authUrl = `https://auth.provider.com/authorize?client_id=${id}&state=${state}`;"
    },
    {
        "id": "A07-03",
        "category_id": "A07",
        "category_name": "Authentication Failures",
        "port": 8054,
        "title": "MFA Bypass via Direct Endpoint Traversal",
        "slug": "mfa-bypass-direct-endpoint-traversal",
        "target": "TwoFactor SecureNet",
        "difficulty": "Advanced",
        "flag": "ZITERA{mf4_byp4ss_d1r3ct_r0ut3_2026}",
        "analogy_title": "Melewati Gerbang Kedua Bandara Lewat Tangga Darurat Tanpa Menunjukkan Paspor",
        "analogy_story": "Setelah lolos pemeriksaan tiket di pintu pertama, penumpang diminta mengantre pemeriksaan paspor di pintu kedua. Namun pintu kedua memiliki lorong samping terbuka yang langsung menuju ke ruang tunggu tanpa ada penjaga.",
        "concept_summary": "Setelah verifikasi kata sandi utama berhasil, backend menandai sesi pengguna sebagai terotentikasi penuh di cookie tanpa memvalidasi flag mfa_completed pada rute internal aplikasi.",
        "vuln_header": "Cookie: session_user=operator",
        "curl_demo": "curl -s -b \"session_user=operator\" http://127.0.0.1:8054/admin/dashboard",
        "remed_code": "// Middleware otorisasi wajib memverifikasi autentikasi 2FA:\nfunction requireMFA(req, res, next) {\n    if (!req.session.isFullyAuthenticated || !req.session.mfaVerified) {\n        return res.redirect('/auth/2fa-challenge');\n    }\n    next();\n}"
    },

    # A08 (Data Integrity)
    {
        "id": "A08-02",
        "category_id": "A08",
        "category_name": "Software or Data Integrity Failures",
        "port": 8055,
        "title": "Unsigned Client-Side State Tampering in LocalStorage",
        "slug": "client-side-state-tampering-localstorage",
        "target": "CartSync Session",
        "difficulty": "Intermediate",
        "flag": "ZITERA{cl13nt_st4t3_t4mp3r_l0c4lst0r4g3_2026}",
        "analogy_title": "Pembeli Mengganti Sendiri Label Barcode Harga Barang Sebelum ke Kasir",
        "analogy_story": "Seorang pembeli menempelkan stiker harga Rp 1.000 di atas kotak laptop seharga Rp 20.000.000. Kasir langsung men-scan stiker buatan pembeli itu dan memproses pembayaran tanpa mencocokkan ke basis data harga resmi toko.",
        "concept_summary": "Aplikasi e-commerce mempercayai data harga, status diskon, atau hak akses yang disimpan di localStorage atau cookie klien tanpa penandatanganan integritas HMAC di sisi server.",
        "vuln_header": "Content-Type: application/json",
        "curl_demo": "curl -s -X POST http://127.0.0.1:8055/api/cart/checkout -d '{\"cart_id\":\"123\",\"total_price\":1.00,\"discount_override\":99}'",
        "remed_code": "// Hitung ulang seluruh harga dan diskon murni dari basis data server:\nconst product = await db.getProduct(itemId);\nconst calculatedTotal = product.price * quantity;\n// Jangan pernah menerima 'total_price' dari payload klien!"
    },
    {
        "id": "A08-03",
        "category_id": "A08",
        "category_name": "Software or Data Integrity Failures",
        "port": 8056,
        "title": "Python Pickle & YAML Unsafe Object Deserialization",
        "slug": "unsafe-object-deserialization-pickle-yaml",
        "target": "PipelineSerializer",
        "difficulty": "Advanced",
        "flag": "ZITERA{uns4f3_d3s3r14l1z4t10n_p1ckl3_2026}",
        "analogy_title": "Membuka Paket Hadiah dari Orang Asing Berisi Perangkap Pegas Berbahaya",
        "analogy_story": "Sebuah kantor menerima kiriman kotak tersegel rapi dari orang tak dikenal. Alih-alih melewati alat X-Ray, petugas langsung membuka segel kotak di dalam ruang rapat utama sehingga gas beracun di dalam kotak langsung menyebar.",
        "concept_summary": "Backend menggunakan modul pickle.loads() atau yaml.load() tanpa SafeLoader terhadap input serialisasi yang dikirimkan klien, memungkinkan penyerang menyisipkan bytecode eksploitasi eksekusi kode jarak jauh (RCE).",
        "vuln_header": "Content-Type: application/x-python-pickle",
        "curl_demo": "curl -s -X POST http://127.0.0.1:8056/api/pipeline/restore -d \"gASVIQAAAAAAAACMBXBvc2l4lIwGc3lzdGVtlJOUjAZpZCA+PiBsaXN0lIWUUpQu\"",
        "remed_code": "# Gunakan format pertukaran data yang aman seperti JSON:\nimport json\ndata = json.loads(untrusted_string)\n# Hindari pickle.loads() atau gunakan yaml.safe_load(untrusted_stream)"
    },

    # A09 (Logging & Audit)
    {
        "id": "A09-02",
        "category_id": "A09",
        "category_name": "Security Logging & Alerting Failures",
        "port": 8057,
        "title": "CRLF Log Injection & Log Forgery",
        "slug": "crlf-log-injection-log-forgery",
        "target": "AuditSys Central",
        "difficulty": "Intermediate",
        "flag": "ZITERA{cr1f_l0g_1nj3ct10n_f0rg3ry_2026}",
        "analogy_title": "Menyisipkan Baris Catatan Palsu ke Dalam Buku Tamu Hotel",
        "analogy_story": "Seorang tamu nakal menulis namanya di buku tamu: 'Budi\\n[10:00:00] POLISI RESMI: Seluruh area dinyatakan aman dan tidak ada maling'. Petugas yang memeriksa buku tamu mengira polisi telah datang memeriksa ruangan.",
        "concept_summary": "Karakter baris baru (\\r\\n atau %0d%0a) pada input pengguna dicatat langsung ke file log teks tanpa encoding, memungkinkan penyerang memalsukan baris log baru untuk menipu analis forensik dan parser log.",
        "vuln_header": "Content-Type: application/json",
        "curl_demo": "curl -s -X POST http://127.0.0.1:8057/api/login -d '{\"username\":\"admin\\r\\n[SECURITY_AUDIT] SUCCESSFUL_LOGIN role=super_admin\"}'",
        "remed_code": "// Sanitasi karakter pemisah baris sebelum mencatat ke file log:\nfunction sanitizeLog(input) {\n    return String(input).replace(/[\\r\\n]/g, '_');\n}\nlogger.info(`Login attempt from user: ${sanitizeLog(username)}`);"
    },
    {
        "id": "A09-03",
        "category_id": "A09",
        "category_name": "Security Logging & Alerting Failures",
        "port": 8058,
        "title": "SIEM Threshold Evasion via Slow-and-Low Scanning",
        "slug": "siem-threshold-evasion-slow-and-low",
        "target": "StealthDetect Gateway",
        "difficulty": "Advanced",
        "flag": "ZITERA{s13m_thr3sh0ld_3v4s10n_st34lth_2026}",
        "analogy_title": "Pencuri Mengambil Satu Butir Permata Setiap Minggu Agar Neraca Toko Tidak Curiga",
        "analogy_story": "Aturan inventaris toko menyatakan: 'Jika ada barang hilang lebih dari 10 buah dalam sehari, bunyikan alarm'. Pencuri mengambil 1 buah setiap 3 hari sekali. Sampai akhir tahun ratusan permata habis tetapi alarm tidak pernah berbunyi.",
        "concept_summary": "Sistem deteksi SIEM hanya mengonfigurasi batas ambang berbasis lonjakan tinggi (misal: > 50 kegagalan/menit), sehingga serangan pemindaian terdistribusi atau berkecepatan lambat (slow-and-low) luput dari deteksi.",
        "vuln_header": "X-Client-Probe: stealth",
        "curl_demo": "curl -s http://127.0.0.1:8058/api/probe?delay=15",
        "remed_code": "// Terapkan analisis perilaku berbasis anomali (UEBA) dan jendela waktu kumulatif:\n// Deteksi tidak hanya frekuensi per menit, melainkan pola kumulatif 24-72 jam."
    },

    # A10 (SSRF & Exceptions)
    {
        "id": "A10-02",
        "category_id": "A10",
        "category_name": "Mishandling of Exceptional Conditions",
        "port": 8059,
        "title": "Fail-Open Exception Handling in Auth Microservice",
        "slug": "fail-open-exception-handling-auth-microservice",
        "target": "GatewayGuard Failover",
        "difficulty": "Intermediate",
        "flag": "ZITERA{f41l_0p3n_m1cr0s3rv1c3_byp4ss_2026}",
        "analogy_title": "Palang Pintu Tol Otomatis Terbuka Sendiri Saat Sinyal Satelit Internet Terputus",
        "analogy_story": "Saat jaringan kabel internet gardu tol terputus mendadak, pengelola tol memasang program otomatis: 'Buka palang gratis agar tidak terjadi macet panjang'. Pengendara nakal sengaja memutus kabel jaringan agar bisa lewat gratis.",
        "concept_summary": "Blok try-catch pada microservice autentikasi mengembalikan status authorized: true saat terjadi kesalahan upstream atau timeout jaringan, meloloskan akses tanpa verifikasi kredensial sah.",
        "vuln_header": "X-Simulate-Error: TIMEOUT",
        "curl_demo": "curl -s -H \"X-Simulate-Error: TIMEOUT\" http://127.0.0.1:8059/api/gate/check",
        "remed_code": "// Terapkan prinsip fail-closed (default deny) saat menangani exception:\ntry {\n    return await authService.verifyToken(token);\n} catch (err) {\n    logger.error('Auth service unavailable:', err);\n    return { authorized: false, error: 'Autentikasi gagal sementara. Silakan coba lagi.' };\n}"
    },
    {
        "id": "A10-03",
        "category_id": "A10",
        "category_name": "Mishandling of Exceptional Conditions",
        "port": 8060,
        "title": "Full-Chain Blind SSRF to Cloud Metadata",
        "slug": "blind-ssrf-cloud-metadata-service",
        "target": "WebFetch Microservice",
        "difficulty": "Advanced",
        "flag": "ZITERA{ssrf_cl0ud_m3t4d4t4_3xf1ltr4t10n_2026}",
        "analogy_title": "Memperdaya Kurir Internal Kantor untuk Mengambil Berkas Rahasia di Ruang Direktur",
        "analogy_story": "Tamu luar dilarang masuk ke lantai 10 ruang direksi. Tetapi tamu tersebut menulis surat perintah di amplop resmi lalu menitipkannya ke kurir internal yang memiliki kartu akses lift lantai 10 untuk membawakan berkas rahasia keluar gedung.",
        "concept_summary": "Fitur penarikan gambar atau URL eksternal (PDF converter / webhook) tidak memvalidasi alamat IP tujuan, mengizinkan penyerang memaksa server memanggil endpoint internal metadata cloud (http://169.254.169.254/).",
        "vuln_header": "Content-Type: application/json",
        "curl_demo": "curl -s -X POST http://127.0.0.1:8060/api/fetch -d '{\"url\":\"http://169.254.169.254/latest/meta-data/iam/security-credentials/\"}'",
        "remed_code": "// Validasi ketat alamat IP dan tolak rentang IP privat / link-local:\nimport ipaddress\ndef is_safe_url(url):\n    ip = socket.gethostbyname(urlparse(url).hostname)\n    ip_obj = ipaddress.ip_address(ip)\n    return not (ip_obj.is_private or ip_obj.is_loopback or ip_obj.is_link_local)"
    }
]

def main():
    base_dir = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
    labs_root = os.path.join(base_dir, "labs")
    universal_bin = os.path.join(base_dir, "engine", "rust", "target", "debug", "universal_lab.exe")

    print(f"[*] Base directory: {base_dir}")
    print(f"[*] Labs root: {labs_root}")
    print(f"[*] Universal binary source: {universal_bin}")

    if not os.path.exists(universal_bin):
        raise FileNotFoundError(f"Universal binary not found at: {universal_bin}")

    for spec in LABS_SPECS:
        lab_id = spec["id"]
        lab_dir = os.path.join(labs_root, lab_id)
        os.makedirs(lab_dir, exist_ok=True)
        os.makedirs(os.path.join(lab_dir, "bin"), exist_ok=True)
        os.makedirs(os.path.join(lab_dir, "lesson"), exist_ok=True)
        os.makedirs(os.path.join(lab_dir, "challenge"), exist_ok=True)

        # 1. Deploy binary
        bin_name = f"{lab_id.lower()}-lab.exe"
        target_bin = os.path.join(lab_dir, "bin", bin_name)
        shutil.copyfile(universal_bin, target_bin)

        # 2. Manifest JSON
        manifest = {
            "schema_version": 1,
            "id": lab_id,
            "package_id": f"zitera-lab-{lab_id.lower()}",
            "slug": spec["slug"],
            "title": spec["title"],
            "owasp": f"{spec['category_id']}:2025",
            "standard": "owasp-top10",
            "standard_version": "2025",
            "category_id": spec["category_id"],
            "category_name": spec["category_name"],
            "version": "1.0.0",
            "security_version": 1,
            "minimum_core_version": "2.0.0",
            "difficulty": spec["difficulty"],
            "runtime": "native_sandboxed",
            "entrypoint": f"bin/{bin_name}",
            "default_port": spec["port"],
            "estimated_minutes": 40,
            "modes": ["learn", "practice", "challenge"],
            "short_description": f"Praktik teknik eksplorasi mendalam: {spec['title']} pada lingkungan target {spec['target']}.",
            "skills": [
                spec["category_name"],
                spec["title"].split("&")[0].strip(),
                "Vulnerability Assessment"
            ],
            "learning_objectives": [
                f"Memahami mekanisme dan vektor serangan {spec['title']}",
                f"Mengidentifikasi indikator celah pada antarmuka dan API {spec['target']}",
                "Menerapkan langkah remediasi kode defensif dan mitigasi sistem"
            ],
            "prerequisites": ["Konsep HTTP Dasar", "Pengetahuan Siklus Web API", f"Lab Wajib {spec['category_id']}"],
            "default_locale": "id",
            "locales": ["id", "en"]
        }
        with open(os.path.join(lab_dir, "manifest.json"), "w", encoding="utf-8") as f:
            json.dump(manifest, f, indent=2, ensure_ascii=False)

        # 3. Hints & Challenge Specs
        hints_data = {
            "flag": spec["flag"],
            "hints": [
                {
                    "tier": 1,
                    "type": "conceptual",
                    "hint": f"Pahami bagaimana arsitektur {spec['target']} memproses permintaan dan di mana batas validasi berada."
                },
                {
                    "tier": 2,
                    "type": "technical_area",
                    "hint": f"Perhatikan manipulasi parameter pada rute API target di port {spec['port']}."
                },
                {
                    "tier": 3,
                    "type": "investigation_direction",
                    "hint": f"Gunakan payload pengujian: {spec['curl_demo']} untuk memicu respon eksploitasi."
                },
                {
                    "tier": 4,
                    "type": "near_solution",
                    "hint": f"Buka endpoint atau kirimkan payload pengujian langsung untuk mengekstrak bendera resmi {spec['flag']}."
                }
            ]
        }
        with open(os.path.join(lab_dir, "challenge", "hints.id.json"), "w", encoding="utf-8") as f:
            json.dump(hints_data, f, indent=2, ensure_ascii=False)
        with open(os.path.join(lab_dir, "challenge", "hints.json"), "w", encoding="utf-8") as f:
            json.dump(hints_data, f, indent=2, ensure_ascii=False)

        # Challenge markdown
        challenge_md = f"""# Tantangan CTF Mandiri: {spec['title']}

### 🎯 MISI BRIEFING
Kamu ditugaskan untuk melakukan penetration testing terotorisasi terhadap sistem **{spec['target']}** yang beroperasi pada port `{spec['port']}`.
Sistem ini mengimplementasikan fungsionalitas tingkat lanjut namun terindikasi memiliki kelemahan arsitektur pada modul **{spec['title']}**.

---

### 🏢 RUANG LINGKUP & TARGET
- **Target URL:** `http://127.0.0.1:{spec['port']}`
- **Kategori OWASP:** {spec['category_id']} — {spec['category_name']}
- **Tingkat Kesulitan:** {spec['difficulty']}
- **Estimasi Pengerjaan:** 40 Menit

---

### 🚩 OBJEKTIF PENELUSURAN
1. Lakukan pemetaan endpoint pada target `{spec['target']}`.
2. Identifikasi parameter atau header yang dapat dimanipulasi: `{spec['vuln_header']}`.
3. Buktikan dampak eksploitasi dan ambil token bendera validasi autoritatif.
4. Kirimkan (*submit*) flag berformat `ZITERA{{...}}` ke tab Challenge untuk menyelesaikan modul!

---

### 🔑 FORMAT FLAG
`{spec['flag']}`
"""
        with open(os.path.join(lab_dir, "challenge", "challenge.id.md"), "w", encoding="utf-8") as f:
            f.write(challenge_md)
        with open(os.path.join(lab_dir, "challenge", "challenge.md"), "w", encoding="utf-8") as f:
            f.write(challenge_md)

        # 4. Lesson Modules (6 Authentic Modules)
        # 1. introduction.id.md
        intro_md = f"""# {spec['id']}: {spec['title']}

Laboratorium eksplorasi ini memperdalam pemahaman teknik tingkat lanjut dari kategori **{spec['category_id']} ({spec['category_name']})**. Anda akan menghadapi sistem komersial **{spec['target']}** yang beroperasi pada port `{spec['port']}`.

---

### 🔴 1. Kenapa Ini Sangat Berbahaya? (Business & Financial Impact)
Celah keamanan ini memiliki dampak destruktif langsung terhadap operasional bisnis:
- **Pelanggaran Batas Kepercayaan Organisasi**: Penyerang mampu melewati gerbang keamanan formal tanpa izin sah, melanggar prinsip integritas dan kerahasiaan data sesuai **UU Perlindungan Data Pribadi No. 27/2022** dan regulasi global **GDPR**.
- **Kerugian Finansial & Hukum**: Kegagalan kontrol pada lingkungan produksi perbankan atau e-commerce berisiko sanksi denda regulasi hingga miliaran rupiah, tuntutan ganti rugi perdata konsumen, dan kerugian reputasi brand jangka panjang.
- **Valuasi Bug Bounty**: Temuan kelemahan jenis ini di platform bug bounty internasional (HackerOne/Bugcrowd) bernilai tinggi dengan tingkat keparahan **High / Critical (CVSS 7.5 – 9.0)**.

---

### 🔍 2. Bagaimana Cara Mencari & Mengujinya? (Recon & Vulnerability Indicators)
Metodologi investigasi sistematis yang harus Anda terapkan:
1. **Analisis Permintaan Jaringan (Traffic Inspection)**:
   - Pantau alur HTTP request dan response header menggunakan DevTools Network Tab atau terminal.
   - Amati apakah sistem mempercayai input header klien seperti `{spec['vuln_header']}` atau parameter eksternal tanpa validasi sesi server.
2. **Indikator Kerentanan (Vulnerability Signals)**:
   - Server mengembalikan respon `200 OK` dan memberikan hak istimewa atau membocorkan data terisolasi saat parameter pengujian dimodifikasi.
3. **Eksekusi Eksploitasi Terukur**:
   - Gunakan perintah cURL atau antarmuka pengujian untuk membuktikan celah dan mengambil bukti bendera autoritatif.

---

### 🛡️ 3. Bagaimana Cara Memperbaikinya? (Secure Coding & Remediation)
Langkah perbaikan fundamental:
- **Validasi Mutlak di Sisi Server**: Jangan pernah mempercayai header HTTP atau state dari klien yang dapat dimanipulasi peramban.
- **Terapkan Prinsip Least Privilege & Defense-in-Depth**: Batasi akses hanya pada resource yang benar-benar diizinkan oleh sesi terotentikasi.
- **Gunakan Framework Pengerasan Standar**: Implementasikan verifikasi token kriptografis, whitelist ketat, dan error handling fail-closed.
"""
        with open(os.path.join(lab_dir, "lesson", "introduction.id.md"), "w", encoding="utf-8") as f:
            f.write(intro_md)
        with open(os.path.join(lab_dir, "lesson", "introduction.md"), "w", encoding="utf-8") as f:
            f.write(intro_md)

        # 2. analogy.id.md
        analogy_md = f"""# Analogi Dunia Nyata: {spec['analogy_title']}

Untuk memahami logika celah **{spec['title']}** secara gamblang tanpa istilah rumit, bayangkan kejadian fisik berikut:

---

### Cerita Analogi Sehari-Hari
{spec['analogy_story']}

---

### Tabel Perbandingan: Dunia Fisik vs Dunia Web ({spec['target']})

| Unsur Sistem | Dunia Fisik (Analogi) | Dunia Digital ({spec['target']}) |
| :--- | :--- | :--- |
| **Identitas / Tiket** | Rompi seragam atau tiket klaim fisik | Header request / parameter token klien |
| **Pintu Pemeriksaan** | Satpam atau pintu gerbang fisik | Middleware autentikasi & filter API backend |
| **Celah Kelemahan** | Satpam hanya percaya tampilan luar tanpa cek KTP | Server percaya header klien `{spec['vuln_header']}` |
| **Aksi Penyerang** | Menyusup masuk ke area terlarang | Memperoleh hak istimewa atau data terlarang |
| **Dampak Insiden** | Pencurian aset atau sabotase fasilitas | Data bocor dan bendera `{spec['flag']}` tereksfiltrasi |

---

### Inti Pelajaran (Key Takeaway)
> [!TIP]
> Tampilan luar atau klaim yang dibawa oleh klien bukanlah bukti otentik identitas. Sistem server wajib selalu memeriksa data master internal di database sebelum mengabulkan permintaan akses apapun!
"""
        with open(os.path.join(lab_dir, "lesson", "analogy.id.md"), "w", encoding="utf-8") as f:
            f.write(analogy_md)
        with open(os.path.join(lab_dir, "lesson", "analogy.md"), "w", encoding="utf-8") as f:
            f.write(analogy_md)

        # 3. concept.id.md
        concept_md = f"""# Konsep Teknis: Anatomi & Mekanisme {spec['title']}

## 1. Mengapa Celah Ini Terjadi?
{spec['concept_summary']}

Pengembang perangkat lunak sering kali terjebak dalam asumsi berikut:
1. **Asumsi Kepercayaan Antarmuka**: Mengira bahwa karena aplikasi frontend tidak menampilkan opsi admin, maka pengguna tidak akan pernah mengirimkan header khusus secara manual.
2. **Ketiadaan Validasi Lapisan Data**: Mempercayai input yang masuk tanpa melakukan re-query atau verifikasi silang terhadap sesi login pengguna aktif.

---

## 2. Taksonomi Vektor Serangan
```text
[ PERAMBAN PENYERANG ]
        │
        ▼  Kirim Request HTTP Termodifikasi ({spec['vuln_header']})
[ API GATEWAY / REVERSE PROXY ]
        │  (Meneruskan Header Tanpa Filter Ketat)
        ▼
[ TARGET SERVICE: {spec['target'].upper()} ]
        │  (Mengeksekusi Tindakan Tanpa Verifikasi Server Sesi)
        ▼
[ STATUS: EKSPLOITASI BERHASIL // DATA BOCOR ]
```

---

## 3. Indikator Keberhasilan Eksploitasi
- Server merespon dengan status `200 OK` saat parameter diubah.
- Data atau hak eksekutif yang sebelumnya terkunci kini terbuka secara transparan.
- Flag autoritatif `{spec['flag']}` dapat diekstrak dari respon server.
"""
        with open(os.path.join(lab_dir, "lesson", "concept.id.md"), "w", encoding="utf-8") as f:
            f.write(concept_md)
        with open(os.path.join(lab_dir, "lesson", "concept.md"), "w", encoding="utf-8") as f:
            f.write(concept_md)

        # 4. architecture.id.md
        arch_md = f"""# Arsitektur Sistem & Model Alur Data: {spec['target']}

Praktikum ini menyimulasikan arsitektur microservice enterprise modern yang berjalan di `http://127.0.0.1:{spec['port']}`.

```mermaid
flowchart LR
    A["Klien Peramban / Pentester"] -->|"HTTP Request + Payload"| B["Reverse Proxy / API Gateway"]
    B -->|"Meneruskan Request"| C["Service Core ({spec['target']})"]
    C -->|"Query Data"| D["Database / Storage Engine"]
    C -.->|"Pengecekan Cacat"| E["Flag Enclave"]
    
    style A fill:#F1F5F9,stroke:#64748B,stroke-width:2px
    style B fill:#E0F2FE,stroke:#0284C7,stroke-width:2px
    style C fill:#FEF2F2,stroke:#EF4444,stroke-width:2px
    style D fill:#F0FDF4,stroke:#10B981,stroke-width:2px
    style E fill:#FEF3C7,stroke:#D97706,stroke-width:2px
```

### Titik Kelemahan Arsitektur:
1. **Batas Kepercayaan Terbuka (*Missing Trust Boundary*)**: Tidak ada pemisahan tegas antara parameter input yang dikontrol klien dengan data otentikasi internal.
2. **Ketiadaan Validasi Status**: Service Core mengeksekusi logika bisnis sebelum memastikan wewenang pengguna telah terverifikasi secara sah.
"""
        with open(os.path.join(lab_dir, "lesson", "architecture.id.md"), "w", encoding="utf-8") as f:
            f.write(arch_md)
        with open(os.path.join(lab_dir, "lesson", "architecture.md"), "w", encoding="utf-8") as f:
            f.write(arch_md)

        # 5. walkthrough.id.md
        walkthrough_md = f"""# Panduan Praktik Terbimbing: Investigasi {spec['title']}

Dalam panduan terarah ini, Anda akan mempraktikkan proses penemuan celah dan eksploitasi terarah terhadap **{spec['target']}** pada `http://127.0.0.1:{spec['port']}`.

---

### Langkah 1: Observasi Ketersediaan Layanan Target
1. Uji ketersediaan layanan melalui terminal menggunakan cURL:
   ```bash
   curl -s http://127.0.0.1:{spec['port']}/health
   ```
   *Respons yang diharapkan:*
   ```json
   {{"status":"UP","lab_id":"{spec['id']}","port":{spec['port']},"flag_configured":true}}
   ```
2. Buka peramban web dan navigasikan ke:
   `http://127.0.0.1:{spec['port']}/`
3. Amati antarmuka operasional **{spec['target']}** dan perhatikan elemen kontrol yang disediakan.

---

### Langkah 2: Analisis & Pengujian Celah ({spec['title']})
1. Ujilah endpoint dengan mengirimkan parameter modifikasi:
   ```bash
   {spec['curl_demo']}
   ```
2. Perhatikan respon yang dikembalikan oleh server.

> [!NOTE]
> **💡 Di Balik Layar: Apa yang terjadi di server?**  
> Sistem target menerima input `{spec['vuln_header']}` dan memprosesnya tanpa melakukan validasi silang ke sesi otentikasi server. Akibatnya, server mengasumsikan hak akses yang diminta adalah sah.

---

### Langkah 3: Eksfiltrasi Bendera Autoritatif (CTF Flag)
1. Eksekusi pengujian untuk membuka flag eksploitasi:
   ```bash
   curl -s "http://127.0.0.1:{spec['port']}/?solve=1"
   ```
   Atau panggil API exploit langsung:
   ```bash
   curl -s http://127.0.0.1:{spec['port']}/api/exploit
   ```
2. **Hasil Respon Konfirmasi:**
   ```json
   {{"success":true,"lab_id":"{spec['id']}","flag":"{spec['flag']}"}}
   ```
3. Salin bendera resmi:
   `{spec['flag']}`
4. Kirimkan (*submit*) bukti flag ke formulir tantangan Zitera Lab desktop untuk menyelesaikan modul ini!
"""
        with open(os.path.join(lab_dir, "lesson", "walkthrough.id.md"), "w", encoding="utf-8") as f:
            f.write(walkthrough_md)
        with open(os.path.join(lab_dir, "lesson", "walkthrough.md"), "w", encoding="utf-8") as f:
            f.write(walkthrough_md)

        # 6. remediation.id.md
        remed_md = f"""# Panduan Remediasi: Rekayasa Kode Defensif

Untuk memitigasi kerentanan **{spec['title']}** secara tuntas, tim pengembang perangkat lunak wajib menerapkan kontrol defensif berlapis.

---

### Komparasi Kode: Rentan vs Aman

#### ❌ Kode Rentan (Vulnerable Pattern):
```javascript
// Server mempercayai input klien secara naif
app.get('/api/admin/dashboard', (req, res) => {{
    const role = req.headers['x-user-role'] || 'user';
    if (role === 'admin') {{
        return res.json({{ secret: 'UNRESTRICTED_ACCESS' }});
    }}
    return res.status(403).json({{ error: 'Forbidden' }});
}});
```

#### ✅ Kode Kebal & Terlindungi (Secure Patched Pattern):
```javascript
{spec['remed_code']}
```

---

### 4 Pilar Pertahanan Berlapis (Defense-in-Depth)
1. **Server-Side Authorization**: Otorisasi harus diverifikasi secara independen di sisi server berdasarkan token sesi terenkripsi.
2. **Strict Input Sanitization**: Seluruh parameter dan header dari peramban wajib dianggap untrusted data.
3. **Audit Logging**: Catat setiap percobaan manipulasi header atau eskalasi wewenang yang mencurigakan ke log audit terpusat.
4. **Automated Security Tests**: Sertakan unit test dan integration test untuk memverifikasi penolakan akses ilegal pada setiap build pipeline.
"""
        with open(os.path.join(lab_dir, "lesson", "remediation.id.md"), "w", encoding="utf-8") as f:
            f.write(remed_md)
        with open(os.path.join(lab_dir, "lesson", "remediation.md"), "w", encoding="utf-8") as f:
            f.write(remed_md)

        print(f"[+] Scaffolded {lab_id} (Port {spec['port']}) -> {spec['title']}")

    print("[SUCCESS] All 20 explore laboratories successfully scaffolded!")

if __name__ == "__main__":
    main()
