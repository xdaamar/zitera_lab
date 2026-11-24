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
import { TacticalGate } from './TacticalGate'
import { ErrorBoundary } from './ErrorBoundary'
import {
  initAuth,
  getCurrentUser,
  subscribeAuth,
  touchSession,
  getSessionLastActive,
  getSessionStartedAt,
  SESSION_MAX_INACTIVE_MS,
  SESSION_MAX_LIFETIME_MS,
  logout,
  type UserModel,
} from './authService'
import { useZoomManager, ZoomFloatingWidget } from './ZoomControls'
import './App.css'

export function App() {
  const [currentUser, setCurrentUser] = useState<UserModel | null>(getCurrentUser())
  const [authInitialized, setAuthInitialized] = useState(false)
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

  const {
    zoom,
    zoomIn,
    zoomOut,
    resetZoom,
    setZoom,
    widgetVisible,
    keepWidgetVisible,
    dismissWidgetAfterDelay,
  } = useZoomManager()

  useEffect(() => {
    const unsub = subscribeAuth((user) => {
      setCurrentUser(user)
    })
    initAuth().finally(() => {
      setAuthInitialized(true)
    })

    // Session activity tracker (12-hour session with inactivity & lifetime reset)
    const handleActivity = () => {
      touchSession()
    }
    const sessionInterval = setInterval(() => {
      const lastActive = getSessionLastActive()
      const startedAt = getSessionStartedAt()
      const now = Date.now()
      if (
        (lastActive > 0 && now - lastActive > SESSION_MAX_INACTIVE_MS) ||
        (startedAt > 0 && now - startedAt > SESSION_MAX_LIFETIME_MS)
      ) {
        logout()
      }
    }, 60000)

    window.addEventListener('mousedown', handleActivity, { passive: true })
    window.addEventListener('keydown', handleActivity, { passive: true })

    // Anti-tamper & anti-inspect keyboard prevention
    const handleKeyDown = (e: KeyboardEvent) => {
      if (
        e.key === 'F12' ||
        (e.ctrlKey && e.shiftKey && ['I', 'i', 'J', 'j', 'C', 'c', 'K', 'k'].includes(e.key)) ||
        (e.ctrlKey && (e.key === 'u' || e.key === 'U' || e.key === 's' || e.key === 'S'))
      ) {
        e.preventDefault()
        e.stopPropagation()
      }
    }
    const handleContextMenu = (e: MouseEvent) => {
      // Prevent browser inspect element context menu
      e.preventDefault()
    }

    window.addEventListener('keydown', handleKeyDown, true)
    window.addEventListener('contextmenu', handleContextMenu, true)

    return () => {
      unsub()
      clearInterval(sessionInterval)
      window.removeEventListener('mousedown', handleActivity)
      window.removeEventListener('keydown', handleActivity)
      window.removeEventListener('keydown', handleKeyDown, true)
      window.removeEventListener('contextmenu', handleContextMenu, true)
    }
  }, [])

  const handleToggleCursor = (enabled: boolean) => {
    setCursorEnabled(enabled)
    try {
      localStorage.setItem('zitera_cursor_enabled', String(enabled))
    } catch {
      /* ignore */
    }
  }

  const handleOpenLab = (labId: string) => {
    setActiveLabId(labId)
  }

  const handleBackFromLab = () => {
    setActiveLabId(null)
  }

  // 1. Mandatory Tactical Gate: If user is not logged in, show Full-Screen Tactical Gate
  if (authInitialized && !currentUser) {
    return <TacticalGate onLoginSuccess={() => {}} />
  }

  return (
    <div className="flex h-full w-full overflow-hidden bg-[#FCFBF8] text-[#1E1A14]">
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

          <ErrorBoundary>
            {activeLabId ? (
              <LabDetailView
                labId={activeLabId}
                onBack={handleBackFromLab}
                onSelectLab={handleOpenLab}
              />
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
                    zoom={zoom}
                    onZoomIn={zoomIn}
                    onZoomOut={zoomOut}
                    onResetZoom={resetZoom}
                    onSetZoom={setZoom}
                  />
                )}
              </>
            )}
          </ErrorBoundary>
        </div>
      </main>

      {/* 4. Quick Zoom Floating Capsule (Ctrl + / - / Wheel / Click) - Auto-hides after zoom */}
      <ZoomFloatingWidget
        zoom={zoom}
        onZoomIn={zoomIn}
        onZoomOut={zoomOut}
        onReset={resetZoom}
        visible={widgetVisible}
        onMouseEnter={keepWidgetVisible}
        onMouseLeave={dismissWidgetAfterDelay}
      />

      {/* 6. Auth & Avatar Modal Dialog */}
      <AuthDialog isOpen={authOpen} onClose={() => setAuthOpen(false)} />
    </div>
  )
}

export default App
