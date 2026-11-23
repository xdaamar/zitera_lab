import { useState, useEffect, useRef, type FormEvent } from 'react'
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
  Loader2,
  BookOpen,
  Unlock,
  Award,
  Lock,
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
import { getCurrentUser } from './authService'
import { CuteAnimeLoading } from './CuteAnimeLoading'
import { getInteractiveCurriculumForLab, getBugBountyReportForLab } from './lessonInteractiveData'
import {
  AnalogyCard,
  CoreConceptCard,
  InteractiveWalkthroughCard,
  RemediationCard,
  ArchitectureFlowCard,
  BugBountyReportCard,
} from './LessonCards'
import { VideoDrawerModal, YouTubeIcon } from './VideoDrawerModal'
import { HackerOneTriagerCard } from './HackerOneTriagerCard'
import { LessonMarkdownRenderer } from './LessonMarkdownRenderer'
import { ZetaMentorModal } from './ZetaMentorModal'
import {
  getLabProgression,
  recordLabSolveProgression,
  canUserAccessLab,
} from './progressionHelper'

interface LabDetailViewProps {
  labId: string
  onBack: () => void
  onSelectLab?: (labId: string) => void
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

export function LabDetailView({ labId, onBack, onSelectLab }: LabDetailViewProps) {
  const currentUser = getCurrentUser()
  const tierAccess = canUserAccessLab(labId, currentUser?.tier)

  const [content, setContent] = useState<LabContent | null>(null)
  const [status, setStatus] = useState<LabItem | null>(null)
  const [activeSection, setActiveSection] = useState<SectionKey>('introduction')
  const [loading, setLoading] = useState(true)

  // Collapsible Terminal Panel State (Default hidden per UX requirement to keep curriculum reading distraction-free)
  const [terminalOpen, setTerminalOpen] = useState(false)

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

  // Video Bootcamp Modal State
  const [videoModalOpen, setVideoModalOpen] = useState(false)

  // Zeta Socratic AI Mentor Floating Modal State
  const [zetaMentorOpen, setZetaMentorOpen] = useState(false)

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
  const scrollContainerRef = useRef<HTMLDivElement | null>(null)

  // Target Operation Modal & Anti-Spam state
  const [targetOp, setTargetOp] = useState<{
    isOpen: boolean
    type: 'launch' | 'stop'
    status: 'progress' | 'success' | 'error'
    step: number
    errorMessage?: string
  }>({
    isOpen: false,
    type: 'launch',
    status: 'progress',
    step: 1,
  })

  const goToSection = (secKey: SectionKey) => {
    setActiveSection(secKey)
    if (scrollContainerRef.current) {
      scrollContainerRef.current.scrollTo({ top: 0, behavior: 'smooth' })
    }
  }

  // Requirement: Otomatis scroll ke paling atas saat berpindah tab atau lab
  useEffect(() => {
    if (scrollContainerRef.current) {
      scrollContainerRef.current.scrollTo({ top: 0, behavior: 'smooth' })
    }
  }, [activeSection, labId])

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
    if (!status || targetOp.isOpen) return
    const isStopping = status.running

    if (isStopping) {
      setTargetOp({
        isOpen: true,
        type: 'stop',
        status: 'progress',
        step: 1,
      })
      try {
        await stopLab(labId)
        const updated = await getLabStatus(labId)
        setStatus(updated)
        setTargetOp((prev) => ({ ...prev, status: 'success', step: 3 }))
        setTimeout(() => {
          setTargetOp((prev) => ({ ...prev, isOpen: false }))
        }, 800)
      } catch (err: unknown) {
        setTargetOp((prev) => ({
          ...prev,
          status: 'error',
          errorMessage: err instanceof Error ? err.message : String(err) || 'Gagal menghentikan lab target.',
        }))
      }
    } else {
      // Authoritative Tier Check before launching Target Sandbox
      if (!tierAccess.allowed) {
        setTargetOp({
          isOpen: true,
          type: 'launch',
          status: 'error',
          step: 1,
          errorMessage:
            tierAccess.reason ||
            'AKSES DITOLAK: Sandbox target ini hanya tersedia untuk akun Pro Edition.',
        })
        return
      }

      // Launching Target with progressive steps & loading popup
      setTargetOp({
        isOpen: true,
        type: 'launch',
        status: 'progress',
        step: 1,
      })

      const timer1 = setTimeout(() => {
        setTargetOp((prev) => (prev.isOpen && prev.status === 'progress' ? { ...prev, step: 2 } : prev))
      }, 700)
      const timer2 = setTimeout(() => {
        setTargetOp((prev) => (prev.isOpen && prev.status === 'progress' ? { ...prev, step: 3 } : prev))
      }, 1500)

      try {
        await startLab(labId)
        clearTimeout(timer1)
        clearTimeout(timer2)
        const updated = await getLabStatus(labId)
        setStatus(updated)
        setTargetOp((prev) => ({ ...prev, status: 'success', step: 3 }))
        setTimeout(() => {
          setTargetOp((prev) => ({ ...prev, isOpen: false }))
        }, 900)
      } catch (err: unknown) {
        clearTimeout(timer1)
        clearTimeout(timer2)
        setTargetOp((prev) => ({
          ...prev,
          status: 'error',
          errorMessage: err instanceof Error ? err.message : String(err) || 'Gagal meluncurkan sandbox target.',
        }))
      }
    }
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
      } catch {}
      return next
    })
  }

  const handleFlagSubmit = async (e: FormEvent) => {
    e.preventDefault()
    if (!flagInput.trim()) return

    // Authoritative Tier Check before flag validation and score recording
    if (!tierAccess.allowed) {
      setFlagResult({
        passed: false,
        message: 'Akses Ditolak: Akun tier Foundation tidak memiliki izin mencatat skor CTF pada Track ini.',
      })
      return
    }

    setSubmittingFlag(true)
    setFlagResult(null)

    try {
      const res = await validateChallenge(labId, flagInput)
      if (res.status === 'passed') {
        setFlagResult({ passed: true, message: res.message || 'CTF FLAG CAPTURED! +100 PTS' })
        setChallengeSolved(true)
        recordLabSolveProgression(labId)

        // Trigger confetti celebration!
        confetti({
          particleCount: 160,
          spread: 85,
          origin: { y: 0.6 },
          colors: ['#2DD4BF', '#0F766E', '#F59E0B', '#3B82F6', '#10B981'],
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
            className="w-full h-full object-cover object-center opacity-80"
            onError={(e) => {
              ;(e.target as HTMLElement).style.display = 'none'
            }}
          />
          <div className="absolute inset-0 bg-[#FCFBF8]/55 cyber-dot-grid" />
        </div>
        <div className="relative z-10">
          <CuteAnimeLoading
            message="MEMPERSIAPKAN MATERI KURIKULUM & TARGET..."
            subMessage="DEPLOYING SANDBOX ENVIRONMENT"
          />
        </div>
      </div>
    )
  }

  const targetPort = status?.port ?? content.manifest.defaultPort ?? 8011
  const activeTargetUrl = status?.url || `http://127.0.0.1:${targetPort}/`
  const briefing = getMissionBriefing(labId)
  const progression = getLabProgression(labId)
  const interactiveCurriculum = getInteractiveCurriculumForLab(labId, targetPort)
  const bugBountyReport = getBugBountyReportForLab(labId, targetPort)

  const handleSendToTerminal = (cmd: string) => {
    setTerminalOpen(true)
    setTerminalInput(cmd)
    setTerminalHistory((prev) => [
      ...prev.slice(-248),
      { type: 'output', text: `[Tilix Terminal] Perintah siap dieksekusi: ${cmd}` },
    ])
  }

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
              <div className="flex flex-wrap items-center gap-2">
                <h1 className="text-sm font-bold font-['Space_Grotesk'] text-[#1E1A14]">
                  {content.manifest.title}
                </h1>
                <span className="text-[11px] text-[#78716C] font-['JetBrains_Mono'] hidden md:inline">
                  • {briefing.targetAppName}
                </span>

                {/* Requirement 1: Prerequisite & Progression Breadcrumbs */}
                {progression.isCore ? (
                  <span className="px-2.5 py-0.5 rounded-full bg-[#DCFCE7] text-[#15803D] border border-[#86EFAC] text-[10px] font-bold font-['JetBrains_Mono'] flex items-center gap-1.5 shadow-2xs">
                    <Shield size={12} className="text-[#15803D] shrink-0" />
                    <span>LAB WAJIB (Pondasi Dasar & Konsep Inti)</span>
                  </span>
                ) : (
                  <span className="px-2.5 py-0.5 rounded-full bg-[#EDE9FE] text-[#6D28D9] border border-[#D8B4FE] text-[10px] font-bold font-['JetBrains_Mono'] flex items-center gap-1.5 shadow-2xs">
                    <Compass size={12} className="text-[#6D28D9] shrink-0" />
                    <span>
                      LAB EKSPLORASI (Teknik Lanjutan) • Prasyarat:{' '}
                      <span className="underline decoration-[#A78BFA]">{progression.prerequisiteTitle}</span>{' '}
                      (Lulus/Terselesaikan)
                    </span>
                  </span>
                )}
              </div>
            </div>
          </div>
        </div>

        {/* Right: Actions & Collapsible Terminal Toggle */}
        <div className="flex items-center gap-2.5">
          {/* Target Service Controller with Anti-Spam Loading Indicator */}
          {status?.running ? (
            <>
              <button
                onClick={() => openBrowser(activeTargetUrl)}
                className="px-3 py-1.5 bg-[#0F766E] hover:bg-[#115E59] text-white text-xs font-bold font-['Space_Grotesk'] rounded-lg flex items-center gap-1.5 shadow-xs transition-colors cursor-pointer"
                title={`Buka ${activeTargetUrl} di browser`}
              >
                <span>Buka Web (: {targetPort})</span>
                <ExternalLink size={13} />
              </button>
              <button
                onClick={handleToggleTarget}
                disabled={targetOp.isOpen}
                className="px-3 py-1.5 bg-white border border-[#EF4444] text-[#EF4444] hover:bg-[#FEE2E2] text-xs font-bold font-['Space_Grotesk'] rounded-lg flex items-center gap-1 transition-colors cursor-pointer disabled:opacity-50 disabled:cursor-not-allowed"
                title="Hentikan Target Sandbox"
              >
                {targetOp.isOpen && targetOp.type === 'stop' ? (
                  <Loader2 size={13} className="animate-spin text-[#EF4444]" />
                ) : (
                  <Square size={13} />
                )}
                <span>{targetOp.isOpen && targetOp.type === 'stop' ? 'STOPPING...' : 'Stop'}</span>
              </button>
            </>
          ) : !tierAccess.allowed ? (
            <button
              onClick={() => {
                const username = currentUser?.username || 'user'
                const msg = encodeURIComponent(
                  `Halo Admin Zitera Lab, saya ingin upgrade akun (username: ${username}) ke Pro Edition untuk membuka seluruh 45 lab!`,
                )
                openBrowser(`https://wa.me/6282283088667?text=${msg}`)
              }}
              className="px-3.5 py-1.5 bg-[#D97706] hover:bg-[#B45309] text-white text-xs font-bold font-['Space_Grotesk'] rounded-lg flex items-center gap-1.5 shadow-[0_2px_0_0_#92400E] active:translate-y-0.5 transition-all cursor-pointer"
              title="Terkunci: Khusus Pro Edition"
            >
              <Lock size={13} />
              <span>UPGRADE KE PRO</span>
            </button>
          ) : (
            <button
              onClick={handleToggleTarget}
              disabled={targetOp.isOpen}
              className="px-3.5 py-1.5 bg-[#0F766E] hover:bg-[#115E59] active:bg-[#094D47] text-white text-xs font-bold font-['Space_Grotesk'] rounded-lg flex items-center gap-1.5 shadow-[0_2px_0_0_#063934] active:translate-y-0.5 transition-all cursor-pointer disabled:opacity-50 disabled:cursor-not-allowed"
              title="Jalankan Target Sandbox"
            >
              {targetOp.isOpen && targetOp.type === 'launch' ? (
                <Loader2 size={13} className="animate-spin text-[#5EEAD4]" />
              ) : (
                <Play size={13} />
              )}
              <span>{targetOp.isOpen && targetOp.type === 'launch' ? 'LAUNCHING...' : 'LAUNCH TARGET'}</span>
            </button>
          )}

          <div className="h-4 w-px bg-[#E6E2D8]" />

          {/* Video Bootcamp YouTube Button */}
          <button
            onClick={() => setVideoModalOpen(true)}
            className="px-3 py-1.5 rounded-lg border border-[#FECACA] bg-[#FEF2F2] hover:bg-[#FEE2E2] text-[#DC2626] text-xs font-bold font-['Space_Grotesk'] flex items-center gap-1.5 transition-all shadow-2xs cursor-pointer"
            title="Tonton Video Bootcamp YouTube"
          >
            <YouTubeIcon size={15} className="text-[#DC2626]" />
            <span>Video Bootcamp</span>
          </button>

          {/* Requirement 1: Collapsible Terminal Panel Toggle Button */}
          <button
            onClick={() => setTerminalOpen((prev) => !prev)}
            className={`px-3 py-1.5 rounded-lg border text-xs font-bold font-['Space_Grotesk'] flex items-center gap-1.5 transition-all cursor-pointer ${
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
      <div className="border-b border-[#E6E2D8] bg-[#FCFBF8] px-4 md:px-6 pt-2 shrink-0 flex items-center justify-between gap-3 overflow-hidden">
        {/* Scrollable Tabs Row */}
        <div className="flex items-center gap-1 overflow-x-auto scrollbar-none min-w-0">
          {CURRICULUM_SECTIONS.map((sec, idx) => {
            const Icon = sec.icon
            const isActive = activeSection === sec.key
            const isDone = completedSections.has(sec.key)
            return (
              <button
                key={sec.key}
                onClick={() => goToSection(sec.key)}
                className={`flex items-center gap-1.5 px-3 py-2 text-xs font-['Space_Grotesk'] font-bold rounded-t-lg transition-all border-t border-x cursor-pointer shrink-0 ${
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
            onClick={() => goToSection('challenge')}
            className={`flex items-center gap-1.5 px-3.5 py-2 text-xs font-['Space_Grotesk'] font-bold rounded-t-lg transition-all border-t border-x cursor-pointer shrink-0 ${
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

        {/* Section Completion Progress Counter — Always Pinned on Right, Never Truncated */}
        <div className="flex items-center gap-1.5 px-2.5 py-1 mb-1 rounded-md bg-[#CCFBF1] border border-[#2DD4BF]/40 text-[11px] font-['JetBrains_Mono'] font-bold text-[#0F766E] shrink-0 whitespace-nowrap shadow-2xs">
          <Sparkles size={12} />
          <span>Materi: {completedSections.size}/6 Selesai</span>
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
              className="w-full h-full object-cover object-center opacity-85"
              onError={(e) => {
                ;(e.target as HTMLElement).style.display = 'none'
              }}
            />
            <div className="absolute inset-0 bg-[#FCFBF8]/50 cyber-dot-grid" />
          </div>

          {/* Reading Scrollable Content */}
          <div
            ref={scrollContainerRef}
            className="relative z-10 h-full overflow-y-auto p-5 md:p-8 scroll-smooth"
          >
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

                  {/* Section 2: Dedicated Tactile Analogy Card */}
                  {activeSection === 'analogy' && (
                    <AnalogyCard data={interactiveCurriculum.analogy} />
                  )}

                  {/* Section 3: Dedicated Tactile Core Concept Card */}
                  {activeSection === 'concept' && (
                    <CoreConceptCard data={interactiveCurriculum.concept} />
                  )}

                  {/* Section 4: Dedicated Tactile System Architecture Card */}
                  {activeSection === 'architecture' && (
                    <ArchitectureFlowCard targetPort={targetPort} />
                  )}

                  {/* Section 5: Dedicated Interactive Step-by-Step Walkthrough Card */}
                  {activeSection === 'walkthrough' && (
                    !tierAccess.allowed ? (
                      <div className="p-8 rounded-2xl border-2 border-[#F59E0B] bg-[#FFFBEB] shadow-[0_4px_0_0_#D97706] text-center space-y-4 animate-in fade-in duration-150">
                        <div className="w-14 h-14 rounded-2xl bg-[#FEF3C7] border-2 border-[#F59E0B] text-[#D97706] flex items-center justify-center mx-auto shadow-[0_2px_0_0_#D97706]">
                          <Lock size={26} />
                        </div>
                        <div>
                          <span className="px-2.5 py-0.5 rounded-full bg-[#FEF3C7] border border-[#FCD34D] text-[#B45309] text-[10px] font-['JetBrains_Mono'] font-bold uppercase tracking-wider">
                            PRO EDITION EXCLUSIVE // WALKTHROUGH LOCKED
                          </span>
                          <h3 className="text-lg font-bold font-['Space_Grotesk'] text-[#92400E] mt-1.5">
                            Panduan Eksploitasi & Payload Step-by-Step Terkunci
                          </h3>
                          <p className="text-xs text-[#78350F] max-w-lg mx-auto leading-relaxed mt-1">
                            Langkah-langkah exploit interaktif, payload cURL/HTTP, analisis server-side, serta laporan bug bounty untuk laboratorium ini merupakan materi eksklusif paket Pro Edition.
                          </p>
                        </div>
                        <button
                          onClick={() => {
                            const username = currentUser?.username || 'user'
                            const msg = encodeURIComponent(
                              `Halo Admin Zitera Lab, saya ingin upgrade akun (username: ${username}) ke Pro Edition untuk membuka seluruh 45 lab!`,
                            )
                            openBrowser(`https://wa.me/6282283088667?text=${msg}`)
                          }}
                          className="px-6 py-3 bg-[#D97706] hover:bg-[#B45309] text-white border-2 border-[#B45309] shadow-[0_3px_0_0_#92400E] hover:translate-y-0.5 active:translate-y-1 rounded-xl text-xs font-bold font-['Space_Grotesk'] inline-flex items-center gap-2 cursor-pointer transition-all"
                        >
                          <Sparkles size={15} />
                          <span>Upgrade ke Pro Edition via WhatsApp</span>
                        </button>
                      </div>
                    ) : (
                      <>
                        <InteractiveWalkthroughCard
                          data={interactiveCurriculum.walkthrough}
                          targetPort={targetPort}
                          onSendToTerminal={handleSendToTerminal}
                        />
                        <BugBountyReportCard
                          report={bugBountyReport}
                          onSendToTerminal={handleSendToTerminal}
                        />
                      </>
                    )
                  )}

                  {/* Section 6: Dedicated Remediation Code Comparison Card */}
                  {activeSection === 'remediation' && (
                    <RemediationCard data={interactiveCurriculum.remediation} />
                  )}

                  {/* Comprehensive Deep-Dive Lesson Reading Container with Anti-Fatigue Typography */}
                  <div className="rounded-2xl border-2 border-[#E6E2D8] bg-white shadow-[0_4px_0_0_#CEC8BF,0_12px_24px_rgba(0,0,0,0.03)] p-6 md:p-8 space-y-4">
                    <div className="flex items-center justify-between pb-3 border-b border-[#E6E2D8] text-xs font-['Space_Grotesk'] font-bold text-[#78716C]">
                      <div className="flex items-center gap-2">
                        <BookOpen size={16} className="text-[#0F766E]" />
                        <span className="uppercase tracking-wider">
                          MODUL MATERI LENGKAP & DEEP DIVE TUTORIAL
                        </span>
                      </div>
                      <span className="font-['JetBrains_Mono'] text-[11px] text-[#9C9080]">
                        ZITERA CURRICULUM // ID
                      </span>
                    </div>

                    <LessonMarkdownRenderer
                      markdown={
                        currentLessonMarkdown ||
                        `# ${content.manifest.title}\n\nMateri kurikulum sedang dimuat...`
                      }
                      onSendToTerminal={handleSendToTerminal}
                    />
                  </div>

                  {/* Requirement: Menu Tanda Selesai & Navigasi di Bawah (User tidak perlu scroll ke atas untuk menandai selesai!) */}
                  <div className="p-5 md:p-6 rounded-2xl border-2 border-[#E6E2D8] bg-[#F7F5F0] shadow-sm space-y-4 select-none">
                    <div className="flex flex-col sm:flex-row items-center justify-between gap-4">
                      {/* Section Status Info */}
                      <div className="flex items-center gap-3 w-full sm:w-auto">
                        <div
                          className={`w-11 h-11 rounded-xl flex items-center justify-center border-2 transition-all ${
                            isCurrentSectionCompleted
                              ? 'bg-[#DCFCE7] border-[#86EFAC] text-[#16A34A] shadow-[0_0_12px_rgba(22,163,74,0.2)]'
                              : 'bg-white border-[#E6E2D8] text-[#9C9080]'
                          }`}
                        >
                          {isCurrentSectionCompleted ? (
                            <Check size={22} className="stroke-[3]" />
                          ) : (
                            <CheckCircle2 size={22} />
                          )}
                        </div>
                        <div>
                          <div className="text-xs font-bold font-['Space_Grotesk'] text-[#1E1A14]">
                            {isCurrentSectionCompleted
                              ? 'Bab Ini Telah Selesai Dipahami'
                              : 'Sudah Selesai Membaca Bab Ini?'}
                          </div>
                          <div className="text-[11px] text-[#78716C] font-['JetBrains_Mono']">
                            {isCurrentSectionCompleted
                              ? 'Progres pembelajaran tersimpan secara otomatis.'
                              : 'Tandai selesai untuk menyimpan progres pemahamanmu.'}
                          </div>
                        </div>
                      </div>

                      {/* Interactive Mark Complete Toggle Button */}
                      <button
                        onClick={() => toggleSectionCompleted(activeSection)}
                        className={`w-full sm:w-auto px-4 py-2.5 text-xs font-bold font-['Space_Grotesk'] rounded-xl border-2 transition-all cursor-pointer flex items-center justify-center gap-2 shadow-xs ${
                          isCurrentSectionCompleted
                            ? 'bg-white border-[#86EFAC] text-[#16A34A] hover:bg-[#F0FDF4] hover:shadow-[0_2px_0_0_#86EFAC]'
                            : 'bg-[#10B981] border-[#059669] text-white hover:bg-[#059669] shadow-[0_3px_0_0_#047857] hover:shadow-[0_1px_0_0_#047857] hover:translate-y-0.5 active:translate-y-1'
                        }`}
                      >
                        {isCurrentSectionCompleted ? (
                          <>
                            <Check size={15} />
                            <span>Batalkan Tanda Selesai</span>
                          </>
                        ) : (
                          <>
                            <Check size={15} />
                            <span>Tandai Selesai Dipahami</span>
                          </>
                        )}
                      </button>
                    </div>

                    {/* Divider */}
                    <div className="h-px bg-[#E6E2D8]" />

                    {/* Previous / Next / Complete & Advance Navigation Bar */}
                    <div className="flex flex-wrap items-center justify-between gap-3 pt-1">
                      {prevSection ? (
                        <button
                          onClick={() => goToSection(prevSection.key)}
                          className="px-4 py-2.5 text-xs font-bold font-['Space_Grotesk'] rounded-lg border-2 border-[#E6E2D8] bg-white hover:bg-[#F0EDE8] hover:border-[#CEC8BF] text-[#5C5347] flex items-center gap-2 transition-all cursor-pointer shadow-xs hover:translate-y-0.5"
                        >
                          <ChevronLeft size={16} />
                          <span>Sebelumnya: {prevSection.shortLabel}</span>
                        </button>
                      ) : (
                        <div />
                      )}

                      <div className="flex items-center gap-2.5">
                        {nextSection ? (
                          <>
                            {/* Fast shortcut: Tandai Selesai & Lanjut */}
                            {!isCurrentSectionCompleted && (
                              <button
                                onClick={() => {
                                  toggleSectionCompleted(activeSection)
                                  goToSection(nextSection.key)
                                }}
                                className="px-4 py-2.5 text-xs font-bold font-['Space_Grotesk'] rounded-lg border-2 border-[#2DD4BF] bg-[#CCFBF1] hover:bg-[#99F6E4] text-[#0F766E] flex items-center gap-1.5 shadow-[0_2px_0_0_#5EEAD4] hover:shadow-[0_1px_0_0_#5EEAD4] hover:translate-y-0.5 transition-all cursor-pointer"
                                title="Tandai bab ini selesai dan langsung lanjut ke bab berikutnya"
                              >
                                <Check size={14} />
                                <span>Tandai & Lanjut</span>
                              </button>
                            )}

                            <button
                              onClick={() => goToSection(nextSection.key)}
                              className="px-5 py-2.5 text-xs font-bold font-['Space_Grotesk'] rounded-lg bg-[#0F766E] hover:bg-[#115E59] text-white border-2 border-[#094D47] shadow-[0_3px_0_0_#063934] hover:shadow-[0_1px_0_0_#063934] hover:translate-y-0.5 active:translate-y-1 flex items-center gap-2 transition-all cursor-pointer"
                            >
                              <span>Lanjut: {nextSection.shortLabel}</span>
                              <ChevronRight size={16} />
                            </button>
                          </>
                        ) : (
                          <>
                            {!isCurrentSectionCompleted && (
                              <button
                                onClick={() => {
                                  toggleSectionCompleted(activeSection)
                                  goToSection('challenge')
                                }}
                                className="px-4 py-2.5 text-xs font-bold font-['Space_Grotesk'] rounded-lg border-2 border-[#FCD34D] bg-[#FEF3C7] hover:bg-[#FDE68A] text-[#B45309] flex items-center gap-1.5 shadow-[0_2px_0_0_#FCD34D] hover:shadow-[0_1px_0_0_#FCD34D] hover:translate-y-0.5 transition-all cursor-pointer"
                              >
                                <Check size={14} />
                                <span>Tandai & Buka CTF</span>
                              </button>
                            )}

                            <button
                              onClick={() => goToSection('challenge')}
                              className="px-5 py-2.5 text-xs font-bold font-['Space_Grotesk'] rounded-lg bg-[#F59E0B] hover:bg-[#D97706] text-white border-2 border-[#B45309] shadow-[0_3px_0_0_#78350F] hover:shadow-[0_1px_0_0_#78350F] hover:translate-y-0.5 active:translate-y-1 flex items-center gap-2 transition-all cursor-pointer"
                            >
                              <span>Lanjut ke Tantangan CTF</span>
                              <Trophy size={16} />
                            </button>
                          </>
                        )}
                      </div>
                    </div>
                  </div>
                </div>
              ) : (
                /* View Section: CTF Challenge Mode */
                <div className="space-y-6">
                  {/* Requirement 2: Celebratory Progression Unlock Banner on CTF Solve */}
                  {challengeSolved && (
                    <div className="p-5 md:p-6 rounded-2xl border-2 border-[#10B981] bg-gradient-to-r from-[#ECFDF5] via-[#F0FDF4] to-[#CCFBF1] shadow-[0_4px_0_0_#059669,0_16px_32px_rgba(16,185,129,0.14)] space-y-3.5 animate-in zoom-in-95 duration-200">
                      <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-4">
                        <div className="flex items-center gap-3.5">
                          <div className="w-13 h-13 rounded-2xl bg-[#10B981] text-white flex items-center justify-center shadow-xs shrink-0">
                            <Award size={26} className="text-white" />
                          </div>
                          <div>
                            <span className="text-[10px] font-bold font-['JetBrains_Mono'] px-2.5 py-0.5 rounded-full bg-[#DCFCE7] text-[#15803D] border border-[#86EFAC] uppercase tracking-wider">
                              STATUS: TERCAPAI & TERVERIFIKASI (+100 PTS)
                            </span>
                            <h3 className="text-base md:text-lg font-bold font-['Space_Grotesk'] text-[#065F46] mt-1 flex items-center gap-1.5">
                              <Sparkles size={18} className="text-[#059669]" />
                              <span>Tantangan Terverifikasi! {progression.nextLabId ? 'Lab Eksplorasi Berikutnya Berhasil Dibuka!' : 'Seluruh Rangkaian Lab Kategori Ini Tuntas!'}</span>
                            </h3>
                          </div>
                        </div>

                        {progression.nextLabId ? (
                          <div className="flex items-center gap-2">
                            <div className="px-3.5 py-1.5 rounded-xl bg-white border-2 border-[#86EFAC] text-xs font-['JetBrains_Mono'] font-bold text-[#047857] shadow-2xs flex items-center gap-1.5">
                              <Unlock size={14} className="text-[#047857]" />
                              <span>Terbuka: {progression.nextLabId}</span>
                            </div>
                          </div>
                        ) : (
                          <div className="px-3.5 py-1.5 rounded-xl bg-white border-2 border-[#86EFAC] text-xs font-['JetBrains_Mono'] font-bold text-[#047857] shadow-2xs flex items-center gap-1.5">
                            <Trophy size={14} className="text-[#047857]" />
                            <span>Modul Kategori {progression.categoryCode} 100% Selesai!</span>
                          </div>
                        )}
                      </div>

                      {progression.nextLabId && (
                        <div className="p-3.5 rounded-xl bg-white/90 border border-[#A7F3D0] flex flex-wrap items-center justify-between gap-3 shadow-2xs">
                          <div className="text-xs text-[#065F46] font-['Plus_Jakarta_Sans']">
                            <span className="font-bold">Lab Eksplorasi Siap Diakses:</span>{' '}
                            <span className="font-['JetBrains_Mono'] font-bold text-[#0F766E]">{progression.nextLabId}</span>{' '}
                            — {progression.nextLabTitle}
                          </div>
                          {onSelectLab && (
                            <button
                              onClick={() => onSelectLab(progression.nextLabId!)}
                              className="px-4 py-2 rounded-lg bg-[#0F766E] hover:bg-[#115E59] active:bg-[#094D47] text-white text-xs font-['Space_Grotesk'] font-bold flex items-center gap-1.5 shadow-[0_2px_0_0_#063934] transition-all cursor-pointer"
                            >
                              <span>Buka {progression.nextLabId} Sekarang</span>
                              <ChevronRight size={15} />
                            </button>
                          )}
                        </div>
                      )}
                    </div>
                  )}

                  {/* Challenge Mission Objective Banner */}
                  <div className="p-5 rounded-xl border border-[#FDBA74] bg-[#FFEDD5]/80 text-[#9A3412] shadow-xs space-y-2">
                    <div className="flex items-center gap-2 font-['Space_Grotesk'] font-bold text-sm text-[#C2410C]">
                      <Trophy size={18} />
                      <span>MISI & SASARAN TANTANGAN CTF</span>
                    </div>
                    <div className="text-xs font-sans text-[#7C2D12]">
                      <LessonMarkdownRenderer
                        markdown={content.challengeObjective}
                        onSendToTerminal={handleSendToTerminal}
                      />
                    </div>
                  </div>

                  {!tierAccess.allowed ? (
                    <div className="p-8 rounded-2xl border-2 border-[#F59E0B] bg-[#FFFBEB] shadow-[0_4px_0_0_#D97706] text-center space-y-4 animate-in fade-in duration-150">
                      <div className="w-14 h-14 rounded-2xl bg-[#FEF3C7] border-2 border-[#F59E0B] text-[#D97706] flex items-center justify-center mx-auto shadow-[0_2px_0_0_#D97706]">
                        <Lock size={26} />
                      </div>
                      <div>
                        <span className="px-2.5 py-0.5 rounded-full bg-[#FEF3C7] border border-[#FCD34D] text-[#B45309] text-[10px] font-['JetBrains_Mono'] font-bold uppercase tracking-wider">
                          PRO EDITION EXCLUSIVE // CTF CHALLENGE LOCKED
                        </span>
                        <h3 className="text-lg font-bold font-['Space_Grotesk'] text-[#92400E] mt-1.5">
                          Tantangan CTF & Submit Flag Terkunci
                        </h3>
                        <p className="text-xs text-[#78350F] max-w-lg mx-auto leading-relaxed mt-1">
                          Verifikasi bendera CTF, peluncuran target sandbox langsung, dan pencatatan skor leaderboard pada laboratorium ini hanya dapat diakses oleh akun Pro Edition.
                        </p>
                      </div>
                      <button
                        onClick={() => {
                          const username = currentUser?.username || 'user'
                          const msg = encodeURIComponent(
                            `Halo Admin Zitera Lab, saya ingin upgrade akun (username: ${username}) ke Pro Edition untuk membuka seluruh 45 lab!`,
                          )
                          openBrowser(`https://wa.me/6282283088667?text=${msg}`)
                        }}
                        className="px-6 py-3 bg-[#D97706] hover:bg-[#B45309] text-white border-2 border-[#B45309] shadow-[0_3px_0_0_#92400E] hover:translate-y-0.5 active:translate-y-1 rounded-xl text-xs font-bold font-['Space_Grotesk'] inline-flex items-center gap-2 cursor-pointer transition-all"
                      >
                        <Sparkles size={15} />
                        <span>Upgrade ke Pro Edition via WhatsApp</span>
                      </button>
                    </div>
                  ) : (
                    <>
                      {/* Target Connection Card */}
                      <div className="p-4 rounded-xl border border-[#E6E2D8] bg-white flex flex-wrap items-center justify-between gap-3 text-xs">
                        <div>
                          <span className="font-bold font-['Space_Grotesk'] text-[#1E1A14]">
                            Target URL Sandbox:
                          </span>{' '}
                          <code className="px-2 py-1 rounded bg-[#F0EDE8] text-[#0F766E] font-['JetBrains_Mono']">
                            {activeTargetUrl}
                          </code>
                        </div>
                        {status?.running ? (
                          <button
                            onClick={() => openBrowser(activeTargetUrl)}
                            className="px-3 py-1 bg-[#F0EDE8] hover:bg-[#E6E2D8] text-[#0F766E] font-bold rounded flex items-center gap-1 font-['Space_Grotesk'] cursor-pointer"
                          >
                            <span>Buka di Peramban</span>
                            <ExternalLink size={12} />
                          </button>
                        ) : (
                          <button
                            onClick={handleToggleTarget}
                            disabled={targetOp.isOpen}
                            className="px-3.5 py-1.5 bg-[#0F766E] hover:bg-[#115E59] active:bg-[#094D47] text-white text-xs font-bold rounded-lg flex items-center gap-1.5 font-['Space_Grotesk'] shadow-xs cursor-pointer disabled:opacity-50 disabled:cursor-not-allowed transition-all"
                          >
                            {targetOp.isOpen && targetOp.type === 'launch' ? (
                              <Loader2 size={12} className="animate-spin text-[#5EEAD4]" />
                            ) : (
                              <Play size={12} />
                            )}
                            <span>{targetOp.isOpen && targetOp.type === 'launch' ? 'Memulai...' : 'Jalankan Target'}</span>
                          </button>
                        )}
                      </div>

                      {/* CTF Flag Submission Form */}
                      <form
                        onSubmit={handleFlagSubmit}
                        className="p-5 rounded-xl border border-[#E6E2D8] bg-white shadow-xs space-y-3"
                      >
                        <label className="block text-xs font-['Space_Grotesk'] font-bold text-[#1E1A14]">
                          Kirim Flag CTF yang Ditemukan (*Submit Flag*)
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
                            <span>{submittingFlag ? 'Verifikasi...' : 'Kirim Flag'}</span>
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
                    </>
                  )}

                  {/* HackerOne Bug Bounty Report Triage & Grading Card */}
                  <HackerOneTriagerCard
                    labId={labId}
                    referenceReport={bugBountyReport}
                    targetPort={targetPort}
                  />

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

                  {/* Return to Curriculum & Lab Navigation Card */}
                  <div className="p-4 rounded-xl border border-[#E6E2D8] bg-[#F7F5F0] flex flex-wrap items-center justify-between gap-3 select-none">
                    <button
                      onClick={() => goToSection('remediation')}
                      className="px-4 py-2.5 text-xs font-bold font-['Space_Grotesk'] rounded-lg border border-[#E6E2D8] bg-white hover:bg-[#F0EDE8] text-[#5C5347] flex items-center gap-2 transition-all cursor-pointer shadow-xs hover:translate-y-0.5"
                    >
                      <ChevronLeft size={16} />
                      <span>Kembali ke: Remediasi Kode</span>
                    </button>

                    <div className="flex items-center gap-2.5">
                      <button
                        onClick={onBack}
                        className="px-4 py-2.5 text-xs font-bold font-['Space_Grotesk'] rounded-lg border border-[#E6E2D8] bg-white hover:bg-[#F0EDE8] text-[#5C5347] transition-all cursor-pointer shadow-xs hover:translate-y-0.5"
                      >
                        Kembali ke Katalog Lab
                      </button>

                      {challengeSolved && progression.nextLabId && onSelectLab && (
                        <button
                          onClick={() => onSelectLab(progression.nextLabId!)}
                          className="px-4 py-2.5 text-xs font-bold font-['Space_Grotesk'] rounded-lg bg-[#0F766E] hover:bg-[#115E59] text-white flex items-center gap-1.5 shadow-xs transition-all cursor-pointer hover:translate-y-0.5"
                        >
                          <span>Lanjut ke {progression.nextLabId}</span>
                          <ChevronRight size={15} />
                        </button>
                      )}
                      {status?.running ? (
                        <button
                          onClick={() => openBrowser(activeTargetUrl)}
                          className="px-4 py-2.5 text-xs font-bold font-['Space_Grotesk'] rounded-lg bg-[#0F766E] hover:bg-[#115E59] text-white flex items-center gap-1.5 shadow-xs transition-all cursor-pointer hover:translate-y-0.5"
                        >
                          <span>Buka Target (: {targetPort})</span>
                          <ExternalLink size={13} />
                        </button>
                      ) : (
                        <button
                          onClick={handleToggleTarget}
                          disabled={targetOp.isOpen}
                          className="px-4 py-2.5 text-xs font-bold font-['Space_Grotesk'] rounded-lg bg-[#0F766E] hover:bg-[#115E59] active:bg-[#094D47] text-white flex items-center gap-1.5 shadow-xs transition-all cursor-pointer hover:translate-y-0.5 disabled:opacity-50 disabled:cursor-not-allowed"
                        >
                          {targetOp.isOpen && targetOp.type === 'launch' ? (
                            <Loader2 size={13} className="animate-spin text-[#5EEAD4]" />
                          ) : (
                            <Play size={13} />
                          )}
                          <span>{targetOp.isOpen && targetOp.type === 'launch' ? 'Memulai...' : `Jalankan Target (: ${targetPort})`}</span>
                        </button>
                      )}
                    </div>
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

      {/* Requirement: Target Operation Anti-Spam Loading Popup Modal */}
      {targetOp.isOpen && (
        <div className="fixed inset-0 z-50 flex items-center justify-center p-4 bg-[#1E1A14]/75 backdrop-blur-sm select-none animate-in fade-in duration-200">
          <div className="relative w-full max-w-md bg-[#FCFBF8] border-2 border-[#094D47] rounded-2xl shadow-[0_8px_0_0_#063934,0_24px_48px_rgba(0,0,0,0.4)] overflow-hidden">
            {/* Top Neo-Retro Accent Bar */}
            <div className="px-5 py-3 bg-gradient-to-r from-[#0F766E] to-[#115E59] text-white flex items-center justify-between border-b-2 border-[#094D47]">
              <div className="flex items-center gap-2 text-xs font-bold font-['JetBrains_Mono'] tracking-wider">
                <span className="w-2 h-2 rounded-full bg-[#5EEAD4] animate-ping" />
                <span>
                  {targetOp.type === 'launch'
                    ? 'TARGET RUNTIME // MEMULAI TARGET'
                    : 'TARGET RUNTIME // MENGHENTIKAN TARGET'}
                </span>
              </div>
              <span className="text-[10px] font-['JetBrains_Mono'] text-teal-200 bg-[#042F2E]/60 px-2 py-0.5 rounded border border-[#2DD4BF]/40">
                :{targetPort}
              </span>
            </div>

            {/* Modal Content */}
            <div className="p-6 space-y-5 text-center">
              {targetOp.status === 'progress' && (
                <>
                  {/* Surfing Cute Anime Loading Canvas */}
                  <CuteAnimeLoading
                    message={
                      targetOp.type === 'launch'
                        ? `MENYIAPKAN TARGET: ${labId}`
                        : `MENGHENTIKAN SANDBOX: ${labId}`
                    }
                    subMessage={
                      targetOp.type === 'launch'
                        ? 'BINDING LOCALHOST & ISOLATING ENVIRONMENT'
                        : 'RELEASING CHILD PROCESSES & PORTS'
                    }
                  />

                  {/* Operation Info Card */}
                  <div className="p-3.5 rounded-xl border border-[#2DD4BF]/40 bg-[#CCFBF1]/40 text-left space-y-2">
                    <div className="flex items-center justify-between text-xs font-['Space_Grotesk'] font-bold text-[#0F766E]">
                      <span>{content.manifest.title}</span>
                      <span className="font-['JetBrains_Mono'] text-[11px] text-[#0D9488]">
                        {briefing.targetAppName}
                      </span>
                    </div>

                    {/* Progressive Step Checklist */}
                    <div className="space-y-1.5 pt-1 text-[11px] font-['Space_Grotesk']">
                      <div className="flex items-center gap-2 text-[#0F766E]">
                        {targetOp.step >= 1 ? (
                          <CheckCircle2 size={13} className="text-[#10B981] shrink-0" />
                        ) : (
                          <Loader2 size={13} className="animate-spin text-[#0F766E] shrink-0" />
                        )}
                        <span>Mengalokasikan AppContainer & security descriptor</span>
                      </div>
                      <div className="flex items-center gap-2">
                        {targetOp.step >= 2 ? (
                          <CheckCircle2 size={13} className="text-[#10B981] shrink-0" />
                        ) : targetOp.step === 1 ? (
                          <Loader2 size={13} className="animate-spin text-[#0F766E] shrink-0" />
                        ) : (
                          <span className="w-3 h-3 rounded-full border border-[#CBD5E1] shrink-0 inline-block" />
                        )}
                        <span className={targetOp.step >= 2 ? 'text-[#0F766E]' : 'text-[#78716C]'}>
                          Memverifikasi ketersediaan port localhost:{targetPort}
                        </span>
                      </div>
                      <div className="flex items-center gap-2">
                        {targetOp.step >= 3 ? (
                          <CheckCircle2 size={13} className="text-[#10B981] shrink-0" />
                        ) : targetOp.step === 2 ? (
                          <Loader2 size={13} className="animate-spin text-[#0F766E] shrink-0" />
                        ) : (
                          <span className="w-3 h-3 rounded-full border border-[#CBD5E1] shrink-0 inline-block" />
                        )}
                        <span className={targetOp.step >= 3 ? 'text-[#0F766E]' : 'text-[#78716C]'}>
                          Menjalankan web service target aplikasi
                        </span>
                      </div>
                    </div>
                  </div>

                  {/* Anti-spam Reassurance Notice */}
                  <div className="px-3 py-2 rounded-lg bg-[#F5F5F4] border border-[#E7E5E4] text-[11px] text-[#78716C] leading-relaxed flex items-center gap-2 text-left">
                    <Shield size={14} className="shrink-0 text-[#0F766E]" />
                    <span>
                      Sistem sedang menyiapkan sandbox di latar belakang. Tombol dinonaktifkan untuk mencegah spam click!
                    </span>
                  </div>
                </>
              )}

              {targetOp.status === 'success' && (
                <div className="py-4 space-y-3 animate-in zoom-in-95 duration-200">
                  <div className="w-14 h-14 mx-auto rounded-full bg-[#D1FAE5] border-2 border-[#10B981] text-[#059669] flex items-center justify-center shadow-md">
                    <CheckCircle2 size={32} />
                  </div>
                  <div>
                    <h3 className="text-base font-bold font-['Space_Grotesk'] text-[#1E1A14]">
                      {targetOp.type === 'launch'
                        ? 'TARGET BERHASIL DIAKTIFKAN!'
                        : 'TARGET BERHASIL DIHENTIKAN!'}
                    </h3>
                    <p className="text-xs text-[#5C5347] mt-1 font-['JetBrains_Mono']">
                      {targetOp.type === 'launch'
                        ? `${activeTargetUrl} siap diakses`
                        : 'Semua proses sandbox target telah dibersihkan'}
                    </p>
                  </div>
                </div>
              )}

              {targetOp.status === 'error' && (
                <div className="py-2 space-y-4 animate-in zoom-in-95 duration-200">
                  <div className="w-14 h-14 mx-auto rounded-full bg-[#FEE2E2] border-2 border-[#EF4444] text-[#DC2626] flex items-center justify-center shadow-md">
                    <AlertCircle size={32} />
                  </div>
                  <div className="space-y-1">
                    <h3 className="text-base font-bold font-['Space_Grotesk'] text-[#1E1A14]">
                      {targetOp.type === 'launch'
                        ? 'GAGAL MELUNCURKAN TARGET'
                        : 'GAGAL MENGHENTIKAN TARGET'}
                    </h3>
                    <p className="text-xs text-[#DC2626] font-['JetBrains_Mono'] bg-[#FEF2F2] p-2.5 rounded-lg border border-[#FCA5A5] text-left">
                      {targetOp.errorMessage || 'Terjadi kendala saat menginisialisasi sandbox target.'}
                    </p>
                  </div>
                  <button
                    onClick={() => setTargetOp((prev) => ({ ...prev, isOpen: false }))}
                    className="px-4 py-2 bg-[#1E1A14] hover:bg-[#3D372E] text-white text-xs font-bold font-['Space_Grotesk'] rounded-lg transition-colors cursor-pointer"
                  >
                    Tutup Dialog
                  </button>
                </div>
              )}
            </div>
          </div>
        </div>
      )}

      {/* Video Bootcamp YouTube Modal */}
      <VideoDrawerModal
        isOpen={videoModalOpen}
        onClose={() => setVideoModalOpen(false)}
        labId={labId}
        labTitle={content.manifest.title}
      />

      {/* Floating AI Mentor Contact Bubble (Landing Page Style FAB) */}
      <div className="fixed bottom-6 right-6 z-40 flex items-center group">
        <button
          onClick={() => setZetaMentorOpen(true)}
          className="flex items-center gap-2.5 px-3.5 py-2.5 rounded-full bg-[#12151D] hover:bg-[#1A202C] text-white border-2 border-[#2DD4BF] shadow-[0_6px_20px_rgba(15,118,110,0.4),0_3px_0_0_#0F766E] hover:shadow-[0_8px_25px_rgba(45,212,191,0.5),0_4px_0_0_#0F766E] hover:-translate-y-1 active:translate-y-0.5 active:shadow-[0_2px_8px_rgba(15,118,110,0.3)] transition-all cursor-pointer"
          title="Tanya Petunjuk AI (Zeta Mentor)"
        >
          {/* Cute Robot Mascot Icon */}
          <div className="relative w-8 h-8 rounded-full overflow-hidden border-2 border-[#2DD4BF] bg-[#090D16] shrink-0 shadow-xs flex items-center justify-center">
            <img
              src="/assets/robot_mentor_icon.png"
              alt="Robot Mentor"
              className="w-full h-full object-cover object-center group-hover:scale-110 transition-transform"
              onError={(e) => {
                ;(e.target as HTMLElement).style.display = 'none'
              }}
            />
          </div>

          {/* Label & Pulsing Online Dot */}
          <div className="flex flex-col text-left pr-1">
            <span className="text-xs font-bold font-['Space_Grotesk'] text-[#5EEAD4] tracking-wide flex items-center gap-1.5">
              <span>Petunjuk AI</span>
              <span className="w-2 h-2 rounded-full bg-[#10B981] animate-pulse" />
            </span>
            <span className="text-[9px] font-['JetBrains_Mono'] text-[#94A3B8] -mt-0.5">
              Tanya Mentor
            </span>
          </div>
        </button>
      </div>

      {/* Zeta Socratic Cyber Mentor Modal */}
      {zetaMentorOpen && (
        <ZetaMentorModal
          isOpen={zetaMentorOpen}
          onClose={() => setZetaMentorOpen(false)}
          currentLabId={labId}
          userTier={currentUser?.tier}
        />
      )}
    </div>
  )
}
