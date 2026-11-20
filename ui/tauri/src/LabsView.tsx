import { useState, useEffect, useRef, useMemo } from 'react'
import {
  Search,
  X,
  Play,
  Square,
  RotateCcw,
  ExternalLink,
  DownloadCloud,
  ChevronRight,
  RefreshCw,
  Wifi,
  Sparkles,
  ShieldCheck,
  Zap,
  Trophy,
  Layers,
  Activity,
  CheckCircle2,
  Filter,
  Lock,
  ShieldAlert,
} from 'lucide-react'
import {
  getLabs,
  startLab,
  stopLab,
  resetLab,
  updateLab,
  openBrowser,
  type LabItem,
} from './engineClient'
import { CuteAnimeLoading } from './CuteAnimeLoading'
import {
  getLabProgression,
  isLabSolved,
  type LabProgressionInfo,
} from './progressionHelper'

interface LabsViewProps {
  onSelectLab: (labId: string) => void
}

type StatusFilter = 'ALL' | 'RUNNING' | 'SOLVED' | 'LOCKED' | 'READY' | 'UPDATE'
type DifficultyFilter = 'ALL' | 'Beginner' | 'Intermediate' | 'Advanced'

const STICKER_MAP: Record<string, string> = {
  A01: '/assets/sticker_pink.png',
  'A01-02': '/assets/sticker_duck.png',
  'A01-03': '/assets/sticker_bunny.png',
  A02: '/assets/sticker_duck.png',
  'A02-02': '/assets/sticker_cat_laptop.png',
  'A02-03': '/assets/sticker_hamster.png',
  A03: '/assets/sticker_bunny.png',
  'A03-02': '/assets/sticker_shiba.png',
  'A03-03': '/assets/sticker_blonde.png',
  A04: '/assets/sticker_cat_laptop.png',
  'A04-02': '/assets/sticker_purple.png',
  'A04-03': '/assets/sticker_pink.png',
  A05: '/assets/sticker_hamster.png',
  'A05-02': '/assets/sticker_duck.png',
  'A05-03': '/assets/sticker_bunny.png',
  A06: '/assets/sticker_shiba.png',
  'A06-02': '/assets/sticker_cat_laptop.png',
  'A06-03': '/assets/sticker_hamster.png',
  A07: '/assets/sticker_blonde.png',
  'A07-02': '/assets/sticker_shiba.png',
  'A07-03': '/assets/sticker_blonde.png',
  A08: '/assets/sticker_purple.png',
  'A08-02': '/assets/sticker_purple.png',
  'A08-03': '/assets/sticker_pink.png',
  A09: '/assets/sticker_pink.png',
  'A09-02': '/assets/sticker_duck.png',
  'A09-03': '/assets/sticker_bunny.png',
  A10: '/assets/sticker_duck.png',
  'A10-02': '/assets/sticker_cat_laptop.png',
  'A10-03': '/assets/sticker_hamster.png',
}

interface LabMetadata {
  targetApp: string
  domain: string
  desc: string
  skills: string[]
  difficulty: 'Beginner' | 'Intermediate' | 'Advanced'
}

