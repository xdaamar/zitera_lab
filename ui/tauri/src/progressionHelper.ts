// Progression Gating & Multi-Lab Architecture Helper for Zitera Lab
// Supports all 45 laboratories (Track 1: Foundations, Track 2: OWASP Top 10, Track 3: Specialist)

export type UserTier = 'foundation' | 'pro' | 'enterprise' | 'trial'

export interface TierAccessResult {
  allowed: boolean
  isTierLocked: boolean
  requiredTier: 'foundation' | 'pro'
  track: 'TRACK_1' | 'TRACK_2' | 'TRACK_3'
  trackName: string
  reason?: string
}

export interface LabProgressionInfo {
  isCore: boolean
  isExplore: boolean
  tier: 'core' | 'explore-1' | 'explore-2'
  tierLabel: string
  categoryCode: string
  categoryName: string
  prerequisiteId: string | null
  prerequisiteTitle: string | null
  prerequisiteSolved: boolean
  nextLabId: string | null
  nextLabTitle: string | null
}

export const CATEGORY_NAMES: Record<string, string> = {
  // Track 1
  FOUNDATIONS: 'Web Security Foundations',

  // Track 2 (OWASP Top 10)
  A01: 'Broken Access Control',
  A02: 'Security Misconfiguration',
  A03: 'Supply Chain Failures',
  A04: 'Cryptographic Failures',
  A05: 'Injection',
  A06: 'Insecure Design',
  A07: 'Authentication Failures',
  A08: 'Software & Data Integrity',
  A09: 'Security Logging & SIEM',
  A10: 'Server-Side Request Forgery',

  // Track 3
  WEB_EXPLOIT: 'Web Security Specialist',
  DEFENSE: 'Threat Hunting & Defense',
}

export const LAB_CATALOG_TITLES: Record<string, string> = {
  // A01 (Broken Access Control)
  A01: 'Broken Access Control (IDOR)',
  'A01-02': 'Vertical Privilege Escalation & Header Role Manipulation',
  'A01-03': 'Multi-Tenant Boundary Escape & Insecure Direct Object Deletion',

  // A02 (Security Misconfiguration)
  A02: 'Security Misconfiguration',
  'A02-02': 'Insecure CORS Misconfiguration & Origin Reflection',
  'A02-03': 'Exposed Git Directory & Object Carving Extraction',

  // A03 (Supply Chain Failures)
  A03: 'Supply Chain Failures',
  'A03-02': 'Dependency Confusion & Internal Namespace Hijacking',
  'A03-03': 'Malicious Preinstall Script & CI/CD Secret Exfiltration',

  // A04 (Cryptographic Failures)
  A04: 'Cryptographic Failures',
  'A04-02': 'Predictable Password Reset Token via Insecure PRNG',
  'A04-03': 'JWT Algorithm None & HMAC Weak Key Cracking',

  // A05 (Injection)
  A05: 'Injection (SQLi)',
  'A05-02': 'Blind Boolean & Time-Based SQL Injection',
  'A05-03': 'OS Command Injection via Unsanitized File Utilities',

  // A06 (Insecure Design)
  A06: 'Insecure Design',
  'A06-02': 'Concurrency Race Condition & Double Spending',
  'A06-03': 'Multi-Step Workflow Step Skipping',

  // A07 (Authentication Failures)
  A07: 'Authentication Failures',
  'A07-02': 'OAuth 2.0 Account Linking Hijacking via Missing State',
  'A07-03': 'MFA Bypass via Direct Endpoint Traversal',

  // A08 (Software & Data Integrity)
  A08: 'Software & Data Integrity',
  'A08-02': 'Unsigned Client-Side State Tampering in LocalStorage',
  'A08-03': 'Python Pickle & YAML Unsafe Object Deserialization',

  // A09 (Security Logging & SIEM)
  A09: 'Security Logging & Monitoring Failures',
  'A09-02': 'CRLF Log Injection & Log Forgery',
  'A09-03': 'Blind Log Deletion & Audit Trail Evasion',

  // A10 (Server-Side Request Forgery)
  A10: 'Server-Side Request Forgery (SSRF)',
  'A10-02': 'Blind Out-of-Band SSRF via Webhook Callback',
  'A10-03': 'Server-Side Request Forgery via Internal DNS Rebinding',

  // Track 1 (Foundations)
  F01: 'Dasar Protokol HTTP & Web Architecture',
  F02: 'Cookies, Tokens & Session Security',
  F03: 'Authentication vs Authorization & Trust Boundaries',
  F04: 'Kriptografi Esensial: Encoding vs Hashing vs Enkripsi',
  F05: 'Dasar Query Database & Pemisahan Data/Perintah',
  F06: 'Mekanisme Lab ZITERA & Pengiriman Flag CTF',

  // Track 3 (Specialist & Defense)
  W01: 'Cross-Site Request Forgery (CSRF) & Token Defense',
  W02: 'Server-Side Request Forgery (SSRF) & Metadata Service',
  W03: 'Broken Object Level Authorization (API BOLA)',
  W04: 'Arbitrary File Upload & MIME Sniffing Bypass',
  W05: 'Path Traversal & Local File Inclusion (LFI)',
  D01: 'Web Server Log Analysis & Threat Detection',
  D02: 'Binary & Package Cryptographic Integrity',
  A11: 'API Mass Assignment & Privilege Escalation',
  A12: 'OAuth 2.0 & Account Linking Takeover',
}

