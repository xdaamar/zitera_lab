import { useState, useEffect, type FormEvent } from 'react'
import { X, CheckCircle2, AlertTriangle, Check, LogOut } from 'lucide-react'
import {
  AVATAR_REGISTRY,
  getAvatarById,
  type AvatarDefinition,
} from './AvatarRegistry'
import {
  getCurrentUser,
  login,
  updateAvatar,
  logout,
} from './authService'

interface AuthDialogProps {
  isOpen: boolean
  onClose: () => void
  initialMode?: 'login' | 'avatars'
}

type TabMode = 'login' | 'avatars'
type GenderFilter = 'ALL' | 'Female' | 'Male' | 'Classic'

export function AuthDialog({ isOpen, onClose, initialMode = 'login' }: AuthDialogProps) {
  const currentUser = getCurrentUser()
  const [tab, setTab] = useState<TabMode>(currentUser ? 'avatars' : initialMode)
  const [username, setUsername] = useState('')
  const [password, setPassword] = useState('')
  const [selectedAvatarId, setSelectedAvatarId] = useState(currentUser?.avatarId ?? 'zeta')
  const [genderFilter, setGenderFilter] = useState<GenderFilter>('ALL')
  const [searchQuery, setSearchQuery] = useState('')
  const [loading, setLoading] = useState(false)
  const [message, setMessage] = useState<{ text: string; error?: boolean } | null>(null)

  useEffect(() => {
    if (!isOpen) return
    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.key === 'Escape') onClose()
    }
    window.addEventListener('keydown', handleKeyDown)
    return () => window.removeEventListener('keydown', handleKeyDown)
  }, [isOpen, onClose])

  if (!isOpen) return null

  const filteredAvatars = AVATAR_REGISTRY.filter((a) => {
    if (genderFilter === 'Female' && a.gender !== 'Female') return false
    if (genderFilter === 'Male' && a.gender !== 'Male') return false
    if (genderFilter === 'Classic' && a.gender !== 'Operator' && a.gender !== 'Mascot') return false

    if (searchQuery.trim()) {
      const q = searchQuery.toLowerCase()
      return (
        a.name.toLowerCase().includes(q) ||
        a.role.toLowerCase().includes(q) ||
        a.expression.toLowerCase().includes(q)
      )
    }
    return true
  })

  const handleLogin = async (e: FormEvent) => {
    e.preventDefault()
    setLoading(true)
    setMessage(null)
    const res = await login(username, password)
    setLoading(false)
    if (res.success) {
      setMessage({ text: `Selamat datang kembali, Operative ${res.user?.username}!` })
      setTimeout(() => {
        onClose()
      }, 700)
    } else {
      setMessage({ text: res.message, error: true })
    }
  }

  const handleAvatarSelect = async (avatar: AvatarDefinition) => {
    setSelectedAvatarId(avatar.id)
    if (currentUser) {
      setLoading(true)
      const ok = await updateAvatar(avatar.id)
      setLoading(false)
      if (ok) {
        setMessage({ text: `Identitas avatar berhasil diperbarui ke ${avatar.name}!` })
      } else {
        setMessage({ text: 'Gagal memperbarui avatar.', error: true })
      }
    }
  }

  const handleLogout = async () => {
    setLoading(true)
    await logout()
    setLoading(false)
    setTab('login')
    setMessage({ text: 'Berhasil logout dari sistem.' })
    onClose()
  }

  const selectedAvatar = getAvatarById(selectedAvatarId)

  return (
    <div
      onClick={onClose}
      className="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/60 backdrop-blur-xs cursor-pointer"
    >
      <div
        className="w-full max-w-2xl bg-[#FCFBF8] border-2 border-[#1E1A14] rounded-2xl shadow-[0_10px_0_0_#1E1A14] overflow-hidden flex flex-col max-h-[90vh] cursor-default"
        onClick={(e) => e.stopPropagation()}
      >
        {/* Header */}
        <div className="flex items-center justify-between px-6 py-4 bg-[#F0EDE8] border-b-2 border-[#E6E2D8]">
          <div className="flex items-center gap-3">
            <div className="w-10 h-10 rounded-full border-2 border-[#2DD4BF] bg-[#CCFBF1] flex items-center justify-center overflow-hidden shadow-[0_2px_0_0_#5EEAD4]">
              <img
                src={selectedAvatar.assetPath}
                alt={selectedAvatar.name}
                className="w-full h-full object-cover"
                onError={(e) => {
                  ;(e.target as HTMLElement).style.display = 'none'
                }}
              />
            </div>
            <div>
              <h2 className="text-base font-bold font-['Space_Grotesk'] text-[#1E1A14] flex items-center gap-1.5">
                <span>OPERATIVE HQ // {currentUser ? currentUser.username.toUpperCase() : 'AUTH GATEWAY'}</span>
                {currentUser?.role === 'admin' && (
                  <span className="px-1.5 py-0.2 rounded bg-[#FEF3C7] border border-[#FBBF24] text-[9.5px] font-['JetBrains_Mono'] font-bold text-[#B45309]">
                    ADMIN
                  </span>
                )}
              </h2>
              <p className="text-xs font-['JetBrains_Mono'] text-[#0F766E]">
                {currentUser
                  ? `Active Identity: ${selectedAvatar.name} (${selectedAvatar.role})`
                  : 'Sesi Resmi ZITERA LAB'}
              </p>
            </div>
          </div>
          <button
            onClick={onClose}
            className="w-8 h-8 rounded-lg border border-[#CEC8BF] bg-[#FCFBF8] hover:bg-[#F0EDE8] text-[#5C5347] flex items-center justify-center text-sm font-bold transition-colors cursor-pointer"
            title="Tutup Dialog"
          >
            <X className="w-4 h-4" />
          </button>
        </div>

        {/* Tab switcher */}
        <div className="flex border-b-2 border-[#E6E2D8] bg-[#F7F5F0] px-6 gap-2 pt-2">
          {!currentUser ? (
            <button
              onClick={() => {
                setTab('login')
                setMessage(null)
              }}
              className="px-4 py-2 text-xs font-['Space_Grotesk'] font-bold rounded-t-lg transition-colors border-t border-x bg-[#FCFBF8] border-[#2DD4BF] text-[#0F766E]"
            >
              LOGIN KE LAB
            </button>
          ) : (
            <button
              onClick={() => {
                setTab('avatars')
                setMessage(null)
              }}
              className="px-4 py-2 text-xs font-['Space_Grotesk'] font-bold rounded-t-lg transition-colors border-t border-x bg-[#FCFBF8] border-[#2DD4BF] text-[#0F766E]"
            >
              PILIH KARAKTER & AVATAR ({AVATAR_REGISTRY.length})
            </button>
          )}
        </div>

        {/* Message Banner */}
        {message && (
          <div
            className={`px-6 py-2.5 text-xs font-['JetBrains_Mono'] flex items-center gap-2 border-b ${
              message.error
                ? 'bg-[#FEE2E2] text-[#991B1B] border-[#F87171]'
                : 'bg-[#DCFCE7] text-[#166534] border-[#86EFAC]'
            }`}
          >
            {message.error ? (
              <AlertTriangle className="w-4 h-4 shrink-0 text-[#EF4444]" />
            ) : (
              <CheckCircle2 className="w-4 h-4 shrink-0 text-[#16A34A]" />
            )}
            <span>{message.text}</span>
          </div>
        )}

        {/* Tab Body */}
        <div className="p-6 overflow-y-auto flex-1">
          {tab === 'login' && (
            <form onSubmit={handleLogin} className="space-y-4 max-w-md mx-auto py-4">
              <div>
                <label className="block text-xs font-['Space_Grotesk'] font-bold text-[#5C5347] mb-1">
                  OPERATIVE USERNAME
                </label>
                <input
                  type="text"
                  required
                  value={username}
                  onChange={(e) => setUsername(e.target.value)}
                  placeholder="e.g. operative_andi"
                  className="w-full px-3 py-2 text-xs bg-white border-2 border-[#CEC8BF] rounded-xl focus:outline-none focus:border-[#2DD4BF] font-['JetBrains_Mono']"
                />
              </div>

              <div>
                <label className="block text-xs font-['Space_Grotesk'] font-bold text-[#5C5347] mb-1">
                  PASSWORD / ACCESS KEY
                </label>
                <input
                  type="password"
                  required
                  value={password}
                  onChange={(e) => setPassword(e.target.value)}
                  placeholder="••••••••"
                  className="w-full px-3 py-2 text-xs bg-white border-2 border-[#CEC8BF] rounded-xl focus:outline-none focus:border-[#2DD4BF] font-['JetBrains_Mono']"
                />
              </div>

              <button
                type="submit"
                disabled={loading}
                className="w-full py-2.5 px-4 bg-[#0F766E] hover:bg-[#115E59] text-white text-xs font-['Space_Grotesk'] font-bold rounded-xl shadow-[0_3px_0_0_#042F2C] transition-all disabled:opacity-50 cursor-pointer"
              >
                {loading ? 'MEMVERIFIKASI...' : 'MASUK KE RUANG OPERASI'}
              </button>

              <p className="text-center text-[11px] text-[#786F62] font-sans">
                Kredensial resmi diterbitkan oleh Administrator ZITERA LAB melalui WhatsApp.
              </p>
            </form>
          )}

          {tab === 'avatars' && (
            <div className="space-y-4">
              {/* Filter controls */}
              <div className="flex flex-col sm:flex-row gap-3 items-center justify-between">
                <div className="flex items-center gap-1.5 w-full sm:w-auto">
                  {(['ALL', 'Female', 'Male', 'Classic'] as GenderFilter[]).map((g) => (
                    <button
                      key={g}
                      onClick={() => setGenderFilter(g)}
                      className={`px-3 py-1 text-xs font-bold rounded-lg transition-colors cursor-pointer ${
                        genderFilter === g
                          ? 'bg-[#2DD4BF] text-[#042F2C]'
                          : 'bg-[#F0EDE8] text-[#5C5347] hover:bg-[#E6E2D8]'
                      }`}
                    >
                      {g}
                    </button>
                  ))}
                </div>
                <input
                  type="text"
                  placeholder="Cari avatar..."
                  value={searchQuery}
                  onChange={(e) => setSearchQuery(e.target.value)}
                  className="w-full sm:w-48 px-3 py-1 text-xs bg-white border border-[#CEC8BF] rounded-lg focus:outline-none focus:border-[#2DD4BF]"
                />
              </div>

              {/* Grid */}
              <div className="grid grid-cols-2 sm:grid-cols-4 md:grid-cols-5 gap-3 max-h-[48vh] overflow-y-auto pr-1">
                {filteredAvatars.map((avatar) => {
                  const isSelected = selectedAvatarId === avatar.id
                  return (
                    <button
                      key={avatar.id}
                      onClick={() => handleAvatarSelect(avatar)}
                      className={`group relative p-2.5 rounded-xl border-2 text-left flex flex-col items-center gap-1.5 transition-all cursor-pointer ${
                        isSelected
                          ? 'bg-[#CCFBF1] border-[#0F766E] shadow-[0_3px_0_0_#0F766E]'
                          : 'bg-white border-[#E6E2D8] hover:border-[#2DD4BF] hover:shadow-xs'
                      }`}
                    >
                      {isSelected && (
                        <div className="absolute top-1.5 right-1.5 w-4 h-4 bg-[#0F766E] rounded-full flex items-center justify-center text-white">
                          <Check className="w-2.5 h-2.5 stroke-[3]" />
                        </div>
                      )}
                      <div className="w-14 h-14 rounded-full overflow-hidden border-2 border-white/80 bg-[#F0EDE8] shadow-xs">
                        <img
                          src={avatar.assetPath}
                          alt={avatar.name}
                          className="w-full h-full object-cover group-hover:scale-105 transition-transform"
                          onError={(e) => {
                            ;(e.target as HTMLElement).style.display = 'none'
                          }}
                        />
                      </div>
                      <div className="text-center w-full">
                        <div className="text-xs font-bold font-['Space_Grotesk'] text-[#1E1A14] truncate">
                          {avatar.name}
                        </div>
                        <div className="text-[10px] text-[#786F62] truncate">
                          {avatar.role}
                        </div>
                      </div>
                    </button>
                  )
                })}
              </div>

              {/* Logout button at bottom */}
              {currentUser && (
                <div className="pt-4 border-t-2 border-[#E6E2D8] flex items-center justify-between">
                  <div className="text-xs font-['JetBrains_Mono'] text-[#786F62]">
                    Hardware Bound: {currentUser.deviceId ? 'TERIKAT' : 'UNBOUND'}
                  </div>
                  <button
                    onClick={handleLogout}
                    className="px-3.5 py-1.5 rounded-xl bg-[#FEE2E2] hover:bg-[#FCA5A5] text-[#991B1B] border border-[#F87171] text-xs font-bold font-['Space_Grotesk'] flex items-center gap-1.5 transition-all cursor-pointer"
                  >
                    <LogOut size={13} />
                    <span>Logout dari Lab</span>
                  </button>
                </div>
              )}
            </div>
          )}
        </div>
      </div>
    </div>
  )
}
