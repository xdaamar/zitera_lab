// Auth Service — ZiteraLab React/Tauri
// Mirrors Flutter auth_service.dart + storage_paths via Tauri IPC

import { tursoExecute } from './tursoClient'

export interface UserModel {
  id: string
  username: string
  createdAt: number
  avatarId: string
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

function generateId(): string {
  const bytes = new Uint8Array(8)
  crypto.getRandomValues(bytes)
  const hex = Array.from(bytes)
    .map((b) => b.toString(16).padStart(2, '0'))
    .join('')
  return `u_${Date.now()}_${hex}`
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

// ── Session persistence via Tauri IPC ────────────────────────────────────────
async function persistSession(user: UserModel): Promise<void> {
  try {
    const { invoke } = await import('@tauri-apps/api/core')
    await invoke('save_auth_session', { session: JSON.stringify(user) })
  } catch {
    try {
      localStorage.setItem('zitera_auth_session', JSON.stringify(user))
    } catch { /* ignore */ }
  }
}

async function clearSession(): Promise<void> {
  try {
    const { invoke } = await import('@tauri-apps/api/core')
    await invoke('clear_auth_session')
  } catch {
    localStorage.removeItem('zitera_auth_session')
  }
}

async function loadSession(): Promise<UserModel | null> {
  try {
    const { invoke } = await import('@tauri-apps/api/core')
    const raw = await invoke<string>('get_auth_session')
    if (raw && raw.trim().length > 0) {
      const data = JSON.parse(raw) as Record<string, unknown>
      const user: UserModel = {
        id: String(data.id ?? ''),
        username: String(data.username ?? ''),
        createdAt: Number(data.created_at ?? data.createdAt ?? 0),
        avatarId: String(data.avatar_id ?? data.avatarId ?? 'zeta'),
      }
      if (user.id && user.username) return user
    }
  } catch {
    try {
      const raw = localStorage.getItem('zitera_auth_session')
      if (raw) {
        const data = JSON.parse(raw) as Record<string, unknown>
        const user: UserModel = {
          id: String(data.id ?? ''),
          username: String(data.username ?? ''),
          createdAt: Number(data.created_at ?? data.createdAt ?? 0),
          avatarId: String(data.avatar_id ?? data.avatarId ?? 'zeta'),
        }
        if (user.id && user.username) return user
      }
    } catch { /* ignore */ }
  }
  return null
}

export async function initAuth(): Promise<void> {
  const user = await loadSession()
  _notify(user)
}

// ── Auth operations ───────────────────────────────────────────────────────────
export async function register(
  username: string,
  password: string,
  avatarId = 'zeta',
): Promise<AuthResult> {
  const cleanUsername = username.trim()
  if (cleanUsername.length < 3)
    return { success: false, message: 'Username must be at least 3 characters long.' }
  if (!/^[a-zA-Z0-9_]+$/.test(cleanUsername))
    return { success: false, message: 'Username can only contain letters, numbers, and underscores.' }
  if (password.length < 4)
    return { success: false, message: 'Password must be at least 4 characters long.' }

  try {
    const checkRes = await tursoExecute('SELECT id FROM users WHERE username = ?', [cleanUsername])
    if (checkRes.rows.length > 0)
      return { success: false, message: `Username "${cleanUsername}" is already registered.` }

    const userId = generateId()
    const passHash = await hashPassword(password)
    const createdAt = Date.now()
    const cleanAvatarId = avatarId.trim() || 'zeta'

    await tursoExecute(
      'INSERT INTO users (id, username, password_hash, created_at, avatar_id) VALUES (?, ?, ?, ?, ?)',
      [userId, cleanUsername, passHash, createdAt, cleanAvatarId],
    )

    const user: UserModel = { id: userId, username: cleanUsername, createdAt, avatarId: cleanAvatarId }
    await persistSession(user)
    _notify(user)
    return { success: true, message: 'Account registered successfully!', user }
  } catch (e) {
    return { success: false, message: `Registration failed: ${e}` }
  }
}

export async function login(username: string, password: string): Promise<AuthResult> {
  const cleanUsername = username.trim()
  if (!cleanUsername || !password)
    return { success: false, message: 'Username and password are required.' }

  try {
    const res = await tursoExecute(
      'SELECT id, username, password_hash, created_at, avatar_id FROM users WHERE username = ?',
      [cleanUsername],
    )
    if (res.rows.length === 0)
      return { success: false, message: `User "${cleanUsername}" not found.` }

    const row = res.rows[0]
    const storedHash = String(row.password_hash ?? '')
    const ok = await verifyPassword(password, storedHash)
    if (!ok) return { success: false, message: 'Incorrect password.' }

    const rawAvatar = String(row.avatar_id ?? 'zeta')
    const user: UserModel = {
      id: String(row.id ?? ''),
      username: String(row.username ?? cleanUsername),
      createdAt: Number(row.created_at ?? 0),
      avatarId: rawAvatar.trim() || 'zeta',
    }
    await persistSession(user)
    _notify(user)
    return { success: true, message: `Logged in as ${cleanUsername}`, user }
  } catch (e) {
    return { success: false, message: `Login failed: ${e}` }
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
