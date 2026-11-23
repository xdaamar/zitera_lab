// ZITERA CYBER_LAB — Fuzzy Semantic Intelligence Engine
// Serverless, zero-API-cost heuristic parser & pedagogical AI Overview generator.
// Implements Levenshtein distance, token fuzzy matching, cyber jargon normalization,
// colloquial Indonesian/English parsing, and deterministic lab curriculum synthesis.

import {
  getSocraticCurriculum,
  getLabPayloadBlueprint,
  type SocraticLabCurriculum,
} from './zetaSocraticData'

export interface ZiteraAiOverviewData {
  diagnosticSnapshot: string
  reconHypothesis: string
  socraticQuestion: string
  suggestedActions: string[]
  payloadBlueprint?: {
    title: string
    payload: string
    description: string
    targetParam?: string
  }
  isTypoCorrected: boolean
  correctedQuery?: string
  confidenceScore: number
  matchedTopic: string
}

// ── LEVENSHTEIN DISTANCE & TOKEN SIMILARITY ─────────────────────────────────

/**
 * Calculates standard Levenshtein distance between two strings.
 * Case-insensitive, optimal for short tokens and cyber jargon typos.
 */
export function levenshteinDistance(a: string, b: string): number {
  const strA = (a || '').toLowerCase()
  const strB = (b || '').toLowerCase()

  if (strA === strB) return 0
  if (strA.length === 0) return strB.length
  if (strB.length === 0) return strA.length

  const m = strA.length
  const n = strB.length
  let prevRow = new Array(n + 1)
  let currRow = new Array(n + 1)

  for (let j = 0; j <= n; j++) {
    prevRow[j] = j
  }

  for (let i = 1; i <= m; i++) {
    currRow[0] = i
    const charA = strA.charCodeAt(i - 1)

    for (let j = 1; j <= n; j++) {
      const charB = strB.charCodeAt(j - 1)
      const cost = charA === charB ? 0 : 1

      currRow[j] = Math.min(
        prevRow[j] + 1, // deletion
        currRow[j - 1] + 1, // insertion
        prevRow[j - 1] + cost, // substitution
      )
    }

    const temp = prevRow
    prevRow = currRow
    currRow = temp
  }

  return prevRow[n]
}

/**
 * Returns normalized similarity ratio between 0.0 (completely distinct) and 1.0 (identical).
 */
export function tokenSimilarity(a: string, b: string): number {
  const maxLen = Math.max(a.length, b.length)
  if (maxLen === 0) return 1.0
  const dist = levenshteinDistance(a, b)
  return Math.max(0, 1 - dist / maxLen)
}

// ── CYBER DICTIONARY & TYPO CORRECTION MAPS ─────────────────────────────────

interface TermCorrection {
  canonical: string
  category: string
  maxDistance: number
  synonyms?: string[]
}

