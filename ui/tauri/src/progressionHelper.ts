// Progression Gating & Multi-Lab Architecture Helper for Zitera Lab
// Supports all 30 laboratories (10 Lab Wajib: A01..A10 + 20 Lab Eksplorasi)

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
}

export function isLabSolved(labId: string): boolean {
  try {
    return localStorage.getItem(`zitera_solved_${labId}`) === 'true'
  } catch {
    return false
  }
}

export function getLabProgression(labId: string): LabProgressionInfo {
  const parts = labId.split('-')
  const baseCategory = parts[0].toUpperCase()
  const suffix = parts.length > 1 ? parts[1] : null
  const categoryName = CATEGORY_NAMES[baseCategory] || 'Security Category'

  // 1. Lab Wajib (Core Foundation) e.g. A01, A02, etc.
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

  // 2. Lab Explore 1 (e.g. A01-02)
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

  // 3. Lab Explore 2 (e.g. A01-03)
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
    localStorage.setItem(`zitera_solved_${labId}`, 'true')
    // Dispatch custom event to notify all components of progression change
    window.dispatchEvent(new CustomEvent('zitera_progression_solved', { detail: { labId } }))
  } catch {
    // Ignore storage errors in restricted sandboxes
  }
}
