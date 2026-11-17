import { useState, useEffect } from 'react'
import {
  Sparkles,
  Database,
  User,
} from 'lucide-react'
import { getCurrentUser, logout, subscribeAuth, type UserModel } from './authService'
import { getAvatarById } from './AvatarRegistry'

interface SettingsViewProps {
  cursorEnabled: boolean
  onToggleCursor: (val: boolean) => void
  onOpenAuth: () => void
}

export function SettingsView({ cursorEnabled, onToggleCursor, onOpenAuth }: SettingsViewProps) {
  const [currentUser, setCurrentUser] = useState<UserModel | null>(getCurrentUser())
  const [language, setLanguage] = useState<'id' | 'en'>('en')

  useEffect(() => {
    return subscribeAuth((user) => setCurrentUser(user))
  }, [])

  const activeAvatar = getAvatarById(currentUser?.avatarId ?? 'zeta')

  return (
    <div className="flex-1 h-full overflow-hidden relative bg-[#FCFBF8]">
      {/* Pinned Ambient Background Artwork (Matches Flutter bg_settings.jpg) */}
      <div className="absolute inset-0 pointer-events-none z-0 overflow-hidden select-none">
        <img
          src="/assets/bg_settings.jpg"
          alt=""
          className="w-full h-full object-cover object-center opacity-22"
          onError={(e) => {
            ;(e.target as HTMLElement).style.display = 'none'
          }}
        />
        <div className="absolute inset-0 bg-[#FCFBF8]/80" />
      </div>

      <div className="relative z-10 h-full overflow-y-auto p-6 space-y-6">
        {/* Header */}
      <div className="flex flex-col md:flex-row md:items-center justify-between gap-4 border-b border-[#E6E2D8] pb-5">
        <div>
          <div className="flex items-center gap-2">
            <span className="px-2 py-0.5 rounded bg-[#F0EDE8] text-[#5C5347] border border-[#E6E2D8] text-[10px] font-['Space_Grotesk'] font-bold">
              PREFERENCES
            </span>
            <span className="text-[10px] font-['JetBrains_Mono'] text-[#9C9080]">
              SYSTEM CONFIGURATION
            </span>
          </div>
          <h1 className="text-xl font-bold font-['Space_Grotesk'] text-[#1E1A14] mt-1">
            Application Settings & Session
          </h1>
        </div>
      </div>

      <div className="space-y-6 max-w-2xl">
        {/* Section 1: Operative Profile */}
        <div className="p-5 rounded-xl border-2 border-[#CEC8BF] bg-[#FCFBF8] shadow-[0_3px_0_0_#E4DFD7] space-y-4">
          <div className="flex items-center gap-2 font-['Space_Grotesk'] font-bold text-sm text-[#1E1A14]">
            <User size={16} className="text-[#0F766E]" />
            <span>OPERATIVE IDENTITY</span>
          </div>

          {currentUser ? (
            <div className="flex items-center justify-between p-3.5 bg-[#F7F5F0] rounded-xl border-2 border-[#E4DFD7] shadow-[0_1px_0_0_#E4DFD7]">
              <div className="flex items-center gap-3">
                <div className="w-12 h-12 rounded-full border-2 border-[#2DD4BF] overflow-hidden bg-white">
                  <img
                    src={activeAvatar.assetPath}
                    alt=""
                    className="w-full h-full object-cover"
                  />
                </div>
                <div>
                  <div className="font-['Space_Grotesk'] font-bold text-sm text-[#1E1A14]">
                    {currentUser.username}
                  </div>
                  <div className="text-[10px] font-['JetBrains_Mono'] text-[#0F766E]">
                    {activeAvatar.name} — {activeAvatar.role}
                  </div>
                  <div className="text-[9px] font-['JetBrains_Mono'] text-[#9C9080] mt-0.5">
                    ID: {currentUser.id}
                  </div>
                </div>
              </div>

              <div className="flex items-center gap-2">
                <button
                  onClick={onOpenAuth}
                  className="px-3 py-1.5 text-xs font-bold font-['Space_Grotesk'] bg-[#CCFBF1] text-[#0F766E] border-2 border-[#2DD4BF] shadow-[0_2px_0_0_#5EEAD4] hover:translate-y-0.5 active:translate-y-1 rounded-lg cursor-pointer transition-all"
                >
                  Change Avatar
                </button>
                <button
                  onClick={() => logout()}
                  className="px-3 py-1.5 text-xs font-bold font-['Space_Grotesk'] text-[#EF4444] border-2 border-[#EF4444]/40 hover:bg-[#FEE2E2] shadow-[0_2px_0_0_#FCA5A5] hover:translate-y-0.5 active:translate-y-1 rounded-lg cursor-pointer transition-all"
                >
                  Logout
                </button>
              </div>
            </div>
          ) : (
            <div className="p-4 bg-[#F7F5F0] rounded-xl border-2 border-[#E4DFD7] shadow-[0_1px_0_0_#E4DFD7] flex items-center justify-between">
              <div>
                <div className="text-xs font-bold font-['Space_Grotesk'] text-[#1E1A14]">
                  Anonymous Operator Session
                </div>
                <div className="text-[10px] text-[#5C5347] font-['JetBrains_Mono']">
                  Login or create an account to record your CTF flags to Turso.
                </div>
              </div>
              <button
                onClick={onOpenAuth}
                className="px-4 py-2 bg-[#0F766E] hover:bg-[#115E59] text-white text-xs font-bold rounded-lg font-['Space_Grotesk'] border-2 border-[#094D47] shadow-[0_2px_0_0_#094D47] hover:translate-y-0.5 active:translate-y-1 cursor-pointer transition-all"
              >
                Sign In
              </button>
            </div>
          )}
        </div>

        {/* Section 2: Visual & Interactions */}
        <div className="p-5 rounded-xl border-2 border-[#CEC8BF] bg-[#FCFBF8] shadow-[0_3px_0_0_#E4DFD7] space-y-4">
          <div className="flex items-center gap-2 font-['Space_Grotesk'] font-bold text-sm text-[#1E1A14]">
            <Sparkles size={16} className="text-[#2DD4BF]" />
            <span>VISUAL & INTERACTION TWEAKS</span>
          </div>

          {/* Kawaii Cat Paw Toggle */}
          <div className="flex items-center justify-between p-3 bg-[#F7F5F0] rounded-lg border-2 border-[#E4DFD7]">
            <div>
              <div className="text-xs font-bold font-['Space_Grotesk'] text-[#1E1A14]">
                Kawaii Anime Cat Paw Cursor
              </div>
              <div className="text-[10px] text-[#5C5347] font-['JetBrains_Mono']">
                Reactive cat paw pointer with cute sparkle particle trail
              </div>
            </div>
            <button
              onClick={() => onToggleCursor(!cursorEnabled)}
              className={`w-12 h-6 flex items-center rounded-full p-1 transition-colors ${
                cursorEnabled ? 'bg-[#0F766E]' : 'bg-[#E6E2D8]'
              }`}
            >
              <div
                className={`bg-white w-4 h-4 rounded-full shadow-md transform transition-transform ${
                  cursorEnabled ? 'translate-x-6' : 'translate-x-0'
                }`}
              />
            </button>
          </div>

          {/* Language Toggle */}
          <div className="flex items-center justify-between p-3 bg-[#F7F5F0] rounded-lg border-2 border-[#E4DFD7]">
            <div>
              <div className="text-xs font-bold font-['Space_Grotesk'] text-[#1E1A14]">
                Curriculum Language
              </div>
              <div className="text-[10px] text-[#5C5347] font-['JetBrains_Mono']">
                Toggle lab theory and challenge instructions
              </div>
            </div>
            <div className="flex items-center gap-1 bg-white p-1 rounded-lg border-2 border-[#E4DFD7]">
              <button
                onClick={() => setLanguage('en')}
                className={`px-3 py-1 text-xs font-bold rounded ${
                  language === 'en'
                    ? 'bg-[#0F766E] text-white'
                    : 'text-[#5C5347] hover:text-[#1E1A14]'
                }`}
              >
                EN
              </button>
              <button
                onClick={() => setLanguage('id')}
                className={`px-3 py-1 text-xs font-bold rounded ${
                  language === 'id'
                    ? 'bg-[#0F766E] text-white'
                    : 'text-[#5C5347] hover:text-[#1E1A14]'
                }`}
              >
                ID
              </button>
            </div>
          </div>
        </div>

        {/* Section 3: Turso LibSQL Cloud Database */}
        <div className="p-5 rounded-xl border-2 border-[#CEC8BF] bg-[#FCFBF8] shadow-[0_3px_0_0_#E4DFD7] space-y-3">
          <div className="flex items-center gap-2 font-['Space_Grotesk'] font-bold text-sm text-[#1E1A14]">
            <Database size={16} className="text-[#38BDF8]" />
            <span>TURSO LIBSQL CLOUD SYNC</span>
          </div>
          <div className="text-xs text-[#5C5347] font-['JetBrains_Mono'] bg-[#F7F5F0] p-3 rounded-lg border border-[#E6E2D8] space-y-1">
            <div className="flex items-center gap-2">
              <span className="w-2 h-2 rounded-full bg-[#10B981]" />
              <span className="font-bold text-[#1E1A14]">Turso v2 Pipeline Online</span>
            </div>
            <div className="text-[10px] text-[#9C9080] truncate">
              Endpoint: https://ziteralab-xdaamar.aws-ap-northeast-1.turso.io/v2/pipeline
            </div>
          </div>
        </div>
      </div>
      </div>
    </div>
  )
}