const LAB_DETAILS: Record<string, LabMetadata> = {
  // A01
  A01: {
    targetApp: 'VaultPay Enterprise',
    domain: 'Fintech B2B Treasury & Wire Transfer Portal',
    desc: 'Investigate Insecure Direct Object References (IDOR) and bypass horizontal authorization controls in a mock banking API.',
    skills: ['IDOR', 'BOLA', 'Access Control', 'API Security'],
    difficulty: 'Beginner',
  },
  'A01-02': {
    targetApp: 'OpsAdmin Portal',
    domain: 'Enterprise Internal Admin Console',
    desc: 'Audit vertical privilege escalation vulnerabilities and manipulate custom HTTP role headers to access restricted admin operations.',
    skills: ['Privilege Escalation', 'Header Manipulation', 'Role Verification'],
    difficulty: 'Intermediate',
  },
  'A01-03': {
    targetApp: 'TenantCloud Workspaces',
    domain: 'B2B Multi-Tenant Cloud Workspace',
    desc: 'Exploit cross-tenant isolation gaps to delete resources and manipulate workspace scopes across isolated corporate tenants.',
    skills: ['Multi-Tenancy', 'Boundary Escape', 'IDOR Deletion'],
    difficulty: 'Advanced',
  },

  // A02
  A02: {
    targetApp: 'CloudCore OpsGateway',
    domain: 'Enterprise Cloud Management & Microservices Monitor',
    desc: 'Exploit verbose stack traces, default administrative credentials, and exposed diagnostics endpoints in OpsGateway.',
    skills: ['Misconfiguration', 'Directory Listing', 'Debug Leaks', 'CORS'],
    difficulty: 'Beginner',
  },
  'A02-02': {
    targetApp: 'APINexus Gateway',
    domain: 'Microservices API Gateway & Public Ingress',
    desc: 'Exploit wildcard origin reflections and null origin trusts in CORS headers to exfiltrate private API keys.',
    skills: ['CORS Misconfiguration', 'Origin Reflection', 'Cross-Origin Exfiltration'],
    difficulty: 'Intermediate',
  },
  'A02-03': {
    targetApp: 'DevBucket Storage',
    domain: 'Cloud DevOps Artifact Storage & Git Repositories',
    desc: 'Reconstruct commit history, tree blobs, and hardcoded production secrets from an exposed .git directory.',
    skills: ['Git Object Carving', 'Information Disclosure', 'Source Code Recovery'],
    difficulty: 'Advanced',
  },

  // A03
  A03: {
    targetApp: 'PackNexus Supply Hub',
    domain: 'B2B Package Registry & Build Pipeline Hub',
    desc: 'Analyze compromised upstream dependencies, poisoned package manifests, and insecure package resolution pipelines.',
    skills: ['Supply Chain', 'Dependency Confusion', 'Integrity Checking'],
    difficulty: 'Beginner',
  },
  'A03-02': {
    targetApp: 'Enterprise Artifactory',
    domain: 'Private Package & Container Repository',
    desc: 'Hijack unreserved internal organization package namespaces in public registries to trigger automated pipeline execution.',
    skills: ['Dependency Confusion', 'Namespace Hijacking', 'Package Registry Audit'],
    difficulty: 'Intermediate',
  },
  'A03-03': {
    targetApp: 'NodeBuild Engine',
    domain: 'Automated Build & CI/CD Compilation Pipeline',
    desc: 'Analyze rogue preinstall lifecycle hooks exfiltrating environment variables and credentials during automated builds.',
    skills: ['Lifecycle Hook Exploitation', 'Build Pipeline Security', 'Secret Exfiltration'],
    difficulty: 'Advanced',
  },

  // A04
  A04: {
    targetApp: 'SafeKey Corporate Vault',
    domain: 'Banking Encryption & Credential Vault',
    desc: 'Exploit unsanitized user input in cryptographic authentication hashes and race conditions in financial ledger updates.',
    skills: ['Crypto Flaws', 'Timing Attacks', 'Insecure Hashes'],
    difficulty: 'Intermediate',
  },
  'A04-02': {
    targetApp: 'AuthKey Recovery',
    domain: 'Corporate Password Recovery & Token Service',
    desc: 'Reverse predictable pseudo-random number generator seeds to forge valid password reset tokens for high-privileged users.',
    skills: ['Insecure PRNG', 'Token Prediction', 'Seed Reconstruction'],
    difficulty: 'Intermediate',
  },
  'A04-03': {
    targetApp: 'TokenShield SSO',
    domain: 'Centralized Token Broker & Single Sign-On',
    desc: 'Bypass signature validation using algorithm none injection and crack weak HMAC secrets offline to forge admin tokens.',
    skills: ['JWT Algorithm Confusion', 'None Alg Attack', 'HMAC Key Cracking'],
    difficulty: 'Advanced',
  },

  // A05
  A05: {
    targetApp: 'MegaStore Global',
    domain: 'Commercial E-Commerce & Retail Inventory Portal',
    desc: 'Perform classic SQL injection, boolean-based inference, and command injection across legacy backend query handlers.',
    skills: ['SQL Injection', 'Blind SQLi', 'Input Sanitization'],
    difficulty: 'Intermediate',
  },
  'A05-02': {
    targetApp: 'CustPortal Database',
    domain: 'Customer Relationship & CRM Data Engine',
    desc: 'Extract database schemas and records character-by-character using boolean inference and database sleep functions.',
    skills: ['Blind SQLi', 'Time-Based Inference', 'Query Timing Analysis'],
    difficulty: 'Intermediate',
  },
  'A05-03': {
    targetApp: 'CloudPing Network Tool',
    domain: 'Cloud Diagnostic & Infrastructure Telemetry Console',
    desc: 'Break out of shell command arguments using command separators and bypass basic character blacklists.',
    skills: ['Command Injection', 'Shell Metacharacters', 'Filter Bypass'],
    difficulty: 'Advanced',
  },

  // A06
  A06: {
    targetApp: 'AuthShield Identity',
    domain: 'Enterprise SSO & Access Management',
    desc: 'Audit vulnerable microservice endpoints lacking integrity checks and exploit insecure deserialization vulnerabilities.',
    skills: ['Deserialization', 'Object Injection', 'Memory Corrupt'],
    difficulty: 'Intermediate',
  },
  'A06-02': {
    targetApp: 'FlashSale Pay',
    domain: 'High-Throughput E-Commerce Flash Sale Engine',
    desc: 'Exploit Time-of-Check to Time-of-Use (TOCTOU) windows to redeem balances and promotional coupons multiple times.',
    skills: ['Race Conditions', 'Concurrency Flaws', 'TOCTOU Exploitation'],
    difficulty: 'Intermediate',
  },
  'A06-03': {
    targetApp: 'KYC Onboarding Portal',
    domain: 'Digital Banking Identity Verification & Compliance',
    desc: 'Bypass mandatory identity verification checkpoints by manipulating client navigation states and direct endpoint calls.',
    skills: ['Business Logic Flaws', 'Step Skipping', 'State Machine Validation'],
    difficulty: 'Advanced',
  },

  // A07
  A07: {
    targetApp: 'NexHealth Portal',
    domain: 'Hospital Electronic Medical Records & Patient Portal',
    desc: 'Bypass broken multi-factor authentication, exploit session fixation, and hijack persistent JWT tokens.',
    skills: ['Session Hijacking', 'JWT Tampering', 'Brute Force Defense'],
    difficulty: 'Intermediate',
  },
  'A07-02': {
    targetApp: 'SocialConnect Auth',
    domain: 'Identity Federation & Social Login Gateway',
    desc: 'Perform CSRF on OAuth callback flows lacking cryptographically secure state parameters to hijack target user profiles.',
    skills: ['OAuth 2.0 Security', 'Missing State Parameter', 'Account Linking CSRF'],
    difficulty: 'Intermediate',
  },
  'A07-03': {
    targetApp: 'TwoFactor SecureNet',
    domain: 'Enterprise Multi-Factor Verification Service',
    desc: 'Circumvent second-factor challenges through direct session state manipulation and incomplete backend step enforcement.',
    skills: ['MFA Bypass', 'Session State Flaws', 'Enforcement Traversal'],
    difficulty: 'Advanced',
  },

  // A08
  A08: {
    targetApp: 'DataFlow Analytics',
    domain: 'Enterprise BI & Data Processing Engine',
    desc: 'Perform Server-Side Request Forgery (SSRF) and audit unsafe deserialization pipelines across internal microservices.',
    skills: ['SSRF', 'Cloud Metadata', 'Internal Port Scan'],
    difficulty: 'Advanced',
  },
  'A08-02': {
    targetApp: 'CartSync Session',
    domain: 'Distributed Shopping Cart & Client State Store',
    desc: 'Tamper with unverified client-side session states, pricing parameters, and role claims stored insecurely in the browser.',
    skills: ['Client State Tampering', 'HMAC Integrity', 'LocalStorage Security'],
    difficulty: 'Intermediate',
  },
  'A08-03': {
    targetApp: 'PipelineSerializer',
    domain: 'Machine Learning Data Pipeline & Serialization Service',
    desc: 'Construct malicious deserialization gadgets using Python __reduce__ and unsafe YAML loaders to gain remote code execution.',
    skills: ['Insecure Deserialization', 'Pickle RCE', 'YAML Safe Loader'],
    difficulty: 'Advanced',
  },

  // A09
  A09: {
    targetApp: 'FleetTrack Logistics',
    domain: 'Commercial Telematics & IoT Fleet Management',
    desc: 'Investigate blind log injection, audit trail evasion, and insufficient security monitoring during anomalous attack traffic.',
    skills: ['Log Injection', 'Audit Evasion', 'Incident Response'],
    difficulty: 'Advanced',
  },
  'A09-02': {
    targetApp: 'AuditSys Central',
    domain: 'Compliance Audit Trail & SIEM Ingestion Service',
    desc: 'Inject carriage return and line feed (CRLF) characters into log inputs to fake successful authentication and blind SIEM parsers.',
    skills: ['Log Injection', 'CRLF Forgery', 'Audit Trail Defense'],
    difficulty: 'Intermediate',
  },
  'A09-03': {
    targetApp: 'StealthDetect Gateway',
    domain: 'Network Security Intrusion Detection System',
    desc: 'Evade rate-based alerting rules and anomaly thresholds by modulating reconnaissance velocity and distributed origin headers.',
    skills: ['SIEM Evasion', 'Threshold Bypass', 'Behavioral Detection'],
    difficulty: 'Advanced',
  },

  // A10
  A10: {
    targetApp: 'TemplateCloud Studio',
    domain: 'Commercial SaaS Email & PDF Document Generator',
    desc: 'Weaponize complex server-side template injection (SSTI) and remote code execution across edge renderers.',
    skills: ['SSTI', 'RCE', 'Sandbox Escape', 'Advanced Exploitation'],
    difficulty: 'Advanced',
  },
  'A10-02': {
    targetApp: 'GatewayGuard Failover',
    domain: 'Edge Microservices Security Perimeter',
    desc: 'Trigger null reference and timeout exceptions in boundary services to force authorization routines into permissive fail-open mode.',
    skills: ['Fail-Open Defaults', 'Exception Auditing', 'Boundary Resilience'],
    difficulty: 'Intermediate',
  },
  'A10-03': {
    targetApp: 'WebFetch Microservice',
    domain: 'Cloud Document & Metadata Importer',
    desc: 'Bypass URL parsing sanitizers and DNS rebinding protections to pivot into cloud instance metadata endpoints (169.254.169.254).',
    skills: ['Blind SSRF', 'Cloud Metadata Pivot', 'DNS Rebinding Defense'],
    difficulty: 'Advanced',
  },
}

interface CategoryStyle {
  id: string
  name: string
  slug: string
  owaspTag: string
  accent: string
  border: string
  shadow: string
  glowRgba: string
  bgLight: string
  cartridgeBg: string
  cartridgeBorder: string
  cartridgeText: string
  cartridgeShadow: string
  badgeBg: string
  badgeBorder: string
  badgeText: string
  badgeShadow: string
}

