// 14 Zitera Operative Avatars with roles, expressions, and gender classifications

export interface AvatarDefinition {
  id: string
  name: string
  gender: 'Operator' | 'Mascot' | 'Female' | 'Male'
  role: string
  expression: string
  assetPath: string
}

export const AVATAR_REGISTRY: AvatarDefinition[] = [
  // ── Classics ──
  {
    id: 'zeta',
    name: 'Zeta',
    gender: 'Operator',
    role: 'Lead Operator',
    expression: 'Operator Classic',
    assetPath: '/assets/zeta_avatar.png',
  },
  {
    id: 'neko',
    name: 'Neko',
    gender: 'Mascot',
    role: 'Cyber Mascot',
    expression: 'Calico Guardian',
    assetPath: '/assets/calico_cat.png',
  },

  // ── Operatives: Girls ──
  {
    id: 'girl_cheerful',
    name: 'Aoi',
    gender: 'Female',
    role: 'Stealth Recon',
    expression: 'Cheerful / Ceria',
    assetPath: '/assets/avatars/girl_cheerful.png',
  },
  {
    id: 'girl_cool',
    name: 'Rin',
    gender: 'Female',
    role: 'Exploit Dev',
    expression: 'Cool / Tactical',
    assetPath: '/assets/avatars/girl_cool.png',
  },
  {
    id: 'girl_smug',
    name: 'Mia',
    gender: 'Female',
    role: 'Web Infiltration',
    expression: 'Smug / Tengil',
    assetPath: '/assets/avatars/girl_smug.png',
  },
  {
    id: 'girl_serious',
    name: 'Hana',
    gender: 'Female',
    role: 'Threat Intelligence',
    expression: 'Serious / Analyst',
    assetPath: '/assets/avatars/girl_serious.png',
  },
  {
    id: 'girl_pouty',
    name: 'Yuki',
    gender: 'Female',
    role: 'Cryptanalysis',
    expression: 'Pouty / Tsundere',
    assetPath: '/assets/avatars/girl_pouty.png',
  },
  {
    id: 'girl_fierce',
    name: 'Luna',
    gender: 'Female',
    role: 'Cloud & DevSecOps',
    expression: 'Fierce / Ambisius',
    assetPath: '/assets/avatars/girl_fierce.png',
  },

  // ── Operatives: Boys ──
  {
    id: 'boy_cheerful',
    name: 'Ren',
    gender: 'Male',
    role: 'Reverse Engineering',
    expression: 'Energetic / Enerjik',
    assetPath: '/assets/avatars/boy_cheerful.png',
  },
  {
    id: 'boy_cool',
    name: 'Kaito',
    gender: 'Male',
    role: 'System Forensics',
    expression: 'Hacker Hoodie / Cool',
    assetPath: '/assets/avatars/boy_cool.png',
  },
  {
    id: 'boy_smug',
    name: 'Shin',
    gender: 'Male',
    role: 'Kernel Exploitation',
    expression: 'Smirk / Smug',
    assetPath: '/assets/avatars/boy_smug.png',
  },
  {
    id: 'boy_serious',
    name: 'Haru',
    gender: 'Male',
    role: 'Hardware & Radio',
    expression: 'Tactical Visor',
    assetPath: '/assets/avatars/boy_serious.png',
  },
  {
    id: 'boy_pouty',
    name: 'Sora',
    gender: 'Male',
    role: 'Network Operative',
    expression: 'Tsundere / Focused',
    assetPath: '/assets/avatars/boy_pouty.png',
  },
  {
    id: 'boy_fierce',
    name: 'Kai',
    gender: 'Male',
    role: 'Binary Exploitation',
    expression: 'Competitive / Fierce',
    assetPath: '/assets/avatars/boy_fierce.png',
  },
]

const _MAP = new Map<string, AvatarDefinition>()
AVATAR_REGISTRY.forEach((a) => {
  _MAP.set(a.id.toLowerCase(), a)
  _MAP.set(a.name.toLowerCase(), a)
})

export function getAvatarById(id?: string | null): AvatarDefinition {
  if (!id) return AVATAR_REGISTRY[0]
  const key = id.trim().toLowerCase()
  return _MAP.get(key) ?? AVATAR_REGISTRY[0]
}

export function getAvatarAsset(id?: string | null): string {
  return getAvatarById(id).assetPath
}
