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

interface LabsViewProps {
  onSelectLab: (labId: string) => void
}

type StatusFilter = 'ALL' | 'RUNNING' | 'SOLVED' | 'READY' | 'UPDATE'
type DifficultyFilter = 'ALL' | 'Beginner' | 'Intermediate' | 'Advanced'

const STICKER_MAP: Record<string, string> = {
  A01: '/assets/sticker_pink.png',
  A02: '/assets/sticker_duck.png',
  A03: '/assets/sticker_bunny.png',
  A04: '/assets/sticker_cat_laptop.png',
  A05: '/assets/sticker_hamster.png',
  A06: '/assets/sticker_shiba.png',
  A07: '/assets/sticker_blonde.png',
  A08: '/assets/sticker_purple.png',
  A09: '/assets/sticker_pink.png',
  A10: '/assets/sticker_duck.png',
  W01: '/assets/sticker_cat_laptop.png',
  W02: '/assets/sticker_bunny.png',
  W03: '/assets/sticker_hamster.png',
  W04: '/assets/sticker_shiba.png',
  W05: '/assets/sticker_pink.png',
  F01: '/assets/sticker_duck.png',
  F02: '/assets/sticker_blonde.png',
  F03: '/assets/sticker_purple.png',
  F04: '/assets/sticker_cat_laptop.png',
  F05: '/assets/sticker_bunny.png',
  F06: '/assets/sticker_hamster.png',
  D01: '/assets/sticker_shiba.png',
  D02: '/assets/sticker_pink.png',
  A11: '/assets/sticker_blonde.png',
  A12: '/assets/sticker_purple.png',
}

interface LabMetadata {
  targetApp: string
  domain: string
  desc: string
  skills: string[]
  difficulty: 'Beginner' | 'Intermediate' | 'Advanced'
}

