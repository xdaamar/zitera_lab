// Auth Service — ZiteraLab React/Tauri
// Secure Enterprise Authentication, Hardware Enclave Binding & Admin Provisioning

import { tursoExecute } from './tursoClient'

export interface UserModel {
  id: string
  username: string
  createdAt: number
  avatarId: string
  role: 'admin' | 'student'
  tier: 'foundation' | 'pro' | 'enterprise'
  deviceId?: string | null
  deviceName?: string | null
  boundAt?: number | null
  isActive?: boolean
  notes?: string | null
}

export interface AuthResult {
  success: boolean
  message: string
  user?: UserModel
}

// ── Session listeners ────────────────────────────────────────────────────────
type Listener = (user: UserModel | null) => void
let _currentUser: UserModel | null = null
const _listeners = new Set<Listener>()

export function getCurrentUser(): UserModel | null {
  return _currentUser
}

export function isLoggedIn(): boolean {
  return _currentUser !== null
}

export function subscribeAuth(fn: Listener): () => void {
  _listeners.add(fn)
  return () => _listeners.delete(fn)
}

function _notify(user: UserModel | null) {
  _currentUser = user
  _listeners.forEach((fn) => fn(user))
}

// ── Session 5-Hour Inactivity Expiry ─────────────────────────────────────────
export const SESSION_MAX_INACTIVE_MS = 5 * 60 * 60 * 1000 // 5 hours

let _lastTouched = 0
export function touchSession(): void {
  const now = Date.now()
  if (now - _lastTouched < 30000) return // Throttled to max once every 30s
  _lastTouched = now
  try {
    localStorage.setItem('zitera_session_last_active', String(now))
  } catch {}
}

export function getSessionLastActive(): number {
  try {
    const val = localStorage.getItem('zitera_session_last_active')
    return val ? Number(val) : 0
  } catch {
    return 0
  }
}

// ── Robust Hardware Device Matching ─────────────────────────────────────────
export function isDeviceMatch(stored?: string | null, current?: string | null): boolean {
  if (!stored || !current) return false
  const s = stored.trim().toLowerCase()
  const c = current.trim().toLowerCase()
  if (s === c) return true
  // Extract MachineGuid component if present (e.g. DESKTOP-ABC_52036371-cb2b-428d-bbc6-e681e3a3a154)
  const extractGuid = (str: string) => {
    const parts = str.split('_')
    return parts.length >= 2 ? parts[parts.length - 1] : str
  }
  const sGuid = extractGuid(s)
  const cGuid = extractGuid(c)
  if (sGuid && cGuid && sGuid.length > 10 && sGuid === cGuid) {
    return true
  }
  return false
}

// ── Device Hardware Fingerprint ──────────────────────────────────────────────
export async function getDeviceFingerprint(): Promise<string> {
  try {
    const { invoke } = await import('@tauri-apps/api/core')
    const fp = await invoke<string>('get_device_fingerprint')
    if (fp && fp.trim().length > 0) return fp.trim()
  } catch {
    // Fallback for development / web preview
  }
  let fallback = localStorage.getItem('zitera_device_fingerprint')
  if (!fallback) {
    const randomHex = Array.from(crypto.getRandomValues(new Uint8Array(16)))
      .map((b) => b.toString(16).padStart(2, '0'))
      .join('')
    fallback = `DEV_MACHINE_${randomHex}`
    localStorage.setItem('zitera_device_fingerprint', fallback)
  }
  return fallback
}

// ── Password hashing (SHA-256 via Web Crypto) ────────────────────────────────
async function sha256hex(message: string): Promise<string> {
  const msgBuffer = new TextEncoder().encode(message)
  const hashBuffer = await crypto.subtle.digest('SHA-256', msgBuffer)
  const hashArray = Array.from(new Uint8Array(hashBuffer))
  return hashArray.map((b) => b.toString(16).padStart(2, '0')).join('')
}

function generateSalt(length = 16): string {
  const bytes = new Uint8Array(length)
  crypto.getRandomValues(bytes)
  return Array.from(bytes)
    .map((b) => b.toString(16).padStart(2, '0'))
    .join('')
}

export async function hashPassword(password: string, existingSalt?: string): Promise<string> {
  const salt = existingSalt ?? generateSalt()
  const digest = await sha256hex(`${salt}:${password}`)
  return `${salt}:${digest}`
}

