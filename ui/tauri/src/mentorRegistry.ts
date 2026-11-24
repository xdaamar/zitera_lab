// ZITERA CYBER_LAB — Multi-Mentor Character Intelligence Registry
// Features distinct cute anime cyber mentors (Chiyo, Kaede, Valerie) with personalized
// pedagogical personas, interactive multi-pose teaching animations, and thematic visual styles.
// Zeta is exclusively preserved as the lead player operative icon.

export type MentorId = 'chiyo' | 'kaede' | 'valerie' | 'zeta' | 'aoi' | 'elysia'

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

export const MENTOR_REGISTRY: Record<'chiyo' | 'kaede' | 'valerie', MentorProfile> = {
  chiyo: {
    id: 'chiyo',
    name: 'Chiyo',
    codename: 'CHIYO-SOCRATIC-01',
    role: 'Socratic Cyber Intel Mentor',
    specialty: 'Reconnaissance, Analisis Protokol & Socratic Logic',
    themeColor: '#2DD4BF',
    badgeBg: 'bg-teal-500/10 border-teal-500/30',
    badgeText: 'text-teal-300',
    accentBorder: 'border-teal-500/40',
    accentGlow: 'shadow-[0_0_24px_rgba(45,212,191,0.25)]',
    avatar: '/assets/mentors/chiyo_avatar.png',
    poses: {
      teaching: '/assets/mentors/chiyo_teaching.jpg',
      eureka: '/assets/mentors/chiyo_praise.jpg',
      thinking: '/assets/mentors/chiyo_teaching.jpg',
      warning: '/assets/mentors/chiyo_teaching.jpg',
    },
    greeting: (_labId, labTitle) =>
      `Salam taktis, Operative. Saya Chiyo, mentor intelijen siber Anda di ${labTitle}. Misi saya adalah melatih insting investigasi Anda melalui bimbingan Sokrates—mengajarkan cara berpikir dan menemukan celah tanpa membocorkan jawaban langsung. Bagaimana progres pemetaan Anda di target ini?`,
    styleSummary: 'Pendekatan analisis terstruktur, membimbing lewat pertanyaan kritis dan pemodelan ancaman bertahap.',
    quote: '"Seorang hacker sejati bukan yang hafal payload, melainkan yang memahami cara kerja sistem."',
  },
  kaede: {
    id: 'kaede',
    name: 'Kaede',
    codename: 'KAEDE-EXPLOIT-02',
    role: 'Offensive Web Exploitation Specialist',
    specialty: 'Web Payloads, Input Validation Bypass & Burp Repeater Mastery',
    themeColor: '#A855F7',
    badgeBg: 'bg-purple-500/10 border-purple-500/30',
    badgeText: 'text-purple-300',
    accentBorder: 'border-purple-500/40',
    accentGlow: 'shadow-[0_0_24px_rgba(168,85,247,0.25)]',
    avatar: '/assets/mentors/kaede_avatar.png',
    poses: {
      teaching: '/assets/mentors/kaede_teaching.jpg',
      eureka: '/assets/mentors/kaede_praise.jpg',
      thinking: '/assets/mentors/kaede_teaching.jpg',
      warning: '/assets/mentors/kaede_teaching.jpg',
    },
    greeting: (_labId, labTitle) =>
      `Hei Operative! Aku Kaede, spesialis eksploitasi web kamu untuk modul ${labTitle}. Mau bedah request payload, parameter tersembunyi, atau trik bypass proteksi? Tanya aku apa saja, tapi ingat ya: aku bakal kasih clue tajam biar kamu sendiri yang merasakan serunya membobol celah ini!`,
    styleSummary: 'Praktis dan berenergi tinggi, fokus pada eksperimen payload langsung dan manipulasi parameter.',
    quote: '"Jika aplikasi web mengira input kita aman, mari kita buktikan sebaliknya!"',
  },
  valerie: {
    id: 'valerie',
    name: 'Valerie',
    codename: 'VALERIE-DEFENSE-03',
    role: 'Secure Architecture & Defensive Code Auditor',
    specialty: 'White-Box Code Audit, Root Cause Analysis & ISO 27001 Patching',
    themeColor: '#F59E0B',
    badgeBg: 'bg-amber-500/10 border-amber-500/30',
    badgeText: 'text-amber-300',
    accentBorder: 'border-amber-500/40',
    accentGlow: 'shadow-[0_0_24px_rgba(245,158,11,0.25)]',
    avatar: '/assets/mentors/valerie_avatar.png',
    poses: {
      teaching: '/assets/mentors/valerie_teaching.jpg',
      eureka: '/assets/mentors/valerie_praise.jpg',
      thinking: '/assets/mentors/valerie_teaching.jpg',
      warning: '/assets/mentors/valerie_teaching.jpg',
    },
    greeting: (_labId, labTitle) =>
      `Selamat datang, Operative. Saya Valerie, mentor arsitektur pertahanan Anda. Di modul ${labTitle}, kita akan meneliti bagaimana celah keamanan muncul dari logika kode backend dan bagaimana mengeksploitasinya secara terarah untuk kemudian memperbaikinya. Mari kita bedah bersama dengan sabar dan teliti.`,
    styleSummary: 'Elegan dan penuh empati, menekankan pemahaman akar masalah (root cause) dan arsitektur pengamanan.',
    quote: '"Menyerang mengajarkan kita di mana batas sistem berada; bertahan memastikan batas itu tidak roboh."',
  },
}

export const MENTOR_LIST: MentorProfile[] = Object.values(MENTOR_REGISTRY)

export function getMentor(id: MentorId): MentorProfile {
  if (id === 'zeta') return MENTOR_REGISTRY.chiyo
  if (id === 'aoi') return MENTOR_REGISTRY.kaede
  if (id === 'elysia') return MENTOR_REGISTRY.valerie
  return MENTOR_REGISTRY[id as 'chiyo' | 'kaede' | 'valerie'] || MENTOR_REGISTRY.chiyo
}