const LAB_DETAILS: Record<string, LabMetadata> = {
  A01: {
    targetApp: 'VaultPay Enterprise',
    domain: 'Fintech B2B Treasury & Wire Transfer Portal',
    desc: 'Investigate Insecure Direct Object References (IDOR) and bypass horizontal authorization controls in a mock banking API.',
    skills: ['IDOR', 'BOLA', 'Access Control', 'API Security'],
    difficulty: 'Beginner',
  },
  A02: {
    targetApp: 'CloudCore OpsGateway',
    domain: 'Enterprise Cloud Management & Microservices Monitor',
    desc: 'Exploit verbose stack traces, default administrative credentials, and exposed diagnostics endpoints in OpsGateway.',
    skills: ['Misconfiguration', 'Directory Listing', 'Debug Leaks', 'CORS'],
    difficulty: 'Beginner',
  },
  A03: {
    targetApp: 'PackNexus Supply Hub',
    domain: 'B2B Package Registry & Build Pipeline Hub',
    desc: 'Analyze compromised upstream dependencies, poisoned package manifests, and insecure package resolution pipelines.',
    skills: ['Supply Chain', 'Dependency Confusion', 'Integrity Checking'],
    difficulty: 'Beginner',
  },
  A04: {
    targetApp: 'SafeKey Corporate Vault',
    domain: 'Banking Encryption & Credential Vault',
    desc: 'Exploit unsanitized user input in cryptographic authentication hashes and race conditions in financial ledger updates.',
    skills: ['Crypto Flaws', 'Timing Attacks', 'Insecure Hashes'],
    difficulty: 'Intermediate',
  },
  A05: {
    targetApp: 'MegaStore Global',
    domain: 'Commercial E-Commerce & Retail Inventory Portal',
    desc: 'Perform classic SQL injection, boolean-based inference, and command injection across legacy backend query handlers.',
    skills: ['SQL Injection', 'Blind SQLi', 'Input Sanitization'],
    difficulty: 'Intermediate',
  },
  A06: {
    targetApp: 'AuthShield Identity',
    domain: 'Enterprise SSO & Access Management',
    desc: 'Audit vulnerable microservice endpoints lacking integrity checks and exploit insecure deserialization vulnerabilities.',
    skills: ['Deserialization', 'Object Injection', 'Memory Corrupt'],
    difficulty: 'Intermediate',
  },
  A07: {
    targetApp: 'NexHealth Portal',
    domain: 'Hospital Electronic Medical Records & Patient Portal',
    desc: 'Bypass broken multi-factor authentication, exploit session fixation, and hijack persistent JWT tokens.',
    skills: ['Session Hijacking', 'JWT Tampering', 'Brute Force Defense'],
    difficulty: 'Intermediate',
  },
  A08: {
    targetApp: 'DataFlow Analytics',
    domain: 'Enterprise BI & Data Processing Engine',
    desc: 'Perform Server-Side Request Forgery (SSRF) and audit unsafe deserialization pipelines across internal microservices.',
    skills: ['SSRF', 'Cloud Metadata', 'Internal Port Scan'],
    difficulty: 'Advanced',
  },
  A09: {
    targetApp: 'FleetTrack Logistics',
    domain: 'Commercial Telematics & IoT Fleet Management',
    desc: 'Investigate blind log injection, audit trail evasion, and insufficient security monitoring during anomalous attack traffic.',
    skills: ['Log Injection', 'Audit Evasion', 'Incident Response'],
    difficulty: 'Advanced',
  },
  A10: {
    targetApp: 'TemplateCloud Studio',
    domain: 'Commercial SaaS Email & PDF Document Generator',
    desc: 'Weaponize complex server-side template injection (SSTI) and remote code execution across edge renderers.',
    skills: ['SSTI', 'RCE', 'Sandbox Escape', 'Advanced Exploitation'],
    difficulty: 'Advanced',
  },
  W01: {
    targetApp: 'NexusPay Wire Portal',
    domain: 'Fintech Interbank Settlement & Wire Gateway',
    desc: 'Pelajari bagaimana browser menyertakan cookie sesi pada request lintas origin dan mitigasi anti-CSRF token.',
    skills: ['CSRF', 'SameSite Cookie', 'Anti-CSRF Token', 'Cross-Origin Defense'],
    difficulty: 'Intermediate',
  },
  W02: {
    targetApp: 'CloudFetch Webhook Dispatcher',
    domain: 'Cloud Integration & Microservice Webhook Engine',
    desc: 'Eksploitasi fitur fetch URL internal untuk memindai port privat lokal dan mengakses metadata service.',
    skills: ['SSRF', 'Cloud Metadata', 'Loopback Bypass', 'Network Segmentation'],
    difficulty: 'Advanced',
  },
  W03: {
    targetApp: 'ShopMatrix Order Gateway',
    domain: 'B2B E-Commerce Wholesale & Order Routing API',
    desc: 'Identifikasi celah otorisasi level objek pada REST API modern di mana manipulasi ID pesanan membocorkan transaksi lain.',
    skills: ['API BOLA', 'Object Authorization', 'REST Security', 'IDOR API'],
    difficulty: 'Intermediate',
  },
  W04: {
    targetApp: 'DocuCloud Asset Manager',
    domain: 'Enterprise Cloud Document & Asset Repository',
    desc: 'Uji kelemahan validasi berkas unggahan, manipulasi MIME type, bypass ekstensi ganda, dan eksekusi payload.',
    skills: ['File Upload', 'MIME Sniffing', 'Extension Bypass', 'Webshell Analysis'],
    difficulty: 'Advanced',
  },
  W05: {
    targetApp: 'Enterprise DocViewer',
    domain: 'Corporate Document Management & Report Reader',
    desc: 'Pahami bagaimana karakter dot-dot-slash memanipulasi path sistem berkas server untuk membaca file rahasia.',
    skills: ['Path Traversal', 'LFI', 'Directory Traversal', 'File Reading Defense'],
    difficulty: 'Intermediate',
  },
  F01: {
    targetApp: 'HTTP Protocol Inspector',
    domain: 'Cybersecurity Educational Core Architecture',
    desc: 'Kuasai dasar protokol HTTP: analisis method GET/POST/PUT, status code 200/403/500, dan struktur header permintaan web.',
    skills: ['HTTP Protocol', 'Headers', 'Status Codes', 'Web Traffic'],
    difficulty: 'Beginner',
  },
  F02: {
    targetApp: 'SessionGuard Simulator',
    domain: 'Session Management & Web State Engineering',
    desc: 'Pahami sifat stateless HTTP, pemeriksaan atribut keamanan cookie (HttpOnly, Secure, SameSite), dan siklus hidup token.',
    skills: ['Cookies', 'HttpOnly', 'SameSite', 'Session Management'],
    difficulty: 'Beginner',
  },
  F03: {
    targetApp: 'TrustBoundary Analyzer',
    domain: 'Core Security Architecture & Threat Modeling',
    desc: 'Pahami perbedaan fundamental siapa Anda (AuthN) dan apa izin Anda (AuthZ), serta batas kepercayaan klien-server.',
    skills: ['AuthN vs AuthZ', 'Trust Boundary', 'Client Bypass', 'Access Principles'],
    difficulty: 'Beginner',
  },
  F04: {
    targetApp: 'CryptoLab Sandbox',
    domain: 'Applied Cryptography & Data Protection Core',
    desc: 'Hilangkan kerancuan antara Base64, SHA-256, dan AES. Pelajari sifat satu arah cryptographic hash dan peran salt.',
    skills: ['Base64', 'SHA-256', 'AES', 'Salting & Hashing'],
    difficulty: 'Beginner',
  },
  F05: {
    targetApp: 'SQL Query Compiler Visualizer',
    domain: 'Database Engineering & Injection Prevention',
    desc: 'Pelajari bagaimana parser SQL mengompilasi string query dan amati titik bahaya di mana data input mengubah sintaks.',
    skills: ['SQL Grammar', 'Query Trees', 'Parameterized Queries', 'Data Separation'],
    difficulty: 'Beginner',
  },
  F06: {
    targetApp: 'Zitera Cadet Onboarding Station',
    domain: 'Zitera Lab Operational Architecture',
    desc: 'Orientasi lengkap alur kerja ZITERA LAB: kontrol sandbox, status readiness, terminal interaktif, dan format verifikasi flag.',
    skills: ['CTF Workflow', 'Flag Submission', 'Sandbox Mechanics', 'Verification'],
    difficulty: 'Beginner',
  },
  D01: {
    targetApp: 'CyberSentinel Log SIEM',
    domain: 'Enterprise Incident Detection & Threat Hunting',
    desc: 'Analisis berkas log web server, identifikasi jejak scanning botnet, injeksi SQL, dan rekonstruksi timeline insiden.',
    skills: ['Log Forensics', 'Threat Hunting', 'Incident Response', 'Pattern Analysis'],
    difficulty: 'Intermediate',
  },
  D02: {
    targetApp: 'SigCheck Package Validator',
    domain: 'Software Supply Chain & Binary Authenticity',
    desc: 'Verifikasi ringkasan hash SHA-256 dan tanda tangan digital asimetris Ed25519 untuk mendeteksi manipulasi biner.',
    skills: ['Ed25519', 'SHA-256', 'Binary Integrity', 'Signature Verification'],
    difficulty: 'Intermediate',
  },
  A11: {
    targetApp: 'UserHub Identity Admin',
    domain: 'Enterprise User Directory & RBAC Gateway',
    desc: 'Eksploitasi fitur binding otomatis payload JSON ke model database yang memungkinkan eskalasi peran menjadi administrator.',
    skills: ['Mass Assignment', 'Privilege Escalation', 'JSON Binding', 'DTO Hardening'],
    difficulty: 'Intermediate',
  },
  A12: {
    targetApp: 'SocialAuth Connect Gateway',
    domain: 'Federated Identity & Single Sign-On Portal',
    desc: 'Audit kelemahan implementasi parameter state pada alur OAuth 2.0 Authorization Code yang memungkinkan pembajakan akun.',
    skills: ['OAuth 2.0', 'State Parameter', 'Account Takeover', 'SSO Security'],
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
    name: 'Misconfiguration',
    slug: 'misconfiguration',
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
    name: 'Supply Chain',
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
    name: 'Cryptography',
    slug: 'cryptography',
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
    name: 'Authentication',
    slug: 'authentication',
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
    name: 'Deserialization',
    slug: 'deserialization',
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
    name: 'Logging & Audit',
    slug: 'logging',
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
    name: 'SSRF & SSTI',
    slug: 'ssrf',
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
  W01: {
    id: 'W01',
    name: 'CSRF & Token Defense',
    slug: 'csrf',
    owaspTag: 'WEB W01:2026',
    accent: '#0891B2',
    border: '#22D3EE',
    shadow: '#0891B2',
    glowRgba: 'rgba(34,211,238,0.22)',
    bgLight: '#ECFEFF',
    cartridgeBg: '#CFFAFE',
    cartridgeBorder: '#22D3EE',
    cartridgeText: '#0891B2',
    cartridgeShadow: '#A5F3FC',
    badgeBg: '#CFFAFE',
    badgeBorder: '#22D3EE',
    badgeText: '#0891B2',
    badgeShadow: '#A5F3FC',
  },
  W02: {
    id: 'W02',
    name: 'SSRF & Cloud Metadata',
    slug: 'ssrf-cloud',
    owaspTag: 'WEB W02:2026',
    accent: '#EA580C',
    border: '#FB923C',
    shadow: '#EA580C',
    glowRgba: 'rgba(251,146,60,0.22)',
    bgLight: '#FFF7ED',
    cartridgeBg: '#FFEDD5',
    cartridgeBorder: '#FB923C',
    cartridgeText: '#EA580C',
    cartridgeShadow: '#FED7AA',
    badgeBg: '#FFEDD5',
    badgeBorder: '#FB923C',
    badgeText: '#EA580C',
    badgeShadow: '#FED7AA',
  },
  W03: {
    id: 'W03',
    name: 'API BOLA / IDOR',
    slug: 'api-bola',
    owaspTag: 'API W03:2026',
    accent: '#4F46E5',
    border: '#818CF8',
    shadow: '#4F46E5',
    glowRgba: 'rgba(129,140,248,0.22)',
    bgLight: '#EEF2FF',
    cartridgeBg: '#E0E7FF',
    cartridgeBorder: '#818CF8',
    cartridgeText: '#4F46E5',
    cartridgeShadow: '#C7D2FE',
    badgeBg: '#E0E7FF',
    badgeBorder: '#818CF8',
    badgeText: '#4F46E5',
    badgeShadow: '#C7D2FE',
  },
  W04: {
    id: 'W04',
    name: 'Arbitrary File Upload',
    slug: 'file-upload',
    owaspTag: 'WEB W04:2026',
    accent: '#D97706',
    border: '#FBBF24',
    shadow: '#D97706',
    glowRgba: 'rgba(251,191,36,0.22)',
    bgLight: '#FFFBEB',
    cartridgeBg: '#FEF3C7',
    cartridgeBorder: '#FBBF24',
    cartridgeText: '#D97706',
    cartridgeShadow: '#FDE68A',
    badgeBg: '#FEF3C7',
    badgeBorder: '#FBBF24',
    badgeText: '#D97706',
    badgeShadow: '#FDE68A',
  },
  W05: {
    id: 'W05',
    name: 'Path Traversal & LFI',
    slug: 'path-traversal',
    owaspTag: 'WEB W05:2026',
    accent: '#DC2626',
    border: '#F87171',
    shadow: '#DC2626',
    glowRgba: 'rgba(248,113,113,0.22)',
    bgLight: '#FEF2F2',
    cartridgeBg: '#FEE2E2',
    cartridgeBorder: '#F87171',
    cartridgeText: '#DC2626',
    cartridgeShadow: '#FECACA',
    badgeBg: '#FEE2E2',
    badgeBorder: '#F87171',
    badgeText: '#DC2626',
    badgeShadow: '#FECACA',
  },
  F01: {
    id: 'F01',
    name: 'HTTP Protocol Basics',
    slug: 'http-basics',
    owaspTag: 'FOUNDATION F01',
    accent: '#059669',
    border: '#34D399',
    shadow: '#059669',
    glowRgba: 'rgba(52,211,153,0.22)',
    bgLight: '#ECFDF5',
    cartridgeBg: '#D1FAE5',
    cartridgeBorder: '#34D399',
    cartridgeText: '#059669',
    cartridgeShadow: '#A7F3D0',
    badgeBg: '#D1FAE5',
    badgeBorder: '#34D399',
    badgeText: '#059669',
    badgeShadow: '#A7F3D0',
  },
  F02: {
    id: 'F02',
    name: 'Cookies & Sessions',
    slug: 'cookies-sessions',
    owaspTag: 'FOUNDATION F02',
    accent: '#2563EB',
    border: '#60A5FA',
    shadow: '#2563EB',
    glowRgba: 'rgba(96,165,250,0.22)',
    bgLight: '#EFF6FF',
    cartridgeBg: '#DBEAFE',
    cartridgeBorder: '#60A5FA',
    cartridgeText: '#2563EB',
    cartridgeShadow: '#BFDBFE',
    badgeBg: '#DBEAFE',
    badgeBorder: '#60A5FA',
    badgeText: '#2563EB',
    badgeShadow: '#BFDBFE',
  },
  F03: {
    id: 'F03',
    name: 'AuthN vs AuthZ',
    slug: 'authn-authz',
    owaspTag: 'FOUNDATION F03',
    accent: '#7C3AED',
    border: '#A78BFA',
    shadow: '#7C3AED',
    glowRgba: 'rgba(167,139,250,0.22)',
    bgLight: '#F5F3FF',
    cartridgeBg: '#EDE9FE',
    cartridgeBorder: '#A78BFA',
    cartridgeText: '#7C3AED',
    cartridgeShadow: '#DDD6FE',
    badgeBg: '#EDE9FE',
    badgeBorder: '#A78BFA',
    badgeText: '#7C3AED',
    badgeShadow: '#DDD6FE',
  },
  F04: {
    id: 'F04',
    name: 'Crypto & Hash Basics',
    slug: 'crypto-basics',
    owaspTag: 'FOUNDATION F04',
    accent: '#9333EA',
    border: '#C084FC',
    shadow: '#9333EA',
    glowRgba: 'rgba(192,132,252,0.22)',
    bgLight: '#FAF5FF',
    cartridgeBg: '#F3E8FF',
    cartridgeBorder: '#C084FC',
    cartridgeText: '#9333EA',
    cartridgeShadow: '#E9D5FF',
    badgeBg: '#F3E8FF',
    badgeBorder: '#C084FC',
    badgeText: '#9333EA',
    badgeShadow: '#E9D5FF',
  },
  F05: {
    id: 'F05',
    name: 'SQL Query Grammar',
    slug: 'sql-grammar',
    owaspTag: 'FOUNDATION F05',
    accent: '#B45309',
    border: '#FBBF24',
    shadow: '#B45309',
    glowRgba: 'rgba(251,191,36,0.22)',
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
  F06: {
    id: 'F06',
    name: 'Zitera Lab Mechanics',
    slug: 'lab-mechanics',
    owaspTag: 'FOUNDATION F06',
    accent: '#0F766E',
    border: '#2DD4BF',
    shadow: '#0F766E',
    glowRgba: 'rgba(45,212,191,0.22)',
    bgLight: '#F0FDFA',
    cartridgeBg: '#CCFBF1',
    cartridgeBorder: '#2DD4BF',
    cartridgeText: '#0F766E',
    cartridgeShadow: '#99F6E4',
    badgeBg: '#CCFBF1',
    badgeBorder: '#2DD4BF',
    badgeText: '#0F766E',
    badgeShadow: '#99F6E4',
  },
  D01: {
    id: 'D01',
    name: 'Web Server Log SIEM',
    slug: 'log-siem',
    owaspTag: 'DEFENSIVE D01',
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
  D02: {
    id: 'D02',
    name: 'Package Integrity & Sig',
    slug: 'sig-check',
    owaspTag: 'DEFENSIVE D02',
    accent: '#6366F1',
    border: '#818CF8',
    shadow: '#6366F1',
    glowRgba: 'rgba(129,140,248,0.22)',
    bgLight: '#EEF2FF',
    cartridgeBg: '#E0E7FF',
    cartridgeBorder: '#818CF8',
    cartridgeText: '#6366F1',
    cartridgeShadow: '#C7D2FE',
    badgeBg: '#E0E7FF',
    badgeBorder: '#818CF8',
    badgeText: '#6366F1',
    badgeShadow: '#C7D2FE',
  },
  A11: {
    id: 'A11',
    name: 'API Mass Assignment',
    slug: 'mass-assignment',
    owaspTag: 'ADVANCED A11',
    accent: '#C2410C',
    border: '#FB923C',
    shadow: '#C2410C',
    glowRgba: 'rgba(251,146,60,0.22)',
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
  A12: {
    id: 'A12',
    name: 'OAuth 2.0 Takeover',
    slug: 'oauth-takeover',
    owaspTag: 'ADVANCED A12',
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
}

const TRACK_CHIPS = [
  { id: 'ALL', name: 'ALL CATEGORIES' },
  { id: 'OWASP', name: 'OWASP Top 10' },
  { id: 'WEB', name: 'Web Security' },
  { id: 'API', name: 'API Security' },
  { id: 'FOUNDATIONS', name: 'Foundations' },
  { id: 'DEFENSIVE', name: 'Defensive & Forensics' },
  { id: 'ADVANCED', name: 'Advanced Attacks' },
]


export function LabsView({ onSelectLab }: LabsViewProps) {
  const [labs, setLabs] = useState<LabItem[]>([])
  const [search, setSearch] = useState('')
  const [statusFilter, setStatusFilter] = useState<StatusFilter>('ALL')
  const [difficulty, setDifficulty] = useState<DifficultyFilter>('ALL')
  const [selectedCategory, setSelectedCategory] = useState<string>('ALL')
  const [loading, setLoading] = useState(true)
  const [actionInProgress, setActionInProgress] = useState<Record<string, string>>({})
  const [solvedLabsMap, setSolvedLabsMap] = useState<Record<string, boolean>>({})
  const isMountedRef = useRef(true)

    const syncSolvedStatus = () => {
    try {
      const map: Record<string, boolean> = {}
      labs.forEach((l) => {
        map[l.id] = localStorage.getItem(`zitera_solved_${l.id}`) === 'true'
      })
      const allKnown = [
        'A01','A02','A03','A04','A05','A06','A07','A08','A09','A10',
        'W01','W02','W03','W04','W05','F01','F02','F03','F04','F05','F06',
        'D01','D02','A11','A12'
      ]
      allKnown.forEach((id) => {
        if (localStorage.getItem(`zitera_solved_${id}`) === 'true') {
          map[id] = true
        }
      })
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
        syncSolvedStatus()
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

    // Listen to local storage events for solved updates
    const handleStorageChange = () => syncSolvedStatus()
    window.addEventListener('storage', handleStorageChange)

    return () => {
      isMountedRef.current = false
      window.removeEventListener('storage', handleStorageChange)
    }
  }, [])

  const handleAction = async (labId: string, action: 'start' | 'stop' | 'reset' | 'update') => {
    setActionInProgress((prev) => ({ ...prev, [labId]: action }))
    try {
      if (action === 'start') {
        await startLab(labId)
        await loadLabs()
        // Auto-open browser on launch matching Flutter behavior
        const current = labs.find((l) => l.id === labId)
        const targetUrl = current?.url || `http://127.0.0.1:${current?.port || 8011}/`
        openBrowser(targetUrl)
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

  const totalCount = labs.length || 10
  const solvedPercent = Math.round((solvedCount / totalCount) * 100)

  const filteredLabs = useMemo(() => {
    return labs.filter((lab) => {
      const q = search.trim().toLowerCase()
      const detail = LAB_DETAILS[lab.id]
      const style = CATEGORY_STYLES[lab.id]
      const isSolved = !!solvedLabsMap[lab.id]

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
        (statusFilter === 'READY' && lab.installed && !lab.running) ||
        (statusFilter === 'UPDATE' && lab.hasUpdate)

      const labDiff = detail ? detail.difficulty : 'Beginner'
      const matchesDifficulty = difficulty === 'ALL' || difficulty === labDiff

      const matchesCategory =
        selectedCategory === 'ALL' ||
        (selectedCategory === 'OWASP' && lab.id.startsWith('A') && parseInt(lab.id.slice(1) || '99') <= 10) ||
        (selectedCategory === 'WEB' && (lab.id.startsWith('W') && lab.id !== 'W03')) ||
        (selectedCategory === 'API' && (lab.id === 'W03' || lab.id === 'A11')) ||
        (selectedCategory === 'FOUNDATIONS' && lab.id.startsWith('F')) ||
        (selectedCategory === 'DEFENSIVE' && lab.id.startsWith('D')) ||
        (selectedCategory === 'ADVANCED' && (lab.id === 'A11' || lab.id === 'A12')) ||
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
          className="w-full h-full object-cover object-center opacity-85"
          onError={(e) => {
            ;(e.target as HTMLElement).style.display = 'none'
          }}
        />
        <div className="absolute inset-0 bg-[#FCFBF8]/50 cyber-dot-grid" />
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
                OWASP TOP 10:2025 STANDARD
              </span>
            </div>
            <h1 className="text-2xl lg:text-[26px] font-black font-['Space_Grotesk'] text-[#1E1A14] tracking-tight">
              SECURITY LABS CATALOG
            </h1>
            <p className="text-xs text-[#5C5347] font-sans mt-0.5">
              Isolated offline vulnerability training environments with deterministic runtime targets ({labs.length} laboratories available)
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
                {solvedPercent === 100 ? '🎉 All Labs Mastered!' : `${solvedPercent}% Curriculum Completed`}
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
                  Vulnerabilities
                </span>
              </div>
              <div className="text-[10px] font-semibold text-[#0F766E]/90 font-['JetBrains_Mono']">
                100% Offline Deterministic
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
                {updateCount > 0 ? `${updateCount} Lab Update(s)` : 'AppContainer • Zero Cloud Leak'}
              </div>
            </div>

            <div className="w-12 h-12 rounded-xl bg-white/70 border-2 border-[#D8B4FE] flex items-center justify-center text-[#7C3AED] shadow-[0_2px_0_0_#D8B4FE]">
              <CheckCircle2 size={22} />
            </div>
          </div>
        </div>

        {/* 4. Search & Filter Bar with 3D Depth & Category Chips */}
        <div className="space-y-3 bg-[#FCFBF8] border-2 border-[#CEC8BF] rounded-2xl p-4 shadow-[0_3px_0_0_#E4DFD7]">
          {/* Top Row: Search Input + Status Tabs + Difficulty */}
          <div className="flex flex-col xl:flex-row xl:items-center justify-between gap-3">
            {/* Search Field */}
            <div className="relative flex-1 min-w-[280px]">
              <Search className="absolute left-3.5 top-3 text-[#9C9080]" size={16} />
              <input
                type="text"
                placeholder="Search by lab ID, title, target app, or attack skill (e.g. A01, IDOR, SQLi, VaultPay)..."
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
              {(['ALL', 'RUNNING', 'SOLVED', 'READY', 'UPDATE'] as StatusFilter[]).map((st) => {
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
                    {st === 'ALL' ? 'ALL STATUS' : st}
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

          {/* Bottom Row: Domain / Track Filter Chips with 3D Bevel */}
          <div className="pt-2 border-t-2 border-[#E6E2D8]/60 flex items-center gap-1.5 overflow-x-auto pb-1 scrollbar-none">
            <span className="text-[11px] font-['JetBrains_Mono'] font-bold text-[#786F62] uppercase tracking-wider shrink-0 mr-1 flex items-center gap-1">
              <Filter size={12} />
              <span>Domain:</span>
            </span>

            {TRACK_CHIPS.map((track) => {
              const isSelected = selectedCategory === track.id
              return (
                <button
                  key={track.id}
                  onClick={() => setSelectedCategory(track.id)}
                  className={`px-3 py-1 rounded-lg text-[11px] font-['Space_Grotesk'] font-bold border-2 transition-all cursor-pointer shrink-0 ${
                    isSelected
                      ? 'bg-[#1E1A14] text-white border-[#1E1A14] shadow-[0_2px_0_0_#000000] translate-y-0.5'
                      : 'bg-white border-[#CEC8BF] text-[#5C5347] shadow-[0_2px_0_0_#E4DFD7] hover:border-[#2DD4BF] hover:-translate-y-0.5'
                  }`}
                >
                  {track.name}
                </button>
              )
            })}
          </div>
        </div>

        {/* 5. Labs Grid / List */}
        {loading ? (
          <div className="py-16 flex items-center justify-center">
            <CuteAnimeLoading
              message="ENUMERATING LABS CATALOG..."
              subMessage="( ≧ᗜ≦) CYBER MATRIX LOADED"
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
              />
            ))}
          </div>
        )}
      </div>
    </div>
  )
}

// ─────────────────────────────────────────────────────────────────────────────
// Tactile 3D Lab Card with Rich Deep Shadows and Flutter Aesthetic
// ─────────────────────────────────────────────────────────────────────────────
interface TactileLabCardProps {
  lab: LabItem
  isSolved: boolean
  inProgress?: string
  onSelectLab: (id: string) => void
  onAction: (id: string, action: 'start' | 'stop' | 'reset' | 'update') => Promise<void>
}

function TactileLabCard({
  lab,
  isSolved,
  inProgress,
  onSelectLab,
  onAction,
}: TactileLabCardProps) {
  const [isHovered, setIsHovered] = useState(false)
  const detail = LAB_DETAILS[lab.id] ?? {
    targetApp: 'Corporate Portal Target',
    domain: 'Enterprise Security Assessment Target',
    desc: 'Offline interactive security engineering environment.',
    skills: ['Exploitation', 'Defense'],
    difficulty: 'Beginner',
  }
  const sticker = STICKER_MAP[lab.id] ?? '/assets/sticker_pink.png'
  const style = CATEGORY_STYLES[lab.id] ?? CATEGORY_STYLES['A01']
  const isRunning = lab.running

  // Dynamic Multi-layer 3D Box Shadow matching Flutter tactile specifications
  const computeShadow = () => {
    if (isRunning) {
      return isHovered
        ? '0 4px 0 0 #0F766E, 0 12px 24px rgba(45,212,191,0.32)'
        : '0 6px 0 0 #0F766E, 0 16px 32px rgba(45,212,191,0.25)'
    }
    return isHovered
      ? `0 3px 0 0 ${style.shadow}, 0 8px 18px ${style.glowRgba}`
      : `0 6px 0 0 ${style.shadow}, 0 14px 28px ${style.glowRgba}`
  }

  return (
    <div
      onMouseEnter={() => setIsHovered(true)}
      onMouseLeave={() => setIsHovered(false)}
      className="relative p-5 rounded-2xl border-2 flex flex-col justify-between overflow-hidden group select-none transition-all duration-200"
      style={{
        backgroundColor: isRunning ? style.bgLight : '#FCFBF8',
        borderColor: isRunning ? '#2DD4BF' : isHovered ? style.border : '#CEC8BF',
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
              backgroundColor: style.cartridgeBg,
              borderColor: style.cartridgeBorder,
              boxShadow: `0 3px 0 0 ${style.cartridgeShadow}`,
            }}
          >
            <span
              className="font-['JetBrains_Mono'] font-black text-lg leading-none"
              style={{ color: style.cartridgeText }}
            >
              {lab.id}
            </span>
            <span
              className="font-['JetBrains_Mono'] font-bold text-[8.5px] tracking-wider mt-0.5 opacity-90"
              style={{ color: style.cartridgeText }}
            >
              2025
            </span>
          </div>

          {/* Header Info */}
          <div className="flex-1 min-w-0 pr-8">
            <div className="flex flex-wrap items-center gap-1.5 mb-1.5">
              {/* Live Status Badge */}
              {isRunning ? (
                <span className="inline-flex items-center gap-1.5 px-2.5 py-0.5 rounded-full bg-[#DCFCE7] border-2 border-[#22C55E] text-[10px] font-['JetBrains_Mono'] font-extrabold text-[#15803D] shadow-[0_2px_0_0_#16A34A,inset_0_1px_0_0_rgba(255,255,255,0.8)]">
                  <span className="relative flex h-2 w-2">
                    <span className="animate-ping absolute inline-flex h-full w-full rounded-full bg-[#22C55E] opacity-75" />
                    <span className="relative inline-flex rounded-full h-2 w-2 bg-[#16A34A]" />
                  </span>
                  <span>RUNNING</span>
                </span>
              ) : (
                <span className="inline-flex items-center gap-1.5 px-2.5 py-0.5 rounded-full bg-[#F1F5F9] border-2 border-[#CBD5E1] text-[10px] font-['JetBrains_Mono'] font-bold text-[#475569] shadow-[0_2px_0_0_#94A3B8,inset_0_1px_0_0_rgba(255,255,255,0.8)]">
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
                <span className="px-2.5 py-0.5 rounded-full text-[10px] font-['JetBrains_Mono'] font-extrabold uppercase border-2 border-[#93C5FD] bg-[#EFF6FF] text-[#1D4ED8] shadow-[0_2px_0_0_#60A5FA,inset_0_1px_0_0_rgba(255,255,255,0.8)]">
                  BEGINNER
                </span>
              )}
              {detail.difficulty === 'Intermediate' && (
                <span className="px-2.5 py-0.5 rounded-full text-[10px] font-['JetBrains_Mono'] font-extrabold uppercase border-2 border-[#D8B4FE] bg-[#F5F3FF] text-[#6D28D9] shadow-[0_2px_0_0_#A855F7,inset_0_1px_0_0_rgba(255,255,255,0.8)]">
                  INTERMEDIATE
                </span>
              )}
              {detail.difficulty === 'Advanced' && (
                <span className="px-2.5 py-0.5 rounded-full text-[10px] font-['JetBrains_Mono'] font-extrabold uppercase border-2 border-[#FECDD3] bg-[#FFF1F2] text-[#BE123C] shadow-[0_2px_0_0_#FB7185,inset_0_1px_0_0_rgba(255,255,255,0.8)]">
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

              {/* Version */}
              <span className="px-1.5 py-0.5 rounded-md bg-[#F7F5F0] border border-[#E4DFD7] text-[9.5px] font-['JetBrains_Mono'] text-[#5C5347] font-bold">
                v{lab.version}
              </span>

              {/* Update Badge */}
              {lab.hasUpdate && (
                <span className="px-2 py-0.5 rounded bg-[#FEF3C7] border border-[#F59E0B] text-[9.5px] font-['JetBrains_Mono'] font-bold text-[#B45309] animate-pulse">
                  UPDATE v{lab.latestVersion}
                </span>
              )}
            </div>

            {/* Lab Title */}
            <h3
              onClick={() => onSelectLab(lab.id)}
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
          {isRunning ? (
            <>
              <button
                onClick={() => openBrowser(lab.url || `http://127.0.0.1:${lab.port}/`)}
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
          onClick={() => onSelectLab(lab.id)}
          className="px-3.5 py-2 bg-white hover:bg-[#F0FDFA] border-2 border-[#2DD4BF] text-[#0F766E] text-xs font-bold font-['Space_Grotesk'] rounded-xl shadow-[0_3px_0_0_#5EEAD4] hover:shadow-[0_1px_0_0_#5EEAD4] hover:translate-y-0.5 active:translate-y-1 active:shadow-none flex items-center gap-1.5 cursor-pointer transition-all"
        >
          <span>Inspect Lab</span>
          <ChevronRight size={14} />
        </button>
      </div>
    </div>
  )
}