const CATEGORY_STYLES: Record<string, CategoryStyle> = {
  A01: {
    id: 'A01',
    name: 'Access Control',
    slug: 'access-control',
    owaspTag: 'OWASP A01:2025',
    accent: '#0F766E',
    border: '#2DD4BF',
    shadow: '#0F766E',
    glowRgba: 'rgba(45,212,191,0.22)',
    bgLight: '#F0FDFA',
    cartridgeBg: '#CCFBF1',
    cartridgeBorder: '#2DD4BF',
    cartridgeText: '#0F766E',
    cartridgeShadow: '#5EEAD4',
    badgeBg: '#CCFBF1',
    badgeBorder: '#2DD4BF',
    badgeText: '#0F766E',
    badgeShadow: '#99F6E4',
  },
  A02: {
    id: 'A02',
    name: 'Security Misconfiguration',
    slug: 'security-misconfiguration',
    owaspTag: 'OWASP A02:2025',
    accent: '#0284C7',
    border: '#38BDF8',
    shadow: '#0284C7',
    glowRgba: 'rgba(56,189,248,0.22)',
    bgLight: '#F0F9FF',
    cartridgeBg: '#E0F2FE',
    cartridgeBorder: '#38BDF8',
    cartridgeText: '#0369A1',
    cartridgeShadow: '#7DD3FC',
    badgeBg: '#E0F2FE',
    badgeBorder: '#38BDF8',
    badgeText: '#0369A1',
    badgeShadow: '#BAE6FD',
  },
  A03: {
    id: 'A03',
    name: 'Supply Chain Failures',
    slug: 'supply-chain',
    owaspTag: 'OWASP A03:2025',
    accent: '#6D28D9',
    border: '#A78BFA',
    shadow: '#6D28D9',
    glowRgba: 'rgba(167,139,250,0.22)',
    bgLight: '#F5F3FF',
    cartridgeBg: '#EDE9FE',
    cartridgeBorder: '#A78BFA',
    cartridgeText: '#6D28D9',
    cartridgeShadow: '#C4B5FD',
    badgeBg: '#EDE9FE',
    badgeBorder: '#A78BFA',
    badgeText: '#6D28D9',
    badgeShadow: '#DDD6FE',
  },
  A04: {
    id: 'A04',
    name: 'Cryptographic Failures',
    slug: 'cryptographic-failures',
    owaspTag: 'OWASP A04:2025',
    accent: '#7E22CE',
    border: '#C084FC',
    shadow: '#7E22CE',
    glowRgba: 'rgba(192,132,252,0.22)',
    bgLight: '#FAF5FF',
    cartridgeBg: '#F3E8FF',
    cartridgeBorder: '#C084FC',
    cartridgeText: '#7E22CE',
    cartridgeShadow: '#E9D5FF',
    badgeBg: '#F3E8FF',
    badgeBorder: '#C084FC',
    badgeText: '#7E22CE',
    badgeShadow: '#E9D5FF',
  },
  A05: {
    id: 'A05',
    name: 'Injection',
    slug: 'injection',
    owaspTag: 'OWASP A05:2025',
    accent: '#B45309',
    border: '#FBBF24',
    shadow: '#B45309',
    glowRgba: 'rgba(251,191,36,0.25)',
    bgLight: '#FFFBEB',
    cartridgeBg: '#FEF3C7',
    cartridgeBorder: '#FBBF24',
    cartridgeText: '#B45309',
    cartridgeShadow: '#FDE68A',
    badgeBg: '#FEF3C7',
    badgeBorder: '#FBBF24',
    badgeText: '#B45309',
    badgeShadow: '#FDE68A',
  },
  A06: {
    id: 'A06',
    name: 'Insecure Design',
    slug: 'insecure-design',
    owaspTag: 'OWASP A06:2025',
    accent: '#C2410C',
    border: '#FB923C',
    shadow: '#C2410C',
    glowRgba: 'rgba(251,146,60,0.25)',
    bgLight: '#FFF7ED',
    cartridgeBg: '#FFEDD5',
    cartridgeBorder: '#FB923C',
    cartridgeText: '#C2410C',
    cartridgeShadow: '#FED7AA',
    badgeBg: '#FFEDD5',
    badgeBorder: '#FB923C',
    badgeText: '#C2410C',
    badgeShadow: '#FED7AA',
  },
  A07: {
    id: 'A07',
    name: 'Authentication Failures',
    slug: 'authentication-failures',
    owaspTag: 'OWASP A07:2025',
    accent: '#1D4ED8',
    border: '#60A5FA',
    shadow: '#1D4ED8',
    glowRgba: 'rgba(96,165,250,0.22)',
    bgLight: '#EFF6FF',
    cartridgeBg: '#DBEAFE',
    cartridgeBorder: '#60A5FA',
    cartridgeText: '#1D4ED8',
    cartridgeShadow: '#BFDBFE',
    badgeBg: '#DBEAFE',
    badgeBorder: '#60A5FA',
    badgeText: '#1D4ED8',
    badgeShadow: '#BFDBFE',
  },
  A08: {
    id: 'A08',
    name: 'Data & Software Integrity',
    slug: 'data-integrity',
    owaspTag: 'OWASP A08:2025',
    accent: '#BE123C',
    border: '#FB7185',
    shadow: '#BE123C',
    glowRgba: 'rgba(251,113,133,0.22)',
    bgLight: '#FFF1F2',
    cartridgeBg: '#FFE4E6',
    cartridgeBorder: '#FB7185',
    cartridgeText: '#BE123C',
    cartridgeShadow: '#FECDD3',
    badgeBg: '#FFE4E6',
    badgeBorder: '#FB7185',
    badgeText: '#BE123C',
    badgeShadow: '#FECDD3',
  },
  A09: {
    id: 'A09',
    name: 'Security Logging & SIEM',
    slug: 'logging-siem',
    owaspTag: 'OWASP A09:2025',
    accent: '#047857',
    border: '#34D399',
    shadow: '#047857',
    glowRgba: 'rgba(52,211,153,0.22)',
    bgLight: '#ECFDF5',
    cartridgeBg: '#D1FAE5',
    cartridgeBorder: '#34D399',
    cartridgeText: '#047857',
    cartridgeShadow: '#A7F3D0',
    badgeBg: '#D1FAE5',
    badgeBorder: '#34D399',
    badgeText: '#047857',
    badgeShadow: '#A7F3D0',
  },
  A10: {
    id: 'A10',
    name: 'SSRF & Exception Handling',
    slug: 'ssrf-exceptions',
    owaspTag: 'OWASP A10:2025',
    accent: '#E11D48',
    border: '#FB7185',
    shadow: '#E11D48',
    glowRgba: 'rgba(251,113,133,0.25)',
    bgLight: '#FFF1F2',
    cartridgeBg: '#FFE4E6',
    cartridgeBorder: '#FB7185',
    cartridgeText: '#BE123C',
    cartridgeShadow: '#FECDD3',
    badgeBg: '#FFE4E6',
    badgeBorder: '#FB7185',
    badgeText: '#BE123C',
    badgeShadow: '#FECDD3',
  },
}

const CATEGORY_LIST = Object.values(CATEGORY_STYLES)

export function getBaseCategory(labId: string): string {
  const match = labId.trim().toUpperCase().match(/^(A\d{2})/)
  return match ? match[1] : 'A01'
}

