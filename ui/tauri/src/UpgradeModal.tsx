// ZITERA CYBER_LAB — High-Conversion 3-Tier Monthly Pricing Modal
// Implements behavioral economics tripwire (<100k) & uniform increment ladder (+200k).
// All tiers are purely monthly and aligned directly with user account tier permissions.
// Tiers: Foundation (Rp 70k/bln) -> PRO Specialist (Rp 270k/bln) -> Enterprise VIP (Rp 470k/bln).
// Dark glassmorphism, responsive cyber aesthetic (#090D16, #161F30, #1E293B, #10B981, #06B6D4, #8B5CF6).
// Zero emojis — Lucide icons and masterclass vector styling exclusively.

import { useEffect } from 'react'
import {
  X,
  Shield,
  Zap,
  Sparkles,
  Lock,
  Check,
  ChevronRight,
  ArrowRight,
  Award,
  Cpu,
  Clock,
  CheckCircle2,
  FileCode,
  FileText,
} from 'lucide-react'
import { openBrowser } from './engineClient'

export type LicenseTierChoice = 'foundation' | 'pro' | 'vip'

export interface UpgradeModalProps {
  isOpen: boolean
  onClose: () => void
  currentTier?: 'free' | 'foundation' | 'pro' | 'vip' | 'enterprise' | string
  onSelectTier?: (tier: LicenseTierChoice) => void
  targetFeature?: string
  username?: string
}

