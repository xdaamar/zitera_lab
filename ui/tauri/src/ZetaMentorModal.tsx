// ZITERA CYBER_LAB — Multi-Mentor Character Intelligence Modal
// Features multiple realistic anime cyber mentors (Zeta, Aoi, Elysia), dynamic teaching & praise poses,
// progressive Socratic clue ladder, and finite tactical question tree (Option A) with realistic typewriter cadence.
// Zero emojis — Lucide icons and masterclass vector/anime assets used exclusively.

import { useState, useEffect, useRef } from 'react'
import {
  ShieldCheck,
  Terminal,
  Lock,
  Unlock,
  Sparkles,
  AlertTriangle,
  ChevronDown,
  ChevronUp,
  HelpCircle,
  X,
  Cpu,
  ArrowRight,
  KeyRound,
  UserCheck,
  RotateCcw,
  FastForward,
  CheckCircle2,
  Maximize2,
  Minimize2,
} from 'lucide-react'
import {
  getSocraticCurriculum,
  querySocraticEngine,
  type ZetaMood,
  type SocraticClue,
} from './zetaSocraticData'
import { LAB_CATALOG_TITLES } from './progressionHelper'
import {
  getMentor,
  MENTOR_LIST,
  type MentorId,
  type MentorProfile,
} from './mentorRegistry'
import {
  generateAiOverview,
  type ZiteraAiOverviewData,
} from './fuzzySemanticEngine'
import { ZiteraAiOverview } from './ZiteraAiOverview'
import { UpgradeModal } from './UpgradeModal'

export interface ZetaMentorModalProps {
  isOpen: boolean
  onClose: () => void
  currentLabId: string
  userTier?: string | null
  onOpenBurpLite?: () => void
  zIndex?: number
  onBringToFront?: () => void
}

interface ChatMessage {
  id: string
  sender: 'student' | 'zeta'
  text: string
  timestamp: string
  mood?: ZetaMood
  codeSnippet?: string
  antiSpoilerWarning?: boolean
  mentorId?: MentorId
}

