import { useState, useEffect, type FormEvent } from 'react'
import ReactMarkdown from 'react-markdown'
import remarkGfm from 'remark-gfm'
import confetti from 'canvas-confetti'
import {
  ArrowLeft,
  BookOpen,
  Terminal as TerminalIcon,
  Trophy,
  Play,
  Square,
  ExternalLink,
  Lightbulb,
  CheckCircle2,
  AlertCircle,
  Send,
} from 'lucide-react'
import {
  getLabContent,
  getLabStatus,
  startLab,
  stopLab,
  openBrowser,
  executeTerminal,
  validateChallenge,
  type LabContent,
  type LabItem,
} from './engineClient'
import { recordSolve } from './scoreboardService'
import { CuteAnimeLoading } from './CuteAnimeLoading'

interface LabDetailViewProps {
  labId: string
  onBack: () => void
}

type ModeTab = 'theory' | 'practice' | 'challenge'

export function LabDetailView({ labId, onBack }: LabDetailViewProps) {
  const [content, setContent] = useState<LabContent | null>(null)
  const [status, setStatus] = useState<LabItem | null>(null)
  const [activeTab, setActiveTab] = useState<ModeTab>('theory')
  const [loading, setLoading] = useState(true)

  // CTF state
  const [flagInput, setFlagInput] = useState('')
  const [revealedHints, setRevealedHints] = useState<number[]>([])
  const [flagResult, setFlagResult] = useState<{ passed: boolean; message: string } | null>(null)
  const [submittingFlag, setSubmittingFlag] = useState(false)

  // Terminal state
  const [terminalHistory, setTerminalHistory] = useState<
    Array<{ type: 'input' | 'output' | 'error'; text: string }>
  >([
    { type: 'output', text: `Zitera Tilix Interactive Shell // Context: ${labId}` },
    { type: 'output', text: `Type 'help' or 'curl http://127.0.0.1:<PORT>' to probe target.` },
  ])
  const [terminalInput, setTerminalInput] = useState('')
  const [executing, setExecuting] = useState(false)

  const loadData = async () => {
    setLoading(true)
    try {
      const [c, s] = await Promise.all([
        getLabContent(labId),
        getLabStatus(labId).catch(() => null),
      ])
      setContent(c)
      setStatus(s)
    } finally {
      setLoading(false)
    }
  }

  useEffect(() => {
    loadData()
  }, [labId])

  const handleToggleTarget = async () => {
    if (!status) return
    if (status.running) {
      await stopLab(labId)
    } else {
      await startLab(labId)
    }
    const updated = await getLabStatus(labId)
    setStatus(updated)
  }

  const handleFlagSubmit = async (e: FormEvent) => {
    e.preventDefault()
    if (!flagInput.trim()) return
    setSubmittingFlag(true)
    setFlagResult(null)

    try {
      const res = await validateChallenge(labId, flagInput)
      if (res.status === 'passed') {
        setFlagResult({ passed: true, message: res.message || 'CTF FLAG CAPTURED! +100 PTS' })
        // Trigger confetti celebration!
        confetti({
          particleCount: 120,
          spread: 80,
          origin: { y: 0.6 },
          colors: ['#2DD4BF', '#0F766E', '#F59E0B', '#A855F7'],
        })
        await recordSolve(labId, 100)
      } else {
        setFlagResult({ passed: false, message: res.message || 'Incorrect flag. Inspect the clues!' })
      }
    } finally {
      setSubmittingFlag(false)
    }
  }

  const handleTerminalSubmit = async (e: FormEvent) => {
    e.preventDefault()
    const cmd = terminalInput.trim()
    if (!cmd) return

    setTerminalHistory((prev) => [...prev, { type: 'input', text: `$ ${cmd}` }])
    setTerminalInput('')
    setExecuting(true)

    try {
      if (cmd === 'clear') {
        setTerminalHistory([])
        return
      }
      const res = await executeTerminal(cmd, labId)
      if (res.stdout) {
        setTerminalHistory((prev) => [...prev, { type: 'output', text: res.stdout }])
      }
      if (res.stderr) {
        setTerminalHistory((prev) => [...prev, { type: 'error', text: res.stderr }])
      }
    } catch (err) {
      setTerminalHistory((prev) => [
        ...prev,
        { type: 'error', text: `Command execution error: ${err}` },
      ])
    } finally {
      setExecuting(false)
    }
  }

  const revealHint = (tier: number) => {
    if (!revealedHints.includes(tier)) {
      setRevealedHints((prev) => [...prev, tier])
    }
  }

  if (loading || !content) {
    return (
      <div className="flex-1 flex items-center justify-center p-8 bg-[#FCFBF8]">
        <CuteAnimeLoading
          message="PREPARING TARGET SANDBOX..."
          subMessage="( ⚡ •̀ᴗ•́) DEPLOYING WIN32 CONTAINER"
        />
      </div>
    )
  }

  const targetPort = status?.port ?? content.manifest.defaultPort

  return (
    <div className="flex-1 flex flex-col h-full overflow-hidden bg-[#FCFBF8]">
      {/* Top Navigation Bar */}
      <div className="px-6 py-3.5 border-b border-[#E6E2D8] bg-[#F7F5F0] flex items-center justify-between shrink-0">
        <div className="flex items-center gap-3">
          <button
            onClick={onBack}
            className="p-1.5 rounded-lg border border-[#E6E2D8] bg-white hover:bg-[#F0EDE8] text-[#5C5347] transition-colors flex items-center gap-1 text-xs font-bold"
          >
            <ArrowLeft size={16} />
            <span className="hidden sm:inline">Back</span>
          </button>
          <div className="h-4 w-px bg-[#E6E2D8]" />
          <div>
            <div className="flex items-center gap-2">
              <span className="px-2 py-0.5 rounded bg-[#0F766E] text-white text-[10px] font-bold font-['JetBrains_Mono']">
                {content.manifest.id}
              </span>
              <h1 className="text-sm font-bold font-['Space_Grotesk'] text-[#1E1A14]">
                {content.manifest.title}
              </h1>
            </div>
          </div>
        </div>

        {/* Live Target Service Launcher Button */}
        <div className="flex items-center gap-2">
          {status?.running ? (
            <>
              <button
                onClick={() => openBrowser(`http://127.0.0.1:${targetPort}`)}
                className="px-3 py-1.5 bg-[#0F766E] hover:bg-[#115E59] text-white text-xs font-bold rounded-lg flex items-center gap-1.5 shadow-xs"
              >
                <span>Buka Web (: {targetPort})</span>
                <ExternalLink size={13} />
              </button>
              <button
                onClick={handleToggleTarget}
                className="px-3 py-1.5 bg-white border border-[#EF4444] text-[#EF4444] hover:bg-[#FEE2E2] text-xs font-bold rounded-lg flex items-center gap-1"
              >
                <Square size={13} />
                <span>Stop</span>
              </button>
            </>
          ) : (
            <button
              onClick={handleToggleTarget}
              className="px-4 py-1.5 bg-[#0F766E] hover:bg-[#115E59] text-white text-xs font-bold rounded-lg flex items-center gap-1.5 shadow-xs"
            >
              <Play size={13} />
              <span>LAUNCH TARGET</span>
            </button>
          )}
        </div>
      </div>

      {/* 3-Tier Progressive Track Switcher */}
      <div className="flex border-b border-[#E6E2D8] bg-[#FCFBF8] px-6 gap-2 pt-2 shrink-0">
        <button
          onClick={() => setActiveTab('theory')}
          className={`flex items-center gap-2 px-4 py-2 text-xs font-['Space_Grotesk'] font-bold rounded-t-lg transition-colors border-t border-x ${
            activeTab === 'theory'
              ? 'bg-[#FCFBF8] border-[#2DD4BF] text-[#0F766E]'
              : 'border-transparent text-[#9C9080] hover:text-[#1E1A14]'
          }`}
        >
          <BookOpen size={14} />
          <span>TIER 1: THEORY & MECHANICS</span>
        </button>

        <button
          onClick={() => setActiveTab('practice')}
          className={`flex items-center gap-2 px-4 py-2 text-xs font-['Space_Grotesk'] font-bold rounded-t-lg transition-colors border-t border-x ${
            activeTab === 'practice'
              ? 'bg-[#FCFBF8] border-[#2DD4BF] text-[#0F766E]'
              : 'border-transparent text-[#9C9080] hover:text-[#1E1A14]'
          }`}
        >
          <TerminalIcon size={14} />
          <span>TIER 2: GUIDED PRACTICE</span>
        </button>

        <button
          onClick={() => setActiveTab('challenge')}
          className={`flex items-center gap-2 px-4 py-2 text-xs font-['Space_Grotesk'] font-bold rounded-t-lg transition-colors border-t border-x ${
            activeTab === 'challenge'
              ? 'bg-[#FCFBF8] border-[#2DD4BF] text-[#0F766E]'
              : 'border-transparent text-[#9C9080] hover:text-[#1E1A14]'
          }`}
        >
          <Trophy size={14} />
          <span>TIER 3: CTF CHALLENGE</span>
        </button>
      </div>

      {/* Main Split Layout: Content + Hacker Tilix Terminal */}
      <div className="flex-1 flex flex-col lg:flex-row overflow-hidden">
        {/* Left Column: Markdown content or challenge */}
        <div className="flex-1 overflow-y-auto p-6 border-r border-[#E6E2D8] space-y-6">
          {activeTab === 'theory' && (
            <div className="prose-zitera max-w-none">
              <ReactMarkdown remarkPlugins={[remarkGfm]}>
                {content.lessons['01-overview.md'] ||
                  content.lessons['theory.md'] ||
                  `# ${content.manifest.title}\n\n${content.manifest.shortDescription}`}
              </ReactMarkdown>
            </div>
          )}

          {activeTab === 'practice' && (
            <div className="prose-zitera max-w-none">
              <ReactMarkdown remarkPlugins={[remarkGfm]}>
                {content.lessons['02-practice.md'] ||
                  content.lessons['practice.md'] ||
                  `# Guided Exploitation Walkthrough\n\nEnsure the target service is running, then follow the instructions in the Tilix terminal on the right.`}
              </ReactMarkdown>
            </div>
          )}

          {activeTab === 'challenge' && (
            <div className="space-y-6">
              {/* Objective Banner */}
              <div className="p-4 rounded-xl border border-[#FDBA74] bg-[#FFEDD5] text-[#C2410C]">
                <div className="flex items-center gap-2 font-['Space_Grotesk'] font-bold text-sm">
                  <Trophy size={16} />
                  <span>CHALLENGE OBJECTIVE</span>
                </div>
                <p className="text-xs font-['JetBrains_Mono'] mt-1 leading-relaxed">
                  {content.challengeObjective}
                </p>
              </div>

              {/* CTF Flag Submission Form */}
              <form
                onSubmit={handleFlagSubmit}
                className="p-5 rounded-xl border border-[#E6E2D8] bg-[#F7F5F0] space-y-3"
              >
                <label className="block text-xs font-['Space_Grotesk'] font-bold text-[#1E1A14]">
                  SUBMIT CAPTURED CTF FLAG
                </label>
                <div className="flex items-center gap-2">
                  <input
                    type="text"
                    value={flagInput}
                    onChange={(e) => setFlagInput(e.target.value)}
                    placeholder="FLAG{...} or ZITERA{...}"
                    className="flex-1 px-3.5 py-2 text-xs bg-white border border-[#E6E2D8] rounded-lg focus:outline-none focus:border-[#2DD4BF] font-['JetBrains_Mono']"
                  />
                  <button
                    type="submit"
                    disabled={submittingFlag || !flagInput.trim()}
                    className="px-4 py-2 bg-[#0F766E] hover:bg-[#115E59] text-white text-xs font-['Space_Grotesk'] font-bold rounded-lg shadow-sm transition-all disabled:opacity-50 flex items-center gap-1.5"
                  >
                    <span>{submittingFlag ? 'Verifying...' : 'SUBMIT'}</span>
                    <Send size={13} />
                  </button>
                </div>

                {flagResult && (
                  <div
                    className={`p-3 rounded-lg border text-xs font-['JetBrains_Mono'] flex items-center gap-2 ${
                      flagResult.passed
                        ? 'bg-[#E6F9F2] border-[#86EFAC] text-[#16A34A]'
                        : 'bg-[#FEE2E2] border-[#FCA5A5] text-[#DC2626]'
                    }`}
                  >
                    {flagResult.passed ? <CheckCircle2 size={16} /> : <AlertCircle size={16} />}
                    <span>{flagResult.message}</span>
                  </div>
                )}
              </form>

              {/* Progressive Hints Section */}
              <div className="space-y-3">
                <div className="flex items-center gap-2 text-xs font-['Space_Grotesk'] font-bold text-[#5C5347]">
                  <Lightbulb size={15} className="text-[#F59E0B]" />
                  <span>PROGRESSIVE HINTS ({content.hints.length})</span>
                </div>

                <div className="space-y-2">
                  {content.hints.map((hint) => {
                    const isRevealed = revealedHints.includes(hint.tier)
                    return (
                      <div
                        key={hint.tier}
                        className="p-3.5 rounded-lg border border-[#E6E2D8] bg-[#FCFBF8] text-xs font-['JetBrains_Mono']"
                      >
                        <div className="flex items-center justify-between">
                          <span className="font-bold text-[#0F766E]">
                            HINT TIER {hint.tier} // {hint.type.toUpperCase()}
                          </span>
                          {!isRevealed ? (
                            <button
                              onClick={() => revealHint(hint.tier)}
                              className="px-2.5 py-1 text-[10px] bg-[#FEF3C7] border border-[#FCD34D] text-[#B45309] rounded hover:bg-[#FDE68A] font-bold"
                            >
                              Unlock Hint
                            </button>
                          ) : (
                            <span className="text-[10px] text-[#16A34A] font-bold">REVEALED</span>
                          )}
                        </div>
                        {isRevealed ? (
                          <p className="mt-2 text-[#5C5347] bg-[#F0EDE8] p-2 rounded">
                            {hint.hint}
                          </p>
                        ) : (
                          <p className="mt-1 text-[#9C9080] italic text-[11px]">
                            Hint locked. Click to view if you are stuck.
                          </p>
                        )}
                      </div>
                    )
                  })}
                </div>
              </div>
            </div>
          )}
        </div>

        {/* Right Column: Hacker Tilix Terminal Widget */}
        <div className="w-full lg:w-[420px] shrink-0 h-72 lg:h-full bg-[#090D16] border-t lg:border-t-0 border-[#25334D] flex flex-col overflow-hidden">
          {/* Terminal Titlebar */}
          <div className="px-4 py-2.5 bg-[#101726] border-b border-[#25334D] flex items-center justify-between text-xs text-[#9CA3AF]">
            <div className="flex items-center gap-2">
              <span className="w-3 h-3 rounded-full bg-[#EF4444]" />
              <span className="w-3 h-3 rounded-full bg-[#F59E0B]" />
              <span className="w-3 h-3 rounded-full bg-[#10B981]" />
              <span className="font-['JetBrains_Mono'] text-[11px] text-[#38BDF8] ml-2">
                tilix://{labId.toLowerCase()}
              </span>
            </div>
            <button
              onClick={() => setTerminalHistory([])}
              className="text-[10px] hover:text-white transition-colors"
              title="Clear terminal"
            >
              CLEAR
            </button>
          </div>

          {/* Terminal Output Stream */}
          <div className="flex-1 p-4 overflow-y-auto space-y-1 font-['JetBrains_Mono'] text-xs text-[#E5E7EB]">
            {terminalHistory.map((item, idx) => (
              <div
                key={idx}
                className={`terminal-line break-all whitespace-pre-wrap ${
                  item.type === 'input'
                    ? 'text-[#2DD4BF] font-bold'
                    : item.type === 'error'
                    ? 'text-[#EF4444]'
                    : 'text-[#9CA3AF]'
                }`}
              >
                {item.text}
              </div>
            ))}
            {executing && (
              <div className="text-[11px] text-[#38BDF8] animate-pulse">
                [Executing in sandbox environment...]
              </div>
            )}
          </div>

          {/* Terminal Input Prompt */}
          <form
            onSubmit={handleTerminalSubmit}
            className="p-2.5 bg-[#101726] border-t border-[#25334D] flex items-center gap-2"
          >
            <span className="text-[#2DD4BF] font-['JetBrains_Mono'] text-xs font-bold">$</span>
            <input
              type="text"
              value={terminalInput}
              onChange={(e) => setTerminalInput(e.target.value)}
              placeholder="curl http://127.0.0.1:..."
              className="flex-1 bg-transparent border-none text-xs text-white font-['JetBrains_Mono'] focus:outline-none"
            />
          </form>
        </div>
      </div>
    </div>
  )
}