export function UpgradeModal({
  isOpen,
  onClose,
  currentTier = 'free',
  onSelectTier,
  targetFeature,
  username = 'operative',
}: UpgradeModalProps) {
  // Close on Escape key
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.key === 'Escape') onClose()
    }
    if (isOpen) {
      window.addEventListener('keydown', handleKeyDown)
    }
    return () => window.removeEventListener('keydown', handleKeyDown)
  }, [isOpen, onClose])

  if (!isOpen) return null

  const isCurrentFoundation = currentTier === 'foundation'
  const isCurrentPro = currentTier === 'pro'
  const isCurrentVip = currentTier === 'vip' || currentTier === 'enterprise'

  const handleCheckout = (tier: LicenseTierChoice) => {
    if (onSelectTier) {
      onSelectTier(tier)
      return
    }

    // Default high-converting WhatsApp concierge checkout
    let tierName = ''
    if (tier === 'foundation') {
      tierName = 'Foundation Edition (Rp 70.000 / bln)'
    } else if (tier === 'vip') {
      tierName = 'Enterprise VIP (Rp 470.000 / bln)'
    } else {
      if (isCurrentFoundation) {
        tierName = 'PRO Specialist (Upgrade Credit Roll-Over: Rp 200.000)'
      } else {
        tierName = 'PRO Specialist (Rp 270.000 / bln)'
      }
    }

    const contextFeature = targetFeature ? ` [Trigger: ${targetFeature}]` : ''
    const msg = encodeURIComponent(
      `Halo Admin Zitera Lab, saya ingin aktivasi lisensi ${tierName} untuk username: ${username}${contextFeature}. Mohon kirim instruksi QRIS/Virtual Account.`
    )
    openBrowser(`https://wa.me/6282283088667?text=${msg}`)
    onClose()
  }

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center p-3 sm:p-5 overflow-y-auto bg-black/85 backdrop-blur-md animate-in fade-in duration-200">
      {/* Background Cyber Ambient Lights */}
      <div
        className="fixed top-1/4 left-1/4 w-96 h-96 rounded-full bg-[#06B6D4]/10 blur-[120px] pointer-events-none"
        aria-hidden="true"
      />
      <div
        className="fixed bottom-1/4 right-1/4 w-96 h-96 rounded-full bg-[#8B5CF6]/10 blur-[120px] pointer-events-none"
        aria-hidden="true"
      />

      {/* Modal Dialog Card */}
      <div
        className="relative w-full max-w-5xl rounded-2xl bg-[#090D16] border border-[#1E293B] shadow-[0_20px_60px_rgba(0,0,0,0.85)] overflow-hidden font-sans my-auto"
        onClick={(e) => e.stopPropagation()}
      >
        {/* Top Gradient Banner */}
        <div className="h-1.5 w-full bg-gradient-to-r from-[#10B981] via-[#06B6D4] to-[#8B5CF6]" />

        {/* Close Button */}
        <button
          onClick={onClose}
          className="absolute top-4 right-4 z-20 p-2 rounded-lg bg-[#161F30]/80 hover:bg-[#1E293B] text-slate-400 hover:text-white border border-[#1E293B] transition-colors cursor-pointer"
          title="Tutup Modal"
        >
          <X size={18} />
        </button>

        <div className="p-6 sm:p-8 space-y-6">
          {/* Header Title & Subtitle */}
          <div className="text-center space-y-2 max-w-2xl mx-auto">
            <div className="inline-flex items-center gap-2 px-3 py-1 rounded-full bg-[#161F30] border border-[#1E293B] text-cyan-400 font-mono text-xs font-semibold">
              <Shield size={13} className="text-[#06B6D4]" />
              <span>[ ZITERA SECURITY SUITE &bull; TIER ACCESS UPGRADE ]</span>
            </div>

            <h2 className="text-2xl sm:text-3xl font-extrabold text-white font-mono tracking-tight">
              Buka Potensi Penuh Riset Siber
            </h2>
            <p className="text-xs sm:text-sm text-slate-400 font-sans leading-relaxed">
              Pilih paket all-in-one bulanan sesuai target belajar Anda. Seluruh materi lab, petunjuk bertahap, dan bimbingan taktis mentor otomatis terbuka tanpa biaya tambahan.
            </p>

            {/* Target Feature Context Notice */}
            {targetFeature && (
              <div className="inline-flex items-center gap-2 px-3 py-1.5 rounded-lg bg-amber-500/10 border border-amber-500/30 text-amber-300 font-mono text-xs">
                <Zap size={14} className="text-amber-400 shrink-0" />
                <span>
                  Fitur Terpilih:{' '}
                  <span className="font-bold text-white">{targetFeature}</span>
                </span>
              </div>
            )}
          </div>

          {/* ============================================================== */}
          {/* 3-TIER PRICING GRID (FOUNDATION 70K -> PRO 270K -> VIP 470K)   */}
          {/* ============================================================== */}
          <div className="grid grid-cols-1 md:grid-cols-3 gap-5 items-stretch pt-2">
            {/* ------------------------------------------------------------ */}
            {/* TIER 1: FOUNDATION (Entry Tripwire - Rp 70.000 / bln)        */}
            {/* ------------------------------------------------------------ */}
            <div className="relative flex flex-col justify-between rounded-xl bg-[#161F30]/60 border border-[#1E293B] p-5 hover:border-amber-500/40 transition-all">
              <div className="space-y-4">
                <div className="space-y-1">
                  <div className="flex items-center justify-between">
                    <span className="text-[11px] font-mono uppercase tracking-wider text-amber-400 font-semibold">
                      [ TIER 01 &bull; PEMULA ]
                    </span>
                    <span className="px-2 py-0.5 rounded text-[10px] font-mono bg-amber-950/60 text-amber-300 border border-amber-800/50">
                      FONDASI
                    </span>
                  </div>
                  <h3 className="text-lg font-bold text-slate-100 font-mono">
                    Foundation Edition
                  </h3>
                  <p className="text-xs text-slate-400 font-sans">
                    Akses fundamental protokol HTTP, sesi, dan 2 lab OWASP nyata untuk pemula.
                  </p>
                </div>

                <div className="pt-2 pb-1 border-y border-[#1E293B]">
                  <div className="flex items-baseline gap-1">
                    <span className="text-2xl font-extrabold text-white font-mono">
                      Rp 70.000
                    </span>
                    <span className="text-xs text-slate-400 font-sans">
                      / bulan
                    </span>
                  </div>
                  <div className="text-[11px] font-mono text-emerald-400 mt-0.5">
                    Paket pemula hemat di bawah Rp 100k
                  </div>
                </div>

                {/* Features List */}
                <ul className="space-y-2.5 text-xs text-slate-300 font-sans">
                  <li className="flex items-start gap-2">
                    <Check size={14} className="text-[#10B981] mt-0.5 shrink-0" />
                    <span>
                      <strong className="text-white">8 Lab Lengkap</strong> (F01-F06 + A01 IDOR + A02 Misconfig)
                    </span>
                  </li>
                  <li className="flex items-start gap-2">
                    <Check size={14} className="text-[#10B981] mt-0.5 shrink-0" />
                    <span>Clue Level 1 & Level 2 penuh (8 lab terbuka)</span>
                  </li>
                  <li className="flex items-start gap-2">
                    <Check size={14} className="text-[#10B981] mt-0.5 shrink-0" />
                    <span>Chat Pertanyaan Taktis interaktif penuh</span>
                  </li>
                  <li className="flex items-start gap-2">
                    <Check size={14} className="text-[#10B981] mt-0.5 shrink-0" />
                    <span>Burp View Log & Header Inspector pasif</span>
                  </li>
                  <li className="flex items-start gap-2">
                    <Check size={14} className="text-[#10B981] mt-0.5 shrink-0" />
                    <span>Badge Digital: [ Junior Operative &bull; Verified ]</span>
                  </li>

                  {/* Locked items */}
                  <li className="flex items-start gap-2 text-slate-500 opacity-70">
                    <Lock size={13} className="text-slate-600 mt-0.5 shrink-0" />
                    <span>Track 2 OWASP Top 10 Terkunci (Butuh PRO)</span>
                  </li>
                  <li className="flex items-start gap-2 text-slate-500 opacity-70">
                    <Lock size={13} className="text-slate-600 mt-0.5 shrink-0" />
                    <span>Burp Lite Repeater Aktif Terkunci (Butuh PRO)</span>
                  </li>
                </ul>
              </div>

              {/* Action Button */}
              <div className="pt-6">
                {isCurrentFoundation ? (
                  <button
                    disabled
                    className="w-full py-2.5 px-4 rounded-lg bg-[#090D16] border border-[#1E293B] text-slate-500 text-xs font-mono font-semibold text-center cursor-not-allowed"
                  >
                    [ Tier Fondasi Aktif ]
                  </button>
                ) : (
                  <button
                    onClick={() => handleCheckout('foundation')}
                    className="w-full py-2.5 px-4 rounded-lg bg-[#161F30] hover:bg-[#1E293B] border border-amber-500/50 hover:border-amber-400 text-amber-200 text-xs font-mono font-bold flex items-center justify-center gap-1.5 transition-all cursor-pointer"
                  >
                    <span>[ Pilih Foundation (Rp 70k) ]</span>
                    <ChevronRight size={13} />
                  </button>
                )}
              </div>
            </div>

            {/* ------------------------------------------------------------ */}
            {/* TIER 2: PRO (Specialist - Rp 270.000 / bln) [BEST SELLER]    */}
            {/* ------------------------------------------------------------ */}
            <div className="relative flex flex-col justify-between rounded-xl bg-[#161F30] border-2 border-[#06B6D4] p-5 shadow-[0_0_35px_rgba(6,182,212,0.25)] hover:shadow-[0_0_45px_rgba(6,182,212,0.4)] transition-all">
              {/* Highlight Badge */}
              <div className="absolute -top-3 left-1/2 -translate-x-1/2 px-3 py-0.5 rounded-full bg-gradient-to-r from-[#10B981] via-[#06B6D4] to-[#8B5CF6] text-black font-mono text-[10px] font-extrabold uppercase tracking-widest shadow-md flex items-center gap-1">
                <Sparkles size={11} className="text-black" />
                <span>PALING POPULER // BEST SELLER</span>
              </div>

              <div className="space-y-4 pt-1">
                <div className="space-y-1">
                  <div className="flex items-center justify-between">
                    <span className="text-[11px] font-mono uppercase tracking-wider text-cyan-300 font-semibold">
                      [ TIER 02 &bull; SPESIALIS ]
                    </span>
                    <span className="px-2 py-0.5 rounded text-[10px] font-mono bg-cyan-950 text-cyan-300 border border-cyan-500/60">
                      PRO SPESIALIS
                    </span>
                  </div>
                  <h3 className="text-lg font-bold text-white font-mono flex items-center gap-1.5">
                    <span>PRO Specialist</span>
                  </h3>
                  <p className="text-xs text-cyan-100/80 font-sans">
                    Arsenal penuh: seluruh 40+ lab OWASP Top 10, Burp Repeater, & mentor Aoi.
                  </p>
                </div>

                <div className="pt-2 pb-1 border-y border-[#1E293B]">
                  <div className="flex items-baseline gap-1">
                    <span className="text-3xl font-extrabold text-white font-mono">
                      Rp 270.000
                    </span>
                    <span className="text-xs text-cyan-300 font-sans">
                      / bulan
                    </span>
                  </div>
                  <div className="text-[11px] font-mono text-emerald-400 font-semibold mt-0.5">
                    {isCurrentFoundation
                      ? 'Upgrade Roll-Over: Cukup bayar selisih Rp 200.000!'
                      : 'Investasi Rp 9.000 / hari'}
                  </div>
                </div>

                {/* Features List */}
                <ul className="space-y-2.5 text-xs text-slate-100 font-sans">
                  <li className="flex items-start gap-2">
                    <CheckCircle2
                      size={14}
                      className="text-[#06B6D4] mt-0.5 shrink-0"
                    />
                    <span>
                      <strong className="text-white">Seluruh 40+ Lab Industri</strong> (Track 1 & 2 OWASP Top 10)
                    </span>
                  </li>
                  <li className="flex items-start gap-2">
                    <CheckCircle2
                      size={14}
                      className="text-[#06B6D4] mt-0.5 shrink-0"
                    />
                    <span>
                      <strong className="text-cyan-300">Burp Lite Repeater Terintegrasi</strong> (Modifikasi raw HTTP)
                    </span>
                  </li>
                  <li className="flex items-start gap-2">
                    <FileText
                      size={14}
                      className="text-[#06B6D4] mt-0.5 shrink-0"
                    />
                    <span>
                      <strong className="text-white">Modul PoC Standar Industri</strong> (cURL & Raw HTTP Capture)
                    </span>
                  </li>
                  <li className="flex items-start gap-2">
                    <CheckCircle2
                      size={14}
                      className="text-[#06B6D4] mt-0.5 shrink-0"
                    />
                    <span>Clue Level 1 & Level 2 di seluruh 40+ lab</span>
                  </li>
                  <li className="flex items-start gap-2">
                    <CheckCircle2
                      size={14}
                      className="text-[#06B6D4] mt-0.5 shrink-0"
                    />
                    <span>Chat Pertanyaan Taktis Lapis 1 & 2 lengkap</span>
                  </li>
                  <li className="flex items-start gap-2">
                    <CheckCircle2
                      size={14}
                      className="text-[#06B6D4] mt-0.5 shrink-0"
                    />
                    <span>Multi-Mentor: Bebas pilih Zeta atau Aoi</span>
                  </li>
                  <li className="flex items-start gap-2">
                    <CheckCircle2
                      size={14}
                      className="text-[#06B6D4] mt-0.5 shrink-0"
                    />
                    <span>Scoreboard Rank, Leaderboard & Streak Saver</span>
                  </li>
                </ul>
              </div>

              {/* Action Button */}
              <div className="pt-6">
                {isCurrentPro ? (
                  <button
                    disabled
                    className="w-full py-3 px-4 rounded-xl bg-[#090D16] border border-[#06B6D4]/50 text-cyan-400 text-xs font-mono font-bold text-center cursor-not-allowed"
                  >
                    [ Tier PRO Aktif ]
                  </button>
                ) : (
                  <button
                    onClick={() => handleCheckout('pro')}
                    className="w-full py-3 px-4 rounded-xl bg-gradient-to-r from-[#10B981] via-[#06B6D4] to-[#8B5CF6] hover:brightness-110 active:scale-[0.99] text-black text-xs font-mono font-extrabold flex items-center justify-center gap-2 shadow-[0_4px_24px_rgba(6,182,212,0.45)] transition-all cursor-pointer"
                  >
                    <Zap size={14} className="fill-black text-black" />
                    <span>
                      {isCurrentFoundation
                        ? '[ UPGRADE KE PRO &bull; TAMBAH RP 200K ]'
                        : '[ PILIH PRO SPECIALIST SEKARANG ]'}
                    </span>
                    <ArrowRight size={14} />
                  </button>
                )}
              </div>
            </div>

            {/* ------------------------------------------------------------ */}
            {/* TIER 3: VIP (Enterprise VIP - Rp 470.000 / bln) [ELITE]      */}
            {/* ------------------------------------------------------------ */}
            <div className="relative flex flex-col justify-between rounded-xl bg-[#161F30]/80 border border-purple-500/50 p-5 hover:border-purple-400 transition-all">
              <div className="space-y-4 pt-1">
                <div className="space-y-1">
                  <div className="flex items-center justify-between">
                    <span className="text-[11px] font-mono uppercase tracking-wider text-purple-300 font-semibold">
                      [ TIER 03 &bull; MAKSIMUM ]
                    </span>
                    <span className="px-2 py-0.5 rounded text-[10px] font-mono bg-purple-950 text-purple-300 border border-purple-500/60">
                      ELITE OPERATIVE
                    </span>
                  </div>
                  <h3 className="text-lg font-bold text-white font-mono flex items-center gap-1.5">
                    <span>Enterprise VIP</span>
                    <Award size={16} className="text-amber-400" />
                  </h3>
                  <p className="text-xs text-purple-200/80 font-sans">
                    Masterclass karir: bedah source code asli, remediasi aman, & sertifikasi.
                  </p>
                </div>

                <div className="pt-2 pb-1 border-y border-[#1E293B]">
                  <div className="flex items-baseline gap-1">
                    <span className="text-3xl font-extrabold text-white font-mono">
                      Rp 470.000
                    </span>
                    <span className="text-xs text-purple-300 font-sans">
                      / bulan
                    </span>
                  </div>
                  <div className="text-[11px] font-mono text-emerald-400 font-semibold mt-0.5">
                    Hanya selisih Rp 200k dari PRO!
                  </div>
                </div>

                {/* Features List */}
                <ul className="space-y-2.5 text-xs text-slate-100 font-sans">
                  <li className="flex items-start gap-2">
                    <CheckCircle2
                      size={14}
                      className="text-purple-400 mt-0.5 shrink-0"
                    />
                    <span>
                      <strong className="text-white">Seluruh 60+ Lab Lengkap</strong> (Track 3 Advanced Exploit Engineering)
                    </span>
                  </li>
                  <li className="flex items-start gap-2">
                    <Zap
                      size={14}
                      className="text-purple-400 mt-0.5 shrink-0"
                    />
                    <span>
                      <strong className="text-purple-300">Elite Bug Bounty Automated PoC Suite:</strong> Skrip Python/Go + Bimbingan AI
                    </span>
                  </li>
                  <li className="flex items-start gap-2">
                    <FileCode
                      size={14}
                      className="text-purple-400 mt-0.5 shrink-0"
                    />
                    <span>
                      <strong className="text-purple-300">Clue Level 3 Masterclass:</strong> White-Box Code Anatomy & DevSecOps Patch
                    </span>
                  </li>
                  <li className="flex items-start gap-2">
                    <FileText
                      size={14}
                      className="text-purple-400 mt-0.5 shrink-0"
                    />
                    <span>
                      <strong className="text-white">HackerOne Report Draft Generator</strong> (Triage-Ready Markdown)
                    </span>
                  </li>
                  <li className="flex items-start gap-2">
                    <Shield
                      size={14}
                      className="text-purple-400 mt-0.5 shrink-0"
                    />
                    <span>
                      <strong className="text-amber-300">Pedoman Etika & Safe Harbor</strong> (Aturan RoE Pentest Legal)
                    </span>
                  </li>
                  <li className="flex items-start gap-2">
                    <CheckCircle2
                      size={14}
                      className="text-purple-400 mt-0.5 shrink-0"
                    />
                    <span>
                      Mentor Senior Eksklusif <strong className="text-cyan-300">Elysia (Red-Team Lead)</strong>
                    </span>
                  </li>
                  <li className="flex items-start gap-2">
                    <CheckCircle2
                      size={14}
                      className="text-purple-400 mt-0.5 shrink-0"
                    />
                    <span>Jalur Konsultasi Prioritas / Concierge VIP</span>
                  </li>
                </ul>
              </div>

              {/* Action Button */}
              <div className="pt-6">
                {isCurrentVip ? (
                  <button
                    disabled
                    className="w-full py-2.5 px-4 rounded-lg bg-[#090D16] border border-purple-500/50 text-purple-400 text-xs font-mono font-bold text-center cursor-not-allowed"
                  >
                    [ Tier VIP Aktif ]
                  </button>
                ) : (
                  <button
                    onClick={() => handleCheckout('vip')}
                    className="w-full py-2.5 px-4 rounded-lg bg-[#161F30] hover:bg-[#1E293B] border border-purple-500/50 hover:border-purple-400 text-purple-200 text-xs font-mono font-bold flex items-center justify-center gap-1.5 transition-all cursor-pointer"
                  >
                    <span>[ Aktifkan Enterprise VIP ]</span>
                    <ChevronRight size={13} />
                  </button>
                )}
              </div>
            </div>
          </div>

          {/* Footer Security / Trust Info */}
          <div className="pt-4 border-t border-[#1E293B] flex flex-wrap items-center justify-between gap-4 text-[11px] font-mono text-slate-400">
            <div className="flex items-center gap-2">
              <Shield size={13} className="text-[#10B981]" />
              <span>Cryptographic Hardware-Enclave Bound License</span>
            </div>
            <div className="flex items-center gap-2">
              <Cpu size={13} className="text-[#06B6D4]" />
              <span>Aktivasi Instan Otomatis via QRIS / VA / Admin WhatsApp</span>
            </div>
            <div className="flex items-center gap-2">
              <Clock size={13} className="text-[#8B5CF6]" />
              <span>Garansi Kredit Roll-Over Antar Tingkatan</span>
            </div>
          </div>
        </div>
      </div>
    </div>
  )
}

export default UpgradeModal
