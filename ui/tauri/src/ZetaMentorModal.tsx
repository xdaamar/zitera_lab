// ZITERA CYBER_LAB — Zeta Socratic Cyber Mentor Modal
// Visual anime companion, progressive hint ladder, and serverless Socratic heuristic chat.
// Zero emojis — Lucide icons used exclusively.

import { useState, useEffect, useRef, type FormEvent } from 'react'
import {
  ShieldCheck,
  Terminal,
  Lock,
  Unlock,
  Send,
  MessageSquare,
  Sparkles,
  AlertTriangle,
  ChevronDown,
  ChevronUp,
  ExternalLink,
  HelpCircle,
  X,
  Compass,
  Cpu,
  ArrowRight,
  Zap,
  KeyRound,
} from 'lucide-react'
import {
  getSocraticCurriculum,
  querySocraticEngine,
  type ZetaMood,
  type SocraticClue,
} from './zetaSocraticData'
import { canUserAccessLab, LAB_CATALOG_TITLES } from './progressionHelper'

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
  const accessResult = canUserAccessLab(cleanLabId, userTier)

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

  // ── Initial Setup on Open / Lab Change ─────────────────────────────────────
  useEffect(() => {
    if (!isOpen) return

    // Initial greeting from Zeta
    const greeting = `Salam taktis, Operative. Saya Zeta, mentor intelijen siber Anda. Saya mendampingi Anda di ${labTitle}. Misi saya adalah melatih insting investigasi Anda melalui bimbingan Sokrates—mengajarkan Anda cara berpikir dan menemukan celah tanpa merusak keseruan tantangan dengan spoiler langsung. Bagaimana progres pemetaan Anda sejauh ini?`

    setActiveMood('idle')
    setFullSpeechTarget(greeting)
    setMessages([
      {
        id: 'msg_init',
        sender: 'zeta',
        text: greeting,
        timestamp: new Date().toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' }),
        mood: 'idle',
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

  // Escape key listener
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.key === 'Escape' && isOpen) {
        onClose()
      }
    }
    window.addEventListener('keydown', handleKeyDown)
    return () => window.removeEventListener('keydown', handleKeyDown)
  }, [isOpen, onClose])

  if (!isOpen) return null

  // ── Action Handlers ────────────────────────────────────────────────────────
  const handleSkipTyping = () => {
    if (typingTimerRef.current) clearInterval(typingTimerRef.current)
    setTypedSpeech(fullSpeechTarget)
    setIsTyping(false)
  }

  const handleUnlockClue = (level: number) => {
    if (!unlockedClues.includes(level)) {
      setUnlockedClues((prev) => [...prev, level])
    }
    setExpandedClueLevel(level)
    setActiveMood(level === 3 ? 'eureka' : 'thinking')

    const clueData = curriculum.clues.find((c) => c.level === level)
    if (clueData) {
      const announceText = `Petunjuk Tingkat ${level} (${clueData.category}): ${clueData.summary} Telaah pertanyaan investigasi dan cetak biru taktis yang telah saya sediakan di panel ini.`
      setFullSpeechTarget(announceText)
      setMessages((prev) => [
        ...prev,
        {
          id: `unlock_${level}_${Date.now()}`,
          sender: 'zeta',
          text: announceText,
          timestamp: new Date().toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' }),
          mood: level === 3 ? 'eureka' : 'thinking',
          codeSnippet: clueData.codeBlueprint,
        },
      ])
    }
  }

  const handleSendMessage = (e?: FormEvent) => {
    if (e) e.preventDefault()
    const prompt = inputPrompt.trim()
    if (!prompt) return

    const studentTimestamp = new Date().toLocaleTimeString([], {
      hour: '2-digit',
      minute: '2-digit',
    })
    const newStudentMsg: ChatMessage = {
      id: `student_${Date.now()}`,
      sender: 'student',
      text: prompt,
      timestamp: studentTimestamp,
    }

    setMessages((prev) => [...prev, newStudentMsg])
    setInputPrompt('')
    setActiveMood('thinking')

    // Evaluate Socratic Engine
    setTimeout(() => {
      const response = querySocraticEngine(prompt, cleanLabId)
      setActiveMood(response.mood)
      setFullSpeechTarget(response.message)

      const zetaTimestamp = new Date().toLocaleTimeString([], {
        hour: '2-digit',
        minute: '2-digit',
      })
      const newZetaMsg: ChatMessage = {
        id: `zeta_${Date.now()}`,
        sender: 'zeta',
        text: response.message,
        timestamp: zetaTimestamp,
        mood: response.mood,
        antiSpoilerWarning: response.antiSpoilerWarning,
      }
      setMessages((prev) => [...prev, newZetaMsg])
    }, 280)
  }

  const handlePromptChipClick = (chipText: string) => {
    setInputPrompt(chipText)
  }

  const handleOpenWhatsApp = () => {
    const waUrl =
      'https://wa.me/?text=' +
      encodeURIComponent(
        `Halo Admin ZITERA LAB, saya sedang mengerjakan laboratorium ${cleanLabId} (${labTitle}) dan ingin mengupgrade lisensi resmi ZITERA LAB Pro Edition agar dapat mengakses seluruh kurikulum spesialis dan materi eksklusif. Mohon info aktivasi lisensinya. Terima kasih!`,
      )
    window.open(waUrl, '_blank')
  }

  // ── Tactical Companion Visual Config ──────────────────────────────────────
  const moodConfig: Record<
    ZetaMood,
    {
      label: string
      badgeBg: string
      badgeText: string
      avatarRing: string
      bubbleBorder: string
      glowColor: string
      icon: typeof Sparkles
    }
  > = {
    idle: {
      label: 'SIAGA / READY',
      badgeBg: 'bg-teal-500/10 border-teal-500/30',
      badgeText: 'text-teal-400',
      avatarRing: 'ring-teal-500/50 shadow-[0_0_24px_rgba(20,184,166,0.3)]',
      bubbleBorder: 'border-teal-500/30 shadow-[0_0_20px_rgba(20,184,166,0.1)]',
      glowColor: 'from-teal-500/20 to-cyan-500/10',
      icon: Compass,
    },
    thinking: {
      label: 'MEMPROSES VEKTOR',
      badgeBg: 'bg-amber-500/10 border-amber-500/30',
      badgeText: 'text-amber-400',
      avatarRing: 'ring-amber-500/50 shadow-[0_0_24px_rgba(245,158,11,0.3)]',
      bubbleBorder: 'border-amber-500/30 shadow-[0_0_20px_rgba(245,158,11,0.1)]',
      glowColor: 'from-amber-500/20 to-orange-500/10',
      icon: Cpu,
    },
    eureka: {
      label: 'INSIGHT DITEMUKAN',
      badgeBg: 'bg-cyan-500/10 border-cyan-500/30',
      badgeText: 'text-cyan-300',
      avatarRing: 'ring-cyan-400/70 shadow-[0_0_30px_rgba(6,182,212,0.45)]',
      bubbleBorder: 'border-cyan-400/40 shadow-[0_0_24px_rgba(6,182,212,0.15)]',
      glowColor: 'from-cyan-500/25 to-emerald-500/15',
      icon: Sparkles,
    },
    warning: {
      label: 'ANTI-SPOILER GUARD',
      badgeBg: 'bg-rose-500/10 border-rose-500/30',
      badgeText: 'text-rose-400',
      avatarRing: 'ring-rose-500/60 shadow-[0_0_28px_rgba(244,63,94,0.4)]',
      bubbleBorder: 'border-rose-500/30 shadow-[0_0_20px_rgba(244,63,94,0.15)]',
      glowColor: 'from-rose-500/25 to-amber-500/10',
      icon: AlertTriangle,
    },
  }

  const currentMoodUi = moodConfig[activeMood]
  const MoodIcon = currentMoodUi.icon

  return (
    <div className="fixed inset-0 z-[10000] flex items-center justify-center p-3 sm:p-4 md:p-6 bg-stone-950/85 backdrop-blur-md select-none overflow-hidden animate-in fade-in duration-200">
      {/* Container Card */}
      <div className="relative w-full max-w-6xl h-[92vh] max-h-[880px] flex flex-col rounded-2xl bg-stone-900/95 border border-teal-500/30 shadow-[0_0_60px_rgba(0,0,0,0.8),0_0_20px_rgba(20,184,166,0.15)] overflow-hidden text-stone-200">
        {/* Subtle Cyber Grid Background Texture */}
        <div className="absolute inset-0 pointer-events-none opacity-[0.03] bg-[linear-gradient(to_right,#14b8a6_1px,transparent_1px),linear-gradient(to_bottom,#14b8a6_1px,transparent_1px)] bg-[size:24px_24px]" />

        {/* ── TOP HEADER BAR ───────────────────────────────────────────────── */}
        <header className="relative z-10 flex items-center justify-between px-5 py-3.5 border-b border-stone-800 bg-stone-950/80 backdrop-blur-sm">
          {/* Brand & Companion Identity */}
          <div className="flex items-center gap-3">
            <div className="relative flex items-center justify-center w-9 h-9 rounded-lg bg-teal-950/60 border border-teal-500/40 shadow-[0_0_12px_rgba(20,184,166,0.3)]">
              <img
                src="/assets/zeta_avatar.png"
                alt="Zeta"
                className="w-7 h-7 rounded-md object-cover"
                onError={(e) => {
                  ;(e.target as HTMLImageElement).src = '/assets/zeta_avatar.png'
                }}
              />
              <span className="absolute -bottom-0.5 -right-0.5 w-2.5 h-2.5 rounded-full bg-teal-400 border-2 border-stone-950 animate-pulse" />
            </div>

            <div>
              <div className="flex items-center gap-2">
                <span className="font-['Space_Grotesk'] font-bold text-base tracking-wide bg-gradient-to-r from-teal-200 via-cyan-200 to-white bg-clip-text text-transparent">
                  ZETA // SOCRATIC CYBER MENTOR
                </span>
                <span className="hidden sm:inline-flex items-center gap-1 px-2 py-0.5 rounded text-[10px] font-mono font-bold tracking-wider uppercase bg-teal-500/10 text-teal-300 border border-teal-500/20">
                  <ShieldCheck className="w-3 h-3 text-teal-400" />
                  PEDAGOGICAL ENGINE
                </span>
              </div>
              <p className="text-[11px] font-mono text-stone-400 truncate max-w-[280px] sm:max-w-md">
                Target: <span className="text-teal-300 font-semibold">{cleanLabId}</span> -{' '}
                {labTitle}
              </p>
            </div>
          </div>

          {/* Right Controls: Tier Badge & Close */}
          <div className="flex items-center gap-2.5">
            <span
              className={`px-2.5 py-1 rounded-full text-[10px] font-mono font-bold tracking-widest uppercase border ${
                userTier === 'pro' || userTier === 'enterprise'
                  ? 'bg-amber-500/10 text-amber-300 border-amber-500/30'
                  : 'bg-stone-800 text-stone-300 border-stone-700'
              }`}
            >
              TIER: {(userTier || 'FOUNDATION').toUpperCase()}
            </span>

            <button
              onClick={onClose}
              className="p-1.5 rounded-lg text-stone-400 hover:text-white hover:bg-stone-800 border border-transparent hover:border-stone-700 transition-colors"
              title="Tutup Modal (Esc)"
              aria-label="Tutup Modal"
            >
              <X className="w-5 h-5" />
            </button>
          </div>
        </header>

        {/* ── MAIN TWO-COLUMN WORKSPACE ─────────────────────────────────────── */}
        <div className="relative z-10 flex-1 grid grid-cols-1 lg:grid-cols-12 overflow-hidden">
          {/* ── LEFT COLUMN: ANIME COMPANION & PROGRESSIVE HINT LADDER (5 Cols) ── */}
          <div className="lg:col-span-5 flex flex-col border-b lg:border-b-0 lg:border-r border-stone-800 bg-stone-950/50 overflow-y-auto custom-scrollbar p-4 md:p-5 gap-4">
            {/* Visual Anime Companion Card */}
            <div
              className={`relative rounded-xl border p-4 bg-gradient-to-b ${currentMoodUi.glowColor} ${currentMoodUi.bubbleBorder} transition-all duration-300`}
            >
              {/* Tactical Status Bar */}
              <div className="flex items-center justify-between mb-3">
                <span className="flex items-center gap-1.5 text-[11px] font-mono font-semibold text-stone-300">
                  <Terminal className="w-3.5 h-3.5 text-teal-400" />
                  UNIT: ZETA-SOCRATIC-01
                </span>
                <span
                  className={`inline-flex items-center gap-1 px-2 py-0.5 rounded text-[10px] font-mono font-bold border ${currentMoodUi.badgeBg} ${currentMoodUi.badgeText}`}
                >
                  <MoodIcon className="w-3 h-3" />
                  {currentMoodUi.label}
                </span>
              </div>

              {/* Avatar + Speech Bubble Flex Container */}
              <div className="flex items-start gap-3.5">
                {/* Anime Avatar Portrait with Dynamic Ring */}
                <div className="relative shrink-0">
                  <div
                    className={`relative w-20 h-20 sm:w-24 sm:h-24 rounded-2xl overflow-hidden ring-2 ${currentMoodUi.avatarRing} bg-stone-900 transition-all duration-300`}
                  >
                    <img
                      src="/assets/zeta_avatar.png"
                      alt="Zeta Cyber Mentor"
                      className="w-full h-full object-cover select-none"
                      onError={(e) => {
                        ;(e.target as HTMLImageElement).src = '/assets/zeta_avatar.png'
                      }}
                    />
                    <div className="absolute inset-0 bg-gradient-to-t from-black/40 via-transparent to-transparent pointer-events-none" />
                  </div>

                  {/* Micro indicator badge */}
                  <div className="absolute -bottom-1 -right-1 px-1.5 py-0.5 rounded text-[8px] font-mono font-bold uppercase bg-stone-900 border border-teal-500/40 text-teal-300">
                    AI MENTOR
                  </div>
                </div>

                {/* Cyber-Tactical Speech Bubble with Typewriter pacing */}
                <div
                  onClick={handleSkipTyping}
                  className="flex-1 min-h-[90px] rounded-xl p-3 bg-stone-900/90 border border-stone-700/70 text-xs sm:text-[13px] leading-relaxed text-stone-200 cursor-pointer hover:border-teal-500/40 transition-colors shadow-inner"
                  title="Klik untuk menyelesaikan ketikan instan"
                >
                  <p className="whitespace-pre-wrap font-sans">
                    {typedSpeech || fullSpeechTarget}
                    {isTyping && (
                      <span className="inline-block w-1.5 h-3.5 ml-1 bg-teal-400 animate-pulse align-middle" />
                    )}
                  </p>
                  {isTyping && (
                    <div className="mt-2 flex items-center justify-between text-[10px] font-mono text-stone-500">
                      <span>Zeta sedang menyusun bimbingan...</span>
                      <span className="text-teal-400 hover:underline">Klik untuk percepat</span>
                    </div>
                  )}
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
                          <div className="flex items-center gap-1.5">
                            <span className="text-[11px] font-mono font-semibold uppercase tracking-wider text-teal-400">
                              LEVEL {clue.level} // {clue.category}
                            </span>
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
                            className="px-2.5 py-1 rounded text-[11px] font-mono font-semibold bg-stone-800 hover:bg-teal-950/80 text-stone-300 hover:text-teal-300 border border-stone-700 hover:border-teal-500/40 transition-colors flex items-center gap-1.5"
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
                                  className="flex items-start gap-1.5 text-[11px] sm:text-xs text-stone-400"
                                >
                                  <ArrowRight className="w-3 h-3 text-teal-400 shrink-0 mt-0.5" />
                                  <span>{q}</span>
                                </li>
                              ))}
                            </ul>
                          </div>
                        )}

                        {/* Code / Command Blueprint */}
                        {clue.codeBlueprint && (
                          <div className="space-y-1.5">
                            <div className="flex items-center justify-between">
                              <span className="text-[11px] font-mono font-bold tracking-wide text-amber-400 flex items-center gap-1">
                                <Terminal className="w-3 h-3 text-amber-400" />
                                CETAK BIRU TAKTIS (ANTI-SPOILER):
                              </span>
                              <span className="text-[9px] font-mono text-stone-500">
                                BUKAN FLAG MENTAH
                              </span>
                            </div>
                            <pre className="p-2.5 rounded-lg bg-stone-950 border border-stone-800 text-[11px] font-mono text-teal-300 overflow-x-auto whitespace-pre-wrap leading-snug">
                              {clue.codeBlueprint}
                            </pre>
                          </div>
                        )}
                      </div>
                    )}
                  </div>
                )
              })}
            </div>

            {/* ── LOCKED / PRO TIER CONVERSION CALLOUT ──────────────────────── */}
            {accessResult.isTierLocked && (
              <div className="mt-auto rounded-xl border border-amber-500/30 bg-gradient-to-r from-amber-950/40 via-stone-900 to-amber-950/20 p-3.5 space-y-2.5">
                <div className="flex items-center gap-2">
                  <div className="p-1.5 rounded-lg bg-amber-500/20 border border-amber-500/40 text-amber-300">
                    <Lock className="w-4 h-4" />
                  </div>
                  <div>
                    <h4 className="text-xs font-bold text-amber-200 uppercase tracking-wide">
                      ZITERA PRO EDITION REQUIRED
                    </h4>
                    <p className="text-[11px] text-stone-400">
                      Laboratorium ini merupakan bagian dari kurikulum spesialis / Pro Track.
                    </p>
                  </div>
                </div>

                <p className="text-[11px] text-stone-300 leading-relaxed">
                  Dapatkan akses penuh ke 45 laboratorium OWASP Top 10 Enterprise, materi bimbingan
                  mendalam, dan simulasi pengujian bug bounty komersial.
                </p>

                <button
                  onClick={handleOpenWhatsApp}
                  className="w-full py-2 px-3 rounded-lg text-xs font-mono font-bold bg-amber-500 hover:bg-amber-400 text-stone-950 transition-colors flex items-center justify-center gap-2 shadow-[0_0_15px_rgba(245,158,11,0.2)]"
                >
                  <MessageSquare className="w-4 h-4" />
                  UPGRADE VIA WHATSAPP (1-KLIK)
                  <ExternalLink className="w-3.5 h-3.5" />
                </button>
              </div>
            )}
          </div>

          {/* ── RIGHT COLUMN: FREE-FORM SOCRATIC CHAT CONSOLE (7 Cols) ────────── */}
          <div className="lg:col-span-7 flex flex-col bg-stone-950/80 overflow-hidden">
            {/* Chat Console Header */}
            <div className="flex items-center justify-between px-4 py-2.5 border-b border-stone-800 bg-stone-950 text-stone-400 text-xs font-mono">
              <span className="flex items-center gap-1.5 text-stone-300 font-semibold">
                <Terminal className="w-3.5 h-3.5 text-teal-400" />
                SOCRATIC INQUIRY TERMINAL
              </span>
              <span className="text-[11px] text-stone-500">100% Client-Side Engine // Zero API Cost</span>
            </div>

            {/* Chat Messages Stream */}
            <div
              ref={chatScrollRef}
              className="flex-1 overflow-y-auto custom-scrollbar p-4 space-y-3.5 text-xs sm:text-[13px]"
            >
              {messages.map((msg) => {
                const isZeta = msg.sender === 'zeta'
                return (
                  <div
                    key={msg.id}
                    className={`flex items-start gap-2.5 ${isZeta ? 'justify-start' : 'justify-end'}`}
                  >
                    {isZeta && (
                      <div className="w-7 h-7 rounded-lg overflow-hidden shrink-0 border border-teal-500/40 bg-stone-900 mt-0.5">
                        <img
                          src="/assets/zeta_avatar.png"
                          alt="Zeta"
                          className="w-full h-full object-cover"
                        />
                      </div>
                    )}

                    <div
                      className={`max-w-[85%] rounded-xl p-3 leading-relaxed ${
                        isZeta
                          ? msg.antiSpoilerWarning
                            ? 'bg-rose-950/40 border border-rose-500/40 text-rose-200'
                            : 'bg-stone-900/90 border border-stone-800 text-stone-200 shadow-sm'
                          : 'bg-teal-600/90 text-white font-medium shadow-sm'
                      }`}
                    >
                      {/* Sender Tag & Timestamp */}
                      <div className="flex items-center justify-between gap-3 text-[10px] font-mono opacity-60 mb-1">
                        <span>{isZeta ? 'MENTOR ZETA' : 'STUDENT / OPERATIVE'}</span>
                        <span>{msg.timestamp}</span>
                      </div>

                      <p className="whitespace-pre-wrap">{msg.text}</p>

                      {/* Code Snippet if present */}
                      {msg.codeSnippet && (
                        <pre className="mt-2 p-2 rounded bg-black/60 border border-stone-700/60 text-[11px] font-mono text-teal-300 overflow-x-auto">
                          {msg.codeSnippet}
                        </pre>
                      )}
                    </div>

                    {!isZeta && (
                      <div className="w-7 h-7 rounded-lg shrink-0 border border-teal-400 bg-teal-500/20 text-teal-300 flex items-center justify-center font-mono font-bold text-xs mt-0.5">
                        U
                      </div>
                    )}
                  </div>
                )
              })}
            </div>

            {/* Quick Suggestion Chips */}
            <div className="px-3 pt-2 pb-1 border-t border-stone-800 bg-stone-950/90">
              <div className="flex items-center gap-1.5 overflow-x-auto no-scrollbar py-1 text-[11px] font-mono">
                <span className="text-stone-500 flex items-center gap-1 shrink-0">
                  <Zap className="w-3 h-3 text-teal-400" />
                  QUICK PROMPT:
                </span>
                {curriculum.suggestedPrompts.slice(0, 3).map((prompt, pIdx) => (
                  <button
                    key={pIdx}
                    onClick={() => handlePromptChipClick(prompt)}
                    className="shrink-0 px-2.5 py-1 rounded-full bg-stone-900 hover:bg-teal-950/60 text-stone-300 hover:text-teal-200 border border-stone-800 hover:border-teal-500/30 transition-colors truncate max-w-[240px]"
                    title={prompt}
                  >
                    {prompt}
                  </button>
                ))}
              </div>
            </div>

            {/* Chat Input Console */}
            <form
              onSubmit={handleSendMessage}
              className="p-3 border-t border-stone-800 bg-stone-950 flex items-center gap-2"
            >
              <div className="relative flex-1">
                <input
                  type="text"
                  value={inputPrompt}
                  onChange={(e) => setInputPrompt(e.target.value)}
                  placeholder="Ketik pertanyaan investigasi atau analisis celah ke Mentor Zeta..."
                  className="w-full px-3.5 py-2.5 rounded-xl bg-stone-900/90 border border-stone-800 focus:border-teal-500/60 focus:outline-none text-xs sm:text-sm text-stone-200 placeholder:text-stone-500 font-sans transition-colors"
                />
              </div>

              <button
                type="submit"
                disabled={!inputPrompt.trim()}
                className="px-4 py-2.5 rounded-xl bg-teal-500 hover:bg-teal-400 disabled:bg-stone-800 disabled:text-stone-600 text-stone-950 font-mono font-bold text-xs transition-colors flex items-center gap-1.5 shadow-[0_0_15px_rgba(20,184,166,0.2)]"
                title="Kirim Pertanyaan (Enter)"
              >
                <Send className="w-4 h-4" />
                <span className="hidden sm:inline">KIRIM</span>
              </button>
            </form>
          </div>
        </div>
      </div>
    </div>
  )
}
