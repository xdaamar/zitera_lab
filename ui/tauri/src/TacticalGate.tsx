import { useState, useEffect, type FormEvent } from 'react'
import {
  ShieldCheck,
  KeyRound,
  User,
  Sparkles,
  Eye,
  EyeOff,
  MessageSquare,
  ExternalLink,
  Cpu,
  CheckCircle2,
  AlertCircle,
  Maximize2,
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
  const [isFormVisible, setIsFormVisible] = useState(true)

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
      {/* 1. Cinematic Background: Makoto Shinkai / Genshin Impact Mountain Scenic Wallpaper */}
      <div className="absolute inset-0 overflow-hidden pointer-events-none">
        <div
          className="absolute -inset-10 bg-cover bg-center animate-anime-pan"
          style={{
            backgroundImage: "url('/assets/login_bg.jpg')",
            willChange: 'transform',
            transform: 'translateZ(0)',
          }}
        />
        {/* Soft atmospheric gradient overlays for iOS-level contrast & anime sunset vibrance */}
        <div className="absolute inset-0 bg-gradient-to-r from-stone-950/60 via-transparent to-stone-950/70" />
        <div className="absolute inset-0 bg-gradient-to-t from-stone-950/80 via-transparent to-stone-950/30" />
        
        {/* Radial subtle anime lens flare & cyan ambient glow */}
        <div className="absolute top-1/4 left-1/3 w-96 h-96 bg-cyan-400/10 rounded-full blur-3xl animate-pulse-glow pointer-events-none" />
        <div className="absolute bottom-1/3 right-1/4 w-80 h-80 bg-teal-400/15 rounded-full blur-3xl animate-pulse-glow pointer-events-none" />
      </div>

      {/* 2. Top Navigation Bar: Stylized Brand Typography & Quick Controls */}
      <header className="absolute top-0 left-0 right-0 z-20 flex items-center justify-between px-6 md:px-10 py-5">
        <div className="flex items-center gap-3.5">
          <div className="w-10 h-10 rounded-2xl bg-white/10 backdrop-blur-xl border border-white/20 flex items-center justify-center shadow-[0_4px_20px_rgba(45,212,191,0.25)]">
            <Sparkles size={20} className="text-teal-300 animate-float-gentle" />
          </div>
          <div>
            <div className="flex items-center gap-2">
              <span className="font-['Space_Grotesk'] font-black text-2xl tracking-wider bg-gradient-to-r from-white via-teal-200 to-cyan-300 bg-clip-text text-transparent drop-shadow-[0_2px_12px_rgba(45,212,191,0.4)]">
                ZITERA
              </span>
              <span className="px-2 py-0.5 rounded-full text-[10px] font-bold tracking-widest uppercase bg-teal-500/20 text-teal-300 border border-teal-400/30 backdrop-blur-md">
                ACADEMY
              </span>
            </div>
            <p className="text-[11px] font-['JetBrains_Mono'] text-stone-300/80 tracking-wide">
              GENSHIN IMPACT AESTHETIC // VIRTUAL CYBER RANGE
            </p>
          </div>
        </div>

        {/* Top-Right Toggle & View Button */}
        <div className="flex items-center gap-3">
          <button
            type="button"
            onClick={() => setIsFormVisible((prev) => !prev)}
            className="flex items-center gap-2 px-4 py-2 rounded-2xl bg-white/10 hover:bg-white/20 active:scale-95 text-xs font-bold text-white backdrop-blur-2xl border border-white/20 shadow-[0_4px_20px_rgba(0,0,0,0.3)] transition-all duration-300 cursor-pointer"
            title={isFormVisible ? 'Sembunyikan menu login untuk menikmati pemandangan' : 'Tampilkan menu login'}
          >
            {isFormVisible ? (
              <>
                <EyeOff size={15} className="text-teal-300" />
                <span className="hidden sm:inline">Nikmati Pemandangan</span>
              </>
            ) : (
              <>
                <Eye size={15} className="text-teal-300" />
                <span>Buka Form Login</span>
              </>
            )}
          </button>
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

      {/* 4. Right Side: iPhone Glass Glow Aesthetic Login Card */}
      <div
        className={`relative z-20 w-full sm:w-[480px] p-4 sm:p-8 sm:pr-12 lg:pr-16 flex flex-col justify-center transition-all duration-500 ease-out ${
          isFormVisible
            ? 'translate-x-0 opacity-100 pointer-events-auto'
            : 'translate-x-24 opacity-0 pointer-events-none'
        }`}
      >
        <div className="relative w-full rounded-3xl backdrop-blur-3xl bg-black/35 border border-white/20 p-6 sm:p-8 shadow-[0_25px_60px_-15px_rgba(0,0,0,0.7),0_0_40px_rgba(45,212,191,0.18)] text-white overflow-hidden">
          {/* Frosted Glass Top Edge Highlight */}
          <div className="absolute top-0 left-0 right-0 h-[1.5px] bg-gradient-to-r from-transparent via-white/50 to-transparent" />

          {/* Card Header & Security Badge */}
          <div className="space-y-3 mb-6">
            <div className="flex items-center justify-between">
              <div className="inline-flex items-center gap-1.5 px-3 py-1 rounded-full bg-teal-500/15 border border-teal-400/30 backdrop-blur-md">
                <span className="w-2 h-2 rounded-full bg-teal-400 animate-ping" />
                <span className="text-[10px] font-black font-['JetBrains_Mono'] tracking-wider text-teal-300 uppercase">
                  HARDWARE BOUND
                </span>
              </div>
              <button
                type="button"
                onClick={() => setIsFormVisible(false)}
                className="p-1 rounded-xl text-stone-400 hover:text-white hover:bg-white/10 transition-colors"
                title="Sembunyikan Menu"
              >
                <Maximize2 size={16} />
              </button>
            </div>

            <div>
              <h2 className="text-2xl font-black font-['Space_Grotesk'] tracking-tight text-white flex items-center gap-2">
                <span>Ruang Operasi Siswa</span>
                <Sparkles size={18} className="text-teal-300" />
              </h2>
              <p className="text-xs text-stone-300/80 mt-1">
                Gunakan kredensial yang diberikan Administrator untuk membuka lisensi.
              </p>
            </div>
          </div>

          {/* Single-Machine Hardware Enclave Fingerprint Card */}
          <div className="mb-5 p-3 rounded-2xl bg-white/[0.06] border border-white/10 backdrop-blur-md flex items-center gap-3">
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
              <label className="block text-xs font-bold text-stone-200 uppercase tracking-wider">
                Username Lisensi
              </label>
              <div className="relative flex items-center">
                <div className="absolute left-3.5 text-stone-400 pointer-events-none">
                  <User size={16} />
                </div>
                <input
                  type="text"
                  value={username}
                  onChange={(e) => setUsername(e.target.value)}
                  placeholder="Masukkan username Anda..."
                  disabled={loading}
                  className="w-full pl-10 pr-4 py-3 bg-white/[0.08] hover:bg-white/[0.12] focus:bg-white/[0.15] border border-white/15 focus:border-teal-400 focus:shadow-[0_0_20px_rgba(45,212,191,0.35)] rounded-2xl text-sm font-medium text-white placeholder-stone-400/70 outline-none backdrop-blur-xl transition-all"
                />
              </div>
            </div>

            {/* Input Password */}
            <div className="space-y-1.5">
              <div className="flex items-center justify-between">
                <label className="block text-xs font-bold text-stone-200 uppercase tracking-wider">
                  Password Akun
                </label>
              </div>
              <div className="relative flex items-center">
                <div className="absolute left-3.5 text-stone-400 pointer-events-none">
                  <KeyRound size={16} />
                </div>
                <input
                  type={showPassword ? 'text' : 'password'}
                  value={password}
                  onChange={(e) => setPassword(e.target.value)}
                  placeholder="Masukkan password Anda..."
                  disabled={loading}
                  className="w-full pl-10 pr-11 py-3 bg-white/[0.08] hover:bg-white/[0.12] focus:bg-white/[0.15] border border-white/15 focus:border-teal-400 focus:shadow-[0_0_20px_rgba(45,212,191,0.35)] rounded-2xl text-sm font-medium text-white placeholder-stone-400/70 outline-none backdrop-blur-xl transition-all"
                />
                <button
                  type="button"
                  onClick={() => setShowPassword((prev) => !prev)}
                  className="absolute right-3.5 text-stone-400 hover:text-white transition-colors"
                  title={showPassword ? 'Sembunyikan password' : 'Lihat password'}
                >
                  {showPassword ? <EyeOff size={16} /> : <Eye size={16} />}
                </button>
              </div>
            </div>

            {/* Alert / Notification Feedback */}
            {errorMsg && (
              <div className="p-3 rounded-2xl bg-rose-500/20 border border-rose-500/40 backdrop-blur-md flex items-start gap-2.5 text-xs text-rose-200">
                <AlertCircle size={16} className="shrink-0 text-rose-400 mt-0.5" />
                <span>{errorMsg}</span>
              </div>
            )}
            {successMsg && (
              <div className="p-3 rounded-2xl bg-teal-500/20 border border-teal-500/40 backdrop-blur-md flex items-start gap-2.5 text-xs text-teal-200">
                <CheckCircle2 size={16} className="shrink-0 text-teal-300 mt-0.5" />
                <span>{successMsg}</span>
              </div>
            )}

            {/* Submit Button with iPhone Glass Glow Style */}
            <button
              type="submit"
              disabled={loading}
              className="w-full py-3.5 px-6 rounded-2xl bg-gradient-to-r from-teal-400 via-emerald-400 to-teal-300 hover:from-teal-300 hover:to-emerald-300 active:scale-[0.98] text-stone-950 font-black text-sm tracking-wide shadow-[0_10px_30px_-5px_rgba(20,184,166,0.6)] hover:shadow-[0_15px_35px_-5px_rgba(20,184,166,0.8)] transition-all duration-300 cursor-pointer disabled:opacity-50 disabled:cursor-not-allowed flex items-center justify-center gap-2 mt-2"
            >
              {loading ? (
                <>
                  <div className="w-4 h-4 border-2 border-stone-950 border-t-transparent rounded-full animate-spin" />
                  <span>Memverifikasi Hardware...</span>
                </>
              ) : (
                <>
                  <ShieldCheck size={18} />
                  <span>Masuk ke Ruang Operasi</span>
                </>
              )}
            </button>
          </form>

          {/* Footer: WhatsApp License Purchase & Reset */}
          <div className="mt-6 pt-5 border-t border-white/10 text-center space-y-2.5">
            <p className="text-[11px] text-stone-300/80">
              Belum punya lisensi atau butuh reset hardware binding?
            </p>
            <button
              type="button"
              onClick={handleOpenWhatsApp}
              className="w-full py-2.5 px-4 rounded-xl bg-white/10 hover:bg-white/15 active:scale-[0.98] border border-white/15 text-xs font-bold text-teal-300 hover:text-teal-200 backdrop-blur-md flex items-center justify-center gap-2 transition-all cursor-pointer shadow-sm"
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
            className="flex items-center gap-2.5 px-6 py-3.5 rounded-full bg-black/40 hover:bg-black/60 text-white font-black text-sm backdrop-blur-3xl border border-white/30 shadow-[0_10px_40px_rgba(0,0,0,0.6),0_0_30px_rgba(45,212,191,0.3)] active:scale-95 transition-all duration-300 cursor-pointer"
          >
            <Sparkles size={16} className="text-teal-300 animate-spin" style={{ animationDuration: '6s' }} />
            <span>Masuk ke Ruang Operasi</span>
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
