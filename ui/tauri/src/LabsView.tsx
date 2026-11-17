import { useState, useEffect, useRef } from 'react'
import {
  Search,
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

type StatusFilter = 'ALL' | 'RUNNING' | 'READY' | 'UPDATE'
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
}

const LAB_DETAILS: Record<string, { desc: string; skills: string[]; difficulty: 'Beginner' | 'Intermediate' | 'Advanced' }> = {
  A01: {
    desc: 'Investigate Insecure Direct Object References (IDOR) and bypass horizontal authorization controls in a mock banking API.',
    skills: ['IDOR', 'BOLA', 'Access Control', 'API Security'],
    difficulty: 'Beginner',
  },
  A02: {
    desc: 'Exploit verbose stack traces, default administrative credentials, and exposed diagnostics endpoints in OpsGateway.',
    skills: ['Misconfiguration', 'Directory Listing', 'Debug Leaks', 'CORS'],
    difficulty: 'Beginner',
  },
  A03: {
    desc: 'Analyze compromised upstream dependencies, poisoned package manifests, and insecure package resolution pipelines.',
    skills: ['Supply Chain', 'Dependency Confusion', 'Integrity Checking'],
    difficulty: 'Beginner',
  },
  A04: {
    desc: 'Exploit unsanitized user input in cryptographic authentication hashes and race conditions in financial ledger updates.',
    skills: ['Crypto Flaws', 'Timing Attacks', 'Insecure Hashes'],
    difficulty: 'Intermediate',
  },
  A05: {
    desc: 'Perform classic SQL injection, boolean-based inference, and command injection across legacy backend query handlers.',
    skills: ['SQL Injection', 'Blind SQLi', 'Input Sanitization'],
    difficulty: 'Intermediate',
  },
  A06: {
    desc: 'Audit vulnerable microservice endpoints lacking integrity checks and exploit insecure deserialization vulnerabilities.',
    skills: ['Deserialization', 'Object Injection', 'Memory Corrupt'],
    difficulty: 'Intermediate',
  },
  A07: {
    desc: 'Bypass broken multi-factor authentication, exploit session fixation, and hijack persistent JWT tokens.',
    skills: ['Session Hijacking', 'JWT Tampering', 'Brute Force Defense'],
    difficulty: 'Intermediate',
  },
  A08: {
    desc: 'Perform Server-Side Request Forgery (SSRF) to pivot into internal cloud metadata services and pivot through private subnets.',
    skills: ['SSRF', 'Cloud Metadata', 'Internal Port Scan'],
    difficulty: 'Advanced',
  },
  A09: {
    desc: 'Investigate blind log injection, audit trail evasion, and insufficient security monitoring during anomalous attack traffic.',
    skills: ['Log Injection', 'Audit Evasion', 'Incident Response'],
    difficulty: 'Advanced',
  },
  A10: {
    desc: 'Weaponize complex server-side template injection (SSTI) and remote code execution across edge renderers.',
    skills: ['SSTI', 'RCE', 'Sandbox Escape', 'Advanced Exploitation'],
    difficulty: 'Advanced',
  },
}

