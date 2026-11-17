import React, { useState, useEffect } from 'react'
import {
  LayoutDashboard,
  FlaskConical,
  Trophy,
  Wrench,
  Cpu,
  Settings,
  ChevronRight,
  ChevronDown,
  ChevronUp,
} from 'lucide-react'
import { getCurrentUser, subscribeAuth, type UserModel } from './authService'
import { getAvatarById } from './AvatarRegistry'

export type NavIndex = 0 | 1 | 2 | 3 | 4 | 5

interface SidebarProps {
  selectedIndex: NavIndex
  onSelect: (index: NavIndex) => void
  onOpenAuth: () => void
}

interface NavItemDef {
  index: NavIndex
  label: string
  icon: React.ComponentType<{ className?: string; size?: number }>
  tag: string
  sticker?: string
}

const NAV_ITEMS: NavItemDef[] = [
  { index: 0, label: 'Dashboard', icon: LayoutDashboard, tag: 'HOME_SYS' },
  { index: 1, label: 'Labs Catalog', icon: FlaskConical, tag: 'OWASP_25', sticker: '/assets/sticker_pink.png' },
  { index: 2, label: 'Scoreboard', icon: Trophy, tag: 'CTF_RANKS', sticker: '/assets/sticker_duck.png' },
  { index: 3, label: 'Security Tools', icon: Wrench, tag: 'SEC_TOOLS', sticker: '/assets/sticker_cat_laptop.png' },
  { index: 4, label: 'System Environment', icon: Cpu, tag: 'SYS_DIAG', sticker: '/assets/sticker_bunny.png' },
  { index: 5, label: 'Settings', icon: Settings, tag: 'PREFERENCES', sticker: '/assets/sticker_shiba.png' },
]

