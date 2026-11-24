// ZITERA CYBER_LAB — Zitera AI Overview Component
// High-tech Google AI Overview layout engineered with Zitera dark cyber aesthetic.
// Masterclass terminal styling, mentor persona intelligence, and multi-tier payload gating.
// Zero emojis — Lucide icons and dark glassmorphic styling exclusively.

import { useState, useEffect, useMemo } from 'react'
import {
  Shield,
  Terminal,
  Sparkles,
  Lock,
  Check,
  ChevronRight,
  Zap,
  RefreshCw,
  AlertTriangle,
  Copy,
  Cpu,
  Activity,
  CornerDownRight,
  Loader2,
} from 'lucide-react'
import {
  getMentor,
  MENTOR_LIST,
  type MentorId,
  type MentorProfile,
} from './mentorRegistry'

export interface PayloadBlueprint {
  title: string
  payload: string
  description: string
  targetParam?: string
}

export interface ZiteraAiOverviewData {
  diagnosticSnapshot: string
  reconHypothesis: string
  socraticQuestion: string
  suggestedActions: string[]
  payloadBlueprint?: PayloadBlueprint
  isTypoCorrected: boolean
  correctedQuery?: string
  confidenceScore: number
  matchedTopic: string
}

export interface ZiteraAiOverviewProps {
  data?: ZiteraAiOverviewData | null
  currentTier?: 'free' | 'pro' | 'vip' | 'foundation' | 'enterprise' | string
  activeMentorId?: MentorId
  energy?: number
  maxEnergy?: number
  nextResetTimestamp?: number | null
  onAutoInject?: (payload: string, targetParam?: string) => void
  onOpenUpgradeModal?: () => void
  onMentorChange?: (mentorId: MentorId) => void
  onQueryChange?: (correctedQuery: string) => void
  isLoading?: boolean
  className?: string
}

