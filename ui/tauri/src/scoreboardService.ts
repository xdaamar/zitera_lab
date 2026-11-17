// Scoreboard Service — ZiteraLab React/Tauri
import { tursoExecute } from './tursoClient'
import { getCurrentUser } from './authService'

export interface ScoreboardEntry {
  rank: number
  userId: string
  username: string
  labsSolved: number
  totalPoints: number
  lastSolveAt: Date | null
  avatarId: string
  isCurrentUser: boolean
}

export const DEFAULT_POINTS_PER_LAB = 100

export async function recordSolve(labId: string, points = DEFAULT_POINTS_PER_LAB): Promise<boolean> {
  const user = getCurrentUser()
  if (!user) return false

  const cleanLabId = labId.trim().toUpperCase()
  const solveId = `s_${user.id}_${cleanLabId}`
  const solvedAt = Date.now()
  const avatarId = user.avatarId || 'zeta'

  try {
    await tursoExecute(
      'INSERT OR IGNORE INTO solves (id, user_id, username, lab_id, points, solved_at, avatar_id) VALUES (?, ?, ?, ?, ?, ?, ?)',
      [solveId, user.id, user.username, cleanLabId, points, solvedAt, avatarId],
    )
    return true
  } catch (e) {
    console.error('[ScoreboardService] Failed to record solve:', e)
    return false
  }
}

export async function getLeaderboard(): Promise<ScoreboardEntry[]> {
  const sql = `
    SELECT
      u.id AS user_id,
      u.username,
      COALESCE(u.avatar_id, 'zeta') AS avatar_id,
      COALESCE(COUNT(s.id), 0) AS labs_solved,
      COALESCE(SUM(s.points), 0) AS total_points,
      MAX(s.solved_at) AS last_solve_at
    FROM users u
    LEFT JOIN solves s ON u.id = s.user_id
    GROUP BY u.id, u.username, u.avatar_id
    ORDER BY total_points DESC, last_solve_at ASC, u.created_at ASC
  `
  const res = await tursoExecute(sql)
  const currentUser = getCurrentUser()

  return res.rows.map((row, i) => {
    const userId = String(row.user_id ?? '')
    const rawLastSolve = row.last_solve_at
    let lastSolveAt: Date | null = null
    if (rawLastSolve !== null && rawLastSolve !== undefined) {
      const ms = Number(rawLastSolve)
      if (!isNaN(ms) && ms > 0) lastSolveAt = new Date(ms)
    }

    return {
      rank: i + 1,
      userId,
      username: String(row.username ?? 'Anonymous'),
      labsSolved: Number(row.labs_solved ?? 0),
      totalPoints: Number(row.total_points ?? 0),
      lastSolveAt,
      avatarId: String(row.avatar_id ?? 'zeta'),
      isCurrentUser: currentUser ? currentUser.id === userId : false,
    }
  })
}

export function formatLastSolve(date: Date | null): string {
  if (!date) return 'No solves yet'
  const diff = Date.now() - date.getTime()
  const secs = Math.floor(diff / 1000)
  if (secs < 60) return `${secs}s ago`
  const mins = Math.floor(secs / 60)
  if (mins < 60) return `${mins}m ago`
  const hours = Math.floor(mins / 60)
  if (hours < 24) return `${hours}h ago`
  return `${Math.floor(hours / 24)}d ago`
}
