// IPC Client for ZiteraLab Tauri Rust Backend
// Talks to Tauri commands with fallback mocks for browser dev mode

import { getOfflineLabContent, getMissionBriefing, type MissionBriefing } from './offlineLabData'

export { getOfflineLabContent, getMissionBriefing, type MissionBriefing }

export interface ComponentItem {
  name: string
  installed: boolean
  version?: string
  status: string
  message: string
  recommendation?: string
}

export interface DiagnosticsResult {
  os: ComponentItem
  git: ComponentItem
  wsl: ComponentItem
  docker: ComponentItem
  dockerDaemon: ComponentItem
  powershell: ComponentItem
  memoryGb: number
  diskFreeGb: number
  allReady: boolean
}

export interface ToolItem {
  id: string
  name: string
  category: string
  installed: boolean
  version?: string
  minVersion: string
  status: string
  path?: string
  capabilities: string[]
  installMethod: string
  installGuide: string
}

export interface LabItem {
  id: string
  title: string
  installed: boolean
  running: boolean
  port: number
  url?: string
  version: string
  status: string
  learnReadiness: string
  practiceReadiness: string
  challengeReadiness: string
  recommendedTools: string[]
  hasUpdate: boolean
  latestVersion: string
}

export interface CatalogEntry {
  id: string
  packageId: string
  title: string
  repository: string
  version: string
  difficulty: string
  owasp: string
  standard: string
  standardVersion: string
  categoryId: string
  categoryName: string
  description: string
  shortDescription: string
  securityVersion: number
  minimumCoreVersion: string
  estimatedTime: number
  skills: string[]
  learningObjectives: string[]
  prerequisites: string[]
  installed: boolean
  installedVersion?: string
  hasUpdate: boolean
  latestVersion: string
  defaultPort: number
}

export interface ProgressiveHint {
  tier: number
  type: string
  hint: string
}

export interface LabManifest {
  schemaVersion: number
  id: string
  packageId: string
  slug: string
  title: string
  owasp: string
  standard: string
  standardVersion: string
  categoryId: string
  categoryName: string
  version: string
  securityVersion: number
  minimumCoreVersion: string
  difficulty: string
  runtime: string
  entrypoint: string
  defaultPort: number
  estimatedMinutes: number
  modes: string[]
  shortDescription: string
  skills: string[]
  learningObjectives: string[]
  prerequisites: string[]
}

export interface LabContent {
  manifest: LabManifest
  lessons: Record<string, string>
  challengeObjective: string
  hints: ProgressiveHint[]
}

export interface ChallengeResult {
  labId: string
  status: string
  message: string
}

export interface PracticeVerificationResult {
  labId: string
  status: string
  message: string
}

export interface TerminalResult {
  stdout: string
  stderr: string
  exitCode: number
  cwd: string
}

