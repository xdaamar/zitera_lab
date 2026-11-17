import { useEffect, useState } from 'react'
import {
  ShieldCheck,
  FlaskConical,
  Trophy,
  Wrench,
  Play,
  ExternalLink,
  Sparkles,
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

  const loadData = async () => {
    setLoading(true)
    try {
      const [l, t, d, lb] = await Promise.all([
        getLabs(),
        getTools(),
        checkDoctor(),
        getLeaderboard().catch(() => []),
      ])
      setLabs(l)
      setTools(t)
      setDiagnostics(d)
      setLeaderboard(lb)
    } finally {
      setLoading(false)
    }
  }

  useEffect(() => {
    loadData()
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
      <div className="flex-1 flex flex-col items-center justify-center p-8 text-center space-y-4">
        <div className="w-16 h-16 rounded-full border-4 border-[#2DD4BF] border-t-transparent animate-spin flex items-center justify-center">
          <Sparkles className="text-[#0F766E] animate-pulse" size={24} />
        </div>
        <div>
          <div className="text-base font-bold font-['Space_Grotesk'] text-[#1E1A14]">
            INITIALIZING CYBER DASHBOARD...
          </div>
          <div className="text-xs font-['JetBrains_Mono'] text-[#0F766E]">
            ( •̀ ω •́ )✧ NEKO CORE ONLINE
          </div>
        </div>
      </div>
    )
  }

  return (
    <div className="flex-1 overflow-y-auto p-6 space-y-6 bg-[#FCFBF8]">
      {/* Top Banner / Hero */}
      <div className="relative overflow-hidden rounded-2xl bg-gradient-to-r from-[#0F766E] to-[#14B8A6] p-6 text-white shadow-lg border border-[#0F766E]">
        <div className="relative z-10 max-w-xl space-y-2">
          <div className="inline-flex items-center gap-1.5 px-2.5 py-1 rounded-full bg-white/15 text-[11px] font-['JetBrains_Mono'] tracking-wide">
            <span className="w-2 h-2 rounded-full bg-[#34D399] animate-ping" />
            <span>NATIVE SANDBOX ENGINE // READY</span>
          </div>
          <h1 className="text-2xl font-bold font-['Space_Grotesk'] tracking-tight">
            OWASP Top 10:2025 Defensive Training
          </h1>
          <p className="text-xs text-white/80 leading-relaxed font-sans">
            Offline-first security engineering laboratory with native Win32 AppContainer
            containment, deterministic vulnerability targets, and real-time CTF validation.
          </p>
          <div className="pt-2 flex items-center gap-3">
            <button
              onClick={() => onNavigate(1)}
              className="px-4 py-2 bg-white text-[#0F766E] hover:bg-[#F0EDE8] rounded-lg text-xs font-bold font-['Space_Grotesk'] shadow-sm transition-all flex items-center gap-1.5"
            >
              <span>Explore Labs Catalog</span>
              <ArrowRight size={14} />
            </button>
            <button
              onClick={loadData}
              className="p-2 bg-white/10 hover:bg-white/20 rounded-lg text-white text-xs transition-colors"
              title="Refresh Dashboard"
            >
              <RefreshCw size={15} />
            </button>
          </div>
        </div>

        {/* Ambient sticker illustration */}
        <div className="absolute right-4 bottom-2 opacity-90 hidden sm:block pointer-events-none">
          <img
            src="/assets/sticker_pink.png"
            alt="Mascot"
            className="w-32 h-32 object-contain drop-shadow-md"
            onError={(e) => {
              ;(e.target as HTMLElement).style.display = 'none'
            }}
          />
        </div>
      </div>

      {/* 4 Pastel Stat Cards (matches Flutter ZiteraColors) */}
      <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
        {/* Stat 1: Challenges Solved */}
        <div className="p-4 rounded-xl border border-[#FDBA74] bg-[#FFEDD5] text-[#C2410C] shadow-xs flex flex-col justify-between">
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
        <div className="p-4 rounded-xl border border-[#5EEAD4] bg-[#CCFBF1] text-[#0F766E] shadow-xs flex flex-col justify-between">
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
        <div className="p-4 rounded-xl border border-[#FCD34D] bg-[#FEF3C7] text-[#B45309] shadow-xs flex flex-col justify-between">
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
        <div className="p-4 rounded-xl border border-[#D8B4FE] bg-[#EDE9FE] text-[#6D28D9] shadow-xs flex flex-col justify-between">
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
      <div className="bg-[#FCFBF8] border-[1.5px] border-[#E6E2D8] rounded-xl p-5 shadow-xs">
        <div className="flex items-center justify-between mb-3 border-b border-[#E6E2D8] pb-2">
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
          <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-4 bg-[#E6F9F2] border border-[#86EFAC] rounded-lg p-4">
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
                onClick={() => openBrowser(`http://127.0.0.1:${runningLab.port}`)}
                className="px-3.5 py-2 bg-[#0F766E] hover:bg-[#115E59] text-white text-xs font-bold rounded-lg flex items-center gap-1.5 shadow-xs"
              >
                <span>Buka Web</span>
                <ExternalLink size={14} />
              </button>
              <button
                onClick={() => onOpenLab(runningLab.id)}
                className="px-3.5 py-2 bg-[#FCFBF8] border border-[#2DD4BF] text-[#0F766E] hover:bg-[#F0FDFA] text-xs font-bold rounded-lg"
              >
                Inspect Lab
              </button>
            </div>
          </div>
        ) : (
          <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-4 bg-[#F7F5F0] border border-[#E6E2D8] rounded-lg p-4">
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
              className="px-4 py-2 bg-[#0F766E] hover:bg-[#115E59] text-white text-xs font-bold rounded-lg flex items-center gap-1.5 shadow-xs disabled:opacity-50 shrink-0"
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
        <div className="md:col-span-2 bg-[#FCFBF8] border-[1.5px] border-[#E6E2D8] rounded-xl p-5 shadow-xs">
          <div className="flex items-center justify-between mb-3 border-b border-[#E6E2D8] pb-2">
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
                className={`flex items-center justify-between p-2.5 rounded-lg border text-xs transition-colors ${
                  entry.isCurrentUser
                    ? 'bg-[#CCFBF1]/40 border-[#2DD4BF]'
                    : 'bg-[#F7F5F0] border-[#E6E2D8]'
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
        <div className="bg-[#FCFBF8] border-[1.5px] border-[#E6E2D8] rounded-xl p-5 shadow-xs flex flex-col justify-between">
          <div>
            <h3 className="text-sm font-bold font-['Space_Grotesk'] text-[#1E1A14] mb-3 border-b border-[#E6E2D8] pb-2">
              QUICK COMMANDS
            </h3>
            <div className="space-y-2">
              <button
                onClick={() => onNavigate(1)}
                className="w-full p-2.5 text-left rounded-lg border border-[#E6E2D8] bg-[#F7F5F0] hover:bg-[#F0FDFA] hover:border-[#2DD4BF] text-xs font-bold text-[#1E1A14] flex items-center justify-between"
              >
                <span>Browse All 10 Labs</span>
                <FlaskConical size={16} className="text-[#0F766E]" />
              </button>
              <button
                onClick={() => onNavigate(3)}
                className="w-full p-2.5 text-left rounded-lg border border-[#E6E2D8] bg-[#F7F5F0] hover:bg-[#F0FDFA] hover:border-[#2DD4BF] text-xs font-bold text-[#1E1A14] flex items-center justify-between"
              >
                <span>Security Tools Manager</span>
                <Wrench size={16} className="text-[#6D28D9]" />
              </button>
              <button
                onClick={() => onNavigate(4)}
                className="w-full p-2.5 text-left rounded-lg border border-[#E6E2D8] bg-[#F7F5F0] hover:bg-[#F0FDFA] hover:border-[#2DD4BF] text-xs font-bold text-[#1E1A14] flex items-center justify-between"
              >
                <span>System Environment Check</span>
                <ShieldCheck size={16} className="text-[#B45309]" />
              </button>
            </div>
          </div>

          <div className="pt-4 border-t border-[#E6E2D8] flex items-center gap-2">
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
  )
}
