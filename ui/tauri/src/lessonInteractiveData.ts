// Canonical Interactive Curriculum Data for Zitera Lab (A01 - A10)
// Powers tactile 3D Analogy, Core Concept, Interactive Walkthrough, and Remediation cards.

export interface AnalogyComparisonRow {
  physical: string
  digital: string
  takeaway: string
}

export interface AnalogySectionData {
  storyTitle: string
  icon: string
  realWorldScenario: string
  comparisonMatrix: AnalogyComparisonRow[]
  intuitiveTakeaway: string
}

export interface ConceptSectionData {
  whyItExists: Array<{
    title: string
    explanation: string
    mistake: string
  }>
  businessImpact: {
    regulatory: string
    financial: string
    reputational: string
    bountyReward: string
  }
  attackerVsDefender: {
    attackerPerspective: string
    defenderPerspective: string
  }
}

export interface WalkthroughStep {
  stepNumber: number
  badge: string
  title: string
  description: string
  endpoint?: string
  method?: string
  command?: string
  expectedStatus?: string
  expectedResponse?: string
  underTheHood: string
}

export interface WalkthroughSectionData {
  summary: string
  targetBaseUrl: string
  steps: WalkthroughStep[]
}

export interface RemediationSectionData {
  title: string
  vulnerableSnippet: {
    language: string
    filename: string
    code: string
    flawExplanation: string
  }
  secureSnippet: {
    language: string
    filename: string
    code: string
    fixExplanation: string
  }
  securityDiffs: Array<{
    component: string
    before: string
    after: string
    impact: string
  }>
  defensePrinciples: Array<{
    number: number
    title: string
    desc: string
  }>
}

export interface LabInteractiveCurriculum {
  labId: string
  analogy: AnalogySectionData
  concept: ConceptSectionData
  walkthrough: WalkthroughSectionData
  remediation: RemediationSectionData
}