// ── Mock data for browser preview fallback ─────────────────────────────────────
const MOCK_LABS: LabItem[] = [
  {
    id: 'A01',
    title: 'Broken Access Control',
    installed: true,
    running: false,
    port: 8011,
    url: 'http://127.0.0.1:8011',
    version: '1.0.1',
    status: 'READY',
    learnReadiness: 'READY',
    practiceReadiness: 'READY',
    challengeReadiness: 'READY',
    recommendedTools: ['curl', 'burpsuite', 'browser'],
    hasUpdate: false,
    latestVersion: '1.0.1',
  },
  {
    id: 'A02',
    title: 'Security Misconfiguration',
    installed: true,
    running: false,
    port: 8012,
    url: 'http://127.0.0.1:8012',
    version: '1.1.0',
    status: 'READY',
    learnReadiness: 'READY',
    practiceReadiness: 'READY',
    challengeReadiness: 'READY',
    recommendedTools: ['nmap', 'curl', 'browser'],
    hasUpdate: false,
    latestVersion: '1.1.0',
  },
  {
    id: 'A03',
    title: 'Software Supply Chain Failures',
    installed: true,
    running: false,
    port: 8013,
    url: 'http://127.0.0.1:8013',
    version: '1.0.1',
    status: 'READY',
    learnReadiness: 'READY',
    practiceReadiness: 'READY',
    challengeReadiness: 'READY',
    recommendedTools: ['git', 'curl', 'python'],
    hasUpdate: false,
    latestVersion: '1.0.1',
  },
  {
    id: 'A04',
    title: 'Cryptographic Failures',
    installed: true,
    running: false,
    port: 8014,
    url: 'http://127.0.0.1:8014',
    version: '1.0.0',
    status: 'READY',
    learnReadiness: 'READY',
    practiceReadiness: 'READY',
    challengeReadiness: 'READY',
    recommendedTools: ['hashcat', 'cyberchef', 'python'],
    hasUpdate: false,
    latestVersion: '1.0.0',
  },
  {
    id: 'A05',
    title: 'Injection',
    installed: true,
    running: false,
    port: 8015,
    url: 'http://127.0.0.1:8015',
    version: '1.0.0',
    status: 'READY',
    learnReadiness: 'READY',
    practiceReadiness: 'READY',
    challengeReadiness: 'READY',
    recommendedTools: ['sqlmap', 'burpsuite', 'curl'],
    hasUpdate: false,
    latestVersion: '1.0.0',
  },
  {
    id: 'A06',
    title: 'Insecure Design',
    installed: true,
    running: false,
    port: 8016,
    url: 'http://127.0.0.1:8016',
    version: '1.0.0',
    status: 'READY',
    learnReadiness: 'READY',
    practiceReadiness: 'READY',
    challengeReadiness: 'READY',
    recommendedTools: ['burpsuite', 'ffuf', 'curl'],
    hasUpdate: false,
    latestVersion: '1.0.0',
  },
  {
    id: 'A07',
    title: 'Identification and Authentication Failures',
    installed: true,
    running: false,
    port: 8017,
    url: 'http://127.0.0.1:8017',
    version: '1.0.0',
    status: 'READY',
    learnReadiness: 'READY',
    practiceReadiness: 'READY',
    challengeReadiness: 'READY',
    recommendedTools: ['hydra', 'jwt_tool', 'burpsuite'],
    hasUpdate: false,
    latestVersion: '1.0.0',
  },
  {
    id: 'A08',
    title: 'Software and Data Integrity Failures',
    installed: true,
    running: false,
    port: 8018,
    url: 'http://127.0.0.1:8018',
    version: '1.0.0',
    status: 'READY',
    learnReadiness: 'READY',
    practiceReadiness: 'READY',
    challengeReadiness: 'READY',
    recommendedTools: ['cosign', 'git', 'python'],
    hasUpdate: false,
    latestVersion: '1.0.0',
  },
  {
    id: 'A09',
    title: 'Security Logging and Monitoring Failures',
    installed: true,
    running: false,
    port: 8019,
    url: 'http://127.0.0.1:8019',
    version: '1.0.0',
    status: 'READY',
    learnReadiness: 'READY',
    practiceReadiness: 'READY',
    challengeReadiness: 'READY',
    recommendedTools: ['wireshark', 'jq', 'curl'],
    hasUpdate: false,
    latestVersion: '1.0.0',
  },
  {
    id: 'A10',
    title: 'Mishandling of Exceptional Conditions',
    installed: true,
    running: false,
    port: 8020,
    url: 'http://127.0.0.1:8020',
    version: '1.0.0',
    status: 'READY',
    learnReadiness: 'READY',
    practiceReadiness: 'READY',
    challengeReadiness: 'READY',
    recommendedTools: ['curl', 'burpsuite', 'collaborator'],
    hasUpdate: false,
    latestVersion: '1.0.0',
  },
]

