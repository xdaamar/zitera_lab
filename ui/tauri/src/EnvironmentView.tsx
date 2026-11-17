import { useState, useEffect } from 'react'
import {
  CheckCircle2,
  XCircle,
  HardDrive,
  Cpu,
  RefreshCw,
} from 'lucide-react'
import { checkDoctor, type DiagnosticsResult, type ComponentItem } from './engineClient'

export function EnvironmentView() {
  const [diag, setDiag] = useState<DiagnosticsResult | null>(null)
  const [loading, setLoading] = useState(true)

  const loadDiag = async () => {
    setLoading(true)
    try {
      const data = await checkDoctor()
      setDiag(data)
    } finally {
      setLoading(false)
    }
  }

  useEffect(() => {
    loadDiag()
  }, [])

  const renderComponentRow = (title: string, item?: ComponentItem) => {
    if (!item) return null
    const isReady = item.status === 'READY' || item.installed

    return (
      <div className="p-4 rounded-xl border border-[#E6E2D8] bg-[#FCFBF8] flex items-center justify-between">
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
    <div className="flex-1 overflow-y-auto p-6 space-y-6 bg-[#FCFBF8]">
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
          className="self-start md:self-auto px-3.5 py-1.5 border border-[#E6E2D8] hover:border-[#2DD4BF] bg-white rounded-lg text-xs font-bold text-[#5C5347] hover:text-[#0F766E] transition-colors flex items-center gap-2"
        >
          <RefreshCw size={14} />
          <span>Rerun Doctor</span>
        </button>
      </div>

      {loading ? (
        <div className="py-20 flex flex-col items-center justify-center space-y-3">
          <div className="w-12 h-12 rounded-full border-3 border-[#2DD4BF] border-t-transparent animate-spin" />
          <div className="text-xs font-['JetBrains_Mono'] text-[#5C5347]">
            Validating AppContainer SID and system binaries...
          </div>
        </div>
      ) : (
        diag && (
          <div className="space-y-6">
            {/* Quick Metrics */}
            <div className="grid grid-cols-1 sm:grid-cols-2 gap-4">
              <div className="p-4 rounded-xl border border-[#E6E2D8] bg-[#F7F5F0] flex items-center gap-3">
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

              <div className="p-4 rounded-xl border border-[#E6E2D8] bg-[#F7F5F0] flex items-center gap-3">
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
  )
}
