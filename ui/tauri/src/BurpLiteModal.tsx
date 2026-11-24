import React, { useState, useEffect, useMemo, useCallback } from 'react'
import {
  X,
  Send,
  Trash2,
  Plus,
  Globe,
  Shield,
  ArrowRightLeft,
  Radio,
  FileCode,
  Copy,
  Check,
  Search,
  Activity,
  Clock,
  Layers,
  RefreshCw,
  AlertCircle,
  CheckCircle2,
  Server,
  Wifi,
  WifiOff,
  Maximize2,
  Minimize2,
  Terminal,
  Filter,
  CornerDownRight,
  Sliders
} from 'lucide-react'

export interface BurpLiteModalProps {
  isOpen: boolean
  onClose: () => void
  activeLabId?: string | null
  activeLabPort?: number | string | null
  zIndex?: number
  onBringToFront?: () => void
}

export type HttpMethod = 'GET' | 'POST' | 'PUT' | 'DELETE' | 'PATCH' | 'HEAD' | 'OPTIONS'

export interface HttpResponse {
  status: number
  statusText: string
  headers: Record<string, string>
  rawHeaders: string
  body: string
  latencyMs: number
  sizeBytes: number
  timestamp: string
  error?: string
}

export interface RepeaterTab {
  id: string
  name: string
  method: HttpMethod
  path: string
  headersText: string
  body: string
  bodyFormat: 'raw' | 'pretty'
  response: HttpResponse | null
  responseSubTab: 'pretty' | 'raw' | 'render' | 'headers'
  isLoading: boolean
}

export interface HistoryItem {
  id: number
  timestamp: string
  method: HttpMethod
  targetUrl: string
  path: string
  status: number
  statusText: string
  sizeBytes: number
  latencyMs: number
  requestHeaders: string
  requestBody: string
  responseHeaders: Record<string, string>
  rawResponseHeaders: string
  responseBody: string
  error?: string
}

const DEFAULT_HEADERS = `Accept: application/json, text/plain, */*
Content-Type: application/json
User-Agent: BurpLite/1.0 (ZiteraLab Security Suite)`