const MOCK_TOOLS: ToolItem[] = [
  {
    id: 'curl',
    name: 'cURL Network Client',
    category: 'Network',
    installed: true,
    version: '8.4.0',
    minVersion: '7.68.0',
    status: 'READY',
    path: 'C:\\Windows\\System32\\curl.exe',
    capabilities: ['HTTP/HTTPS', 'Custom Headers', 'Raw Payloads'],
    installMethod: 'builtin',
    installGuide: 'Pre-installed with modern Windows.',
  },
  {
    id: 'nmap',
    name: 'Nmap Security Scanner',
    category: 'Reconnaissance',
    installed: true,
    version: '7.94',
    minVersion: '7.80',
    status: 'READY',
    path: 'C:\\Program Files (x86)\\Nmap\\nmap.exe',
    capabilities: ['Port Scanning', 'OS Detection', 'NSE Scripts'],
    installMethod: 'winget',
    installGuide: 'winget install Insecure.Nmap',
  },
  {
    id: 'burpsuite',
    name: 'Burp Suite Community',
    category: 'Proxy & Interception',
    installed: true,
    version: '2024.1',
    minVersion: '2023.1',
    status: 'READY',
    path: 'C:\\Program Files\\BurpSuiteCommunity\\BurpSuiteCommunity.exe',
    capabilities: ['HTTP Proxy', 'Repeater', 'Decoder'],
    installMethod: 'winget',
    installGuide: 'winget install PortSwigger.BurpSuite.CommunityEdition',
  },
  {
    id: 'python',
    name: 'Python 3 Runtime',
    category: 'Scripting',
    installed: true,
    version: '3.12.2',
    minVersion: '3.10.0',
    status: 'READY',
    path: 'C:\\Python312\\python.exe',
    capabilities: ['Exploit Scripting', 'Virtualenv', 'Requests'],
    installMethod: 'winget',
    installGuide: 'winget install Python.Python.3.12',
  },
  {
    id: 'git',
    name: 'Git Version Control',
    category: 'SCM',
    installed: true,
    version: '2.44.0',
    minVersion: '2.30.0',
    status: 'READY',
    path: 'C:\\Program Files\\Git\\cmd\\git.exe',
    capabilities: ['Clone', 'Diff', 'History Inspection'],
    installMethod: 'winget',
    installGuide: 'winget install Git.Git',
  },
]

// ── IPC Invoker helper (cached dynamic import) ─────────────────────────────────
let _invokePromise: Promise<typeof import('@tauri-apps/api/core').invoke> | null = null

function getInvoke() {
  if (!_invokePromise) {
    _invokePromise = import('@tauri-apps/api/core').then((m) => m.invoke)
  }
  return _invokePromise
}

async function callTauri<T>(cmd: string, args: Record<string, unknown> = {}): Promise<T> {
  try {
    const invoke = await getInvoke()
    return await invoke<T>(cmd, args)
  } catch (err) {
    console.warn(`[IPC fallback] invoke("${cmd}") failed:`, err)
    throw err
  }
}

// ── Public API methods ────────────────────────────────────────────────────────

