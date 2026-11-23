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

// ── CANONICAL SOCRATIC CURRICULUM DATA ────────────────────────────────────────

export const SOCRATIC_CURRICULUM_DATA: Record<string, SocraticLabCurriculum> = {
  F01: {
    labId: 'F01',
    labTitle: 'F01: Dasar Protokol HTTP & Web Architecture',
    tacticalRole: 'Network Protocol & Message Structure Reconnaissance',
    missionOverview:
      'Memahami siklus request-response HTTP, struktur header, metode transmisi (GET, POST, dsb), serta bagaimana server memetakan atribut request klien ke logika pemrosesan internal.',
    keywords: [
      'http',
      'request',
      'response',
      'header',
      'verb',
      'method',
      'get',
      'post',
      'user-agent',
      'content-type',
      'status code',
      '200',
      '404',
      '403',
      '500',
    ],
    suggestedPrompts: [
      'Parameter atau header apa yang perlu saya amati pertama kali di F01?',
      'Mengapa server mempercayai data dari header klien?',
      'Bagaimana cara memodifikasi HTTP verb dengan curl atau Burp?',
      'Apa implikasi keamanan jika server tidak memvalidasi Content-Type?',
    ],
    clues: [
      {
        level: 1,
        title: 'Pemetaan Header & Parameter Request',
        category: 'Observational',
        summary:
          'Amati anatomi paket HTTP yang dikirimkan oleh browser atau klien HTTP ke endpoint target.',
        guidance:
          'Buka DevTools (F12) Network tab atau Burp Suite HTTP history. Perhatikan secara spesifik: 1) Metode HTTP apa yang digunakan pada endpoint target? 2) Header apa yang mengontrol representasi data (Content-Type, Accept)? 3) Apakah ada header khusus seperti X-Requested-With, Authorization, atau parameter query string yang memicu respons berbeda saat nilainya diubah?',
        investigativeQuestions: [
          'Apakah server merespons secara berbeda saat User-Agent atau Accept diubah?',
          'Bagaimana respons server terhadap request OPTIONS atau HEAD pada endpoint ini?',
          'Data apa saja yang dikirimkan klien yang sebenarnya berada di luar kendali server?',
        ],
        tacticalAction:
          'Kirim request observasi via curl dengan opsi verbose (-v) untuk membaca raw header secara utuh.',
        codeBlueprint:
          'curl -v -X GET "http://127.0.0.1:<PORT>/api/status" \\\n  -H "Accept: application/json" \\\n  -H "User-Agent: Mozilla/5.0 (Security-Audit-Agent)"',
      },
      {
        level: 2,
        title: 'Pelanggaran Batas Kepercayaan (Client-Controlled Attributes)',
        category: 'Vulnerability Logic',
        summary:
          'Pahami mengapa mempercayai atribut yang dapat dimanipulasi klien merusak batas keamanan sistem.',
        guidance:
          'Arsitektur web sering kali keliru berasumsi bahwa input yang tiba dari browser pasti dihasilkan oleh kode frontend resmi mereka. Jika backend mengandalkan header HTTP khusus klien (seperti header peran buatan, cookie nilai mentah tanpa tanda tangan kriptografis, atau HTTP verb) untuk mengarahkan alur eksekusi, batas kepercayaan (trust boundary) runtuh seketika karena klien HTTP mana pun dapat merekayasa paket tersebut.',
        investigativeQuestions: [
          'Mengapa validasi di frontend JavaScript tidak pernah dianggap sebagai kontrol keamanan?',
          'Bagaimana backend seharusnya memverifikasi status pengguna secara independen dari header mentah?',
          'Apa bahaya mengizinkan sembarang metode HTTP masuk ke controller sensitif?',
        ],
        tacticalAction:
          'Lakukan pengujian HTTP Verb Tampering dengan mengubah request GET menjadi POST atau PUT pada endpoint terkait.',
        codeBlueprint:
          '// Vulnerable Logic Pattern:\n// Backend mengecek rute berdasarkan header client tanpa verifikasi session server:\nif (req.headers["x-client-role"] === "auditor" || req.method === "POST") {\n  // Logika privilegi terbuka bagi penyerang yang mengubah header!\n  executePrivilegedRoutine();\n}',
      },
      {
        level: 3,
        title: 'Cetak Biru Tampering Parameter & Header',
        category: 'Payload Blueprint',
        summary:
          'Struktur template manipulasi request untuk mengekstrak informasi tanpa merusak integritas lingkungan lab.',
        guidance:
          'Gunakan Burp Suite Repeater atau curl untuk merekonstruksi request dengan memanipulasi verb dan menyisipkan header konteks. Ingat: jangan mencoba menebak token atau flag secara acak—fokuslah memicu respons server yang membuka status sistem atau variabel diagnostik yang berisi flag pembuktian.',
        investigativeQuestions: [
          'Apakah respons status berubah dari 403 Forbidden menjadi 200 OK setelah verb diganti?',
          'Apakah body respons menyertakan metadata internal yang sebelumnya disembunyikan?',
        ],
        tacticalAction:
          'Kirim request buatan dengan format blueprint di bawah ini melalui terminal lab atau Burp Repeater.',
        codeBlueprint:
          'POST /api/v1/diagnostic/inspect HTTP/1.1\nHost: 127.0.0.1:<PORT>\nContent-Type: application/json\nX-Requested-With: XMLHttpRequest\nX-Forwarded-For: 127.0.0.1\n\n{\n  "action": "protocol_audit",\n  "scope": "internal_telemetry"\n}',
      },
    ],
  },

  F06: {
    labId: 'F06',
    labTitle: 'F06: Mekanisme Lab ZITERA & Pengiriman Flag CTF',
    tacticalRole: 'Cyber Range Verification & Enclave Protocol',
    missionOverview:
      'Menguasai arsitektur pengujian laboratorium ZITERA, siklus container enclave, serta mekanisme validasi bukti eksploitasi (Capture The Flag) berstandar industri.',
    keywords: [
      'flag',
      'ctf',
      'zitera{',
      'submit',
      'verifikasi',
      'enclave',
      'container',
      'docker',
      'token',
      'hash',
      'bukti',
      'challenge',
    ],
    suggestedPrompts: [
      'Bagaimana format resmi penyerahan flag di ZITERA LAB?',
      'Di mana flag biasanya diekspos setelah eksploitasi berhasil?',
      'Mengapa flag saya dianggap tidak valid saat di-submit?',
      'Bagaimana cara memastikan container lab berada dalam status siap?',
    ],
    clues: [
      {
        level: 1,
        title: 'Identifikasi Pola Output & Endpoint Verifikasi',
        category: 'Observational',
        summary:
          'Kenali format pembungkus flag deterministik dan lokasi verifikasi bukti.',
        guidance:
          'Semua laboratorium di ZITERA LAB menggunakan pembungkus bukti eksploitasi seragam. Perhatikan: 1) Format flag selalu diawali dengan prefix ZITERA{ dan diakhiri dengan tanda kurung kurawal penutup }. 2) Di dalam terminal interaktif atau respons HTTP, flag biasanya muncul setelah kondisi kerentanan terpenuhi. 3) Buka tab Challenge / Verifikasi di panel kontrol laboratorium untuk menemukan form submission.',
        investigativeQuestions: [
          'Apakah output terminal atau respons JSON mengandung string yang diawali prefix ZITERA{?',
          'Apakah Anda telah mencatat seluruh string token termasuk tanda kurung kurawal?',
          'Apakah container laboratorium berada dalam status RUNNING (hijau)?',
        ],
        tacticalAction:
          'Gunakan utilitas grep di terminal untuk mencari pola ekspresi reguler flag.',
        codeBlueprint:
          '# Cari pola flag pada file respons atau log runtime:\ngrep -oE "ZITERA\\{[A-Za-z0-9_\\-]+\\}" /tmp/execution.log || echo "Flag belum diekspos."',
      },
      {
        level: 2,
        title: 'Integritas Pembuktian Eksploitasi & State Verification',
        category: 'Vulnerability Logic',
        summary:
          'Pahami mengapa flag terikat pada kondisi eksploitasi dan sesi aktif container.',
        guidance:
          'Flag ZITERA bukan sekadar kata sandi statis, melainkan representasi kriptografis bahwa Anda berhasil mengeksekusi urutan taktis yang diminta. Engine verifikasi backend memvalidasi hash flag terhadap session ID unik container Anda. Jika Anda mencoba men-submit flag yang ditemukan di internet atau dari sesi laboratorium lama yang telah di-reset, verifier akan menolaknya karena signature tidak sinkron.',
        investigativeQuestions: [
          'Mengapa container yang di-reset menghasilkan instance baru dengan sesi berbeda?',
          'Bagaimana backend memverifikasi bahwa flag diperoleh dari pemicu eksploitasi nyata?',
          'Apa perbedaan antara flag tantangan dan kredensial login akun pengguna?',
        ],
        tacticalAction:
          'Periksa status runtime lab melalui tab Diagnostics atau panel Kontrol Kontainer sebelum melakukan submit.',
        codeBlueprint:
          '// Verifier Logic Architecture:\n// Server memvalidasi keterikatan token dengan session container:\nconst expectedToken = hmacSha256(containerSessionId + labId, enclaveMasterKey);\nif (submittedToken !== expectedToken) {\n  throw new VerificationError("TOKEN_SESSION_MISMATCH: Restart atau selesaikan eksploitasi ulang.");\n}',
      },
      {
        level: 3,
        title: 'Cetak Biru Ekstraksi Bukti & Format Submit',
        category: 'Payload Blueprint',
        summary:
          'Prosedur standar ekstraksi dan verifikasi bukti tanpa membocorkan nilai hash spesifik.',
        guidance:
          'Setelah Anda mengeksekusi payload eksploitasi dan melihat string dengan format ZITERA{...}, salin nilai tersebut dengan teliti tanpa menyertakan spasi di awal atau akhir. Buka panel Submission pada detail lab, tempelkan nilai tersebut, dan tekan tombol Verifikasi. Jika verifikasi berhasil, status tantangan akan berubah menjadi SOLVED dan skor Anda akan dicatat di scoreboard lokal.',
        investigativeQuestions: [
          'Apakah Anda memastikan tidak ada karakter spasi atau newline yang terselip saat menyalin?',
          'Apakah status jaringan lokal Anda stabil saat mengirimkan submission?',
        ],
        tacticalAction:
          'Verifikasi format flag dengan validator di bawah sebelum menekan tombol Kirim.',
        codeBlueprint:
          '// Format baku submission:\nFlagFormat: ZITERA{alphanumeric_or_hex_token_here}\nContoh Struktur (Bukan Flag Sebenarnya):\nZITERA{example_token_blueprint_f06_do_not_submit}\n\n// Submit endpoint via engineClient:\nPOST /api/v1/challenge/verify\nBody: { "lab_id": "F06", "flag": "ZITERA{...}" }',
      },
    ],
  },

  A01: {
    labId: 'A01',
    labTitle: 'A01: Broken Access Control (VaultPay Enterprise IDOR)',
    tacticalRole: 'Authorization Boundary & Tenant Isolation Analysis',
    missionOverview:
      'Menganalisis kegagalan kontrol akses horizontal (Insecure Direct Object Reference / IDOR) pada portal finansial multi-tenant VaultPay Enterprise, di mana penyerang dapat mengakses invoice rahasia milik pengguna lain.',
    keywords: [
      'idor',
      'bola',
      'access control',
      'broken access',
      'authorization',
      'vaultpay',
      'invoice',
      'document',
      'account_id',
      'doc_id',
      'tenant',
      'privilege',
      'horizontal',
    ],
    suggestedPrompts: [
      'Bagaimana cara menemukan parameter IDOR pada VaultPay Enterprise?',
      'Kenapa pengguna sah bisa mengakses invoice orang lain?',
      'Bagaimana membedakan otentikasi (401) dan otorisasi (403)?',
      'Apa payload template untuk menguji IDOR di Burp Suite Repeater?',
    ],
    clues: [
      {
        level: 1,
        title: 'Pemeriksaan Parameter Objek Langsung (Direct Object Reference)',
        category: 'Observational',
        summary:
          'Telusuri parameter identifikasi numerik atau ID referensi pada URL dan body request.',
        guidance:
          'Masuk ke portal VaultPay menggunakan akun pengguna standar yang diberikan. Navigasikan ke menu Transaksi atau Riwayat Invoice. Buka DevTools Network tab atau Burp Suite HTTP history. Amati request yang terjadi saat Anda melihat invoice Anda sendiri: apakah ada endpoint seperti `/api/documents/{id}`, `/api/invoices?doc_id=1001`, atau `/api/users/profile`? Perhatikan apakah ID tersebut berupa angka berurutan (inkremental) yang mudah ditebak.',
        investigativeQuestions: [
          'Berapa nilai ID dokumen invoice milik akun Anda saat ini?',
          'Apakah ada pola angka berurutan seperti 1001, 1002, 1003?',
          'Apakah Anda melihat indikator kepemilikan dokumen di URL atau body JSON?',
        ],
        tacticalAction:
          'Kirim request sah melihat dokumen milik sendiri ke Burp Suite Repeater dengan menekan Ctrl+R.',
        codeBlueprint:
          'GET /api/documents/download?doc_id=1001 HTTP/1.1\nHost: 127.0.0.1:<PORT>\nAuthorization: Bearer <STUDENT_JWT_TOKEN>\nAccept: application/json',
      },
      {
        level: 2,
        title: 'Pemisahan Autentikasi vs Otorisasi',
        category: 'Vulnerability Logic',
        summary:
          'Pahami kegagalan arsitektur di mana backend memvalidasi siapa Anda, tetapi lupa memeriksa hak Anda atas objek.',
        guidance:
          'Aplikasi VaultPay dengan benar memeriksa apakah Anda telah login (autentikasi berhasil melalui token JWT atau session cookie). Namun, saat query basis data dijalankan untuk mengambil invoice, backend hanya mengeksekusi query naif: `SELECT * FROM invoices WHERE id = :doc_id`. Backend sama sekali TIDAK menyertakan filter relasi kepemilikan: `AND user_id = :session_user_id`. Akibatnya, siapapun yang memiliki akun login yang sah dapat membaca dokumen rahasia milik pengguna lain hanya dengan mengganti nomor ID.',
        investigativeQuestions: [
          'Mengapa pemeriksaan login saja tidak cukup untuk melindungi data pengguna?',
          'Apa yang membedakan eskalasi hak istimewa horizontal (antar user selevel) dan vertikal (user ke admin)?',
          'Bagaimana pengembang seharusnya menulis query database yang aman dari IDOR?',
        ],
        tacticalAction:
          'Ganti nilai doc_id di Burp Repeater dengan ID sebelum atau sesudah nomor invoice Anda.',
        codeBlueprint:
          '// Vulnerable Backend Implementation:\napp.get("/api/documents/download", authenticateToken, async (req, res) => {\n  const docId = req.query.doc_id;\n  // FLAW: Tidak memvalidasi apakah req.user.id adalah pemilik dokumen ini!\n  const doc = await db.query("SELECT * FROM invoices WHERE id = ?", [docId]);\n  if (!doc) return res.status(404).send("Document not found");\n  return res.json(doc); // Mengembalikan dokumen target ke sembarang user!\n});',
      },
      {
        level: 3,
        title: 'Cetak Biru Tampering ID Objek & Ekstraksi Dokumen Sasaran',
        category: 'Payload Blueprint',
        summary:
          'Struktur request manipulasi ID target untuk mengekstrak invoice administratif tanpa merusak data.',
        guidance:
          'Di Burp Suite Repeater, pertahankan token Authorization sah milik akun Anda. Kemudian ubah nilai identifier target ke ID yang dicurigai menyimpan invoice rahasia manajemen (misalnya ID 1000, 1002, atau ID administratif awal). Kirim request dan telaah responsnya: apakah data transaksi milik nasabah lain muncul beserta flag pembuktian keberhasilan audit?',
        investigativeQuestions: [
          'Berapa ID dokumen target yang mengembalikan status 200 OK dengan data milik eksekutif?',
          'Apakah dokumen tersebut memuat token bukti ZITERA{...} di dalam rincian transaksi?',
        ],
        tacticalAction:
          'Lakukan uji tamper parameter ID secara terkontrol menggunakan template di bawah.',
        codeBlueprint:
          'GET /api/documents/download?doc_id=1000 HTTP/1.1\nHost: 127.0.0.1:<PORT>\nAuthorization: Bearer <YOUR_CURRENT_ACTIVE_TOKEN>\nUser-Agent: ZiteraSecurityAuditor/2.0\nAccept: application/json\n\n// Harapan Respons:\n// Status 200 OK dengan field invoice_owner: "CFO_VaultPay" dan metadata flag.',
      },
    ],
  },
}

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
          '# Catat dan uji endpoint target:\ncurl -s -D - "http://127.0.0.1:<PORT>/" -o /dev/null',
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
          '// Pola analisis logika kontrol:\n// Input Klien -> Gateway Autentikasi -> [KONTROL OTORISASI / VALIDASI?] -> Eksekusi Sumber Daya',
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
          '// Cetak Biru Pengujian Standar:\nMETHOD /target/endpoint HTTP/1.1\nHost: 127.0.0.1:<PORT>\nContent-Type: application/json\n\n{\n  "test_parameter": "tampered_value"\n}',
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
      'SQL Injection terjadi saat masukan mentah klien digabungkan langsung ke dalam string query SQL tanpa parameterisasi. Langkah investigasi: sisipkan karakter pemutus sintaks seperti tanda petik tunggal (\') atau ganda (") untuk melihat apakah server memunculkan pesan kesalahan sintaks database. Jika ada error, Anda telah menemukan titik injeksi potensial.',
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
    regex: /(stuck|bingung|buntu|tolong|help|nggak tau|pusing|harus gimana|langkah awal)/i,
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