export function LabsView({ onSelectLab }: LabsViewProps) {
  const [labs, setLabs] = useState<LabItem[]>([])
  const [search, setSearch] = useState('')
  const [statusFilter, setStatusFilter] = useState<StatusFilter>('ALL')
  const [difficulty, setDifficulty] = useState<DifficultyFilter>('ALL')
  const [selectedCategory, setSelectedCategory] = useState<string>('ALL')
  const [loading, setLoading] = useState(true)
  const [actionInProgress, setActionInProgress] = useState<Record<string, string>>({})
  const [solvedLabsMap, setSolvedLabsMap] = useState<Record<string, boolean>>({})
  const [lockedModalInfo, setLockedModalInfo] = useState<LabProgressionInfo | null>(null)
  const isMountedRef = useRef(true)

  const syncSolvedStatus = (labList?: LabItem[]) => {
    try {
      const list = labList ?? labs
      const map: Record<string, boolean> = {}
      list.forEach((l) => {
        map[l.id] = isLabSolved(l.id)
      })
      // Also pre-check standard IDs
      for (let i = 1; i <= 10; i++) {
        const base = `A${String(i).padStart(2, '0')}`
        map[base] = isLabSolved(base)
        map[`${base}-02`] = isLabSolved(`${base}-02`)
        map[`${base}-03`] = isLabSolved(`${base}-03`)
      }
      setSolvedLabsMap(map)
    } catch {
      // Ignore localStorage errors
    }
  }

  const loadLabs = async () => {
    setLoading(true)
    try {
      const data = await getLabs()
      if (isMountedRef.current) {
        setLabs(data)
        syncSolvedStatus(data)
      }
    } finally {
      if (isMountedRef.current) {
        setLoading(false)
      }
    }
  }

  useEffect(() => {
    isMountedRef.current = true
    loadLabs()
    syncSolvedStatus()

    // Listen to local storage and custom progression solve events
    const handleStorageChange = () => syncSolvedStatus()
    const handleProgressionSolved = () => syncSolvedStatus()

    window.addEventListener('storage', handleStorageChange)
    window.addEventListener('zitera_progression_solved', handleProgressionSolved)

    return () => {
      isMountedRef.current = false
      window.removeEventListener('storage', handleStorageChange)
      window.removeEventListener('zitera_progression_solved', handleProgressionSolved)
    }
  }, [])

  const handleAction = async (labId: string, action: 'start' | 'stop' | 'reset' | 'update') => {
    // Check progression gate before starting
    const progression = getLabProgression(labId)
    if (action === 'start' && !progression.prerequisiteSolved) {
      setLockedModalInfo(progression)
      return
    }

    setActionInProgress((prev) => ({ ...prev, [labId]: action }))
    try {
      if (action === 'start') {
        await startLab(labId)
        await loadLabs()
        const current = labs.find((l) => l.id === labId)
        const port = current?.port || 8011
        openBrowser(`http://127.0.0.1:${port}`)
      } else if (action === 'stop') {
        await stopLab(labId)
        await loadLabs()
      } else if (action === 'reset') {
        await resetLab(labId)
        await loadLabs()
      } else if (action === 'update') {
        await updateLab(labId)
        await loadLabs()
      }
    } finally {
      setActionInProgress((prev) => {
        const next = { ...prev }
        delete next[labId]
        return next
      })
    }
  }

  const runningCount = labs.filter((l) => l.running).length
  const updateCount = labs.filter((l) => l.hasUpdate).length
  const solvedCount = useMemo(() => {
    return labs.filter((l) => solvedLabsMap[l.id]).length
  }, [labs, solvedLabsMap])

  const totalCount = labs.length || 30
  const solvedPercent = Math.round((solvedCount / totalCount) * 100)

  // Pre-calculate count and solved stats per category
  const categoryStats = useMemo(() => {
    const stats: Record<string, { total: number; solved: number }> = {}
    CATEGORY_LIST.forEach((cat) => {
      stats[cat.id] = { total: 0, solved: 0 }
    })
    labs.forEach((lab) => {
      const base = getBaseCategory(lab.id)
      if (stats[base]) {
        stats[base].total += 1
        if (solvedLabsMap[lab.id]) {
          stats[base].solved += 1
        }
      }
    })
    return stats
  }, [labs, solvedLabsMap])

  const filteredLabs = useMemo(() => {
    return labs.filter((lab) => {
      const q = search.trim().toLowerCase()
      const detail = LAB_DETAILS[lab.id]
      const baseCat = getBaseCategory(lab.id)
      const style = CATEGORY_STYLES[baseCat]
      const isSolved = !!solvedLabsMap[lab.id]
      const progression = getLabProgression(lab.id)
      const isLocked = !progression.prerequisiteSolved

      const matchesSearch =
        !q ||
        lab.id.toLowerCase().includes(q) ||
        lab.title.toLowerCase().includes(q) ||
        (detail && detail.targetApp.toLowerCase().includes(q)) ||
        (detail && detail.domain.toLowerCase().includes(q)) ||
        (detail && detail.skills.some((s) => s.toLowerCase().includes(q))) ||
        (detail && detail.desc.toLowerCase().includes(q)) ||
        (style && style.name.toLowerCase().includes(q)) ||
        (lab.recommendedTools && lab.recommendedTools.some((t) => t.toLowerCase().includes(q)))

      const matchesStatus =
        statusFilter === 'ALL' ||
        (statusFilter === 'RUNNING' && lab.running) ||
        (statusFilter === 'SOLVED' && isSolved) ||
        (statusFilter === 'LOCKED' && isLocked) ||
        (statusFilter === 'READY' && lab.installed && !lab.running && !isLocked) ||
        (statusFilter === 'UPDATE' && lab.hasUpdate)

      const labDiff = detail ? detail.difficulty : 'Beginner'
      const matchesDifficulty = difficulty === 'ALL' || difficulty === labDiff

      const matchesCategory =
        selectedCategory === 'ALL' ||
        baseCat === selectedCategory ||
        (style && style.slug === selectedCategory)

      return matchesSearch && matchesStatus && matchesDifficulty && matchesCategory
    })
  }, [labs, search, statusFilter, difficulty, selectedCategory, solvedLabsMap])

  return (
    <div className="flex-1 h-full overflow-hidden relative bg-[#FCFBF8]">
      {/* 1. Ambient Background Artwork Texture */}
      <div className="absolute inset-0 pointer-events-none z-0 overflow-hidden select-none">
        <img
          src="/assets/bg_labs.jpg"
          alt=""
          className="w-full h-full object-cover object-center opacity-20"
          onError={(e) => {
            ;(e.target as HTMLElement).style.display = 'none'
          }}
        />
        <div className="absolute inset-0 bg-[#FCFBF8]/82" />
      </div>

      <div className="relative z-10 h-full overflow-y-auto p-6 space-y-6">
        {/* 2. Top Header & Primary Controls */}
        <div className="flex flex-col lg:flex-row lg:items-center justify-between gap-4 border-b-2 border-[#E6E2D8] pb-5">
          <div>
            <div className="flex items-center gap-2 mb-1.5 flex-wrap">
              <span className="px-2.5 py-0.5 rounded-md bg-[#CCFBF1] text-[#0F766E] border-2 border-[#2DD4BF] text-[10.5px] font-['Space_Grotesk'] font-bold flex items-center gap-1.5 shadow-[0_2px_0_0_#99F6E4]">
                <Wifi size={12} className="animate-pulse" />
                <span>DYNAMIC CATALOG // ONLINE</span>
              </span>
              <span className="px-2 py-0.5 rounded-md bg-[#F0EDE8] border-2 border-[#CEC8BF] text-[10.5px] font-['JetBrains_Mono'] text-[#5C5347] font-bold shadow-[0_1.5px_0_0_#E4DFD7]">
                OWASP TOP 10:2025 STANDARD • MULTI-LAB PROGRESSION
              </span>
            </div>
            <h1 className="text-2xl lg:text-[26px] font-black font-['Space_Grotesk'] text-[#1E1A14] tracking-tight">
              SECURITY LABS CATALOG
            </h1>
            <p className="text-xs text-[#5C5347] font-sans mt-0.5">
              Multi-Tier Learning Architecture ({labs.length} laboratories: 10 Foundation + 20 Advanced Exploration Targets)
            </p>
          </div>

          <div className="flex items-center gap-3">
            <button
              onClick={loadLabs}
              className="px-4 py-2 bg-white text-[#0F766E] hover:bg-[#F0FDFA] border-2 border-[#2DD4BF] shadow-[0_3px_0_0_#5EEAD4] hover:shadow-[0_1px_0_0_#5EEAD4] hover:translate-y-0.5 active:translate-y-1 rounded-xl text-xs font-bold font-['Space_Grotesk'] flex items-center gap-2 cursor-pointer transition-all"
            >
              <RefreshCw size={14} className={loading ? 'animate-spin' : ''} />
              <span>Refresh Catalog</span>
            </button>
          </div>
        </div>

        {/* 3. Vibrant 3D Stats Banner Header matching Flutter's Pastel Aesthetic */}
        <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
          {/* Card 1: CTF Solved Flags (Soft Peach) */}
          <div className="p-4 bg-[#FFEDD5] border-2 border-[#FDBA74] rounded-2xl shadow-[0_4px_0_0_#EA580C,0_10px_20px_rgba(234,88,12,0.12)] hover:-translate-y-0.5 transition-all flex items-center justify-between">
            <div className="space-y-1">
              <div className="flex items-center gap-1.5 text-[11px] font-['Space_Grotesk'] font-bold text-[#9A3412] uppercase tracking-wider">
                <Trophy size={14} className="text-[#C2410C]" />
                <span>CTF Solved Flags</span>
              </div>
              <div className="flex items-baseline gap-2">
                <span className="text-2xl font-black font-['JetBrains_Mono'] text-[#9A3412]">
                  {solvedCount}
                </span>
                <span className="text-xs font-bold font-['JetBrains_Mono'] text-[#C2410C]/80">
                  / {totalCount} Targets
                </span>
              </div>
              <div className="text-[10px] font-semibold text-[#C2410C]/90 font-['JetBrains_Mono']">
                {solvedPercent === 100 ? '🎉 All 30 Labs Mastered!' : `${solvedPercent}% Complete (${solvedCount}/${totalCount})`}
              </div>
            </div>

            {/* Colorful Mini Circular Progress Ring */}
            <div className="relative w-12 h-12 shrink-0">
              <svg className="w-full h-full -rotate-90" viewBox="0 0 36 36">
                <path
                  className="text-[#FDBA74]/40"
                  strokeWidth="3.8"
                  stroke="currentColor"
                  fill="none"
                  d="M18 2.0845 a 15.9155 15.9155 0 0 1 0 31.831 a 15.9155 15.9155 0 0 1 0 -31.831"
                />
                <path
                  className="text-[#EA580C] transition-all duration-700 ease-out"
                  strokeDasharray={`${solvedPercent}, 100`}
                  strokeWidth="3.8"
                  strokeLinecap="round"
                  stroke="currentColor"
                  fill="none"
                  d="M18 2.0845 a 15.9155 15.9155 0 0 1 0 31.831 a 15.9155 15.9155 0 0 1 0 -31.831"
                />
              </svg>
              <div className="absolute inset-0 flex items-center justify-center font-['JetBrains_Mono'] font-black text-[10px] text-[#9A3412]">
                {solvedPercent}%
              </div>
            </div>
          </div>

          {/* Card 2: Total Labs (Soft Aqua Mint) */}
          <div className="p-4 bg-[#CCFBF1] border-2 border-[#5EEAD4] rounded-2xl shadow-[0_4px_0_0_#0F766E,0_10px_20px_rgba(15,118,110,0.12)] hover:-translate-y-0.5 transition-all flex items-center justify-between">
            <div className="space-y-1">
              <div className="flex items-center gap-1.5 text-[11px] font-['Space_Grotesk'] font-bold text-[#0F766E] uppercase tracking-wider">
                <ShieldCheck size={14} className="text-[#0F766E]" />
                <span>Total Catalog</span>
              </div>
              <div className="flex items-baseline gap-2">
                <span className="text-2xl font-black font-['JetBrains_Mono'] text-[#0F766E]">
                  {labs.length}
                </span>
                <span className="text-xs font-bold font-['JetBrains_Mono'] text-[#0D9488]">
                  Laboratories (10x3)
                </span>
              </div>
              <div className="text-[10px] font-semibold text-[#0F766E]/90 font-['JetBrains_Mono']">
                10 Foundations • 20 Explorations
              </div>
            </div>

            <div className="w-12 h-12 rounded-xl bg-white/70 border-2 border-[#2DD4BF] flex items-center justify-center text-[#0F766E] shadow-[0_2px_0_0_#5EEAD4]">
              <Layers size={22} />
            </div>
          </div>

          {/* Card 3: Active Sandboxes (Soft Mint Green) */}
          <div className="p-4 bg-[#DCFCE7] border-2 border-[#86EFAC] rounded-2xl shadow-[0_4px_0_0_#16A34A,0_10px_20px_rgba(22,163,74,0.12)] hover:-translate-y-0.5 transition-all flex items-center justify-between">
            <div className="space-y-1">
              <div className="flex items-center gap-1.5 text-[11px] font-['Space_Grotesk'] font-bold text-[#166534] uppercase tracking-wider">
                <Zap size={14} className="text-[#16A34A]" />
                <span>Active Services</span>
              </div>
              <div className="flex items-baseline gap-2">
                <span className="text-2xl font-black font-['JetBrains_Mono'] text-[#166534]">
                  {runningCount}
                </span>
                <span className="text-xs font-bold font-['JetBrains_Mono'] text-[#15803D]">
                  {runningCount === 1 ? 'Target Live' : 'Targets Live'}
                </span>
              </div>
              <div className="text-[10px] font-semibold text-[#166534]/90 font-['JetBrains_Mono'] flex items-center gap-1.5">
                {runningCount > 0 ? (
                  <>
                    <span className="w-2 h-2 rounded-full bg-[#16A34A] animate-ping" />
                    <span>Sandboxed Ports Online</span>
                  </>
                ) : (
                  <span>All Sandboxes Idle</span>
                )}
              </div>
            </div>

            <div className="w-12 h-12 rounded-xl bg-white/70 border-2 border-[#86EFAC] flex items-center justify-center text-[#16A34A] shadow-[0_2px_0_0_#86EFAC]">
              <Activity size={22} className={runningCount > 0 ? 'animate-pulse' : ''} />
            </div>
          </div>

          {/* Card 4: Containment Architecture (Soft Lavender) */}
          <div className="p-4 bg-[#EDE9FE] border-2 border-[#D8B4FE] rounded-2xl shadow-[0_4px_0_0_#7C3AED,0_10px_20px_rgba(124,58,237,0.12)] hover:-translate-y-0.5 transition-all flex items-center justify-between">
            <div className="space-y-1">
              <div className="flex items-center gap-1.5 text-[11px] font-['Space_Grotesk'] font-bold text-[#5B21B6] uppercase tracking-wider">
                <Sparkles size={14} className="text-[#7C3AED]" />
                <span>Containment</span>
              </div>
              <div className="flex items-baseline gap-2">
                <span className="text-xl font-black font-['JetBrains_Mono'] text-[#5B21B6]">
                  Win32 Sandbox
                </span>
              </div>
              <div className="text-[10px] font-semibold text-[#5B21B6]/90 font-['JetBrains_Mono']">
                {updateCount > 0 ? `${updateCount} Lab Update(s)` : 'AppContainer • Gated Progression'}
              </div>
            </div>

            <div className="w-12 h-12 rounded-xl bg-white/70 border-2 border-[#D8B4FE] flex items-center justify-center text-[#7C3AED] shadow-[0_2px_0_0_#D8B4FE]">
              <CheckCircle2 size={22} />
            </div>
          </div>
        </div>

        {/* 4. Search & Filter Bar with 3D Depth & Category Progression Tabs */}
        <div className="space-y-3 bg-[#FCFBF8] border-2 border-[#CEC8BF] rounded-2xl p-4 shadow-[0_3px_0_0_#E4DFD7]">
          {/* Top Row: Search Input + Status Tabs + Difficulty */}
          <div className="flex flex-col xl:flex-row xl:items-center justify-between gap-3">
            {/* Search Field */}
            <div className="relative flex-1 min-w-[280px]">
              <Search className="absolute left-3.5 top-3 text-[#9C9080]" size={16} />
              <input
                type="text"
                placeholder="Search by lab ID, title, target app, or attack skill (e.g. A01, A01-02, IDOR, OpsAdmin)..."
                value={search}
                onChange={(e) => setSearch(e.target.value)}
                className="w-full pl-10 pr-9 py-2 text-xs bg-white border-2 border-[#CEC8BF] rounded-xl shadow-[0_2px_0_0_#E4DFD7] focus:outline-none focus:border-[#2DD4BF] focus:shadow-[0_2px_0_0_#2DD4BF] font-['Space_Grotesk'] text-[#1E1A14] transition-all placeholder:text-[#9C9080]"
              />
              {search && (
                <button
                  onClick={() => setSearch('')}
                  className="absolute right-3 top-2.5 text-[#9C9080] hover:text-[#1E1A14] cursor-pointer"
                  title="Clear search"
                >
                  <X size={15} />
                </button>
              )}
            </div>

            {/* Status Filter Tabs */}
            <div className="flex flex-wrap items-center gap-1.5">
              {(['ALL', 'RUNNING', 'SOLVED', 'LOCKED', 'READY', 'UPDATE'] as StatusFilter[]).map((st) => {
                const active = statusFilter === st
                return (
                  <button
                    key={st}
                    onClick={() => setStatusFilter(st)}
                    className={`px-3 py-1.5 rounded-xl text-xs font-bold font-['Space_Grotesk'] border-2 transition-all cursor-pointer ${
                      active
                        ? 'bg-[#CCFBF1] border-[#0F766E] text-[#0F766E] shadow-[0_2px_0_0_#0F766E] translate-y-0.5'
                        : 'bg-[#FCFBF8] border-[#CEC8BF] text-[#5C5347] shadow-[0_3px_0_0_#E4DFD7] hover:border-[#2DD4BF] hover:text-[#0F766E] hover:shadow-[0_2px_0_0_#2DD4BF] hover:-translate-y-0.5 active:translate-y-0.5'
                    }`}
                  >
                    {st === 'ALL' ? `ALL STATUS (${labs.length})` : st}
                  </button>
                )
              })}

              {/* Difficulty Dropdown / Tabs */}
              <div className="h-6 w-px bg-[#E6E2D8] mx-1 hidden sm:block" />

              <div className="flex items-center bg-[#F0EDE8] p-0.5 rounded-xl border-2 border-[#CEC8BF] shadow-[0_2px_0_0_#E4DFD7]">
                {(['ALL', 'Beginner', 'Intermediate', 'Advanced'] as DifficultyFilter[]).map((d) => {
                  const active = difficulty === d
                  return (
                    <button
                      key={d}
                      onClick={() => setDifficulty(d)}
                      className={`px-2.5 py-1 text-[11px] font-['Space_Grotesk'] font-bold rounded-lg transition-all cursor-pointer ${
                        active
                          ? 'bg-white text-[#0F766E] border-2 border-[#2DD4BF] shadow-[0_2px_0_0_#5EEAD4]'
                          : 'text-[#5C5347] hover:text-[#1E1A14]'
                      }`}
                    >
                      {d}
                    </button>
                  )
                })}
              </div>
            </div>
          </div>

          {/* Bottom Row: Dynamic 10 Category Tabs with Lab Count & Solved Progress */}
          <div className="pt-2 border-t-2 border-[#E6E2D8]/60 flex items-center gap-1.5 overflow-x-auto pb-1 scrollbar-none">
            <span className="text-[11px] font-['JetBrains_Mono'] font-bold text-[#786F62] uppercase tracking-wider shrink-0 mr-1 flex items-center gap-1">
              <Filter size={12} />
              <span>Category:</span>
            </span>

            {/* All Categories Chip */}
            <button
              onClick={() => setSelectedCategory('ALL')}
              className={`px-3 py-1 rounded-xl text-[11px] font-['Space_Grotesk'] font-bold border-2 transition-all cursor-pointer shrink-0 flex items-center gap-1.5 ${
                selectedCategory === 'ALL'
                  ? 'bg-[#1E1A14] text-white border-[#1E1A14] shadow-[0_2px_0_0_#000000] translate-y-0.5'
                  : 'bg-white border-[#CEC8BF] text-[#5C5347] shadow-[0_3px_0_0_#E4DFD7] hover:border-[#2DD4BF] hover:-translate-y-0.5'
              }`}
            >
              <span>ALL CATEGORIES ({labs.length || 30})</span>
              <span className="px-1.5 py-0.2 rounded-full bg-white/20 text-[9.5px]">
                {solvedCount}/{totalCount}
              </span>
            </button>

            {/* 10 Category Tabs */}
            {CATEGORY_LIST.map((cat) => {
              const isSelected = selectedCategory === cat.id || selectedCategory === cat.slug
              const stat = categoryStats[cat.id] || { total: 3, solved: 0 }
              const isComplete = stat.total > 0 && stat.solved === stat.total

              return (
                <button
                  key={cat.id}
                  onClick={() => setSelectedCategory(isSelected ? 'ALL' : cat.id)}
                  className={`px-3 py-1 rounded-xl text-[11px] font-['Space_Grotesk'] font-bold border-2 transition-all cursor-pointer shrink-0 flex items-center gap-1.5 ${
                    isSelected ? 'translate-y-0.5' : 'hover:-translate-y-0.5'
                  }`}
                  style={{
                    backgroundColor: isSelected ? cat.badgeBg : '#FFFFFF',
                    borderColor: isSelected ? cat.border : isComplete ? '#22C55E' : '#CEC8BF',
                    color: isSelected ? cat.badgeText : '#5C5347',
                    boxShadow: isSelected
                      ? `0 2px 0 0 ${cat.shadow}`
                      : '0 3px 0 0 #E4DFD7',
                  }}
                >
                  <span>{cat.name} ({stat.total || 3})</span>
                  <span
                    className={`px-1.5 py-0.2 rounded-full text-[9.5px] font-['JetBrains_Mono'] ${
                      isComplete
                        ? 'bg-[#DCFCE7] text-[#15803D] border border-[#86EFAC]'
                        : isSelected
                        ? 'bg-black/10 text-inherit'
                        : 'bg-[#F0EDE8] text-[#786F62]'
                    }`}
                  >
                    {stat.solved}/{stat.total || 3} Selesai
                  </span>
                </button>
              )
            })}
          </div>
        </div>

        {/* 5. Labs Grid / List */}
        {loading ? (
          <div className="py-16 flex items-center justify-center">
            <CuteAnimeLoading
              message="ENUMERATING MULTI-TIER LABS CATALOG..."
              subMessage="( ≧ᗜ≦) 30 CYBER LABORATORIES LOADED"
            />
          </div>
        ) : filteredLabs.length === 0 ? (
          <div className="bg-[#FCFBF8] border-2 border-[#CEC8BF] rounded-2xl p-12 text-center shadow-[0_4px_0_0_#E4DFD7]">
            <img
              src="/assets/calico_cat.png"
              alt=""
              className="w-16 h-16 object-contain mx-auto mb-3 opacity-60"
            />
            <h3 className="font-['Space_Grotesk'] font-bold text-base text-[#1E1A14]">
              No laboratories matched your search or filters
            </h3>
            <p className="text-xs text-[#5C5347] font-sans mt-1">
              Try changing the search query or resetting the difficulty, category, and status filters.
            </p>
            <button
              onClick={() => {
                setSearch('')
                setStatusFilter('ALL')
                setDifficulty('ALL')
                setSelectedCategory('ALL')
              }}
              className="mt-4 px-4 py-2 bg-[#CCFBF1] text-[#0F766E] border-2 border-[#2DD4BF] shadow-[0_2px_0_0_#5EEAD4] rounded-lg text-xs font-bold font-['Space_Grotesk'] hover:translate-y-0.5 cursor-pointer transition-all"
            >
              Reset All Filters
            </button>
          </div>
        ) : (
          <div className="grid grid-cols-1 xl:grid-cols-2 gap-5">
            {filteredLabs.map((lab) => (
              <TactileLabCard
                key={lab.id}
                lab={lab}
                isSolved={!!solvedLabsMap[lab.id]}
                inProgress={actionInProgress[lab.id]}
                onSelectLab={onSelectLab}
                onAction={handleAction}
                onOpenLockModal={(info) => setLockedModalInfo(info)}
              />
            ))}
          </div>
        )}
      </div>

      {/* 6. Friendly Educational Progression Lock Modal */}
      {lockedModalInfo && (
        <div className="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/50 backdrop-blur-xs select-none">
          <div className="relative max-w-md w-full bg-[#FCFBF8] border-2 border-[#CEC8BF] rounded-2xl p-6 shadow-[0_8px_0_0_#1E1A14,0_20px_35px_rgba(0,0,0,0.25)] space-y-4 animate-in fade-in zoom-in-95 duration-150">
            {/* Header with Lock Icon */}
            <div className="flex items-center gap-3">
              <div className="w-12 h-12 rounded-xl bg-[#FEF2F2] border-2 border-[#EF4444] text-[#DC2626] flex items-center justify-center shadow-[0_3px_0_0_#DC2626] shrink-0">
                <Lock size={24} />
              </div>
              <div>
                <span className="px-2 py-0.5 rounded-md bg-[#FEE2E2] text-[#DC2626] border border-[#FCA5A5] text-[10px] font-['JetBrains_Mono'] font-bold">
                  PROGRESSION GATE // LOCKED
                </span>
                <h3 className="text-lg font-black font-['Space_Grotesk'] text-[#1E1A14] leading-tight mt-0.5">
                  Lab Eksplorasi Terkunci
                </h3>
              </div>
            </div>

            <p className="text-xs text-[#5C5347] font-sans leading-relaxed">
              Selesaikan Lab Wajib <strong className="text-[#1E1A14]">"{lockedModalInfo.prerequisiteTitle || lockedModalInfo.prerequisiteId}"</strong> terlebih dahulu untuk membangun pondasi bug hunting sebelum membuka variasi teknik ini!
            </p>

            {/* Prerequisite Info Box */}
            <div className="p-3 bg-[#F0EDE8] border-2 border-[#CEC8BF] rounded-xl flex items-center justify-between">
              <div className="flex items-center gap-2.5">
                <ShieldAlert size={20} className="text-[#D97706] shrink-0" />
                <div>
                  <div className="text-[10px] font-['JetBrains_Mono'] text-[#786F62] uppercase font-bold">
                    Prasyarat Pembelajaran
                  </div>
                  <div className="text-xs font-bold font-['Space_Grotesk'] text-[#1E1A14]">
                    {lockedModalInfo.prerequisiteId}: {lockedModalInfo.prerequisiteTitle}
                  </div>
                </div>
              </div>
            </div>

            {/* Action Buttons */}
            <div className="flex items-center justify-end gap-2.5 pt-2">
              <button
                onClick={() => setLockedModalInfo(null)}
                className="px-4 py-2 bg-white text-[#5C5347] border-2 border-[#CEC8BF] shadow-[0_2px_0_0_#CEC8BF] hover:bg-[#F0EDE8] rounded-xl text-xs font-bold font-['Space_Grotesk'] cursor-pointer transition-all"
              >
                Mengerti
              </button>
              {lockedModalInfo.prerequisiteId && (
                <button
                  onClick={() => {
                    const prereq = lockedModalInfo.prerequisiteId
                    setLockedModalInfo(null)
                    if (prereq) onSelectLab(prereq)
                  }}
                  className="px-4 py-2 bg-[#0F766E] hover:bg-[#115E59] text-white border-2 border-[#094D47] shadow-[0_3px_0_0_#063934] hover:shadow-[0_1px_0_0_#063934] hover:translate-y-0.5 active:translate-y-1 rounded-xl text-xs font-bold font-['Space_Grotesk'] flex items-center gap-1.5 cursor-pointer transition-all"
                >
                  <span>Buka Lab Prasyarat</span>
                  <ChevronRight size={14} />
                </button>
              )}
            </div>
          </div>
        </div>
      )}
    </div>
  )
}