function normalizeLabItem(raw: any): LabItem {
  if (!raw) return MOCK_LABS[0]
  const data = raw.data ?? raw
  const id = String(data.id ?? 'A01').toUpperCase()
  const port = Number(data.port ?? 8011)
  const isRunning = Boolean(data.running === true || data.status === 'RUNNING')
  
  let url: string | null = data.url ?? null
  if (isRunning && !url) {
    url = `http://127.0.0.1:${port}/`
  } else if (url && typeof url === 'string') {
    if (!url.startsWith('http://') && !url.startsWith('https://')) {
      url = `http://${url}`
    }
    if (!url.endsWith('/') && !url.includes('?') && !url.includes('#')) {
      url = `${url}/`
    }
  }

  const defaultMock = MOCK_LABS.find((l) => l.id.toUpperCase() === id)

  return {
    id,
    title: data.title ?? defaultMock?.title ?? `Lab ${id}`,
    installed: Boolean(data.installed ?? true),
    running: isRunning,
    port,
    url: url ?? undefined,
    version: data.version ?? defaultMock?.version ?? '1.0.0',
    status: isRunning ? 'RUNNING' : (data.status ?? 'STOPPED'),
    learnReadiness: data.learn_readiness ?? data.learnReadiness ?? defaultMock?.learnReadiness ?? 'READY',
    practiceReadiness: data.practice_readiness ?? data.practiceReadiness ?? defaultMock?.practiceReadiness ?? 'READY',
    challengeReadiness: data.challenge_readiness ?? data.challengeReadiness ?? defaultMock?.challengeReadiness ?? 'READY',
    recommendedTools: data.recommended_tools ?? data.recommendedTools ?? defaultMock?.recommendedTools ?? [],
    hasUpdate: Boolean(data.has_update ?? data.hasUpdate ?? false),
    latestVersion: data.latest_version ?? data.latestVersion ?? data.version ?? defaultMock?.latestVersion ?? '1.0.0',
  }
}

export async function getLabs(): Promise<LabItem[]> {
  try {
    const data = await callTauri<LabItem[] | string>('get_labs')
    let parsed: any = data
    if (typeof data === 'string') {
      try {
        parsed = JSON.parse(data)
      } catch {
        parsed = data
      }
    }
    const rawList = Array.isArray(parsed)
      ? parsed
      : Array.isArray(parsed?.data)
        ? parsed.data
        : Array.isArray(parsed?.labs)
          ? parsed.labs
          : MOCK_LABS
    return rawList.map(normalizeLabItem)
  } catch {
    return MOCK_LABS
  }
}

export async function getLabStatus(labId: string): Promise<LabItem> {
  try {
    const data = await callTauri<LabItem | string>('get_lab_status', { labId })
    let parsed: any = data
    if (typeof data === 'string') {
      try {
        parsed = JSON.parse(data)
      } catch {
        parsed = data
      }
    }
    const rawObj = parsed?.data ?? parsed
    return normalizeLabItem(rawObj)
  } catch {
    const found = MOCK_LABS.find((l) => l.id.toUpperCase() === labId.toUpperCase())
    return found ?? MOCK_LABS[0]
  }
}

export async function startLab(labId: string): Promise<string> {
  try {
    const res = await callTauri<string>('start_lab', { labId })
    if (typeof res === 'string') {
      try {
        const parsed = JSON.parse(res)
        return parsed.data || parsed.message || res
      } catch {
        return res
      }
    }
    return String(res)
  } catch {
    const lab = MOCK_LABS.find((l) => l.id.toUpperCase() === labId.toUpperCase())
    if (lab) lab.running = true
    return `[MOCK] Lab ${labId} started on port ${lab?.port ?? 8011}`
  }
}

export async function stopLab(labId: string): Promise<string> {
  try {
    const res = await callTauri<string>('stop_lab', { labId })
    if (typeof res === 'string') {
      try {
        const parsed = JSON.parse(res)
        return parsed.data || parsed.message || res
      } catch {
        return res
      }
    }
    return String(res)
  } catch {
    const lab = MOCK_LABS.find((l) => l.id.toUpperCase() === labId.toUpperCase())
    if (lab) lab.running = false
    return `[MOCK] Lab ${labId} stopped.`
  }
}

export async function resetLab(labId: string): Promise<string> {
  try {
    return await callTauri<string>('reset_lab', { labId })
  } catch {
    return `[MOCK] Lab ${labId} reset to initial state.`
  }
}

export async function installLab(labId: string): Promise<string> {
  try {
    return await callTauri<string>('install_lab', { labId })
  } catch {
    return `[MOCK] Lab ${labId} installed successfully.`
  }
}

export async function updateLab(labId: string): Promise<string> {
  try {
    return await callTauri<string>('update_lab', { labId })
  } catch {
    return `[MOCK] Lab ${labId} updated successfully.`
  }
}