const CANONICAL_CYBER_TERMS: Record<string, TermCorrection> = {
  // Authentication & Passwords
  password: { canonical: 'password', category: 'auth', maxDistance: 2, synonyms: ['pass', 'passwd', 'pswd', 'katasandi', 'sandi'] },
  authentication: { canonical: 'authentication', category: 'auth', maxDistance: 3, synonyms: ['auth', 'autentikasi', 'otentikasi', 'login', 'signin', 'masuk'] },
  authorization: { canonical: 'authorization', category: 'authz', maxDistance: 3, synonyms: ['authz', 'otorisasi', 'permission', 'privilege', 'hak'] },
  token: { canonical: 'token', category: 'auth', maxDistance: 1, synonyms: ['jwt', 'bearer', 'sessionid'] },
  session: { canonical: 'session', category: 'auth', maxDistance: 2, synonyms: ['cookie', 'sesi', 'cookies'] },

  // Attack Vectors
  'sql injection': { canonical: 'sql injection', category: 'sqli', maxDistance: 2, synonyms: ['sqli', 'injection', 'injeksion', 'injeksi'] },
  'cross-site scripting': { canonical: 'xss', category: 'xss', maxDistance: 2, synonyms: ['xss', 'scripting'] },
  'server-side request forgery': { canonical: 'ssrf', category: 'ssrf', maxDistance: 2, synonyms: ['ssrf', 'metadata'] },
  'cross-site request forgery': { canonical: 'csrf', category: 'csrf', maxDistance: 2, synonyms: ['csrf', 'xsrf'] },
  idor: { canonical: 'idor', category: 'idor', maxDistance: 1, synonyms: ['bola', 'broken access', 'direct object'] },
  bypass: { canonical: 'bypass', category: 'exploit', maxDistance: 2, synonyms: ['nembus', 'tembus', 'lewat', 'skip'] },
  payload: { canonical: 'payload', category: 'exploit', maxDistance: 2, synonyms: ['peload', 'poc', 'exploit'] },
  parameter: { canonical: 'parameter', category: 'recon', maxDistance: 2, synonyms: ['param', 'field', 'input'] },
  header: { canonical: 'header', category: 'protocol', maxDistance: 2, synonyms: ['headers', 'user-agent', 'cookie-header'] },
  burp: { canonical: 'burp suite', category: 'tooling', maxDistance: 1, synonyms: ['burpsuite', 'repeater', 'proxy'] },
  upload: { canonical: 'file upload', category: 'upload', maxDistance: 2, synonyms: ['mime', 'avatar', 'webshell'] },
  traversal: { canonical: 'path traversal', category: 'lfi', maxDistance: 2, synonyms: ['lfi', 'dot dot slash', 'passwd'] },
  concurrency: { canonical: 'race condition', category: 'race', maxDistance: 2, synonyms: ['race', 'toctou', 'double spend'] },
  deserialization: { canonical: 'deserialization', category: 'integrity', maxDistance: 3, synonyms: ['pickle', 'yaml', 'unserialize'] },
  crlf: { canonical: 'log injection', category: 'logging', maxDistance: 1, synonyms: ['siem', 'log forgery'] },

  // HTTP Status Codes
  forbidden: { canonical: '403 forbidden', category: 'status', maxDistance: 2, synonyms: ['403', 'ditolak', 'terlarang'] },
  unauthorized: { canonical: '401 unauthorized', category: 'status', maxDistance: 2, synonyms: ['401', 'belum login'] },
  error: { canonical: '500 server error', category: 'status', maxDistance: 1, synonyms: ['500', 'crash', 'internal error'] },
  flag: { canonical: 'flag ctf', category: 'flag', maxDistance: 1, synonyms: ['ctf', 'zitera', 'jawaban', 'kunci'] },
  stuck: { canonical: 'stuck guidance', category: 'guidance', maxDistance: 1, synonyms: ['buntu', 'mentok', 'bingung', 'pusing', 'lost'] },
}

// Known common raw typos directly mapped to canonical words
const EXACT_TYPO_MAP: Record<string, string> = {
  // Indonesian colloquial & common typos
  passwrod: 'password',
  pasword: 'password',
  passwrd: 'password',
  katasandi: 'password',
  buntu: 'stuck',
  mentok: 'stuck',
  pusing: 'stuck',
  bingung: 'stuck',
  sqlinjecktion: 'sql injection',
  sqlinjection: 'sql injection',
  sqli: 'sql injection',
  injecktion: 'injection',
  injeksion: 'injection',
  forbiden: 'forbidden',
  forbideen: 'forbidden',
  bypas: 'bypass',
  baypas: 'bypass',
  baipass: 'bypass',
  autentikasi: 'authentication',
  otentikasi: 'authentication',
  otorisasi: 'authorization',
  autorization: 'authorization',
  repeter: 'repeater',
  burpsuit: 'burp suite',
  peload: 'payload',
  paylod: 'payload',
  cooki: 'cookie',
  cokie: 'cookie',
  sesi: 'session',
  traversl: 'traversal',
  uplod: 'upload',
  uploadd: 'upload',
}

export interface TypoCorrectionResult {
  correctedQuery: string
  isTypoCorrected: boolean
  corrections: Array<{ original: string; corrected: string }>
}

/**
 * Scans tokens in query and replaces misspellings with canonical cyber terminology.
 */
