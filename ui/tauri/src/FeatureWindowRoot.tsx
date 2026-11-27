import { useCallback, useEffect, useState } from 'react'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { emit } from '@tauri-apps/api/event'
import { AlertTriangle, Loader2, RotateCcw } from 'lucide-react'
import { ZetaMentorModal } from './ZetaMentorModal'
import { BurpLiteModal } from './BurpLiteModal'
import {
  loadFeatureWindowContext,
  onFeatureContext,
  openFeatureWindow,
  type FeatureWindowContext,
  type FeatureWindowKind,
} from './featureWindows'
import { getCurrentUser } from './authService'

interface FeatureWindowRootProps {
  kind: FeatureWindowKind
}

export function FeatureWindowRoot({ kind }: FeatureWindowRootProps) {
  const [context, setContext] = useState<FeatureWindowContext | null>(null)
  const [error, setError] = useState<string | null>(null)

  const load = useCallback(async () => {
    setError(null)
    try {
      const next = await loadFeatureWindowContext()
      if (!next || !next.labId) {
        setError('Konteks laboratorium belum tersedia. Buka lab dari jendela utama lalu coba lagi.')
        return
      }
      setContext(next)
    } catch (cause) {
      setError(cause instanceof Error ? cause.message : 'Gagal memuat konteks laboratorium.')
    }
  }, [])

  useEffect(() => {
    void load()
    const stop = onFeatureContext((next) => setContext(next))
    return stop
  }, [load])

  useEffect(() => {
    void emit('zitera:feature-window', { kind, open: true })
  }, [kind])

  const closeWindow = useCallback(() => {
    getCurrentWindow()
      .close()
      .catch(() => {})
  }, [])

  useEffect(() => {
    const handleEscape = (event: KeyboardEvent) => {
      if (event.key !== 'Escape') return
      const target = event.target
      if (target instanceof HTMLElement && target.closest('input, textarea, select, [contenteditable="true"]')) {
        return
      }
      event.preventDefault()
      closeWindow()
    }
    window.addEventListener('keydown', handleEscape)
    return () => window.removeEventListener('keydown', handleEscape)
  }, [closeWindow])

  const openTier = context?.tier || getCurrentUser()?.tier || 'foundation'

  if (error) {
    return (
      <div className="flex h-full w-full flex-col items-center justify-center gap-4 bg-stone-950 px-8 text-center text-stone-200">
        <div className="flex items-center justify-center w-12 h-12 rounded-xl border border-rose-500/40 bg-rose-500/10">
          <AlertTriangle className="w-6 h-6 text-rose-400" aria-hidden="true" />
        </div>
        <div className="space-y-1">
          <p className="font-['Space_Grotesk'] text-base font-bold text-white">
            Jendela tidak bisa memuat konteks
          </p>
          <p className="max-w-sm text-xs leading-relaxed text-stone-400">{error}</p>
        </div>
        <button
          type="button"
          onClick={() => void load()}
          className="inline-flex items-center gap-2 rounded-lg border border-teal-500/40 bg-teal-500/10 px-3.5 py-2 text-xs font-bold text-teal-300 transition-colors hover:bg-teal-500/20 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-teal-400"
        >
          <RotateCcw className="w-3.5 h-3.5" aria-hidden="true" />
          Muat ulang konteks
        </button>
      </div>
    )
  }

  if (!context) {
    return (
      <div className="flex h-full w-full flex-col items-center justify-center gap-3 bg-stone-950 text-stone-300">
        <Loader2 className="w-6 h-6 animate-spin text-teal-400" aria-hidden="true" />
        <p className="font-['JetBrains_Mono'] text-[11px] font-bold tracking-widest uppercase text-stone-400">
          Sinkronisasi konteks lab
        </p>
      </div>
    )
  }

  if (kind === 'mentor') {
    return (
      <ZetaMentorModal
        windowMode
        isOpen
        onClose={closeWindow}
        currentLabId={context.labId}
        userTier={openTier}
        onOpenBurpLite={() => {
          void openFeatureWindow('burp', context)
        }}
      />
    )
  }

  return (
    <BurpLiteModal
      windowMode
      isOpen
      onClose={closeWindow}
      activeLabId={context.labId}
      activeLabPort={context.port}
    />
  )
}