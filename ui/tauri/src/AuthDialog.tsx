import { useState, type FormEvent } from 'react'
import {
  AVATAR_REGISTRY,
  getAvatarById,
  type AvatarDefinition,
} from './AvatarRegistry'
import {
  getCurrentUser,
  login,
  register,
  updateAvatar,
  logout,
} from './authService'

interface AuthDialogProps {
  isOpen: boolean
  onClose: () => void
  initialMode?: 'login' | 'register' | 'avatars'
}

type TabMode = 'login' | 'register' | 'avatars'
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
      setMessage({ text: `Welcome back, Operative ${res.user?.username}!` })
      setTimeout(() => {
        onClose()
      }, 700)
    } else {
      setMessage({ text: res.message, error: true })
    }
  }

  const handleRegister = async (e: FormEvent) => {
    e.preventDefault()
    setLoading(true)
    setMessage(null)
    const res = await register(username, password, selectedAvatarId)
    setLoading(false)
    if (res.success) {
      setMessage({ text: `Operative registered! Assigned: ${selectedAvatarId}` })
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
        setMessage({ text: `Operative identity updated to ${avatar.name}!` })
      } else {
        setMessage({ text: 'Failed to update avatar.', error: true })
      }
    }
  }

  const handleLogout = async () => {
    setLoading(true)
    await logout()
    setLoading(false)
    setTab('login')
    setMessage({ text: 'Logged out successfully.' })
  }

  const selectedAvatar = getAvatarById(selectedAvatarId)

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/50 backdrop-blur-xs">
      <div
        className="w-full max-w-2xl bg-[#FCFBF8] border-2 border-[#E6E2D8] rounded-xl shadow-2xl overflow-hidden flex flex-col max-h-[90vh]"
        onClick={(e) => e.stopPropagation()}
      >
        {/* Header */}
        <div className="flex items-center justify-between px-6 py-4 bg-[#F0EDE8] border-b border-[#E6E2D8]">
          <div className="flex items-center gap-3">
            <div className="w-9 h-9 rounded-full border-2 border-[#2DD4BF] bg-[#CCFBF1] flex items-center justify-center overflow-hidden">
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
              <h2 className="text-base font-bold font-['Space_Grotesk'] text-[#1E1A14]">
                OPERATIVE HQ // {currentUser ? currentUser.username.toUpperCase() : 'AUTH GATEWAY'}
              </h2>
              <p className="text-xs font-['JetBrains_Mono'] text-[#0F766E]">
                {currentUser ? `Active Avatar: ${selectedAvatar.name} (${selectedAvatar.role})` : 'Turso CTF Session Hub'}
              </p>
            </div>
          </div>
          <button
            onClick={onClose}
            className="w-8 h-8 rounded-lg border border-[#E6E2D8] bg-[#FCFBF8] hover:bg-[#F0EDE8] text-[#5C5347] flex items-center justify-center text-sm font-bold"
          >
            ✕
          </button>
        </div>

        {/* Tab switcher */}
        <div className="flex border-b border-[#E6E2D8] bg-[#F7F5F0] px-6 gap-2 pt-2">
          {!currentUser ? (
            <>
              <button
                onClick={() => { setTab('login'); setMessage(null) }}
                className={`px-4 py-2 text-xs font-['Space_Grotesk'] font-bold rounded-t-lg transition-colors border-t border-x ${
                  tab === 'login'
                    ? 'bg-[#FCFBF8] border-[#2DD4BF] text-[#0F766E]'
                    : 'border-transparent text-[#9C9080] hover:text-[#1E1A14]'
                }`}
              >
                LOGIN
              </button>
              <button
                onClick={() => { setTab('register'); setMessage(null) }}
                className={`px-4 py-2 text-xs font-['Space_Grotesk'] font-bold rounded-t-lg transition-colors border-t border-x ${
                  tab === 'register'
                    ? 'bg-[#FCFBF8] border-[#2DD4BF] text-[#0F766E]'
                    : 'border-transparent text-[#9C9080] hover:text-[#1E1A14]'
                }`}
              >
                REGISTER
              </button>
            </>
          ) : (
            <button
              onClick={() => { setTab('avatars'); setMessage(null) }}
              className="px-4 py-2 text-xs font-['Space_Grotesk'] font-bold rounded-t-lg bg-[#FCFBF8] border-t border-x border-[#2DD4BF] text-[#0F766E]"
            >
              OPERATIVE ROSTER (14 AVATARS)
            </button>
          )}

          <button
            onClick={() => { setTab('avatars'); setMessage(null) }}
            className={`px-4 py-2 text-xs font-['Space_Grotesk'] font-bold rounded-t-lg transition-colors border-t border-x ${
              tab === 'avatars'
                ? 'bg-[#FCFBF8] border-[#2DD4BF] text-[#0F766E]'
                : 'border-transparent text-[#9C9080] hover:text-[#1E1A14]'
            }`}
          >
            ROSTER ({selectedAvatar.name})
          </button>

          {currentUser && (
            <div className="ml-auto flex items-center pb-2">
              <button
                onClick={handleLogout}
                disabled={loading}
                className="px-3 py-1 text-xs font-['JetBrains_Mono'] text-[#EF4444] hover:bg-[#FEE2E2] rounded border border-[#EF4444]/30"
              >
                LOGOUT
              </button>
            </div>
          )}
        </div>

        {/* Message Banner */}
        {message && (
          <div
            className={`px-6 py-2 text-xs font-['JetBrains_Mono'] flex items-center gap-2 border-b ${
              message.error
                ? 'bg-[#FEF2F2] border-[#FCA5A5] text-[#DC2626]'
                : 'bg-[#ECFDF5] border-[#6EE7B7] text-[#0F766E]'
            }`}
          >
            <span>{message.error ? '⚠' : '✓'}</span>
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
                  placeholder="e.g. shadow_neko"
                  className="w-full px-3 py-2 text-sm bg-white border border-[#E6E2D8] rounded-lg focus:outline-none focus:border-[#2DD4BF] font-['JetBrains_Mono']"
                />
              </div>

              <div>
                <label className="block text-xs font-['Space_Grotesk'] font-bold text-[#5C5347] mb-1">
                  PASSWORD
                </label>
                <input
                  type="password"
                  required
                  value={password}
                  onChange={(e) => setPassword(e.target.value)}
                  placeholder="••••••••"
                  className="w-full px-3 py-2 text-sm bg-white border border-[#E6E2D8] rounded-lg focus:outline-none focus:border-[#2DD4BF] font-['JetBrains_Mono']"
                />
              </div>

              <button
                type="submit"
                disabled={loading}
                className="w-full py-2.5 px-4 bg-[#0F766E] hover:bg-[#115E59] text-white text-xs font-['Space_Grotesk'] font-bold rounded-lg shadow-sm transition-all disabled:opacity-50"
              >
                {loading ? 'AUTHENTICATING...' : 'LOGIN TO LAB'}
              </button>

              <p className="text-center text-xs text-[#9C9080]">
                Need an account?{' '}
                <button
                  type="button"
                  onClick={() => setTab('register')}
                  className="text-[#0F766E] font-bold hover:underline"
                >
                  Register new Operative
                </button>
              </p>
            </form>
          )}

          {tab === 'register' && (
            <form onSubmit={handleRegister} className="space-y-4 max-w-md mx-auto py-2">
              <div>
                <label className="block text-xs font-['Space_Grotesk'] font-bold text-[#5C5347] mb-1">
                  DESIRED OPERATIVE CODENAME
                </label>
                <input
                  type="text"
                  required
                  value={username}
                  onChange={(e) => setUsername(e.target.value)}
                  placeholder="e.g. cyber_operator"
                  className="w-full px-3 py-2 text-sm bg-white border border-[#E6E2D8] rounded-lg focus:outline-none focus:border-[#2DD4BF] font-['JetBrains_Mono']"
                />
              </div>

              <div>
                <label className="block text-xs font-['Space_Grotesk'] font-bold text-[#5C5347] mb-1">
                  PASSWORD
                </label>
                <input
                  type="password"
                  required
                  value={password}
                  onChange={(e) => setPassword(e.target.value)}
                  placeholder="At least 4 characters"
                  className="w-full px-3 py-2 text-sm bg-white border border-[#E6E2D8] rounded-lg focus:outline-none focus:border-[#2DD4BF] font-['JetBrains_Mono']"
                />
              </div>

              {/* Chosen avatar preview */}
              <div className="p-3 bg-[#F0EDE8] rounded-lg border border-[#E6E2D8] flex items-center justify-between">
                <div className="flex items-center gap-3">
                  <div className="w-12 h-12 rounded-full border-2 border-[#2DD4BF] overflow-hidden bg-white">
                    <img
                      src={selectedAvatar.assetPath}
                      alt={selectedAvatar.name}
                      className="w-full h-full object-cover"
                    />
                  </div>
                  <div>
                    <div className="text-xs font-bold font-['Space_Grotesk'] text-[#1E1A14]">
                      {selectedAvatar.name} — {selectedAvatar.role}
                    </div>
                    <div className="text-[10px] font-['JetBrains_Mono'] text-[#5C5347]">
                      {selectedAvatar.expression}
                    </div>
                  </div>
                </div>
                <button
                  type="button"
                  onClick={() => setTab('avatars')}
                  className="text-xs font-['Space_Grotesk'] font-bold px-3 py-1.5 bg-[#2DD4BF]/20 text-[#0F766E] border border-[#2DD4BF] rounded-md hover:bg-[#2DD4BF]/30"
                >
                  Change
                </button>
              </div>

              <button
                type="submit"
                disabled={loading}
                className="w-full py-2.5 px-4 bg-[#0F766E] hover:bg-[#115E59] text-white text-xs font-['Space_Grotesk'] font-bold rounded-lg shadow-sm transition-all disabled:opacity-50"
              >
                {loading ? 'REGISTERING...' : 'REGISTER OPERATIVE'}
              </button>
            </form>
          )}

          {tab === 'avatars' && (
            <div className="space-y-4">
              {/* Filter controls */}
              <div className="flex flex-wrap items-center justify-between gap-2">
                <div className="flex items-center gap-1 bg-[#F0EDE8] p-1 rounded-lg border border-[#E6E2D8]">
                  {(['ALL', 'Female', 'Male', 'Classic'] as GenderFilter[]).map((filter) => (
                    <button
                      key={filter}
                      onClick={() => setGenderFilter(filter)}
                      className={`px-3 py-1 text-xs font-['Space_Grotesk'] font-bold rounded-md transition-colors ${
                        genderFilter === filter
                          ? 'bg-[#FCFBF8] text-[#0F766E] shadow-xs'
                          : 'text-[#5C5347] hover:text-[#1E1A14]'
                      }`}
                    >
                      {filter}
                    </button>
                  ))}
                </div>

                <input
                  type="text"
                  placeholder="Filter by name or role..."
                  value={searchQuery}
                  onChange={(e) => setSearchQuery(e.target.value)}
                  className="px-3 py-1.5 text-xs bg-white border border-[#E6E2D8] rounded-lg focus:outline-none focus:border-[#2DD4BF] font-['JetBrains_Mono'] w-52"
                />
              </div>

              {/* Grid of 14 avatars */}
              <div className="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 gap-3">
                {filteredAvatars.map((avatar) => {
                  const isSelected = selectedAvatarId === avatar.id
                  return (
                    <div
                      key={avatar.id}
                      onClick={() => handleAvatarSelect(avatar)}
                      className={`p-3 rounded-xl border cursor-pointer transition-all flex flex-col items-center text-center ${
                        isSelected
                          ? 'border-[#2DD4BF] bg-[#CCFBF1]/30 shadow-md ring-2 ring-[#2DD4BF]/50'
                          : 'border-[#E6E2D8] bg-[#FCFBF8] hover:border-[#2DD4BF]/60 hover:bg-[#F0FDFA]'
                      }`}
                    >
                      <div className="relative w-16 h-16 rounded-full border-2 border-[#2DD4BF] overflow-hidden bg-white mb-2 shadow-xs">
                        <img
                          src={avatar.assetPath}
                          alt={avatar.name}
                          className="w-full h-full object-cover"
                          onError={(e) => {
                            ;(e.target as HTMLElement).style.display = 'none'
                          }}
                        />
                        {isSelected && (
                          <div className="absolute top-0 right-0 w-4 h-4 bg-[#0F766E] text-white text-[10px] rounded-full flex items-center justify-center font-bold">
                            ✓
                          </div>
                        )}
                      </div>

                      <div className="font-['Space_Grotesk'] font-bold text-xs text-[#1E1A14]">
                        {avatar.name}
                      </div>
                      <div className="text-[10px] font-['JetBrains_Mono'] text-[#0F766E] font-medium leading-tight">
                        {avatar.role}
                      </div>
                      <div className="text-[9px] text-[#9C9080] mt-1 line-clamp-1">
                        {avatar.expression}
                      </div>
                    </div>
                  )
                })}
              </div>

              {/* Confirm button if not logged in */}
              {!currentUser && (
                <div className="pt-2 flex justify-end">
                  <button
                    type="button"
                    onClick={() => setTab('register')}
                    className="px-5 py-2 bg-[#0F766E] hover:bg-[#115E59] text-white text-xs font-['Space_Grotesk'] font-bold rounded-lg"
                  >
                    SELECT {selectedAvatar.name.toUpperCase()} & RETURN
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
