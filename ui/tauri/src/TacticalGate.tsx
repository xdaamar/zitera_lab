import { useState, useEffect, type FormEvent } from 'react'
import {
  ShieldCheck,
  KeyRound,
  User,
  UserCircle2,
  Eye,
  EyeOff,
  MessageSquare,
  ExternalLink,
  Cpu,
  CheckCircle2,
  AlertCircle,
  Compass,
} from 'lucide-react'
import { login, getDeviceFingerprint } from './authService'

interface TacticalGateProps {
  onLoginSuccess: () => void
}

export function TacticalGate({ onLoginSuccess }: TacticalGateProps) {
  const [username, setUsername] = useState('')
  const [password, setPassword] = useState('')
  const [showPassword, setShowPassword] = useState(false)
  const [loading, setLoading] = useState(false)
  const [errorMsg, setErrorMsg] = useState<string | null>(null)
  const [successMsg, setSuccessMsg] = useState<string | null>(null)
  const [deviceId, setDeviceId] = useState<string>('DETECTING...')
  const [isFormVisible, setIsFormVisible] = useState(false)

  useEffect(() => {
    let mounted = true
    getDeviceFingerprint().then((fp) => {
      if (mounted) setDeviceId(fp)
    })
    return () => {
      mounted = false
    }
  }, [])

  const handleSubmit = async (e: FormEvent) => {
    e.preventDefault()
    if (!username.trim() || !password) {
      setErrorMsg('Username dan Password lisensi wajib diisi.')
      return
    }

    setLoading(true)
    setErrorMsg(null)
    setSuccessMsg(null)

    try {
      const res = await login(username, password)
      if (res.success) {
        setSuccessMsg(res.message || 'Otentikasi Berhasil. Membuka Ruang Operasi...')
        setTimeout(() => {
          onLoginSuccess()
        }, 600)
      } else {
        setErrorMsg(res.message)
      }
    } catch (err) {
      setErrorMsg(`Kesalahan jaringan/sistem: ${err}`)
    } finally {
      setLoading(false)
    }
  }

  const handleOpenWhatsApp = () => {
    const waUrl =
      'https://wa.me/?text=' +
      encodeURIComponent(
        'Halo Admin ZITERA LAB, saya ingin mendaftar dan membeli lisensi resmi ZITERA LAB Commercial Edition. Mohon informasi rekening dan kredensialnya.',
      )
    window.open(waUrl, '_blank')
  }

  return (
    <div className="fixed inset-0 z-[9999] flex items-center justify-between overflow-hidden bg-stone-950 text-white select-none">
      {/* 1. Raw HD Anime Video Background: Pure Unfiltered Clarity (Permanently Muted) */}
      <div className="absolute inset-0 overflow-hidden pointer-events-none">
        <video
          autoPlay
          loop
          muted
          playsInline
          className="absolute inset-0 w-full h-full object-cover select-none video-hd-crisp"
          src="/assets/gemini_generated_video_4357dadb.mp4"
        />
      </div>

      {/* 2. Top Navigation Bar: Minimal Brand Typography */}
      <header className="absolute top-0 left-0 right-0 z-20 flex items-center justify-between px-6 md:px-10 py-5">
        <div className="flex items-center gap-2.5">
          <span className="font-['Space_Grotesk'] font-black text-2xl tracking-wider bg-gradient-to-r from-white via-teal-200 to-cyan-300 bg-clip-text text-transparent drop-shadow-[0_2px_12px_rgba(45,212,191,0.4)]">
            ZITERA
          </span>
          <span className="px-2 py-0.5 rounded-full text-[10px] font-bold tracking-widest uppercase bg-white/10 text-teal-300 border border-white/20 backdrop-blur-md">
            ACADEMY
          </span>
        </div>
      </header>

      {/* 3. Left Hero Callout: Atmospheric Floating Typography (Non-Intrusive) */}
      <div className="relative z-10 hidden xl:flex flex-col justify-end p-12 max-w-xl pb-16 pointer-events-none">
        <div className="space-y-4">
          <div className="inline-flex items-center gap-2 px-3.5 py-1.5 rounded-full bg-white/10 backdrop-blur-xl border border-white/15 text-xs font-semibold text-teal-200 shadow-lg">
            <Compass size={14} className="text-teal-300 animate-spin" style={{ animationDuration: '12s' }} />
            <span>Puncak Petualangan Offensive Security</span>
          </div>
          <h1 className="text-5xl font-black font-['Space_Grotesk'] leading-[1.15] tracking-tight text-white drop-shadow-[0_4px_25px_rgba(0,0,0,0.8)]">
            Eksplorasi Dunia Cyber dengan Jiwa Sang Hunter.
          </h1>
          <p className="text-sm font-medium text-stone-200/90 leading-relaxed drop-shadow-[0_2px_10px_rgba(0,0,0,0.8)] max-w-md">
            Materi kurikulum OWASP Top 10:2025, simulasi bug bounty realistis, dan lab praktikum interaktif berkecepatan tinggi dalam satu ekosistem desktop.
          </p>
        </div>
      </div>

      {/* 4. Right Side: Transparent Dark Glass Login Card (Zero Blur - 100% Sharp Background Visibility) */}
      <div
        className={`relative z-20 w-full sm:w-[480px] p-4 sm:p-8 sm:pr-12 lg:pr-16 flex flex-col justify-center transition-all duration-500 ease-out ${
          isFormVisible
            ? 'translate-x-0 opacity-100 pointer-events-auto'
            : 'translate-x-24 opacity-0 pointer-events-none'
        }`}
      >
        <div className="relative w-full rounded-3xl bg-black/25 border border-white/15 p-6 sm:p-8 shadow-[0_20px_50px_rgba(0,0,0,0.5)] text-white overflow-hidden">
          {/* Top Edge Subtle Highlight */}
          <div className="absolute top-0 left-0 right-0 h-[1px] bg-gradient-to-r from-transparent via-white/30 to-transparent" />

          {/* Card Header & Security Badge */}
          <div className="space-y-3 mb-6">
            <div className="flex items-center justify-between">
              <div className="inline-flex items-center gap-1.5 px-3 py-1 rounded-full bg-teal-900/40 border border-teal-500/30">
                <span className="w-2 h-2 rounded-full bg-teal-300 animate-ping" />
                <span className="text-[10px] font-black font-['JetBrains_Mono'] tracking-wider text-teal-300 uppercase">
                  HARDWARE BOUND
                </span>
              </div>
              <button
                type="button"
                onClick={() => setIsFormVisible(false)}
                className="p-2 rounded-2xl bg-black/30 hover:bg-black/50 active:scale-95 text-teal-300 hover:text-white border border-white/10 transition-all cursor-pointer"
                title="Sembunyikan Form"
              >
                <EyeOff size={16} />
              </button>
            </div>

            <div>
              <h2 className="text-2xl font-black font-['Space_Grotesk'] tracking-tight text-white">
                Ruang Operasi Siswa
              </h2>
              <p className="text-xs text-white/80 mt-1 font-medium">
                Gunakan kredensial yang diberikan Administrator untuk membuka lisensi.
              </p>
            </div>
          </div>

          {/* Single-Machine Hardware Enclave Fingerprint Card (Dark Semi-Transparent, No Blur) */}
          <div className="mb-5 p-3 rounded-2xl bg-black/40 border border-white/10 flex items-center gap-3">
            <div className="w-8 h-8 rounded-xl bg-teal-500/20 border border-teal-400/30 flex items-center justify-center shrink-0">
              <Cpu size={16} className="text-teal-300" />
            </div>
            <div className="overflow-hidden">
              <div className="text-[10px] font-bold text-stone-400 uppercase tracking-wider">
                ID Perangkat Terdeteksi
              </div>
              <div className="text-xs font-mono font-bold text-teal-200 truncate">
                {deviceId}
              </div>
            </div>
          </div>

          {/* Login Form */}
          <form onSubmit={handleSubmit} className="space-y-4">
            {/* Input Username */}
            <div className="space-y-1.5">
              <label className="block text-xs font-bold text-stone-300 uppercase tracking-wider">
                Username Lisensi
              </label>
              <div className="relative flex items-center">
                <div className="absolute left-3.5 text-teal-400/80 pointer-events-none">
                  <User size={16} />
                </div>
                <input
                  type="text"
                  value={username}
                  onChange={(e) => setUsername(e.target.value)}
                  placeholder="Masukkan username Anda..."
                  disabled={loading}
                  className="w-full pl-10 pr-4 py-3 bg-black/35 hover:bg-black/45 focus:bg-black/55 border border-white/10 focus:border-teal-400 rounded-2xl text-sm font-semibold text-white placeholder-stone-400 outline-none transition-all"
                />
              </div>
            </div>

            {/* Input Password */}
            <div className="space-y-1.5">
              <div className="flex items-center justify-between">
                <label className="block text-xs font-bold text-stone-300 uppercase tracking-wider">
                  Password Akun
                </label>
              </div>
              <div className="relative flex items-center">
                <div className="absolute left-3.5 text-teal-400/80 pointer-events-none">
                  <KeyRound size={16} />
                </div>
                <input
                  type={showPassword ? 'text' : 'password'}
                  value={password}
                  onChange={(e) => setPassword(e.target.value)}
                  placeholder="Masukkan password Anda..."
                  disabled={loading}
                  className="w-full pl-10 pr-11 py-3 bg-black/35 hover:bg-black/45 focus:bg-black/55 border border-white/10 focus:border-teal-400 rounded-2xl text-sm font-semibold text-white placeholder-stone-400 outline-none transition-all"
                />
                <button
                  type="button"
                  onClick={() => setShowPassword((prev) => !prev)}
                  className="absolute right-3.5 text-stone-400 hover:text-white transition-colors cursor-pointer"
                  title={showPassword ? 'Sembunyikan password' : 'Lihat password'}
                >
                  {showPassword ? <EyeOff size={16} /> : <Eye size={16} />}
                </button>
              </div>
            </div>

            {/* Alert / Notification Feedback */}
            {errorMsg && (
              <div className="p-3 rounded-2xl bg-rose-500/25 border border-rose-500/50 flex items-start gap-2.5 text-xs font-bold text-rose-100">
                <AlertCircle size={16} className="shrink-0 text-rose-300 mt-0.5" />
                <span>{errorMsg}</span>
              </div>
            )}
            {successMsg && (
              <div className="p-3 rounded-2xl bg-teal-500/25 border border-teal-500/50 flex items-start gap-2.5 text-xs font-bold text-teal-100">
                <CheckCircle2 size={16} className="shrink-0 text-teal-300 mt-0.5" />
                <span>{successMsg}</span>
              </div>
            )}

            {/* Submit Button: Outlined Translucent Teal Capsule (Exact match with reference image) */}
            <button
              type="submit"
              disabled={loading}
              className="w-full py-3.5 px-6 rounded-2xl border border-teal-400/60 hover:border-teal-300 bg-teal-950/25 hover:bg-teal-900/35 active:scale-[0.98] text-teal-300 hover:text-teal-200 font-bold text-sm tracking-wider uppercase transition-all duration-300 cursor-pointer disabled:opacity-50 disabled:cursor-not-allowed flex items-center justify-center gap-2 mt-3"
            >
              {loading ? (
                <>
                  <div className="w-4 h-4 border-2 border-teal-300 border-t-transparent rounded-full animate-spin" />
                  <span className="text-teal-300">Memverifikasi Hardware...</span>
                </>
              ) : (
                <>
                  <ShieldCheck size={18} className="text-teal-300" />
                  <span>GASSS LOGHINN</span>
                </>
              )}
            </button>
          </form>

          {/* Footer: WhatsApp License Purchase & Reset */}
          <div className="mt-6 pt-5 border-t border-white/10 text-center space-y-2.5">
            <p className="text-[11px] text-stone-300 font-medium">
              Belum punya lisensi atau butuh reset hardware binding?
            </p>
            <button
              type="button"
              onClick={handleOpenWhatsApp}
              className="w-full py-2.5 px-4 rounded-xl bg-black/35 hover:bg-black/50 active:scale-[0.98] border border-white/10 text-xs font-bold text-teal-300 hover:text-teal-200 flex items-center justify-center gap-2 transition-all cursor-pointer shadow-sm"
            >
              <MessageSquare size={14} />
              <span>Aktivasi / Bantuan via WhatsApp</span>
              <ExternalLink size={12} className="opacity-70" />
            </button>
          </div>
        </div>
      </div>

      {/* 5. Minimalist Floating Pill when Form is Hidden */}
      {!isFormVisible && (
        <div className="absolute bottom-8 right-8 z-30">
          <button
            type="button"
            onClick={() => setIsFormVisible(true)}
            className="flex items-center gap-2.5 px-7 py-3.5 rounded-full bg-black/40 hover:bg-black/60 text-teal-300 font-bold text-sm tracking-wider uppercase border border-teal-400/60 shadow-[0_15px_40px_rgba(0,0,0,0.4)] active:scale-95 transition-all duration-300 cursor-pointer"
          >
            <UserCircle2 size={18} className="text-teal-300" />
            <span>GASSS LOGHINN</span>
          </button>
        </div>
      )}

      {/* 6. Subtle Bottom Copyright */}
      <footer className="absolute bottom-4 left-6 md:left-10 z-10 pointer-events-none">
        <p className="text-[10px] font-mono text-white/50 tracking-wider">
          © 2026 ZITERA LABS. ALL RIGHTS RESERVED. PROTECTED BY ENCLAVE HARDWARE BINDING.
        </p>
      </footer>
    </div>
  )
}