export async function verifyPassword(password: string, storedHash: string): Promise<boolean> {
  const parts = storedHash.split(':')
  if (parts.length !== 2) return false
  const [salt, expectedHash] = parts
  const actualHash = await sha256hex(`${salt}:${password}`)
  return actualHash === expectedHash
}

// ── Session persistence via Tauri IPC / LocalStorage ─────────────────────────
async function persistSession(user: UserModel): Promise<void> {
  touchSession()
  try {
    const { invoke } = await import('@tauri-apps/api/core')
    await invoke('save_auth_session', { session: JSON.stringify(user) })
  } catch {
    try {
      localStorage.setItem('zitera_auth_session', JSON.stringify(user))
    } catch {
      /* ignore */
    }
  }
}

async function clearSession(): Promise<void> {
  try {
    const { invoke } = await import('@tauri-apps/api/core')
    await invoke('clear_auth_session')
  } catch {
    localStorage.removeItem('zitera_auth_session')
  }
  try {
    localStorage.removeItem('zitera_session_last_active')
  } catch {
    /* ignore */
  }
}

async function loadSession(): Promise<UserModel | null> {
  const lastActive = getSessionLastActive()
  if (lastActive > 0 && Date.now() - lastActive > SESSION_MAX_INACTIVE_MS) {
    console.warn('[authService] Sesi login telah berakhir (> 5 jam tidak aktif). Membuka gerbang masuk kembali.')
    await clearSession()
    return null
  }
  let raw: string | null = null
  try {
    const { invoke } = await import('@tauri-apps/api/core')
    raw = await invoke<string>('get_auth_session')
  } catch {
    raw = localStorage.getItem('zitera_auth_session')
  }

  if (raw && typeof raw === 'object') {
    // If returned as parsed object by Tauri
    raw = JSON.stringify(raw)
  }

  if (raw && typeof raw === 'string' && raw.trim().length > 0) {
    try {
      const data = JSON.parse(raw) as Record<string, unknown>
      const user: UserModel = {
        id: String(data.id ?? ''),
        username: String(data.username ?? ''),
        createdAt: Number(data.created_at ?? data.createdAt ?? 0),
        avatarId: String(data.avatar_id ?? data.avatarId ?? 'zeta'),
        role: (data.role as 'admin' | 'student') || 'student',
        tier: (data.tier as 'foundation' | 'pro' | 'enterprise') || 'pro',
        deviceId: data.device_id ? String(data.device_id) : data.deviceId ? String(data.deviceId) : null,
        deviceName: data.device_name ? String(data.device_name) : data.deviceName ? String(data.deviceName) : null,
        boundAt: data.bound_at ? Number(data.bound_at) : data.boundAt ? Number(data.boundAt) : null,
        isActive: data.is_active !== undefined ? Number(data.is_active) === 1 : data.isActive !== undefined ? !!data.isActive : true,
        notes: data.notes ? String(data.notes) : null,
      }
      if (user.id && user.username) return user
    } catch {
      /* parse error */
    }
  }
  return null
}

export async function verifyServerSession(cachedUser: UserModel): Promise<UserModel | null> {
  try {
    const res = await tursoExecute(
      'SELECT id, username, role, tier, device_id, is_active FROM users WHERE id = ?',
      [cachedUser.id],
    )
    if (res.rows.length === 0) {
      await clearSession()
      _notify(null)
      return null
    }

    const row = res.rows[0]
    const isActive = row.is_active !== undefined && row.is_active !== null ? Number(row.is_active) === 1 : true
    if (!isActive) {
      console.warn('[authService] Account deactivated by administrator.')
      await clearSession()
      _notify(null)
      return null
    }

    const currentFp = await getDeviceFingerprint()
    const storedDeviceId = row.device_id ? String(row.device_id).trim() : ''
    const userRole = (row.role as 'admin' | 'student') || 'student'
    if (userRole !== 'admin' && storedDeviceId && !isDeviceMatch(storedDeviceId, currentFp)) {
      console.warn('[authService] Hardware device mismatch detected.')
      await clearSession()
      _notify(null)
      return null
    }

    touchSession()

    const serverTier = (row.tier as 'foundation' | 'pro' | 'enterprise') || 'foundation'
    const updatedUser: UserModel = {
      ...cachedUser,
      role: userRole,
      tier: serverTier,
      deviceId: storedDeviceId || cachedUser.deviceId,
      isActive: true,
    }

    await persistSession(updatedUser)
    _notify(updatedUser)
    return updatedUser
  } catch (err) {
    console.warn('[authService] Offline or network error during session verification:', err)
    return cachedUser
  }
}