export function ZetaMentorModal({
  isOpen,
  onClose,
  currentLabId,
  userTier,
  onOpenBurpLite,
  zIndex = 10000,
  onBringToFront,
}: ZetaMentorModalProps) {
  // Floating Draggable Window Position State
  const [position, setPosition] = useState<{ x: number; y: number }>(() => {
    if (typeof window !== 'undefined') {
      const defaultX = Math.max(16, window.innerWidth - 600)
      const defaultY = Math.max(16, 40)
      return { x: defaultX, y: defaultY }
    }
    return { x: 700, y: 40 }
  })
  const [isMaximized, setIsMaximized] = useState(false)

  // Drag handler for window title bar
  const handleDragMouseDown = (e: React.MouseEvent) => {
    if (e.button !== 0 || isMaximized) return
    if ((e.target as HTMLElement).closest('button, input, a, select, textarea')) return

    onBringToFront?.()

    const startX = e.clientX
    const startY = e.clientY
    const initialX = position.x
    const initialY = position.y

    const handleMouseMove = (moveEvent: MouseEvent) => {
      const deltaX = moveEvent.clientX - startX
      const deltaY = moveEvent.clientY - startY
      const maxX = Math.max(10, window.innerWidth - 180)
      const maxY = Math.max(10, window.innerHeight - 80)
      setPosition({
        x: Math.min(maxX, Math.max(10, initialX + deltaX)),
        y: Math.min(maxY, Math.max(10, initialY + deltaY)),
      })
    }

    const handleMouseUp = () => {
      window.removeEventListener('mousemove', handleMouseMove)
      window.removeEventListener('mouseup', handleMouseUp)
    }

    window.addEventListener('mousemove', handleMouseMove)
    window.addEventListener('mouseup', handleMouseUp)
  }
  const cleanLabId = (currentLabId || 'F01').trim().toUpperCase()
  const labTitle = LAB_CATALOG_TITLES[cleanLabId] || `Laboratorium ${cleanLabId}`
  const curriculum = getSocraticCurriculum(cleanLabId, labTitle)

  // ── Multi-Mentor Persona State ─────────────────────────────────────────────
  const [selectedMentorId, setSelectedMentorId] = useState<MentorId>('chiyo')
  const currentMentor: MentorProfile = getMentor(selectedMentorId)

  // ── State Management ───────────────────────────────────────────────────────
  const [unlockedClues, setUnlockedClues] = useState<number[]>([1]) // Level 1 unlocked by default
  const [expandedClueLevel, setExpandedClueLevel] = useState<number | null>(1)
  const [activeMood, setActiveMood] = useState<ZetaMood>('idle')
  const [isTyping, setIsTyping] = useState<boolean>(false)
  const [messages, setMessages] = useState<ChatMessage[]>([])

  // ── Tactical Question Tree & Typewriter State (Option A) ────────────────────
  const [investigationDepth, setInvestigationDepth] = useState<number>(0)
  const [activeQuestions, setActiveQuestions] = useState<string[]>([])
  const [isTerminal, setIsTerminal] = useState<boolean>(false)
  const [typingState, setTypingState] = useState<{
    text: string
    isStreaming: boolean
    targetFullText: string
    activeMessageId: string | null
    mood: ZetaMood
    mentorId: MentorId
  }>({
    text: '',
    isStreaming: false,
    targetFullText: '',
    activeMessageId: null,
    mood: 'idle',
    mentorId: selectedMentorId,
  })

  const typewriterTimerRef = useRef<ReturnType<typeof setTimeout> | null>(null)

  // ── AI Overview & Paywall State ────────────────────────────────────────────
  const [activeConsoleTab, setActiveConsoleTab] = useState<'overview' | 'chat'>('overview')
  const [aiOverviewData, setAiOverviewData] = useState<ZiteraAiOverviewData>(() =>
    generateAiOverview('', cleanLabId, (userTier as 'free' | 'pro' | 'vip') || 'free')
  )
  const [upgradeModalOpen, setUpgradeModalOpen] = useState<boolean>(false)
  const [upgradeTargetFeature, setUpgradeTargetFeature] = useState<string>(
    'AI Payload Blueprint & Burp Auto-Inject'
  )
  const [injectToast, setInjectToast] = useState<string | null>(null)
  const [mentorEnergy, setMentorEnergy] = useState<number>(5)

  const chatScrollRef = useRef<HTMLDivElement>(null)

  // Auto-inject handler connecting to Burp Lite Repeater
  const handleAutoInject = (payload: string, targetParam?: string) => {
    window.dispatchEvent(
      new CustomEvent('zitera:inject-repeater', {
        detail: {
          payload,
          targetParam,
          path: targetParam
            ? `/?${targetParam}=${encodeURIComponent(payload)}`
            : `/?q=${encodeURIComponent(payload)}`,
        },
      })
    )
    if (onOpenBurpLite) {
      onOpenBurpLite()
    }
    setInjectToast('Payload berhasil diinjeksikan ke Burp Lite Repeater!')
    setTimeout(() => setInjectToast(null), 3500)
  }

  // Sync AI Overview when lab changes
  useEffect(() => {
    setAiOverviewData(
      generateAiOverview('', cleanLabId, (userTier as 'free' | 'pro' | 'vip') || 'free')
    )
  }, [cleanLabId, userTier])

  // ── Mentor Switching Handler ───────────────────────────────────────────────
  const handleSwitchMentor = (mentorId: MentorId) => {
    setSelectedMentorId(mentorId)
    const mentor = getMentor(mentorId)
    const newGreeting = mentor.greeting(cleanLabId, labTitle)
    setActiveMood('idle')
    setMessages((prev) => [
      ...prev,
      {
        id: `msg_switch_${Date.now()}`,
        sender: 'zeta',
        text: newGreeting,
        timestamp: new Date().toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' }),
        mood: 'idle',
        mentorId: mentorId,
      },
    ])
  }

  // ── Initial Setup on Open / Lab Change ─────────────────────────────────────
  useEffect(() => {
    if (!isOpen) return

    const initialMentor = getMentor(selectedMentorId)
    const greeting = initialMentor.greeting(cleanLabId, labTitle)

    setActiveMood('idle')
    setMessages([
      {
        id: 'msg_init',
        sender: 'zeta',
        text: greeting,
        timestamp: new Date().toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' }),
        mood: 'idle',
        mentorId: selectedMentorId,
      },
    ])
    setUnlockedClues([1])
    setExpandedClueLevel(1)

    // Reset Tactical Question Tree to Root Questions
    setInvestigationDepth(0)
    setIsTerminal(false)
    const rootQuestions =
      curriculum.suggestedPrompts && curriculum.suggestedPrompts.length > 0
        ? curriculum.suggestedPrompts
        : [
            'Bagaimana cara memetakan parameter sensitif pada endpoint ini?',
            'Indikasi apa pada response HTTP yang membuktikan adanya celah?',
            'Langkah observasi awal apa yang harus saya lakukan?',
          ]
    setActiveQuestions(rootQuestions)
  }, [isOpen, cleanLabId, labTitle])

  // Auto-scroll chat console
  useEffect(() => {
    if (chatScrollRef.current) {
      chatScrollRef.current.scrollTop = chatScrollRef.current.scrollHeight
    }
  }, [messages, typingState.text])

  // Tier Gating Helpers
  const isFoundationEligibleLab = [
    'F01',
    'F02',
    'F03',
    'F04',
    'F05',
    'F06',
    'A01',
    'A02',
  ].includes(cleanLabId)

  const canAccessLevel2 =
    userTier === 'pro' ||
    userTier === 'vip' ||
    userTier === 'enterprise' ||
    (userTier === 'foundation' && isFoundationEligibleLab)

  const canAccessLevel3 = userTier === 'vip' || userTier === 'enterprise'

  // ── Unlock Clue Progression ────────────────────────────────────────────────
  const handleUnlockClue = (level: number) => {
    if (level === 2 && !canAccessLevel2) {
      setActiveMood('warning')
      setUpgradeTargetFeature('Clue Level 2 (Vektor Serangan & Logika Bypass)')
      setUpgradeModalOpen(true)
      return
    }

    if (level === 3 && !canAccessLevel3) {
      setActiveMood('warning')
      setUpgradeTargetFeature('Clue Level 3 (White-Box Code Anatomy & DevSecOps Patch)')
      setUpgradeModalOpen(true)
      return
    }

    if (!unlockedClues.includes(level)) {
      setUnlockedClues((prev) => [...prev, level])
    }
    setExpandedClueLevel(level)

    const targetClue = curriculum.clues.find((c) => c.level === level)
    if (targetClue) {
      const unlockSpeech = `[${currentMentor.name.toUpperCase()} // CLUE LEVEL ${level} TERBUKA]: ${targetClue.summary} Pelajari pertanyaan investigasi di bawah untuk mengarahkan pengujian Anda.`
      setActiveMood('eureka')
      setMessages((prev) => [
        ...prev,
        {
          id: `clue_${level}_${Date.now()}`,
          sender: 'zeta',
          text: unlockSpeech,
          timestamp: new Date().toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' }),
          mood: 'eureka',
          mentorId: selectedMentorId,
        },
      ])
    }
  }

  // ── Skip Typewriter Animation ──────────────────────────────────────────────
  const handleSkipTyping = () => {
    if (!typingState.isStreaming) return
    if (typewriterTimerRef.current) {
      clearTimeout(typewriterTimerRef.current)
      typewriterTimerRef.current = null
    }

    const completedText = typingState.targetFullText
    const completedMsgId = typingState.activeMessageId
    const finalMood = typingState.mood

    setTypingState((prev) => ({
      ...prev,
      text: completedText,
      isStreaming: false,
    }))
    setIsTyping(false)
    setActiveMood(finalMood)

    if (completedMsgId) {
      setMessages((prev) =>
        prev.map((m) =>
          m.id === completedMsgId ? { ...m, text: completedText, mood: finalMood } : m
        )
      )
    }
  }

  // ── Realistic Typewriter Engine ────────────────────────────────────────────
  const streamTypewriter = (
    fullText: string,
    messageId: string,
    mood: ZetaMood,
    mentorId: MentorId,
    onFinish: () => void
  ) => {
    let currentIndex = 0
    setIsTyping(true)
    setActiveMood('thinking')

    setTypingState({
      text: '',
      isStreaming: true,
      targetFullText: fullText,
      activeMessageId: messageId,
      mood,
      mentorId,
    })

    const typeNextChar = () => {
      if (currentIndex >= fullText.length) {
        setTypingState((prev) => ({ ...prev, isStreaming: false, text: fullText }))
        setIsTyping(false)
        setActiveMood(mood)
        setMessages((prev) =>
          prev.map((m) =>
            m.id === messageId ? { ...m, text: fullText, mood } : m
          )
        )
        onFinish()
        return
      }

      const char = fullText[currentIndex]
      currentIndex++
      const partial = fullText.slice(0, currentIndex)

      setTypingState((prev) => ({ ...prev, text: partial }))
      setMessages((prev) =>
        prev.map((m) =>
          m.id === messageId ? { ...m, text: partial } : m
        )
      )

      // Variable cadence simulation (18-32ms base, 200ms punctuation pause)
      let delay = 18 + Math.floor(Math.random() * 14)
      if (char === '.' || char === '?' || char === '!') {
        delay = 200
      } else if (char === ',' || char === ':' || char === ';') {
        delay = 90
      } else if (char === '\n') {
        delay = 140
      }

      typewriterTimerRef.current = setTimeout(typeNextChar, delay)
    }

    typewriterTimerRef.current = setTimeout(typeNextChar, 80)
  }

  // ── Handle Tactical Question Selection (Option A Tree) ──────────────────────
  const handleSelectTacticalQuestion = (question: string) => {
    if (isTyping || typingState.isStreaming) return

    // Deduct energy
    if (mentorEnergy > 0) {
      setMentorEnergy((prev) => Math.max(0, prev - 1))
    }

    const studentTime = new Date().toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' })
    const studentMsg: ChatMessage = {
      id: `std_${Date.now()}`,
      sender: 'student',
      text: question,
      timestamp: studentTime,
    }

    // Evaluate heuristic Socratic engine response
    const response = querySocraticEngine(question, cleanLabId)
    const mentorTime = new Date().toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' })

    let personalizedText = response.message
    if ((selectedMentorId === 'kaede' || selectedMentorId === 'aoi') && !response.antiSpoilerWarning) {
      personalizedText = `[Kaede Exploit Note]: ${response.message}`
    } else if ((selectedMentorId === 'valerie' || selectedMentorId === 'elysia') && !response.antiSpoilerWarning) {
      personalizedText = `[Valerie Defense Review]: ${response.message}`
    }

    const mentorMsgId = `zeta_${Date.now()}`
    const placeholderMentorMsg: ChatMessage = {
      id: mentorMsgId,
      sender: 'zeta',
      text: '',
      timestamp: mentorTime,
      mood: 'thinking',
      antiSpoilerWarning: response.antiSpoilerWarning,
      mentorId: selectedMentorId,
    }

    setMessages((prev) => [...prev, studentMsg, placeholderMentorMsg])

    // Update AI Overview context
    const newOverview = generateAiOverview(
      question,
      cleanLabId,
      (userTier as 'free' | 'pro' | 'vip') || 'free'
    )
    setAiOverviewData(newOverview)

    // Stream typewriter response
    streamTypewriter(
      personalizedText,
      mentorMsgId,
      response.mood,
      selectedMentorId,
      () => {
        const nextDepth = investigationDepth + 1
        setInvestigationDepth(nextDepth)

        const followups = response.suggestedFollowups.filter((f) => f && f !== question)
        if (nextDepth >= 2 || followups.length === 0) {
          setIsTerminal(true)
          setActiveQuestions([])
        } else {
          setIsTerminal(false)
          setActiveQuestions(followups)
        }
      }
    )
  }

  // ── Reset Investigation Tree ───────────────────────────────────────────────
  const handleResetInvestigation = () => {
    if (isTyping || typingState.isStreaming) {
      handleSkipTyping()
    }
    setInvestigationDepth(0)
    setIsTerminal(false)

    const rootQuestions =
      curriculum.suggestedPrompts && curriculum.suggestedPrompts.length > 0
        ? curriculum.suggestedPrompts
        : [
            'Bagaimana cara memetakan parameter sensitif pada endpoint ini?',
            'Indikasi apa pada response HTTP yang membuktikan adanya celah?',
            'Langkah observasi awal apa yang harus saya lakukan?',
          ]
    setActiveQuestions(rootQuestions)

    const resetMsg: ChatMessage = {
      id: `reset_${Date.now()}`,
      sender: 'zeta',
      text: `[${currentMentor.name.toUpperCase()} // INVESTIGASI DIRESET]: Rantai penyelidikan telah dikembalikan ke awal. Pilih sudut investigasi baru di bawah untuk melanjutkan eksplorasi ${labTitle}.`,
      timestamp: new Date().toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' }),
      mood: 'idle',
      mentorId: selectedMentorId,
    }
    setMessages((prev) => [...prev, resetMsg])
    setActiveMood('idle')
  }

  // ── Dynamic Visual Pose Resolution ─────────────────────────────────────────
  const activePoseImage =
    activeMood === 'eureka'
      ? currentMentor.poses.eureka
      : currentMentor.poses.teaching

  // ── Mood Visual Configurations ─────────────────────────────────────────────
  const moodConfig = {
    idle: {
      label: 'SIAP MEMBIMBING',
      badgeBg: currentMentor.badgeBg,
      badgeText: currentMentor.badgeText,
      avatarRing: `${currentMentor.accentBorder} ${currentMentor.accentGlow}`,
      bubbleBorder: `${currentMentor.accentBorder}`,
      glowColor: 'from-stone-900 to-stone-950',
      icon: Terminal,
      poseLabel: 'MODE PENGAJARAN // LECTURE',
    },
    thinking: {
      label: 'MENGANALISIS HIPOTESIS',
      badgeBg: 'bg-amber-500/10 border-amber-500/30',
      badgeText: 'text-amber-300',
      avatarRing: 'ring-amber-400/60 shadow-[0_0_24px_rgba(245,158,11,0.35)]',
      bubbleBorder: 'border-amber-400/40 shadow-[0_0_20px_rgba(245,158,11,0.15)]',
      glowColor: 'from-amber-500/10 to-stone-950',
      icon: Cpu,
      poseLabel: 'ANALISIS INTEL // THOUGHTFUL',
    },
    eureka: {
      label: 'INSIGHT DITEMUKAN',
      badgeBg: 'bg-emerald-500/10 border-emerald-500/30',
      badgeText: 'text-emerald-300',
      avatarRing: 'ring-emerald-400/70 shadow-[0_0_30px_rgba(16,185,129,0.45)]',
      bubbleBorder: 'border-emerald-400/40 shadow-[0_0_24px_rgba(16,185,129,0.2)]',
      glowColor: 'from-emerald-500/15 to-stone-950',
      icon: Sparkles,
      poseLabel: 'BREAKTHROUGH // PRAISE POSE',
    },
    warning: {
      label: 'ANTI-SPOILER GUARD',
      badgeBg: 'bg-rose-500/10 border-rose-500/30',
      badgeText: 'text-rose-400',
      avatarRing: 'ring-rose-500/60 shadow-[0_0_28px_rgba(244,63,94,0.4)]',
      bubbleBorder: 'border-rose-500/30 shadow-[0_0_20px_rgba(244,63,94,0.15)]',
      glowColor: 'from-rose-500/15 to-stone-950',
      icon: AlertTriangle,
      poseLabel: 'GUARDRAIL // WARNING',
    },
  }

  const currentMoodUi = moodConfig[activeMood]
  const MoodIcon = currentMoodUi.icon

  if (!isOpen) return null

  return (
    <div
      style={
        isMaximized
          ? { left: '1vw', top: '1vh', width: '98vw', height: '98vh', zIndex }
          : {
              left: `${position.x}px`,
              top: `${position.y}px`,
              width: 'min(640px, 94vw)',
              height: 'min(780px, 88vh)',
              zIndex,
            }
      }
      className="fixed flex flex-col rounded-2xl bg-stone-900/95 border border-teal-500/40 shadow-[0_20px_60px_rgba(0,0,0,0.85),0_0_30px_rgba(45,212,191,0.2)] overflow-hidden text-stone-200 select-none animate-in fade-in zoom-in-95 duration-150"
      onClick={() => onBringToFront?.()}
    >
      {/* Subtle Cyber Grid Background Texture */}
      <div className="absolute inset-0 pointer-events-none opacity-[0.03] bg-[linear-gradient(to_right,#14b8a6_1px,transparent_1px),linear-gradient(to_bottom,#14b8a6_1px,transparent_1px)] bg-[size:24px_24px]" />

      {/* ── TOP HEADER BAR (Draggable Handle) ─────────────────────────────────── */}
      <header
        onMouseDown={handleDragMouseDown}
        className="relative z-10 flex flex-wrap items-center justify-between px-4 sm:px-5 py-3 border-b border-stone-800 bg-stone-950/85 backdrop-blur-sm gap-3 cursor-grab active:cursor-grabbing select-none"
      >
          {/* Brand & Companion Identity */}
          <div className="flex items-center gap-3">
            <div className="relative flex items-center justify-center w-9 h-9 rounded-lg bg-stone-900 border border-teal-500/40 shadow-[0_0_12px_rgba(20,184,166,0.2)] shrink-0 overflow-hidden">
              <img
                src={currentMentor.avatar}
                alt={currentMentor.name}
                className="w-full h-full object-cover"
                onError={(e) => {
                  ;(e.target as HTMLImageElement).src = '/assets/mentors/zeta_avatar.png'
                }}
              />
              <span className="absolute -bottom-0.5 -right-0.5 w-2.5 h-2.5 rounded-full bg-emerald-400 border-2 border-stone-950 animate-pulse" />
            </div>

            <div>
              <div className="flex items-center gap-2 flex-wrap">
                <span className="font-['Space_Grotesk'] font-bold text-base tracking-wide bg-gradient-to-r from-teal-200 via-cyan-200 to-white bg-clip-text text-transparent">
                  {currentMentor.name.toUpperCase()} // AI CYBER MENTOR
                </span>
                <span className="hidden sm:inline-flex items-center gap-1 px-2 py-0.5 rounded text-[10px] font-mono font-bold tracking-wider uppercase bg-teal-500/10 text-teal-300 border border-teal-500/20">
                  <ShieldCheck className="w-3 h-3 text-teal-400" />
                  {currentMentor.codename}
                </span>
              </div>
              <p className="text-[11px] font-mono text-stone-400 truncate max-w-[260px] sm:max-w-md">
                Target: <span className="text-teal-300 font-semibold">{cleanLabId}</span> —{' '}
                {labTitle}
              </p>
            </div>
          </div>

          {/* Center/Right: Mentor Switcher Pills */}
          <div className="flex items-center gap-2">
            <div className="flex items-center gap-1 bg-stone-950/90 p-1 rounded-xl border border-stone-800">
              <span className="text-[10px] font-mono text-stone-500 px-2 hidden md:inline">
                PILIH MENTOR:
              </span>
              {MENTOR_LIST.map((m) => {
                const isSelected = selectedMentorId === m.id
                return (
                  <button
                    key={m.id}
                    onClick={() => handleSwitchMentor(m.id)}
                    className={`flex items-center gap-1.5 px-2 py-1 rounded-lg text-xs font-['Space_Grotesk'] font-bold transition-all cursor-pointer ${
                      isSelected
                        ? `${m.badgeBg} ${m.badgeText} border shadow-xs`
                        : 'text-stone-400 hover:text-stone-200 hover:bg-stone-800/60 border border-transparent'
                    }`}
                    title={`Ganti mentor ke ${m.name} (${m.role})`}
                  >
                    <img
                      src={m.avatar}
                      alt={m.name}
                      className="w-4 h-4 rounded-full object-cover border border-stone-600 shrink-0"
                      onError={(e) => {
                        ;(e.target as HTMLImageElement).style.display = 'none'
                      }}
                    />
                    <span>{m.name}</span>
                  </button>
                )
              })}
            </div>

            {/* Tier Badge */}
            <span
              className={`hidden lg:inline-block px-2.5 py-1 rounded-full text-[10px] font-mono font-bold tracking-widest uppercase border ${
                userTier === 'pro' || userTier === 'enterprise'
                  ? 'bg-amber-500/10 text-amber-300 border-amber-500/30'
                  : 'bg-stone-800 text-stone-300 border-stone-700'
              }`}
            >
              {(userTier || 'FOUNDATION').toUpperCase()}
            </span>

            {/* Maximize / Restore Button */}
            <button
              onClick={() => setIsMaximized(!isMaximized)}
              className="p-1.5 rounded-lg text-stone-400 hover:text-white hover:bg-stone-800 border border-transparent hover:border-stone-700 transition-colors cursor-pointer"
              title={isMaximized ? 'Kembalikan Ukuran' : 'Perbesar Jendela'}
            >
              {isMaximized ? <Minimize2 className="w-4 h-4" /> : <Maximize2 className="w-4 h-4" />}
            </button>

            {/* Close Modal Button */}
            <button
              onClick={onClose}
              className="p-1.5 rounded-lg text-stone-400 hover:text-white hover:bg-stone-800 border border-transparent hover:border-stone-700 transition-colors cursor-pointer"
              title="Tutup Jendela (Esc)"
              aria-label="Tutup Jendela"
            >
              <X className="w-5 h-5" />
            </button>
          </div>
        </header>

        {/* ── MAIN TWO-COLUMN WORKSPACE ─────────────────────────────────────── */}
        <div className="relative z-10 flex-1 grid grid-cols-1 lg:grid-cols-12 overflow-hidden">
          {/* ── LEFT COLUMN: TEACHING STAGE & CLUE LADDER (5 Cols) ───────────── */}
          <div className="lg:col-span-5 flex flex-col border-b lg:border-b-0 lg:border-r border-stone-800 bg-stone-950/60 overflow-y-auto custom-scrollbar p-4 md:p-5 gap-4">
            {/* Visual Anime Mentor Interactive Stage */}
            <div
              className={`relative rounded-2xl border p-4 bg-gradient-to-b ${currentMoodUi.glowColor} ${currentMoodUi.bubbleBorder} transition-all duration-300 space-y-3.5`}
            >
              {/* Tactical Status Bar */}
              <div className="flex items-center justify-between text-xs font-mono">
                <span className="flex items-center gap-1.5 text-[11px] font-semibold text-stone-300">
                  <Terminal className="w-3.5 h-3.5 text-teal-400" />
                  {currentMentor.role}
                </span>
                <span
                  className={`inline-flex items-center gap-1 px-2 py-0.5 rounded text-[10px] font-bold border ${currentMoodUi.badgeBg} ${currentMoodUi.badgeText}`}
                >
                  <MoodIcon className="w-3 h-3" />
                  {currentMoodUi.label}
                </span>
              </div>

              {/* Realistic Anime Mentor Teaching Figure Stage */}
              <div className="relative w-full h-72 sm:h-80 md:h-[350px] rounded-2xl overflow-hidden border border-stone-800 bg-stone-950 flex items-center justify-center group shadow-xl">
                <img
                  src={activePoseImage}
                  alt={`${currentMentor.name} Teaching Pose`}
                  className="w-full h-full object-cover object-top transition-all duration-300 group-hover:scale-105 select-none"
                  onError={(e) => {
                    ;(e.target as HTMLImageElement).src = currentMentor.avatar
                  }}
                />

                {/* Subtle vignette shadow */}
                <div className="absolute inset-0 bg-gradient-to-t from-stone-950/90 via-transparent to-stone-950/30 pointer-events-none" />

                {/* Live Active Mentor Tag Overlay */}
                <div className="absolute top-3 left-3 px-2.5 py-1 rounded-full bg-stone-950/85 border border-stone-700/80 text-[10px] font-mono font-bold text-stone-200 flex items-center gap-1.5 backdrop-blur-md shadow-md">
                  <span
                    className="w-2 h-2 rounded-full animate-ping"
                    style={{ backgroundColor: currentMentor.themeColor }}
                  />
                  <span>{currentMentor.name.toUpperCase()} // ACTIVE</span>
                </div>

                {/* Active Pose Mode Tag */}
                <div className="absolute bottom-3 right-3 px-2.5 py-1 rounded-lg bg-stone-950/85 border border-stone-700/80 text-[10px] font-mono text-stone-300 backdrop-blur-md flex items-center gap-1.5 shadow-md">
                  <UserCheck className="w-3 h-3 text-teal-400" />
                  <span>{currentMoodUi.poseLabel}</span>
                </div>
              </div>
            </div>

            {/* ── PROGRESSIVE HINT LADDER (ACCORDION) ────────────────────────── */}
            <div className="flex flex-col gap-2.5">
              <div className="flex items-center justify-between">
                <span className="flex items-center gap-1.5 text-xs font-mono font-bold tracking-wider text-teal-300 uppercase">
                  <KeyRound className="w-3.5 h-3.5 text-teal-400" />
                  PROGRESSIVE CLUE LADDER ({unlockedClues.length}/3)
                </span>
                <span className="text-[11px] font-mono text-stone-400">Anti-Spoiler Guardrail</span>
              </div>

              {/* Clues Accordion List */}
              {curriculum.clues.map((clue: SocraticClue) => {
                const isUnlocked = unlockedClues.includes(clue.level)
                const isExpanded = expandedClueLevel === clue.level

                return (
                  <div
                    key={clue.level}
                    className={`rounded-xl border transition-all duration-200 overflow-hidden ${
                      isUnlocked
                        ? isExpanded
                          ? 'bg-stone-900/90 border-teal-500/40 shadow-[0_0_15px_rgba(20,184,166,0.1)]'
                          : 'bg-stone-900/60 border-stone-800 hover:border-stone-700'
                        : 'bg-stone-950/60 border-stone-800/80 opacity-75'
                    }`}
                  >
                    {/* Clue Header Bar */}
                    <div
                      onClick={() => {
                        if (isUnlocked) {
                          setExpandedClueLevel(isExpanded ? null : clue.level)
                        } else {
                          handleUnlockClue(clue.level)
                        }
                      }}
                      className="flex items-center justify-between px-3.5 py-3 cursor-pointer select-none"
                    >
                      <div className="flex items-center gap-2.5">
                        <div
                          className={`flex items-center justify-center w-6 h-6 rounded-md text-[11px] font-mono font-bold ${
                            isUnlocked
                              ? 'bg-teal-500/20 text-teal-300 border border-teal-500/30'
                              : 'bg-stone-800 text-stone-400 border border-stone-700'
                          }`}
                        >
                          L{clue.level}
                        </div>
                        <div>
                          <div className="text-[10px] font-mono text-stone-400 uppercase tracking-wider">
                            {clue.category}
                          </div>
                          <h4 className="text-xs sm:text-[13px] font-semibold text-stone-200">
                            {clue.title}
                          </h4>
                        </div>
                      </div>

                      <div className="flex items-center gap-2">
                        {isUnlocked ? (
                          <>
                            <span className="text-[10px] font-mono text-teal-400 flex items-center gap-1">
                              <Unlock className="w-3 h-3" />
                              TERBUKA
                            </span>
                            {isExpanded ? (
                              <ChevronUp className="w-4 h-4 text-stone-400" />
                            ) : (
                              <ChevronDown className="w-4 h-4 text-stone-400" />
                            )}
                          </>
                        ) : (
                          <button
                            onClick={(e) => {
                              e.stopPropagation()
                              handleUnlockClue(clue.level)
                            }}
                            className="px-2.5 py-1 rounded text-[11px] font-mono font-semibold bg-stone-800 hover:bg-teal-950/80 text-stone-300 hover:text-teal-300 border border-stone-700 hover:border-teal-500/40 transition-colors flex items-center gap-1.5 cursor-pointer"
                          >
                            <Lock className="w-3 h-3 text-stone-400" />
                            BUKA CLUE
                          </button>
                        )}
                      </div>
                    </div>

                    {/* Clue Content Accordion Body */}
                    {isUnlocked && isExpanded && (
                      <div className="px-3.5 pb-4 pt-1 border-t border-stone-800/80 text-xs text-stone-300 space-y-3">
                        <p className="leading-relaxed text-stone-300 bg-stone-950/40 p-2.5 rounded-lg border border-stone-800/60">
                          {clue.guidance}
                        </p>

                        {/* Investigative Questions */}
                        {clue.investigativeQuestions && clue.investigativeQuestions.length > 0 && (
                          <div className="space-y-1.5">
                            <span className="text-[11px] font-mono font-bold tracking-wide text-teal-400 flex items-center gap-1">
                              <HelpCircle className="w-3 h-3 text-teal-400" />
                              PERTANYAAN INVESTIGASI SOKRATES:
                            </span>
                            <ul className="space-y-1 pl-1">
                              {clue.investigativeQuestions.map((q, idx) => (
                                <li
                                  key={idx}
                                  className="flex items-start gap-2 text-stone-300 leading-normal"
                                >
                                  <span className="text-teal-400 text-xs mt-0.5">•</span>
                                  <span>{q}</span>
                                </li>
                              ))}
                            </ul>
                          </div>
                        )}

                        {/* Tactical Action & Code Blueprint (Level 3) */}
                        {clue.tacticalAction && (
                          <div className="p-2.5 rounded-lg bg-teal-950/30 border border-teal-500/30 text-teal-200 text-xs leading-relaxed">
                            <span className="font-mono font-bold block mb-1 text-[11px] text-teal-300">
                              // AKSI TAKTIS OBSERVASI:
                            </span>
                            {clue.tacticalAction}
                          </div>
                        )}

                        {clue.codeBlueprint && (
                          <div className="space-y-1">
                            <span className="font-mono text-[10px] text-amber-400 font-bold block">
                              // PAYLOAD BLUEPRINT (REPRESENTATIF):
                            </span>
                            <pre className="p-2.5 rounded-lg bg-stone-950 border border-stone-800 text-[11px] font-mono text-cyan-300 overflow-x-auto">
                              <code>{clue.codeBlueprint}</code>
                            </pre>
                          </div>
                        )}
                      </div>
                    )}
                  </div>
                )
              })}
            </div>
          </div>

          {/* ── RIGHT COLUMN: INTERACTIVE SOCRATIC CHAT CONSOLE (7 Cols) ─────── */}
          <div className="lg:col-span-7 flex flex-col bg-stone-950/40 h-full overflow-hidden">
            {/* Top Mode Switcher Bar */}
            <div className="px-4 py-2.5 bg-stone-950/90 border-b border-stone-800 flex items-center justify-between">
              <div className="flex items-center gap-1.5 p-1 rounded-xl bg-stone-900 border border-stone-800">
                <button
                  type="button"
                  onClick={() => setActiveConsoleTab('overview')}
                  className={`flex items-center gap-1.5 px-3 py-1 rounded-lg text-xs font-mono font-bold transition-all cursor-pointer ${
                    activeConsoleTab === 'overview'
                      ? 'bg-teal-500/20 text-teal-300 border border-teal-500/40 shadow-xs'
                      : 'text-stone-400 hover:text-stone-200 border border-transparent'
                  }`}
                >
                  <Sparkles className="w-3.5 h-3.5 text-teal-400" />
                  <span>AI OVERVIEW</span>
                </button>
                <button
                  type="button"
                  onClick={() => setActiveConsoleTab('chat')}
                  className={`flex items-center gap-1.5 px-3 py-1 rounded-lg text-xs font-mono font-bold transition-all cursor-pointer ${
                    activeConsoleTab === 'chat'
                      ? 'bg-stone-800 text-stone-200 border border-stone-700 shadow-xs'
                      : 'text-stone-400 hover:text-stone-200 border border-transparent'
                  }`}
                >
                  <Terminal className="w-3.5 h-3.5 text-stone-400" />
                  <span>SOCRATIC CHAT ({messages.length})</span>
                </button>
              </div>

              {/* Energy Badge Quick View */}
              <button
                type="button"
                onClick={() => {
                  setUpgradeTargetFeature('Overclock Instan & Unlimited AI Overview')
                  setUpgradeModalOpen(true)
                }}
                className="flex items-center gap-1.5 px-2.5 py-1 rounded-lg bg-teal-950/40 border border-teal-500/30 text-[11px] font-mono text-teal-300 hover:border-teal-400 transition-colors cursor-pointer"
                title="Buka Upgrade Modal untuk Unlimited Token"
              >
                <Cpu className="w-3 h-3 text-teal-400" />
                <span>ENERGI: {mentorEnergy}/5</span>
              </button>
            </div>

            {/* Notification Toast for Auto-Inject */}
            {injectToast && (
              <div className="mx-4 mt-2 px-3.5 py-2 rounded-xl bg-emerald-950/90 border border-emerald-500/60 text-emerald-200 text-xs font-mono flex items-center gap-2 shadow-lg animate-in fade-in slide-in-from-top-2">
                <ShieldCheck className="w-4 h-4 text-emerald-400 shrink-0" />
                <span className="flex-1 font-semibold">{injectToast}</span>
              </div>
            )}

            {/* Dynamic Console Body */}
            {activeConsoleTab === 'overview' ? (
              <div className="flex-1 p-4 md:p-5 overflow-y-auto custom-scrollbar">
                <ZiteraAiOverview
                  data={aiOverviewData}
                  currentTier={(userTier as 'free' | 'pro' | 'vip') || 'free'}
                  activeMentorId={selectedMentorId}
                  energy={mentorEnergy}
                  maxEnergy={5}
                  onAutoInject={handleAutoInject}
                  onOpenUpgradeModal={() => {
                    setUpgradeTargetFeature('Level 3 White-Box Code & DevSecOps Patch')
                    setUpgradeModalOpen(true)
                  }}
                  onMentorChange={handleSwitchMentor}
                />
              </div>
            ) : (
              /* Chat Messages Stream */
              <div
                ref={chatScrollRef}
                className="flex-1 p-4 md:p-5 overflow-y-auto custom-scrollbar space-y-4"
              >
                {messages.map((msg) => {
                  const isStudent = msg.sender === 'student'
                  const msgMentor = msg.mentorId ? getMentor(msg.mentorId) : currentMentor
                  const isThisMsgTyping =
                    msg.id === typingState.activeMessageId && typingState.isStreaming

                  return (
                    <div
                      key={msg.id}
                      className={`flex gap-3 ${isStudent ? 'justify-end' : 'justify-start'}`}
                    >
                      {!isStudent && (
                        <div className="w-9 h-9 rounded-xl bg-stone-900 border border-teal-500/40 flex items-center justify-center shrink-0 mt-0.5 overflow-hidden shadow-xs">
                          <img
                            src={msgMentor.avatar}
                            alt={msgMentor.name}
                            className="w-full h-full object-cover"
                            onError={(e) => {
                              ;(e.target as HTMLImageElement).src = '/assets/mentors/zeta_avatar.png'
                            }}
                          />
                        </div>
                      )}

                      <div
                        className={`max-w-[85%] rounded-2xl p-3.5 text-xs sm:text-[13px] leading-relaxed ${
                          isStudent
                            ? 'bg-teal-600 text-white rounded-br-xs shadow-md'
                            : msg.antiSpoilerWarning
                            ? 'bg-rose-950/60 border border-rose-500/40 text-rose-200 rounded-bl-xs'
                            : 'bg-stone-900/90 border border-stone-800 text-stone-200 rounded-bl-xs shadow-sm'
                        }`}
                      >
                        {!isStudent && (
                          <div className="flex items-center justify-between gap-3 pb-1 mb-1.5 border-b border-stone-800 text-[10px] font-mono text-stone-400">
                            <span className="font-bold text-teal-400 uppercase">
                              {msgMentor.name} // {msgMentor.codename}
                            </span>
                            <span>{msg.timestamp}</span>
                          </div>
                        )}

                        <p className="whitespace-pre-wrap">
                          {msg.text}
                          {isThisMsgTyping && (
                            <span className="inline-block w-2 h-3.5 ml-1 bg-teal-400 animate-pulse align-middle" />
                          )}
                        </p>

                        {msg.codeSnippet && (
                          <pre className="mt-2 p-2 rounded bg-stone-950 border border-stone-800 text-[11px] font-mono text-cyan-300 overflow-x-auto">
                            <code>{msg.codeSnippet}</code>
                          </pre>
                        )}

                        {isStudent && (
                          <div className="text-right text-[10px] font-mono text-teal-200/80 mt-1">
                            {msg.timestamp}
                          </div>
                        )}
                      </div>
                    </div>
                  )
                })}
              </div>
            )}

            {/* ============================================================== */}
            {/* TACTICAL QUESTION TREE PANEL (OPTION A - NO OPEN PROMPT)       */}
            {/* ============================================================== */}
            <div className="bg-stone-950/95 border-t border-stone-800 p-3 sm:p-4 space-y-2.5">
              {/* Header & Status Indicator */}
              <div className="flex items-center justify-between text-[11px] font-mono text-stone-400">
                <div className="flex items-center gap-2">
                  <Terminal className="w-3.5 h-3.5 text-teal-400" />
                  <span className="font-semibold text-stone-300">
                    PERTANYAAN TAKTIS // LEVEL {investigationDepth + 1}/3
                  </span>
                  {isTerminal ? (
                    <span className="px-2 py-0.5 rounded text-[10px] bg-emerald-950/80 border border-emerald-500/40 text-emerald-300 font-bold">
                      KESIMPULAN TERCAPAI
                    </span>
                  ) : (
                    <span className="text-stone-500 text-[10px]">
                      ({activeQuestions.length} Opsi Tersedia)
                    </span>
                  )}
                </div>

                <div className="flex items-center gap-2">
                  {typingState.isStreaming && (
                    <button
                      type="button"
                      onClick={handleSkipTyping}
                      className="px-2 py-0.5 rounded bg-teal-950/80 hover:bg-teal-900 border border-teal-500/50 text-teal-300 text-[10px] font-mono flex items-center gap-1 transition-colors cursor-pointer"
                      title="Lewati animasi ketik"
                    >
                      <FastForward className="w-3 h-3 text-teal-400" />
                      <span>Lewati Animasi</span>
                    </button>
                  )}

                  <button
                    type="button"
                    onClick={handleResetInvestigation}
                    className="px-2 py-0.5 rounded bg-stone-900 hover:bg-stone-800 border border-stone-700 text-stone-400 hover:text-stone-200 text-[10px] font-mono flex items-center gap-1 transition-colors cursor-pointer"
                    title="Kembalikan pertanyaan ke awal investigasi"
                  >
                    <RotateCcw className="w-3 h-3 text-stone-400" />
                    <span>Reset Awal</span>
                  </button>
                </div>
              </div>

              {/* Question Tree Action Cards */}
              {isTerminal ? (
                <div className="p-3.5 rounded-xl bg-stone-900/90 border border-emerald-500/30 flex flex-col sm:flex-row sm:items-center justify-between gap-3 animate-in fade-in">
                  <div className="space-y-1">
                    <div className="flex items-center gap-2 text-xs font-mono font-bold text-emerald-300">
                      <CheckCircle2 className="w-4 h-4 text-emerald-400" />
                      <span>Cabang Penyelidikan Selesai</span>
                    </div>
                    <p className="text-[12px] text-stone-300 font-sans leading-relaxed">
                      Hipotesis investigasi telah dirumuskan secara tuntas. Silakan uji temuan Anda pada lab atau telusuri jalur investigasi lainnya.
                    </p>
                  </div>

                  <button
                    type="button"
                    onClick={handleResetInvestigation}
                    className="px-4 py-2 rounded-lg bg-teal-600 hover:bg-teal-500 text-white text-xs font-mono font-bold flex items-center justify-center gap-2 transition-all shadow-md shrink-0 cursor-pointer"
                  >
                    <RotateCcw className="w-3.5 h-3.5" />
                    <span>[ ↺ Kembali ke Awal Investigasi ]</span>
                  </button>
                </div>
              ) : (
                <div className="max-h-40 overflow-y-auto custom-scrollbar space-y-2 pr-1">
                  {activeQuestions.map((q, idx) => (
                    <button
                      key={idx}
                      type="button"
                      disabled={isTyping || typingState.isStreaming}
                      onClick={() => handleSelectTacticalQuestion(q)}
                      className="w-full text-left p-2.5 rounded-xl bg-stone-900/80 hover:bg-teal-950/60 border border-stone-800 hover:border-teal-500/50 disabled:opacity-40 disabled:hover:bg-stone-900 transition-all flex items-start gap-2.5 group cursor-pointer"
                    >
                      <HelpCircle className="w-3.5 h-3.5 text-teal-400 mt-0.5 shrink-0 group-hover:scale-110 transition-transform" />
                      <span className="flex-1 text-xs text-stone-200 font-mono group-hover:text-teal-200 leading-snug">
                        {q}
                      </span>
                      <ArrowRight className="w-3.5 h-3.5 text-stone-500 group-hover:text-teal-400 group-hover:translate-x-0.5 transition-all mt-0.5 shrink-0" />
                    </button>
                  ))}
                </div>
              )}
            </div>
          </div>
        </div>

      {/* High-Conversion 3-Tier Decoy Upgrade Modal */}
      <UpgradeModal
        isOpen={upgradeModalOpen}
        onClose={() => setUpgradeModalOpen(false)}
        currentTier={(userTier as 'free' | 'pro' | 'vip') || 'free'}
        targetFeature={upgradeTargetFeature}
      />
    </div>
  )
}
