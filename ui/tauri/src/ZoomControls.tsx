import { useState, useEffect, useCallback, useRef } from 'react'
import { ZoomIn, ZoomOut, RotateCcw } from 'lucide-react'

export const MIN_ZOOM = 0.7
export const MAX_ZOOM = 1.8
export const ZOOM_STEP = 0.1

export const ZOOM_PRESETS = [0.75, 0.9, 1.0, 1.1, 1.25, 1.5]

/**
 * Invokes native WebView2 controller zoom via Tauri IPC.
 * This changes the browser zoom factor directly (identical to native browser Ctrl+/Ctrl-).
 * Layout reflows and fills 100% of the window without letterboxing or canvas shrink.
 */
async function setNativeZoom(scale: number): Promise<boolean> {
  try {
    if (typeof window !== 'undefined' && ('__TAURI_INTERNALS__' in window || '__TAURI__' in window)) {
      const { invoke } = await import('@tauri-apps/api/core')
      await invoke('set_app_zoom', { scale })
      return true
    }
  } catch (err) {
    console.warn('[Zoom] Native webview zoom invoke failed:', err)
  }
  return false
}

export function useZoomManager() {
  const [zoom, setZoomState] = useState<number>(() => {
    try {
      const saved = localStorage.getItem('zitera_ui_zoom')
      if (saved) {
        const val = parseFloat(saved)
        if (!isNaN(val) && val >= MIN_ZOOM && val <= MAX_ZOOM) {
          return Math.round(val * 100) / 100
        }
      }
    } catch {
      // ignore
    }
    return 1.0
  })

  const [toastVisible, setToastVisible] = useState(false)
  const toastTimeoutRef = useRef<number | null>(null)

  const applyZoom = useCallback((newZoom: number, showToast = true) => {
    const clamped = Math.max(MIN_ZOOM, Math.min(MAX_ZOOM, Math.round(newZoom * 100) / 100))
    setZoomState(clamped)
    try {
      localStorage.setItem('zitera_ui_zoom', String(clamped))
    } catch {
      // ignore
    }

    setNativeZoom(clamped).then((ok) => {
      if (ok) {
        // Clear any lingering CSS zoom properties
        if (document.documentElement.style.zoom) {
          document.documentElement.style.zoom = ''
        }
        if (document.body.style.zoom) {
          document.body.style.zoom = ''
          document.body.style.width = ''
          document.body.style.height = ''
        }
      } else {
        // Fallback for regular web browsers (dev preview):
        // Scale body while dynamically adjusting width/height so it fills 100% of the viewport seamlessly.
        document.documentElement.style.zoom = ''
        const body = document.body
        body.style.zoom = String(clamped)
        body.style.width = `${(100 / clamped).toFixed(3)}vw`
        body.style.height = `${(100 / clamped).toFixed(3)}vh`
      }
    })

    if (showToast) {
      setToastVisible(true)
      if (toastTimeoutRef.current) {
        window.clearTimeout(toastTimeoutRef.current)
      }
      toastTimeoutRef.current = window.setTimeout(() => {
        setToastVisible(false)
      }, 1200)
    }
  }, [])

  // Apply on mount
  useEffect(() => {
    applyZoom(zoom, false)
  }, [applyZoom, zoom])

  const zoomIn = useCallback(() => {
    applyZoom(zoom + ZOOM_STEP)
  }, [zoom, applyZoom])

  const zoomOut = useCallback(() => {
    applyZoom(zoom - ZOOM_STEP)
  }, [zoom, applyZoom])

  const resetZoom = useCallback(() => {
    applyZoom(1.0)
  }, [applyZoom])

  const setZoom = useCallback(
    (targetZoom: number) => {
      applyZoom(targetZoom)
    },
    [applyZoom]
  )

  // Keyboard and Ctrl+Wheel listeners
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.ctrlKey || e.metaKey) {
        // Zoom in: Ctrl + '=' or '+' or NumpadAdd
        if (e.key === '=' || e.key === '+' || e.code === 'NumpadAdd') {
          e.preventDefault()
          setZoomState((prev) => {
            const next = Math.min(MAX_ZOOM, Math.round((prev + ZOOM_STEP) * 100) / 100)
            applyZoom(next)
            return next
          })
        }
        // Zoom out: Ctrl + '-' or NumpadSubtract
        else if (e.key === '-' || e.key === '_' || e.code === 'NumpadSubtract') {
          e.preventDefault()
          setZoomState((prev) => {
            const next = Math.max(MIN_ZOOM, Math.round((prev - ZOOM_STEP) * 100) / 100)
            applyZoom(next)
            return next
          })
        }
        // Reset zoom: Ctrl + '0' or Numpad0
        else if (e.key === '0' || e.code === 'Numpad0') {
          e.preventDefault()
          applyZoom(1.0)
        }
      }
    }

    const handleWheel = (e: WheelEvent) => {
      if (e.ctrlKey || e.metaKey) {
        e.preventDefault()
        setZoomState((prev) => {
          const step = e.deltaY < 0 ? 0.05 : -0.05
          const next = Math.max(MIN_ZOOM, Math.min(MAX_ZOOM, Math.round((prev + step) * 100) / 100))
          applyZoom(next)
          return next
        })
      }
    }

    window.addEventListener('keydown', handleKeyDown)
    window.addEventListener('wheel', handleWheel, { passive: false })

    return () => {
      window.removeEventListener('keydown', handleKeyDown)
      window.removeEventListener('wheel', handleWheel)
    }
  }, [applyZoom])

  return {
    zoom,
    zoomIn,
    zoomOut,
    resetZoom,
    setZoom,
    toastVisible,
  }
}