export function correctQueryTypos(query: string): TypoCorrectionResult {
  const rawQuery = (query || '').trim()
  if (!rawQuery) {
    return { correctedQuery: '', isTypoCorrected: false, corrections: [] }
  }

  const tokens = rawQuery.split(/(\s+|[.,!?]+)/)
  const corrections: Array<{ original: string; corrected: string }> = []
  let isTypoCorrected = false

  const processedTokens = tokens.map((segment) => {
    // If it's punctuation or whitespace, preserve as is
    if (/^[\s.,!?]+$/.test(segment) || !segment) {
      return segment
    }

    const clean = segment.toLowerCase()

    // 1. Check exact typo map
    if (EXACT_TYPO_MAP[clean]) {
      const corrected = EXACT_TYPO_MAP[clean]
      if (clean !== corrected) {
        corrections.push({ original: segment, corrected })
        isTypoCorrected = true
        return corrected
      }
      return segment
    }

    // 2. Check Levenshtein against canonical terms
    for (const [term, meta] of Object.entries(CANONICAL_CYBER_TERMS)) {
      if (clean.length >= 4 && clean !== term) {
        const dist = levenshteinDistance(clean, term)
        if (dist > 0 && dist <= meta.maxDistance && dist <= Math.floor(term.length / 2)) {
          corrections.push({ original: segment, corrected: term })
          isTypoCorrected = true
          return term
        }
      }
    }

    return segment
  })

  const correctedQuery = processedTokens.join('')
  return { correctedQuery, isTypoCorrected, corrections }
}

// ── COLLOQUIAL PARSER (INDONESIAN & ENGLISH) ────────────────────────────────

export type UserIntent =
  | 'ANTI_SPOILER_REQUEST'
  | 'WHY_STATUS_403'
  | 'WHY_STATUS_500'
  | 'STUCK_IN_CHALLENGE'
  | 'HOW_TO_EXPLOIT'
  | 'PARAMETER_RECON'
  | 'CONCEPTUAL_INQUIRY'
  | 'GENERAL_INVESTIGATION'

export interface ParsedColloquialIntent {
  intent: UserIntent
  primaryTopic: string
  confidenceScore: number
  cleanedTokens: string[]
  isColloquialIndonesian: boolean
}

/**
 * Parses colloquial Indonesian and English syntax, identifying student intent and topic.
 */