export function Sidebar({ selectedIndex, onSelect, onOpenAuth }: SidebarProps) {
  const [currentUser, setCurrentUser] = useState<UserModel | null>(getCurrentUser())
  const [isMenuVisible, setIsMenuVisible] = useState(true)
  const [videoError, setVideoError] = useState(false)

  useEffect(() => {
    return subscribeAuth((user) => setCurrentUser(user))
  }, [])

  const activeAvatar = getAvatarById(currentUser?.avatarId ?? 'zeta')

  return (
    <aside className="w-[270px] shrink-0 h-full relative border-r-[1.5px] border-[#E6E2D8] flex flex-col justify-between overflow-hidden select-none bg-[#F7F5F0]">
      {/* 1. Animated Looping Background (Standard 24 FPS Anime Speed, Smooth & Vivid) */}
      <div className="absolute inset-0 pointer-events-none z-0 overflow-hidden opacity-90">
        {!videoError ? (
          <video
            src="/assets/sidebar_bg.mp4"
            autoPlay
            loop
            muted
            playsInline
            className="w-full h-full object-cover"
            onError={() => setVideoError(true)}
          />
        ) : (
          <picture>
            <source srcSet="/assets/sidebar_bg.webp" type="image/webp" />
            <img
              src="/assets/sidebar_bg.jpg"
              alt="Sidebar Ambient"
              className="w-full h-full object-cover"
              onError={(e) => {
                ;(e.target as HTMLElement).style.display = 'none'
              }}
            />
          </picture>
        )}
        {/* Soft, lightweight ambient overlay (reduced thickness so anime video is vivid & clear) */}
        <div className="absolute inset-0 bg-gradient-to-b from-[#FCFBF8]/25 via-transparent to-[#FCFBF8]/35" />
      </div>

      {/* 2. Top Header & Nav Items */}
      <div className="relative z-10 flex flex-col pt-4">
        {/* Brand Header Card */}
        <div className="px-3.5 mb-3">
          <div className="bg-[#FCFBF8]/95 border-[1.2px] border-[#E6E2D8] rounded-xl p-2.5 shadow-xs flex items-center gap-2.5">
            {/* Avatar Profile Ring */}
            <button
              onClick={onOpenAuth}
              title="Click to view operative profile & avatars"
              className="relative w-11 h-11 rounded-full border-2 border-[#2DD4BF] bg-[#CCFBF1] shadow-[0_0_8px_rgba(45,212,191,0.35)] shrink-0 overflow-hidden cursor-pointer hover:scale-105 transition-transform"
            >
              <img
                src={activeAvatar.assetPath}
                alt={activeAvatar.name}
                className="w-full h-full object-cover"
                onError={(e) => {
                  ;(e.target as HTMLImageElement).src = '/assets/zeta_avatar.png'
                }}
              />
            </button>

            {/* Title / Codename info */}
            <div className="flex-1 min-w-0" onClick={onOpenAuth} role="button">
              <div className="flex items-center gap-1.5">
                <span className="font-['Space_Grotesk'] font-bold text-sm tracking-wide text-[#1E1A14] truncate">
                  {currentUser ? currentUser.username.toUpperCase() : 'ZITERA'}
                </span>
                <span className="px-1.5 py-0.5 rounded bg-[#CCFBF1] border border-[#2DD4BF] text-[9px] font-['Space_Grotesk'] font-bold text-[#0F766E] uppercase">
                  {currentUser ? 'OPERATIVE' : 'LAB'}
                </span>
                {!currentUser && (
                  <div className="w-[20px] h-[20px] rounded-full border border-[#2DD4BF] shadow-[0_0_6px_rgba(45,212,191,0.35)] overflow-hidden shrink-0">
                    <img
                      src="/assets/logo.webp"
                      alt="Zitera"
                      className="w-full h-full object-cover"
                      onError={(e) => {
                        ;(e.target as HTMLImageElement).src = '/assets/logo.png'
                      }}
                    />
                  </div>
                )}
              </div>
              <div className="text-[10px] font-['JetBrains_Mono'] text-[#0F766E] font-semibold truncate tracking-tight">
                {currentUser ? `OPERATIVE // ${activeAvatar.name.toUpperCase()}` : 'CYBER LAB // WS-01'}
              </div>
            </div>

            {/* Menu Visibility Toggle */}
            <button
              onClick={() => setIsMenuVisible(!isMenuVisible)}
              className={`p-1 rounded-md border text-xs transition-colors ${
                isMenuVisible
                  ? 'bg-[#E6FFFA] border-[#2DD4BF] text-[#0F766E]'
                  : 'bg-[#F8FAFC] border-[#E6E2D8] text-[#5C5347]'
              }`}
              title={isMenuVisible ? 'Hide Menu' : 'Show Menu'}
            >
              {isMenuVisible ? <ChevronUp size={16} /> : <ChevronDown size={16} />}
            </button>
          </div>
        </div>

        {/* Collapsible Nav Links */}
        {isMenuVisible ? (
          <nav className="flex flex-col px-3.5 space-y-1">
            {NAV_ITEMS.map((item) => {
              const Icon = item.icon
              const isSelected = selectedIndex === item.index
              return (
                <button
                  key={item.index}
                  onClick={() => onSelect(item.index)}
                  className={`w-full flex items-center justify-between px-3 py-2 rounded-lg text-xs transition-all duration-150 border cursor-pointer ${
                    isSelected
                      ? 'bg-[#FCFBF8] border-[#2DD4BF] text-[#0F766E] font-bold shadow-[0_2px_8px_rgba(45,212,191,0.2)]'
                      : 'bg-[#FCFBF8]/80 border-[#E6E2D8] text-[#1E1A14] hover:bg-[#F0FDFA] hover:border-[#2DD4BF] hover:text-[#0F766E] hover:translate-x-1'
                  }`}
                >
                  <div className="flex items-center gap-2.5">
                    <Icon size={17} className={isSelected ? 'text-[#0F766E]' : 'text-[#5C5347]'} />
                    <span className="font-['Space_Grotesk'] font-medium">{item.label}</span>
                  </div>

                  {item.sticker ? (
                    <img
                      src={item.sticker}
                      alt=""
                      className="w-7 h-7 object-contain shrink-0"
                      onError={(e) => {
                        ;(e.target as HTMLElement).style.display = 'none'
                      }}
                    />
                  ) : (
                    <span
                      className={`text-[9px] font-['JetBrains_Mono'] font-bold tracking-tight ${
                        isSelected ? 'text-[#0F766E]' : 'text-[#9C9080]'
                      }`}
                    >
                      {item.tag}
                    </span>
                  )}
                </button>
              )
            })}
          </nav>
        ) : (
          <div className="px-3.5 py-2">
            <div className="bg-[#FCFBF8]/85 border border-[#E6E2D8] rounded-lg p-2.5 text-center text-[10px] font-['JetBrains_Mono'] text-[#9C9080] font-bold tracking-wider">
              MENU HIDDEN
            </div>
          </div>
        )}
      </div>

      {/* 3. Bottom Operative Auth Card & Neko Status */}
      <div className="relative z-10 p-3.5 space-y-2.5">
        {/* Operative Auth Pill */}
        <button
          onClick={onOpenAuth}
          className={`w-full flex items-center justify-between px-3 py-2 rounded-xl border-[1.2px] text-left transition-all shadow-xs cursor-pointer ${
            currentUser
              ? 'bg-[#CCFBF1] border-[#2DD4BF] text-[#0F766E]'
              : 'bg-[#FCFBF8]/95 border-[#E6E2D8] hover:border-[#2DD4BF] text-[#1E1A14]'
          }`}
        >
          <div className="flex items-center gap-2.5 min-w-0">
            <div className="w-7 h-7 rounded-full border border-[#0F766E]/40 overflow-hidden bg-[#2DD4BF] flex items-center justify-center shrink-0">
              <img
                src={activeAvatar.assetPath}
                alt=""
                className="w-full h-full object-cover"
                onError={(e) => {
                  ;(e.target as HTMLImageElement).src = '/assets/zeta_avatar.png'
                }}
              />
            </div>
            <div className="min-w-0">
              <div className="text-xs font-['Space_Grotesk'] font-bold truncate">
                {currentUser ? currentUser.username : 'LOGIN TO LAB'}
              </div>
              <div
                className={`text-[9px] font-['JetBrains_Mono'] truncate ${
                  currentUser ? 'text-[#115E59] font-bold' : 'text-[#78716C]'
                }`}
              >
                {currentUser ? 'TURSO CTF ACTIVE' : 'Sync CTF Solves'}
              </div>
            </div>
          </div>
          <ChevronRight size={16} className={currentUser ? 'text-[#0F766E]' : 'text-[#78716C]'} />
        </button>

        {/* Neko Status Card */}
        <div className="bg-[#FCFBF8]/95 border-[1.5px] border-[#E6E2D8] rounded-xl p-2.5 shadow-sm flex items-center gap-3">
          <div className="w-13 h-13 shrink-0 flex items-center justify-center">
            <img
              src="/assets/calico_cat.png"
              alt="Calico Neko"
              className="w-full h-full object-contain"
              onError={(e) => {
                ;(e.target as HTMLElement).style.display = 'none'
              }}
            />
          </div>
          <div className="flex-1 min-w-0">
            <div className="flex items-center justify-between">
              <span className="text-[10px] font-['Space_Grotesk'] font-bold tracking-tight text-[#1E1A14]">
                NEKO STATUS
              </span>
              <span className="text-[11px] font-['JetBrains_Mono'] font-bold text-[#16A34A] tracking-wider animate-pulse">
                READY
              </span>
            </div>
            <div className="text-[10px] font-['JetBrains_Mono'] text-[#5C5347] truncate mt-0.5">
              127.0.0.1 // WS-01
            </div>
          </div>
        </div>
      </div>
    </aside>
  )
}
