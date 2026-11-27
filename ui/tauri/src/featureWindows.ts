// ZITERA CYBER_LAB — Native auxiliary window bridge
// Keeps Tanya Mentor and Burp Suite Lite in real OS windows that can be dragged,
// resized, and moved to another monitor instead of being trapped inside the main UI.

import { invoke } from '@tauri-apps/api/core'
import { emit, emitTo, listen, type UnlistenFn } from '@tauri-apps/api/event'
import { getAllWindows } from '@tauri-apps/api/window'
import { WebviewWindow, getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow'
import { PhysicalPosition } from '@tauri-apps/api/dpi'

export type FeatureWindowKind = 'mentor' | 'burp'

export interface FeatureWindowContext {
  labId: string
  labTitle: string
  port: number
  tier: string
}

export interface FeatureWindowSignal {
  kind: FeatureWindowKind
  open: boolean
}

const OPEN_EVENT = 'zitera:feature-window'
const CONTEXT_EVENT = 'zitera:feature-context'
const INJECT_EVENT = 'zitera:inject-repeater'
const PENDING_INJECT_KEY = 'zitera_pending_repeater_payload'

let runtimeAvailable: boolean | null = null

export function isNativeRuntime(): boolean {
  if (runtimeAvailable !== null) return runtimeAvailable
  runtimeAvailable =
    typeof window !== 'undefined' && ('__TAURI_INTERNALS__' in window || '__TAURI__' in window)
  return runtimeAvailable
}

interface FeatureWindowSpec {
  title: string
  width: number
  height: number
  minWidth: number
  minHeight: number
  offsetX: number
  offsetY: number
}

const FEATURE_WINDOW_SPECS: Record<FeatureWindowKind, FeatureWindowSpec> = {
  mentor: {
    title: 'ZITERA // TANYA MENTOR',
    width: 1060,
    height: 840,
    minWidth: 560,
    minHeight: 560,
    offsetX: -72,
    offsetY: -24,
  },
  burp: {
    title: 'ZITERA // BURP SUITE LITE',
    width: 1180,
    height: 780,
    minWidth: 720,
    minHeight: 520,
    offsetX: 64,
    offsetY: 40,
  },
}

function featureLabel(kind: FeatureWindowKind): string {
  return `zitera-${kind}`
}

export async function openFeatureWindow(
  kind: FeatureWindowKind,
  context: FeatureWindowContext,
): Promise<void> {
  if (!isNativeRuntime()) return

  await invoke('set_feature_window_context', { context })
  const label = featureLabel(kind)
  const existing = await WebviewWindow.getByLabel(label)

  if (existing) {
    await emitTo(label, CONTEXT_EVENT, context).catch(() => {})
    await existing.show().catch(() => {})
    await existing.unminimize().catch(() => {})
    await existing.setFocus().catch(() => {})
    return
  }

  const spec = FEATURE_WINDOW_SPECS[kind]
  const featureWin = new WebviewWindow(label, {
    url: '/',
    title: spec.title,
    width: spec.width,
    height: spec.height,
    minWidth: spec.minWidth,
    minHeight: spec.minHeight,
    center: true,
    resizable: true,
    decorations: true,
    focus: true,
  })

  await new Promise<void>((resolve, reject) => {
    featureWin.once('tauri://created', () => resolve())
    featureWin.once('tauri://error', (event) => {
      reject(new Error(typeof event.payload === 'string' ? event.payload : `Gagal membuka ${kind}.`))
    })
  })

  try {
    const position = await featureWin.outerPosition()
    await featureWin.setPosition(
      new PhysicalPosition(position.x + spec.offsetX, position.y + spec.offsetY),
    )
  } catch {
    /* position adjustment is cosmetic */
  }
  await featureWin.setFocus().catch(() => {})
}

export async function closeFeatureWindow(kind: FeatureWindowKind): Promise<void> {
  if (!isNativeRuntime()) return
  const window = await WebviewWindow.getByLabel(featureLabel(kind))
  if (window) await window.close()
}

export async function syncFeatureWindowContext(context: FeatureWindowContext): Promise<void> {
  if (!isNativeRuntime()) return
  await invoke('set_feature_window_context', { context })
}

export async function loadFeatureWindowContext(): Promise<FeatureWindowContext | null> {
  if (!isNativeRuntime()) return null
  try {
    return await invoke<FeatureWindowContext>('get_feature_window_context')
  } catch {
    return null
  }
}

export function onFeatureWindowSignal(handler: (signal: FeatureWindowSignal) => void): UnlistenFn {
  if (!isNativeRuntime()) return () => {}
  let disposed = false
  const unlisten = listen<FeatureWindowSignal>(OPEN_EVENT, (event) => handler(event.payload))
  return () => {
    if (disposed) return
    disposed = true
    unlisten.then((stop) => stop())
  }
}

export function onFeatureContext(handler: (context: FeatureWindowContext) => void): UnlistenFn {
  if (!isNativeRuntime()) return () => {}
  const unlisten = listen<FeatureWindowContext>(CONTEXT_EVENT, (event) => handler(event.payload))
  return () => {
    unlisten.then((stop) => stop())
  }
}

export async function emitRepeaterPayload(detail: {
  payload: string
  targetParam?: string
  path?: string
}): Promise<void> {
  try {
    localStorage.setItem(PENDING_INJECT_KEY, JSON.stringify(detail))
  } catch {
    /* storage unavailable */
  }
  window.dispatchEvent(new CustomEvent(INJECT_EVENT, { detail }))
  if (!isNativeRuntime()) return
  await emit(INJECT_EVENT, detail)
}

export function consumePendingRepeaterPayload(): {
  payload: string
  targetParam?: string
  path?: string
} | null {
  try {
    const raw = localStorage.getItem(PENDING_INJECT_KEY)
    if (!raw) return null
    localStorage.removeItem(PENDING_INJECT_KEY)
    return JSON.parse(raw) as { payload: string; targetParam?: string; path?: string }
  } catch {
    return null
  }
}

export function clearPendingRepeaterPayload(): void {
  try {
    localStorage.removeItem(PENDING_INJECT_KEY)
  } catch {
    /* storage unavailable */
  }
}

export function subscribeRepeaterPayload(
  handler: (detail: { payload: string; targetParam?: string; path?: string }) => void,
): UnlistenFn {
  if (!isNativeRuntime() || currentWindowLabel() === 'main') return () => {}
  const unlisten = listen<{ payload: string; targetParam?: string; path?: string }>(
    INJECT_EVENT,
    (event) => handler(event.payload),
  )
  return () => {
    unlisten.then((stop) => stop())
  }
}

export function currentWindowLabel(): string {
  if (!isNativeRuntime()) return 'main'
  return getCurrentWebviewWindow().label
}

export async function describeFeatureWindows(): Promise<{ label: string; kind: FeatureWindowKind }[]> {
  if (!isNativeRuntime()) return []
  const windows = await getAllWindows()
  return windows
    .map((w) => w.label)
    .filter((label) => label.startsWith('zitera-'))
    .map((label) => ({
      label,
      kind: label.replace('zitera-', '') as FeatureWindowKind,
    }))
}