export function ZiteraAiOverview({
  data,
  currentTier = 'free',
  activeMentorId = 'chiyo',
  energy = 5,
  maxEnergy = 5,
  nextResetTimestamp,
  onAutoInject,
  onOpenUpgradeModal,
  onMentorChange,
  onQueryChange,
  isLoading = false,
  className = '',
}: ZiteraAiOverviewProps) {
  // Normalize tier checking
  const isVip = currentTier === 'vip' || currentTier === 'enterprise'

  // Internal mentor override allowing interactive switching
  const [mentorOverride, setMentorOverride] = useState<MentorId | null>(null)
  const currentMentorId = mentorOverride ?? activeMentorId

  const mentor: MentorProfile = useMemo(() => {
    return getMentor(currentMentorId)
  }, [currentMentorId])

  // Copy payload state
  const [copied, setCopied] = useState<boolean>(false)

  // 12-Hour countdown timer
  const [secondsRemaining, setSecondsRemaining] = useState<number>(() => {
    if (nextResetTimestamp && nextResetTimestamp > Date.now()) {
      return Math.floor((nextResetTimestamp - Date.now()) / 1000)
    }
    const twelveHoursMs = 12 * 60 * 60 * 1000
    const now = Date.now()
    const remainder = twelveHoursMs - (now % twelveHoursMs)
    return Math.floor(remainder / 1000)
  })

  useEffect(() => {
    const timer = setInterval(() => {
      setSecondsRemaining((prev) => (prev > 0 ? prev - 1 : 12 * 3600))
    }, 1000)
    return () => clearInterval(timer)
  }, [])

  const formattedCountdown = useMemo(() => {
    const hours = Math.floor(secondsRemaining / 3600)
    const minutes = Math.floor((secondsRemaining % 3600) / 60)
    const seconds = secondsRemaining % 60
    return `${hours.toString().padStart(2, '0')}:${minutes
      .toString()
      .padStart(2, '0')}:${seconds.toString().padStart(2, '0')}`
  }, [secondsRemaining])

  const handleCopyPayload = (payload: string) => {
    if (!payload) return
    navigator.clipboard.writeText(payload)
    setCopied(true)
    setTimeout(() => setCopied(false), 2200)
  }

  const handleMentorSwitch = (id: MentorId) => {
    setMentorOverride(id)
    onMentorChange?.(id)
  }

  // Fallback data if data is empty but component is rendered
  const resolvedData: ZiteraAiOverviewData = useMemo(() => {
    if (data) return data
    return {
      diagnosticSnapshot:
        'Sistem siap menerima parameter uji. Jalankan observasi input dan header HTTP untuk memetakan respon target.',
      reconHypothesis:
        'Target berpotensi merefleksikan karakter tanpa sanitasi ketat pada parameter input form atau query string.',
      socraticQuestion:
        'Apa yang terjadi pada struktur query backend ketika karakter kutip tunggal diinjeksikan bersamaan dengan operator logika?',
      suggestedActions: [
        'Kirim karakter pembatas string pada parameter target',
        'Periksa kode status HTTP dan panjang respon (Content-Length)',
        'Bandingkan respon boolean true vs false pada form input',
      ],
      payloadBlueprint: {
        title: 'Universal SQLi String Breakout & Boolean Evaluation',
        payload: "' OR '1'='1' -- -",
        description:
          'Mengevaluasi klausa boolean selalu bernilai benar untuk mem-bypass autentikasi dasar.',
        targetParam: 'username',
      },
      isTypoCorrected: false,
      confidenceScore: 94.6,
      matchedTopic: 'VULNERABILITY RECONNAISSANCE',
    }
  }, [data])

  return (
    <div
      className={`relative w-full rounded-xl bg-[#090D16] border border-[#1E293B] shadow-[0_12px_40px_rgba(0,0,0,0.65)] overflow-hidden font-sans transition-all duration-300 ${className}`}
    >
      {/* Top Google AI Overview Gradient Accent Strip */}
      <div className="h-1.5 w-full bg-gradient-to-r from-[#10B981] via-[#06B6D4] to-[#8B5CF6]" />

      {/* Cyber Ambient Background Glows */}
      <div
        className="absolute -top-24 -right-24 w-80 h-80 rounded-full bg-[#06B6D4]/10 blur-3xl pointer-events-none"
        aria-hidden="true"
      />
      <div
        className="absolute -bottom-24 -left-24 w-80 h-80 rounded-full bg-[#8B5CF6]/10 blur-3xl pointer-events-none"
        aria-hidden="true"
      />

      {/* Loading Overlay */}
      {isLoading && (
        <div className="absolute inset-0 z-30 flex flex-col items-center justify-center bg-[#090D16]/90 backdrop-blur-sm gap-3">
          <Loader2 size={32} className="animate-spin text-[#06B6D4]" />
          <span className="text-xs font-mono text-cyan-300 tracking-wider">
            [ MENGANALISIS TELEMETRI TARGET &bull; MENGHASILKAN AI OVERVIEW... ]
          </span>
        </div>
      )}

      <div className="relative p-5 sm:p-6 space-y-5">
        {/* ============================================================== */}
        {/* HEADER: Title, Confidence, Mentor Portrait, Typo Banner        */}
        {/* ============================================================== */}
        <div className="space-y-4">
          <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-3 pb-4 border-b border-[#1E293B]">
            {/* Title & Topic Pill */}
            <div className="flex flex-wrap items-center gap-2.5">
              <div className="flex items-center gap-2 px-3 py-1 rounded-md bg-[#161F30] border border-[#1E293B] text-cyan-400 font-mono text-xs font-semibold tracking-wider shadow-inner">
                <Sparkles size={14} className="text-[#06B6D4] animate-pulse" />
                <span>[ ZITERA AI OVERVIEW ]</span>
              </div>
              <span className="px-2.5 py-0.5 rounded-full bg-[#8B5CF6]/15 border border-[#8B5CF6]/30 text-[#8B5CF6] font-mono text-[11px] font-medium tracking-wide">
                {resolvedData.matchedTopic}
              </span>
            </div>

            {/* Confidence Score Badge */}
            <div className="flex items-center gap-2 px-3 py-1 rounded-md bg-[#161F30] border border-[#1E293B] text-slate-300 font-mono text-xs">
              <Activity
                size={14}
                className={
                  resolvedData.confidenceScore >= 85
                    ? 'text-[#10B981]'
                    : resolvedData.confidenceScore >= 70
                    ? 'text-[#06B6D4]'
                    : 'text-amber-400'
                }
              />
              <span className="text-slate-400">INTEL CONFIDENCE:</span>
              <span
                className={`font-bold ${
                  resolvedData.confidenceScore >= 85
                    ? 'text-[#10B981]'
                    : resolvedData.confidenceScore >= 70
                    ? 'text-[#06B6D4]'
                    : 'text-amber-400'
                }`}
              >
                {resolvedData.confidenceScore.toFixed(1)}%
              </span>
            </div>
          </div>

          {/* Mentor Profile Bar & Selector */}
          <div className="flex flex-col md:flex-row md:items-center justify-between gap-3 p-3 rounded-lg bg-[#161F30]/70 border border-[#1E293B]">
            <div className="flex items-center gap-3">
              {/* Mentor Portrait Avatar */}
              <div className="relative">
                <div
                  className={`w-11 h-11 rounded-lg overflow-hidden border-2 flex items-center justify-center bg-[#090D16] ${mentor.accentBorder} ${mentor.accentGlow}`}
                >
                  <img
                    src={mentor.avatar}
                    alt={mentor.name}
                    className="w-full h-full object-cover"
                    onError={(e) => {
                      // Fallback if avatar image is missing
                      ;(e.target as HTMLElement).style.display = 'none'
                    }}
                  />
                  <Cpu
                    size={20}
                    className="text-cyan-400 absolute inset-auto -z-10"
                  />
                </div>
                <div className="absolute -bottom-1 -right-1 w-3.5 h-3.5 rounded-full bg-[#10B981] border-2 border-[#090D16]" />
              </div>

              {/* Mentor Codename & Role */}
              <div>
                <div className="flex items-center gap-2">
                  <span className="font-bold text-sm text-slate-100 font-mono">
                    {mentor.name}
                  </span>
                  <span
                    className={`text-[10px] font-mono px-1.5 py-0.5 rounded border ${mentor.badgeBg} ${mentor.badgeText}`}
                  >
                    [{mentor.codename}]
                  </span>
                </div>
                <p className="text-xs text-slate-400 font-sans mt-0.5 line-clamp-1">
                  {mentor.role} &bull; {mentor.specialty}
                </p>
              </div>
            </div>

            {/* Interactive Mentor Selector Pills */}
            <div className="flex items-center gap-1.5 self-start md:self-auto bg-[#090D16]/80 p-1 rounded-lg border border-[#1E293B]">
              <span className="text-[10px] font-mono text-slate-500 uppercase px-1.5">
                Mentor:
              </span>
              {MENTOR_LIST.map((m) => {
                const isActive = m.id === mentor.id
                return (
                  <button
                    key={m.id}
                    onClick={() => handleMentorSwitch(m.id)}
                    className={`px-2 py-0.5 rounded text-xs font-mono font-medium transition-all cursor-pointer ${
                      isActive
                        ? 'bg-[#161F30] text-cyan-300 border border-cyan-500/40 shadow-[0_0_10px_rgba(6,182,212,0.2)]'
                        : 'text-slate-400 hover:text-slate-200 hover:bg-[#161F30]/50 border border-transparent'
                    }`}
                  >
                    {m.name}
                  </button>
                )
              })}
            </div>
          </div>

          {/* Typo Correction Banner */}
          {resolvedData.isTypoCorrected && resolvedData.correctedQuery && (
            <div className="flex items-center justify-between gap-3 p-3 rounded-lg bg-[#06B6D4]/10 border border-[#06B6D4]/30 text-xs">
              <div className="flex items-center gap-2 text-cyan-300 font-sans">
                <AlertTriangle size={15} className="text-[#06B6D4] shrink-0" />
                <span>
                  Maksud Anda:{' '}
                  <span className="font-mono font-bold text-white bg-[#090D16] px-2 py-0.5 rounded border border-[#06B6D4]/40">
                    "{resolvedData.correctedQuery}"
                  </span>
                </span>
              </div>
              {onQueryChange && (
                <button
                  onClick={() => onQueryChange(resolvedData.correctedQuery!)}
                  className="px-2.5 py-1 rounded bg-[#06B6D4] hover:bg-[#0891B2] text-black font-mono font-bold text-[11px] flex items-center gap-1 transition-colors cursor-pointer"
                >
                  <RefreshCw size={12} />
                  <span>Terapkan</span>
                </button>
              )}
            </div>
          )}
        </div>

        {/* ============================================================== */}
        {/* SECTION 1: Diagnostic Snapshot Card                            */}
        {/* ============================================================== */}
        <div className="rounded-lg bg-[#161F30]/60 border border-[#1E293B] p-4 space-y-2 hover:border-[#10B981]/40 transition-colors">
          <div className="flex items-center gap-2 text-xs font-mono font-semibold tracking-wide text-[#10B981]">
            <Terminal size={14} />
            <span>[ 01 // SNAPSHOT DIAGNOSTIK ]</span>
          </div>
          <p className="text-sm text-slate-200 leading-relaxed font-sans pl-1">
            {resolvedData.diagnosticSnapshot}
          </p>
        </div>

        {/* ============================================================== */}
        {/* SECTION 2: Reconnaissance Hypothesis Card                      */}
        {/* ============================================================== */}
        <div className="rounded-lg bg-[#161F30]/60 border border-[#1E293B] p-4 space-y-2 hover:border-[#8B5CF6]/40 transition-colors">
          <div className="flex items-center gap-2 text-xs font-mono font-semibold tracking-wide text-[#8B5CF6]">
            <Shield size={14} />
            <span>[ 02 // HIPOTESIS PENGINTAIAN ]</span>
          </div>
          <p className="text-sm text-slate-300 leading-relaxed font-sans pl-1">
            {resolvedData.reconHypothesis}
          </p>
        </div>

        {/* ============================================================== */}
        {/* SECTION 3: Socratic Probe Card                                 */}
        {/* ============================================================== */}
        <div className="rounded-lg bg-[#161F30]/60 border border-[#1E293B] p-4 space-y-2 hover:border-[#06B6D4]/40 transition-colors">
          <div className="flex items-center gap-2 text-xs font-mono font-semibold tracking-wide text-[#06B6D4]">
            <Sparkles size={14} />
            <span>[ 03 // SOCRATIC PROBE ]</span>
          </div>
          <div className="border-l-3 border-[#06B6D4] bg-[#06B6D4]/5 pl-4 py-2.5 rounded-r-md">
            <p className="text-sm text-cyan-200 font-mono italic leading-relaxed">
              "{resolvedData.socraticQuestion}"
            </p>
          </div>
        </div>

        {/* ============================================================== */}
        {/* SECTION 4: Tactical Action & Payload Blueprint                */}
        {/* ============================================================== */}
        <div className="rounded-lg bg-[#161F30]/80 border border-[#1E293B] p-4 space-y-4">
          <div className="flex items-center justify-between">
            <div className="flex items-center gap-2 text-xs font-mono font-semibold tracking-wide text-amber-400">
              <Zap size={14} />
              <span>[ 04 // TACTICAL ACTION & PAYLOAD BLUEPRINT ]</span>
            </div>
            {isVip ? (
              <span className="flex items-center gap-1 text-[11px] font-mono px-2 py-0.5 rounded bg-[#10B981]/15 border border-[#10B981]/40 text-[#10B981]">
                <Check size={12} />
                <span>VIP UNLOCKED</span>
              </span>
            ) : (
              <span className="flex items-center gap-1 text-[11px] font-mono px-2 py-0.5 rounded bg-[#8B5CF6]/15 border border-[#8B5CF6]/40 text-[#8B5CF6]">
                <Lock size={12} />
                <span>VIP EXCLUSIVE</span>
              </span>
            )}
          </div>

          {/* Suggested Actions Checklist */}
          {resolvedData.suggestedActions && resolvedData.suggestedActions.length > 0 && (
            <div className="space-y-1.5 pl-1">
              <div className="text-[11px] font-mono text-slate-400 uppercase tracking-wider mb-1">
                Langkah Taktis Terarah:
              </div>
              {resolvedData.suggestedActions.map((action, idx) => (
                <div
                  key={idx}
                  className="flex items-start gap-2 text-xs text-slate-300 font-sans"
                >
                  <ChevronRight
                    size={13}
                    className="text-[#06B6D4] mt-0.5 shrink-0"
                  />
                  <span>{action}</span>
                </div>
              ))}
            </div>
          )}

          {/* Payload Blueprint Container */}
          {resolvedData.payloadBlueprint && (
            <div className="space-y-2 pt-2 border-t border-[#1E293B]">
              <div className="flex items-center justify-between text-xs">
                <span className="font-mono text-slate-300 font-semibold">
                  {resolvedData.payloadBlueprint.title}
                </span>
                {resolvedData.payloadBlueprint.targetParam && (
                  <span className="text-[11px] font-mono text-slate-400 bg-[#090D16] px-2 py-0.5 rounded border border-[#1E293B]">
                    Param:{' '}
                    <span className="text-cyan-300">
                      {resolvedData.payloadBlueprint.targetParam}
                    </span>
                  </span>
                )}
              </div>
              <p className="text-xs text-slate-400 font-sans">
                {resolvedData.payloadBlueprint.description}
              </p>

              {/* TIER GATED PAYLOAD BOX */}
              {isVip ? (
                // VIP UNLOCKED VIEW
                <div className="space-y-3 pt-1">
                  <div className="relative group rounded-lg bg-[#090D16] border border-[#10B981]/50 p-3 shadow-inner">
                    <pre className="font-mono text-xs text-[#10B981] overflow-x-auto whitespace-pre-wrap select-all">
                      <code>{resolvedData.payloadBlueprint.payload}</code>
                    </pre>
                  </div>

                  <div className="flex flex-wrap items-center gap-2.5">
                    <button
                      onClick={() =>
                        handleCopyPayload(
                          resolvedData.payloadBlueprint!.payload
                        )
                      }
                      className="px-3 py-1.5 rounded-lg bg-[#161F30] hover:bg-[#1E293B] border border-[#1E293B] text-slate-200 text-xs font-mono font-medium flex items-center gap-1.5 transition-colors cursor-pointer"
                    >
                      {copied ? (
                        <>
                          <Check size={13} className="text-[#10B981]" />
                          <span className="text-[#10B981]">Tersalin!</span>
                        </>
                      ) : (
                        <>
                          <Copy size={13} />
                          <span>Salin Cetak Biru</span>
                        </>
                      )}
                    </button>

                    <button
                      onClick={() =>
                        onAutoInject?.(
                          resolvedData.payloadBlueprint!.payload,
                          resolvedData.payloadBlueprint!.targetParam
                        )
                      }
                      className="px-3.5 py-1.5 rounded-lg bg-gradient-to-r from-[#0F766E] to-[#06B6D4] hover:from-[#115E59] hover:to-[#0891B2] text-white text-xs font-mono font-bold flex items-center gap-1.5 shadow-[0_2px_12px_rgba(6,182,212,0.3)] hover:shadow-[0_2px_18px_rgba(6,182,212,0.5)] transition-all cursor-pointer"
                    >
                      <Zap size={13} className="text-cyan-200" />
                      <span>Kirim ke Burp Lite Repeater</span>
                    </button>
                  </div>
                </div>
              ) : (
                // NON-VIP GATED VIEW (CLEAN CAREER ASSET TEASER - NO BLURRED CHEAT)
                <div className="rounded-xl border border-purple-500/30 bg-[#090D16]/90 p-4 space-y-3">
                  <div className="flex items-center justify-between">
                    <div className="flex items-center gap-2 text-xs font-mono font-bold text-purple-300">
                      <Lock size={14} className="text-purple-400" />
                      <span>Aset Masterclass Karir & Bedah Kode (Khusus VIP)</span>
                    </div>
                    <span className="px-2 py-0.5 rounded text-[10px] font-mono bg-purple-950 text-purple-300 border border-purple-800/60 font-semibold">
                      TIER VIP
                    </span>
                  </div>

                  <p className="text-xs text-slate-300 font-sans leading-relaxed">
                    Buka bedah kode sumber backend asli (White-Box), pola perbaikan aman standar DevSecOps ISO 27001, serta draf laporan Bug Bounty HackerOne siap pakai.
                  </p>

                  <div className="pt-1">
                    <button
                      onClick={onOpenUpgradeModal}
                      className="w-full sm:w-auto px-4 py-2 rounded-lg bg-gradient-to-r from-[#8B5CF6] via-[#06B6D4] to-[#10B981] text-white text-xs font-mono font-bold flex items-center justify-center gap-2 shadow-[0_4px_20px_rgba(139,92,246,0.35)] hover:shadow-[0_4px_28px_rgba(139,92,246,0.55)] hover:scale-[1.01] active:scale-[0.99] transition-all cursor-pointer"
                    >
                      <Lock size={13} />
                      <span>[ Buka White-Box Code & Remediasi dengan VIP ]</span>
                      <CornerDownRight size={13} />
                    </button>
                  </div>
                </div>
              )}
            </div>
          )}
        </div>

        {/* ============================================================== */}
        {/* ENERGY METER & 12H RESET TIMER                                 */}
        {/* ============================================================== */}
        <div className="pt-3 border-t border-[#1E293B] flex flex-col sm:flex-row sm:items-center justify-between gap-3 text-xs">
          {/* Energy Level Bar */}
          <div className="flex items-center gap-3">
            <div className="flex items-center gap-1.5 font-mono text-slate-300 font-medium">
              <Zap size={14} className="text-amber-400 shrink-0" />
              <span>[ Energi Mentor: {energy}/{maxEnergy} ]</span>
            </div>

            {/* Battery Cells Visual */}
            <div className="flex items-center gap-1 bg-[#161F30] p-1 rounded border border-[#1E293B]">
              {Array.from({ length: maxEnergy }).map((_, i) => (
                <div
                  key={i}
                  className={`w-3.5 h-2 rounded-xs transition-all ${
                    i < energy
                      ? 'bg-[#10B981] shadow-[0_0_8px_rgba(16,185,129,0.7)]'
                      : 'bg-[#1E293B]'
                  }`}
                />
              ))}
            </div>
          </div>

          {/* 12h Timer & Overclock Action */}
          <div className="flex items-center gap-3">
            <div className="text-[11px] font-mono text-slate-400">
              Reset Siklus 12J:{' '}
              <span className="text-cyan-300 font-bold tracking-wider">
                {formattedCountdown}
              </span>
            </div>

            <button
              onClick={onOpenUpgradeModal}
              className="px-2.5 py-1 rounded bg-[#161F30] hover:bg-[#1E293B] border border-amber-500/40 hover:border-amber-500 text-amber-300 text-[11px] font-mono font-semibold flex items-center gap-1 transition-colors cursor-pointer"
            >
              <Zap size={11} className="text-amber-400" />
              <span>Overclock Instan</span>
            </button>
          </div>
        </div>
      </div>
    </div>
  )
}

export default ZiteraAiOverview
