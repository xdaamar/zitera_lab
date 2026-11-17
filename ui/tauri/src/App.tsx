import { useState, useEffect } from 'react'
import { Sidebar, type NavIndex } from './Sidebar'
import { KawaiiCursor } from './KawaiiCursor'
import { AuthDialog } from './AuthDialog'
import { DashboardView } from './DashboardView'
import { LabsView } from './LabsView'
import { LabDetailView } from './LabDetailView'
import { ScoreboardView } from './ScoreboardView'
import { ToolsView } from './ToolsView'
import { EnvironmentView } from './EnvironmentView'
import { SettingsView } from './SettingsView'
import { initAuth } from './authService'
import './App.css'

export function App() {
  const [navIndex, setNavIndex] = useState<NavIndex>(0)
  const [activeLabId, setActiveLabId] = useState<string | null>(null)
  const [authOpen, setAuthOpen] = useState(false)
  const [cursorEnabled, setCursorEnabled] = useState<boolean>(() => {
    try {
      const stored = localStorage.getItem('zitera_cursor_enabled')
      return stored !== null ? stored === 'true' : true
    } catch {
      return true
    }
  })

  useEffect(() => {
    initAuth()
  }, [])

  const handleToggleCursor = (enabled: boolean) => {
    setCursorEnabled(enabled)
    try {
      localStorage.setItem('zitera_cursor_enabled', String(enabled))
    } catch { /* ignore */ }
  }

  const handleOpenLab = (labId: string) => {
    setActiveLabId(labId)
  }

  const handleBackFromLab = () => {
    setActiveLabId(null)
  }

  return (
    <div className="flex h-screen w-screen overflow-hidden bg-[#FCFBF8] text-[#1E1A14]">
      {/* 1. Custom Kawaii Cat Paw Cursor with sparkle trail */}
      <KawaiiCursor enabled={cursorEnabled} />

      {/* 2. Interactive Anime Collapsible Sidebar */}
      <Sidebar
        selectedIndex={navIndex}
        onSelect={(idx) => {
          setNavIndex(idx)
          setActiveLabId(null) // return to normal view when nav tab changed
        }}
        onOpenAuth={() => setAuthOpen(true)}
      />

      {/* 3. Main Dynamic Content Area with Smooth Tilix-Style Page Entrance */}
      <main className="flex-1 flex flex-col h-full overflow-hidden relative">
        <div
          key={activeLabId ? `lab-${activeLabId}` : `nav-${navIndex}`}
          className="flex-1 flex flex-col h-full overflow-hidden relative animate-page-enter"
        >
          {/* Subtle neon scan-beam sweep line across top border on page change */}
          <div className="absolute top-0 left-0 right-0 h-[2px] z-50 pointer-events-none overflow-hidden">
            <div className="h-full w-1/3 bg-gradient-to-r from-transparent via-[#2DD4BF] to-transparent animate-beam-sweep" />
          </div>

          {activeLabId ? (
            <LabDetailView labId={activeLabId} onBack={handleBackFromLab} />
          ) : (
            <>
              {navIndex === 0 && (
                <DashboardView
                  onNavigate={(idx) => {
                    setNavIndex(idx as NavIndex)
                    setActiveLabId(null)
                  }}
                  onOpenLab={handleOpenLab}
                />
              )}
              {navIndex === 1 && <LabsView onSelectLab={handleOpenLab} />}
              {navIndex === 2 && <ScoreboardView />}
              {navIndex === 3 && <ToolsView />}
              {navIndex === 4 && <EnvironmentView />}
              {navIndex === 5 && (
                <SettingsView
                  cursorEnabled={cursorEnabled}
                  onToggleCursor={handleToggleCursor}
                  onOpenAuth={() => setAuthOpen(true)}
                />
              )}
            </>
          )}
        </div>
      </main>

      {/* 4. Auth & Avatar Modal Dialog */}
      <AuthDialog isOpen={authOpen} onClose={() => setAuthOpen(false)} />
    </div>
  )
}

export default App