export async function getCatalog(): Promise<CatalogEntry[]> {
  try {
    const data = await callTauri<CatalogEntry[] | string>('get_catalog')
    if (typeof data === 'string') {
      const parsed = JSON.parse(data)
      return Array.isArray(parsed) ? parsed : parsed.labs ?? []
    }
    return Array.isArray(data) ? data : []
  } catch {
    return MOCK_LABS.map((lab) => ({
      id: lab.id,
      packageId: `zitera-lab-${lab.id.toLowerCase()}`,
      title: lab.title,
      repository: 'https://github.com/zitera/zitera-labs',
      version: lab.version,
      difficulty: 'Beginner',
      owasp: `A0${lab.id.replace('A', '')}:2025`,
      standard: 'owasp-top10',
      standardVersion: '2025',
      categoryId: lab.id,
      categoryName: lab.title,
      description: `Comprehensive vulnerability challenge for ${lab.title}.`,
      shortDescription: `Interactive sandbox lab for ${lab.title}.`,
      securityVersion: 1,
      minimumCoreVersion: '2.0.0',
      estimatedTime: 45,
      skills: ['Exploitation', 'Remediation', 'Code Review'],
      learningObjectives: ['Understand root cause', 'Exploit vulnerability', 'Implement mitigation'],
      prerequisites: ['Basic HTTP', 'Web Dev Basics'],
      installed: lab.installed,
      hasUpdate: false,
      latestVersion: lab.version,
      defaultPort: lab.port,
    }))
  }
}

export async function getLabContent(labId: string): Promise<LabContent> {
  const offline = getOfflineLabContent(labId)
  try {
    const data = await callTauri<LabContent | string>('get_lab_content', { labId })
    if (typeof data === 'string') {
      const parsed = JSON.parse(data)
      const contentData = parsed.data ?? parsed
      if (contentData && (contentData.manifest || contentData.lessons)) {
        const lessons = contentData.lessons ?? {}
        const resolvedLessons =
          Object.keys(lessons).length > 0 ? { ...offline.lessons, ...lessons } : offline.lessons
        return {
          manifest: contentData.manifest ?? offline.manifest,
          lessons: resolvedLessons,
          challengeObjective:
            contentData.challenge_objective ??
            contentData.challengeObjective ??
            offline.challengeObjective,
          hints:
            contentData.hints && contentData.hints.length > 0
              ? contentData.hints
              : offline.hints,
        }
      }
      return parsed as LabContent
    }
    const rawObj = data as {
      data?: {
        manifest?: LabContent['manifest']
        lessons?: Record<string, string>
        challenge_objective?: string
        challengeObjective?: string
        hints?: ProgressiveHint[]
      }
    }
    if (rawObj && rawObj.data) {
      const lessons = rawObj.data.lessons ?? {}
      const resolvedLessons =
        Object.keys(lessons).length > 0 ? { ...offline.lessons, ...lessons } : offline.lessons
      return {
        manifest: rawObj.data.manifest ?? offline.manifest,
        lessons: resolvedLessons,
        challengeObjective:
          rawObj.data.challenge_objective ??
          rawObj.data.challengeObjective ??
          offline.challengeObjective,
        hints:
          rawObj.data.hints && rawObj.data.hints.length > 0
            ? rawObj.data.hints
            : offline.hints,
      }
    }
    return data
  } catch {
    return offline
  }
}

export async function openBrowser(url: string): Promise<void> {
  let targetUrl = url.trim()
  if (!targetUrl.startsWith('http://') && !targetUrl.startsWith('https://')) {
    targetUrl = `http://${targetUrl}`
  }
  // Ensure trailing slash for bare host:port without paths, query params or fragments
  try {
    const parsed = new URL(targetUrl)
    if (parsed.pathname === '') {
      parsed.pathname = '/'
      targetUrl = parsed.toString()
    }
  } catch {
    // If standard URL parsing fails, proceed with targetUrl
  }

  try {
    await callTauri<void>('open_browser', { url: targetUrl })
  } catch {
    window.open(targetUrl, '_blank')
  }
}