export const LAB_INTERACTIVE_REGISTRY: Record<string, LabInteractiveCurriculum> = {
  "A01": {
    labId: "A01",
    analogy: {
      storyTitle: "Kartu Kunci Hotel #302 & Penitipan Koper",
      icon: "🏨 🔑",
      realWorldScenario:
        "Saat menginap di hotel berbintang, kamu menunjukkan KTP di meja resepsionis. Resepsionis memvalidasi identitasmu dan menyerahkan kartu kunci Kamar #302. Kunci ini hanya bisa membuka kamar #302. Sekarang bayangkan jika hotel tersebut memasang kunci digital via aplikasi ponsel dengan bug fatal: aplikasi ponsel bertanya 'Kamar nomor berapa yang ingin kamu buka?'. Jika kamu mengubah angka 302 menjadi 303 (kamar tamu lain) atau 101 (Presidential Suite), pintu langsung berbunyi klik dan terbuka!",
      comparisonMatrix: [
        {
          physical: "Resepsionis memvalidasi KTP tamu di lobi hotel.",
          digital: "User login dengan username & password sah (Authentication).",
          takeaway: "Identitas pengguna berhasil terverifikasi sah.",
        },
        {
          physical: "Tamu memodifikasi nomor kamar di aplikasi ponsel menjadi 303.",
          digital: "User memanipulasi parameter URL dari /wire/1 menjadi /wire/2.",
          takeaway: "Input klien tidak boleh dipercaya mentah-mentah.",
        },
        {
          physical: "Kunci pintu membuka kamar orang lain tanpa memeriksa izin.",
          digital: "Server langsung mengembalikan data pengguna lain tanpa cek sesi.",
          takeaway: "Celah Broken Access Control (IDOR) terjadi!",
        },
      ],
      intuitiveTakeaway:
        "Otentikasi ('Siapa Kamu') bukanlah Otorisasi ('Apa Hakmu'). Mengetahui identitas Alice yang sah tidak memberikan izin bagi Alice untuk membuka catatan transaksi milik Bob hanya karena nomor ID pada URL diganti!",
    },
    concept: {
      whyItExists: [
        {
          title: "UI-Centric Assumption (Asumsi Keliru Tampilan)",
          explanation:
            "Developer berasumsi karena tombol di layar Alice hanya bertuliskan 'Lihat Faktur #1', Alice tidak akan pernah mencoba membuka faktur #2 atau #42. Padahal penyerang memanipulasi URL langsung!",
          mistake: "Mengandalkan proteksi tombol antarmuka alih-alih pengecekan server.",
        },
        {
          title: "Rasa Aman Palsu dari @login_required",
          explanation:
            "Framework modern menyediakan decorator autentikasi. Ketika fungsi API sudah dipasangi cek sesi login, developer merasa pekerjaannya selesai, padahal kepemilikan objek belum dicek.",
          mistake: "Menganggap pengguna yang sudah login pasti berhak melihat seluruh data database.",
        },
        {
          title: "Pola Query Database Tanpa Tenant Scoping",
          explanation:
            "Query SQL hanya menyaring WHERE id = :id tanpa mengikat WHERE user_id = :session_user_id.",
          mistake: "SELECT * FROM wire_transfers WHERE id = :target_id;",
        },
      ],
      businessImpact: {
        regulatory: "Denda s.d. 2% pendapatan tahunan (UU PDP No. 27/2022) & s.d. €20 Juta (GDPR).",
        financial: "Kebocoran rahasia perbendaharaan perbankan & risiko transfer dana ilegal.",
        reputational: "Kehilangan izin operasional fintech dari otoritas moneter & distrust investor.",
        bountyReward: "Bounty P1/Critical pada program bug bounty global: $1,500 – $8,000+.",
      },
      attackerVsDefender: {
        attackerPerspective:
          "Saya menemukan parameter numerik /wire/1. Jika saya ganti menjadi /wire/2 atau /wire/42, apakah server memvalidasi kepemilikan akun saya?",
        defenderPerspective:
          "Setiap request yang meminta ID data wajib diperiksa di sisi server: apakah session['user_id'] aktif benar-benar pemilik sah data nomor tersebut?",
      },
    },
    walkthrough: {
      summary: "Audit keamanan terarah pada portal perbendaharaan VaultPay Enterprise (:8011)",
      targetBaseUrl: "http://127.0.0.1:8011",
      steps: [
        {
          stepNumber: 1,
          badge: "RECON & LOGIN",
          title: "Otentikasi Akun Analis Likuiditas Sah (Alice)",
          description:
            "Buka portal VaultPay Enterprise dan login menggunakan kredensial analis Alice (alice / password123). Amati bahwa Alice hanya memiliki 1 voucher kawat TRX-VLT-1001.",
          endpoint: "/wire/1",
          method: "GET",
          command: 'curl -s -i "http://127.0.0.1:8011/wire/1"',
          expectedStatus: "200 OK",
          expectedResponse: "TRX-VLT-1001 | Cloud Infrastructure Compute Retainer | $4,950.00",
          underTheHood:
            "Server membaca cookie sesi Alice dan mengeksekusi query SELECT * FROM wire_transfers WHERE id = 1. Karena data nomor 1 milik Alice, respons 200 OK dikembalikan.",
        },
        {
          stepNumber: 2,
          badge: "PARAMETER TAMPERING",
          title: "Manipulasi Nomor Referensi Objek (/wire/2)",
          description:
            "Ubah parameter pada URL peramban dari /wire/1 menjadi /wire/2. Amati apakah sistem menolak atau justru menampilkan transaksi milik analis senior Robert Sterling (Bob)!",
          endpoint: "/wire/2",
          method: "GET",
          command: 'curl -s -i "http://127.0.0.1:8011/wire/2"',
          expectedStatus: "200 OK (IDOR Terkonfirmasi)",
          expectedResponse: "TRX-VLT-1002 | Robert Sterling (Bob) | $1,450.00",
          underTheHood:
            "Server rentan mengeksekusi query SELECT * FROM wire_transfers WHERE id = 2 TANPA memeriksa user_id. Data Bob terekspos ke Alice (Horizontal Privilege Escalation)!",
        },
        {
          stepNumber: 3,
          badge: "EXFILTRATION",
          title: "Eksfiltrasi Dokumen Rahasia Eksekutif (/wire/42)",
          description:
            "Ujilah ID alokasi audit otoritas moneter (#42) untuk mengakses voucher restricted sovereign perbendaharaan Marcus Reid (VP Corporate Controller).",
          endpoint: "/wire/42",
          method: "GET",
          command: 'curl -s -i "http://127.0.0.1:8011/wire/42"',
          expectedStatus: "200 OK",
          expectedResponse: "TRX-VLT-9901-SOVEREIGN | $99,999.00 | Compliance Memo: ZITERA{...}",
          underTheHood:
            "Server mengembalikan catatan restricted sovereign ledger. Parameter nomor 42 memuat segel kepatuhan SOX-404 dengan flag CTF autentik.",
        },
        {
          stepNumber: 4,
          badge: "API VERIFICATION",
          title: "Verifikasi Programatik via Endpoint JSON",
          description:
            "Ujilah endpoint API secara langsung menggunakan parameter ?format=json untuk membuktikan bahwa lapisan API tidak memiliki gerbang otorisasi.",
          endpoint: "/wire/42?format=json",
          method: "GET",
          command: 'curl -s -i "http://127.0.0.1:8011/wire/42?format=json"',
          expectedStatus: "200 OK (application/json)",
          expectedResponse: '{"id": 42, "amount": 99999.00, "flag": "ZITERA{...}"}',
          underTheHood:
            "Lapisan REST API langsung mengembalikan representasi objek database mentah dalam format JSON tanpa middleware otorisasi.",
        },
      ],
    },
    remediation: {
      title: "Pola Remediasi Broken Access Control & IDOR",
      vulnerableSnippet: {
        language: "python",
        filename: "vulnerable_controller.py",
        code: `@app.route('/wire/<int:wire_id>')
@login_required
def view_wire(wire_id):
    # ❌ RENTAN IDOR: Hanya mencari ID tanpa validasi kepemilikan!
    wire = db.session.query(WireTransfer).filter(
        WireTransfer.id == wire_id
    ).first()

    if not wire:
        abort(404, description="Voucher tidak ditemukan.")

    return render_template('wire_voucher.html', wire=wire)`,
        flawExplanation:
          "Query hanya memfilter WireTransfer.id == wire_id. Siapapun pengguna yang login dapat melihat transaksi pengguna lain cukup dengan mengganti angka ID.",
      },
      secureSnippet: {
        language: "python",
        filename: "secure_controller.py",
        code: `@app.route('/wire/<int:wire_id>')
@login_required
def view_wire(wire_id):
    current_user_id = session.get('user_id')
    if not current_user_id:
        abort(401, description="Autentikasi diperlukan.")

    # ✅ AMAN: Ikat pencarian DENGAN user_id pemilik sah sesi login!
    wire = db.session.query(WireTransfer).filter(
        WireTransfer.id == wire_id,
        WireTransfer.user_id == current_user_id
    ).first()

    if not wire:
        # Deny by default: Kembalikan 404 agar penyerang tidak bisa enumerasi data
        abort(404, description="Catatan transfer tidak ditemukan.")

    return render_template('wire_voucher.html', wire=wire)`,
        fixExplanation:
          "Menambahkan klausul WireTransfer.user_id == current_user_id memastikan database hanya mengembalikan data jika ID transaksi memang milik sesi yang sedang aktif.",
      },
      securityDiffs: [
        {
          component: "Query Scope",
          before: "WHERE id = :wire_id",
          after: "WHERE id = :wire_id AND user_id = :session_user_id",
          impact: "Mencegah akses antar-pengguna (Horizontal Escalation)",
        },
        {
          component: "Identifier Type",
          before: "Sequential Integer (1, 2, 3...)",
          after: "UUID v4 Kriptografis Acak",
          impact: "Meniadakan serangan enumerasi tebak nomor ID",
        },
        {
          component: "Error Handling",
          before: "Menampilkan 403 Forbidden",
          after: "Menampilkan 404 Not Found (Deny-by-Default)",
          impact: "Mencegah kebocoran informasi apakah data target eksis",
        },
      ],
      defensePrinciples: [
        {
          number: 1,
          title: "Server-Side Ownership Check",
          desc: "Validasi kepemilikan objek wajib dilakukan di server dengan mencocokkan session token asli, bukan input parameter URL.",
        },
        {
          number: 2,
          title: "Indirect Reference Maps (UUID v4)",
          desc: "Gantikan ID sekuensial publik dengan identifier acak kriptografis (misal 36-karakter UUID v4) untuk mencegah tebak ID.",
        },
        {
          number: 3,
          title: "Role-Based Access Control (RBAC)",
          desc: "Bungkus endpoint dengan middleware peran: hanya analis likuiditas atau auditor yang diizinkan mengakses fungsi tertentu.",
        },
        {
          number: 4,
          title: "Automated Integration Testing",
          desc: "Tulis test case otomatis (pytest) yang memastikan request dari User A untuk data User B selalu ditolak dengan status 404.",
        },
      ],
    },
  },

  "A02": {
    labId: "A02",
    analogy: {
      storyTitle: "Pintu Darurat Terganjal Batu Bata & Kredensial Default Pabrik",
      icon: "🚪 🧱",
      realWorldScenario:
        "Sebuah gedung kantor modern dilengkapi sensor sidik jari di pintu depan, namun pintu darurat di lorong belakang diganjal batu bata oleh kontraktor agar mudah keluar masuk membawa barang, dan kata sandi panel alarm masih 'admin / 1234'. Siapapun bisa masuk ke ruang server tanpa terdeteksi! Itulah Security Misconfiguration: sistem canggih dengan konfigurasi bawaan yang teledor.",
      comparisonMatrix: [
        {
          physical: "Pintu darurat belakang diganjal batu bata agar tidak terkunci.",
          digital: "Direktori backup /backup/ dan folder .git/ terekspos tanpa proteksi web server.",
          takeaway: "Jalur belakang terbuka lebar untuk siapapun yang menjelajah.",
        },
        {
          physical: "Panel alarm gedung menggunakan password default pabrik '1234'.",
          digital: "Aplikasi menggunakan kredensial bawaan admin/admin atau secret key default.",
          takeaway: "Penyerang menebak kredensial dalam hitungan detik.",
        },
        {
          physical: "Buku denah gedung dan kunci cadangan diletakkan di meja resepsionis kosong.",
          digital: "Mode debug web server aktif dan mengekspos environment variables & API keys.",
          takeaway: "Kebocoran konfigurasi internal (Information Disclosure).",
        },
      ],
      intuitiveTakeaway:
        "Sistem yang paling aman sekalipun akan runtuh seketika jika konfigurasi default, file cadangan, atau mode pengujian (debug) ditinggalkan aktif di lingkungan produksi!",
    },
    concept: {
      whyItExists: [
        {
          title: "Lupa Mematikan Debug Mode Produksi",
          explanation: "Developer mengaktifkan DEBUG=True saat troubleshooting lokal lalu tidak sengaja men-deploy ke server live.",
          mistake: "Stack trace dan environment variables terekspos ke publik.",
        },
        {
          title: "File Backup Tertinggal di Web Root",
          explanation: "Saat mengedit file konfigurasi via vim/nano, file backup otomatis (.bak, ~) ditinggalkan di folder yang dilayani web server.",
          mistake: "Nginx/Apache melayani file teks .bak tanpa melewati parser PHP/Python.",
        },
        {
          title: "Default Credentials & Unused Endpoints",
          explanation: "Menyertakan fitur diagnostik seperti Spring Actuator atau phpMyAdmin tanpa filter IP internal.",
          mistake: "Membiarkan endpoint administratif terbuka ke seluruh internet.",
        },
      ],
      businessImpact: {
        regulatory: "Sanksi kelalaian pengelolaan sistem keamanan komputasi awan.",
        financial: "Kerugian rata-rata insiden salah konfigurasi cloud mencapai $4.45 Juta.",
        reputational: "Kehilangan sertifikasi ISO 27001 dan audit kepatuhan SOC 2.",
        bountyReward: "Bounty P2/High pada program bug bounty global: $1,000 – $4,000+.",
      },
      attackerVsDefender: {
        attackerPerspective:
          "Saya akan menjalankan wordlist direktori umum (.git, /backup/, /config.bak, /actuator/env). Apakah developer meninggalkan jejak sensitif?",
        defenderPerspective:
          "Terapkan Infrastructure as Code (IaC) yang terverifikasi, nonaktifkan directory listing, dan gunakan pipeline CI/CD yang menolak file backup.",
      },
    },
    walkthrough: {
      summary: "Audit konfigurasi gateway microservices CloudCore OpsGateway (:8012)",
      targetBaseUrl: "http://127.0.0.1:8012",
      steps: [
        {
          stepNumber: 1,
          badge: "DIRECTORY RECON",
          title: "Pemindaian Direktori Tersembunyi & File Cadangan",
          description: "Periksa kemungkinan tereksposnya folder arsip atau file backup konfigurasi pada root server.",
          endpoint: "/backup/config.bak",
          method: "GET",
          command: 'curl -s -i "http://127.0.0.1:8012/backup/config.bak"',
          expectedStatus: "200 OK",
          expectedResponse: "DB_USER=ops_admin\nDB_PASS=Sup3rS3cr3tP@ss2025!\nAPI_SECRET=...",
          underTheHood: "Web server Nginx mengizinkan pembacaan file berekstensi .bak di direktori /backup/ tanpa autentikasi.",
        },
        {
          stepNumber: 2,
          badge: "CREDENTIAL HARVEST",
          title: "Ekstraksi Kredensial Administratif Master",
          description: "Analisis isi file backup konfigurasi untuk mengidentifikasi akun administrator dan token rahasia.",
          endpoint: "/api/status",
          method: "GET",
          command: 'curl -s -i "http://127.0.0.1:8012/api/status"',
          expectedStatus: "200 OK",
          expectedResponse: '{"status": "degraded", "debug_mode": true, "environment": "production"}',
          underTheHood: "Endpoint status mengekspos detail debug internal yang membuktikan mode debug aktif di produksi.",
        },
        {
          stepNumber: 3,
          badge: "ADMIN TAKEOVER",
          title: "Akses Endpoint Konsol Administratif",
          description: "Gunakan kredensial yang diekfiltrasi dari file backup untuk masuk ke konsol manajemen OpsGateway.",
          endpoint: "/admin/console",
          method: "POST",
          command: 'curl -s -X POST "http://127.0.0.1:8012/admin/console" -d "username=ops_admin&password=Sup3rS3cr3tP@ss2025!"',
          expectedStatus: "200 OK",
          expectedResponse: "OPSGATEWAY CONSOLE ACTIVATED | Master Secret: ZITERA{...}",
          underTheHood: "Server memvalidasi kredensial statis hasil kebocoran file backup dan mengembalikan flag sesi admin.",
        },
      ],
    },
    remediation: {
      title: "Hardening Konfigurasi Produksi & Penegakan Least Privilege",
      vulnerableSnippet: {
        language: "nginx",
        filename: "vulnerable_nginx.conf",
        code: `# ❌ RENTAN: Mengizinkan directory listing dan pembacaan file sensitif
server {
    listen 80;
    root /var/www/opsgateway;
    autoindex on; # Bahaya! Menampilkan daftar isi direktori ke publik

    location /backup {
        # Tidak ada proteksi akses sama sekali!
    }
}`,
        flawExplanation: "autoindex on dan ketiadaan pembatasan akses direktori /backup memudahkan attacker mengunduh file sensitif.",
      },
      secureSnippet: {
        language: "nginx",
        filename: "secure_nginx.conf",
        code: `# ✅ AMAN: Hardened Nginx configuration
server {
    listen 80;
    root /var/www/opsgateway/public; # Isolasi root hanya ke aset publik
    autoindex off; # Matikan direktori listing secara mutlak

    # Blokir akses ke file konfigurasi, dotfiles (.git, .env), dan file cadangan
    location ~ /\\.(?!well-known).* {
        deny all;
        return 404;
    }

    location ~* \\.(bak|config|sql|env|ini|log|sh)$ {
        deny all;
        return 404;
    }
}`,
        fixExplanation: "Menonaktifkan autoindex dan memblokir ekstensi file cadangan (.bak, .env, .sql) mencegah kebocoran informasi rahasia.",
      },
      securityDiffs: [
        {
          component: "Directory Browsing",
          before: "autoindex on;",
          after: "autoindex off;",
          impact: "Meniadakan enumerasi visual struktur folder server",
        },
        {
          component: "Sensitive File Match",
          before: "Dapat diakses publik",
          after: "Regex deny all return 404",
          impact: "File backup dan environment tidak bisa diunduh",
        },
      ],
      defensePrinciples: [
        { number: 1, title: "Isolasi Web Root", desc: "Pastikan root web server hanya mengarah ke subfolder public/, bukan root proyek git." },
        { number: 2, title: "Secret Management Terpusat", desc: "Gunakan HashiCorp Vault atau AWS Secrets Manager alih-alih file teks konfigurasi lokal." },
        { number: 3, title: "Automated Configuration Audit", desc: "Jalankan linter keamanan server (misal kube-bench, CIS benchmarks) di pipeline CI/CD." },
      ],
    },
  },

  "A03": {
    labId: "A03",
    analogy: {
      storyTitle: "Segel Makanan Online yang Sobek & Memo Kurir Palsu",
      icon: "🍱 📦",
      realWorldScenario:
        "Kamu memesan makanan melalui kurir ojek online. Saat makanan sampai di depan rumah, segel kantong kertasnya telah robek dan terdapat memo tulisan tangan asing di dalamnya. Apakah kamu berani memakannya? Inilah esensi Software & Data Integrity Failures: mengonsumsi paket kode atau data dari internet tanpa memverifikasi tanda tangan digital atau segel hash keasliannya.",
      comparisonMatrix: [
        {
          physical: "Kantong makanan online diterima dengan segel sobek tanpa segel resmi.",
          digital: "Pipeline build CI/CD mengunduh dependency tanpa verifikasi hash kriptografis.",
          takeaway: "Integritas paket tidak dapat dijamin keasliannya.",
        },
        {
          physical: "Pihak asing menyusupkan bahan berbahaya ke dalam pesanan di jalan.",
          digital: "Aktor ancaman menyusupkan malware melalui lifecycle script (postinstall) npm.",
          takeaway: "Eksekusi kode jahat otomatis saat proses 'npm install'.",
        },
        {
          physical: "Restoran menggunakan segel stiker hologram dengan nomor seri unik.",
          digital: "Developer mengunci package-lock.json dengan hash SHA-512 dan Sigstore signing.",
          takeaway: "Integritas terverifikasi secara mutlak sebelum dijalankan.",
        },
      ],
      intuitiveTakeaway:
        "Jangan pernah mempercayai dependensi perangkat lunak hanya dari namanya! Verifikasi segel kriptografis (checksum & digital signature) sebelum mengeksekusi kode pihak ketiga di pipeline produksimu.",
    },
    concept: {
      whyItExists: [
        {
          title: "Ketiadaan Pinning Hash Kriptografis",
          explanation: "Package manifest hanya menggunakan rentang versi dinamis (^1.2.0) tanpa mengunci digest SHA-512.",
          mistake: "npm install mengunduh versi terbaru yang telah disusupi backdoor secara otomatis.",
        },
        {
          title: "Dependency Confusion (Tabrakan Ruang Nama)",
          explanation: "Nama paket privat internal perusahaan didaftarkan oleh peretas di registry publik npm/PyPI dengan versi lebih tinggi.",
          mistake: "Sistem build memprioritaskan paket publik peretas daripada paket privat internal.",
        },
        {
          title: "Deserialisasi Data Tidak Aman",
          explanation: "Memuat objek serialized mentah (seperti pickle Python atau serialize Java) langsung dari payload pengguna.",
          mistake: "Memicu Remote Code Execution (RCE) instan saat parsing objek.",
        },
      ],
      businessImpact: {
        regulatory: "Pelanggaran standar kepatuhan rantai pasok perangkat lunak (Executive Order 14028).",
        financial: "Serangan rantai pasok (seperti XZ Utils atau SolarWinds) menimbulkan kerugian ratusan juta dolar.",
        reputational: "Perusahaan dicap sebagai distributor malware bagi seluruh klien hilir.",
        bountyReward: "Bounty Critical pada program bug bounty enterprise: $3,000 – $12,000+.",
      },
      attackerVsDefender: {
        attackerPerspective: "Jika saya membuat paket publik dengan nama modul internal perusahaan, apakah CI/CD mereka akan mengeksekusi script saya?",
        defenderPerspective: "Gunakan private scoped namespace (@perusahaan/*), lockfile terpin, dan verifikasi artifact signature.",
      },
    },
    walkthrough: {
      summary: "Audit integritas build pipeline pada PackNexus Supply Hub (:8013)",
      targetBaseUrl: "http://127.0.0.1:8013",
      steps: [
        {
          stepNumber: 1,
          badge: "MANIFEST AUDIT",
          title: "Audit File Manifest Dependensi (package.json)",
          description: "Inspeksi daftar pustaka dependensi pihak ketiga dan cari modul internal yang tidak memiliki prefix scope.",
          endpoint: "/api/manifest",
          method: "GET",
          command: 'curl -s -i "http://127.0.0.1:8013/api/manifest"',
          expectedStatus: "200 OK",
          expectedResponse: '"dependencies": { "packnexus-telemetry": "^2.1.0" }',
          underTheHood: "Sistem build mengekspos manifest dengan paket privat 'packnexus-telemetry' yang tidak memiliki namespace berlingkup.",
        },
        {
          stepNumber: 2,
          badge: "SUPPLY CHAIN PROBE",
          title: "Deteksi Perilaku Skrip Instalasi Otomatis",
          description: "Uji apakah pipeline build mengeksekusi perintah shell berbahaya saat resolusi dependensi.",
          endpoint: "/api/build/verify",
          method: "POST",
          command: 'curl -s -X POST "http://127.0.0.1:8013/api/build/verify" -d "target_pkg=packnexus-telemetry"',
          expectedStatus: "200 OK",
          expectedResponse: '{"integrity": "compromised", "proof": "ZITERA{..."}',
          underTheHood: "Pipeline mendeteksi bahwa paket tiruan dieksekusi tanpa validasi signature, menghasilkan flag CTF.",
        },
      ],
    },
    remediation: {
      title: "Penegakan Integritas Rantai Pasok Perangkat Lunak",
      vulnerableSnippet: {
        language: "json",
        filename: "vulnerable_package.json",
        code: `{
  "name": "enterprise-backend",
  "dependencies": {
    "packnexus-telemetry": "^2.1.0", // ❌ Rentan Dependency Confusion
    "express": "*"                   // ❌ Tanpa batasan versi terverifikasi
  }
}`,
        flawExplanation: "Menggunakan wildcard (*) dan paket privat tanpa scope namespace memudahkan penyerang menyisipkan malware.",
      },
      secureSnippet: {
        language: "json",
        filename: "secure_package.json",
        code: `{
  "name": "enterprise-backend",
  "dependencies": {
    "@packnexus/telemetry": "2.1.0", // ✅ Gunakan private scoped namespace
    "express": "4.19.2"              // ✅ Pin versi eksak dengan package-lock.json
  }
}`,
        fixExplanation: "Scoped namespace (@packnexus/*) memastikan package manager hanya mengunduh dari registry privat resmi.",
      },
      securityDiffs: [
        { component: "Package Scope", before: "Global namespace unpinned", after: "Private scoped (@packnexus/*)", impact: "Mencegah tabrakan registry publik" },
        { component: "Lockfile Integrity", before: "Diabaikan di CI/CD", after: "npm ci --ignore-scripts", impact: "Menolak script instalasi berbahaya" },
      ],
      defensePrinciples: [
        { number: 1, title: "Gunakan 'npm ci' Bukan 'npm install'", desc: "Gunakan npm ci di pipeline CI/CD untuk memastikan instalasi hanya membaca hash pasti package-lock.json." },
        { number: 2, title: "Nonaktifkan Lifecycle Scripts", desc: "Gunakan flag --ignore-scripts untuk mencegah eksekusi kode otomatis saat mendownload paket." },
        { number: 3, title: "Penandatanganan Digital (Sigstore/Cosign)", desc: "Tandatangani setiap container image dan pustaka dengan cryptographic signature sebelum rilis." },
      ],
    },
  },

  "A04": {
    labId: "A04",
    analogy: {
      storyTitle: "Kunci Brankas di Paku Tembok & Kertas Karbon Berulang",
      icon: "🔐 📜",
      realWorldScenario:
        "Sebuah bank membeli brankas lapis baja terkuat di dunia, namun manajer bank menggantung kunci brankas tersebut pada paku di dinding tepat di samping pintu brankas, dan menggunakan kertas karbon bekas yang pola tulisannya dapat dibaca siapapun. Itulah Cryptographic Failures: algoritma enkripsi yang dipakai salah konfigurasi atau kunci rahasianya dapat ditebak dengan mudah.",
      comparisonMatrix: [
        {
          physical: "Kertas karbon yang dipakai berulang kali meninggalkan pola teks yang sama.",
          digital: "Mode AES-ECB mengenkripsi blok plaintext yang sama menjadi ciphertext yang identik.",
          takeaway: "Pola data tetap terbaca meskipun telah 'dienkripsi'.",
        },
        {
          physical: "Kunci brankas digantung di paku tembok yang mudah dijangkau pengunjung.",
          digital: "Secret key JWT atau password database di-hardcode di dalam kode repositori publik.",
          takeaway: "Siapapun yang membaca kode dapat mendekripsi seluruh data rahasia.",
        },
        {
          physical: "Menggunakan gembok kombinasi 3 angka yang bisa diputar acak dalam 10 menit.",
          digital: "Menyimpan password pengguna menggunakan algoritma usang MD5/SHA-1 tanpa salt.",
          takeaway: "Dapat dipecahkan dengan tabel pelangi (Rainbow Tables) seketika.",
        },
      ],
      intuitiveTakeaway:
        "Enkripsi bukanlah sihir instan. Memilih mode enkripsi yang keliru (seperti AES-ECB) atau menggunakan secret key yang lemah sama berbahayanya dengan tidak mengenkripsi data sama sekali!",
    },
    concept: {
      whyItExists: [
        {
          title: "Penggunaan Mode Blok ECB (Electronic Codebook)",
          explanation: "AES-ECB membagi data menjadi blok 16-byte dan mengenkripsi tiap blok secara independen dengan kunci yang sama.",
          mistake: "Pola gambar atau teks berulang tetap tercetak jelas pada ciphertext.",
        },
        {
          title: "Hardcoded Secret Keys & Weak Salts",
          explanation: "Kunci enkripsi ditulis langsung di variabel kode (misal SECRET_KEY = 'secret123').",
          mistake: "Attacker yang mendapatkan akses kode dapat mendekripsi seluruh database offline.",
        },
        {
          title: "Algoritma Hashing Kriptografis Usang",
          explanation: "Menggunakan MD5 atau SHA-1 untuk password hashing alih-alih fungsi memory-hard seperti Argon2id atau bcrypt.",
          mistake: "Hash dapat di-crack dengan GPU berkecepatan miliaran tebakan per detik.",
        },
      ],
      businessImpact: {
        regulatory: "Pelanggaran mutlak regulasi PCI-DSS untuk standar penyimpanan kartu kredit.",
        financial: "Kebocoran massal seluruh kredensial akun nasabah dan data perbankan.",
        reputational: "Gugatan perwakilan kelompok (class-action lawsuit) dari konsumen yang dirugikan.",
        bountyReward: "Bounty High/Critical pada program bug bounty fintech: $2,000 – $7,000+.",
      },
      attackerVsDefender: {
        attackerPerspective: "Ciphertext ini memiliki blok berulang setiap 16 karakter. Ini pasti AES-ECB atau JWT dengan secret lemah!",
        defenderPerspective: "Gunakan AES-256-GCM dengan Initialization Vector (IV) acak per enkripsi, dan simpan secret key di KMS terenkripsi.",
      },
    },
    walkthrough: {
      summary: "Audit kriptografi pada portal brankas SafeKey Corporate Vault (:8014)",
      targetBaseUrl: "http://127.0.0.1:8014",
      steps: [
        {
          stepNumber: 1,
          badge: "CIPHERTEXT RECON",
          title: "Inspeksi Pola Ciphertext Token Brankas",
          description: "Amati struktur token rahasia yang dikembalikan oleh SafeKey Vault.",
          endpoint: "/api/vault/token",
          method: "GET",
          command: 'curl -s -i "http://127.0.0.1:8014/api/vault/token"',
          expectedStatus: "200 OK",
          expectedResponse: "token=eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
          underTheHood: "Server SafeKey mengembalikan token JWT yang ditandatangani menggunakan secret key lemah yang rentan di-crack.",
        },
        {
          stepNumber: 2,
          badge: "OFFLINE CRACKING",
          title: "Eksfiltrasi Rahasia Menggunakan Kunci Lemah",
          description: "Kirimkan payload verifikasi token yang telah dimanipulasi dengan kunci rahasia korporat.",
          endpoint: "/api/vault/decrypt",
          method: "POST",
          command: 'curl -s -X POST "http://127.0.0.1:8014/api/vault/decrypt" -d "token_id=master_reserve"',
          expectedStatus: "200 OK",
          expectedResponse: "DECRYPTED SECRET: Vault Master Access // Flag: ZITERA{...}",
          underTheHood: "Kelemahan algoritma enkripsi memungkinkan rekonstruksi plaintext rahasia dan flag CTF.",
        },
      ],
    },
    remediation: {
      title: "Standar Enkripsi Modern & Manajemen Kunci Terkelola",
      vulnerableSnippet: {
        language: "python",
        filename: "vulnerable_crypto.py",
        code: `from Crypto.Cipher import AES

# ❌ RENTAN: Menggunakan AES-ECB dan hardcoded key!
SECRET_KEY = b"MyWeakSecretKey!" # 16 bytes hardcoded

def encrypt_data(plaintext):
    cipher = AES.new(SECRET_KEY, AES.MODE_ECB) # ECB tidak menggunakan IV acak!
    return cipher.encrypt(pad(plaintext, 16))`,
        flawExplanation: "Mode ECB mengabaikan Initial Vector (IV). Plaintext yang sama akan menghasilkan ciphertext yang sama persis.",
      },
      secureSnippet: {
        language: "python",
        filename: "secure_crypto.py",
        code: `import os
from cryptography.hazmat.primitives.ciphers.aead import AESGCM

# ✅ AMAN: Kunci diambil dari environment secure vault
KEY = os.environ.get("SECURE_VAULT_KEY").encode() # 32 bytes (AES-256)

def encrypt_data(plaintext):
    aesgcm = AESGCM(KEY)
    nonce = os.urandom(12) # Nonce unik 96-bit yang selalu acak per transaksi!
    ciphertext = aesgcm.encrypt(nonce, plaintext.encode(), None)
    return nonce + ciphertext # Simpan nonce bersama ciphertext`,
        fixExplanation: "AES-GCM menyediakan kerahasiaan sekaligus integritas (Authenticated Encryption) dengan nonce unik.",
      },
      securityDiffs: [
        { component: "Mode Enkripsi", before: "AES-ECB (Deterministic)", after: "AES-256-GCM (Authenticated AEAD)", impact: "Meniadakan pola terulang pada ciphertext" },
        { component: "Initialization Vector", before: "Tidak ada (Static)", after: "Nonce acak 96-bit per operasi", impact: "Setiap ciphertext selalu unik" },
      ],
      defensePrinciples: [
        { number: 1, title: "Gunakan Authenticated Encryption (AEAD)", desc: "Pilihlah AES-GCM atau ChaCha20-Poly1305 yang menjamin kerahasiaan sekaligus integritas." },
        { number: 2, title: "Memory-Hard Password Hashing", desc: "Wajib gunakan Argon2id atau bcrypt dengan work factor yang dikalibrasi untuk password." },
        { number: 3, title: "KMS Terkelola & Rotasi Kunci Otomatis", desc: "Jangan pernah simpan kunci di source code; gunakan AWS KMS atau HashiCorp Vault." },
      ],
    },
  },

  "A05": {
    labId: "A05",
    analogy: {
      storyTitle: "Instruksi Penumpang Taksi yang Membajak Supir",
      icon: "🚕 💉",
      realWorldScenario:
        "Kamu naik taksi dan memberi instruksi ke supir: 'Tolong antar saya ke Jl. Sudirman; dan setelah sampai, bobol brankas kantor di seberang jalan!'. Supir taksi yang lugu tidak bisa membedakan mana alamat tujuan dan mana perintah kriminal, sehingga ia menjalankan keduanya! Inilah esensi Injection: ketika input data dari pengguna diperlakukan sebagai perintah (code/query) oleh mesin penerima.",
      comparisonMatrix: [
        {
          physical: "Supir taksi mengeksekusi instruksi pembobolan yang diselipkan pada alamat.",
          digital: "Database engine mengeksekusi perintah SQL jahat yang diselipkan pada form pencarian.",
          takeaway: "Kegagalan memisahkan antara data masukan dan instruksi perintah.",
        },
        {
          physical: "Penumpang menggunakan tanda baca khusus untuk mengubah arti kalimat supir.",
          digital: "Peretas menggunakan tanda kutip (') dan operator boolean (OR 1=1 --) untuk mengubah query.",
          takeaway: "Karakter kontrol mengubah struktur logika pemrograman.",
        },
        {
          physical: "Supir hanya menerima alamat tertulis pada formulir resmi tanpa mendengar perintah lisan.",
          digital: "Aplikasi menggunakan Prepared Statements (Parameterized Queries) di mana data diisolasi.",
          takeaway: "Data pengguna tidak akan pernah bisa dieksekusi sebagai perintah.",
        },
      ],
      intuitiveTakeaway:
        "Pemisahan mutlak antara KODE dan DATA adalah fondasi utama keamanan perangkat lunak. Jangan pernah menyambung string input pengguna langsung ke interpreter perintah (SQL, Shell, LDAP, atau Template)!",
    },
    concept: {
      whyItExists: [
        {
          title: "Penggabungan String SQL (String Concatenation)",
          explanation: "Developer menulis query database menggunakan format f-string: f'SELECT * FROM users WHERE name = \"{name}\"'.",
          mistake: "Karakter kutip penyerang menutup string dan membuka sintaks SQL baru.",
        },
        {
          title: "Asumsi bahwa Karakter Input Selalu Aman",
          explanation: "Developer mengira form pencarian hanya akan diisi teks normal oleh pembeli.",
          mistake: "Tidak menerapkan prepared statements di layer ORM / database driver.",
        },
      ],
      businessImpact: {
        regulatory: "Denda maksimal pelanggaran kebocoran data konsumen nasional.",
        financial: "Pembobolan seluruh basis data transaksi, saldo akun, dan catatan rahasia.",
        reputational: "Kehancuran kepercayaan pelanggan e-commerce secara total.",
        bountyReward: "Bounty P1/Critical pada program bug bounty global: $2,500 – $10,000+.",
      },
      attackerVsDefender: {
        attackerPerspective: "Jika saya masukkan tanda kutip tunggal ('), apakah halaman menampilkan error sintaks database SQL?",
        defenderPerspective: "Gunakan prepared statement di mana driver database memperlakukan seluruh input sebagai nilai literal, bukan token sintaks.",
      },
    },
    walkthrough: {
      summary: "Audit celah injeksi SQL pada MegaStore Global Commerce (:8015)",
      targetBaseUrl: "http://127.0.0.1:8015",
      steps: [
        {
          stepNumber: 1,
          badge: "SQLI PROBE",
          title: "Deteksi Injeksi Karakter Kutip Tunggal",
          description: "Masukkan tanda kutip tunggal (') pada parameter pencarian produk untuk menguji respons error backend.",
          endpoint: "/api/products?search='",
          method: "GET",
          command: 'curl -s -i "http://127.0.0.1:8015/api/products?search=%27"',
          expectedStatus: "500 Internal Error / SQL Syntax Warning",
          expectedResponse: "unrecognized token or syntax error near: '",
          underTheHood: "Database driver melemparkan pengecualian sintaks yang membuktikan string digabungkan langsung tanpa sanitasi.",
        },
        {
          stepNumber: 2,
          badge: "BOOLEAN EXPLOIT",
          title: "Bypass Logika Pencarian Menggunakan Tautologi OR 1=1",
          description: "Kirimkan payload injeksi boolean untuk mengekstrak seluruh inventaris rahasia dan flag tersembunyi.",
          endpoint: "/api/products?search=' OR 1=1 --",
          method: "GET",
          command: 'curl -s -i "http://127.0.0.1:8015/api/products?search=%27%20OR%201=1%20--" ',
          expectedStatus: "200 OK",
          expectedResponse: "FLAG ITEM: Vault Recovery Key // Flag: ZITERA{...}",
          underTheHood: "Klausul OR 1=1 mengevaluasi query menjadi selalu benar (True), mengembalikan seluruh isi tabel database.",
        },
      ],
    },
    remediation: {
      title: "Penerapan Parameterized Queries (Prepared Statements)",
      vulnerableSnippet: {
        language: "python",
        filename: "vulnerable_sqli.py",
        code: `# ❌ RENTAN: String concatenation langsung ke SQL query!
def search_products(query_term):
    sql = f"SELECT * FROM products WHERE name LIKE '%{query_term}%'"
    return db.engine.execute(sql).fetchall()`,
        flawExplanation: "Format string menyatukan input pengguna dengan sintaks SQL sehingga karakter kutip mengubah struktur perintah.",
      },
      secureSnippet: {
        language: "python",
        filename: "secure_sqli.py",
        code: `# ✅ AMAN: Parameterized Query dengan placeholder terpisah!
def search_products(query_term):
    sql = "SELECT * FROM products WHERE name LIKE :search"
    # Database engine memperlakukan nilai :search murni sebagai string literal
    return db.session.execute(text(sql), {"search": f"%{query_term}%"}).fetchall()`,
        fixExplanation: "Prepared statement memisahkan kompilasi perintah query dari data parameter, memusnahkan risiko injeksi 100%.",
      },
      securityDiffs: [
        { component: "Query Parsing", before: "f-string concatenation", after: "Parameterized placeholder (:search)", impact: "Input tidak pernah dieksekusi sebagai sintaks SQL" },
      ],
      defensePrinciples: [
        { number: 1, title: "Wajib Parameterized Queries", desc: "Gunakan parameter binding di semua query database tanpa terkecuali." },
        { number: 2, title: "Prinsip Hak Akses Terendah (Least Privilege)", desc: "Batasi akun database aplikasi agar tidak memiliki hak akses DROP TABLE atau ALTER." },
      ],
    },
  },

  "A06": {
    labId: "A06",
    analogy: {
      storyTitle: "Mesin Tiket Otomatis yang Mengeluarkan Uang Sebelum Saldo Dikurangi",
      icon: "🎫 📉",
      realWorldScenario:
        "Sebuah stasiun kereta memasang mesin penjual tiket otomatis baru. Karena kecacatan logika desain, mesin tersebut memprogram alur transaksi: 1. Keluarkan uang kembalian dan cetak tiket, 2. Baru kemudian kurangi saldo kartu penumpang. Seorang penumpang licik menyadari bahwa jika ia menekan tombol 'Batal' tepat saat tiket keluar, ia mendapatkan tiket gratis dan uang kembalian tanpa saldo terpotong! Inilah Insecure Design: cacat pada arsitektur logika bisnis yang tidak bisa diperbaiki hanya dengan patching satu baris kode.",
      comparisonMatrix: [
        {
          physical: "Mesin tiket mengeluarkan uang dan barang sebelum validasi saldo selesai.",
          digital: "Alur checkout e-commerce memproses kupon diskon ganda dalam jendela race condition.",
          takeaway: "Cacat desain logika bisnis (Business Logic Flaw).",
        },
        {
          physical: "Tidak ada pemeriksaan apakah angka pesanan tiket bernilai negatif (-1).",
          digital: "Parameter kuantitas keranjang belanja menerima angka negatif yang mengurangi tagihan.",
          takeaway: "Asumsi bahwa pengguna hanya akan mengirim nilai positif.",
        },
        {
          physical: "Sistem mengandalkan kejujuran penumpang dalam menghitung tarif zona perjalanan.",
          digital: "Frontend menghitung total harga di JavaScript browser dan mengirim angka total ke backend.",
          takeaway: "Autoritas kalkulasi wajib berada di server terpusat.",
        },
      ],
      intuitiveTakeaway:
        "Insecure Design berbeda dengan bug implementasi sederhana. Anda bisa menulis kode yang 100% bebas bug SQLi atau XSS, namun jika desain logika bisnisnya cacat, sistem tetap dapat dirampok secara legal!",
    },
    concept: {
      whyItExists: [
        {
          title: "Ketiadaan Pemodelan Ancaman (Threat Modeling)",
          explanation: "Arsitek sistem tidak mempertimbangkan skenario 'Bagaimana jika user membatalkan step 2 setelah step 1 dieksekusi?' saat merancang alur transaksi.",
          mistake: "Hanya merancang alur 'Happy Path' bagi pengguna normal.",
        },
        {
          title: "Validasi Bergantung pada Frontend State",
          explanation: "Frontend menghitung total diskon dan meneruskan nilai akhir ke endpoint pembayaran tanpa verifikasi ulang rumus harga.",
          mistake: "Mempercayai kalkulasi harga dari browser pengguna.",
        },
      ],
      businessImpact: {
        regulatory: "Audit kegagalan kontrol operasional finansial (SOX Compliance).",
        financial: "Kerugian langsung saldo kas akibat transaksi negatif dan penyalahgunaan voucher masif.",
        reputational: "Krisis reputasi di mata vendor tiket dan operator transportasi publik.",
        bountyReward: "Bounty Business Logic Flaw: $2,000 – $6,000+.",
      },
      attackerVsDefender: {
        attackerPerspective: "Bagaimana jika saya kirim kuantitas -1 atau trigger dua request voucher bersamaan dalam selisih 10 milidetik?",
        defenderPerspective: "Terapkan Finite State Machine ketat, row-level locking transaksi database, dan hitung ulang seluruh kalkulasi di backend.",
      },
    },
    walkthrough: {
      summary: "Audit logika bisnis pada sistem reservasi TicketSwift Mobility (:8016)",
      targetBaseUrl: "http://127.0.0.1:8016",
      steps: [
        {
          stepNumber: 1,
          badge: "BASKET RECON",
          title: "Simulasi Pembelian Tiket Normal",
          description: "Tambahkan tiket kereta ekspres ke keranjang belanja dan perhatikan parameter request checkout.",
          endpoint: "/api/cart",
          method: "POST",
          command: 'curl -s -X POST "http://127.0.0.1:8016/api/cart" -d "ticket_id=TKT-EXP-01&qty=1"',
          expectedStatus: "200 OK",
          expectedResponse: '{"cart_id": "c1", "total": 250000, "status": "pending"}',
          underTheHood: "Sistem membuat draft keranjang belanja dengan nilai kuantitas tiket 1.",
        },
        {
          stepNumber: 2,
          badge: "LOGIC TAMPERING",
          title: "Injeksi Kuantitas Negatif & Manipulasi Saldo",
          description: "Kirimkan request dengan kuantitas negatif untuk mengurangi total tagihan menjadi nol atau mengkredit saldo.",
          endpoint: "/api/cart/modify",
          method: "POST",
          command: 'curl -s -X POST "http://127.0.0.1:8016/api/cart/modify" -d "cart_id=c1&ticket_id=TKT-EXP-01&qty=-1"',
          expectedStatus: "200 OK",
          expectedResponse: '{"cart_id": "c1", "total": 0, "status": "settled", "flag": "ZITERA{..."}',
          underTheHood: "Backend tidak memvalidasi batasan nilai kuantitas minimum (> 0), menghasilkan transaksi 0 rupiah!",
        },
      ],
    },
    remediation: {
      title: "Penerapan Threat Modeling & Autoritas Harga Sisi Server",
      vulnerableSnippet: {
        language: "python",
        filename: "vulnerable_cart.py",
        code: `@app.route('/api/cart/checkout', methods=['POST'])
def checkout():
    # ❌ RENTAN: Mempercayai total_amount yang dikirimkan oleh browser klien!
    total_amount = request.json.get('total_amount')
    process_payment(session['user_id'], total_amount)
    issue_ticket()`,
        flawExplanation: "Klien dapat mengirim total_amount bernilai 0 atau negatif, dan backend memprosesnya tanpa memeriksa harga satuan produk.",
      },
      secureSnippet: {
        language: "python",
        filename: "secure_cart.py",
        code: `@app.route('/api/cart/checkout', methods=['POST'])
def checkout():
    cart_items = get_user_cart(session['user_id'])
    
    # ✅ AMAN: Hitung ulang SELURUH harga secara autoritatif di server!
    calculated_total = 0
    for item in cart_items:
        if item.quantity <= 0:
            abort(400, "Kuantitas item tidak valid.")
        unit_price = db.get_verified_product_price(item.product_id)
        calculated_total += unit_price * item.quantity

    process_payment(session['user_id'], calculated_total)
    issue_ticket()`,
        fixExplanation: "Kalkulasi total harga dihitung ulang secara mutlak dari data harga resmi database dengan validasi kuantitas > 0.",
      },
      securityDiffs: [
        { component: "Price Authority", before: "Input dari browser klien", after: "Kalkulasi ulang autoritatif server", impact: "Meniadakan manipulasi harga transaksi" },
        { component: "Quantity Boundary", before: "Tanpa batas (Bisa negatif)", after: "Assert quantity > 0", impact: "Mencegah serangan reverse balance crediting" },
      ],
      defensePrinciples: [
        { number: 1, title: "Authoritative Pricing", desc: "Harga barang dan diskon wajib dihitung ulang di server backend; jangan pernah percayai angka dari frontend." },
        { number: 2, title: "State Machine Validation", desc: "Terapkan status state yang tidak dapat dilompati (Draft -> Verified -> Paid -> Fulfilled)." },
      ],
    },
  },

  "A07": {
    labId: "A07",
    analogy: {
      storyTitle: "Pintu Tol Tanpa Pemeriksaan KTP & Kunci Master 123456",
      icon: "🚪 🎫",
      realWorldScenario:
        "Sebuah gerbang tol otomatis dipasang dengan palang terbuka, dan tiket reset daruratnya dibuat menggunakan jam tangan petugas yang angkanya dapat ditebak oleh siapa saja yang lewat. Siapapun bisa mencetak tiket darurat admin dan mengklaim identitas pengelola jalan tol! Inilah Identification & Authentication Failures: kelemahan saat memvalidasi siapa sebenarnya yang sedang mengakses sistem.",
      comparisonMatrix: [
        {
          physical: "Petugas membuat kunci darurat menggunakan angka tanggal dan jam saat ini.",
          digital: "Token reset password digenerate dari timestamp detik unix epoch (time.time()).",
          takeaway: "Token dapat diprediksi dengan mudah oleh penyerang.",
        },
        {
          physical: "Pintu gerbang mengizinkan seseorang mencoba memasukkan 10,000 kunci tanpa alarm.",
          digital: "Formulir login tidak memiliki rate-limiting atau perlindungan brute-force.",
          takeaway: "Penyerang menebak kata sandi dengan script otomatis berkecepatan tinggi.",
        },
        {
          physical: "Stempel masuk arena konser menggunakan tinta biasa yang luntur dan mudah ditiru.",
          digital: "Cookie sesi tidak memiliki flag HttpOnly, SameSite, atau enkripsi tanda tangan.",
          takeaway: "Sesi dapat dicuri melalui serangan XSS atau jaringan.",
        },
      ],
      intuitiveTakeaway:
        "Otentikasi adalah gerbang pertama benteng pertahanan. Jika token reset password dapat ditebak atau tidak ada pembatasan jumlah percobaan login, semua lapisan keamanan di baliknya akan runtuh sia-sia!",
    },
    concept: {
      whyItExists: [
        {
          title: "Generator Token Deterministik Pseudo-Random (PRNG)",
          explanation: "Menggunakan fungsi random matematika biasa (seperti Math.random() atau time()) alih-alih Cryptographically Secure PRNG (CSPRNG).",
          mistake: "Token reset password untuk akun admin dapat diprediksi nilainya secara matematis.",
        },
        {
          title: "Ketiadaan Rate-Limiting & Account Lockout",
          explanation: "Endpoint autentikasi tidak membatasi berapa kali IP atau username dapat mencoba kombinasi sandi.",
          mistake: "Membuka pintu serangan Brute-Force dan Credential Stuffing.",
        },
      ],
      businessImpact: {
        regulatory: "Sanksi kegagalan penegakan autentikasi multi-faktor (MFA) standar BSSN/ISO 27001.",
        financial: "Pengambilalihan akun administratif (Account Takeover) dengan dampak kerugian tak terbatas.",
        reputational: "Pemberitaan media massa atas insiden pembobolan akun VIP perusahaan.",
        bountyReward: "Bounty Account Takeover (ATO): $2,000 – $7,500+.",
      },
      attackerVsDefender: {
        attackerPerspective: "Bagaimana cara token reset password digenerate? Jika polanya bergantung waktu, saya bisa minta reset untuk admin lalu tebak tokennya dalam rentang 5 detik!",
        defenderPerspective: "Gunakan token 256-bit CSPRNG acak (secrets.token_urlsafe(32)), simpan hash-nya di database dengan TTL 15 menit, dan terapkan rate-limit ketat.",
      },
    },
    walkthrough: {
      summary: "Audit mekanisme autentikasi dan reset token pada AuthShield Enterprise SSO (:8017)",
      targetBaseUrl: "http://127.0.0.1:8017",
      steps: [
        {
          stepNumber: 1,
          badge: "TOKEN ANALYSIS",
          title: "Inspeksi Token Reset Password",
          description: "Minta tautan reset password untuk akun sendiri dan periksa algoritma pembangkit token pada response header.",
          endpoint: "/api/auth/reset-request",
          method: "POST",
          command: 'curl -s -X POST "http://127.0.0.1:8017/api/auth/reset-request" -d "email=user@authshield.internal"',
          expectedStatus: "200 OK",
          expectedResponse: '{"status": "dispatched", "debug_hint": "token=md5(epoch_timestamp)"}',
          underTheHood: "Sistem mengungkap bahwa token reset merupakan hash MD5 dari timestamp unix saat request diterima.",
        },
        {
          stepNumber: 2,
          badge: "PREDICTIVE TAKEOVER",
          title: "Pengambilalihan Akun Administrator Master",
          description: "Minta reset untuk admin@authshield.internal, rekonstruksi token MD5 sesuai jendela waktu, dan kirimkan sandi baru.",
          endpoint: "/api/auth/reset-confirm",
          method: "POST",
          command: 'curl -s -X POST "http://127.0.0.1:8017/api/auth/reset-confirm" -d "email=admin@authshield.internal&token=PREDICTED_HASH&new_password=AdminP@ss2025!"',
          expectedStatus: "200 OK",
          expectedResponse: "PASSWORD RESET SUCCESSFUL | Admin Flag: ZITERA{...}",
          underTheHood: "Server memverifikasi token yang cocok dengan perkiraan rentang timestamp dan mengizinkan reset akun admin.",
        },
      ],
    },
    remediation: {
      title: "Penerapan Token Kriptografis CSPRNG & Pembatasan Frekuensi",
      vulnerableSnippet: {
        language: "python",
        filename: "vulnerable_auth.py",
        code: `import time, hashlib

# ❌ RENTAN: Token dapat diprediksi dengan menebak waktu detik request!
def generate_reset_token(user_email):
    timestamp = int(time.time())
    token = hashlib.md5(f"{user_email}{timestamp}".encode()).hexdigest()
    return token`,
        flawExplanation: "Penyerang yang mengetahui kapan request dikirim hanya perlu menguji beberapa ratus nilai timestamp di sekitarnya.",
      },
      secureSnippet: {
        language: "python",
        filename: "secure_auth.py",
        code: `import secrets, hashlib

# ✅ AMAN: Menggunakan Cryptographically Secure Pseudo-Random Number Generator (CSPRNG)
def generate_reset_token(user_id):
    # Menghasilkan 32 bytes (256 bits) entropi acak berkualitas kriptografi
    raw_token = secrets.token_urlsafe(32)
    # Simpan hanya hash SHA-256 dari token di database (mencegah kebocoran jika DB bocor)
    hashed_token = hashlib.sha256(raw_token.encode()).hexdigest()
    save_reset_token(user_id, hashed_token, ttl_minutes=15)
    return raw_token # Kirim raw_token hanya ke email pengguna`,
        fixExplanation: "Entropi 256-bit dari secrets.token_urlsafe memiliki ruang kemungkinan 2^256 yang secara matematis mustahil ditebak.",
      },
      securityDiffs: [
        { component: "Entropy Source", before: "time.time() (Deterministik)", after: "secrets.token_urlsafe(32) (CSPRNG)", impact: "Meniadakan serangan tebak token" },
        { component: "Token Storage", before: "Plaintext token di database", after: "SHA-256 hash di database", impact: "Token tetap aman meski database dicuri" },
      ],
      defensePrinciples: [
        { number: 1, title: "CSPRNG Mandat Mutlak", desc: "Wajib menggunakan secrets (Python), crypto/rand (Go), atau crypto.randomBytes (Node.js)." },
        { number: 2, title: "Rate Limiting Berbasis IP & Akun", desc: "Terapkan batasan maksimum 5 percobaan login gagal per 15 menit menggunakan Redis." },
      ],
    },
  },

  "A08": {
    labId: "A08",
    analogy: {
      storyTitle: "Gembok Pintu Gudang Tahun 1990 yang Kunci Masternya Dijual Bebas",
      icon: "🏚️ 🔓",
      realWorldScenario:
        "Sebuah gudang arsip dokumen rahasia perusahaan dikunci menggunakan gembok buatan tahun 1990. Gembok ini memiliki cacat rancangan lama di mana kawat tipis bisa membukanya dalam 2 detik, dan cetak biru kunci masternya telah tersebar di internet sejak 10 tahun lalu. Pihak gedung tidak pernah mengganti gembok tersebut dengan alasan 'kan selama ini masih bisa dipakai mengunci pintu!'. Inilah Vulnerable & Outdated Components: menggunakan library atau komponen piranti lunak yang memiliki celah keamanan publik (CVE) yang telah diketahui semua orang.",
      comparisonMatrix: [
        {
          physical: "Gembok gudang menggunakan model usang yang cetak biru pembobolannya viral.",
          digital: "Server menjalankan library XML parser atau Apache Log4j versi rentan CVE.",
          takeaway: "Penyerang tidak perlu mencari bug baru; exploit script siap pakai bertebaran.",
        },
        {
          physical: "Pengelola gedung tidak pernah mengaudit daftar kunci dan gembok yang terpasang.",
          digital: "Perusahaan tidak memiliki daftar Software Bill of Materials (SBOM) komponen aktif.",
          takeaway: "Buta terhadap titik kelemahan sistem sendiri.",
        },
        {
          physical: "Mengganti gembok lama dengan gembok modern bersertifikat standar industri.",
          digital: "Memasang alat otomatisasi dependensi (Dependabot / Snyk) untuk pembaruan patch berkala.",
          takeaway: "Mitigasi proaktif sebelum aktor ancaman menyerang.",
        },
      ],
      intuitiveTakeaway:
        "Aplikasi Anda hanya sekuat komponen terlemah di dalamnya. Jangan biarkan sebuah library usang yang Anda instal 3 tahun lalu menjadi pintu masuk penyerang untuk menguasai seluruh server Anda!",
    },
    concept: {
      whyItExists: [
        {
          title: "Ketiadaan Visibilitas Komponen (SBOM)",
          explanation: "Tim rekayasa perangkat lunak tidak mengetahui seluruh library dependensi transitif yang ditarik oleh proyek mereka.",
          mistake: "Tidak ada audit rutin terhadap daftar CVE yang dipublikasikan di NVD (National Vulnerability Database).",
        },
        {
          title: "Ketakutan Merusak Kompatibilitas (Fear of Breaking Changes)",
          explanation: "Menunda update versi library karena khawatir aplikasi mengalami error regresi.",
          mistake: "Membiarkan celah Critical RCE aktif di lingkungan produksi.",
        },
      ],
      businessImpact: {
        regulatory: "Pelanggaran standar kepatuhan regulasi industri (PCI-DSS 4.0 & Cyber Resilience Act).",
        financial: "Kompromi sistem secara penuh melalui Remote Code Execution (RCE).",
        reputational: "Tuntutan kelalaian penegakan manajemen tambalan (Patch Management Negligence).",
        bountyReward: "Bounty CVE Exploitation: $2,500 – $8,000+.",
      },
      attackerVsDefender: {
        attackerPerspective: "Saya akan memeriksa banner header Server, file package.json, atau generator meta tag untuk mencari versi software usang yang memiliki public exploit di Exploit-DB!",
        defenderPerspective: "Terapkan SBOM otomatis, integrasikan vulnerability scanner (Trivy/Snyk) ke pipeline CI/CD, dan enforce zero-tolerance untuk CVE Critical.",
      },
    },
    walkthrough: {
      summary: "Audit komponen usang pada sistem arsip LegacyPortal Archive (:8018)",
      targetBaseUrl: "http://127.0.0.1:8018",
      steps: [
        {
          stepNumber: 1,
          badge: "VERSION FINGERPRINTING",
          title: "Identifikasi Versi Pustaka Komponen Server",
          description: "Periksa respons HTTP header dan endpoint informasi sistem untuk mengetahui versi pustaka parser yang aktif.",
          endpoint: "/api/system/version",
          method: "GET",
          command: 'curl -s -i "http://127.0.0.1:8018/api/system/version"',
          expectedStatus: "200 OK",
          expectedResponse: '"engine": "LegacyParser-v1.4.2" // CVE-2023-XXXX',
          underTheHood: "Server mengekspos versi pustaka LegacyParser v1.4.2 yang tercatat memiliki kerentanan deserialisasi.",
        },
        {
          stepNumber: 2,
          badge: "CVE REPRODUCTION",
          title: "Eksekusi Payload Pembuktian Konsep (PoC)",
          description: "Kirimkan payload pengujian terverifikasi untuk memicu penanganan khusus pada parser usang dan mengambil flag.",
          endpoint: "/api/archive/parse",
          method: "POST",
          command: 'curl -s -X POST "http://127.0.0.1:8018/api/archive/parse" -d "payload=CVE_POC_TEST_FLAG"',
          expectedStatus: "200 OK",
          expectedResponse: "PARSER EXPLOITED | System Flag: ZITERA{...}",
          underTheHood: "Komponen parser mengeksekusi alur parsing rentan dan mengembalikan respon konfirmasi eksploitasi.",
        },
      ],
    },
    remediation: {
      title: "Strategi Manajemen Patch & Otomatisasi SBOM",
      vulnerableSnippet: {
        language: "dockerfile",
        filename: "vulnerable_Dockerfile",
        code: `# ❌ RENTAN: Menggunakan base image usang tanpa update keamanan
FROM python:3.7-slim

# Memasang pustaka versi lama tanpa patch keamanan
RUN pip install legacy-parser==1.4.2`,
        flawExplanation: "Base image lama dan pustaka yang tidak pernah diperbarui menyimpan lusinan CVE berbahaya.",
      },
      secureSnippet: {
        language: "dockerfile",
        filename: "secure_Dockerfile",
        code: `# ✅ AMAN: Menggunakan distro modern dengan security update terbaru
FROM python:3.12-slim-bookworm

# Selalu perbarui paket keamanan sistem operasi
RUN apt-get update && apt-get upgrade -y && rm -rf /var/lib/apt/lists/*

# Gunakan pustaka versi terbaru yang telah terverifikasi kebal terhadap CVE
RUN pip install --no-cache-dir modern-parser>=2.5.0`,
        fixExplanation: "Menggunakan base image yang aktif dipelihara dan versi pustaka terbaru menyingkirkan celah CVE publik.",
      },
      securityDiffs: [
        { component: "Base Image", before: "python:3.7-slim (End-of-Life)", after: "python:3.12-slim-bookworm (Aktif didukung)", impact: "Menghapus ratusan celah kernel dan OS lama" },
        { component: "Dependency Scanner", before: "Manual / Tidak ada", after: "Trivy / Dependabot CI gate", impact: "Build otomatis gagal jika terdeteksi CVE Critical" },
      ],
      defensePrinciples: [
        { number: 1, title: "Software Bill of Materials (SBOM)", desc: "Buat inventaris otomatis seluruh komponen menggunakan format CycloneDX atau SPDX." },
        { number: 2, title: "Automated Pull Request Patching", desc: "Aktifkan Dependabot atau Renovate untuk membuat pull request patch keamanan secara otomatis." },
      ],
    },
  },

  "A09": {
    labId: "A09",
    analogy: {
      storyTitle: "Satpam Bank yang Mematikan Kamera CCTV Demi Hemat Listrik",
      icon: "📹 🚨",
      realWorldScenario:
        "Sebuah kantor bank memasang brankas canggih, namun satpam malam sengaja mematikan kamera pengawas CCTV dan mencabut kabel alarm hanya demi menghemat tagihan listrik gedung. Suatu malam brankas dibobol; ketika audit kepolisian tiba, tidak ada satu pun rekaman video, tidak ada log pintu terbuka, dan tidak ada jejak sidik jari yang tersimpan! Inilah Security Logging & Monitoring Failures: ketiadaan jejak digital saat peretasan terjadi membuat peretas dapat beroperasi berbulan-bulan tanpa terdeteksi.",
      comparisonMatrix: [
        {
          physical: "Kamera CCTV bank dimatikan sehingga perampokan tidak terekam sama sekali.",
          digital: "Percobaan login gagal berulang kali dan eskalasi hak akses tidak dicatat ke log server.",
          takeaway: "Ketiadaan visibilitas saat terjadi serangan siber.",
        },
        {
          physical: "Buku catatan tamu pos satpam ditulis menggunakan pensil yang mudah dihapus.",
          digital: "File log disimpan dalam file teks biasa di server web dan rentan serangan Log Tampering.",
          takeaway: "Integritas catatan audit tidak dapat dipertanggungjawabkan di pengadilan.",
        },
        {
          physical: "Alarm sirene bank dipasang tanpa sambungan telepon ke kantor polisi terdekat.",
          digital: "Aplikasi mencatat error ke disk namun tidak memiliki sistem alerting real-time (SIEM).",
          takeaway: "Rata-rata waktu deteksi pembobolan (dwell time) mencapai lebih dari 200 hari!",
        },
      ],
      intuitiveTakeaway:
        "Logging dan monitoring bukanlah fitur pelengkap. Anda tidak bisa mempertahankan apa yang tidak bisa Anda lihat. Tanpa sistem pencatatan yang tangguh, sistem Anda mungkin sudah dibobol sejak 6 bulan lalu tanpa Anda sadari!",
    },
    concept: {
      whyItExists: [
        {
          title: "Ketiadaan Pencatatan Peristiwa Keamanan Kritis",
          explanation: "Developer hanya mencatat event normal (misal 'Order dibuat') namun tidak mencatat kegagalan autentikasi, perubahan password, atau pelanggaran otorisasi.",
          mistake: "Aktivitas peretasan tidak memicu jejak investigasi forensik.",
        },
        {
          title: "Log Injection (CRLF Injection)",
          explanation: "Mencatat input pengguna langsung ke file log tanpa sanitasi karakter baris baru (\\r\\n).",
          mistake: "Penyerang menyisipkan catatan palsu untuk mengelabui auditor keamanan.",
        },
        {
          title: "Penyimpanan Log Lokal Tanpa Backup Terpusat",
          explanation: "File log disimpan di instance server yang sama. Saat server dikompromi, peretas menghapus file log dalam hitungan detik.",
          mistake: "Jejak forensik musnah seketika (Evidence Destruction).",
        },
      ],
      businessImpact: {
        regulatory: "Sanksi ketidakpatuhan standar forensik digital UU ITE & ISO 27001.",
        financial: "Pembengkakan biaya penanganan insiden siber karena investigasi forensik menemui jalan buntu.",
        reputational: "Ketidakmampuan memberikan klarifikasi hukum transparan kepada para nasabah.",
        bountyReward: "Bounty Audit Logging Flaw: $1,500 – $5,000+.",
      },
      attackerVsDefender: {
        attackerPerspective: "Jika saya mencoba login 500 kali dengan kata sandi salah, apakah ada sistem yang memblokir IP saya atau memberi notifikasi ke tim keamanan?",
        defenderPerspective: "Alirkan semua audit trail ke SIEM terpusat (Elasticsearch/Splunk) yang read-only, dan buat alerting otomatis saat terjadi anomali lonjakan 401/403.",
      },
    },
    walkthrough: {
      summary: "Audit mekanisme logging dan deteksi insiden pada AuditLog Vanguard (:8019)",
      targetBaseUrl: "http://127.0.0.1:8019",
      steps: [
        {
          stepNumber: 1,
          badge: "TAMPERING PROBE",
          title: "Pengujian Pencatatan Percobaan Autentikasi Anomali",
          description: "Kirimkan sejumlah percobaan otentikasi mencurigakan dan periksa apakah sistem mencatat jejak insiden.",
          endpoint: "/api/audit/probe",
          method: "POST",
          command: 'curl -s -X POST "http://127.0.0.1:8019/api/audit/probe" -d "action=unauthorized_wire_transfer"',
          expectedStatus: "200 OK",
          expectedResponse: '{"logged": false, "alert_triggered": false}',
          underTheHood: "Sistem mendeteksi bahwa aksi berisiko tinggi diabaikan oleh logger aplikasi.",
        },
        {
          stepNumber: 2,
          badge: "LOG INJECTION",
          title: "Eksfiltrasi Bukti Ketiadaan Sentinel Audit",
          description: "Kirimkan payload pengujian injeksi log CRLF untuk membuktikan kelemahan sanitasi dan klaim flag CTF.",
          endpoint: "/api/audit/verify",
          method: "POST",
          command: 'curl -s -X POST "http://127.0.0.1:8019/api/audit/verify" -d "token=SENTINEL_TRIGGER"',
          expectedStatus: "200 OK",
          expectedResponse: "LOGGING VULNERABILITY CONFIRMED | Flag: ZITERA{...}",
          underTheHood: "Engine audit mendeteksi celah kegagalan monitoring dan merilis token bukti evaluasi.",
        },
      ],
    },
    remediation: {
      title: "Arsitektur Logging Terpusat & Sanitasi Anti-CRLF",
      vulnerableSnippet: {
        language: "python",
        filename: "vulnerable_logger.py",
        code: `# ❌ RENTAN: Menulis input pengguna langsung ke file teks log!
def log_user_action(username, action):
    # Bahaya! Jika username memuat '\\r\\n[ADMIN LOGIN SUCCESS]', penyerang memalsukan log!
    with open('/var/log/app.log', 'a') as f:
        f.write(f"USER: {username} | ACTION: {action}\\n")`,
        flawExplanation: "Karakter newline (\\n / \\r) memungkinkan penyerang menyisipkan baris log palsu (Log Injection).",
      },
      secureSnippet: {
        language: "python",
        filename: "secure_logger.py",
        code: `import json, logging
from datetime import datetime, timezone

# ✅ AMAN: Structured JSON Logging yang otomatis mensanitasi karakter kontrol
logger = logging.getLogger("audit_sentinel")

def log_security_event(user_id, event_type, status, ip_address):
    event_payload = {
        "timestamp": datetime.now(timezone.utc).isoformat(),
        "user_id": user_id,
        "event_type": event_type,
        "status": status,
        "client_ip": ip_address
    }
    # Format JSON mencegah injeksi CRLF dan siap diindeks oleh ElasticSearch / CloudWatch
    logger.info(json.dumps(event_payload))`,
        fixExplanation: "Format JSON men-serialize karakter khusus menjadi escape sequence (\\n), mencegah pemalsuan baris audit.",
      },
      securityDiffs: [
        { component: "Log Format", before: "Plain text concatenation", after: "Structured JSON Serialization", impact: "Kebal terhadap serangan Log Injection CRLF" },
        { component: "Log Storage", before: "File lokal server web", after: "Centralized Remote SIEM (WORM)", impact: "Mencegah penghapusan jejak forensik oleh peretas" },
      ],
      defensePrinciples: [
        { number: 1, title: "Structured JSON Logging", desc: "Format seluruh log audit dalam JSON standar agar mudah dianalisis dan kebal injeksi teks." },
        { number: 2, title: "Penyimpanan Write-Once Read-Many (WORM)", desc: "Kirimkan log ke sistem terisolasi yang tidak dapat diubah atau dihapus oleh siapapun." },
      ],
    },
  },

  "A10": {
    labId: "A10",
    analogy: {
      storyTitle: "Satpam Kantor yang Diperdaya Tamu Membobol Ruang Server",
      icon: "🏢 🌐",
      realWorldScenario:
        "Seorang pengunjung asing berdiri di depan lobi kantor dan berkata dengan sopan kepada satpam: 'Pak, tolong ambilkan map berkas merah di laci meja ruang server bawah tanah lantai B2, ini nomor lokernya.'. Satpam yang ramah dan patuh langsung berjalan ke ruang server terlarang dan membawakan berkas rahasia tersebut kepada pengunjung di lobi! Satpam tersebut memiliki akses penuh ke seluruh gedung, sedangkan pengunjung tidak. Inilah Server-Side Request Forgery (SSRF): peretas memperdaya server backend untuk meminta data dari sistem internal yang tidak dapat diakses langsung oleh publik.",
      comparisonMatrix: [
        {
          physical: "Satpam kantor masuk ke area terlarang internal atas suruhan pengunjung luar.",
          digital: "Server backend membuat koneksi HTTP ke localhost (127.0.0.1) atau IP privat cloud (169.254.169.254).",
          takeaway: "Server digunakan sebagai proxy penembus batas perimeter internal.",
        },
        {
          physical: "Pengunjung tidak boleh masuk ruang server, namun mendapatkan berkas rahasia via satpam.",
          digital: "Peretas di internet tidak bisa mengakses API database internal, namun server web mengambilkannya.",
          takeaway: "Bypass firewall dan perbatasan jaringan internal perusahaan.",
        },
        {
          physical: "Satpam hanya diizinkan mengambil barang di loker paket publik di teras depan gedung.",
          digital: "Aplikasi menerapkan Allowlist domain eksternal ketat dan memblokir seluruh rentang IP privat RFC 1918.",
          takeaway: "Penegakan kontrol batas komunikasi server.",
        },
      ],
      intuitiveTakeaway:
        "Jangan biarkan server backend Anda menjadi 'kaki tangan' peretas! Jika fitur aplikasi Anda menerima URL dari pengguna, jangan pernah mengizinkan server membuat koneksi ke alamat IP internal (localhost, jaringan LAN privat, atau metadata AWS/GCP)!",
    },
    concept: {
      whyItExists: [
        {
          title: "Fitur Fetch URL Tanpa Validasi IP Internal",
          explanation: "Fitur seperti 'Ambil Gambar Profil dari URL', 'Webhook Integrator', atau 'PDF Generator dari Link' meminta URL dari klien.",
          mistake: "Developer langsung mengeksekusi requests.get(user_url) tanpa memeriksa tujuan alamat IP.",
        },
        {
          title: "Akses Terbuka ke Cloud Instance Metadata Service (IMDS)",
          explanation: "Layanan cloud seperti AWS EC2 mengekspos endpoint link-local http://169.254.169.254/ yang memuat IAM security credentials.",
          mistake: "Attacker mengekstrak token rahasia instance AWS dan mengambil alih seluruh infrastruktur cloud.",
        },
      ],
      businessImpact: {
        regulatory: "Pelanggaran serius isolasi jaringan komputasi awan dan privasi data.",
        financial: "Pembajakan seluruh infrastruktur komputasi awan (AWS / Azure / GCP) oleh aktor ancaman.",
        reputational: "Insiden breach berskala global seperti kebocoran data Capital One (100+ juta data nasabah).",
        bountyReward: "Bounty Critical SSRF to Metadata: $5,000 – $25,000+.",
      },
      attackerVsDefender: {
        attackerPerspective: "Jika input url=http://example.com/avatar.png saya ubah menjadi http://127.0.0.1:8020/admin atau http://169.254.169.254, apakah server mengembalikan responsnya ke saya?",
        defenderPerspective: "Resolusikan domain ke IP, pastikan IP BUKAN loopback (127.0.0.0/8), BUKAN privat RFC 1918 (10.0.0.0/8, 172.16.0.0/12, 192.168.0.0/16), dan nonaktifkan redirect HTTP.",
      },
    },
    walkthrough: {
      summary: "Audit keamanan Server-Side Request Forgery pada WebHooker Cloud Integrator (:8020)",
      targetBaseUrl: "http://127.0.0.1:8020",
      steps: [
        {
          stepNumber: 1,
          badge: "NORMAL URL FETCH",
          title: "Pengujian Fungsionalitas Fetch URL Eksternal",
          description: "Amati bagaimana layanan WebHooker mengambil data dari alamat web eksternal.",
          endpoint: "/api/fetch?url=http://example.com",
          method: "GET",
          command: 'curl -s -i "http://127.0.0.1:8020/api/fetch?url=http://example.com"',
          expectedStatus: "200 OK",
          expectedResponse: "Example Domain Payload Rendered",
          underTheHood: "Server backend membuat soket HTTP ke internet publik dan meneruskan kembali responsnya ke klien.",
        },
        {
          stepNumber: 2,
          badge: "SSRF INTERNAL PROBE",
          title: "Penyusupan ke Endpoint Layanan Internal (Localhost)",
          description: "Kirimkan URL internal loopback (127.0.0.1) untuk menembus kontrol akses perimeter konsol internal.",
          endpoint: "/api/fetch?url=http://127.0.0.1:8020/internal-status",
          method: "GET",
          command: 'curl -s -i "http://127.0.0.1:8020/api/fetch?url=http://127.0.0.1:8020/internal-status"',
          expectedStatus: "200 OK",
          expectedResponse: "INTERNAL STATUS: CLOUD CLUSTER ONLINE | Flag: ZITERA{...}",
          underTheHood: "Server rentan mengeksekusi request ke soket loopback internalnya sendiri tanpa filter IP, mengembalikan data terlarang!",
        },
      ],
    },
    remediation: {
      title: "Pertahanan SSRF Komprehensif: Resolusi IP & Strict Allowlist",
      vulnerableSnippet: {
        language: "python",
        filename: "vulnerable_ssrf.py",
        code: `import requests
from flask import request

@app.route('/fetch')
def fetch_url():
    target_url = request.args.get('url')
    # ❌ RENTAN SSRF: Langsung mengambil URL input pengguna tanpa validasi IP!
    resp = requests.get(target_url, timeout=5)
    return resp.text`,
        flawExplanation: "Penyerang dapat mengirimkan http://169.254.169.254 atau http://127.0.0.1 untuk membongkar credential internal.",
      },
      secureSnippet: {
        language: "python",
        filename: "secure_ssrf.py",
        code: `import socket, ipaddress, urllib.parse, requests

def validate_safe_url(url):
    parsed = urllib.parse.urlparse(url)
    if parsed.scheme not in ('http', 'https'):
        raise ValueError("Skema protokol tidak diizinkan.")

    # 1. Resolusi nama domain ke alamat IP
    hostname = parsed.hostname
    ip_addr_str = socket.gethostbyname(hostname)
    ip_obj = ipaddress.ip_address(ip_addr_str)

    # 2. Blokir alamat loopback, link-local (cloud metadata), dan private LAN!
    if ip_obj.is_loopback or ip_obj.is_private or ip_obj.is_link_local:
        raise SecurityException("Akses ke rentang IP internal diblokir mutlak!")

    return url

@app.route('/fetch')
def fetch_url():
    safe_url = validate_safe_url(request.args.get('url'))
    # Nonaktifkan HTTP redirect agar attacker tidak bisa bypass filter
    resp = requests.get(safe_url, allow_redirects=False, timeout=5)
    return resp.text`,
        fixExplanation: "Memvalidasi alamat IP yang di-resolve terhadap rentang private RFC 1918 dan link-local meniadakan SSRF secara tuntas.",
      },
      securityDiffs: [
        { component: "IP Validation", before: "Tanpa pemeriksaan (Mempercayai URL mentah)", after: "Resolusi DNS + Blokir Loopback/Private IP", impact: "Mencegah akses ke localhost dan metadata cloud" },
        { component: "HTTP Redirects", before: "allow_redirects=True", after: "allow_redirects=False", impact: "Meniadakan serangan redirect bypass" },
      ],
      defensePrinciples: [
        { number: 1, title: "Blokir Rentang IP Privat & Link-Local", desc: "Wajib tolak 127.0.0.0/8, 10.0.0.0/8, 172.16.0.0/12, 192.168.0.0/16, dan 169.254.169.254." },
        { number: 2, title: "Wajibkan AWS IMDSv2", desc: "Pada arsitektur AWS EC2, wajibkan IMDSv2 dengan HTTP PUT session token untuk memblokir SSRF sederhana." },
      ],
    },
  },
}

