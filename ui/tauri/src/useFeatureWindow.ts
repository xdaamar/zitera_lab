import { useCallback, useEffect, useState } from 'react'
import {
  closeFeatureWindow,
  describeFeatureWindows,
  isNativeRuntime,
  onFeatureWindowSignal,
  openFeatureWindow,
  type FeatureWindowContext,
  type FeatureWindowKind,
} from './featureWindows'

interface FeatureWindowController {
  open: boolean
  fallback: boolean
  show: (context: FeatureWindowContext) => Promise<void>
  hide: () => Promise<void>
}

export function useFeatureWindow(kind: FeatureWindowKind): FeatureWindowController {
  const [open, setOpen] = useState(false)
  const [fallback, setFallback] = useState(false)

  useEffect(() => {
    let active = true

    const reconcile = () => {
      describeFeatureWindows()
        .then((windows) => {
          if (active) setOpen(windows.some((entry) => entry.kind === kind))
        })
        .catch(() => {})
    }

    reconcile()
    const stop = onFeatureWindowSignal((signal) => {
      if (signal.kind === kind) setOpen(signal.open)
    })
    window.addEventListener('focus', reconcile)

    return () => {
      active = false
      stop()
      window.removeEventListener('focus', reconcile)
    }
  }, [kind])

  const show = useCallback(
    async (context: FeatureWindowContext) => {
      if (!isNativeRuntime()) {
        setFallback(true)
        setOpen(true)
        return
      }
      try {
        await openFeatureWindow(kind, context)
        setOpen(true)
      } catch {
        setFallback(true)
        setOpen(true)
      }
    },
    [kind],
  )

  const hide = useCallback(async () => {
    if (!isNativeRuntime() || fallback) {
      setFallback(false)
      setOpen(false)
      return
    }
    try {
      await closeFeatureWindow(kind)
    } finally {
      setOpen(false)
    }
  }, [fallback, kind])

  return { open, fallback, show, hide }
}