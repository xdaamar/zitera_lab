// ZITERA CYBER_LAB — High-Conversion 3-Tier Decoy Pricing Modal
// Implements behavioral economics decoy pricing to drive maximum conversion to VIP Elite Operative.
// Dark glassmorphism, responsive cyber aesthetic (#090D16, #161F30, #1E293B, #10B981, #06B6D4, #8B5CF6).
// Zero emojis — Lucide icons and masterclass vector styling exclusively.

import { useState, useEffect } from 'react'
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
} from 'lucide-react'
import { openBrowser } from './engineClient'

export type BillingCycle = 'monthly' | 'annual'

export interface UpgradeModalProps {
  isOpen: boolean
  onClose: () => void
  currentTier?: 'free' | 'pro' | 'vip' | 'foundation' | 'enterprise' | string
  onSelectTier?: (tier: 'pro' | 'vip', billingCycle: BillingCycle) => void
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
  const [billingCycle, setBillingCycle] = useState<BillingCycle>('monthly')

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

  const isCurrentPro = currentTier === 'pro'
  const isCurrentVip = currentTier === 'vip' || currentTier === 'enterprise'

  const handleCheckout = (tier: 'pro' | 'vip') => {
    if (onSelectTier) {
      onSelectTier(tier, billingCycle)
      return
    }

    // Default high-converting WhatsApp concierge checkout
    const tierName =
      tier === 'vip'
        ? 'VIP Elite Operative (Rp 89k/bln atau Rp 799k/thn)'
        : 'PRO Specialist (Rp 49k/bln)'
    const periodText = billingCycle === 'annual' ? 'Paket Tahunan' : 'Paket Bulanan'
    const contextFeature = targetFeature ? ` [Trigger: ${targetFeature}]` : ''
    const msg = encodeURIComponent(
      `Halo Admin Zitera Lab, saya ingin aktivasi lisensi ${tierName} - ${periodText} untuk username: ${username}${contextFeature}. Mohon kirim instruksi QRIS/Virtual Account.`
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
              Dapatkan akses tak terbatas ke Zitera AI Overview, blueprint payload eksploitasi, integrasi Burp Suite Repeater satu klik, dan mentor elit.
            </p>

            {/* Target Feature Context Notice */}
            {targetFeature && (
              <div className="inline-flex items-center gap-2 px-3 py-1.5 rounded-lg bg-amber-500/10 border border-amber-500/30 text-amber-300 font-mono text-xs">
                <Zap size={14} className="text-amber-400 shrink-0" />
                <span>
                  Fitur Terpilih:{' '}
                  <span className="font-bold text-white">{targetFeature}</span>{' '}
                  memerlukan lisensi VIP Operative.
                </span>
              </div>
            )}

            {/* Billing Cycle Selector */}
            <div className="pt-3 flex items-center justify-center">
              <div className="inline-flex items-center p-1 rounded-xl bg-[#161F30] border border-[#1E293B]">
                <button
                  onClick={() => setBillingCycle('monthly')}
                  className={`px-4 py-1.5 rounded-lg font-mono text-xs font-semibold transition-all cursor-pointer ${
                    billingCycle === 'monthly'
                      ? 'bg-[#090D16] text-white shadow-sm border border-[#1E293B]'
                      : 'text-slate-400 hover:text-slate-200'
                  }`}
                >
                  Bulanan
                </button>
                <button
                  onClick={() => setBillingCycle('annual')}
                  className={`flex items-center gap-1.5 px-4 py-1.5 rounded-lg font-mono text-xs font-semibold transition-all cursor-pointer ${
                    billingCycle === 'annual'
                      ? 'bg-gradient-to-r from-[#0F766E] to-[#06B6D4] text-white shadow-md'
                      : 'text-slate-400 hover:text-slate-200'
                  }`}
                >
                  <span>Tahunan</span>
                  <span className="px-1.5 py-0.2 rounded-full bg-[#10B981]/25 border border-[#10B981]/50 text-[#10B981] text-[10px] font-bold">
                    HEMAT 25%
                  </span>
                </button>
              </div>
            </div>
          </div>

          {/* ============================================================== */}
          {/* 3-TIER DECOY PRICING GRID                                      */}
          {/* ============================================================== */}
          <div className="grid grid-cols-1 md:grid-cols-3 gap-5 items-stretch pt-2">
            {/* ------------------------------------------------------------ */}
            {/* TIER 1: FREE (Recruit)                                       */}
            {/* ------------------------------------------------------------ */}
            <div className="relative flex flex-col justify-between rounded-xl bg-[#161F30]/50 border border-[#1E293B] p-5 hover:border-slate-700 transition-all">
              <div className="space-y-4">
                <div className="space-y-1">
                  <div className="flex items-center justify-between">
                    <span className="text-[11px] font-mono uppercase tracking-wider text-slate-400 font-semibold">
                      [ TIER 01 ]
                    </span>
                    <span className="px-2 py-0.5 rounded text-[10px] font-mono bg-slate-800 text-slate-400 border border-slate-700">
                      DASAR
                    </span>
                  </div>
                  <h3 className="text-lg font-bold text-slate-200 font-mono">
                    FREE (Recruit)
                  </h3>
                  <p className="text-xs text-slate-400 font-sans">
                    Akses fundamental untuk eksplorasi konsep awal keamanan web.
                  </p>
                </div>

                <div className="pt-2 pb-1 border-y border-[#1E293B]">
                  <div className="flex items-baseline gap-1">
                    <span className="text-2xl font-extrabold text-white font-mono">
                      Rp 0
                    </span>
                    <span className="text-xs text-slate-400 font-sans">
                      / selamanya
                    </span>
                  </div>
                  <div className="text-[11px] font-mono text-slate-500 mt-0.5">
                    Akses publik tanpa kartu kredit
                  </div>
                </div>

                {/* Features List */}
                <ul className="space-y-2.5 text-xs text-slate-300 font-sans">
                  <li className="flex items-start gap-2">
                    <Check size={14} className="text-[#10B981] mt-0.5 shrink-0" />
                    <span>
                      <strong className="text-white">5 AI Overview</strong> / hari
                      (siklus 12 jam)
                    </span>
                  </li>
                  <li className="flex items-start gap-2">
                    <Check size={14} className="text-[#10B981] mt-0.5 shrink-0" />
                    <span>Clue Level 1 Konsep Teori Dasar</span>
                  </li>
                  <li className="flex items-start gap-2">
                    <Check size={14} className="text-[#10B981] mt-0.5 shrink-0" />
                    <span>Burp View Log (Inspeksi HTTP pasif)</span>
                  </li>
                  <li className="flex items-start gap-2">
                    <Check size={14} className="text-[#10B981] mt-0.5 shrink-0" />
                    <span>Akses Lab Track 1 (Foundations F01-F06)</span>
                  </li>

                  {/* Locked items */}
                  <li className="flex items-start gap-2 text-slate-500 opacity-70">
                    <Lock size={13} className="text-slate-600 mt-0.5 shrink-0" />
                    <span>Payload Blueprint Terkunci (Level 2 & 3)</span>
                  </li>
                  <li className="flex items-start gap-2 text-slate-500 opacity-70">
                    <Lock size={13} className="text-slate-600 mt-0.5 shrink-0" />
                    <span>Tanpa Auto-Inject ke Burp Repeater</span>
                  </li>
                </ul>
              </div>

              {/* Action Button */}
              <div className="pt-6">
                <button
                  disabled
                  className="w-full py-2.5 px-4 rounded-lg bg-[#090D16] border border-[#1E293B] text-slate-500 text-xs font-mono font-semibold text-center cursor-not-allowed"
                >
                  [ Tier Aktif Saat Ini ]
                </button>
              </div>
            </div>

            {/* ------------------------------------------------------------ */}
            {/* TIER 2: PRO (Specialist) [The Decoy]                          */}
            {/* ------------------------------------------------------------ */}
            <div className="relative flex flex-col justify-between rounded-xl bg-[#161F30]/70 border border-[#1E293B] p-5 hover:border-purple-500/40 transition-all">
              <div className="space-y-4">
                <div className="space-y-1">
                  <div className="flex items-center justify-between">
                    <span className="text-[11px] font-mono uppercase tracking-wider text-purple-400 font-semibold">
                      [ TIER 02 &bull; DECOY ]
                    </span>
                    <span className="px-2 py-0.5 rounded text-[10px] font-mono bg-purple-950/60 text-purple-300 border border-purple-800/50">
                      SPESIALIS
                    </span>
                  </div>
                  <h3 className="text-lg font-bold text-slate-100 font-mono">
                    PRO (Specialist)
                  </h3>
                  <p className="text-xs text-slate-400 font-sans">
                    Vektor serangan analitis dan pengujian manual parameter HTTP.
                  </p>
                </div>

                <div className="pt-2 pb-1 border-y border-[#1E293B]">
                  <div className="flex items-baseline gap-1">
                    <span className="text-2xl font-extrabold text-white font-mono">
                      {billingCycle === 'monthly' ? 'Rp 49.000' : 'Rp 449.000'}
                    </span>
                    <span className="text-xs text-slate-400 font-sans">
                      {billingCycle === 'monthly' ? '/ bulan' : '/ tahun'}
                    </span>
                  </div>
                  <div className="text-[11px] font-mono text-slate-400 mt-0.5">
                    {billingCycle === 'annual'
                      ? 'Setara Rp 37.400 / bln'
                      : 'Tagihan bulanan fleksibel'}
                  </div>
                </div>

                {/* Features List */}
                <ul className="space-y-2.5 text-xs text-slate-300 font-sans">
                  <li className="flex items-start gap-2">
                    <Check size={14} className="text-[#10B981] mt-0.5 shrink-0" />
                    <span>
                      <strong className="text-white">50 AI Overview</strong> / hari
                    </span>
                  </li>
                  <li className="flex items-start gap-2">
                    <Check size={14} className="text-[#10B981] mt-0.5 shrink-0" />
                    <span>Clue Level 2 Attack Vector & Logika Kerentanan</span>
                  </li>
                  <li className="flex items-start gap-2">
                    <Check size={14} className="text-[#10B981] mt-0.5 shrink-0" />
                    <span>Burp Repeater Manual (Kirim & modifikasi request)</span>
                  </li>
                  <li className="flex items-start gap-2">
                    <Check size={14} className="text-[#10B981] mt-0.5 shrink-0" />
                    <span>Badge Spesialis Operatif & Scoreboard Tracking</span>
                  </li>
                  <li className="flex items-start gap-2">
                    <Check size={14} className="text-[#10B981] mt-0.5 shrink-0" />
                    <span>Akses 30 Lab OWASP Top 10 (Track 2)</span>
                  </li>

                  {/* Missing decoy features compared to VIP */}
                  <li className="flex items-start gap-2 text-slate-500 opacity-80">
                    <Lock size={13} className="text-amber-500 mt-0.5 shrink-0" />
                    <span className="text-slate-400">
                      Tanpa Auto-Inject Otomatis Burp
                    </span>
                  </li>
                  <li className="flex items-start gap-2 text-slate-500 opacity-80">
                    <Lock size={13} className="text-amber-500 mt-0.5 shrink-0" />
                    <span className="text-slate-400">
                      Tanpa Level 3 Payload Blueprint
                    </span>
                  </li>
                </ul>
              </div>

              {/* Action Button */}
              <div className="pt-6">
                {isCurrentPro ? (
                  <button
                    disabled
                    className="w-full py-2.5 px-4 rounded-lg bg-[#090D16] border border-[#1E293B] text-slate-500 text-xs font-mono font-semibold text-center cursor-not-allowed"
                  >
                    [ Tier Pro Aktif ]
                  </button>
                ) : (
                  <button
                    onClick={() => handleCheckout('pro')}
                    className="w-full py-2.5 px-4 rounded-lg bg-[#161F30] hover:bg-[#1E293B] border border-purple-500/50 hover:border-purple-400 text-purple-200 text-xs font-mono font-bold flex items-center justify-center gap-1.5 transition-all cursor-pointer"
                  >
                    <span>[ Pilih Pro Specialist ]</span>
                    <ChevronRight size={13} />
                  </button>
                )}
              </div>
            </div>

            {/* ------------------------------------------------------------ */}
            {/* TIER 3: VIP (Elite Operative) [THE WINNER / GLOWING BORDER]  */}
            {/* ------------------------------------------------------------ */}
            <div className="relative flex flex-col justify-between rounded-xl bg-[#161F30] border-2 border-[#06B6D4] p-5 shadow-[0_0_35px_rgba(6,182,212,0.25)] hover:shadow-[0_0_45px_rgba(6,182,212,0.4)] transition-all">
              {/* Highlight Badge */}
              <div className="absolute -top-3 left-1/2 -translate-x-1/2 px-3 py-0.5 rounded-full bg-gradient-to-r from-[#10B981] via-[#06B6D4] to-[#8B5CF6] text-black font-mono text-[10px] font-extrabold uppercase tracking-widest shadow-md flex items-center gap-1">
                <Sparkles size={11} className="text-black" />
                <span>PALING POPULER // NILAI TERBAIK</span>
              </div>

              <div className="space-y-4 pt-1">
                <div className="space-y-1">
                  <div className="flex items-center justify-between">
                    <span className="text-[11px] font-mono uppercase tracking-wider text-cyan-300 font-semibold">
                      [ TIER 03 &bull; MAKSIMUM ]
                    </span>
                    <span className="px-2 py-0.5 rounded text-[10px] font-mono bg-cyan-950 text-cyan-300 border border-cyan-500/60">
                      ELITE OPERATIVE
                    </span>
                  </div>
                  <h3 className="text-lg font-bold text-white font-mono flex items-center gap-1.5">
                    <span>VIP (Elite Operative)</span>
                    <Award size={16} className="text-amber-400" />
                  </h3>
                  <p className="text-xs text-cyan-100/80 font-sans">
                    Arsenal penuh: blueprint siap pakai, auto-inject repeater, & mentor Elysia.
                  </p>
                </div>

                <div className="pt-2 pb-1 border-y border-[#1E293B]">
                  <div className="flex items-baseline gap-1">
                    <span className="text-3xl font-extrabold text-white font-mono">
                      {billingCycle === 'monthly' ? 'Rp 89.000' : 'Rp 799.000'}
                    </span>
                    <span className="text-xs text-cyan-300 font-sans">
                      {billingCycle === 'monthly' ? '/ bulan' : '/ tahun'}
                    </span>
                  </div>
                  <div className="text-[11px] font-mono text-emerald-400 font-semibold mt-0.5">
                    {billingCycle === 'annual'
                      ? 'Hanya Rp 66.500 / bln (Hemat Rp 269.000!)'
                      : 'Hanya selisih Rp 40k dari Pro!'}
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
                      <strong className="text-cyan-300">
                        Unlimited Priority AI Overview
                      </strong>{' '}
                      (Tanpa batasan)
                    </span>
                  </li>
                  <li className="flex items-start gap-2">
                    <CheckCircle2
                      size={14}
                      className="text-[#06B6D4] mt-0.5 shrink-0"
                    />
                    <span>
                      <strong className="text-white">
                        Level 3 Payload Blueprint
                      </strong>{' '}
                      (Payload presisi siap pakai)
                    </span>
                  </li>
                  <li className="flex items-start gap-2">
                    <CheckCircle2
                      size={14}
                      className="text-[#06B6D4] mt-0.5 shrink-0"
                    />
                    <span>
                      <strong className="text-white">
                        One-Click Auto-Inject ke Burp Repeater
                      </strong>
                    </span>
                  </li>
                  <li className="flex items-start gap-2">
                    <CheckCircle2
                      size={14}
                      className="text-[#06B6D4] mt-0.5 shrink-0"
                    />
                    <span>
                      Mentor Eksklusif{' '}
                      <strong className="text-amber-300">
                        Elysia Red-Team & All Mentors
                      </strong>
                    </span>
                  </li>
                  <li className="flex items-start gap-2">
                    <CheckCircle2
                      size={14}
                      className="text-[#06B6D4] mt-0.5 shrink-0"
                    />
                    <span>
                      Instant Mentor Overclock (Bypass cooldown 12 jam)
                    </span>
                  </li>
                  <li className="flex items-start gap-2">
                    <CheckCircle2
                      size={14}
                      className="text-[#06B6D4] mt-0.5 shrink-0"
                    />
                    <span>
                      Sertifikat Kriptografis Resmi Anti-Pemalsuan
                    </span>
                  </li>
                  <li className="flex items-start gap-2">
                    <CheckCircle2
                      size={14}
                      className="text-[#06B6D4] mt-0.5 shrink-0"
                    />
                    <span>
                      Akses Seluruh 45 Lab Termasuk Track 3 Zero-Day
                    </span>
                  </li>
                </ul>
              </div>

              {/* Action Button */}
              <div className="pt-6">
                {isCurrentVip ? (
                  <button
                    disabled
                    className="w-full py-3 px-4 rounded-xl bg-[#090D16] border border-[#10B981]/50 text-[#10B981] text-xs font-mono font-bold text-center cursor-not-allowed"
                  >
                    [ Tier VIP Aktif ]
                  </button>
                ) : (
                  <button
                    onClick={() => handleCheckout('vip')}
                    className="w-full py-3 px-4 rounded-xl bg-gradient-to-r from-[#10B981] via-[#06B6D4] to-[#8B5CF6] hover:brightness-110 active:scale-[0.99] text-black text-xs font-mono font-extrabold flex items-center justify-center gap-2 shadow-[0_4px_24px_rgba(6,182,212,0.45)] transition-all cursor-pointer"
                  >
                    <Zap size={14} className="fill-black text-black" />
                    <span>[ AKTIFKAN AKSES VIP SEKARANG ]</span>
                    <ArrowRight size={14} />
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
              <span>Dukungan Teknis Komunitas & Tim Pengembang</span>
            </div>
          </div>
        </div>
      </div>
    </div>
  )
}

export default UpgradeModal
