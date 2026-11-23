// ZITERA CYBER_LAB — Multi-Mentor Character Intelligence Registry
// Supports multiple distinct anime mentors with personalized pedagogical personas,
// interactive multi-pose teaching animations, and thematic visual styles.

export type MentorId = 'zeta' | 'aoi' | 'elysia'

export interface MentorProfile {
  id: MentorId
  name: string
  codename: string
  role: string
  specialty: string
  themeColor: string
  badgeBg: string
  badgeText: string
  accentBorder: string
  accentGlow: string
  avatar: string
  poses: {
    teaching: string
    eureka: string
    thinking: string
    warning: string
  }
  greeting: (labId: string, labTitle: string) => string
  styleSummary: string
  quote: string
}

export const MENTOR_REGISTRY: Record<MentorId, MentorProfile> = {
  zeta: {
    id: 'zeta',
    name: 'Zeta',
    codename: 'ZETA-SOCRATIC-01',
    role: 'Socratic Cyber Intel Mentor',
    specialty: 'Reconnaissance, Analisis Protokol & Socratic Logic',
    themeColor: '#2DD4BF',
    badgeBg: 'bg-teal-500/10 border-teal-500/30',
    badgeText: 'text-teal-300',
    accentBorder: 'border-teal-500/40',
    accentGlow: 'shadow-[0_0_24px_rgba(45,212,191,0.25)]',
    avatar: '/assets/mentors/zeta_avatar.png',
    poses: {
      teaching: '/assets/mentors/zeta_teaching.jpg',
      eureka: '/assets/mentors/zeta_praise.jpg',
      thinking: '/assets/mentors/zeta_teaching.jpg',
      warning: '/assets/mentors/zeta_teaching.jpg',
    },
    greeting: (_labId, labTitle) =>
      `Salam taktis, Operative. Saya Zeta, mentor intelijen siber Anda di ${labTitle}. Misi saya adalah melatih insting investigasi Anda melalui bimbingan Sokrates—mengajarkan cara berpikir dan menemukan celah tanpa membocorkan jawaban langsung. Bagaimana progres pemetaan Anda di target ini?`,
    styleSummary: 'Pendekatan analisis terstruktur, membimbing lewat pertanyaan kritis dan pemodelan ancaman bertahap.',
    quote: '"Seorang hacker sejati bukan yang hafal payload, melainkan yang memahami cara kerja sistem."',
  },
  aoi: {
    id: 'aoi',
    name: 'Aoi',
    codename: 'AOI-EXPLOIT-02',
    role: 'Offensive Web Exploitation Specialist',
    specialty: 'Web Payloads, Input Validation Bypass & Burp Repeater Mastery',
    themeColor: '#A855F7',
    badgeBg: 'bg-purple-500/10 border-purple-500/30',
    badgeText: 'text-purple-300',
    accentBorder: 'border-purple-500/40',
    accentGlow: 'shadow-[0_0_24px_rgba(168,85,247,0.25)]',
    avatar: '/assets/mentors/aoi_avatar.png',
    poses: {
      teaching: '/assets/mentors/aoi_teaching.jpg',
      eureka: '/assets/mentors/aoi_praise.jpg',
      thinking: '/assets/mentors/aoi_teaching.jpg',
      warning: '/assets/mentors/aoi_teaching.jpg',
    },
    greeting: (_labId, labTitle) =>
      `Hei Operative! Aku Aoi, spesialis eksploitasi web kamu untuk modul ${labTitle}. Mau bedah request payload, parameter tersembunyi, atau trik bypass proteksi? Tanya aku apa saja, tapi ingat ya: aku bakal kasih clue tajam biar kamu sendiri yang merasakan serunya membobol celah ini!`,
    styleSummary: 'Praktis dan berenergi tinggi, fokus pada eksperimen payload langsung dan manipulasi parameter.',
    quote: '"Jika aplikasi web mengira input kita aman, mari kita buktikan sebaliknya!"',
  },
  elysia: {
    id: 'elysia',
    name: 'Elysia',
    codename: 'ELYSIA-DEFENSE-03',
    role: 'Secure Architecture & Defensive Code Auditor',
    specialty: 'Code Review, Root Cause Analysis & Defense-in-Depth',
    themeColor: '#F59E0B',
    badgeBg: 'bg-amber-500/10 border-amber-500/30',
    badgeText: 'text-amber-300',
    accentBorder: 'border-amber-500/40',
    accentGlow: 'shadow-[0_0_24px_rgba(245,158,11,0.25)]',
    avatar: '/assets/mentors/elysia_avatar.png',
    poses: {
      teaching: '/assets/mentors/elysia_teaching.jpg',
      eureka: '/assets/mentors/elysia_praise.jpg',
      thinking: '/assets/mentors/elysia_teaching.jpg',
      warning: '/assets/mentors/elysia_teaching.jpg',
    },
    greeting: (_labId, labTitle) =>
      `Selamat datang, Operative. Saya Elysia, mentor arsitektur pertahanan Anda. Di modul ${labTitle}, kita akan meneliti bagaimana celah keamanan muncul dari logika kode backend dan bagaimana mengeksploitasinya secara terarah untuk kemudian memperbaikinya. Mari kita bedah bersama dengan sabar dan teliti.`,
    styleSummary: 'Elegan dan penuh empati, menekankan pemahaman akar masalah (root cause) dan arsitektur pengamanan.',
    quote: '"Menyerang mengajarkan kita di mana batas sistem berada; bertahan memastikan batas itu tidak roboh."',
  },
}

export const MENTOR_LIST: MentorProfile[] = Object.values(MENTOR_REGISTRY)

export function getMentor(id: MentorId): MentorProfile {
  return MENTOR_REGISTRY[id] || MENTOR_REGISTRY.zeta
}