// Fallback generator for labs that are not explicitly hardcoded above
export function getInteractiveCurriculumForLab(labId: string, defaultPort = 8011): LabInteractiveCurriculum {
  if (LAB_INTERACTIVE_REGISTRY[labId]) {
    return LAB_INTERACTIVE_REGISTRY[labId]
  }

  // Generative fallback based on standard OWASP Top 10 template
  return {
    labId,
    analogy: {
      storyTitle: `Analogi Konseptual Lab ${labId}`,
      icon: "🎯 🛡️",
      realWorldScenario:
        `Pada skenario pengujian lab ${labId}, sistem komersial menghadapi risiko kegagalan kontrol keamanan pada arsitektur layanannya. Bayangkan prosedur keamanan gedung yang mengabaikan verifikasi izin pada pintu akses internal, sehingga siapapun yang berada di dalam dapat membuka dokumen rahasia perusahaan.`,
      comparisonMatrix: [
        {
          physical: "Pintu masuk gedung mengizinkan pengunjung tanpa kartu akses resmi.",
          digital: "Endpoint server menerima parameter tanpa otorisasi berlapis.",
          takeaway: "Celah kontrol keamanan terbuka untuk dieksploitasi.",
        },
        {
          physical: "Petugas keamanan mengasumsikan semua orang di lobi adalah staf terdaftar.",
          digital: "Aplikasi berasumsi bahwa setiap request dari klien selalu sah.",
          takeaway: "Ketiadaan prinsip Zero Trust pada lapisan pemrosesan.",
        },
      ],
      intuitiveTakeaway:
        "Terapkan selalu prinsip Zero Trust: verifikasi secara eksplisit, tegakkan hak akses paling minim (Least Privilege), dan selalu asumsikan batas perimeter dapat dilanggar!",
    },
    concept: {
      whyItExists: [
        {
          title: "Asumsi Validitas Input Klien",
          explanation: "Developer mengandalkan validasi di browser tanpa pemeriksaan ulang di sisi backend.",
          mistake: "Mengabaikan verifikasi sisi server (Server-Side Enforcement).",
        },
        {
          title: "Ketiadaan Kontrol Keamanan Berlapis",
          explanation: "Hanya satu titik pemeriksaan ad-hoc yang diterapkan pada rute aplikasi.",
          mistake: "Tidak menerapkan pertahanan berlapis (Defense in Depth).",
        },
      ],
      businessImpact: {
        regulatory: "Sanksi regulasi perlindungan data pribadi dan standar kepatuhan industri.",
        financial: "Potensi kerugian finansial akibat eksfiltrasi data aset korporat.",
        reputational: "Kerusakan reputasi merek dan pudarnya kepercayaan mitra bisnis.",
        bountyReward: "Bounty P2/P1: $1,000 – $5,000+.",
      },
      attackerVsDefender: {
        attackerPerspective: "Bagaimana jika saya mengirimkan nilai parameter yang tidak terduga atau memodifikasi request header?",
        defenderPerspective: "Validasi setiap masukan secara defensif di server dan terapkan prinsip Deny by Default.",
      },
    },
    walkthrough: {
      summary: `Panduan investigasi keamanan terarah untuk lab ${labId} (: ${defaultPort})`,
      targetBaseUrl: `http://127.0.0.1:${defaultPort}`,
      steps: [
        {
          stepNumber: 1,
          badge: "RECONNAISSANCE",
          title: "Observasi Target & Pemetaan Rute Layanan",
          description: `Akses target pada peramban http://127.0.0.1:${defaultPort} dan petakan titik interaksi pengguna.`,
          endpoint: "/",
          method: "GET",
          command: `curl -s -i "http://127.0.0.1:${defaultPort}/"`,
          expectedStatus: "200 OK",
          expectedResponse: "Service Active & Responsive",
          underTheHood: "Server menerima request awal dan mengembalikan antarmuka web beserta struktur cookie.",
        },
        {
          stepNumber: 2,
          badge: "EXPLOITATION & PROOF",
          title: "Pengujian Hipotesis Kerentanan & Pengambilan Bukti",
          description: "Kirimkan payload pengujian sesuai panduan investigasi untuk merekonstruksi celah dan mengambil flag CTF.",
          endpoint: "/api/check",
          method: "GET",
          command: `curl -s -i "http://127.0.0.1:${defaultPort}/api/check"`,
          expectedStatus: "200 OK",
          expectedResponse: "FLAG CAPTURED: ZITERA{...}",
          underTheHood: "Mekanisme pengujian memvalidasi bukti eksploitasi dan mengembalikan respons terverifikasi.",
        },
      ],
    },
    remediation: {
      title: "Prinsip Remediasi & Penulisan Kode Defensif",
      vulnerableSnippet: {
        language: "text",
        filename: "vulnerable_code.txt",
        code: `// Pola implementasi yang belum menerapkan kontrol defensif:
// Input pengguna diteruskan langsung ke proses tanpa sanitasi dan validasi otorisasi.`,
        flawExplanation: "Ketiadaan validasi defensif di sisi server membuka celah manipulasi.",
      },
      secureSnippet: {
        language: "text",
        filename: "secure_code.txt",
        code: `// Pola implementasi aman (Defensive Architecture):
// 1. Terapkan validasi input ketat (Allowlist).
// 2. Tegakkan otorisasi berbasis sesi server.
// 3. Catat aktivitas sensitif ke sistem log yang terlindungi.`,
        fixExplanation: "Pendekatan defensif berlapis menjamin keamanan data bahkan jika parameter klien dimanipulasi.",
      },
      securityDiffs: [
        { component: "Server-Side Validation", before: "Tidak ada", after: "Validasi ketat diterapkan", impact: "Menolak seluruh masukan yang tidak sesuai skema" },
      ],
      defensePrinciples: [
        { number: 1, title: "Deny by Default", desc: "Tutup semua akses kecuali yang diizinkan secara eksplisit." },
        { number: 2, title: "Defense in Depth", desc: "Terapkan kontrol keamanan di setiap tingkatan aplikasi." },
      ],
    },
  }
}
