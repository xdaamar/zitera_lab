import { useState, useEffect } from 'react'
import {
  Search,
  Play,
  Square,
  RotateCcw,
  ExternalLink,
  DownloadCloud,
  ChevronRight,
  RefreshCw,
} from 'lucide-react'
import {
  getLabs,
  startLab,
  stopLab,
  resetLab,
  updateLab,
  openBrowser,
  type LabItem,
} from './engineClient'

interface LabsViewProps {
  onSelectLab: (labId: string) => void
}

type DifficultyFilter = 'ALL' | 'Beginner' | 'Intermediate' | 'Advanced'

export function LabsView({ onSelectLab }: LabsViewProps) {
  const [labs, setLabs] = useState<LabItem[]>([])
  const [search, setSearch] = useState('')
  const [difficulty, setDifficulty] = useState<DifficultyFilter>('ALL')
  const [loading, setLoading] = useState(true)
  const [actionInProgress, setActionInProgress] = useState<Record<string, string>>({})

  const loadLabs = async () => {
    setLoading(true)
    try {
      const data = await getLabs()
      setLabs(data)
    } finally {
      setLoading(false)
    }
  }

  useEffect(() => {
    loadLabs()
  }, [])

  const handleAction = async (labId: string, action: 'start' | 'stop' | 'reset' | 'update') => {
    setActionInProgress((prev) => ({ ...prev, [labId]: action }))
    try {
      if (action === 'start') await startLab(labId)
      else if (action === 'stop') await stopLab(labId)
      else if (action === 'reset') await resetLab(labId)
      else if (action === 'update') await updateLab(labId)
      await loadLabs()
    } finally {
      setActionInProgress((prev) => {
        const next = { ...prev }
        delete next[labId]
        return next
      })
    }
  }

  const filteredLabs = labs.filter((lab) => {
    const q = search.trim().toLowerCase()
    const matchesSearch =
      !q ||
      lab.id.toLowerCase().includes(q) ||
      lab.title.toLowerCase().includes(q) ||
      lab.recommendedTools.some((t) => t.toLowerCase().includes(q))

    const matchesDifficulty =
      difficulty === 'ALL' ||
      (difficulty === 'Beginner' && ['A01', 'A02', 'A03'].includes(lab.id)) ||
      (difficulty === 'Intermediate' && ['A04', 'A05', 'A06', 'A07'].includes(lab.id)) ||
      (difficulty === 'Advanced' && ['A08', 'A09', 'A10'].includes(lab.id))

    return matchesSearch && matchesDifficulty
  })

  return (
    <div className="flex-1 overflow-y-auto p-6 space-y-6 bg-[#FCFBF8]">
      {/* Header & Controls */}
      <div className="flex flex-col md:flex-row md:items-center justify-between gap-4 border-b border-[#E6E2D8] pb-5">
        <div>
          <div className="flex items-center gap-2">
            <span className="px-2 py-0.5 rounded bg-[#CCFBF1] text-[#0F766E] border border-[#2DD4BF] text-[10px] font-['Space_Grotesk'] font-bold">
              OWASP TOP 10:2025
            </span>
            <span className="text-[10px] font-['JetBrains_Mono'] text-[#9C9080]">
              OFFLINE-FIRST DEFENSIVE LABS
            </span>
          </div>
          <h1 className="text-xl font-bold font-['Space_Grotesk'] text-[#1E1A14] mt-1">
            Standard Vulnerability Curriculum
          </h1>
        </div>

        {/* Search & Filter Inputs */}
        <div className="flex flex-wrap items-center gap-3">
          <div className="flex items-center bg-[#F0EDE8] p-1 rounded-lg border border-[#E6E2D8]">
            {(['ALL', 'Beginner', 'Intermediate', 'Advanced'] as DifficultyFilter[]).map((d) => (
              <button
                key={d}
                onClick={() => setDifficulty(d)}
                className={`px-2.5 py-1 text-[11px] font-['Space_Grotesk'] font-bold rounded-md transition-colors ${
                  difficulty === d
                    ? 'bg-white text-[#0F766E] shadow-xs'
                    : 'text-[#5C5347] hover:text-[#1E1A14]'
                }`}
              >
                {d}
              </button>
            ))}
          </div>

          <div className="relative">
            <Search className="absolute left-3 top-2.5 text-[#9C9080]" size={15} />
            <input
              type="text"
              placeholder="Search lab..."
              value={search}
              onChange={(e) => setSearch(e.target.value)}
              className="pl-9 pr-3 py-1.5 text-xs bg-white border border-[#E6E2D8] rounded-lg focus:outline-none focus:border-[#2DD4BF] font-['JetBrains_Mono'] w-48"
            />
          </div>

          <button
            onClick={loadLabs}
            className="p-2 border border-[#E6E2D8] hover:border-[#2DD4BF] bg-white rounded-lg text-[#5C5347] hover:text-[#0F766E] transition-colors"
            title="Refresh labs"
          >
            <RefreshCw size={15} />
          </button>
        </div>
      </div>

      {/* Labs Grid */}
      {loading ? (
        <div className="py-20 flex flex-col items-center justify-center text-center space-y-3">
          <div className="w-12 h-12 rounded-full border-3 border-[#2DD4BF] border-t-transparent animate-spin" />
          <div className="text-xs font-['JetBrains_Mono'] text-[#5C5347]">
            Enumerating sandboxed labs...
          </div>
        </div>
      ) : (
        <div className="grid grid-cols-1 lg:grid-cols-2 gap-4">
          {filteredLabs.map((lab) => {
            const inProgress = actionInProgress[lab.id]
            return (
              <div
                key={lab.id}
                className={`p-5 rounded-xl border-[1.5px] transition-all bg-[#FCFBF8] flex flex-col justify-between ${
                  lab.running
                    ? 'border-[#2DD4BF] shadow-[0_4px_16px_rgba(45,212,191,0.2)] bg-[#F0FDFA]'
                    : 'border-[#E6E2D8] hover:border-[#2DD4BF]/60 hover:shadow-xs'
                }`}
              >
                <div>
                  {/* Top Bar of card */}
                  <div className="flex items-center justify-between mb-2">
                    <div className="flex items-center gap-2">
                      <span className="px-2 py-0.5 rounded bg-[#0F766E] text-white text-xs font-bold font-['JetBrains_Mono']">
                        {lab.id}
                      </span>
                      <span className="text-[10px] font-['JetBrains_Mono'] text-[#9C9080]">
                        PORT :{lab.port}
                      </span>
                    </div>

                    {/* Status Pill */}
                    <div className="flex items-center gap-2">
                      {lab.running ? (
                        <span className="inline-flex items-center gap-1.5 px-2 py-0.5 rounded-full bg-[#E6F9F2] border border-[#86EFAC] text-[10px] font-['JetBrains_Mono'] font-bold text-[#16A34A]">
                          <span className="w-1.5 h-1.5 rounded-full bg-[#16A34A] animate-ping" />
                          RUNNING
                        </span>
                      ) : (
                        <span className="px-2 py-0.5 rounded-full bg-[#F0EDE8] border border-[#E6E2D8] text-[10px] font-['JetBrains_Mono'] text-[#5C5347]">
                          STOPPED
                        </span>
                      )}
                    </div>
                  </div>

                  {/* Title & Description */}
                  <h3
                    onClick={() => onSelectLab(lab.id)}
                    className="text-base font-bold font-['Space_Grotesk'] text-[#1E1A14] hover:text-[#0F766E] cursor-pointer transition-colors"
                  >
                    {lab.title}
                  </h3>

                  {/* Recommended tools pills */}
                  {lab.recommendedTools.length > 0 && (
                    <div className="flex flex-wrap items-center gap-1.5 mt-2.5">
                      <span className="text-[10px] font-['JetBrains_Mono'] text-[#9C9080]">
                        Tools:
                      </span>
                      {lab.recommendedTools.map((t) => (
                        <span
                          key={t}
                          className="px-1.5 py-0.5 rounded bg-[#F0EDE8] border border-[#E6E2D8] text-[9px] font-['JetBrains_Mono'] text-[#5C5347]"
                        >
                          {t}
                        </span>
                      ))}
                    </div>
                  )}
                </div>

                {/* Bottom Actions */}
                <div className="pt-4 mt-4 border-t border-[#E6E2D8] flex flex-wrap items-center justify-between gap-2">
                  <div className="flex items-center gap-2">
                    {lab.running ? (
                      <>
                        <button
                          onClick={() => openBrowser(`http://127.0.0.1:${lab.port}`)}
                          className="px-3 py-1.5 bg-[#0F766E] hover:bg-[#115E59] text-white text-xs font-bold rounded-lg flex items-center gap-1.5 shadow-xs"
                        >
                          <span>Buka Web</span>
                          <ExternalLink size={13} />
                        </button>
                        <button
                          onClick={() => handleAction(lab.id, 'stop')}
                          disabled={!!inProgress}
                          className="px-3 py-1.5 bg-white border border-[#EF4444] text-[#EF4444] hover:bg-[#FEE2E2] text-xs font-bold rounded-lg flex items-center gap-1 disabled:opacity-50"
                        >
                          <Square size={12} />
                          <span>Stop</span>
                        </button>
                        <button
                          onClick={() => handleAction(lab.id, 'reset')}
                          disabled={!!inProgress}
                          className="p-1.5 bg-white border border-[#E6E2D8] hover:border-[#2DD4BF] text-[#5C5347] rounded-lg transition-colors"
                          title="Reset target database & files"
                        >
                          <RotateCcw size={14} />
                        </button>
                      </>
                    ) : (
                      <button
                        onClick={() => handleAction(lab.id, 'start')}
                        disabled={!!inProgress}
                        className="px-4 py-1.5 bg-[#0F766E] hover:bg-[#115E59] text-white text-xs font-bold rounded-lg flex items-center gap-1.5 shadow-xs disabled:opacity-50"
                      >
                        <Play size={13} />
                        <span>{inProgress === 'start' ? 'Launching...' : 'RUN LAB'}</span>
                      </button>
                    )}

                    {lab.hasUpdate && (
                      <button
                        onClick={() => handleAction(lab.id, 'update')}
                        disabled={!!inProgress}
                        className="px-2.5 py-1.5 bg-[#FEF3C7] border border-[#FCD34D] text-[#B45309] hover:bg-[#FDE68A] text-xs font-bold rounded-lg flex items-center gap-1"
                        title="Update lab package"
                      >
                        <DownloadCloud size={13} />
                        <span>Update Lab</span>
                      </button>
                    )}
                  </div>

                  {/* Detail view trigger */}
                  <button
                    onClick={() => onSelectLab(lab.id)}
                    className="text-xs font-['Space_Grotesk'] font-bold text-[#0F766E] hover:underline flex items-center gap-1"
                  >
                    <span>Lab Details</span>
                    <ChevronRight size={14} />
                  </button>
                </div>
              </div>
            )
          })}
        </div>
      )}
    </div>
  )
}