export function BurpLiteModal({
  isOpen,
  onClose,
  activeLabId,
  activeLabPort,
  zIndex = 10000,
  onBringToFront,
}: BurpLiteModalProps) {
  // Floating Draggable Window Position State
  const [position, setPosition] = useState<{ x: number; y: number }>(() => {
    if (typeof window !== 'undefined') {
      const defaultX = Math.max(16, Math.floor((window.innerWidth - 840) / 2) - 80)
      const defaultY = Math.max(16, Math.floor((window.innerHeight - 680) / 2))
      return { x: defaultX, y: defaultY }
    }
    return { x: 30, y: 30 }
  })

  // Drag handler for window title bar
  const handleDragMouseDown = (e: React.MouseEvent) => {
    if (e.button !== 0 || isMaximized) return
    if ((e.target as HTMLElement).closest('button, input, a, select, textarea')) return

    onBringToFront?.()

    const startX = e.clientX
    const startY = e.clientY
    const initialX = position.x
    const initialY = position.y

    const handleMouseMove = (moveEvent: MouseEvent) => {
      const deltaX = moveEvent.clientX - startX
      const deltaY = moveEvent.clientY - startY
      const maxX = Math.max(10, window.innerWidth - 180)
      const maxY = Math.max(10, window.innerHeight - 80)
      setPosition({
        x: Math.min(maxX, Math.max(10, initialX + deltaX)),
        y: Math.min(maxY, Math.max(10, initialY + deltaY)),
      })
    }

    const handleMouseUp = () => {
      window.removeEventListener('mousemove', handleMouseMove)
      window.removeEventListener('mouseup', handleMouseUp)
    }

    window.addEventListener('mousemove', handleMouseMove)
    window.addEventListener('mouseup', handleMouseUp)
  }
  // Navigation tabs: 'Target', 'Proxy (HTTP History)', 'Repeater'
  const [activeNavTab, setActiveNavTab] = useState<'target' | 'proxy' | 'repeater'>('repeater')

  // Target URL input (e.g. http://127.0.0.1:8011)
  const [targetUrl, setTargetUrl] = useState<string>(() => {
    const port = activeLabPort ? String(activeLabPort) : '8011'
    return `http://127.0.0.1:${port}`
  })

  // Target Health Status State
  const [targetStatus, setTargetStatus] = useState<'idle' | 'checking' | 'online' | 'offline'>('idle')
  const [targetLatency, setTargetLatency] = useState<number | null>(null)
  const [targetLastChecked, setTargetLastChecked] = useState<string | null>(null)

  // Repeater State (Tabs)
  const [repeaterTabs, setRepeaterTabs] = useState<RepeaterTab[]>([
    {
      id: 'tab-1',
      name: 'Request 1',
      method: 'GET',
      path: '/',
      headersText: DEFAULT_HEADERS,
      body: '{\n  "action": "test",\n  "version": "1.0"\n}',
      bodyFormat: 'pretty',
      response: null,
      responseSubTab: 'pretty',
      isLoading: false,
    },
  ])
  const [activeTabId, setActiveTabId] = useState<string>('tab-1')

  // HTTP History State
  const [history, setHistory] = useState<HistoryItem[]>([])
  const [historyCounter, setHistoryCounter] = useState<number>(1)
  const [historyFilter, setHistoryFilter] = useState<string>('')
  const [historyMethodFilter, setHistoryMethodFilter] = useState<string>('ALL')

  // UI state
  const [isCopied, setIsCopied] = useState<boolean>(false)
  const [isMaximized, setIsMaximized] = useState<boolean>(false)

  // Sync targetUrl when activeLabPort changes
  const [prevPort, setPrevPort] = useState(activeLabPort)
  if (activeLabPort !== prevPort) {
    setPrevPort(activeLabPort)
    if (activeLabPort) {
      setTargetUrl(`http://127.0.0.1:${activeLabPort}`)
    }
  }

  // Listen for zitera:inject-repeater custom event from Zitera AI Overview
  useEffect(() => {
    const handleInject = (e: Event) => {
      const customEvent = e as CustomEvent<{ payload: string; targetParam?: string; path?: string }>
      if (!customEvent.detail) return
      const { payload, targetParam, path } = customEvent.detail
      const tabPath = path || (targetParam ? `/?${targetParam}=${encodeURIComponent(payload)}` : `/?q=${encodeURIComponent(payload)}`)
      const newId = `tab-inject-${Date.now()}`
      const newTab: RepeaterTab = {
        id: newId,
        name: `AI Payload`,
        method: 'GET',
        path: tabPath,
        headersText: DEFAULT_HEADERS,
        body: payload,
        bodyFormat: 'pretty',
        response: null,
        responseSubTab: 'pretty',
        isLoading: false,
      }
      setRepeaterTabs((prev) => [...prev, newTab])
      setActiveTabId(newId)
      setActiveNavTab('repeater')
    }

    window.addEventListener('zitera:inject-repeater', handleInject)
    return () => window.removeEventListener('zitera:inject-repeater', handleInject)
  }, [])

  // Active Repeater Tab
  const activeTab = useMemo(() => {
    return repeaterTabs.find((t) => t.id === activeTabId) || repeaterTabs[0]
  }, [repeaterTabs, activeTabId])

  // Update active tab field helper
  const updateActiveTab = useCallback(
    (updater: Partial<RepeaterTab> | ((prev: RepeaterTab) => RepeaterTab)) => {
      setRepeaterTabs((prev) =>
        prev.map((tab) => {
          if (tab.id === activeTabId) {
            if (typeof updater === 'function') {
              return updater(tab)
            }
            return { ...tab, ...updater }
          }
          return tab
        })
      )
    },
    [activeTabId]
  )

  // Add new tab
  const handleAddTab = () => {
    const newId = `tab-${Date.now()}`
    const newNum = repeaterTabs.length + 1
    const newTab: RepeaterTab = {
      id: newId,
      name: `Request ${newNum}`,
      method: 'GET',
      path: '/',
      headersText: DEFAULT_HEADERS,
      body: '',
      bodyFormat: 'pretty',
      response: null,
      responseSubTab: 'pretty',
      isLoading: false,
    }
    setRepeaterTabs((prev) => [...prev, newTab])
    setActiveTabId(newId)
  }

  // Remove tab
  const handleRemoveTab = (idToRemove: string, e: React.MouseEvent) => {
    e.stopPropagation()
    if (repeaterTabs.length <= 1) return // Keep at least one tab
    const filtered = repeaterTabs.filter((t) => t.id !== idToRemove)
    setRepeaterTabs(filtered)
    if (activeTabId === idToRemove) {
      setActiveTabId(filtered[filtered.length - 1].id)
    }
  }

  // Parse Headers
  const parseHeaders = (rawHeaders: string): Record<string, string> => {
    const result: Record<string, string> = {}
    const lines = rawHeaders.split('\n')
    for (const line of lines) {
      const trimmed = line.trim()
      if (!trimmed || trimmed.startsWith('#')) continue
      const colonIndex = trimmed.indexOf(':')
      if (colonIndex > 0) {
        const key = trimmed.slice(0, colonIndex).trim()
        const value = trimmed.slice(colonIndex + 1).trim()
        // Skip browser-restricted headers that throw in fetch
        const forbidden = ['host', 'connection', 'content-length', 'cookie2', 'keep-alive']
        if (!forbidden.includes(key.toLowerCase())) {
          result[key] = value
        }
      }
    }
    return result
  }

  // Format Bytes helper
  const formatBytes = (bytes: number): string => {
    if (bytes === 0) return '0 B'
    if (bytes < 1024) return `${bytes} B`
    if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`
    return `${(bytes / (1024 * 1024)).toFixed(2)} MB`
  }

  // Format Status Badge
  const getStatusBadge = (status: number) => {
    if (status >= 200 && status < 300) {
      return 'text-emerald-400 bg-emerald-950/70 border-emerald-600/70'
    }
    if (status >= 300 && status < 400) {
      return 'text-sky-400 bg-sky-950/70 border-sky-600/70'
    }
    if (status >= 400 && status < 500) {
      return 'text-amber-400 bg-amber-950/70 border-amber-600/70'
    }
    if (status >= 500) {
      return 'text-rose-400 bg-rose-950/70 border-rose-600/70'
    }
    return 'text-rose-300 bg-rose-950/80 border-rose-700/80'
  }

  // Method Badge
  const getMethodBadge = (method: HttpMethod) => {
    switch (method) {
      case 'GET':
        return 'text-emerald-400 bg-emerald-950/60 border-emerald-800/80'
      case 'POST':
        return 'text-blue-400 bg-blue-950/60 border-blue-800/80'
      case 'PUT':
        return 'text-amber-400 bg-amber-950/60 border-amber-800/80'
      case 'DELETE':
        return 'text-rose-400 bg-rose-950/60 border-rose-800/80'
      case 'PATCH':
        return 'text-purple-400 bg-purple-950/60 border-purple-800/80'
      default:
        return 'text-slate-400 bg-slate-800/60 border-slate-700'
    }
  }

  // Send Request Execution
  const handleSendRequest = useCallback(async () => {
    if (!activeTab || activeTab.isLoading) return

    updateActiveTab({ isLoading: true })
    const startTime = performance.now()
    const now = new Date()
    const timeFormatted = now.toTimeString().split(' ')[0]

    // Normalize URL
    let fullUrl = targetUrl.trim()
    if (!fullUrl.startsWith('http://') && !fullUrl.startsWith('https://')) {
      fullUrl = `http://${fullUrl}`
    }
    if (fullUrl.endsWith('/')) {
      fullUrl = fullUrl.slice(0, -1)
    }

    const cleanPath = activeTab.path.startsWith('/') ? activeTab.path : `/${activeTab.path}`
    const requestEndpoint = `${fullUrl}${cleanPath}`

    const headersRecord = parseHeaders(activeTab.headersText)
    const options: RequestInit = {
      method: activeTab.method,
      headers: headersRecord,
    }

    if (activeTab.method !== 'GET' && activeTab.method !== 'HEAD' && activeTab.body) {
      options.body = activeTab.body
    }

    try {
      const res = await fetch(requestEndpoint, options)
      const endTime = performance.now()
      const latencyMs = Math.round(endTime - startTime)

      const responseText = await res.text()
      const sizeBytes = new Blob([responseText]).size

      // Collect response headers
      const resHeaders: Record<string, string> = {}
      let rawResHeaders = `HTTP/1.1 ${res.status} ${res.statusText}\n`
      res.headers.forEach((val, key) => {
        resHeaders[key] = val
        rawResHeaders += `${key}: ${val}\n`
      })

      const httpResponse: HttpResponse = {
        status: res.status,
        statusText: res.statusText || (res.status === 200 ? 'OK' : 'Response'),
        headers: resHeaders,
        rawHeaders: rawResHeaders,
        body: responseText,
        latencyMs,
        sizeBytes,
        timestamp: timeFormatted,
      }

      updateActiveTab({
        response: httpResponse,
        isLoading: false,
      })

      // Add to Proxy History
      const historyItem: HistoryItem = {
        id: historyCounter,
        timestamp: timeFormatted,
        method: activeTab.method,
        targetUrl: fullUrl,
        path: cleanPath,
        status: res.status,
        statusText: res.statusText || 'OK',
        sizeBytes,
        latencyMs,
        requestHeaders: activeTab.headersText,
        requestBody: activeTab.body,
        responseHeaders: resHeaders,
        rawResponseHeaders: rawResHeaders,
        responseBody: responseText,
      }

      setHistory((prev) => [historyItem, ...prev])
      setHistoryCounter((prev) => prev + 1)
    } catch (err: unknown) {
      const endTime = performance.now()
      const latencyMs = Math.round(endTime - startTime)
      const errorMessage = err instanceof Error ? err.message : 'Unknown Network Error'

      const errorResponse: HttpResponse = {
        status: 0,
        statusText: 'Connection Failed',
        headers: {},
        rawHeaders: 'HTTP/0.0 0 Connection Failed\nError: Network / CORS / Target Offline\n',
        body: `Error: Unable to connect to target.\n${errorMessage}\n\nPlease check if lab service is active on port: ${activeLabPort || '8011'}`,
        latencyMs,
        sizeBytes: 0,
        timestamp: timeFormatted,
        error: errorMessage,
      }

      updateActiveTab({
        response: errorResponse,
        isLoading: false,
      })

      const historyItem: HistoryItem = {
        id: historyCounter,
        timestamp: timeFormatted,
        method: activeTab.method,
        targetUrl: fullUrl,
        path: cleanPath,
        status: 0,
        statusText: 'Error',
        sizeBytes: 0,
        latencyMs,
        requestHeaders: activeTab.headersText,
        requestBody: activeTab.body,
        responseHeaders: {},
        rawResponseHeaders: errorResponse.rawHeaders,
        responseBody: errorResponse.body,
        error: errorMessage,
      }

      setHistory((prev) => [historyItem, ...prev])
      setHistoryCounter((prev) => prev + 1)
    }
  }, [activeTab, targetUrl, activeLabPort, historyCounter, updateActiveTab])

  // Clear current tab request & response
  const handleClearCurrentTab = () => {
    updateActiveTab({
      path: '/',
      body: '',
      response: null,
    })
  }

  // Ctrl+Space Hotkey Support
  useEffect(() => {
    if (!isOpen) return

    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.ctrlKey && (e.code === 'Space' || e.key === ' ')) {
        e.preventDefault()
        handleSendRequest()
      }
    }

    window.addEventListener('keydown', handleKeyDown)
    return () => window.removeEventListener('keydown', handleKeyDown)
  }, [isOpen, handleSendRequest])

  // Check Target Health
  const checkTargetHealth = useCallback(async () => {
    setTargetStatus('checking')
    const start = performance.now()
    let checkUrl = targetUrl.trim()
    if (!checkUrl.startsWith('http://') && !checkUrl.startsWith('https://')) {
      checkUrl = `http://${checkUrl}`
    }

    try {
      const controller = new AbortController()
      const timeoutId = setTimeout(() => controller.abort(), 3500)
      const res = await fetch(checkUrl, {
        method: 'GET',
        signal: controller.signal,
      })
      clearTimeout(timeoutId)
      const lat = Math.round(performance.now() - start)
      setTargetLatency(lat)
      setTargetStatus(res.ok || res.status < 500 ? 'online' : 'offline')
      setTargetLastChecked(new Date().toLocaleTimeString())
    } catch {
      const lat = Math.round(performance.now() - start)
      setTargetLatency(lat)
      setTargetStatus('offline')
      setTargetLastChecked(new Date().toLocaleTimeString())
    }
  }, [targetUrl])

  // Trigger health check on clicking Target tab or manual refresh
  const handleSwitchToTarget = () => {
    setActiveNavTab('target')
    checkTargetHealth()
  }

  // Load history item into Repeater
  const handleLoadHistoryIntoRepeater = (item: HistoryItem) => {
    const newTabId = `tab-${Date.now()}`
    const newTab: RepeaterTab = {
      id: newTabId,
      name: `#${item.id} ${item.method} ${item.path}`,
      method: item.method,
      path: item.path,
      headersText: item.requestHeaders,
      body: item.requestBody,
      bodyFormat: 'pretty',
      response: {
        status: item.status,
        statusText: item.statusText,
        headers: item.responseHeaders,
        rawHeaders: item.rawResponseHeaders,
        body: item.responseBody,
        latencyMs: item.latencyMs,
        sizeBytes: item.sizeBytes,
        timestamp: item.timestamp,
        error: item.error,
      },
      responseSubTab: 'pretty',
      isLoading: false,
    }

    setRepeaterTabs((prev) => [...prev, newTab])
    setActiveTabId(newTabId)
    setActiveNavTab('repeater')
  }

  // Copy helper
  const handleCopy = (text: string) => {
    navigator.clipboard.writeText(text)
    setIsCopied(true)
    setTimeout(() => setIsCopied(false), 1800)
  }

  // Filtered History
  const filteredHistory = useMemo(() => {
    return history.filter((item) => {
      const matchMethod = historyMethodFilter === 'ALL' || item.method === historyMethodFilter
      const query = historyFilter.toLowerCase().trim()
      if (!query) return matchMethod
      const matchPath = item.path.toLowerCase().includes(query)
      const matchStatus = String(item.status).includes(query)
      const matchMethodText = item.method.toLowerCase().includes(query)
      return matchMethod && (matchPath || matchStatus || matchMethodText)
    })
  }, [history, historyFilter, historyMethodFilter])

  // Format Pretty JSON helper
  const getFormattedPretty = (content: string) => {
    try {
      const parsed = JSON.parse(content)
      return JSON.stringify(parsed, null, 2)
    } catch {
      return content
    }
  }

  if (!isOpen) return null

  return (
    <div
      style={
        isMaximized
          ? { left: '1vw', top: '1vh', width: '98vw', height: '98vh', zIndex }
          : {
              left: `${position.x}px`,
              top: `${position.y}px`,
              width: 'min(840px, 94vw)',
              height: 'min(680px, 86vh)',
              zIndex,
            }
      }
      className="fixed flex flex-col rounded-xl border border-slate-700/80 bg-[#12151D] shadow-[0_20px_60px_rgba(0,0,0,0.85),0_0_30px_rgba(255,102,51,0.15)] overflow-hidden font-sans text-slate-200 select-none animate-in fade-in zoom-in-95 duration-150"
      onClick={() => onBringToFront?.()}
    >
      {/* Title Bar - PortSwigger Burp Suite Style (Draggable Handle) */}
      <div
        onMouseDown={handleDragMouseDown}
        className="flex items-center justify-between px-3.5 py-2.5 bg-[#181C26] border-b border-slate-700/80 shrink-0 cursor-grab active:cursor-grabbing select-none"
      >
          <div className="flex items-center gap-3">
            {/* Burp Suite Iconic Flame / Radio Icon */}
            <div className="flex items-center justify-center w-7 h-7 rounded bg-[#FF6633] text-white shadow-sm shadow-[#FF6633]/30">
              <Radio size={16} className="animate-pulse" />
            </div>
            <div className="flex items-center gap-2">
              <span className="text-sm font-bold tracking-wide text-white font-mono">
                BURP SUITE REPLICA LITE
              </span>
              <span className="px-1.5 py-0.5 text-[10px] font-semibold bg-[#FF6633]/20 text-[#FF8855] border border-[#FF6633]/30 rounded">
                v2026.1-DEV
              </span>
            </div>
            {activeLabId && (
              <div className="hidden md:flex items-center gap-1.5 px-2 py-0.5 rounded bg-slate-800 border border-slate-700 text-xs text-slate-300">
                <Shield size={12} className="text-[#FF6633]" />
                <span className="font-mono text-[11px]">{activeLabId}</span>
              </div>
            )}
          </div>

          <div className="flex items-center gap-1.5">
            <button
              onClick={() => setIsMaximized(!isMaximized)}
              className="p-1.5 rounded hover:bg-slate-800 text-slate-400 hover:text-slate-200 transition-colors"
              title={isMaximized ? 'Restore Size' : 'Maximize'}
            >
              {isMaximized ? <Minimize2 size={15} /> : <Maximize2 size={15} />}
            </button>
            <button
              onClick={onClose}
              className="p-1.5 rounded hover:bg-rose-950/60 text-slate-400 hover:text-rose-400 transition-colors"
              title="Close Modal"
            >
              <X size={17} />
            </button>
          </div>
        </div>

        {/* Primary Navigation Tabs */}
        <div className="flex items-center justify-between px-3 bg-[#161922] border-b border-slate-800 shrink-0">
          <div className="flex items-center gap-1">
            <button
              onClick={handleSwitchToTarget}
              className={`flex items-center gap-2 px-3.5 py-2 text-xs font-semibold tracking-wide border-b-2 transition-all cursor-pointer ${
                activeNavTab === 'target'
                  ? 'border-[#FF6633] text-[#FF6633] bg-[#FF6633]/10'
                  : 'border-transparent text-slate-400 hover:text-slate-200 hover:bg-slate-800/40'
              }`}
            >
              <Globe size={14} />
              <span>Target</span>
            </button>

            <button
              onClick={() => setActiveNavTab('proxy')}
              className={`flex items-center gap-2 px-3.5 py-2 text-xs font-semibold tracking-wide border-b-2 transition-all cursor-pointer ${
                activeNavTab === 'proxy'
                  ? 'border-[#FF6633] text-[#FF6633] bg-[#FF6633]/10'
                  : 'border-transparent text-slate-400 hover:text-slate-200 hover:bg-slate-800/40'
              }`}
            >
              <ArrowRightLeft size={14} />
              <span>Proxy (HTTP History)</span>
              {history.length > 0 && (
                <span className="px-1.5 py-0.2 rounded-full text-[10px] bg-slate-800 text-slate-300 font-mono">
                  {history.length}
                </span>
              )}
            </button>

            <button
              onClick={() => setActiveNavTab('repeater')}
              className={`flex items-center gap-2 px-3.5 py-2 text-xs font-semibold tracking-wide border-b-2 transition-all cursor-pointer ${
                activeNavTab === 'repeater'
                  ? 'border-[#FF6633] text-[#FF6633] bg-[#FF6633]/10'
                  : 'border-transparent text-slate-400 hover:text-slate-200 hover:bg-slate-800/40'
              }`}
            >
              <FileCode size={14} />
              <span>Repeater</span>
            </button>
          </div>

          <div className="hidden sm:flex items-center gap-3 text-xs text-slate-400">
            <div className="flex items-center gap-1.5">
              <span className="text-[11px] text-slate-500 font-mono">Target:</span>
              <span className="font-mono text-slate-300 text-[11px] bg-slate-800/80 px-2 py-0.5 rounded border border-slate-700/60">
                {targetUrl}
              </span>
            </div>
            <div className="flex items-center gap-1 font-mono text-[10px] bg-slate-800/50 px-2 py-0.5 rounded border border-slate-700/40 text-slate-400">
              <span className="text-amber-400 font-bold">Ctrl+Space</span>
              <span>Send</span>
            </div>
          </div>
        </div>

        {/* Tab Content Panes */}
        <div className="flex-1 overflow-hidden flex flex-col bg-[#12151D]">
          {/* ========================================================= */}
          {/* 1. REPEATER TAB */}
          {/* ========================================================= */}
          {activeNavTab === 'repeater' && (
            <div className="flex-1 flex flex-col overflow-hidden">
              {/* Repeater Top Bar: URL, Actions, Request Tabs */}
              <div className="p-2.5 bg-[#171B25] border-b border-slate-800 flex flex-wrap items-center justify-between gap-2 shrink-0">
                {/* Control bar */}
                <div className="flex items-center gap-2 flex-1 min-w-[280px]">
                  <div className="flex items-center gap-1.5 bg-[#0F1218] border border-slate-700 rounded px-2.5 py-1.5 flex-1 shadow-inner">
                    <Globe size={14} className="text-[#FF6633] shrink-0" />
                    <span className="text-xs text-slate-500 font-mono select-none">Host:</span>
                    <input
                      type="text"
                      value={targetUrl}
                      onChange={(e) => setTargetUrl(e.target.value)}
                      placeholder="http://127.0.0.1:8011"
                      className="bg-transparent text-xs font-mono text-slate-200 focus:outline-none w-full"
                    />
                  </div>

                  {/* Send Button with Ctrl+Space label */}
                  <button
                    onClick={handleSendRequest}
                    disabled={activeTab.isLoading}
                    className={`flex items-center gap-2 px-4 py-1.5 rounded font-bold text-xs text-white transition-all cursor-pointer shadow-md ${
                      activeTab.isLoading
                        ? 'bg-[#FF6633]/60 cursor-not-allowed'
                        : 'bg-[#FF6633] hover:bg-[#FF7744] active:scale-[0.98] shadow-[#FF6633]/20'
                    }`}
                    title="Send Request (Ctrl + Space)"
                  >
                    {activeTab.isLoading ? (
                      <RefreshCw size={14} className="animate-spin" />
                    ) : (
                      <Send size={14} />
                    )}
                    <span>Send</span>
                    <span className="text-[10px] opacity-75 font-mono hidden sm:inline">(Ctrl+Space)</span>
                  </button>

                  {/* Clear Button */}
                  <button
                    onClick={handleClearCurrentTab}
                    className="flex items-center gap-1.5 px-3 py-1.5 rounded bg-slate-800 hover:bg-slate-700 text-slate-300 border border-slate-700 text-xs font-medium transition-colors cursor-pointer"
                    title="Clear Request & Response"
                  >
                    <Trash2 size={13} />
                    <span>Clear</span>
                  </button>
                </div>

                {/* Request Tabs Selector */}
                <div className="flex items-center gap-1 overflow-x-auto max-w-full pb-0.5">
                  {repeaterTabs.map((tab) => {
                    const isActive = tab.id === activeTabId
                    return (
                      <div
                        key={tab.id}
                        onClick={() => setActiveTabId(tab.id)}
                        className={`group flex items-center gap-1.5 px-3 py-1 rounded text-xs font-mono border transition-all cursor-pointer ${
                          isActive
                            ? 'bg-[#222938] border-[#FF6633]/60 text-white shadow-sm'
                            : 'bg-slate-850/60 border-slate-800 text-slate-400 hover:text-slate-200 hover:bg-slate-800'
                        }`}
                      >
                        <span className={`w-1.5 h-1.5 rounded-full ${isActive ? 'bg-[#FF6633]' : 'bg-slate-600'}`} />
                        <span className="max-w-[120px] truncate">{tab.name}</span>
                        {repeaterTabs.length > 1 && (
                          <button
                            onClick={(e) => handleRemoveTab(tab.id, e)}
                            className="p-0.5 rounded hover:bg-slate-700 text-slate-400 hover:text-rose-400"
                            title="Close Tab"
                          >
                            <X size={12} />
                          </button>
                        )}
                      </div>
                    )
                  })}

                  <button
                    onClick={handleAddTab}
                    className="p-1.5 rounded bg-slate-800/80 hover:bg-slate-700 border border-slate-700 text-slate-300 transition-colors cursor-pointer"
                    title="New Repeater Tab"
                  >
                    <Plus size={13} />
                  </button>
                </div>
              </div>

              {/* Split Pane: Left (Request) vs Right (Response) */}
              <div className="flex-1 grid grid-cols-1 md:grid-cols-2 divide-y md:divide-y-0 md:divide-x divide-slate-800 overflow-hidden">
                {/* --------------------------------------------------------- */}
                {/* LEFT PANE: REQUEST */}
                {/* --------------------------------------------------------- */}
                <div className="flex flex-col h-full overflow-hidden bg-[#13161F]">
                  {/* Request Header Bar */}
                  <div className="px-3 py-2 bg-[#171B25] border-b border-slate-800 flex items-center justify-between shrink-0">
                    <div className="flex items-center gap-2 flex-1">
                      {/* Method Dropdown */}
                      <select
                        value={activeTab.method}
                        onChange={(e) => updateActiveTab({ method: e.target.value as HttpMethod })}
                        className={`text-xs font-mono font-bold px-2 py-1 rounded border bg-[#0F1218] cursor-pointer focus:outline-none ${getMethodBadge(
                          activeTab.method
                        )}`}
                      >
                        <option value="GET">GET</option>
                        <option value="POST">POST</option>
                        <option value="PUT">PUT</option>
                        <option value="DELETE">DELETE</option>
                        <option value="PATCH">PATCH</option>
                        <option value="HEAD">HEAD</option>
                        <option value="OPTIONS">OPTIONS</option>
                      </select>

                      {/* Path input */}
                      <div className="flex items-center gap-1 bg-[#0F1218] border border-slate-700 rounded px-2 py-1 flex-1">
                        <CornerDownRight size={13} className="text-slate-500" />
                        <input
                          type="text"
                          value={activeTab.path}
                          onChange={(e) => updateActiveTab({ path: e.target.value })}
                          placeholder="/api/endpoint"
                          className="bg-transparent text-xs font-mono text-slate-200 focus:outline-none w-full"
                        />
                      </div>
                    </div>
                  </div>

                  {/* Request Sub-sections */}
                  <div className="flex-1 flex flex-col overflow-hidden">
                    {/* Headers Editor Section */}
                    <div className="flex flex-col h-2/5 border-b border-slate-800 overflow-hidden">
                      <div className="px-3 py-1.5 bg-[#161A24] border-b border-slate-800/80 flex items-center justify-between shrink-0">
                        <div className="flex items-center gap-1.5 text-xs font-semibold text-slate-300">
                          <Sliders size={13} className="text-[#FF6633]" />
                          <span>Request Headers</span>
                        </div>
                        <span className="text-[10px] text-slate-500 font-mono">key: value format</span>
                      </div>
                      <div className="flex-1 p-2 bg-[#0C0E14] overflow-hidden">
                        <textarea
                          value={activeTab.headersText}
                          onChange={(e) => updateActiveTab({ headersText: e.target.value })}
                          placeholder="Header-Name: header-value"
                          className="w-full h-full bg-transparent text-xs font-mono text-slate-300 resize-none focus:outline-none leading-relaxed p-1"
                          spellCheck={false}
                        />
                      </div>
                    </div>

                    {/* Request Body Editor Section */}
                    <div className="flex flex-col h-3/5 overflow-hidden">
                      <div className="px-3 py-1.5 bg-[#161A24] border-b border-slate-800/80 flex items-center justify-between shrink-0">
                        <div className="flex items-center gap-2">
                          <span className="text-xs font-semibold text-slate-300">Request Body</span>
                          {/* Pretty JSON / Raw tabs */}
                          <div className="flex items-center bg-[#0F1218] border border-slate-800 rounded p-0.5">
                            <button
                              onClick={() => {
                                const formatted = getFormattedPretty(activeTab.body)
                                updateActiveTab({ bodyFormat: 'pretty', body: formatted })
                              }}
                              className={`px-2 py-0.5 text-[11px] font-mono rounded cursor-pointer ${
                                activeTab.bodyFormat === 'pretty'
                                  ? 'bg-slate-700 text-white font-bold'
                                  : 'text-slate-400 hover:text-slate-200'
                              }`}
                            >
                              Pretty JSON
                            </button>
                            <button
                              onClick={() => updateActiveTab({ bodyFormat: 'raw' })}
                              className={`px-2 py-0.5 text-[11px] font-mono rounded cursor-pointer ${
                                activeTab.bodyFormat === 'raw'
                                  ? 'bg-slate-700 text-white font-bold'
                                  : 'text-slate-400 hover:text-slate-200'
                              }`}
                            >
                              Raw
                            </button>
                          </div>
                        </div>

                        {activeTab.body && (
                          <button
                            onClick={() => updateActiveTab({ body: '' })}
                            className="text-[11px] text-slate-500 hover:text-rose-400 transition-colors"
                          >
                            Clear Body
                          </button>
                        )}
                      </div>

                      <div className="flex-1 p-2 bg-[#0C0E14] overflow-hidden">
                        <textarea
                          value={activeTab.body}
                          onChange={(e) => updateActiveTab({ body: e.target.value })}
                          placeholder="Paste or write request payload (JSON, text, urlencoded)..."
                          className="w-full h-full bg-transparent text-xs font-mono text-slate-300 resize-none focus:outline-none leading-relaxed p-1"
                          spellCheck={false}
                        />
                      </div>
                    </div>
                  </div>
                </div>

                {/* --------------------------------------------------------- */}
                {/* RIGHT PANE: RESPONSE */}
                {/* --------------------------------------------------------- */}
                <div className="flex flex-col h-full overflow-hidden bg-[#13161F]">
                  {/* Response Top Bar: Status, Latency, Size */}
                  <div className="px-3 py-2 bg-[#171B25] border-b border-slate-800 flex items-center justify-between shrink-0 min-h-[44px]">
                    <div className="flex items-center gap-2">
                      <span className="text-xs font-semibold text-slate-400">Response:</span>
                      {activeTab.response ? (
                        <div className="flex items-center gap-2">
                          {/* HTTP Status indicator */}
                          <div
                            className={`flex items-center gap-1.5 px-2.5 py-0.5 rounded border text-xs font-bold font-mono ${getStatusBadge(
                              activeTab.response.status
                            )}`}
                          >
                            {activeTab.response.status >= 200 && activeTab.response.status < 300 ? (
                              <CheckCircle2 size={12} />
                            ) : (
                              <AlertCircle size={12} />
                            )}
                            <span>
                              {activeTab.response.status} {activeTab.response.statusText}
                            </span>
                          </div>

                          {/* Latency */}
                          <div className="flex items-center gap-1 px-2 py-0.5 rounded bg-slate-800/80 border border-slate-700/60 text-xs font-mono text-slate-300">
                            <Clock size={11} className="text-amber-400" />
                            <span>{activeTab.response.latencyMs}ms</span>
                          </div>

                          {/* Size */}
                          <div className="flex items-center gap-1 px-2 py-0.5 rounded bg-slate-800/80 border border-slate-700/60 text-xs font-mono text-slate-300">
                            <Layers size={11} className="text-cyan-400" />
                            <span>{formatBytes(activeTab.response.sizeBytes)}</span>
                          </div>
                        </div>
                      ) : (
                        <span className="text-xs text-slate-500 font-mono italic">
                          (No response yet)
                        </span>
                      )}
                    </div>

                    {/* Copy Response Button */}
                    {activeTab.response && (
                      <button
                        onClick={() => handleCopy(activeTab.response?.body || '')}
                        className="flex items-center gap-1 px-2 py-1 rounded bg-slate-800 hover:bg-slate-700 text-slate-300 text-xs font-mono transition-colors cursor-pointer"
                        title="Copy Body"
                      >
                        {isCopied ? <Check size={12} className="text-emerald-400" /> : <Copy size={12} />}
                        <span>{isCopied ? 'Copied' : 'Copy'}</span>
                      </button>
                    )}
                  </div>

                  {/* Response Sub-tabs: Pretty, Raw, Render, Headers */}
                  <div className="px-3 bg-[#161A24] border-b border-slate-800 flex items-center gap-1 shrink-0">
                    <button
                      onClick={() => updateActiveTab({ responseSubTab: 'pretty' })}
                      className={`px-3 py-1.5 text-xs font-mono border-b-2 transition-all cursor-pointer ${
                        activeTab.responseSubTab === 'pretty'
                          ? 'border-[#FF6633] text-[#FF6633] font-bold'
                          : 'border-transparent text-slate-400 hover:text-slate-200'
                      }`}
                    >
                      Pretty
                    </button>
                    <button
                      onClick={() => updateActiveTab({ responseSubTab: 'raw' })}
                      className={`px-3 py-1.5 text-xs font-mono border-b-2 transition-all cursor-pointer ${
                        activeTab.responseSubTab === 'raw'
                          ? 'border-[#FF6633] text-[#FF6633] font-bold'
                          : 'border-transparent text-slate-400 hover:text-slate-200'
                      }`}
                    >
                      Raw
                    </button>
                    <button
                      onClick={() => updateActiveTab({ responseSubTab: 'render' })}
                      className={`px-3 py-1.5 text-xs font-mono border-b-2 transition-all cursor-pointer ${
                        activeTab.responseSubTab === 'render'
                          ? 'border-[#FF6633] text-[#FF6633] font-bold'
                          : 'border-transparent text-slate-400 hover:text-slate-200'
                      }`}
                    >
                      Render
                    </button>
                    <button
                      onClick={() => updateActiveTab({ responseSubTab: 'headers' })}
                      className={`px-3 py-1.5 text-xs font-mono border-b-2 transition-all cursor-pointer ${
                        activeTab.responseSubTab === 'headers'
                          ? 'border-[#FF6633] text-[#FF6633] font-bold'
                          : 'border-transparent text-slate-400 hover:text-slate-200'
                      }`}
                    >
                      Headers (
                      {activeTab.response ? Object.keys(activeTab.response.headers).length : 0}
                      )
                    </button>
                  </div>

                  {/* Response Display Area */}
                  <div className="flex-1 p-3 bg-[#0C0E14] overflow-y-auto">
                    {activeTab.isLoading ? (
                      <div className="h-full flex flex-col items-center justify-center gap-3 text-slate-400">
                        <RefreshCw size={26} className="animate-spin text-[#FF6633]" />
                        <span className="text-xs font-mono tracking-wide">
                          Dispatching HTTP Request to {targetUrl}...
                        </span>
                      </div>
                    ) : activeTab.response ? (
                      <>
                        {/* 1. Pretty Tab */}
                        {activeTab.responseSubTab === 'pretty' && (
                          <div className="h-full font-mono text-xs text-slate-200 overflow-auto whitespace-pre-wrap leading-relaxed select-text">
                            {getFormattedPretty(activeTab.response.body)}
                          </div>
                        )}

                        {/* 2. Raw Tab */}
                        {activeTab.responseSubTab === 'raw' && (
                          <div className="h-full font-mono text-xs text-slate-300 overflow-auto whitespace-pre leading-relaxed select-text">
                            <span className="text-amber-400 font-bold">{activeTab.response.rawHeaders}</span>
                            {'\n'}
                            <span className="text-slate-200">{activeTab.response.body}</span>
                          </div>
                        )}

                        {/* 3. Render Tab */}
                        {activeTab.responseSubTab === 'render' && (
                          <div className="h-full w-full rounded border border-slate-800 bg-white overflow-hidden">
                            <iframe
                              srcDoc={activeTab.response.body}
                              title="Rendered Response"
                              sandbox="allow-same-origin"
                              className="w-full h-full border-0"
                            />
                          </div>
                        )}

                        {/* 4. Headers Tab */}
                        {activeTab.responseSubTab === 'headers' && (
                          <div className="space-y-1.5 font-mono text-xs">
                            {Object.entries(activeTab.response.headers).length === 0 ? (
                              <div className="text-slate-500 italic p-4 text-center">
                                No response headers captured
                              </div>
                            ) : (
                              Object.entries(activeTab.response.headers).map(([key, val]) => (
                                <div
                                  key={key}
                                  className="flex items-start gap-2 p-1.5 rounded bg-slate-900/60 border border-slate-800/80"
                                >
                                  <span className="text-[#FF6633] font-semibold select-all shrink-0">
                                    {key}:
                                  </span>
                                  <span className="text-slate-300 select-all break-all">{val}</span>
                                </div>
                              ))
                            )}
                          </div>
                        )}
                      </>
                    ) : (
                      <div className="h-full flex flex-col items-center justify-center gap-2 text-slate-500">
                        <Terminal size={32} className="text-slate-700" />
                        <span className="text-xs font-mono">No request sent on this tab.</span>
                        <span className="text-[11px] text-slate-600">
                          Click &quot;Send&quot; or press <span className="text-[#FF6633] font-mono font-bold">Ctrl+Space</span> to dispatch.
                        </span>
                      </div>
                    )}
                  </div>
                </div>
              </div>
            </div>
          )}

          {/* ========================================================= */}
          {/* 2. PROXY (HTTP HISTORY) TAB */}
          {/* ========================================================= */}
          {activeNavTab === 'proxy' && (
            <div className="flex-1 flex flex-col overflow-hidden">
              {/* History Filter Toolbar */}
              <div className="p-2.5 bg-[#171B25] border-b border-slate-800 flex flex-wrap items-center justify-between gap-2 shrink-0">
                <div className="flex items-center gap-2 flex-1 min-w-[240px]">
                  <div className="flex items-center gap-1.5 bg-[#0F1218] border border-slate-700 rounded px-2.5 py-1.5 flex-1 shadow-inner">
                    <Search size={14} className="text-slate-500 shrink-0" />
                    <input
                      type="text"
                      value={historyFilter}
                      onChange={(e) => setHistoryFilter(e.target.value)}
                      placeholder="Filter history by path, status, or method..."
                      className="bg-transparent text-xs font-mono text-slate-200 focus:outline-none w-full"
                    />
                  </div>

                  {/* Method Filter */}
                  <div className="flex items-center gap-1 bg-[#0F1218] border border-slate-700 rounded px-2 py-1">
                    <Filter size={12} className="text-slate-500" />
                    <select
                      value={historyMethodFilter}
                      onChange={(e) => setHistoryMethodFilter(e.target.value)}
                      className="bg-transparent text-xs font-mono text-slate-300 focus:outline-none cursor-pointer"
                    >
                      <option value="ALL">ALL Methods</option>
                      <option value="GET">GET</option>
                      <option value="POST">POST</option>
                      <option value="PUT">PUT</option>
                      <option value="DELETE">DELETE</option>
                      <option value="PATCH">PATCH</option>
                    </select>
                  </div>
                </div>

                <div className="flex items-center gap-2">
                  <span className="text-xs font-mono text-slate-400">
                    Total: <strong className="text-white">{filteredHistory.length}</strong> items
                  </span>
                  <button
                    onClick={() => setHistory([])}
                    disabled={history.length === 0}
                    className="flex items-center gap-1.5 px-3 py-1.5 rounded bg-slate-800 hover:bg-slate-700 disabled:opacity-40 disabled:cursor-not-allowed text-slate-300 border border-slate-700 text-xs font-medium transition-colors cursor-pointer"
                    title="Clear HTTP History"
                  >
                    <Trash2 size={13} />
                    <span>Clear History</span>
                  </button>
                </div>
              </div>

              {/* History Table */}
              <div className="flex-1 overflow-auto bg-[#0C0E14]">
                <table className="w-full text-left border-collapse text-xs font-mono">
                  <thead className="bg-[#151922] sticky top-0 border-b border-slate-800 text-slate-400 z-10">
                    <tr>
                      <th className="py-2 px-3 w-12 text-center">#</th>
                      <th className="py-2 px-3 w-20">Time</th>
                      <th className="py-2 px-3 w-20">Method</th>
                      <th className="py-2 px-3">Path / URL</th>
                      <th className="py-2 px-3 w-28">Status</th>
                      <th className="py-2 px-3 w-24">Size</th>
                      <th className="py-2 px-3 w-24">Latency</th>
                      <th className="py-2 px-3 w-28 text-center">Action</th>
                    </tr>
                  </thead>
                  <tbody className="divide-y divide-slate-800/60">
                    {filteredHistory.length === 0 ? (
                      <tr>
                        <td colSpan={8} className="py-12 text-center text-slate-500">
                          <div className="flex flex-col items-center justify-center gap-2">
                            <ArrowRightLeft size={32} className="text-slate-700" />
                            <span>No HTTP history requests recorded.</span>
                            <span className="text-[11px] text-slate-600">
                              Send requests from Repeater to record network transactions here.
                            </span>
                          </div>
                        </td>
                      </tr>
                    ) : (
                      filteredHistory.map((item) => (
                        <tr
                          key={item.id}
                          onClick={() => handleLoadHistoryIntoRepeater(item)}
                          className="hover:bg-[#1A202D] cursor-pointer transition-colors group"
                        >
                          <td className="py-2 px-3 text-center text-slate-500 font-bold">{item.id}</td>
                          <td className="py-2 px-3 text-slate-400">{item.timestamp}</td>
                          <td className="py-2 px-3">
                            <span
                              className={`px-1.5 py-0.5 rounded text-[10px] font-bold border ${getMethodBadge(
                                item.method
                              )}`}
                            >
                              {item.method}
                            </span>
                          </td>
                          <td className="py-2 px-3 text-slate-200 font-mono font-medium max-w-[300px] truncate">
                            {item.path}
                          </td>
                          <td className="py-2 px-3">
                            <span
                              className={`px-2 py-0.5 rounded text-[11px] font-bold border inline-flex items-center gap-1 ${getStatusBadge(
                                item.status
                              )}`}
                            >
                              {item.status} {item.statusText}
                            </span>
                          </td>
                          <td className="py-2 px-3 text-slate-400">{formatBytes(item.sizeBytes)}</td>
                          <td className="py-2 px-3 text-amber-400/90">{item.latencyMs}ms</td>
                          <td className="py-2 px-3 text-center">
                            <button
                              onClick={(e) => {
                                e.stopPropagation()
                                handleLoadHistoryIntoRepeater(item)
                              }}
                              className="px-2 py-1 rounded bg-[#FF6633]/20 hover:bg-[#FF6633] text-[#FF8855] hover:text-white border border-[#FF6633]/40 text-[10px] font-bold transition-all cursor-pointer"
                              title="Load in Repeater"
                            >
                              Repeater
                            </button>
                          </td>
                        </tr>
                      ))
                    )}
                  </tbody>
                </table>
              </div>
            </div>
          )}

          {/* ========================================================= */}
          {/* 3. TARGET TAB */}
          {/* ========================================================= */}
          {activeNavTab === 'target' && (
            <div className="flex-1 overflow-y-auto p-4 sm:p-6 bg-[#0E1118] space-y-6">
              {/* Target Scope Card */}
              <div className="rounded-xl border border-slate-800 bg-[#151922] p-5 shadow-lg space-y-4">
                <div className="flex items-center justify-between pb-3 border-b border-slate-800">
                  <div className="flex items-center gap-2.5">
                    <div className="w-8 h-8 rounded-lg bg-[#FF6633]/20 border border-[#FF6633]/30 flex items-center justify-center text-[#FF6633]">
                      <Server size={18} />
                    </div>
                    <div>
                      <h3 className="text-sm font-bold text-white tracking-wide">TARGET CONFIGURATION & SCOPE</h3>
                      <p className="text-xs text-slate-400">
                        Local educational sandbox target configuration & live port listener status.
                      </p>
                    </div>
                  </div>

                  <button
                    onClick={checkTargetHealth}
                    disabled={targetStatus === 'checking'}
                    className="flex items-center gap-1.5 px-3 py-1.5 rounded bg-slate-800 hover:bg-slate-700 border border-slate-700 text-xs font-semibold text-slate-200 transition-colors cursor-pointer"
                  >
                    <RefreshCw size={13} className={targetStatus === 'checking' ? 'animate-spin' : ''} />
                    <span>Check Target Status</span>
                  </button>
                </div>

                {/* Status Badges Grid */}
                <div className="grid grid-cols-1 sm:grid-cols-3 gap-3">
                  {/* Port Status */}
                  <div className="p-3.5 rounded-lg bg-[#10131A] border border-slate-800 flex items-center justify-between">
                    <div>
                      <div className="text-[11px] text-slate-500 font-mono">TARGET PORT</div>
                      <div className="text-base font-bold font-mono text-slate-200">
                        {activeLabPort || '8011'}
                      </div>
                    </div>
                    <div className="flex items-center gap-1.5 px-2.5 py-1 rounded-full text-xs font-mono font-bold border">
                      {targetStatus === 'online' ? (
                        <span className="flex items-center gap-1 text-emerald-400 bg-emerald-950/60 border border-emerald-700/60 px-2 py-0.5 rounded-full">
                          <Wifi size={13} />
                          <span>LISTENING</span>
                        </span>
                      ) : targetStatus === 'checking' ? (
                        <span className="flex items-center gap-1 text-amber-400 bg-amber-950/60 border border-amber-700/60 px-2 py-0.5 rounded-full">
                          <RefreshCw size={13} className="animate-spin" />
                          <span>CHECKING</span>
                        </span>
                      ) : (
                        <span className="flex items-center gap-1 text-rose-400 bg-rose-950/60 border border-rose-700/60 px-2 py-0.5 rounded-full">
                          <WifiOff size={13} />
                          <span>OFFLINE</span>
                        </span>
                      )}
                    </div>
                  </div>

                  {/* Active Lab ID */}
                  <div className="p-3.5 rounded-lg bg-[#10131A] border border-slate-800">
                    <div className="text-[11px] text-slate-500 font-mono">ACTIVE LAB ID</div>
                    <div className="text-sm font-bold font-mono text-[#FF8855] truncate">
                      {activeLabId || 'local-environment'}
                    </div>
                  </div>

                  {/* Ping Latency */}
                  <div className="p-3.5 rounded-lg bg-[#10131A] border border-slate-800">
                    <div className="text-[11px] text-slate-500 font-mono">PING RTT / LAST CHECK</div>
                    <div className="text-sm font-bold font-mono text-slate-200 flex items-center gap-2">
                      <span className="text-amber-400">{targetLatency !== null ? `${targetLatency}ms` : '--'}</span>
                      {targetLastChecked && (
                        <span className="text-[10px] text-slate-500">at {targetLastChecked}</span>
                      )}
                    </div>
                  </div>
                </div>

                {/* Scope Definition Table */}
                <div className="space-y-2 pt-2">
                  <div className="text-xs font-bold text-slate-300 font-mono">DEFINED TARGET SCOPE RULES</div>
                  <div className="overflow-x-auto rounded border border-slate-800 bg-[#10131A]">
                    <table className="w-full text-left text-xs font-mono">
                      <thead className="bg-[#171B25] border-b border-slate-800 text-slate-400">
                        <tr>
                          <th className="p-2.5 w-20">Rule</th>
                          <th className="p-2.5">Prefix / Pattern</th>
                          <th className="p-2.5 w-28">Status</th>
                          <th className="p-2.5 w-24">Port</th>
                        </tr>
                      </thead>
                      <tbody className="divide-y divide-slate-800/60">
                        <tr>
                          <td className="p-2.5 text-slate-400">1</td>
                          <td className="p-2.5 text-slate-200 font-bold">{targetUrl}/*</td>
                          <td className="p-2.5 text-emerald-400">IN SCOPE</td>
                          <td className="p-2.5 text-slate-300">{activeLabPort || '8011'}</td>
                        </tr>
                        <tr>
                          <td className="p-2.5 text-slate-400">2</td>
                          <td className="p-2.5 text-slate-200">http://localhost:{activeLabPort || '8011'}/*</td>
                          <td className="p-2.5 text-emerald-400">IN SCOPE</td>
                          <td className="p-2.5 text-slate-300">{activeLabPort || '8011'}</td>
                        </tr>
                        <tr>
                          <td className="p-2.5 text-slate-400">3</td>
                          <td className="p-2.5 text-slate-500">All External Networks (WAN)</td>
                          <td className="p-2.5 text-rose-400">EXCLUDED</td>
                          <td className="p-2.5 text-slate-500">ANY</td>
                        </tr>
                      </tbody>
                    </table>
                  </div>
                </div>
              </div>

              {/* Discovered / Common Endpoints Site Map */}
              <div className="rounded-xl border border-slate-800 bg-[#151922] p-5 shadow-lg space-y-4">
                <div className="flex items-center justify-between pb-3 border-b border-slate-800">
                  <div className="flex items-center gap-2">
                    <Activity size={16} className="text-[#FF6633]" />
                    <h3 className="text-sm font-bold text-white tracking-wide">SITE MAP & RECONNAISSANCE ENDPOINTS</h3>
                  </div>
                  <span className="text-xs text-slate-400 font-mono">Quick test routes</span>
                </div>

                <div className="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-3">
                  {[
                    { path: '/', desc: 'Root Application Index', method: 'GET' },
                    { path: '/api/health', desc: 'API Health Probe', method: 'GET' },
                    { path: '/login', desc: 'Authentication Gate', method: 'POST' },
                    { path: '/admin', desc: 'Administrative Dashboard', method: 'GET' },
                    { path: '/flag', desc: 'Lab Victory Verification', method: 'GET' },
                    { path: '/robots.txt', desc: 'Search Crawler Directives', method: 'GET' },
                  ].map((route, i) => (
                    <div
                      key={i}
                      className="p-3 rounded-lg bg-[#10131A] border border-slate-800 hover:border-slate-700 flex items-center justify-between gap-2 transition-all"
                    >
                      <div className="min-w-0">
                        <div className="flex items-center gap-1.5">
                          <span
                            className={`px-1 py-0.2 rounded text-[9px] font-bold border ${getMethodBadge(
                              route.method as HttpMethod
                            )}`}
                          >
                            {route.method}
                          </span>
                          <span className="text-xs font-mono font-bold text-slate-200 truncate">
                            {route.path}
                          </span>
                        </div>
                        <div className="text-[11px] text-slate-500 truncate mt-0.5">{route.desc}</div>
                      </div>

                      <button
                        onClick={() => {
                          const newTabId = `tab-${Date.now()}`
                          const newTab: RepeaterTab = {
                            id: newTabId,
                            name: route.path,
                            method: route.method as HttpMethod,
                            path: route.path,
                            headersText: DEFAULT_HEADERS,
                            body: route.method === 'POST' ? '{\n  "user": "admin"\n}' : '',
                            bodyFormat: 'pretty',
                            response: null,
                            responseSubTab: 'pretty',
                            isLoading: false,
                          }
                          setRepeaterTabs((prev) => [...prev, newTab])
                          setActiveTabId(newTabId)
                          setActiveNavTab('repeater')
                        }}
                        className="px-2 py-1 rounded bg-slate-800 hover:bg-[#FF6633] text-slate-300 hover:text-white border border-slate-700 text-[10px] font-mono font-bold shrink-0 transition-colors cursor-pointer"
                        title="Send to Repeater"
                      >
                        Send
                      </button>
                    </div>
                  ))}
                </div>
              </div>
            </div>
          )}
        </div>

        {/* Modal Status Footer Bar */}
        <div className="px-3.5 py-1.5 bg-[#141720] border-t border-slate-800 text-[11px] font-mono text-slate-400 flex items-center justify-between shrink-0">
          <div className="flex items-center gap-3">
            <span className="flex items-center gap-1 text-[#FF6633]">
              <Radio size={12} />
              <span>BurpLite Engine Ready</span>
            </span>
            <span className="hidden sm:inline text-slate-600">|</span>
            <span className="hidden sm:inline text-slate-400">Direct Local Communication (No CA required)</span>
          </div>

          <div className="flex items-center gap-3">
            <span>
              Target:{' '}
              <span className="text-slate-200 font-semibold">{targetUrl}</span>
            </span>
          </div>
        </div>
      </div>
  )
}

export default BurpLiteModal