interface ZoomFloatingWidgetProps {
  zoom: number
  onZoomIn: () => void
  onZoomOut: () => void
  onReset: () => void
}

export function ZoomFloatingWidget({ zoom, onZoomIn, onZoomOut, onReset }: ZoomFloatingWidgetProps) {
  const percent = Math.round(zoom * 100)

  return (
    <aside
      aria-label="Pengaturan Skala Tampilan"
      className="fixed bottom-4 right-4 z-50 select-none flex items-center gap-1 bg-[#FCFBF8]/95 backdrop-blur-md border-2 border-[#CEC8BF] rounded-full px-2 py-1 shadow-[0_3px_0_0_#E4DFD7] transition-all duration-200"
    >
      <button
        onClick={onZoomOut}
        disabled={zoom <= MIN_ZOOM}
        className="w-7 h-7 flex items-center justify-center rounded-full hover:bg-[#F0EDE8] active:bg-[#E6E2D8] text-[#5C5347] hover:text-[#1E1A14] disabled:opacity-30 disabled:cursor-not-allowed cursor-pointer transition-colors"
        title="Zoom Out (Ctrl -)"
      >
        <ZoomOut size={13} />
      </button>

      <button
        onClick={onReset}
        className="px-2 py-0.5 rounded-full hover:bg-[#CCFBF1] text-[#0F766E] hover:text-[#115E59] font-['JetBrains_Mono'] text-xs font-bold cursor-pointer transition-colors flex items-center gap-1"
        title="Klik untuk Reset ke 100% (Ctrl 0)"
      >
        <span>{percent}%</span>
        {percent !== 100 && (
          <RotateCcw size={10} className="text-[#0F766E]/70 animate-spin-reverse" />
        )}
      </button>

      <button
        onClick={onZoomIn}
        disabled={zoom >= MAX_ZOOM}
        className="w-7 h-7 flex items-center justify-center rounded-full hover:bg-[#F0EDE8] active:bg-[#E6E2D8] text-[#5C5347] hover:text-[#1E1A14] disabled:opacity-30 disabled:cursor-not-allowed cursor-pointer transition-colors"
        title="Zoom In (Ctrl +)"
      >
        <ZoomIn size={13} />
      </button>
    </aside>
  )
}

export function ZoomToast({ zoom, visible }: { zoom: number; visible: boolean }) {
  if (!visible) return null
  const percent = Math.round(zoom * 100)

  return (
    <div
      role="status"
      aria-live="polite"
      className="fixed top-5 right-6 z-50 pointer-events-none select-none transition-all duration-300 animate-in fade-in slide-in-from-top-2"
    >
      <div className="bg-[#1E1A14]/90 backdrop-blur-md text-[#5EEAD4] border border-[#2DD4BF]/40 rounded-xl px-4 py-2 shadow-2xl flex items-center gap-2.5 font-['JetBrains_Mono'] text-xs font-bold">
        <span className="w-2 h-2 rounded-full bg-[#5EEAD4] animate-ping" />
        <span className="text-[#FCFBF8]">DISPLAY ZOOM:</span>
        <span className="text-[#5EEAD4] text-sm">{percent}%</span>
      </div>
    </div>
  )
}
