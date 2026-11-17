import { useState, useEffect, useRef, type FormEvent } from 'react'
import ReactMarkdown from 'react-markdown'
import remarkGfm from 'remark-gfm'
import confetti from 'canvas-confetti'
import {
  ArrowLeft,
  Compass,
  Lightbulb,
  Cpu,
  Layers,
  Footprints,
  ShieldCheck,
  Shield,
  TrendingUp,
  Server,
  Target,
  Terminal as TerminalIcon,
  Trophy,
  Play,
  Square,
  ExternalLink,
  CheckCircle2,
  AlertCircle,
  Send,
  ChevronLeft,
  ChevronRight,
  Check,
  Sparkles,
  PanelRightClose,
} from 'lucide-react'
import {
  getLabContent,
  getLabStatus,
  startLab,
  stopLab,
  openBrowser,
  executeTerminal,
  validateChallenge,
  getMissionBriefing,
  type LabContent,
  type LabItem,
} from './engineClient'
import { recordSolve } from './scoreboardService'
import { CuteAnimeLoading } from './CuteAnimeLoading'

interface LabDetailViewProps {
  labId: string
  onBack: () => void
}

type SectionKey =
  | 'introduction'
  | 'analogy'
  | 'concept'
  | 'architecture'
  | 'walkthrough'
  | 'remediation'
  | 'challenge'

interface SectionTabDef {
  key: SectionKey
  label: string
  shortLabel: string
  icon: typeof Compass
  badgeText: string
}

const CURRICULUM_SECTIONS: SectionTabDef[] = [
  { key: 'introduction', label: 'Pengantar & Dampak', shortLabel: 'Pengantar', icon: Compass, badgeText: 'Intro' },
  { key: 'analogy', label: 'Analogi Nyata', shortLabel: 'Analogi', icon: Lightbulb, badgeText: 'Analogi' },
  { key: 'concept', label: 'Konsep & Mekanisme', shortLabel: 'Konsep', icon: Cpu, badgeText: 'Konsep' },
  { key: 'architecture', label: 'Arsitektur Sistem', shortLabel: 'Arsitektur', icon: Layers, badgeText: 'Sistem' },
  { key: 'walkthrough', label: 'Panduan Walkthrough', shortLabel: 'Walkthrough', icon: Footprints, badgeText: 'Praktik' },
  { key: 'remediation', label: 'Remediasi Kode', shortLabel: 'Remediasi', icon: ShieldCheck, badgeText: 'Defensif' },
]

function resolveLessonMarkdown(lessons: Record<string, string>, sectionKey: string): string {
  if (!lessons) return ''

  // 1. Prioritize Indonesian keys
  const idCandidates = [
    `${sectionKey}.id.md`,
    `${sectionKey}.id`,
    `${sectionKey}_id.md`,
    `${sectionKey}_id`,
  ]
  for (const c of idCandidates) {
    if (lessons[c] && lessons[c].trim().length > 0) {
      return lessons[c]
    }
  }

  // 2. Search for any key matching sectionKey with .id
  const matchingIdKey = Object.keys(lessons).find(
    (k) =>
      k.toLowerCase().startsWith(sectionKey.toLowerCase()) &&
      k.includes('.id') &&
      lessons[k]?.trim().length > 0
  )
  if (matchingIdKey) {
    return lessons[matchingIdKey]
  }

  // 3. Fallback to standard keys
  const standardCandidates = [sectionKey, `${sectionKey}.md`]
  for (const c of standardCandidates) {
    if (lessons[c] && lessons[c].trim().length > 0) {
      return lessons[c]
    }
  }

  // 4. Fuzzy fallback
  const fuzzyKey = Object.keys(lessons).find(
    (k) => k.toLowerCase().includes(sectionKey.toLowerCase()) && lessons[k]?.trim().length > 0
  )
  if (fuzzyKey) {
    return lessons[fuzzyKey]
  }

  return ''
}