export function parseColloquialQuery(query: string, labId: string): ParsedColloquialIntent {
  const lower = (query || '').toLowerCase().trim()

  // 1. Detect Indonesian colloquial flags
  const indoColloquialKeywords = [
    'gw',
    'gue',
    'gua',
    'aku',
    'saya',
    'buntu',
    'mentok',
    'bingung',
    'pusing',
    'padahal',
    'bener',
    'kok',
    'kenapa',
    'knp',
    'gimana',
    'gmna',
    'cara',
    'caranya',
    'gabisa',
    'ga bisa',
    'gak bisa',
    'gagal',
    'nembus',
    'tembus',
    'pas',
    'udah',
    'udh',
    'dah',
  ]
  const isColloquialIndonesian = indoColloquialKeywords.some((w) =>
    new RegExp(`\\b${w}\\b`, 'i').test(lower),
  )

  // 2. Intent matching with high priority
  // Anti-Spoiler Guardrail
  if (
    /(apa flag|flagnya apa|bocoran flag|minta flag|give me flag|jawaban|kunci jawaban|spoiler|solusi instan|flag langsung|kasih tau flag)/i.test(
      lower,
    )
  ) {
    return {
      intent: 'ANTI_SPOILER_REQUEST',
      primaryTopic: 'anti_spoiler_guardrail',
      confidenceScore: 0.98,
      cleanedTokens: ['anti_spoiler', 'flag', 'guardrail'],
      isColloquialIndonesian,
    }
  }

  // Why 403 / Forbidden
  if (
    /(403|forbidden|akses ditolak|terlarang|padahal bener|kenapa 403|why 403)/i.test(
      lower,
    )
  ) {
    return {
      intent: 'WHY_STATUS_403',
      primaryTopic: 'http_403_authorization',
      confidenceScore: 0.94,
      cleanedTokens: ['403', 'forbidden', 'authorization', 'trust_boundary'],
      isColloquialIndonesian,
    }
  }

  // Why 500 / Crash / Syntax Error
  if (/(500|server error|internal error|crash|syntax error|unrecognized token)/i.test(lower)) {
    return {
      intent: 'WHY_STATUS_500',
      primaryTopic: 'database_syntax_error',
      confidenceScore: 0.91,
      cleanedTokens: ['500', 'syntax_error', 'database', 'crash'],
      isColloquialIndonesian,
    }
  }

  // Stuck / Mentok / Buntu / Help
  if (
    /(stuck|mentok|buntu|bingung|pusing|tolong|help|lost|nggak tau|ga tau|harus gimana|langkah awal)/i.test(
      lower,
    )
  ) {
    return {
      intent: 'STUCK_IN_CHALLENGE',
      primaryTopic: 'stuck_guidance',
      confidenceScore: 0.88,
      cleanedTokens: ['stuck', 'recon_step', 'guidance'],
      isColloquialIndonesian,
    }
  }

  // Specific Attack Vector & Payload Inquiries
  if (/(idor|bola|horizontal|vertical|ganti id|manipulasi id|doc_id)/i.test(lower)) {
    return {
      intent: 'HOW_TO_EXPLOIT',
      primaryTopic: 'idor_access_control',
      confidenceScore: 0.95,
      cleanedTokens: ['idor', 'authorization', 'object_reference'],
      isColloquialIndonesian,
    }
  }

  if (/(sqli|sql injection|union select|query|database|where 1=1)/i.test(lower)) {
    return {
      intent: 'HOW_TO_EXPLOIT',
      primaryTopic: 'sql_injection',
      confidenceScore: 0.96,
      cleanedTokens: ['sqli', 'union', 'database'],
      isColloquialIndonesian,
    }
  }

  if (/(ssrf|169\.254|metadata|internal network|webhook|fetch url)/i.test(lower)) {
    return {
      intent: 'HOW_TO_EXPLOIT',
      primaryTopic: 'ssrf_tactics',
      confidenceScore: 0.95,
      cleanedTokens: ['ssrf', 'metadata', 'loopback'],
      isColloquialIndonesian,
    }
  }

  if (/(csrf|xsrf|cross site request|samesite|ambient)/i.test(lower)) {
    return {
      intent: 'HOW_TO_EXPLOIT',
      primaryTopic: 'csrf_defense',
      confidenceScore: 0.94,
      cleanedTokens: ['csrf', 'ambient_cookie', 'samesite'],
      isColloquialIndonesian,
    }
  }

  if (/(upload|mime|webshell|extension|polyglot)/i.test(lower)) {
    return {
      intent: 'HOW_TO_EXPLOIT',
      primaryTopic: 'arbitrary_file_upload',
      confidenceScore: 0.93,
      cleanedTokens: ['upload', 'webshell', 'mime'],
      isColloquialIndonesian,
    }
  }

  if (/(traversal|lfi|\.\.\/|passwd|path traversal)/i.test(lower)) {
    return {
      intent: 'HOW_TO_EXPLOIT',
      primaryTopic: 'path_traversal',
      confidenceScore: 0.94,
      cleanedTokens: ['traversal', 'dot_dot_slash', 'lfi'],
      isColloquialIndonesian,
    }
  }

  if (/(token|jwt|session|bearer|cookie|alg none)/i.test(lower)) {
    return {
      intent: 'HOW_TO_EXPLOIT',
      primaryTopic: 'token_and_session',
      confidenceScore: 0.92,
      cleanedTokens: ['jwt', 'token', 'session'],
      isColloquialIndonesian,
    }
  }

  if (/(parameter|param|endpoint|dimana|di mana|input apa)/i.test(lower)) {
    return {
      intent: 'PARAMETER_RECON',
      primaryTopic: 'parameter_reconnaissance',
      confidenceScore: 0.86,
      cleanedTokens: ['parameter', 'attack_surface', 'endpoint'],
      isColloquialIndonesian,
    }
  }

  // Default Fallback
  return {
    intent: 'GENERAL_INVESTIGATION',
    primaryTopic: labId.toLowerCase(),
    confidenceScore: 0.80,
    cleanedTokens: ['general_investigation', labId.toLowerCase()],
    isColloquialIndonesian,
  }
}

// ── SOCRATIC AI OVERVIEW SYNTHESIS ENGINE ───────────────────────────────────

/**
 * Authoritative AI Overview generator for Zitera Lab.
 * Returns deterministic, high-quality, socratic AI overview cards tailored to the active lab and tier.
 */
