import { useEffect, useState, useRef } from 'react'
import {
  ShieldCheck,
  FlaskConical,
  Trophy,
  Wrench,
  Play,
  ExternalLink,
  ArrowRight,
  RefreshCw,
} from 'lucide-react'
import {
  getLabs,
  getTools,
  checkDoctor,
  openBrowser,
  startLab,
  type LabItem,
  type ToolItem,
  type DiagnosticsResult,
} from './engineClient'
import { getLeaderboard, type ScoreboardEntry } from './scoreboardService'
import { CuteAnimeLoading } from './CuteAnimeLoading'

interface DashboardViewProps {
  onNavigate: (index: number) => void
  onOpenLab: (labId: string) => void
}

export function DashboardView({ onNavigate, onOpenLab }: DashboardViewProps) {
  const [labs, setLabs] = useState<LabItem[]>([])
  const [tools, setTools] = useState<ToolItem[]>([])
  const [diagnostics, setDiagnostics] = useState<DiagnosticsResult | null>(null)
  const [leaderboard, setLeaderboard] = useState<ScoreboardEntry[]>([])
  const [loading, setLoading] = useState(true)
  const [startingLabId, setStartingLabId] = useState<string | null>(null)
  const isMountedRef = useRef(true)

  const loadData = async () => {
    setLoading(true)
    try {
      const [l, t, d, lb] = await Promise.all([
        getLabs(),
        getTools(),
        checkDoctor(),
        getLeaderboard().catch(() => []),
      ])
      if (isMountedRef.current) {
        setLabs(l)
        setTools(t)
        setDiagnostics(d)
        setLeaderboard(lb)
      }
    } finally {
      if (isMountedRef.current) {
        setLoading(false)
      }
    }
  }

  useEffect(() => {
    isMountedRef.current = true
    loadData()
    return () => {
      isMountedRef.current = false
    }
  }, [])

  const runningLab = labs.find((l) => l.running)
  const installedCount = labs.filter((l) => l.installed).length
  const totalCount = labs.length || 10
  const toolsReady = tools.filter((t) => t.installed).length

  const handleLaunch = async (labId: string) => {
    setStartingLabId(labId)
    try {
      await startLab(labId)
      await loadData()
      onOpenLab(labId)
    } finally {
      setStartingLabId(null)
    }
  }

  if (loading) {
    return (
      <div className="flex-1 h-full overflow-hidden relative flex items-center justify-center p-8 bg-[#FCFBF8]">
        {/* Pinned Ambient Background Artwork */}
        <div className="absolute inset-0 pointer-events-none z-0 overflow-hidden select-none">
          <img
            src="/assets/bg_dashboard.jpg"
            alt=""
            className="w-full h-full object-cover object-center opacity-80"
            onError={(e) => {
              ;(e.target as HTMLElement).style.display = 'none'
            }}
          />
          <div className="absolute inset-0 bg-[#FCFBF8]/55 cyber-dot-grid" />
        </div>
        <div className="relative z-10">
          <CuteAnimeLoading
            message="INITIALIZING CYBER DASHBOARD..."
            subMessage="( •̀ ω •́ )✧ NEKO CORE ONLINE"
          />
        </div>
      </div>
    )
  }

  return (
    <div className="flex-1 h-full overflow-hidden relative bg-[#FCFBF8]">
      {/* Pinned Ambient Background Artwork (Matches Flutter Positioned.fill outside SingleChildScrollView) */}
      <div className="absolute inset-0 pointer-events-none z-0 overflow-hidden select-none">
        <img
          src="/assets/bg_dashboard.jpg"
          alt=""
          className="w-full h-full object-cover object-center opacity-85"
          onError={(e) => {
            ;(e.target as HTMLElement).style.display = 'none'
          }}
        />
        <div className="absolute inset-0 bg-[#FCFBF8]/50 cyber-dot-grid" />
      </div>

      <div className="relative z-10 h-full overflow-y-auto p-6 space-y-6">
        {/* Top Banner / Hero (Tactile Neo-Retro Desktop Card with Cyber Texture & HD Mascot) */}
      <div className="relative overflow-hidden rounded-xl bg-gradient-to-r from-[#0F766E] via-[#0D9488] to-[#14B8A6] p-6 text-white border-2 border-[#094D47] shadow-[0_4px_0_0_#063934,0_10px_24px_rgba(15,118,110,0.22)] select-none">
        {/* Subtle Cyber Grid Texture */}
        <div className="absolute inset-0 bg-[radial-gradient(#ffffff24_1px,transparent_1px)] [background-size:16px_16px] pointer-events-none opacity-60" />
        <div className="absolute top-0 right-0 w-96 h-full bg-gradient-to-l from-white/10 to-transparent pointer-events-none" />

        {/* Cyber corner stamp */}
        <div className="absolute top-3 right-4 text-[9px] font-['JetBrains_Mono'] text-white/60 tracking-widest hidden md:block">
          [SYS_V4.2 // WIN32_SANDBOX]
        </div>

        <div className="relative z-10 max-w-xl space-y-3">
          <div className="inline-flex items-center gap-2 px-3 py-1 rounded-md bg-[#042F2E]/60 border border-[#2DD4BF]/60 text-[11px] font-['JetBrains_Mono'] font-bold text-[#5EEAD4] shadow-xs tracking-wider">
            <span className="w-2 h-2 rounded-full bg-[#34D399] animate-ping" />
            <span>BOOTCAMP EDITION 2026/2027 // READY</span>
          </div>

          <h1 className="text-2xl lg:text-[26px] font-black font-['Space_Grotesk'] tracking-tight text-white drop-shadow-sm leading-tight">
            Web Penetration Testing & Bug Bounty Bootcamp
          </h1>

          <p className="text-[12.5px] text-teal-50 font-normal leading-relaxed font-sans max-w-lg">
            Laboratorium profesional modern dengan target klon entitas komersial nyata, stateful CRUD, video YouTube bootcamp terintegrasi, dan template laporan HackerOne berstandar industri.
          </p>

          <div className="pt-2 flex items-center gap-3">
            <button
              onClick={() => onNavigate(1)}
              className="px-4 py-2.5 bg-white text-[#0F766E] hover:bg-[#F0FDFA] border-2 border-white/90 shadow-[0_3px_0_0_#99F6E4,0_6px_12px_rgba(0,0,0,0.12)] hover:shadow-[0_1px_0_0_#99F6E4] hover:translate-y-0.5 active:translate-y-1 font-bold font-['Space_Grotesk'] text-xs tracking-wide rounded-lg flex items-center gap-2 cursor-pointer transition-all"
            >
              <span>Explore Labs Catalog</span>
              <ArrowRight size={15} />
            </button>
            <button
              onClick={loadData}
              className="p-2.5 bg-white/10 hover:bg-white/20 border-2 border-white/30 text-white shadow-[0_3px_0_0_rgba(255,255,255,0.2)] hover:shadow-[0_1px_0_0_rgba(255,255,255,0.2)] hover:translate-y-0.5 active:translate-y-1 rounded-lg text-xs transition-all cursor-pointer"
              title="Refresh Dashboard"
            >
              <RefreshCw size={15} />
            </button>
          </div>
        </div>

        {/* Animated Cyber Mascot with Dynamic Morphing Organic Shape & Tech HUD */}
        <div className="absolute -right-6 -bottom-6 md:right-2 md:-bottom-4 hidden sm:block pointer-events-none select-none">
          <div className="relative w-56 h-56 md:w-64 md:h-64 flex items-center justify-center">
            {/* 1. Outer Neon Aura Glow (Pulsing soft radiant energy) */}
            <div className="absolute inset-4 bg-gradient-to-tr from-[#2DD4BF]/40 via-[#38BDF8]/35 to-[#F472B6]/40 blur-xl rounded-full animate-energy-pulse" />

            {/* 2. Counter-rotating Organic Liquid Shape Layer */}
            <div className="absolute inset-6 bg-gradient-to-br from-[#065F46]/70 via-[#0D9488]/60 to-[#DB2777]/35 border border-[#5EEAD4]/40 shadow-[0_0_20px_rgba(45,212,191,0.35)] animate-cyber-blob-reverse opacity-80" />

            {/* 3. Primary Organic Morphing Cyber Shape (Random dynamic fluid silhouette) */}
            <div className="absolute inset-8 bg-gradient-to-tr from-[#0F766E]/85 via-[#14B8A6]/75 to-[#F472B6]/40 border-2 border-white/50 shadow-[inset_0_0_15px_rgba(255,255,255,0.25),0_8px_25px_rgba(15,118,110,0.5)] animate-cyber-blob backdrop-blur-xs" />

            {/* 4. Rotating Cyber Tech HUD Rings (Sci-fi circular radar markings) */}
            <div className="absolute inset-6 rounded-full border border-dashed border-[#5EEAD4]/50 animate-hud-spin pointer-events-none" />
            <div className="absolute inset-10 rounded-full border border-dotted border-white/35 animate-hud-counter-spin pointer-events-none" />

            {/* 5. Floating Cyber Particle Accents */}
            <span className="absolute top-5 left-8 w-2 h-2 rounded-full bg-[#5EEAD4] shadow-[0_0_8px_#5EEAD4] animate-ping" />
            <span className="absolute bottom-8 left-10 w-1.5 h-1.5 rounded-full bg-[#F472B6] shadow-[0_0_6px_#F472B6] animate-pulse" />
            <span className="absolute top-10 right-8 w-2 h-2 rounded-full bg-white shadow-[0_0_6px_white] animate-pulse" />

            {/* 6. High-Definition Anime Mascot Sticker (Gentle 3D Float Animation) */}
            <div className="relative z-10 animate-mascot-float flex items-center justify-center">
              <img
                src="/assets/mascot_banner_hd.png"
                alt="Zitera Cyber Mascot"
                className="w-44 h-44 md:w-52 md:h-52 object-contain drop-shadow-[0_12px_24px_rgba(0,0,0,0.4)] select-none"
                onError={(e) => {
                  ;(e.target as HTMLImageElement).src = '/assets/zeta_avatar.png'
                }}
              />
            </div>
          </div>
        </div>
      </div>

      {/* 4 Pastel Stat Cards (matches Flutter ZiteraColors with Tactile Desktop Texture) */}
      <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
        {/* Stat 1: Challenges Solved */}
        <div className="p-4 rounded-xl border-2 border-[#FDBA74] bg-[#FFEDD5] text-[#C2410C] shadow-[0_3px_0_0_#FDBA74] hover:shadow-[0_1px_0_0_#FDBA74] hover:translate-y-0.5 transition-all flex flex-col justify-between">
          <div className="flex items-center justify-between text-xs font-['Space_Grotesk'] font-bold">
            <span>CTF SOLVES</span>
            <Trophy size={16} />
          </div>
          <div className="mt-3">
            <div className="text-2xl font-bold font-['JetBrains_Mono']">
              {leaderboard.find((e) => e.isCurrentUser)?.labsSolved ?? 0} / {totalCount}
            </div>
            <div className="text-[10px] font-medium opacity-80 mt-0.5">Completed flags</div>
          </div>
        </div>

        {/* Stat 2: Active Labs */}
        <div className="p-4 rounded-xl border-2 border-[#5EEAD4] bg-[#CCFBF1] text-[#0F766E] shadow-[0_3px_0_0_#5EEAD4] hover:shadow-[0_1px_0_0_#5EEAD4] hover:translate-y-0.5 transition-all flex flex-col justify-between">
          <div className="flex items-center justify-between text-xs font-['Space_Grotesk'] font-bold">
            <span>LABS READY</span>
            <FlaskConical size={16} />
          </div>
          <div className="mt-3">
            <div className="text-2xl font-bold font-['JetBrains_Mono']">
              {installedCount} / {totalCount}
            </div>
            <div className="text-[10px] font-medium opacity-80 mt-0.5">Sandboxed targets</div>
          </div>
        </div>

        {/* Stat 3: Environment / Runtimes */}
        <div className="p-4 rounded-xl border-2 border-[#FCD34D] bg-[#FEF3C7] text-[#B45309] shadow-[0_3px_0_0_#FCD34D] hover:shadow-[0_1px_0_0_#FCD34D] hover:translate-y-0.5 transition-all flex flex-col justify-between">
          <div className="flex items-center justify-between text-xs font-['Space_Grotesk'] font-bold">
            <span>RUNTIME HEALTH</span>
            <ShieldCheck size={16} />
          </div>
          <div className="mt-3">
            <div className="text-2xl font-bold font-['JetBrains_Mono']">
              {diagnostics?.allReady ? '100%' : '90%'}
            </div>
            <div className="text-[10px] font-medium opacity-80 mt-0.5">Isolation & Broker OK</div>
          </div>
        </div>

        {/* Stat 4: Security Tools */}
        <div className="p-4 rounded-xl border-2 border-[#D8B4FE] bg-[#EDE9FE] text-[#6D28D9] shadow-[0_3px_0_0_#D8B4FE] hover:shadow-[0_1px_0_0_#D8B4FE] hover:translate-y-0.5 transition-all flex flex-col justify-between">
          <div className="flex items-center justify-between text-xs font-['Space_Grotesk'] font-bold">
            <span>SEC TOOLS</span>
            <Wrench size={16} />
          </div>
          <div className="mt-3">
            <div className="text-2xl font-bold font-['JetBrains_Mono']">
              {toolsReady} / {tools.length || 5}
            </div>
            <div className="text-[10px] font-medium opacity-80 mt-0.5">CLI utilities detected</div>
          </div>
        </div>
      </div>

      {/* Active Lab or Quick Launch Card */}
      <div className="bg-[#FCFBF8] border-2 border-[#CEC8BF] rounded-xl p-5 shadow-[0_3px_0_0_#E4DFD7]">
        <div className="flex items-center justify-between mb-3 border-b-2 border-[#E6E2D8] pb-2">
          <div className="flex items-center gap-2">
            <span className="w-2.5 h-2.5 rounded-full bg-[#2DD4BF]" />
            <h2 className="text-sm font-bold font-['Space_Grotesk'] text-[#1E1A14]">
              {runningLab ? 'ACTIVE TARGET SERVICE' : 'RECOMMENDED STARTING LAB'}
            </h2>
          </div>
          <span className="text-[10px] font-['JetBrains_Mono'] text-[#9C9080]">
            {runningLab ? `PORT :${runningLab.port}` : 'OWASP A01:2025'}
          </span>
        </div>

        {runningLab ? (
          <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-4 bg-[#E6F9F2] border-2 border-[#86EFAC] rounded-lg p-4 shadow-[0_2px_0_0_#86EFAC]">
            <div>
              <div className="flex items-center gap-2">
                <span className="px-2 py-0.5 rounded bg-[#0F766E] text-white text-[10px] font-bold font-['JetBrains_Mono']">
                  {runningLab.id}
                </span>
                <span className="font-bold text-sm text-[#1E1A14] font-['Space_Grotesk']">
                  {runningLab.title}
                </span>
              </div>
              <p className="text-xs text-[#5C5347] mt-1">
                Active sandboxed session listening on port {runningLab.port}. Access the challenge web
                interface or investigate via terminal.
              </p>
            </div>
            <div className="flex items-center gap-2 shrink-0">
              <button
                onClick={() => openBrowser(runningLab.url || `http://127.0.0.1:${runningLab.port}/`)}
                className="px-3.5 py-2 bg-[#0F766E] hover:bg-[#115E59] text-white text-xs font-bold rounded-lg flex items-center gap-1.5 border-2 border-[#094D47] shadow-[0_2px_0_0_#094D47] hover:translate-y-0.5 active:translate-y-1 transition-all cursor-pointer"
              >
                <span>Buka Web</span>
                <ExternalLink size={14} />
              </button>
              <button
                onClick={() => onOpenLab(runningLab.id)}
                className="px-3.5 py-2 bg-[#FCFBF8] border-2 border-[#2DD4BF] text-[#0F766E] hover:bg-[#F0FDFA] text-xs font-bold rounded-lg shadow-[0_2px_0_0_#99F6E4] hover:translate-y-0.5 active:translate-y-1 transition-all cursor-pointer"
              >
                Inspect Lab
              </button>
            </div>
          </div>
        ) : (
          <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-4 bg-[#F7F5F0] border-2 border-[#E4DFD7] rounded-lg p-4 shadow-[0_2px_0_0_#E4DFD7]">
            <div>
              <div className="flex items-center gap-2">
                <span className="px-2 py-0.5 rounded bg-[#0F766E] text-white text-[10px] font-bold font-['JetBrains_Mono']">
                  A01
                </span>
                <span className="font-bold text-sm text-[#1E1A14] font-['Space_Grotesk']">
                  Broken Access Control
                </span>
              </div>
              <p className="text-xs text-[#5C5347] mt-1">
                Begin with the #1 vulnerability on the OWASP Top 10:2025 standard. Practice IDOR and
                horizontal privilege bypass.
              </p>
            </div>
            <button
              onClick={() => handleLaunch('A01')}
              disabled={startingLabId === 'A01'}
              className="px-4 py-2 bg-[#0F766E] hover:bg-[#115E59] text-white text-xs font-bold rounded-lg flex items-center gap-1.5 border-2 border-[#094D47] shadow-[0_2px_0_0_#094D47] hover:translate-y-0.5 active:translate-y-1 disabled:opacity-50 shrink-0 transition-all cursor-pointer"
            >
              <Play size={14} />
              <span>{startingLabId === 'A01' ? 'Starting...' : 'RUN LAB'}</span>
            </button>
          </div>
        )}
      </div>

      {/* Grid: Recent Solves Leaderboard Preview + Quick Nav */}
      <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
        {/* Leaderboard snippet (2 cols) */}
        <div className="md:col-span-2 bg-[#FCFBF8] border-2 border-[#CEC8BF] rounded-xl p-5 shadow-[0_3px_0_0_#E4DFD7]">
          <div className="flex items-center justify-between mb-3 border-b-2 border-[#E6E2D8] pb-2">
            <h3 className="text-sm font-bold font-['Space_Grotesk'] text-[#1E1A14]">
              TOP OPERATIVES // LIVE TURSO LEADERBOARD
            </h3>
            <button
              onClick={() => onNavigate(2)}
              className="text-xs text-[#0F766E] font-bold hover:underline flex items-center gap-1"
            >
              <span>View Full</span>
              <ArrowRight size={13} />
            </button>
          </div>

          <div className="space-y-2">
            {leaderboard.slice(0, 4).map((entry) => (
              <div
                key={entry.userId || entry.rank}
                className={`flex items-center justify-between p-2.5 rounded-lg border-2 text-xs transition-all ${
                  entry.isCurrentUser
                    ? 'bg-[#CCFBF1]/50 border-[#2DD4BF] shadow-[0_2px_0_0_#5EEAD4]'
                    : 'bg-[#F7F5F0] border-[#E4DFD7] shadow-[0_1px_0_0_#E4DFD7]'
                }`}
              >
                <div className="flex items-center gap-3">
                  <span className="w-5 text-center font-['JetBrains_Mono'] font-bold text-[#5C5347]">
                    #{entry.rank}
                  </span>
                  <div className="w-7 h-7 rounded-full border border-[#2DD4BF] overflow-hidden bg-white">
                    <img
                      src={`/assets/avatars/${entry.avatarId}.png`}
                      alt=""
                      className="w-full h-full object-cover"
                      onError={(e) => {
                        ;(e.target as HTMLImageElement).src = '/assets/zeta_avatar.png'
                      }}
                    />
                  </div>
                  <div>
                    <span className="font-['Space_Grotesk'] font-bold text-[#1E1A14]">
                      {entry.username}
                    </span>
                    {entry.isCurrentUser && (
                      <span className="ml-2 px-1.5 py-0.2 rounded bg-[#0F766E] text-white text-[9px] font-bold">
                        YOU
                      </span>
                    )}
                  </div>
                </div>

                <div className="flex items-center gap-4 font-['JetBrains_Mono']">
                  <span className="text-[#0F766E] font-bold">{entry.totalPoints} PTS</span>
                  <span className="text-[#9C9080]">{entry.labsSolved} Solved</span>
                </div>
              </div>
            ))}

            {leaderboard.length === 0 && (
              <div className="py-6 text-center text-xs text-[#9C9080] font-['JetBrains_Mono']">
                No CTF solves recorded yet. Be the first to capture the flag!
              </div>
            )}
          </div>
        </div>

        {/* Quick Nav / Shortcuts (1 col) */}
        <div className="bg-[#FCFBF8] border-2 border-[#CEC8BF] rounded-xl p-5 shadow-[0_3px_0_0_#E4DFD7] flex flex-col justify-between">
          <div>
            <h3 className="text-sm font-bold font-['Space_Grotesk'] text-[#1E1A14] mb-3 border-b-2 border-[#E6E2D8] pb-2">
              QUICK COMMANDS
            </h3>
            <div className="space-y-2">
              <button
                onClick={() => onNavigate(1)}
                className="w-full p-2.5 text-left rounded-lg border-2 border-[#E4DFD7] bg-[#F7F5F0] hover:bg-[#F0FDFA] hover:border-[#2DD4BF] text-xs font-bold text-[#1E1A14] flex items-center justify-between shadow-[0_2px_0_0_#E4DFD7] hover:shadow-[0_2px_0_0_#5EEAD4] hover:-translate-y-0.5 transition-all cursor-pointer"
              >
                <span>Browse All 10 Labs</span>
                <FlaskConical size={16} className="text-[#0F766E]" />
              </button>
              <button
                onClick={() => onNavigate(3)}
                className="w-full p-2.5 text-left rounded-lg border-2 border-[#E4DFD7] bg-[#F7F5F0] hover:bg-[#F0FDFA] hover:border-[#2DD4BF] text-xs font-bold text-[#1E1A14] flex items-center justify-between shadow-[0_2px_0_0_#E4DFD7] hover:shadow-[0_2px_0_0_#5EEAD4] hover:-translate-y-0.5 transition-all cursor-pointer"
              >
                <span>Security Tools Manager</span>
                <Wrench size={16} className="text-[#6D28D9]" />
              </button>
              <button
                onClick={() => onNavigate(4)}
                className="w-full p-2.5 text-left rounded-lg border-2 border-[#E4DFD7] bg-[#F7F5F0] hover:bg-[#F0FDFA] hover:border-[#2DD4BF] text-xs font-bold text-[#1E1A14] flex items-center justify-between shadow-[0_2px_0_0_#E4DFD7] hover:shadow-[0_2px_0_0_#5EEAD4] hover:-translate-y-0.5 transition-all cursor-pointer"
              >
                <span>System Environment Check</span>
                <ShieldCheck size={16} className="text-[#B45309]" />
              </button>
            </div>
          </div>

          <div className="pt-4 border-t-2 border-[#E6E2D8] flex items-center gap-2">
            <img
              src="/assets/calico_cat.png"
              alt=""
              className="w-8 h-8 object-contain"
              onError={(e) => {
                ;(e.target as HTMLElement).style.display = 'none'
              }}
            />
            <div className="text-[10px] text-[#5C5347] font-['JetBrains_Mono']">
              Engine connected. AppContainer boundary enforced.
            </div>
          </div>
        </div>
      </div>
      </div>
    </div>
  )
}