export function LabDetailView({ labId, onBack }: LabDetailViewProps) {
  const [content, setContent] = useState<LabContent | null>(null)
  const [status, setStatus] = useState<LabItem | null>(null)
  const [activeSection, setActiveSection] = useState<SectionKey>('introduction')
  const [loading, setLoading] = useState(true)

  // Collapsible Terminal Panel State
  const [terminalOpen, setTerminalOpen] = useState(true)

  // Progress Tracking: Completed Sections (Indonesian curriculum)
  const [completedSections, setCompletedSections] = useState<Set<string>>(() => {
    try {
      const raw = localStorage.getItem(`zitera_completed_${labId}`)
      return raw ? new Set(JSON.parse(raw)) : new Set()
    } catch {
      return new Set()
    }
  })

  // Challenge Solved State
  const [challengeSolved, setChallengeSolved] = useState<boolean>(() => {
    try {
      return localStorage.getItem(`zitera_solved_${labId}`) === 'true'
    } catch {
      return false
    }
  })

  // CTF Challenge state
  const [flagInput, setFlagInput] = useState('')
  const [revealedHints, setRevealedHints] = useState<number[]>([])
  const [flagResult, setFlagResult] = useState<{ passed: boolean; message: string } | null>(null)
  const [submittingFlag, setSubmittingFlag] = useState(false)

  // Terminal state
  const [terminalHistory, setTerminalHistory] = useState<
    Array<{ type: 'input' | 'output' | 'error'; text: string }>
  >([
    { type: 'output', text: `Zitera Tilix Interactive Shell // Context: ${labId}` },
    { type: 'output', text: `Type 'help' or 'curl http://127.0.0.1:<PORT>' to probe target.` },
  ])
  const [terminalInput, setTerminalInput] = useState('')
  const [executing, setExecuting] = useState(false)
  const terminalBottomRef = useRef<HTMLDivElement | null>(null)

  const loadData = async (isMountedRef = { current: true }) => {
    setLoading(true)
    try {
      const [c, s] = await Promise.all([
        getLabContent(labId),
        getLabStatus(labId).catch(() => null),
      ])
      if (isMountedRef.current) {
        setContent(c)
        setStatus(s)
      }
    } finally {
      if (isMountedRef.current) {
        setLoading(false)
      }
    }
  }

  useEffect(() => {
    const isMountedRef = { current: true }
    loadData(isMountedRef)
    return () => {
      isMountedRef.current = false
    }
  }, [labId])

  useEffect(() => {
    terminalBottomRef.current?.scrollIntoView({ behavior: 'smooth' })
  }, [terminalHistory])

  const handleToggleTarget = async () => {
    if (!status) return
    if (status.running) {
      await stopLab(labId)
    } else {
      await startLab(labId)
    }
    const updated = await getLabStatus(labId)
    setStatus(updated)
  }

  const toggleSectionCompleted = (secKey: string) => {
    setCompletedSections((prev) => {
      const next = new Set(prev)
      if (next.has(secKey)) {
        next.delete(secKey)
      } else {
        next.add(secKey)
      }
      try {
        localStorage.setItem(`zitera_completed_${labId}`, JSON.stringify(Array.from(next)))
      } catch (_) {}
      return next
    })
  }

  const handleFlagSubmit = async (e: FormEvent) => {
    e.preventDefault()
    if (!flagInput.trim()) return
    setSubmittingFlag(true)
    setFlagResult(null)

    try {
      const res = await validateChallenge(labId, flagInput)
      if (res.status === 'passed') {
        setFlagResult({ passed: true, message: res.message || 'CTF FLAG CAPTURED! +100 PTS' })
        setChallengeSolved(true)
        try {
          localStorage.setItem(`zitera_solved_${labId}`, 'true')
        } catch (_) {}

        // Trigger confetti celebration!
        confetti({
          particleCount: 140,
          spread: 85,
          origin: { y: 0.6 },
          colors: ['#2DD4BF', '#0F766E', '#F59E0B', '#3B82F6'],
        })
        await recordSolve(labId, 100)
      } else {
        setFlagResult({ passed: false, message: res.message || 'Incorrect flag. Inspect the clues!' })
      }
    } finally {
      setSubmittingFlag(false)
    }
  }

  const handleTerminalSubmit = async (e: FormEvent) => {
    e.preventDefault()
    const cmd = terminalInput.trim()
    if (!cmd) return

    setTerminalHistory((prev) => [...prev.slice(-249), { type: 'input', text: `$ ${cmd}` }])
    setTerminalInput('')
    setExecuting(true)

    try {
      if (cmd === 'clear') {
        setTerminalHistory([])
        return
      }
      const res = await executeTerminal(cmd, labId)
      if (res.stdout) {
        setTerminalHistory((prev) => [...prev.slice(-249), { type: 'output', text: res.stdout }])
      }
      if (res.stderr) {
        setTerminalHistory((prev) => [...prev.slice(-249), { type: 'error', text: res.stderr }])
      }
    } catch (err) {
      setTerminalHistory((prev) => [
        ...prev.slice(-249),
        { type: 'error', text: `Command execution error: ${err}` },
      ])
    } finally {
      setExecuting(false)
    }
  }

  const revealHint = (tier: number) => {
    if (!revealedHints.includes(tier)) {
      setRevealedHints((prev) => [...prev, tier])
    }
  }

  if (loading || !content) {
    return (
      <div className="flex-1 h-full overflow-hidden relative flex items-center justify-center p-8 bg-[#FCFBF8]">
        {/* Pinned Ambient Lab Wallpaper */}
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
        <div className="relative z-10">
          <CuteAnimeLoading
            message="MEMPERSIAPKAN MATERI KURIKULUM & TARGET..."
            subMessage="( ⚡ •̀ᴗ•́) DEPLOYING SANDBOX ENVIRONMENT"
          />
        </div>
      </div>
    )
  }

  const targetPort = status?.port ?? content.manifest.defaultPort ?? 8011
  const briefing = getMissionBriefing(labId)

  // Current curriculum section navigation indices
  const currentSectionIndex = CURRICULUM_SECTIONS.findIndex((s) => s.key === activeSection)
  const prevSection = currentSectionIndex > 0 ? CURRICULUM_SECTIONS[currentSectionIndex - 1] : null
  const nextSection =
    currentSectionIndex >= 0 && currentSectionIndex < CURRICULUM_SECTIONS.length - 1
      ? CURRICULUM_SECTIONS[currentSectionIndex + 1]
      : null

  // Resolve active section markdown text
  const currentLessonMarkdown =
    activeSection !== 'challenge'
      ? resolveLessonMarkdown(content.lessons, activeSection)
      : ''

  const isCurrentSectionCompleted =
    activeSection !== 'challenge' && completedSections.has(activeSection)

  return (
    <div className="flex-1 flex flex-col h-full overflow-hidden bg-[#FCFBF8]">
      {/* Top Header Bar */}
      <div className="px-5 py-3 border-b border-[#E6E2D8] bg-[#F7F5F0] flex flex-wrap items-center justify-between gap-3 shrink-0">
        {/* Left: Back & Lab Title */}
        <div className="flex items-center gap-3">
          <button
            onClick={onBack}
            className="p-1.5 rounded-lg border border-[#E6E2D8] bg-white hover:bg-[#F0EDE8] text-[#5C5347] transition-colors flex items-center gap-1.5 text-xs font-bold font-['Space_Grotesk']"
          >
            <ArrowLeft size={15} />
            <span className="hidden sm:inline">Kembali</span>
          </button>
          <div className="h-4 w-px bg-[#E6E2D8]" />
          <div className="flex items-center gap-2">
            <span className="px-2 py-0.5 rounded bg-[#0F766E] text-white text-[11px] font-bold font-['JetBrains_Mono']">
              {content.manifest.id}
            </span>
            <div>
              <div className="flex items-center gap-2">
                <h1 className="text-sm font-bold font-['Space_Grotesk'] text-[#1E1A14]">
                  {content.manifest.title}
                </h1>
                <span className="text-[11px] text-[#78716C] font-['JetBrains_Mono'] hidden md:inline">
                  • {briefing.targetAppName}
                </span>
              </div>
            </div>
          </div>
        </div>

        {/* Right: Actions & Collapsible Terminal Toggle */}
        <div className="flex items-center gap-2.5">
          {/* Target Service Controller */}
          {status?.running ? (
            <>
              <button
                onClick={() => openBrowser(`http://127.0.0.1:${targetPort}`)}
                className="px-3 py-1.5 bg-[#0F766E] hover:bg-[#115E59] text-white text-xs font-bold font-['Space_Grotesk'] rounded-lg flex items-center gap-1.5 shadow-xs transition-colors"
                title={`Buka http://127.0.0.1:${targetPort} di browser`}
              >
                <span>Buka Web (: {targetPort})</span>
                <ExternalLink size={13} />
              </button>
              <button
                onClick={handleToggleTarget}
                className="px-3 py-1.5 bg-white border border-[#EF4444] text-[#EF4444] hover:bg-[#FEE2E2] text-xs font-bold font-['Space_Grotesk'] rounded-lg flex items-center gap-1 transition-colors"
              >
                <Square size={13} />
                <span>Stop</span>
              </button>
            </>
          ) : (
            <button
              onClick={handleToggleTarget}
              className="px-3.5 py-1.5 bg-[#0F766E] hover:bg-[#115E59] text-white text-xs font-bold font-['Space_Grotesk'] rounded-lg flex items-center gap-1.5 shadow-xs transition-colors"
            >
              <Play size={13} />
              <span>LAUNCH TARGET</span>
            </button>
          )}

          <div className="h-4 w-px bg-[#E6E2D8]" />

          {/* Requirement 1: Collapsible Terminal Panel Toggle Button */}
          <button
            onClick={() => setTerminalOpen((prev) => !prev)}
            className={`px-3 py-1.5 rounded-lg border text-xs font-bold font-['Space_Grotesk'] flex items-center gap-1.5 transition-all ${
              terminalOpen
                ? 'border-[#E6E2D8] bg-white text-[#5C5347] hover:bg-[#F0EDE8]'
                : 'border-[#2DD4BF] bg-[#CCFBF1] text-[#0F766E] hover:bg-[#99F6E4] shadow-xs'
            }`}
            title={terminalOpen ? 'Sembunyikan Terminal Tilix' : 'Buka Terminal Tilix'}
          >
            {terminalOpen ? <PanelRightClose size={14} /> : <TerminalIcon size={14} />}
            <span>{terminalOpen ? 'Sembunyikan Terminal' : 'Buka Terminal Tilix'}</span>
          </button>
        </div>
      </div>

      {/* Requirement 2: Curriculum 6-Section Navigator Bar + CTF Challenge Tab */}
      <div className="border-b border-[#E6E2D8] bg-[#FCFBF8] px-4 md:px-6 pt-2 shrink-0 overflow-x-auto scrollbar-none flex items-center justify-between gap-4">
        <div className="flex items-center gap-1 min-w-max">
          {CURRICULUM_SECTIONS.map((sec, idx) => {
            const Icon = sec.icon
            const isActive = activeSection === sec.key
            const isDone = completedSections.has(sec.key)
            return (
              <button
                key={sec.key}
                onClick={() => setActiveSection(sec.key)}
                className={`flex items-center gap-1.5 px-3 py-2 text-xs font-['Space_Grotesk'] font-bold rounded-t-lg transition-all border-t border-x ${
                  isActive
                    ? 'bg-[#FCFBF8] border-[#2DD4BF] text-[#0F766E] shadow-[0_-2px_4px_rgba(45,212,191,0.08)]'
                    : 'border-transparent text-[#78716C] hover:text-[#1E1A14] hover:bg-[#F7F5F0]'
                }`}
              >
                <Icon size={14} className={isActive ? 'text-[#0F766E]' : 'text-[#A8A29E]'} />
                <span>
                  {idx + 1}. {sec.label}
                </span>
                {isDone && (
                  <CheckCircle2 size={13} className="text-[#10B981] ml-0.5" />
                )}
              </button>
            )
          })}

          {/* 7th Tab: CTF Challenge */}
          <button
            onClick={() => setActiveSection('challenge')}
            className={`flex items-center gap-1.5 px-3.5 py-2 text-xs font-['Space_Grotesk'] font-bold rounded-t-lg transition-all border-t border-x ${
              activeSection === 'challenge'
                ? 'bg-[#FCFBF8] border-[#F59E0B] text-[#B45309] shadow-[0_-2px_4px_rgba(245,158,11,0.08)]'
                : 'border-transparent text-[#78716C] hover:text-[#1E1A14] hover:bg-[#F7F5F0]'
            }`}
          >
            <Trophy
              size={14}
              className={activeSection === 'challenge' ? 'text-[#D97706]' : 'text-[#A8A29E]'}
            />
            <span>Tantangan CTF</span>
            {challengeSolved && (
              <span className="px-1.5 py-0.2 rounded bg-[#DCFCE7] text-[#15803D] text-[10px] font-bold font-['JetBrains_Mono']">
                SOLVED
              </span>
            )}
          </button>
        </div>

        {/* Section Completion Progress Counter */}
        <div className="hidden lg:flex items-center gap-2 pb-2 shrink-0">
          <div className="px-2.5 py-1 rounded-md bg-[#CCFBF1] border border-[#2DD4BF]/40 text-[11px] font-['JetBrains_Mono'] font-bold text-[#0F766E] flex items-center gap-1.5">
            <Sparkles size={12} />
            <span>Materi: {completedSections.size}/6 Selesai</span>
          </div>
        </div>
      </div>

      {/* Main Split Layout: Content + Tilix Terminal Panel */}
      <div className="flex-1 flex flex-col lg:flex-row overflow-hidden">
        {/* Left Column: Reading Content Area */}
        <div
          className={`flex-1 h-full overflow-hidden relative bg-[#FCFBF8] ${
            terminalOpen ? 'border-r border-[#E6E2D8]' : ''
          }`}
        >
          {/* Pinned Ambient Wallpaper */}
          <div className="absolute inset-0 pointer-events-none z-0 overflow-hidden select-none">
            <img
              src="/assets/bg_labs.jpg"
              alt=""
              className="w-full h-full object-cover object-center opacity-18"
              onError={(e) => {
                ;(e.target as HTMLElement).style.display = 'none'
              }}
            />
            <div className="absolute inset-0 bg-[#FCFBF8]/85" />
          </div>

          {/* Reading Scrollable Content */}
          <div className="relative z-10 h-full overflow-y-auto p-5 md:p-8">
            {/* When collapsed, expand to max-w-4xl mx-auto for distraction-free reading; otherwise max-w-none */}
            <div className={terminalOpen ? 'w-full space-y-6' : 'max-w-4xl mx-auto w-full space-y-6'}>
              {/* Requirement 3: Pre-Flight Mission Briefing Card */}
              <div className="rounded-xl border border-[#2DD4BF]/50 bg-gradient-to-b from-[#F0FDF4]/30 via-white to-[#F7F5F0] p-5 shadow-xs space-y-4">
                {/* Briefing Top Banner */}
                <div className="flex flex-wrap items-center justify-between gap-3 pb-3.5 border-b border-[#E6E2D8]">
                  <div className="flex items-center gap-3">
                    <div className="w-9 h-9 rounded-lg bg-[#0F766E] text-white flex items-center justify-center shrink-0 shadow-xs">
                      <Shield size={18} />
                    </div>
                    <div>
                      <div className="flex items-center gap-2">
                        <span className="text-[10px] uppercase font-bold tracking-wider font-['JetBrains_Mono'] px-2 py-0.5 rounded bg-[#CCFBF1] text-[#0F766E] border border-[#2DD4BF]/50">
                          PRE-FLIGHT MISSION BRIEFING
                        </span>
                        <span className="text-xs font-['JetBrains_Mono'] text-[#9C9080]">
                          // {labId} : {content.manifest.owasp}
                        </span>
                      </div>
                      <div className="text-sm font-bold font-['Space_Grotesk'] text-[#1E1A14] mt-0.5">
                        {briefing.targetAppName}{' '}
                        <span className="text-xs font-normal text-[#5C5347]">
                          — {briefing.targetAppSubtitle}
                        </span>
                      </div>
                    </div>
                  </div>

                  <div className="flex items-center gap-2">
                    <span className="text-[10px] font-bold font-['JetBrains_Mono'] px-2.5 py-1 rounded bg-[#FEF3C7] text-[#B45309] border border-[#FDE68A]">
                      {content.manifest.difficulty.toUpperCase()}
                    </span>
                    <span className="text-[10px] font-bold font-['JetBrains_Mono'] px-2.5 py-1 rounded bg-[#F0EDE8] text-[#5C5347] border border-[#E6E2D8]">
                      PORT {targetPort}
                    </span>
                  </div>
                </div>

                {/* 3 Executive Pillars Grid */}
                <div className="grid grid-cols-1 md:grid-cols-3 gap-3">
                  {/* Pillar 1: Kenapa Ini Penting */}
                  <div className="p-4 rounded-lg bg-white border border-[#E6E2D8] hover:border-[#2DD4BF]/60 transition-all flex flex-col shadow-[0_1px_2px_rgba(0,0,0,0.03)]">
                    <div className="flex items-center gap-2 text-xs font-bold font-['Space_Grotesk'] text-[#0F766E] mb-2 pb-1.5 border-b border-[#F0EDE8]">
                      <TrendingUp size={15} className="text-[#0F766E] shrink-0" />
                      <span>Kenapa Ini Penting</span>
                    </div>
                    <p className="text-xs leading-relaxed text-[#5C5347] font-sans flex-1">
                      {briefing.whyItMatters}
                    </p>
                  </div>

                  {/* Pillar 2: Apa yang Akan Dihadapi */}
                  <div className="p-4 rounded-lg bg-white border border-[#E6E2D8] hover:border-[#2DD4BF]/60 transition-all flex flex-col shadow-[0_1px_2px_rgba(0,0,0,0.03)]">
                    <div className="flex items-center gap-2 text-xs font-bold font-['Space_Grotesk'] text-[#B45309] mb-2 pb-1.5 border-b border-[#F0EDE8]">
                      <Server size={15} className="text-[#B45309] shrink-0" />
                      <span>Apa yang Akan Dihadapi</span>
                    </div>
                    <p className="text-xs leading-relaxed text-[#5C5347] font-sans flex-1">
                      {briefing.targetArchitecture}
                    </p>
                  </div>

                  {/* Pillar 3: Misi & Objektif */}
                  <div className="p-4 rounded-lg bg-white border border-[#E6E2D8] hover:border-[#2DD4BF]/60 transition-all flex flex-col shadow-[0_1px_2px_rgba(0,0,0,0.03)]">
                    <div className="flex items-center gap-2 text-xs font-bold font-['Space_Grotesk'] text-[#2563EB] mb-2 pb-1.5 border-b border-[#F0EDE8]">
                      <Target size={15} className="text-[#2563EB] shrink-0" />
                      <span>Misi & Objektif</span>
                    </div>
                    <div className="text-xs leading-relaxed text-[#5C5347] font-sans whitespace-pre-line flex-1">
                      {briefing.missionObjectives}
                    </div>
                  </div>
                </div>
              </div>

              {/* View Section: Curriculum Lesson Content */}
              {activeSection !== 'challenge' ? (
                <div className="space-y-6">
                  {/* Section Title Bar & Completion Toggle */}
                  <div className="flex flex-wrap items-center justify-between gap-3 p-4 rounded-xl border border-[#E6E2D8] bg-[#F7F5F0]">
                    <div className="flex items-center gap-3">
                      <div className="p-2 rounded-lg bg-white border border-[#E6E2D8] text-[#0F766E] shadow-2xs">
                        {(() => {
                          const secDef = CURRICULUM_SECTIONS.find((s) => s.key === activeSection)
                          const IconComp = secDef?.icon ?? Compass
                          return <IconComp size={18} />
                        })()}
                      </div>
                      <div>
                        <span className="text-[10px] font-['JetBrains_Mono'] font-bold text-[#0F766E] uppercase tracking-wider">
                          MODUL PEMBELAJARAN // BAGIAN {currentSectionIndex + 1} DARI 6
                        </span>
                        <h3 className="text-base font-bold font-['Space_Grotesk'] text-[#1E1A14]">
                          {CURRICULUM_SECTIONS.find((s) => s.key === activeSection)?.label}
                        </h3>
                      </div>
                    </div>

                    <button
                      onClick={() => toggleSectionCompleted(activeSection)}
                      className={`px-3 py-1.5 text-xs font-['Space_Grotesk'] font-bold rounded-lg border transition-all flex items-center gap-1.5 ${
                        isCurrentSectionCompleted
                          ? 'bg-[#DCFCE7] border-[#86EFAC] text-[#16A34A] hover:bg-[#BBF7D0]'
                          : 'bg-white border-[#E6E2D8] text-[#5C5347] hover:bg-[#F0EDE8]'
                      }`}
                    >
                      {isCurrentSectionCompleted ? (
                        <>
                          <Check size={14} className="text-[#16A34A]" />
                          <span>Selesai Dipahami</span>
                        </>
                      ) : (
                        <>
                          <CheckCircle2 size={14} className="text-[#9C9080]" />
                          <span>Tandai Selesai</span>
                        </>
                      )}
                    </button>
                  </div>

                  {/* Indonesian Markdown Render */}
                  <div className="p-6 md:p-8 rounded-xl border border-[#E6E2D8] bg-white shadow-xs">
                    <div className="prose-zitera max-w-none">
                      <ReactMarkdown remarkPlugins={[remarkGfm]}>
                        {currentLessonMarkdown ||
                          `# ${content.manifest.title}\n\nMateri kurikulum sedang dimuat...`}
                      </ReactMarkdown>
                    </div>
                  </div>

                  {/* Previous / Next Navigation Pills */}
                  <div className="flex items-center justify-between gap-4 pt-2">
                    {prevSection ? (
                      <button
                        onClick={() => setActiveSection(prevSection.key)}
                        className="px-4 py-2 text-xs font-bold font-['Space_Grotesk'] rounded-lg border border-[#E6E2D8] bg-white hover:bg-[#F0EDE8] text-[#5C5347] flex items-center gap-1.5 transition-colors"
                      >
                        <ChevronLeft size={15} />
                        <span>Sebelumnya: {prevSection.shortLabel}</span>
                      </button>
                    ) : (
                      <div />
                    )}

                    {nextSection ? (
                      <button
                        onClick={() => setActiveSection(nextSection.key)}
                        className="px-4 py-2 text-xs font-bold font-['Space_Grotesk'] rounded-lg bg-[#0F766E] hover:bg-[#115E59] text-white flex items-center gap-1.5 shadow-xs transition-colors"
                      >
                        <span>Lanjut: {nextSection.shortLabel}</span>
                        <ChevronRight size={15} />
                      </button>
                    ) : (
                      <button
                        onClick={() => setActiveSection('challenge')}
                        className="px-4 py-2 text-xs font-bold font-['Space_Grotesk'] rounded-lg bg-[#F59E0B] hover:bg-[#D97706] text-white flex items-center gap-1.5 shadow-xs transition-colors"
                      >
                        <span>Lanjut ke Tantangan CTF</span>
                        <Trophy size={14} />
                      </button>
                    )}
                  </div>
                </div>
              ) : (
                /* View Section: CTF Challenge Mode */
                <div className="space-y-6">
                  {/* Challenge Mission Objective Banner */}
                  <div className="p-5 rounded-xl border border-[#FDBA74] bg-[#FFEDD5]/80 text-[#9A3412] shadow-xs space-y-2">
                    <div className="flex items-center gap-2 font-['Space_Grotesk'] font-bold text-sm text-[#C2410C]">
                      <Trophy size={18} />
                      <span>MISI & SASARAN TANTANGAN CTF</span>
                    </div>
                    <div className="prose-zitera max-w-none text-xs font-sans text-[#7C2D12]">
                      <ReactMarkdown remarkPlugins={[remarkGfm]}>
                        {content.challengeObjective}
                      </ReactMarkdown>
                    </div>
                  </div>

                  {/* Target Connection Card */}
                  <div className="p-4 rounded-xl border border-[#E6E2D8] bg-white flex flex-wrap items-center justify-between gap-3 text-xs">
                    <div>
                      <span className="font-bold font-['Space_Grotesk'] text-[#1E1A14]">
                        Target URL Sandbox:
                      </span>{' '}
                      <code className="px-2 py-1 rounded bg-[#F0EDE8] text-[#0F766E] font-['JetBrains_Mono']">
                        http://127.0.0.1:{targetPort}
                      </code>
                    </div>
                    <button
                      onClick={() => openBrowser(`http://127.0.0.1:${targetPort}`)}
                      className="px-3 py-1 bg-[#F0EDE8] hover:bg-[#E6E2D8] text-[#0F766E] font-bold rounded flex items-center gap-1 font-['Space_Grotesk']"
                    >
                      <span>Buka di Peramban</span>
                      <ExternalLink size={12} />
                    </button>
                  </div>

                  {/* CTF Flag Submission Form */}
                  <form
                    onSubmit={handleFlagSubmit}
                    className="p-5 rounded-xl border border-[#E6E2D8] bg-white shadow-xs space-y-3"
                  >
                    <label className="block text-xs font-['Space_Grotesk'] font-bold text-[#1E1A14]">
                      SUBMIT CAPTURED CTF FLAG
                    </label>
                    <div className="flex items-center gap-2">
                      <input
                        type="text"
                        value={flagInput}
                        onChange={(e) => setFlagInput(e.target.value)}
                        placeholder="ZITERA{...} atau FLAG{...}"
                        className="flex-1 px-3.5 py-2.5 text-xs bg-[#FCFBF8] border border-[#E6E2D8] rounded-lg focus:outline-none focus:border-[#2DD4BF] font-['JetBrains_Mono']"
                      />
                      <button
                        type="submit"
                        disabled={submittingFlag || !flagInput.trim()}
                        className="px-5 py-2.5 bg-[#0F766E] hover:bg-[#115E59] text-white text-xs font-['Space_Grotesk'] font-bold rounded-lg shadow-xs transition-all disabled:opacity-50 flex items-center gap-1.5"
                      >
                        <span>{submittingFlag ? 'Verifikasi...' : 'SUBMIT FLAG'}</span>
                        <Send size={13} />
                      </button>
                    </div>

                    {flagResult && (
                      <div
                        className={`p-3 rounded-lg border text-xs font-['JetBrains_Mono'] flex items-center gap-2 ${
                          flagResult.passed
                            ? 'bg-[#E6F9F2] border-[#86EFAC] text-[#16A34A]'
                            : 'bg-[#FEE2E2] border-[#FCA5A5] text-[#DC2626]'
                        }`}
                      >
                        {flagResult.passed ? <CheckCircle2 size={16} /> : <AlertCircle size={16} />}
                        <span>{flagResult.message}</span>
                      </div>
                    )}
                  </form>

                  {/* Progressive Hints Section */}
                  <div className="space-y-3">
                    <div className="flex items-center justify-between text-xs font-['Space_Grotesk'] font-bold text-[#5C5347]">
                      <div className="flex items-center gap-2">
                        <Lightbulb size={16} className="text-[#F59E0B]" />
                        <span>PETUNJUK BERTAHAP / PROGRESSIVE HINTS ({content.hints.length})</span>
                      </div>
                      <span className="text-[11px] font-['JetBrains_Mono'] text-[#9C9080]">
                        {revealedHints.length}/{content.hints.length} Terbuka
                      </span>
                    </div>

                    <div className="space-y-2">
                      {content.hints.map((hint) => {
                        const isRevealed = revealedHints.includes(hint.tier)
                        return (
                          <div
                            key={hint.tier}
                            className="p-4 rounded-xl border border-[#E6E2D8] bg-white text-xs font-['JetBrains_Mono'] shadow-2xs"
                          >
                            <div className="flex items-center justify-between">
                              <span className="font-bold text-[#0F766E]">
                                TIER {hint.tier} // {hint.type.toUpperCase()}
                              </span>
                              {!isRevealed ? (
                                <button
                                  onClick={() => revealHint(hint.tier)}
                                  className="px-3 py-1 text-[11px] bg-[#FEF3C7] border border-[#FCD34D] text-[#B45309] rounded hover:bg-[#FDE68A] font-bold font-['Space_Grotesk'] transition-colors"
                                >
                                  Buka Petunjuk
                                </button>
                              ) : (
                                <span className="text-[10px] text-[#16A34A] font-bold px-2 py-0.5 rounded bg-[#DCFCE7]">
                                  TERBUKA
                                </span>
                              )}
                            </div>
                            {isRevealed ? (
                              <p className="mt-2.5 text-[#5C5347] bg-[#F7F5F0] p-3 rounded-lg leading-relaxed border border-[#E6E2D8]">
                                {hint.hint}
                              </p>
                            ) : (
                              <p className="mt-1 text-[#9C9080] italic text-[11px]">
                                Petunjuk terkunci. Buka jika kamu membutuhkan bantuan investigasi.
                              </p>
                            )}
                          </div>
                        )
                      })}
                    </div>
                  </div>

                  {/* Return to Curriculum Button */}
                  <div className="pt-2">
                    <button
                      onClick={() => setActiveSection('remediation')}
                      className="px-4 py-2 text-xs font-bold font-['Space_Grotesk'] rounded-lg border border-[#E6E2D8] bg-white hover:bg-[#F0EDE8] text-[#5C5347] flex items-center gap-1.5 transition-colors"
                    >
                      <ChevronLeft size={15} />
                      <span>Kembali ke: Remediasi Kode</span>
                    </button>
                  </div>
                </div>
              )}
            </div>
          </div>
        </div>

        {/* Right Column: Hacker Tilix Terminal Widget (Collapsible) */}
        {terminalOpen && (
          <div className="w-full lg:w-[440px] xl:w-[480px] shrink-0 h-80 lg:h-full bg-[#090D16] border-t lg:border-t-0 border-[#25334D] flex flex-col overflow-hidden animate-in fade-in duration-150">
            {/* Terminal Titlebar */}
            <div className="px-4 py-2.5 bg-[#101726] border-b border-[#25334D] flex items-center justify-between text-xs text-[#9CA3AF]">
              <div className="flex items-center gap-2">
                <span className="w-2.5 h-2.5 rounded-full bg-[#EF4444]" />
                <span className="w-2.5 h-2.5 rounded-full bg-[#F59E0B]" />
                <span className="w-2.5 h-2.5 rounded-full bg-[#10B981]" />
                <span className="font-['JetBrains_Mono'] text-[11px] text-[#38BDF8] ml-2">
                  tilix://{labId.toLowerCase()} : {targetPort}
                </span>
              </div>
              <div className="flex items-center gap-3">
                <button
                  onClick={() => setTerminalHistory([])}
                  className="text-[10px] text-[#9CA3AF] hover:text-white transition-colors font-['JetBrains_Mono']"
                  title="Clear terminal"
                >
                  CLEAR
                </button>
                <button
                  onClick={() => setTerminalOpen(false)}
                  className="text-[10px] text-[#9CA3AF] hover:text-white transition-colors"
                  title="Sembunyikan Terminal"
                >
                  <PanelRightClose size={13} />
                </button>
              </div>
            </div>

            {/* Terminal Output Stream */}
            <div className="flex-1 p-4 overflow-y-auto space-y-1 font-['JetBrains_Mono'] text-xs text-[#E5E7EB]">
              {terminalHistory.map((item, idx) => (
                <div
                  key={idx}
                  className={`terminal-line break-all whitespace-pre-wrap ${
                    item.type === 'input'
                      ? 'text-[#2DD4BF] font-bold'
                      : item.type === 'error'
                      ? 'text-[#EF4444]'
                      : 'text-[#9CA3AF]'
                  }`}
                >
                  {item.text}
                </div>
              ))}
              {executing && (
                <div className="text-[11px] text-[#38BDF8] animate-pulse">
                  [Executing in sandbox environment...]
                </div>
              )}
              <div ref={terminalBottomRef} />
            </div>

            {/* Terminal Input Prompt */}
            <form
              onSubmit={handleTerminalSubmit}
              className="p-2.5 bg-[#101726] border-t border-[#25334D] flex items-center gap-2"
            >
              <span className="text-[#2DD4BF] font-['JetBrains_Mono'] text-xs font-bold">$</span>
              <input
                type="text"
                value={terminalInput}
                onChange={(e) => setTerminalInput(e.target.value)}
                placeholder={`curl http://127.0.0.1:${targetPort}/`}
                className="flex-1 bg-transparent border-none text-xs text-white font-['JetBrains_Mono'] focus:outline-none"
              />
            </form>
          </div>
        )}
      </div>
    </div>
  )
}