export function generateAiOverview(
  query: string,
  labId: string,
  _currentTier: 'free' | 'pro' | 'vip',
): ZiteraAiOverviewData {
  const rawQuery = (query || '').trim()
  const cleanLabId = (labId || 'F01').trim().toUpperCase()
  const curriculum: SocraticLabCurriculum = getSocraticCurriculum(cleanLabId)

  // 1. Run Typo Correction & Normalization
  const typoResult = correctQueryTypos(rawQuery)
  const normalizedQuery = typoResult.correctedQuery || rawQuery

  // 2. Parse Intent & Colloquial Semantics
  const parsed = parseColloquialQuery(normalizedQuery, cleanLabId)

  // 3. Retrieve Clues & Payload Blueprint
  const clue1 = curriculum.clues.find((c) => c.level === 1) || curriculum.clues[0]
  const clue2 = curriculum.clues.find((c) => c.level === 2) || curriculum.clues[1]
  const clue3 = curriculum.clues.find((c) => c.level === 3) || curriculum.clues[2]
  const blueprintData = getLabPayloadBlueprint(cleanLabId)

  // 4. Synthesize Diagnostic Snapshot
  let diagnosticSnapshot = ''
  let reconHypothesis = ''
  let socraticQuestion = ''
  let suggestedActions: string[] = []

  // Case A: Anti-Spoiler Request
  if (parsed.intent === 'ANTI_SPOILER_REQUEST') {
    diagnosticSnapshot =
      `[PROTOKOL INTEGRITAS AKTIF] Permintaan flag langsung terdeteksi pada ${curriculum.labTitle}. Sesuai etika investigasi siber ZITERA, flag pembuktian tidak dibocorkan secara mentah melainkan dibangkitkan oleh backend saat rantai eksploitasi tereksekusi sempurna.`
    reconHypothesis =
      `Sistem laboratorium mengikat token CTF pada sesi aktif container Anda. Anda tidak perlu menebak token secara acak; cukup picu kegagalan logika kontrol akses atau injeksi untuk membuat aplikasi mencetak token secara natural.`
    socraticQuestion =
      `Di mana titik temu antara input klien yang Anda kendalikan dengan logika verifikasi backend pada ${cleanLabId}? Apakah Anda sudah memetakan parameter request di Burp Suite?`
    suggestedActions = [
      'Buka DevTools Network atau Burp HTTP History untuk mencatat semua parameter request aktif.',
      'Bandingkan respons normal dengan respons saat parameter diberi input tak terduga.',
      'Gunakan petunjuk observasi Level 1 di atas untuk mempersempit permukaan serang.',
    ]

    return {
      diagnosticSnapshot,
      reconHypothesis,
      socraticQuestion,
      suggestedActions,
      payloadBlueprint: blueprintData,
      isTypoCorrected: typoResult.isTypoCorrected,
      correctedQuery: typoResult.isTypoCorrected ? typoResult.correctedQuery : undefined,
      confidenceScore: parsed.confidenceScore,
      matchedTopic: parsed.primaryTopic,
    }
  }

  // Case B: Status Code 403 Forbidden / Broken Access
  if (parsed.intent === 'WHY_STATUS_403') {
    diagnosticSnapshot =
      `[DIAGNOSTIK KONTROL AKSES 403] Anda menerima status 403 Forbidden pada ${curriculum.labTitle}. Status ini mengindikasikan server mengenali identitas autentikasi Anda, namun aturan gateway atau firewall menolak wewenang akses Anda pada metode/header saat ini.`
    reconHypothesis =
      `Arsitektur backend kemungkinan mengandalkan header konteks klien (misal: X-Forwarded-For, X-Requested-With, atau HTTP Verb) untuk memisahkan pengguna reguler dan auditor. Jika Anda mengubah HTTP Verb (misal GET ke POST) atau menambahkan header konteks internal, filter gateway dapat dilewati.`
    socraticQuestion =
      `Apakah respons tetap 403 jika Anda mengirim request dengan HTTP Verb yang berbeda atau memanipulasi parameter referensi objek langsung di Burp Repeater?`
    suggestedActions = [
      'Kirim request ke Burp Suite Repeater (Ctrl+R).',
      'Uji HTTP Verb Tampering: ubah method GET menjadi POST atau PUT pada endpoint terkait.',
      'Periksa apakah cookie sesi atau token Authorization Anda masih valid dan tidak expired.',
      'Sisipkan header verifikasi konteks jaringan seperti X-Forwarded-For: 127.0.0.1.',
    ]

    return {
      diagnosticSnapshot,
      reconHypothesis,
      socraticQuestion,
      suggestedActions,
      payloadBlueprint: blueprintData,
      isTypoCorrected: typoResult.isTypoCorrected,
      correctedQuery: typoResult.isTypoCorrected ? typoResult.correctedQuery : undefined,
      confidenceScore: parsed.confidenceScore,
      matchedTopic: parsed.primaryTopic,
    }
  }

  // Case C: Status Code 500 / Database Syntax Crash
  if (parsed.intent === 'WHY_STATUS_500') {
    diagnosticSnapshot =
      `[ANOMALI SISTEM 500 TERDETEKSI] Respons HTTP 500 Internal Server Error menandakan input Anda berhasil memutus alur normal parser backend pada ${curriculum.labTitle}. Ini adalah sinyal kuat adanya kelemahan sintaks atau unhandled exception.`
    reconHypothesis =
      `Backend kemungkinan menggunakan konkatenasi string dinamis tanpa prepared statement atau validasi tipe data. Karakter metakarakter yang Anda kirimkan membuat parser database/interpreter mengalami kegagalan eksekusi kueri.`
    socraticQuestion =
      `Karakter pemutus apa yang secara spesifik memicu error 500 tersebut? Dapatkah Anda menyusun payload penyeimbang (seperti komentar SQL -- atau penutup tanda petik) agar respons kembali normal 200 OK?`
    suggestedActions = [
      'Catat karakter metakarakter terakhir yang memicu respon 500.',
      'Uji penutup sintaks dengan tanda komentar (misal: \' -- atau \' /*) untuk menetralkan query.',
      'Periksa apakah pesan kesalahan di body respons membeberkan nama database atau rute file internal.',
    ]

    return {
      diagnosticSnapshot,
      reconHypothesis,
      socraticQuestion,
      suggestedActions,
      payloadBlueprint: blueprintData,
      isTypoCorrected: typoResult.isTypoCorrected,
      correctedQuery: typoResult.isTypoCorrected ? typoResult.correctedQuery : undefined,
      confidenceScore: parsed.confidenceScore,
      matchedTopic: parsed.primaryTopic,
    }
  }

  // Case D: Stuck / Guidance
  if (parsed.intent === 'STUCK_IN_CHALLENGE') {
    diagnosticSnapshot =
      `[STATUS INVESTIGASI: KEBUNTUAN TAKTIS] Misi pada ${curriculum.labTitle} berfokus pada: ${curriculum.missionOverview}`
    reconHypothesis =
      `Kebuntuan umumnya terjadi karena kita mencoba menebak payload sebelum memahami perilaku normal aplikasi. Titik kerentanan lab ini berakar pada: ${clue2.summary}`
    socraticQuestion =
      clue1.investigativeQuestions[0] ||
      `Parameter apa yang pertama kali berinteraksi dengan server saat Anda memuat halaman lab ini?`
    suggestedActions = [
      clue1.tacticalAction || 'Buka tab Network di DevTools untuk mengamati raw request.',
      'Fokuskan perhatian pada parameter identifikasi atau input teks yang dapat Anda kontrol.',
      'Gunakan Burp Suite Repeater untuk menguji modifikasi parameter secara bertahap.',
    ]

    return {
      diagnosticSnapshot,
      reconHypothesis,
      socraticQuestion,
      suggestedActions,
      payloadBlueprint: blueprintData,
      isTypoCorrected: typoResult.isTypoCorrected,
      correctedQuery: typoResult.isTypoCorrected ? typoResult.correctedQuery : undefined,
      confidenceScore: parsed.confidenceScore,
      matchedTopic: parsed.primaryTopic,
    }
  }

  // Case E: Specific Attack / Lab Synthesis
  diagnosticSnapshot =
    `[ANALISIS TAKTIS AI OVERVIEW] Menganalisis skenario ${curriculum.labTitle}. Fokus peranan Anda adalah ${curriculum.tacticalRole}.`
  reconHypothesis =
    `${clue2.guidance.slice(0, 240)}... Celah terjadi karena batas kepercayaan (trust boundary) di sisi server mempercayai masukan klien tanpa validasi independen.`
  socraticQuestion =
    clue2.investigativeQuestions[0] ||
    `Bagaimana backend membedakan apakah request yang tiba adalah aksi sah pengguna atau manipulasi paket buatan?`
  suggestedActions = [
    clue1.tacticalAction || 'Amati anatomi request di Burp Suite Proxy.',
    clue2.tacticalAction || 'Lakukan pengujian perbandingan nilai parameter di Burp Repeater.',
    clue3.tacticalAction || 'Kirim request dengan format blueprint taktis untuk memicu respon bukti.',
  ]

  return {
    diagnosticSnapshot,
    reconHypothesis,
    socraticQuestion,
    suggestedActions,
    payloadBlueprint: blueprintData,
    isTypoCorrected: typoResult.isTypoCorrected,
    correctedQuery: typoResult.isTypoCorrected ? typoResult.correctedQuery : undefined,
    confidenceScore: Math.min(0.97, Math.max(0.78, parsed.confidenceScore)),
    matchedTopic: parsed.primaryTopic,
  }
}
