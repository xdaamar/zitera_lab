import { useState, useEffect, useRef } from 'react'
import {
  Medal,
  RefreshCw,
  Clock,
  Award,
} from 'lucide-react'
import {
  getLeaderboard,
  formatLastSolve,
  type ScoreboardEntry,
} from './scoreboardService'
import { CuteAnimeLoading } from './CuteAnimeLoading'
import { CertificateModal } from './CertificateModal'
import { getCurrentUser } from './authService'

const LAB_BOUNTY_MAP: Record<string, number> = {
  A01: 5000,
  A02: 8500,
  A03: 4500,
  A04: 7000,
  A05: 6000,
  A06: 3500,
  A07: 4500,
  A08: 6500,
  A09: 2500,
  A10: 6000,
}

export function ScoreboardView() {
  const [entries, setEntries] = useState<ScoreboardEntry[]>([])
  const [loading, setLoading] = useState(true)
  const [certModalOpen, setCertModalOpen] = useState(false)
  const isMountedRef = useRef(true)

  const currentUser = getCurrentUser()
  const studentName = currentUser?.username || 'Security Operative'

  // Calculate local user solves
  const labIds = ['A01', 'A02', 'A03', 'A04', 'A05', 'A06', 'A07', 'A08', 'A09', 'A10']
  const solvedLabs = labIds.filter((id) => {
    try {
      return localStorage.getItem(`zitera_solved_${id}`) === 'true'
    } catch {
      return false
    }
  })
  const currentLeaderboardEntry = entries.find((e) => e.isCurrentUser)
  const solvedCount = Math.max(solvedLabs.length, currentLeaderboardEntry?.labsSolved ?? 0)

  // Calculate total simulated bounty
  let totalBountyEarnedUsd = 0
  for (const id of labIds) {
    let isSolved = false
    try {
      isSolved = localStorage.getItem(`zitera_solved_${id}`) === 'true'
    } catch {}
    if (isSolved) {
      totalBountyEarnedUsd += LAB_BOUNTY_MAP[id] || 4000
    }
  }
  if (totalBountyEarnedUsd === 0 && solvedCount > 0) {
    totalBountyEarnedUsd = solvedCount * 5000
  }
  const formattedBounty = `$${totalBountyEarnedUsd.toLocaleString('en-US')} USD`
  const progressPercent = Math.min(100, Math.round((solvedCount / 10) * 100))
  const isFullyCertified = solvedCount >= 10

  const rankTitle =
    solvedCount >= 10
      ? 'Certified Sovereign Web Pentester (Z-CWPT)'
      : solvedCount >= 6
      ? 'Senior Web Penetration Tester'
      : solvedCount >= 3
      ? 'Bug Bounty Hunter'
      : 'Security Apprentice'

  const loadScores = async () => {
    setLoading(true)
    try {
      const data = await getLeaderboard()
      if (isMountedRef.current) {
        setEntries(data)
      }
    } catch (e) {
      console.error(e)
    } finally {
      if (isMountedRef.current) {
        setLoading(false)
      }
    }
  }

  useEffect(() => {
    isMountedRef.current = true
    loadScores()
    return () => {
      isMountedRef.current = false
    }
  }, [])

  const top3 = entries.slice(0, 3)

  return (
    <div className="flex-1 h-full overflow-hidden relative bg-[#FCFBF8]">
      {/* Pinned Ambient Background Artwork (Matches Flutter bg_dashboard.jpg) */}
      <div className="absolute inset-0 pointer-events-none z-0 overflow-hidden select-none">
        <img
          src="/assets/bg_dashboard.jpg"
          alt=""
          className="w-full h-full object-cover object-center opacity-85"
          onError={(e) => {
            ;(e.target as HTMLElement).style.display = 'none'
          }}
        />
        <div className="absolute inset-0 bg-[#FCFBF8]/50 cyber-dot-grid" />
      </div>

      <div className="relative z-10 h-full overflow-y-auto p-6 space-y-6">
        {/* Header */}
      <div className="flex flex-col md:flex-row md:items-center justify-between gap-4 border-b border-[#E6E2D8] pb-5">
        <div>
          <div className="flex items-center gap-2">
            <span className="px-2 py-0.5 rounded bg-[#FFEDD5] text-[#C2410C] border border-[#FDBA74] text-[10px] font-['Space_Grotesk'] font-bold">
              TURSO CLOUD CTF
            </span>
            <span className="text-[10px] font-['JetBrains_Mono'] text-[#9C9080]">
              REAL-TIME OPERATIVE STANDINGS
            </span>
          </div>
          <h1 className="text-xl font-bold font-['Space_Grotesk'] text-[#1E1A14] mt-1">
            Global Security Operative Leaderboard
          </h1>
        </div>

        <button
          onClick={loadScores}
          className="self-start md:self-auto px-3.5 py-1.5 border-2 border-[#CEC8BF] shadow-[0_2px_0_0_#E4DFD7] hover:border-[#2DD4BF] hover:shadow-[0_2px_0_0_#5EEAD4] hover:translate-y-0.5 active:translate-y-1 bg-white rounded-lg text-xs font-bold text-[#5C5347] hover:text-[#0F766E] transition-all flex items-center gap-2 cursor-pointer"
        >
          <RefreshCw size={14} />
          <span>Refresh Standings</span>
        </button>
      </div>

      {loading ? (
        <div className="py-16 flex items-center justify-center">
          <CuteAnimeLoading
            message="SYNCING TURSO LEADERBOARD..."
            subMessage="( 🏆 ✧ω✧) FETCHING LIVE RANKS"
          />
        </div>
      ) : (
        <>
          {/* Bootcamp Certification & Bounty Portfolio Card */}
          <div className="rounded-2xl border-2 border-[#2DD4BF] bg-gradient-to-r from-[#F0FDF4] via-white to-[#F7F5F0] p-5 sm:p-6 shadow-[0_4px_0_0_#0F766E,0_12px_24px_rgba(15,118,110,0.06)] space-y-4">
            <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-4 pb-4 border-b border-[#E6E2D8]">
              <div className="flex items-center gap-3.5">
                <div className="w-12 h-12 rounded-xl bg-gradient-to-br from-[#0F766E] to-[#14B8A6] text-white flex items-center justify-center shadow-xs shrink-0">
                  <Award size={24} />
                </div>
                <div>
                  <div className="flex items-center gap-2">
                    <span className="text-[10px] font-bold font-mono px-2 py-0.5 rounded-full bg-[#CCFBF1] text-[#0F766E] border border-[#2DD4BF] uppercase">
                      BOOTCAMP CREDENTIAL
                    </span>
                    <span className="text-xs font-mono text-[#78716C]">
                      // {rankTitle}
                    </span>
                  </div>
                  <h2 className="text-base sm:text-lg font-bold font-['Space_Grotesk'] text-[#1E1A14] mt-0.5">
                    Zitera Certified Web Penetration Tester (Z-CWPT)
                  </h2>
                </div>
              </div>

              <div className="flex items-center gap-3 self-start sm:self-auto">
                <div className="text-left sm:text-right">
                  <div className="text-[10px] font-mono text-[#78716C] uppercase font-bold">
                    SIMULATED BOUNTY EARNINGS
                  </div>
                  <div className="text-base sm:text-lg font-bold font-mono text-[#15803D]">
                    {formattedBounty}
                  </div>
                </div>

                <button
                  onClick={() => setCertModalOpen(true)}
                  className={`px-3.5 py-2 text-xs font-bold font-['Space_Grotesk'] rounded-xl border-2 transition-all flex items-center gap-1.5 cursor-pointer shadow-xs ${
                    isFullyCertified
                      ? 'bg-[#10B981] hover:bg-[#059669] text-white border-[#059669]'
                      : 'bg-white hover:bg-[#F0EDE8] text-[#0F766E] border-[#2DD4BF]'
                  }`}
                >
                  <Award size={14} />
                  <span>{isFullyCertified ? 'Unduh Sertifikat (Z-CWPT)' : 'Pratinjau Sertifikat'}</span>
                </button>
              </div>
            </div>

            {/* Progress Bar & Badges */}
            <div className="space-y-2">
              <div className="flex items-center justify-between text-xs font-mono font-bold">
                <span className="text-[#5C5347]">
                  Progres Kelulusan: {solvedCount} / 10 Modul Selesai ({progressPercent}%)
                </span>
                <span className="text-[#0F766E]">
                  {isFullyCertified ? '✓ MEMENUHI SYARAT SERTIFIKASI' : 'TARGET: 10/10 LABS'}
                </span>
              </div>
              <div className="w-full h-3 bg-[#E6E2D8] rounded-full overflow-hidden p-0.5">
                <div
                  className="h-full bg-gradient-to-r from-[#2DD4BF] to-[#0F766E] rounded-full transition-all duration-500"
                  style={{ width: `${Math.max(4, progressPercent)}%` }}
                />
              </div>
            </div>
          </div>

          {/* Podium Top 3 */}
          {top3.length > 0 && (
            <div className="grid grid-cols-1 md:grid-cols-3 gap-4 pt-2">
              {top3.map((entry, idx) => {
                const isFirst = idx === 0
                const isSecond = idx === 1

                const borderCol = isFirst
                  ? 'border-[#F59E0B] bg-[#FEF3C7]/40 ring-2 ring-[#F59E0B]/30'
                  : isSecond
                  ? 'border-[#94A3B8] bg-[#F1F5F9]/60'
                  : 'border-[#B45309] bg-[#FFFBEB]/60'

                const medalColor = isFirst
                  ? 'text-[#F59E0B]'
                  : isSecond
                  ? 'text-[#94A3B8]'
                  : 'text-[#B45309]'

                return (
                  <div
                    key={entry.userId || idx}
                    className={`p-5 rounded-2xl border-2 transition-all flex flex-col items-center text-center relative overflow-hidden shadow-[0_4px_0_0_rgba(0,0,0,0.06)] ${borderCol}`}
                  >
                    {/* Rank Badge */}
                    <div className="absolute top-3 left-3 flex items-center gap-1 font-['JetBrains_Mono'] font-bold text-xs">
                      <Medal className={medalColor} size={18} />
                      <span className={medalColor}>#{entry.rank}</span>
                    </div>

                    {/* Anime Avatar Portrait */}
                    <div className="relative w-20 h-20 rounded-full border-3 border-[#2DD4BF] overflow-hidden bg-white mt-4 mb-3 shadow-md">
                      <img
                        src={`/assets/avatars/${entry.avatarId}.png`}
                        alt=""
                        className="w-full h-full object-cover"
                        onError={(e) => {
                          ;(e.target as HTMLImageElement).src = '/assets/zeta_avatar.png'
                        }}
                      />
                    </div>

                    <div className="font-['Space_Grotesk'] font-bold text-base text-[#1E1A14]">
                      {entry.username}
                      {entry.isCurrentUser && (
                        <span className="ml-1.5 px-1.5 py-0.5 rounded bg-[#0F766E] text-white text-[9px]">
                          YOU
                        </span>
                      )}
                    </div>

                    <div className="mt-2 text-xl font-bold font-['JetBrains_Mono'] text-[#0F766E]">
                      {entry.totalPoints}{' '}
                      <span className="text-xs font-normal text-[#9C9080]">PTS</span>
                    </div>

                    <div className="mt-1 text-[11px] font-['JetBrains_Mono'] text-[#5C5347]">
                      {entry.labsSolved} Labs Solved • {formatLastSolve(entry.lastSolveAt)}
                    </div>
                  </div>
                )
              })}
            </div>
          )}

          {/* Full Standings Table */}
          <div className="bg-[#FCFBF8] border-2 border-[#CEC8BF] rounded-xl overflow-hidden shadow-[0_3px_0_0_#E4DFD7]">
            <div className="px-5 py-3.5 bg-[#F7F5F0] border-b-2 border-[#E6E2D8] flex items-center justify-between">
              <span className="text-xs font-['Space_Grotesk'] font-bold text-[#1E1A14]">
                OPERATIVE STANDINGS ROSTER
              </span>
              <span className="text-[10px] font-['JetBrains_Mono'] text-[#9C9080]">
                TOTAL OPERATIVES: {entries.length}
              </span>
            </div>

            <div className="divide-y divide-[#E6E2D8]">
              {entries.map((entry) => (
                <div
                  key={entry.userId || entry.rank}
                  className={`flex items-center justify-between p-4 text-xs transition-colors ${
                    entry.isCurrentUser
                      ? 'bg-[#CCFBF1]/30 font-semibold'
                      : 'hover:bg-[#F0FDFA]'
                  }`}
                >
                  <div className="flex items-center gap-3">
                    <span className="w-6 text-center font-['JetBrains_Mono'] font-bold text-[#5C5347]">
                      #{entry.rank}
                    </span>

                    <div className="w-9 h-9 rounded-full border-2 border-[#2DD4BF] overflow-hidden bg-white shrink-0">
                      <img
                        src={`/assets/avatars/${entry.avatarId}.png`}
                        alt=""
                        className="w-full h-full object-cover"
                        onError={(e) => {
                          ;(e.target as HTMLImageElement).src = '/assets/zeta_avatar.png'
                        }}
                      />
                    </div>

                    <div>
                      <div className="font-['Space_Grotesk'] font-bold text-sm text-[#1E1A14]">
                        {entry.username}
                        {entry.isCurrentUser && (
                          <span className="ml-2 px-1.5 py-0.5 rounded bg-[#0F766E] text-white text-[9px] font-bold">
                            YOU
                          </span>
                        )}
                      </div>
                      <div className="text-[10px] font-['JetBrains_Mono'] text-[#9C9080]">
                        Codename: {entry.avatarId.toUpperCase()}
                      </div>
                    </div>
                  </div>

                  <div className="flex items-center gap-6 font-['JetBrains_Mono'] text-right">
                    <div>
                      <div className="font-bold text-[#0F766E] text-sm">{entry.totalPoints} PTS</div>
                      <div className="text-[10px] text-[#9C9080]">{entry.labsSolved} Solved</div>
                    </div>

                    <div className="hidden sm:block text-[10px] text-[#5C5347]">
                      <div className="flex items-center gap-1 justify-end">
                        <Clock size={11} className="text-[#9C9080]" />
                        <span>{formatLastSolve(entry.lastSolveAt)}</span>
                      </div>
                    </div>
                  </div>
                </div>
              ))}

              {entries.length === 0 && (
                <div className="p-8 text-center text-xs text-[#9C9080] font-['JetBrains_Mono']">
                  No records found on Turso pipeline. Solves will appear automatically.
                </div>
              )}
            </div>
          </div>
        </>
      )}
      </div>

      {/* Official Certificate Modal */}
      <CertificateModal
        isOpen={certModalOpen}
        onClose={() => setCertModalOpen(false)}
        studentName={studentName}
        totalBountyEarned={formattedBounty}
      />
    </div>
  )
}