/**
 * Authoritative Tier Gating Enforcement
 * - Track 1 (F01..F06): Foundation, Trial, Pro, Enterprise (All users)
 * - Track 2 (A01): Flagship Starter Lab (VaultPay Enterprise IDOR) allowed for Foundation & Trial users
 * - Track 2 (A01-02, A01-03, A02..A10 + explores): Strictly locked (requiredTier: 'pro')
 * - Track 3 (W01..W05, D01..D02, A11, A12): Strictly locked (requiredTier: 'pro')
 */
export function canUserAccessLab(labId: string, userTier?: string | null): TierAccessResult {
  const cleanId = (labId || '').trim().toUpperCase()
  const effectiveTier = (userTier || 'foundation').trim().toLowerCase()
  const isProOrAbove = effectiveTier === 'pro' || effectiveTier === 'enterprise'

  // Track 1: Web Security Foundations (F01..F06)
  if (cleanId.startsWith('F')) {
    return {
      allowed: true,
      isTierLocked: false,
      requiredTier: 'foundation',
      track: 'TRACK_1',
      trackName: 'Track 1: Web Security Foundations',
    }
  }

  // A01 (VaultPay Enterprise IDOR): Flagship trial lab open to foundation & trial tier
  if (cleanId === 'A01') {
    return {
      allowed: true,
      isTierLocked: false,
      requiredTier: 'foundation',
      track: 'TRACK_2',
      trackName: 'Track 2: OWASP Top 10 Enterprise',
    }
  }

  // Track 3: Web Security Specialist & Defense (W01..W05, D01..D02, A11, A12)
  if (
    cleanId.startsWith('W') ||
    cleanId.startsWith('D') ||
    cleanId === 'A11' ||
    cleanId === 'A12'
  ) {
    return {
      allowed: isProOrAbove,
      isTierLocked: !isProOrAbove,
      requiredTier: 'pro',
      track: 'TRACK_3',
      trackName: 'Track 3: Web Security Specialist',
      reason: isProOrAbove
        ? undefined
        : 'Laboratorium ini merupakan bagian dari Track 3 (Web Security Specialist & Defense) dan eksklusif untuk peserta Pro Edition. Upgrade ke Pro Edition untuk membuka seluruh laboratorium spesialis ini.',
    }
  }

  // Track 2: OWASP Top 10 Enterprise (A01-02, A01-03, A02..A10 + explores)
  const isA01Explore = cleanId === 'A01-02' || cleanId === 'A01-03'
  return {
    allowed: isProOrAbove,
    isTierLocked: !isProOrAbove,
    requiredTier: 'pro',
    track: 'TRACK_2',
    trackName: 'Track 2: OWASP Top 10 Enterprise',
    reason: isProOrAbove
      ? undefined
      : isA01Explore
        ? `Laboratorium lanjutan ${cleanId} eksklusif untuk peserta Pro Edition. Lab dasar A01 (VaultPay Enterprise IDOR) terbuka untuk dicoba pada tier Foundation/Trial, namun skenario lanjutan A01-02 & A01-03 memerlukan lisensi Pro Edition.`
        : 'Laboratorium ini merupakan bagian dari Track 2 (OWASP Top 10 Enterprise) dan eksklusif untuk peserta Pro Edition. Upgrade ke Pro Edition untuk mengakses seluruh 30 laboratorium OWASP Top 10.',
  }
}

export function isLabSolved(labId: string): boolean {
  try {
    const cleanId = (labId || '').trim().toUpperCase()
    return (
      localStorage.getItem(`zitera_solved_${cleanId}`) === 'true' ||
      localStorage.getItem(`zitera_solved_${labId}`) === 'true'
    )
  } catch {
    return false
  }
}