// ─────────────────────────────────────────────────────────────────────────────
// Tactile 3D Lab Card with Rich Deep Shadows and Progression Gating
// ─────────────────────────────────────────────────────────────────────────────
interface TactileLabCardProps {
  lab: LabItem
  isSolved: boolean
  inProgress?: string
  onSelectLab: (id: string) => void
  onAction: (id: string, action: 'start' | 'stop' | 'reset' | 'update') => Promise<void>
  onOpenLockModal: (info: LabProgressionInfo) => void
}

function TactileLabCard({
  lab,
  isSolved,
  inProgress,
  onSelectLab,
  onAction,
  onOpenLockModal,
}: TactileLabCardProps) {
  const [isHovered, setIsHovered] = useState(false)
  const progression = getLabProgression(lab.id)
  const isLocked = !progression.prerequisiteSolved
  const baseCat = getBaseCategory(lab.id)

  const detail = LAB_DETAILS[lab.id] ?? {
    targetApp: 'Corporate Portal Target',
    domain: 'Enterprise Security Assessment Target',
    desc: 'Offline interactive security engineering environment.',
    skills: ['Exploitation', 'Defense'],
    difficulty: 'Beginner',
  }
  const sticker = STICKER_MAP[lab.id] ?? STICKER_MAP[baseCat] ?? '/assets/sticker_pink.png'
  const style = CATEGORY_STYLES[baseCat] ?? CATEGORY_STYLES['A01']
  const isRunning = lab.running

  // Dynamic Multi-layer 3D Box Shadow matching Flutter tactile specifications
  const computeShadow = () => {
    if (isLocked) {
      return isHovered
        ? '0 2px 0 0 #A8A29E, 0 6px 12px rgba(120,113,108,0.15)'
        : '0 4px 0 0 #CBD5E1, 0 8px 16px rgba(120,113,108,0.12)'
    }
    if (isRunning) {
      return isHovered
        ? '0 4px 0 0 #0F766E, 0 12px 24px rgba(45,212,191,0.32)'
        : '0 6px 0 0 #0F766E, 0 16px 32px rgba(45,212,191,0.25)'
    }
    return isHovered
      ? `0 3px 0 0 ${style.shadow}, 0 8px 18px ${style.glowRgba}`
      : `0 6px 0 0 ${style.shadow}, 0 14px 28px ${style.glowRgba}`
  }

  const handleCardClick = () => {
    if (isLocked) {
      onOpenLockModal(progression)
    } else {
      onSelectLab(lab.id)
    }
  }

  return (
    <div
      onMouseEnter={() => setIsHovered(true)}
      onMouseLeave={() => setIsHovered(false)}
      className={`relative p-5 rounded-2xl border-2 flex flex-col justify-between overflow-hidden group select-none transition-all duration-200 ${
        isLocked ? 'opacity-85 bg-[#FAF9F5]' : ''
      }`}
      style={{
        backgroundColor: isLocked ? '#F7F5F0' : isRunning ? style.bgLight : '#FCFBF8',
        borderColor: isLocked
          ? '#CBD5E1'
          : isRunning
          ? '#2DD4BF'
          : isHovered
          ? style.border
          : '#CEC8BF',
        transform: isHovered ? 'translateY(-3px)' : 'translateY(0px)',
        boxShadow: computeShadow(),
      }}
    >
      {/* Peeking Cute Animal / Mascot Sticker on Top-Right */}
      <div className="absolute top-2.5 right-3 w-10 h-10 pointer-events-none select-none transition-transform duration-300 group-hover:scale-115 group-hover:rotate-6 drop-shadow-sm opacity-90">
        <img
          src={sticker}
          alt=""
          className="w-full h-full object-contain"
          onError={(e) => {
            ;(e.target as HTMLElement).style.display = 'none'
          }}
        />
      </div>

      <div>
        {/* Top Row: 3D Cartridge ID Box + Badges & Title */}
        <div className="flex items-start gap-3.5">
          {/* Cartridge ID Block */}
          <div
            className="w-14 h-14 rounded-xl border-2 flex flex-col items-center justify-center shrink-0 transition-transform duration-200 group-hover:scale-105"
            style={{
              backgroundColor: isLocked ? '#E2E8F0' : style.cartridgeBg,
              borderColor: isLocked ? '#94A3B8' : style.cartridgeBorder,
              boxShadow: isLocked ? '0 3px 0 0 #94A3B8' : `0 3px 0 0 ${style.cartridgeShadow}`,
            }}
          >
            <span
              className="font-['JetBrains_Mono'] font-black text-base leading-none"
              style={{ color: isLocked ? '#475569' : style.cartridgeText }}
            >
              {lab.id}
            </span>
            <span
              className="font-['JetBrains_Mono'] font-bold text-[8px] tracking-wider mt-0.5 opacity-90"
              style={{ color: isLocked ? '#64748B' : style.cartridgeText }}
            >
              {progression.isCore ? 'CORE' : 'EXPLORE'}
            </span>
          </div>

          {/* Header Info */}
          <div className="flex-1 min-w-0 pr-8">
            <div className="flex flex-wrap items-center gap-1.5 mb-1.5">
              {/* 1. Progression Tier Badge (Required by Coordinator) */}
              {progression.isCore && (
                <span className="inline-flex items-center gap-1 px-2.5 py-0.5 rounded-full bg-[#DCFCE7] border-2 border-[#22C55E] text-[10px] font-['JetBrains_Mono'] font-extrabold text-[#15803D] shadow-[0_2px_0_0_#16A34A]">
                  <span>🛡️ LAB WAJIB (PONDASI UTAMA)</span>
                </span>
              )}
              {progression.tier === 'explore-1' && (
                <span className="inline-flex items-center gap-1 px-2.5 py-0.5 rounded-full bg-[#EFF6FF] border-2 border-[#60A5FA] text-[10px] font-['JetBrains_Mono'] font-extrabold text-[#1D4ED8] shadow-[0_2px_0_0_#2563EB]">
                  <span>🚀 LAB EKSPLORASI: TEKNIK A</span>
                </span>
              )}
              {progression.tier === 'explore-2' && (
                <span className="inline-flex items-center gap-1 px-2.5 py-0.5 rounded-full bg-[#FFF7ED] border-2 border-[#FB923C] text-[10px] font-['JetBrains_Mono'] font-extrabold text-[#C2410C] shadow-[0_2px_0_0_#EA580C]">
                  <span>🚀 LAB EKSPLORASI: TEKNIK B</span>
                </span>
              )}

              {/* 2. Locked Badge if Prerequisite Unmet */}
              {isLocked ? (
                <span className="inline-flex items-center gap-1 px-2.5 py-0.5 rounded-full bg-[#FEF2F2] border-2 border-[#EF4444] text-[10px] font-['JetBrains_Mono'] font-extrabold text-[#DC2626] shadow-[0_2px_0_0_#DC2626]">
                  <Lock size={10} />
                  <span>TERKUNCI (Selesaikan Lab Wajib)</span>
                </span>
              ) : isRunning ? (
                <span className="inline-flex items-center gap-1.5 px-2.5 py-0.5 rounded-full bg-[#DCFCE7] border-2 border-[#22C55E] text-[10px] font-['JetBrains_Mono'] font-extrabold text-[#15803D] shadow-[0_2px_0_0_#16A34A]">
                  <span className="relative flex h-2 w-2">
                    <span className="animate-ping absolute inline-flex h-full w-full rounded-full bg-[#22C55E] opacity-75" />
                    <span className="relative inline-flex rounded-full h-2 w-2 bg-[#16A34A]" />
                  </span>
                  <span>RUNNING</span>
                </span>
              ) : (
                <span className="inline-flex items-center gap-1.5 px-2.5 py-0.5 rounded-full bg-[#F1F5F9] border-2 border-[#CBD5E1] text-[10px] font-['JetBrains_Mono'] font-bold text-[#475569] shadow-[0_2px_0_0_#94A3B8]">
                  <span className="w-1.5 h-1.5 rounded-full bg-[#94A3B8]" />
                  <span>READY</span>
                </span>
              )}

              {/* Solved Trophy Badge */}
              {isSolved && (
                <span className="inline-flex items-center gap-1 px-2.5 py-0.5 rounded-full bg-[#FEF3C7] border-2 border-[#F59E0B] text-[10px] font-['JetBrains_Mono'] font-extrabold text-[#B45309] shadow-[0_2px_0_0_#D97706,inset_0_1px_0_0_rgba(255,255,255,0.8)]">
                  <Trophy size={11} className="text-[#D97706]" />
                  <span>SOLVED</span>
                </span>
              )}

              {/* Difficulty Tag with 3D Bevel Pill Styling */}
              {detail.difficulty === 'Beginner' && (
                <span className="px-2 py-0.5 rounded-full text-[9.5px] font-['JetBrains_Mono'] font-extrabold uppercase border-2 border-[#93C5FD] bg-[#EFF6FF] text-[#1D4ED8] shadow-[0_1.5px_0_0_#60A5FA]">
                  BEGINNER
                </span>
              )}
              {detail.difficulty === 'Intermediate' && (
                <span className="px-2 py-0.5 rounded-full text-[9.5px] font-['JetBrains_Mono'] font-extrabold uppercase border-2 border-[#D8B4FE] bg-[#F5F3FF] text-[#6D28D9] shadow-[0_1.5px_0_0_#A855F7]">
                  INTERMEDIATE
                </span>
              )}
              {detail.difficulty === 'Advanced' && (
                <span className="px-2 py-0.5 rounded-full text-[9.5px] font-['JetBrains_Mono'] font-extrabold uppercase border-2 border-[#FECDD3] bg-[#FFF1F2] text-[#BE123C] shadow-[0_1.5px_0_0_#FB7185]">
                  ADVANCED
                </span>
              )}

              {/* OWASP Category Bevel Pill */}
              <span
                className="px-2 py-0.5 rounded-md text-[9.5px] font-['JetBrains_Mono'] font-bold border-2"
                style={{
                  backgroundColor: style.badgeBg,
                  borderColor: style.badgeBorder,
                  color: style.badgeText,
                  boxShadow: `0 1.5px 0 0 ${style.badgeShadow}`,
                }}
              >
                {style.name}
              </span>
            </div>

            {/* Lab Title */}
            <h3
              onClick={handleCardClick}
              className="font-['Space_Grotesk'] font-bold text-base text-[#1E1A14] hover:text-[#0F766E] cursor-pointer transition-colors leading-snug truncate"
              title={lab.title}
            >
              {lab.title}
            </h3>

            {/* Commercial App Target Subtitle */}
            <div className="text-[11px] font-['Space_Grotesk'] font-medium text-[#786F62] truncate mt-0.5">
              <span className="font-bold text-[#1E1A14]">{detail.targetApp}</span>
              <span className="mx-1.5 opacity-40">•</span>
              <span>{detail.domain}</span>
            </div>
          </div>
        </div>

        {/* Description */}
        <p className="text-xs text-[#5C5347] font-sans mt-3 line-clamp-2 leading-relaxed">
          {detail.desc}
        </p>

        {/* Metadata & Skill Pills */}
        <div className="mt-3 pt-3 border-t-2 border-[#E6E2D8]/60 flex flex-wrap items-center justify-between gap-2">
          <div className="text-[11px] font-['JetBrains_Mono'] text-[#786F62] flex items-center gap-1.5">
            <span className="font-semibold text-[#1E1A14]">PORT :{lab.port}</span>
            <span>•</span>
            <span>127.0.0.1 (Local Sandbox)</span>
          </div>

          <div className="flex flex-wrap items-center gap-1">
            {detail.skills.slice(0, 3).map((sk) => (
              <span
                key={sk}
                className="px-1.5 py-0.5 rounded-md border text-[9.5px] font-['JetBrains_Mono'] font-semibold"
                style={{
                  backgroundColor: style.badgeBg,
                  borderColor: style.badgeBorder,
                  color: style.badgeText,
                }}
              >
                #{sk}
              </span>
            ))}
          </div>
        </div>
      </div>

      {/* Bottom Action Controls with Tactile Press Depth */}
      <div className="pt-3.5 mt-3.5 border-t-2 border-[#E6E2D8] flex flex-wrap items-center justify-between gap-2">
        <div className="flex items-center gap-2">
          {isLocked ? (
            <button
              onClick={() => onOpenLockModal(progression)}
              className="px-4 py-2 bg-[#F1F5F9] hover:bg-[#E2E8F0] text-[#64748B] text-xs font-bold font-['Space_Grotesk'] rounded-xl flex items-center gap-1.5 border-2 border-[#CBD5E1] shadow-[0_3px_0_0_#94A3B8] hover:shadow-[0_1px_0_0_#94A3B8] hover:translate-y-0.5 active:translate-y-1 active:shadow-none transition-all cursor-pointer"
            >
              <Lock size={13} className="text-[#DC2626]" />
              <span>Buka Lab Prasyarat</span>
            </button>
          ) : isRunning ? (
            <>
              <button
                onClick={() => openBrowser(`http://127.0.0.1:${lab.port}`)}
                className="px-3.5 py-2 bg-[#2563EB] hover:bg-[#1D4ED8] text-white text-xs font-bold font-['Space_Grotesk'] rounded-xl flex items-center gap-1.5 border-2 border-[#1E40AF] shadow-[0_3px_0_0_#1E3A8A] hover:shadow-[0_1px_0_0_#1E3A8A] hover:translate-y-0.5 active:translate-y-1 active:shadow-none transition-all cursor-pointer"
              >
                <span>Buka Web</span>
                <ExternalLink size={13} />
              </button>
              <button
                onClick={() => onAction(lab.id, 'stop')}
                disabled={!!inProgress}
                className="px-3 py-2 bg-white hover:bg-[#FEF2F2] border-2 border-[#EF4444] text-[#DC2626] text-xs font-bold font-['Space_Grotesk'] rounded-xl flex items-center gap-1.5 shadow-[0_3px_0_0_#F87171] hover:shadow-[0_1px_0_0_#F87171] hover:translate-y-0.5 active:translate-y-1 active:shadow-none transition-all disabled:opacity-50 cursor-pointer"
              >
                <Square size={12} />
                <span>Stop</span>
              </button>
              <button
                onClick={() => onAction(lab.id, 'reset')}
                disabled={!!inProgress}
                className="p-2 bg-white hover:bg-[#F8FAFC] border-2 border-[#CBD5E1] text-[#475569] rounded-xl shadow-[0_3px_0_0_#94A3B8] hover:shadow-[0_1px_0_0_#94A3B8] hover:translate-y-0.5 active:translate-y-1 active:shadow-none transition-all cursor-pointer"
                title="Reset target database & files"
              >
                <RotateCcw size={14} />
              </button>
            </>
          ) : (
            <button
              onClick={() => onAction(lab.id, 'start')}
              disabled={!!inProgress}
              className="px-4 py-2 bg-[#0F766E] hover:bg-[#115E59] text-white text-xs font-bold font-['Space_Grotesk'] rounded-xl flex items-center gap-1.5 border-2 border-[#094D47] shadow-[0_3px_0_0_#063934] hover:shadow-[0_1px_0_0_#063934] hover:translate-y-0.5 active:translate-y-1 active:shadow-none transition-all disabled:opacity-50 cursor-pointer"
            >
              <Play size={13} />
              <span>{inProgress === 'start' ? 'Launching...' : 'RUN LAB'}</span>
            </button>
          )}

          {lab.hasUpdate && (
            <button
              onClick={() => onAction(lab.id, 'update')}
              disabled={!!inProgress}
              className="px-3 py-2 bg-[#FEF3C7] border-2 border-[#FCD34D] text-[#B45309] hover:bg-[#FDE68A] text-xs font-bold font-['Space_Grotesk'] rounded-xl flex items-center gap-1 shadow-[0_3px_0_0_#FCD34D] hover:shadow-[0_1px_0_0_#FCD34D] hover:translate-y-0.5 active:translate-y-1 active:shadow-none transition-all cursor-pointer"
              title="Update lab package"
            >
              <DownloadCloud size={13} />
              <span>Update Lab</span>
            </button>
          )}
        </div>

        {/* Inspect Lab Link Button */}
        <button
          onClick={handleCardClick}
          className={`px-3.5 py-2 border-2 text-xs font-bold font-['Space_Grotesk'] rounded-xl flex items-center gap-1.5 cursor-pointer transition-all ${
            isLocked
              ? 'bg-[#F1F5F9] border-[#CBD5E1] text-[#64748B] shadow-[0_2px_0_0_#94A3B8]'
              : 'bg-white hover:bg-[#F0FDFA] border-[#2DD4BF] text-[#0F766E] shadow-[0_3px_0_0_#5EEAD4] hover:shadow-[0_1px_0_0_#5EEAD4] hover:translate-y-0.5 active:translate-y-1 active:shadow-none'
          }`}
        >
          <span>{isLocked ? 'Lihat Prasyarat' : 'Inspect Lab'}</span>
          <ChevronRight size={14} />
        </button>
      </div>
    </div>
  )
}
