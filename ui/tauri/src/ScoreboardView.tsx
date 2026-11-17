import { useState, useEffect } from 'react'
import {
  Medal,
  RefreshCw,
  Clock,
} from 'lucide-react'
import {
  getLeaderboard,
  formatLastSolve,
  type ScoreboardEntry,
} from './scoreboardService'

export function ScoreboardView() {
  const [entries, setEntries] = useState<ScoreboardEntry[]>([])
  const [loading, setLoading] = useState(true)

  const loadScores = async () => {
    setLoading(true)
    try {
      const data = await getLeaderboard()
      setEntries(data)
    } catch (e) {
      console.error(e)
    } finally {
      setLoading(false)
    }
  }

  useEffect(() => {
    loadScores()
  }, [])

  const top3 = entries.slice(0, 3)

  return (
    <div className="flex-1 overflow-y-auto p-6 space-y-6 bg-[#FCFBF8]">
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
          className="self-start md:self-auto px-3.5 py-1.5 border border-[#E6E2D8] hover:border-[#2DD4BF] bg-white rounded-lg text-xs font-bold text-[#5C5347] hover:text-[#0F766E] transition-colors flex items-center gap-2"
        >
          <RefreshCw size={14} />
          <span>Refresh Standings</span>
        </button>
      </div>

      {loading ? (
        <div className="py-20 flex flex-col items-center justify-center space-y-3">
          <div className="w-12 h-12 rounded-full border-3 border-[#2DD4BF] border-t-transparent animate-spin" />
          <div className="text-xs font-['JetBrains_Mono'] text-[#5C5347]">
            Fetching Turso pipeline scoreboard...
          </div>
        </div>
      ) : (
        <>
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
  )
}