export function LabsView({ onSelectLab }: LabsViewProps) {
  const [labs, setLabs] = useState<LabItem[]>([])
  const [search, setSearch] = useState('')
  const [statusFilter, setStatusFilter] = useState<StatusFilter>('ALL')
  const [difficulty, setDifficulty] = useState<DifficultyFilter>('ALL')
  const [loading, setLoading] = useState(true)
  const [actionInProgress, setActionInProgress] = useState<Record<string, string>>({})
  const isMountedRef = useRef(true)

  const loadLabs = async () => {
    setLoading(true)
    try {
      const data = await getLabs()
      if (isMountedRef.current) {
        setLabs(data)
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
    return () => {
      isMountedRef.current = false
    }
  }, [])

  const handleAction = async (labId: string, action: 'start' | 'stop' | 'reset' | 'update') => {
    setActionInProgress((prev) => ({ ...prev, [labId]: action }))
    try {
      if (action === 'start') await startLab(labId)
      else if (action === 'stop') await stopLab(labId)
      else if (action === 'reset') await resetLab(labId)
      else if (action === 'update') await updateLab(labId)
      await loadLabs()
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

  const filteredLabs = labs.filter((lab) => {
    const q = search.trim().toLowerCase()
    const detail = LAB_DETAILS[lab.id]
    const matchesSearch =
      !q ||
      lab.id.toLowerCase().includes(q) ||
      lab.title.toLowerCase().includes(q) ||
      (detail && detail.skills.some((s) => s.toLowerCase().includes(q))) ||
      (detail && detail.desc.toLowerCase().includes(q)) ||
      lab.recommendedTools.some((t) => t.toLowerCase().includes(q))

    const matchesStatus =
      statusFilter === 'ALL' ||
      (statusFilter === 'RUNNING' && lab.running) ||
      (statusFilter === 'READY' && lab.installed && !lab.running) ||
      (statusFilter === 'UPDATE' && lab.hasUpdate)

    const labDiff = detail ? detail.difficulty : 'Beginner'
    const matchesDifficulty = difficulty === 'ALL' || difficulty === labDiff

    return matchesSearch && matchesStatus && matchesDifficulty
  })

  return (
    <div className="flex-1 h-full overflow-hidden relative bg-[#FCFBF8]">
      {/* 1. Pinned Ambient Background Artwork Texture (Matches Flutter bg_labs.jpg) */}
      <div className="absolute inset-0 pointer-events-none z-0 overflow-hidden select-none">
        <img
          src="/assets/bg_labs.jpg"
          alt=""
          className="w-full h-full object-cover object-center opacity-22"
          onError={(e) => {
            ;(e.target as HTMLElement).style.display = 'none'
          }}
        />
        <div className="absolute inset-0 bg-[#FCFBF8]/80" />
      </div>

      <div className="relative z-10 h-full overflow-y-auto p-6 space-y-6">
        {/* 2. Top Header & Primary Controls */}
        <div className="flex flex-col lg:flex-row lg:items-center justify-between gap-4 border-b-2 border-[#E6E2D8] pb-5">
          <div>
            <div className="flex items-center gap-2 mb-1">
              <span className="px-2.5 py-0.5 rounded-md bg-[#CCFBF1] text-[#0F766E] border-2 border-[#2DD4BF] text-[10px] font-['Space_Grotesk'] font-bold flex items-center gap-1.5 shadow-[0_2px_0_0_#99F6E4]">
                <Wifi size={12} className="animate-pulse" />
                <span>DYNAMIC CATALOG // ONLINE</span>
              </span>
              <span className="text-[11px] font-['JetBrains_Mono'] text-[#9C9080] font-semibold">
                OWASP TOP 10:2025 STANDARD
              </span>
            </div>
            <h1 className="text-2xl font-black font-['Space_Grotesk'] text-[#1E1A14] tracking-tight">
              SECURITY LABS CATALOG
            </h1>
            <p className="text-xs text-[#5C5347] font-sans mt-0.5">
              Isolated offline vulnerability training environments with deterministic runtime targets ({labs.length} laboratories available)
            </p>
          </div>

          <div className="flex items-center gap-3">
            <button
              onClick={loadLabs}
              className="px-4 py-2 bg-white text-[#0F766E] hover:bg-[#F0FDFA] border-2 border-[#2DD4BF] shadow-[0_3px_0_0_#5EEAD4] hover:shadow-[0_1px_0_0_#5EEAD4] hover:translate-y-0.5 active:translate-y-1 rounded-lg text-xs font-bold font-['Space_Grotesk'] flex items-center gap-2 cursor-pointer transition-all"
            >
              <RefreshCw size={14} className={loading ? 'animate-spin' : ''} />
              <span>Refresh Catalog</span>
            </button>
          </div>
        </div>

        {/* 3. Metric Badges Ribbon */}
        <div className="grid grid-cols-2 sm:grid-cols-4 gap-3">
          <div className="p-3 bg-[#FCFBF8] border-2 border-[#CEC8BF] rounded-xl shadow-[0_2px_0_0_#E4DFD7] flex items-center gap-3">
            <div className="w-8 h-8 rounded-lg bg-[#CCFBF1] border border-[#2DD4BF] flex items-center justify-center text-[#0F766E]">
              <ShieldCheck size={18} />
            </div>
            <div>
              <div className="text-[10px] font-['JetBrains_Mono'] text-[#9C9080] uppercase">Total Labs</div>
              <div className="text-sm font-bold font-['JetBrains_Mono'] text-[#1E1A14]">{labs.length} Targets</div>
            </div>
          </div>

          <div className="p-3 bg-[#FCFBF8] border-2 border-[#CEC8BF] rounded-xl shadow-[0_2px_0_0_#E4DFD7] flex items-center gap-3">
            <div className="w-8 h-8 rounded-lg bg-[#DCFCE7] border border-[#86EFAC] flex items-center justify-center text-[#16A34A]">
              <Zap size={18} />
            </div>
            <div>
              <div className="text-[10px] font-['JetBrains_Mono'] text-[#9C9080] uppercase">Active Targets</div>
              <div className="text-sm font-bold font-['JetBrains_Mono'] text-[#16A34A]">{runningCount} Running</div>
            </div>
          </div>

          <div className="p-3 bg-[#FCFBF8] border-2 border-[#CEC8BF] rounded-xl shadow-[0_2px_0_0_#E4DFD7] flex items-center gap-3">
            <div className="w-8 h-8 rounded-lg bg-[#EDE9FE] border border-[#D8B4FE] flex items-center justify-center text-[#6D28D9]">
              <Sparkles size={18} />
            </div>
            <div>
              <div className="text-[10px] font-['JetBrains_Mono'] text-[#9C9080] uppercase">Containment</div>
              <div className="text-sm font-bold font-['JetBrains_Mono'] text-[#1E1A14]">Win32 Sandbox</div>
            </div>
          </div>

          <div className="p-3 bg-[#FCFBF8] border-2 border-[#CEC8BF] rounded-xl shadow-[0_2px_0_0_#E4DFD7] flex items-center gap-3">
            <div className="w-8 h-8 rounded-lg bg-[#FEF3C7] border border-[#FCD34D] flex items-center justify-center text-[#B45309]">
              <DownloadCloud size={18} />
            </div>
            <div>
              <div className="text-[10px] font-['JetBrains_Mono'] text-[#9C9080] uppercase">Updates</div>
              <div className="text-sm font-bold font-['JetBrains_Mono'] text-[#B45309]">{updateCount > 0 ? `${updateCount} Available` : 'Up to Date'}</div>
            </div>
          </div>
        </div>

        {/* 4. Search & Filter Bar */}
        <div className="flex flex-col md:flex-row md:items-center justify-between gap-3 bg-[#FCFBF8] border-2 border-[#CEC8BF] rounded-xl p-3 shadow-[0_3px_0_0_#E4DFD7]">
          {/* Search Field */}
          <div className="relative flex-1 max-w-md">
            <Search className="absolute left-3 top-2.5 text-[#9C9080]" size={16} />
            <input
              type="text"
              placeholder="Search labs by name, ID, attack skill (e.g. A01, IDOR, SQLi)..."
              value={search}
              onChange={(e) => setSearch(e.target.value)}
              className="w-full pl-9 pr-4 py-2 text-xs bg-[#F7F5F0] border-2 border-[#E4DFD7] rounded-lg focus:outline-none focus:border-[#2DD4BF] focus:bg-white font-['Space_Grotesk'] text-[#1E1A14] transition-all"
            />
          </div>

          {/* Status Filter Tabs */}
          <div className="flex flex-wrap items-center gap-1.5">
            {(['ALL', 'RUNNING', 'READY', 'UPDATE'] as StatusFilter[]).map((st) => {
              const active = statusFilter === st
              return (
                <button
                  key={st}
                  onClick={() => setStatusFilter(st)}
                  className={`px-3 py-1.5 rounded-lg text-xs font-bold font-['Space_Grotesk'] border-2 transition-all cursor-pointer ${
                    active
                      ? 'bg-[#CCFBF1] border-[#2DD4BF] text-[#0F766E] shadow-[0_2px_0_0_#5EEAD4]'
                      : 'bg-[#F7F5F0] border-[#E4DFD7] text-[#5C5347] hover:border-[#2DD4BF] hover:text-[#0F766E]'
                  }`}
                >
                  {st === 'ALL' ? 'ALL STATUS' : st}
                </button>
              )
            })}

            {/* Difficulty Dropdown / Tabs */}
            <div className="h-6 w-px bg-[#E6E2D8] mx-1 hidden sm:block" />

            <div className="flex items-center bg-[#F0EDE8] p-0.5 rounded-lg border-2 border-[#E4DFD7]">
              {(['ALL', 'Beginner', 'Intermediate', 'Advanced'] as DifficultyFilter[]).map((d) => (
                <button
                  key={d}
                  onClick={() => setDifficulty(d)}
                  className={`px-2.5 py-1 text-[11px] font-['Space_Grotesk'] font-bold rounded-md transition-all cursor-pointer ${
                    difficulty === d
                      ? 'bg-white text-[#0F766E] shadow-xs'
                      : 'text-[#5C5347] hover:text-[#1E1A14]'
                  }`}
                >
                  {d}
                </button>
              ))}
            </div>
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
              Try changing the search query or resetting the difficulty and status filters.
            </p>
            <button
              onClick={() => {
                setSearch('')
                setStatusFilter('ALL')
                setDifficulty('ALL')
              }}
              className="mt-4 px-4 py-2 bg-[#CCFBF1] text-[#0F766E] border-2 border-[#2DD4BF] shadow-[0_2px_0_0_#5EEAD4] rounded-lg text-xs font-bold font-['Space_Grotesk'] hover:translate-y-0.5 cursor-pointer transition-all"
            >
              Reset All Filters
            </button>
          </div>
        ) : (
          <div className="grid grid-cols-1 xl:grid-cols-2 gap-4">
            {filteredLabs.map((lab) => {
              const inProgress = actionInProgress[lab.id]
              const detail = LAB_DETAILS[lab.id] ?? {
                desc: 'Offline interactive security engineering environment.',
                skills: ['Exploitation', 'Defense'],
                difficulty: 'Beginner',
              }
              const sticker = STICKER_MAP[lab.id] ?? '/assets/sticker_pink.png'

              return (
                <div
                  key={lab.id}
                  className={`relative p-5 rounded-2xl border-2 transition-all flex flex-col justify-between overflow-hidden ${
                    lab.running
                      ? 'border-[#2DD4BF] shadow-[0_4px_0_0_#2DD4BF,0_10px_24px_rgba(45,212,191,0.22)] bg-[#F0FDFA]'
                      : 'border-[#CEC8BF] shadow-[0_4px_0_0_#E4DFD7] hover:border-[#2DD4BF] hover:shadow-[0_4px_0_0_#2DD4BF] hover:-translate-y-1 bg-[#FCFBF8]'
                  }`}
                >
                  {/* Peeking Cute Animal / Mascot Sticker on Top-Right */}
                  <div className="absolute top-2.5 right-3 w-10 h-10 pointer-events-none opacity-85 select-none hover:scale-110 transition-transform">
                    <img
                      src={sticker}
                      alt=""
                      className="w-full h-full object-contain drop-shadow-sm"
                      onError={(e) => {
                        ;(e.target as HTMLElement).style.display = 'none'
                      }}
                    />
                  </div>

                  <div>
                    {/* Top Row: 60x60 ID Box + Title & Badges */}
                    <div className="flex items-start gap-4">
                      {/* Cartridge ID Block */}
                      <div className="w-14 h-14 rounded-xl bg-[#E6FFFA] border-2 border-[#2DD4BF] shadow-[0_2px_6px_rgba(45,212,191,0.3)] flex flex-col items-center justify-center shrink-0">
                        <span className="font-['JetBrains_Mono'] font-black text-lg text-[#0F766E] leading-none">
                          {lab.id}
                        </span>
                        <span className="font-['JetBrains_Mono'] font-bold text-[9px] text-[#0D9488] tracking-widest mt-0.5">
                          2025
                        </span>
                      </div>

                      {/* Header Info */}
                      <div className="flex-1 min-w-0 pr-8">
                        <div className="flex flex-wrap items-center gap-1.5 mb-1">
                          {/* Live Status Badge */}
                          {lab.running ? (
                            <span className="inline-flex items-center gap-1.5 px-2 py-0.5 rounded-full bg-[#DCFCE7] border border-[#86EFAC] text-[10px] font-['JetBrains_Mono'] font-bold text-[#16A34A] shadow-xs">
                              <span className="w-1.5 h-1.5 rounded-full bg-[#16A34A] animate-ping" />
                              RUNNING
                            </span>
                          ) : (
                            <span className="px-2 py-0.5 rounded-full bg-[#F0EDE8] border border-[#E6E2D8] text-[10px] font-['JetBrains_Mono'] text-[#5C5347] font-semibold">
                              READY
                            </span>
                          )}

                          {/* Version */}
                          <span className="px-1.5 py-0.5 rounded bg-[#F7F5F0] border border-[#E4DFD7] text-[10px] font-['JetBrains_Mono'] text-[#5C5347] font-bold">
                            v{lab.version}
                          </span>

                          {/* Difficulty Tag */}
                          <span
                            className={`px-2 py-0.5 rounded border text-[10px] font-['JetBrains_Mono'] font-bold uppercase ${
                              detail.difficulty === 'Beginner'
                                ? 'bg-[#EFF6FF] border-[#BFDBFE] text-[#1D4ED8]'
                                : detail.difficulty === 'Intermediate'
                                ? 'bg-[#EDE9FE] border-[#D8B4FE] text-[#6D28D9]'
                                : 'bg-[#FEF3C7] border-[#FCD34D] text-[#B45309]'
                            }`}
                          >
                            {detail.difficulty}
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
                            className="px-1.5 py-0.5 rounded bg-[#E6FFFA] border border-[#2DD4BF]/50 text-[9.5px] font-['JetBrains_Mono'] text-[#0F766E] font-medium"
                          >
                            #{sk}
                          </span>
                        ))}
                      </div>
                    </div>
                  </div>

                  {/* Bottom Action Controls */}
                  <div className="pt-3.5 mt-3.5 border-t-2 border-[#E6E2D8] flex flex-wrap items-center justify-between gap-2">
                    <div className="flex items-center gap-2">
                      {lab.running ? (
                        <>
                          <button
                            onClick={() => openBrowser(`http://127.0.0.1:${lab.port}`)}
                            className="px-3.5 py-1.5 bg-[#0F766E] hover:bg-[#115E59] text-white text-xs font-bold rounded-lg flex items-center gap-1.5 border-2 border-[#094D47] shadow-[0_2px_0_0_#094D47] hover:translate-y-0.5 active:translate-y-1 transition-all cursor-pointer"
                          >
                            <span>Buka Web</span>
                            <ExternalLink size={13} />
                          </button>
                          <button
                            onClick={() => handleAction(lab.id, 'stop')}
                            disabled={!!inProgress}
                            className="px-3 py-1.5 bg-white border-2 border-[#EF4444] text-[#EF4444] hover:bg-[#FEE2E2] text-xs font-bold rounded-lg flex items-center gap-1 shadow-[0_2px_0_0_#FCA5A5] hover:translate-y-0.5 active:translate-y-1 transition-all disabled:opacity-50 cursor-pointer"
                          >
                            <Square size={12} />
                            <span>Stop</span>
                          </button>
                          <button
                            onClick={() => handleAction(lab.id, 'reset')}
                            disabled={!!inProgress}
                            className="p-1.5 bg-white border-2 border-[#CEC8BF] hover:border-[#2DD4BF] text-[#5C5347] rounded-lg shadow-[0_2px_0_0_#E4DFD7] hover:translate-y-0.5 active:translate-y-1 transition-all cursor-pointer"
                            title="Reset target database & files"
                          >
                            <RotateCcw size={14} />
                          </button>
                        </>
                      ) : (
                        <button
                          onClick={() => handleAction(lab.id, 'start')}
                          disabled={!!inProgress}
                          className="px-4 py-1.5 bg-[#0F766E] hover:bg-[#115E59] text-white text-xs font-bold rounded-lg flex items-center gap-1.5 border-2 border-[#094D47] shadow-[0_2px_0_0_#094D47] hover:translate-y-0.5 active:translate-y-1 transition-all disabled:opacity-50 cursor-pointer"
                        >
                          <Play size={13} />
                          <span>{inProgress === 'start' ? 'Launching...' : 'RUN LAB'}</span>
                        </button>
                      )}

                      {lab.hasUpdate && (
                        <button
                          onClick={() => handleAction(lab.id, 'update')}
                          disabled={!!inProgress}
                          className="px-2.5 py-1.5 bg-[#FEF3C7] border-2 border-[#FCD34D] text-[#B45309] hover:bg-[#FDE68A] text-xs font-bold rounded-lg flex items-center gap-1 shadow-[0_2px_0_0_#FCD34D] hover:translate-y-0.5 active:translate-y-1 transition-all cursor-pointer"
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
                      className="px-3 py-1.5 bg-white hover:bg-[#F0FDFA] border-2 border-[#2DD4BF] text-[#0F766E] text-xs font-bold font-['Space_Grotesk'] rounded-lg shadow-[0_2px_0_0_#5EEAD4] hover:shadow-[0_1px_0_0_#5EEAD4] hover:translate-y-0.5 active:translate-y-1 flex items-center gap-1.5 cursor-pointer transition-all"
                    >
                      <span>Inspect Lab</span>
                      <ChevronRight size={14} />
                    </button>
                  </div>
                </div>
              )
            })}
          </div>
        )}
      </div>
    </div>
  )
}