export async function initAuth(): Promise<void> {
  const cached = await loadSession()
  if (!cached) {
    _notify(null)
    return
  }

  // Authoritative server-side verification: Prevents LocalStorage privilege escalation
  await verifyServerSession(cached)
}

// ── Student & Admin Login with Hardware Binding ──────────────────────────────
export async function login(username: string, password: string): Promise<AuthResult> {
  const cleanUsername = username.trim()
  if (!cleanUsername || !password)
    return { success: false, message: 'Username dan Password wajib diisi.' }

  try {
    const res = await tursoExecute(
      'SELECT id, username, password_hash, created_at, avatar_id, role, tier, device_id, device_name, bound_at, is_active, notes FROM users WHERE username = ?',
      [cleanUsername],
    )
    if (res.rows.length === 0)
      return { success: false, message: `Akun "${cleanUsername}" tidak ditemukan. Silakan hubungi Admin.` }

    const row = res.rows[0]
    const storedHash = String(row.password_hash ?? '')
    const ok = await verifyPassword(password, storedHash)
    if (!ok) return { success: false, message: 'Password salah. Periksa kembali kredensial Anda.' }

    const isActive = row.is_active !== undefined && row.is_active !== null ? Number(row.is_active) === 1 : true
    if (!isActive) {
      return {
        success: false,
        message: 'AKSES DIBEKUKAN: Akun ini telah dinonaktifkan oleh Administrator ZITERA LAB.',
      }
    }

    const currentFp = await getDeviceFingerprint()
    const storedDeviceId = row.device_id ? String(row.device_id).trim() : ''
    const userRole = (row.role as 'admin' | 'student') || 'student'
    const userTier = (row.tier as 'foundation' | 'pro' | 'enterprise') || 'pro'
    let finalDeviceId = storedDeviceId
    let finalBoundAt = row.bound_at ? Number(row.bound_at) : null

    // Hardware Enclave Binding check for students
    if (userRole !== 'admin') {
      if (!storedDeviceId) {
        // First login: Auto-bind device
        finalDeviceId = currentFp
        finalBoundAt = Date.now()
        const devName = currentFp.split('_')[0] || 'Win32_Host'
        await tursoExecute(
          'UPDATE users SET device_id = ?, device_name = ?, bound_at = ? WHERE id = ?',
          [currentFp, devName, finalBoundAt, String(row.id)],
        )
      } else if (!isDeviceMatch(storedDeviceId, currentFp)) {
        // Device mismatch: Attempted to run on a different unauthorized machine!
        return {
          success: false,
          message: `AKSES DITOLAK: Akun ini telah terikat ke perangkat lain [${storedDeviceId.substring(0, 14)}...]. Hubungi Admin via WhatsApp untuk pengajuan reset hardware.`,
        }
      }
    }

    touchSession()

    const user: UserModel = {
      id: String(row.id ?? ''),
      username: String(row.username ?? cleanUsername),
      createdAt: Number(row.created_at ?? 0),
      avatarId: String(row.avatar_id ?? 'zeta').trim() || 'zeta',
      role: userRole,
      tier: userTier,
      deviceId: finalDeviceId || null,
      deviceName: row.device_name ? String(row.device_name) : null,
      boundAt: finalBoundAt,
      isActive: true,
      notes: row.notes ? String(row.notes) : null,
    }

    await persistSession(user)
    _notify(user)
    return { success: true, message: `Selamat datang, Operative ${user.username}!`, user }
  } catch (e) {
    return { success: false, message: `Gagal memproses autentikasi: ${e}` }
  }
}

export async function updateAvatar(avatarId: string): Promise<boolean> {
  const user = _currentUser
  if (!user) return false
  const cleanAvatarId = avatarId.trim() || 'zeta'
  try {
    await tursoExecute('UPDATE users SET avatar_id = ? WHERE id = ?', [cleanAvatarId, user.id])
    const updated: UserModel = { ...user, avatarId: cleanAvatarId }
    await persistSession(updated)
    _notify(updated)
    return true
  } catch {
    return false
  }
}

export async function logout(): Promise<void> {
  _notify(null)
  await clearSession()
}
