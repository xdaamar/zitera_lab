// ZITERA CYBER_LAB — Multi-Mentor Character Intelligence Modal
// Features multiple realistic anime cyber mentors (Zeta, Aoi, Elysia), dynamic teaching & praise poses,
// progressive Socratic clue ladder, and serverless anti-spoiler heuristic chat.
// Zero emojis — Lucide icons and masterclass vector/anime assets used exclusively.

import { useState, useEffect, useRef, type FormEvent } from 'react'
import {
  ShieldCheck,
  Terminal,
  Lock,
  Unlock,
  Send,
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

export interface ZetaMentorModalProps {
  isOpen: boolean
  onClose: () => void
  currentLabId: string
  userTier?: string | null
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
}: ZetaMentorModalProps) {
  const cleanLabId = (currentLabId || 'F01').trim().toUpperCase()
  const labTitle = LAB_CATALOG_TITLES[cleanLabId] || `Laboratorium ${cleanLabId}`
  const curriculum = getSocraticCurriculum(cleanLabId, labTitle)

  // ── Multi-Mentor Persona State ─────────────────────────────────────────────
  const [selectedMentorId, setSelectedMentorId] = useState<MentorId>('zeta')
  const currentMentor: MentorProfile = getMentor(selectedMentorId)

  // ── State Management ───────────────────────────────────────────────────────
  const [unlockedClues, setUnlockedClues] = useState<number[]>([1]) // Level 1 unlocked by default
  const [expandedClueLevel, setExpandedClueLevel] = useState<number | null>(1)
  const [activeMood, setActiveMood] = useState<ZetaMood>('idle')
  const [typedSpeech, setTypedSpeech] = useState<string>('')
  const [fullSpeechTarget, setFullSpeechTarget] = useState<string>('')
  const [isTyping, setIsTyping] = useState<boolean>(false)
  const [inputPrompt, setInputPrompt] = useState<string>('')
  const [messages, setMessages] = useState<ChatMessage[]>([])

  const chatScrollRef = useRef<HTMLDivElement>(null)
  const typingTimerRef = useRef<ReturnType<typeof setInterval> | null>(null)

  // ── Mentor Switching Handler ───────────────────────────────────────────────
  const handleSwitchMentor = (mentorId: MentorId) => {
    setSelectedMentorId(mentorId)
    const mentor = getMentor(mentorId)
    const newGreeting = mentor.greeting(cleanLabId, labTitle)
    setActiveMood('idle')
    setFullSpeechTarget(newGreeting)
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
    setFullSpeechTarget(greeting)
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
  }, [isOpen, cleanLabId, labTitle])

  // ── Typewriter Animation Pacing ────────────────────────────────────────────
  useEffect(() => {
    if (!fullSpeechTarget) return

    if (typingTimerRef.current) {
      clearInterval(typingTimerRef.current)
    }

    setTypedSpeech('')
    setIsTyping(true)
    let charIndex = 0

    typingTimerRef.current = setInterval(() => {
      charIndex += 2 // Type 2 chars per tick for responsive tactical cadence
      if (charIndex >= fullSpeechTarget.length) {
        setTypedSpeech(fullSpeechTarget)
        setIsTyping(false)
        if (typingTimerRef.current) clearInterval(typingTimerRef.current)
      } else {
        setTypedSpeech(fullSpeechTarget.slice(0, charIndex))
      }
    }, 18)

    return () => {
      if (typingTimerRef.current) clearInterval(typingTimerRef.current)
    }
  }, [fullSpeechTarget])

  // Auto-scroll chat console
  useEffect(() => {
    if (chatScrollRef.current) {
      chatScrollRef.current.scrollTop = chatScrollRef.current.scrollHeight
    }
  }, [messages, typedSpeech])

  // Instant skip typewriter
  const handleSkipTyping = () => {
    if (isTyping && fullSpeechTarget) {
      if (typingTimerRef.current) clearInterval(typingTimerRef.current)
      setTypedSpeech(fullSpeechTarget)
      setIsTyping(false)
    }
  }

  const isFoundationTier = userTier !== 'pro' && userTier !== 'enterprise'

  // ── Unlock Clue Progression ────────────────────────────────────────────────
  const handleUnlockClue = (level: number) => {
    // Tier Gating Enforcement
    if (level === 2 && isFoundationTier) {
      const upgradeMsg = `Clue Level 2 (Vulnerability Logic) adalah materi analisis mendalam. Selesaikan tantangan awal atau tingkatkan ke Pro Access untuk membuka tier analisis ini.`
      setActiveMood('warning')
      setFullSpeechTarget(upgradeMsg)
      return
    }

    if (level === 3 && isFoundationTier) {
      const upgradeMsg = `Clue Level 3 (Payload Blueprint) dilindungi enkripsi lisensi. Fitur ini dirancang khusus untuk anggota Pro & Enterprise Operatives.`
      setActiveMood('warning')
      setFullSpeechTarget(upgradeMsg)
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
      setFullSpeechTarget(unlockSpeech)
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

  // ── Submit Student Prompt to Heuristic Socratic Engine ─────────────────────
  const handleSendMessage = (e?: FormEvent) => {
    if (e) e.preventDefault()
    const query = inputPrompt.trim()
    if (!query || isTyping) return

    const studentTime = new Date().toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' })
    const studentMsg: ChatMessage = {
      id: `std_${Date.now()}`,
      sender: 'student',
      text: query,
      timestamp: studentTime,
    }

    setMessages((prev) => [...prev, studentMsg])
    setInputPrompt('')
    setActiveMood('thinking')

    // Evaluate heuristic Socratic engine response with slight delay for realistic processing
    setTimeout(() => {
      const response = querySocraticEngine(query, cleanLabId)
      const mentorTime = new Date().toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' })

      // Tailor response tone slightly based on active mentor persona
      let personalizedText = response.message
      if (selectedMentorId === 'aoi' && !response.antiSpoilerWarning) {
        personalizedText = `[Aoi Exploit Note]: ${response.message}`
      } else if (selectedMentorId === 'elysia' && !response.antiSpoilerWarning) {
        personalizedText = `[Elysia Defense Review]: ${response.message}`
      }

      setActiveMood(response.mood)
      setFullSpeechTarget(personalizedText)

      setMessages((prev) => [
        ...prev,
        {
          id: `zeta_${Date.now()}`,
          sender: 'zeta',
          text: personalizedText,
          timestamp: mentorTime,
          mood: response.mood,
          antiSpoilerWarning: response.antiSpoilerWarning,
          mentorId: selectedMentorId,
        },
      ])
    }, 380)
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
    <div className="fixed inset-0 z-[10000] flex items-center justify-center p-2 sm:p-4 md:p-6 bg-stone-950/85 backdrop-blur-md select-none overflow-hidden animate-in fade-in duration-200">
      {/* Container Card */}
      <div className="relative w-full max-w-6xl h-[94vh] max-h-[920px] flex flex-col rounded-2xl bg-stone-900/95 border border-teal-500/30 shadow-[0_0_60px_rgba(0,0,0,0.8),0_0_20px_rgba(20,184,166,0.15)] overflow-hidden text-stone-200">
        {/* Subtle Cyber Grid Background Texture */}
        <div className="absolute inset-0 pointer-events-none opacity-[0.03] bg-[linear-gradient(to_right,#14b8a6_1px,transparent_1px),linear-gradient(to_bottom,#14b8a6_1px,transparent_1px)] bg-[size:24px_24px]" />

        {/* ── TOP HEADER BAR ───────────────────────────────────────────────── */}
        <header className="relative z-10 flex flex-wrap items-center justify-between px-4 sm:px-5 py-3 border-b border-stone-800 bg-stone-950/85 backdrop-blur-sm gap-3">
          {/* Brand & Companion Identity */}
          <div className="flex items-center gap-3">
            <div className="relative flex items-center justify-center w-9 h-9 rounded-lg bg-stone-900 border border-teal-500/40 shadow-[0_0_12px_rgba(20,184,166,0.2)] shrink-0 overflow-hidden">
              <img
                src={currentMentor.avatar}
                alt={currentMentor.name}
                className="w-full h-full object-cover"
                onError={(e) => {
                  ;(e.target as HTMLImageElement).src = '/assets/zeta_avatar.png'
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
                    className={`flex items-center gap-1.5 px-2.5 py-1 rounded-lg text-xs font-['Space_Grotesk'] font-bold transition-all cursor-pointer ${
                      isSelected
                        ? `${m.badgeBg} ${m.badgeText} border shadow-xs`
                        : 'text-stone-400 hover:text-stone-200 hover:bg-stone-800/60 border border-transparent'
                    }`}
                    title={`Ganti mentor ke ${m.name} (${m.role})`}
                  >
                    <span
                      className="w-2 h-2 rounded-full"
                      style={{ backgroundColor: m.themeColor }}
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

            {/* Close Modal Button */}
            <button
              onClick={onClose}
              className="p-1.5 rounded-lg text-stone-400 hover:text-white hover:bg-stone-800 border border-transparent hover:border-stone-700 transition-colors cursor-pointer"
              title="Tutup Modal (Esc)"
              aria-label="Tutup Modal"
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
              <div className="relative w-full h-56 sm:h-64 rounded-xl overflow-hidden border border-stone-800 bg-gradient-to-t from-stone-950 via-stone-900 to-stone-950 flex items-center justify-center group shadow-inner">
                <img
                  src={activePoseImage}
                  alt={`${currentMentor.name} Teaching Pose`}
                  className="w-full h-full object-contain object-bottom transition-all duration-300 group-hover:scale-105 select-none"
                  onError={(e) => {
                    ;(e.target as HTMLImageElement).src = currentMentor.avatar
                  }}
                />

                {/* Subtle vignette shadow */}
                <div className="absolute inset-0 bg-gradient-to-t from-stone-950/80 via-transparent to-transparent pointer-events-none" />

                {/* Live Specialty Badge Overlay */}
                <div className="absolute top-2.5 left-2.5 px-2.5 py-0.5 rounded-full bg-stone-950/85 border border-stone-700 text-[10px] font-mono font-bold text-stone-200 flex items-center gap-1.5 backdrop-blur-xs">
                  <span
                    className="w-2 h-2 rounded-full animate-ping"
                    style={{ backgroundColor: currentMentor.themeColor }}
                  />
                  <span>{currentMentor.name}: {currentMentor.specialty}</span>
                </div>

                {/* Active Pose Mode Tag */}
                <div className="absolute bottom-2.5 right-2.5 px-2 py-0.5 rounded bg-stone-950/85 border border-stone-800 text-[9px] font-mono text-stone-400 backdrop-blur-xs flex items-center gap-1">
                  <UserCheck className="w-2.5 h-2.5 text-teal-400" />
                  <span>{currentMoodUi.poseLabel}</span>
                </div>
              </div>

              {/* Cyber-Tactical Speech Bubble with Typewriter Animation */}
              <div
                onClick={handleSkipTyping}
                className="rounded-xl p-3.5 bg-stone-900/90 border border-stone-700/70 text-xs sm:text-[13px] leading-relaxed text-stone-200 cursor-pointer hover:border-teal-500/40 transition-colors shadow-inner"
                title="Klik untuk menyelesaikan ketikan instan"
              >
                <div className="flex items-center gap-2 mb-1.5 font-mono text-[10px] text-teal-400 font-bold uppercase">
                  <span>// {currentMentor.name} BIMBINGAN INTERAKTIF</span>
                </div>
                <p className="whitespace-pre-wrap font-sans">
                  {typedSpeech || fullSpeechTarget}
                  {isTyping && (
                    <span className="inline-block w-1.5 h-3.5 ml-1 bg-teal-400 animate-pulse align-middle" />
                  )}
                </p>
                {isTyping && (
                  <div className="mt-2 flex items-center justify-between text-[10px] font-mono text-stone-500">
                    <span>{currentMentor.name} sedang membimbing...</span>
                    <span className="text-teal-400 hover:underline">Klik untuk percepat</span>
                  </div>
                )}
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
            {/* Chat Messages Stream */}
            <div
              ref={chatScrollRef}
              className="flex-1 p-4 md:p-5 overflow-y-auto custom-scrollbar space-y-4"
            >
              {messages.map((msg) => {
                const isStudent = msg.sender === 'student'
                const msgMentor = msg.mentorId ? getMentor(msg.mentorId) : currentMentor

                return (
                  <div
                    key={msg.id}
                    className={`flex gap-3 ${isStudent ? 'justify-end' : 'justify-start'}`}
                  >
                    {!isStudent && (
                      <div className="w-8 h-8 rounded-lg bg-stone-900 border border-teal-500/40 flex items-center justify-center shrink-0 mt-0.5 overflow-hidden">
                        <img
                          src={msgMentor.avatar}
                          alt={msgMentor.name}
                          className="w-full h-full object-cover"
                          onError={(e) => {
                            ;(e.target as HTMLImageElement).src = '/assets/zeta_avatar.png'
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

                      <p className="whitespace-pre-wrap">{msg.text}</p>

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

            {/* Quick Socratic Suggestion Chips */}
            <div className="px-4 py-2 bg-stone-950/80 border-t border-stone-800/80 flex items-center gap-1.5 overflow-x-auto scrollbar-none">
              <span className="text-[10px] font-mono text-stone-500 shrink-0 mr-1">
                PERTANYAAN TAKTIS:
              </span>
              {curriculum.suggestedPrompts.slice(0, 3).map((prompt, idx) => (
                <button
                  key={idx}
                  onClick={() => {
                    setInputPrompt(prompt)
                  }}
                  className="px-2.5 py-1 rounded-full text-[11px] font-mono text-stone-300 bg-stone-900 hover:bg-teal-950/80 hover:text-teal-300 border border-stone-700/80 hover:border-teal-500/40 transition-colors shrink-0 flex items-center gap-1 cursor-pointer"
                >
                  <ArrowRight className="w-2.5 h-2.5 text-teal-400" />
                  <span className="truncate max-w-[200px]">{prompt}</span>
                </button>
              ))}
            </div>

            {/* Input Form Bar */}
            <form
              onSubmit={handleSendMessage}
              className="p-3 bg-stone-950 border-t border-stone-800 flex items-center gap-2"
            >
              <div className="relative flex-1">
                <input
                  type="text"
                  value={inputPrompt}
                  onChange={(e) => setInputPrompt(e.target.value)}
                  placeholder={`Tanya ${currentMentor.name} seputar logika celah di ${cleanLabId}...`}
                  className="w-full px-3.5 py-2.5 rounded-xl bg-stone-900 border border-stone-700 text-stone-200 placeholder-stone-500 text-xs sm:text-[13px] focus:outline-none focus:border-teal-400 transition-colors pr-10 font-sans"
                />
                <span className="absolute right-3 top-3 text-[10px] font-mono text-stone-500 hidden sm:inline">
                  Enter ↵
                </span>
              </div>

              <button
                type="submit"
                disabled={!inputPrompt.trim() || isTyping}
                className="px-4 py-2.5 rounded-xl bg-teal-600 hover:bg-teal-500 disabled:opacity-40 disabled:hover:bg-teal-600 text-white font-['Space_Grotesk'] font-bold text-xs flex items-center gap-1.5 transition-all shadow-sm cursor-pointer"
                title="Kirim pesan ke mentor"
              >
                <span>Kirim</span>
                <Send className="w-3.5 h-3.5" />
              </button>
            </form>
          </div>
        </div>
      </div>
    </div>
  )
}