export function getLabProgression(labId: string): LabProgressionInfo {
  const cleanId = (labId || '').trim().toUpperCase()
  const parts = cleanId.split('-')
  const baseCategory = parts[0]
  const suffix = parts.length > 1 ? parts[1] : null
  const categoryName = CATEGORY_NAMES[baseCategory] || 'Security Category'

  // 1. Track 1: Foundations (F01..F06)
  if (baseCategory.startsWith('F')) {
    const num = parseInt(baseCategory.substring(1), 10)
    const nextLabId = num < 6 ? `F0${num + 1}` : null
    return {
      isCore: true,
      isExplore: false,
      tier: 'core',
      tierLabel: 'Web Security Foundations',
      categoryCode: 'FOUNDATIONS',
      categoryName: CATEGORY_NAMES['FOUNDATIONS'] || 'Web Security Foundations',
      prerequisiteId: null,
      prerequisiteTitle: null,
      prerequisiteSolved: true,
      nextLabId,
      nextLabTitle: nextLabId ? LAB_CATALOG_TITLES[nextLabId] || `Lab ${nextLabId}` : null,
    }
  }

  // 2. Track 3: Specialist & Defense (W01..W05, D01..D02, A11, A12)
  if (
    baseCategory.startsWith('W') ||
    baseCategory.startsWith('D') ||
    baseCategory === 'A11' ||
    baseCategory === 'A12'
  ) {
    const isDefense = baseCategory.startsWith('D')
    const catCode = isDefense ? 'DEFENSE' : 'WEB_EXPLOIT'
    return {
      isCore: true,
      isExplore: false,
      tier: 'core',
      tierLabel: isDefense ? 'Threat Hunting & Defense' : 'Web Security Specialist',
      categoryCode: catCode,
      categoryName: CATEGORY_NAMES[catCode] || 'Specialist Lab',
      prerequisiteId: null,
      prerequisiteTitle: null,
      prerequisiteSolved: true,
      nextLabId: null,
      nextLabTitle: null,
    }
  }

  // 3. Track 2: OWASP Top 10 Enterprise (A01..A10)
  // 3.1 Lab Wajib (Core Foundation) e.g. A01, A02, etc.
  if (!suffix) {
    const nextLabId = `${baseCategory}-02`
    return {
      isCore: true,
      isExplore: false,
      tier: 'core',
      tierLabel: 'Lab Wajib (Pondasi Dasar)',
      categoryCode: baseCategory,
      categoryName,
      prerequisiteId: null,
      prerequisiteTitle: null,
      prerequisiteSolved: true,
      nextLabId,
      nextLabTitle: LAB_CATALOG_TITLES[nextLabId] || `Lab Eksplorasi ${nextLabId}`,
    }
  }

  // 3.2 Lab Explore 1 (e.g. A01-02)
  if (suffix === '02') {
    const prereqId = baseCategory
    const nextLabId = `${baseCategory}-03`
    return {
      isCore: false,
      isExplore: true,
      tier: 'explore-1',
      tierLabel: 'Lab Eksplorasi (Teknik Lanjutan A)',
      categoryCode: baseCategory,
      categoryName,
      prerequisiteId: prereqId,
      prerequisiteTitle: LAB_CATALOG_TITLES[prereqId] || `Lab Wajib ${prereqId}`,
      prerequisiteSolved: isLabSolved(prereqId),
      nextLabId,
      nextLabTitle: LAB_CATALOG_TITLES[nextLabId] || `Lab Eksplorasi ${nextLabId}`,
    }
  }

  // 3.3 Lab Explore 2 (e.g. A01-03)
  const prereqId = `${baseCategory}-02`
  return {
    isCore: false,
    isExplore: true,
    tier: 'explore-2',
    tierLabel: 'Lab Eksplorasi (Teknik Lanjutan B)',
    categoryCode: baseCategory,
    categoryName,
    prerequisiteId: prereqId,
    prerequisiteTitle: LAB_CATALOG_TITLES[prereqId] || LAB_CATALOG_TITLES[baseCategory] || `Lab ${prereqId}`,
    prerequisiteSolved: isLabSolved(prereqId),
    nextLabId: null,
    nextLabTitle: null,
  }
}

export function recordLabSolveProgression(labId: string): void {
  try {
    const cleanId = (labId || '').trim().toUpperCase()
    localStorage.setItem(`zitera_solved_${cleanId}`, 'true')
    localStorage.setItem(`zitera_solved_${labId}`, 'true')
    // Dispatch custom event to notify all components of progression change
    window.dispatchEvent(new CustomEvent('zitera_progression_solved', { detail: { labId: cleanId } }))
  } catch {
    // Ignore storage errors in restricted sandboxes
  }
}