export async function checkDoctor(): Promise<DiagnosticsResult> {
  try {
    const data = await callTauri<DiagnosticsResult | string>('check_doctor')
    if (typeof data === 'string') return JSON.parse(data) as DiagnosticsResult
    return data
  } catch {
    return {
      os: { name: 'Windows 11', installed: true, version: '10.0.22631', status: 'READY', message: 'Supported Windows version' },
      git: { name: 'Git SCM', installed: true, version: '2.44.0', status: 'READY', message: 'Git installed and available' },
      wsl: { name: 'WSL 2', installed: true, version: '2.0.9', status: 'READY', message: 'WSL2 subsystem active' },
      docker: { name: 'Docker CLI', installed: true, version: '26.0.0', status: 'READY', message: 'Docker CLI detected' },
      dockerDaemon: { name: 'Docker Daemon', installed: true, version: '26.0.0', status: 'READY', message: 'Daemon responsive' },
      powershell: { name: 'PowerShell', installed: true, version: '7.4.1', status: 'READY', message: 'Modern PowerShell ready' },
      memoryGb: 16.0,
      diskFreeGb: 124.5,
      allReady: true,
    }
  }
}

export async function getTools(): Promise<ToolItem[]> {
  try {
    const data = await callTauri<ToolItem[] | string>('get_tools')
    if (typeof data === 'string') {
      const parsed = JSON.parse(data)
      return Array.isArray(parsed) ? parsed : MOCK_TOOLS
    }
    return Array.isArray(data) ? data : MOCK_TOOLS
  } catch {
    return MOCK_TOOLS
  }
}

export async function executeTerminal(command: string, labId?: string): Promise<TerminalResult> {
  try {
    return await callTauri<TerminalResult>('execute_terminal', { command, labId })
  } catch {
    // Client-side mock terminal responses
    const cmd = command.trim()
    if (cmd === 'help') {
      return {
        stdout: `Zitera Tilix Terminal v2.0\nAvailable commands:\n  help          - Show this help\n  curl <url>    - Make an HTTP request\n  status        - Check lab status\n  clear         - Clear terminal screen\n  whoami        - Print active user`,
        stderr: '',
        exitCode: 0,
        cwd: labId ? `/labs/${labId}` : '/workspace',
      }
    }
    if (cmd.startsWith('curl')) {
      return {
        stdout: `HTTP/1.1 200 OK\nServer: Zitera-Broker/2.0\nContent-Type: application/json\n\n{\n  "status": "online",\n  "service": "${labId ?? 'A01'}",\n  "hint": "Try visiting /admin/secret"\n}`,
        stderr: '',
        exitCode: 0,
        cwd: labId ? `/labs/${labId}` : '/workspace',
      }
    }
    return {
      stdout: `Executed: ${command}\nOutput: Process returned status 0`,
      stderr: '',
      exitCode: 0,
      cwd: labId ? `/labs/${labId}` : '/workspace',
    }
  }
}

export async function validateChallenge(labId: string, flag: string): Promise<ChallengeResult> {
  try {
    const data = await callTauri<ChallengeResult | string>('validate_challenge', { labId, flag })
    if (typeof data === 'string') return JSON.parse(data) as ChallengeResult
    return data
  } catch {
    const cleanFlag = flag.trim()
    const valid =
      cleanFlag.toUpperCase().includes('FLAG{') ||
      cleanFlag.toUpperCase().includes('ZITERA{') ||
      cleanFlag.length > 8
    if (valid) {
      return { labId, status: 'passed', message: 'CTF flag successfully verified!' }
    }
    return { labId, status: 'failed', message: 'Invalid flag. Check the objective and try again!' }
  }
}
