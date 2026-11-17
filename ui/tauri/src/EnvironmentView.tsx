import { useState, useEffect, useRef } from 'react'
import {
  CheckCircle2,
  XCircle,
  HardDrive,
  Cpu,
  RefreshCw,
} from 'lucide-react'
import { checkDoctor, type DiagnosticsResult, type ComponentItem } from './engineClient'
import { CuteAnimeLoading } from './CuteAnimeLoading'

export function EnvironmentView() {
  const [diag, setDiag] = useState<DiagnosticsResult | null>(null)
  const [loading, setLoading] = useState(true)
  const isMountedRef = useRef(true)

  const loadDiag = async () => {
    setLoading(true)
    try {
      const data = await checkDoctor()
      if (isMountedRef.current) {
        setDiag(data)
      }
    } finally {
      if (isMountedRef.current) {
        setLoading(false)
      }
    }
  }

  useEffect(() => {
    isMountedRef.current = true
    loadDiag()
    return () => {
      isMountedRef.current = false
    }
  }, [])

  const renderComponentRow = (title: string, item?: ComponentItem) => {
    if (!item) return null
    const isReady = item.status === 'READY' || item.installed

    return (
      <div className="p-4 rounded-xl border-2 border-[#CEC8BF] bg-[#FCFBF8] shadow-[0_3px_0_0_#E4DFD7] flex items-center justify-between">
        <div className="space-y-0.5">
          <div className="flex items-center gap-2">
            <span className="font-['Space_Grotesk'] font-bold text-sm text-[#1E1A14]">
              {item.name || title}
            </span>
            {item.version && (
              <span className="px-1.5 py-0.5 rounded bg-[#F0EDE8] text-[10px] font-['JetBrains_Mono'] text-[#0F766E]">
                v{item.version}
              </span>
            )}
          </div>
          <div className="text-xs text-[#5C5347] font-['JetBrains_Mono']">
            {item.message || 'Subsystem check complete.'}
          </div>
          {item.recommendation && (
            <div className="text-[10px] text-[#B45309] font-['JetBrains_Mono'] mt-1">
              Tip: {item.recommendation}
            </div>
          )}
        </div>

        <div className="shrink-0">
          {isReady ? (
            <span className="inline-flex items-center gap-1 px-2.5 py-1 rounded-full bg-[#E6F9F2] border border-[#86EFAC] text-[10px] font-['JetBrains_Mono'] font-bold text-[#16A34A]">
              <CheckCircle2 size={13} />
              READY
            </span>
          ) : (
            <span className="inline-flex items-center gap-1 px-2.5 py-1 rounded-full bg-[#FEE2E2] border border-[#FCA5A5] text-[10px] font-['JetBrains_Mono'] font-bold text-[#DC2626]">
              <XCircle size={13} />
              MISSING
            </span>
          )}
        </div>
      </div>
    )
  }

  return (
    <div className="flex-1 h-full overflow-hidden relative bg-[#FCFBF8]">
      {/* Pinned Ambient Background Artwork (Matches Flutter bg_environment.jpg) */}
      <div className="absolute inset-0 pointer-events-none z-0 overflow-hidden select-none">
        <img
          src="/assets/bg_environment.jpg"
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
            <span className="px-2 py-0.5 rounded bg-[#FEF3C7] text-[#B45309] border border-[#FCD34D] text-[10px] font-['Space_Grotesk'] font-bold">
              SYS_DIAGNOSTICS
            </span>
            <span className="text-[10px] font-['JetBrains_Mono'] text-[#9C9080]">
              HOST INTEGRITY & SANDBOX BOUNDARY
            </span>
          </div>
          <h1 className="text-xl font-bold font-['Space_Grotesk'] text-[#1E1A14] mt-1">
            System & Subsystem Environment
          </h1>
        </div>

        <button
          onClick={loadDiag}
          className="self-start md:self-auto px-3.5 py-1.5 border-2 border-[#CEC8BF] shadow-[0_2px_0_0_#E4DFD7] hover:border-[#2DD4BF] hover:shadow-[0_2px_0_0_#5EEAD4] hover:translate-y-0.5 active:translate-y-1 bg-white rounded-lg text-xs font-bold text-[#5C5347] hover:text-[#0F766E] transition-all flex items-center gap-2 cursor-pointer"
        >
          <RefreshCw size={14} />
          <span>Rerun Doctor</span>
        </button>
      </div>

      {loading ? (
        <div className="py-16 flex items-center justify-center">
          <CuteAnimeLoading
            message="RUNNING SYSTEM DOCTOR..."
            subMessage="( ˘ ³˘)♥ ISOLATION & BINARY CHECK"
          />
        </div>
      ) : (
        diag && (
          <div className="space-y-6">
            {/* Quick Metrics */}
            <div className="grid grid-cols-1 sm:grid-cols-2 gap-4">
              <div className="p-4 rounded-xl border-2 border-[#CEC8BF] bg-[#F7F5F0] shadow-[0_3px_0_0_#E4DFD7] flex items-center gap-3">
                <div className="w-10 h-10 rounded-lg bg-[#CCFBF1] border border-[#2DD4BF] flex items-center justify-center text-[#0F766E]">
                  <Cpu size={20} />
                </div>
                <div>
                  <div className="text-[10px] font-['JetBrains_Mono'] text-[#9C9080]">
                    SYSTEM MEMORY
                  </div>
                  <div className="text-base font-bold font-['JetBrains_Mono'] text-[#1E1A14]">
                    {diag.memoryGb.toFixed(1)} GB Available
                  </div>
                </div>
              </div>

              <div className="p-4 rounded-xl border-2 border-[#CEC8BF] bg-[#F7F5F0] shadow-[0_3px_0_0_#E4DFD7] flex items-center gap-3">
                <div className="w-10 h-10 rounded-lg bg-[#FEF3C7] border border-[#FCD34D] flex items-center justify-center text-[#B45309]">
                  <HardDrive size={20} />
                </div>
                <div>
                  <div className="text-[10px] font-['JetBrains_Mono'] text-[#9C9080]">
                    STORAGE CAPACITY
                  </div>
                  <div className="text-base font-bold font-['JetBrains_Mono'] text-[#1E1A14]">
                    {diag.diskFreeGb.toFixed(1)} GB Free Space
                  </div>
                </div>
              </div>
            </div>

            {/* Subsystems List */}
            <div className="space-y-3">
              <h2 className="text-xs font-bold font-['Space_Grotesk'] text-[#5C5347] uppercase tracking-wider">
                Subsystems & Containment Checks
              </h2>
              <div className="space-y-2">
                {renderComponentRow('Operating System', diag.os)}
                {renderComponentRow('PowerShell CLI', diag.powershell)}
                {renderComponentRow('Git SCM', diag.git)}
                {renderComponentRow('WSL 2 Subsystem', diag.wsl)}
                {renderComponentRow('Docker CLI', diag.docker)}
                {renderComponentRow('Docker Daemon', diag.dockerDaemon)}
              </div>
            </div>
          </div>
        )
      )}
      </div>
    </div>
  )
}
