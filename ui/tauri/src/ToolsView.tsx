import { useState, useEffect, useRef } from 'react'
import {
  CheckCircle2,
  AlertTriangle,
  RefreshCw,
} from 'lucide-react'
import { getTools, type ToolItem } from './engineClient'
import { CuteAnimeLoading } from './CuteAnimeLoading'

export function ToolsView() {
  const [tools, setTools] = useState<ToolItem[]>([])
  const [loading, setLoading] = useState(true)
  const isMountedRef = useRef(true)

  const loadTools = async () => {
    setLoading(true)
    try {
      const data = await getTools()
      if (isMountedRef.current) {
        setTools(data)
      }
    } finally {
      if (isMountedRef.current) {
        setLoading(false)
      }
    }
  }

  useEffect(() => {
    isMountedRef.current = true
    loadTools()
    return () => {
      isMountedRef.current = false
    }
  }, [])

  return (
    <div className="flex-1 h-full overflow-hidden relative bg-[#FCFBF8]">
      {/* Pinned Ambient Background Artwork (Matches Flutter bg_tools.jpg) */}
      <div className="absolute inset-0 pointer-events-none z-0 overflow-hidden select-none">
        <img
          src="/assets/bg_tools.jpg"
          alt=""
          className="w-full h-full object-cover object-center opacity-22"
          onError={(e) => {
            ;(e.target as HTMLElement).style.display = 'none'
          }}
        />
        <div className="absolute inset-0 bg-[#FCFBF8]/80" />
      </div>

      <div className="relative z-10 h-full overflow-y-auto p-6 space-y-6">
        {/* Header */}
      <div className="flex flex-col md:flex-row md:items-center justify-between gap-4 border-b border-[#E6E2D8] pb-5">
        <div>
          <div className="flex items-center gap-2">
            <span className="px-2 py-0.5 rounded bg-[#EDE9FE] text-[#6D28D9] border border-[#D8B4FE] text-[10px] font-['Space_Grotesk'] font-bold">
              SECURITY TOOLING
            </span>
            <span className="text-[10px] font-['JetBrains_Mono'] text-[#9C9080]">
              LOCAL RECON & EXPLOIT UTILITIES
            </span>
          </div>
          <h1 className="text-xl font-bold font-['Space_Grotesk'] text-[#1E1A14] mt-1">
            Penetration Testing & Diagnostics Suite
          </h1>
        </div>

        <button
          onClick={loadTools}
          className="self-start md:self-auto px-3.5 py-1.5 border border-[#E6E2D8] hover:border-[#2DD4BF] bg-white rounded-lg text-xs font-bold text-[#5C5347] hover:text-[#0F766E] transition-colors flex items-center gap-2"
        >
          <RefreshCw size={14} />
          <span>Rescan System Tools</span>
        </button>
      </div>

      {loading ? (
        <div className="py-16 flex items-center justify-center">
          <CuteAnimeLoading
            message="QUERYING SECURITY TOOLS..."
            subMessage="( 🛠️ •̀_•́) SCANNING SYSTEM PATH"
          />
        </div>
      ) : (
        <div className="grid grid-cols-1 lg:grid-cols-2 gap-4">
          {tools.map((tool) => (
            <div
              key={tool.id}
              className="p-5 rounded-xl border-2 border-[#CEC8BF] bg-[#FCFBF8] shadow-[0_3px_0_0_#E4DFD7] hover:border-[#2DD4BF] hover:shadow-[0_4px_0_0_#2DD4BF] hover:-translate-y-0.5 transition-all flex flex-col justify-between"
            >
              <div>
                <div className="flex items-center justify-between mb-2">
                  <span className="px-2 py-0.5 rounded bg-[#F0EDE8] border border-[#E6E2D8] text-[10px] font-['JetBrains_Mono'] text-[#5C5347] font-bold">
                    {tool.category.toUpperCase()}
                  </span>
                  <div className="flex items-center gap-1.5">
                    {tool.installed ? (
                      <span className="inline-flex items-center gap-1 px-2 py-0.5 rounded-full bg-[#E6F9F2] border border-[#86EFAC] text-[10px] font-['JetBrains_Mono'] font-bold text-[#16A34A]">
                        <CheckCircle2 size={12} />
                        INSTALLED
                      </span>
                    ) : (
                      <span className="inline-flex items-center gap-1 px-2 py-0.5 rounded-full bg-[#FEF3C7] border border-[#FCD34D] text-[10px] font-['JetBrains_Mono'] font-bold text-[#B45309]">
                        <AlertTriangle size={12} />
                        NOT FOUND
                      </span>
                    )}
                  </div>
                </div>

                <h3 className="text-base font-bold font-['Space_Grotesk'] text-[#1E1A14]">
                  {tool.name}
                </h3>

                {tool.version && (
                  <div className="text-xs font-['JetBrains_Mono'] text-[#0F766E] mt-0.5">
                    Version: {tool.version} (min: {tool.minVersion || 'any'})
                  </div>
                )}

                {tool.path && (
                  <div className="text-[10px] font-['JetBrains_Mono'] text-[#9C9080] truncate mt-1">
                    Path: {tool.path}
                  </div>
                )}

                {/* Capabilities pills */}
                {tool.capabilities.length > 0 && (
                  <div className="flex flex-wrap gap-1 mt-3">
                    {tool.capabilities.map((cap) => (
                      <span
                        key={cap}
                        className="px-2 py-0.5 rounded bg-[#F0EDE8] text-[10px] font-['JetBrains_Mono'] text-[#5C5347]"
                      >
                        {cap}
                      </span>
                    ))}
                  </div>
                )}
              </div>

              {/* Install guide */}
              <div className="mt-4 pt-3 border-t border-[#E6E2D8] text-[11px] font-['JetBrains_Mono'] text-[#5C5347]">
                <div className="font-bold text-[#1E1A14] text-[10px] mb-1">INSTALL COMMAND:</div>
                <code className="block bg-[#101726] text-[#2DD4BF] p-2 rounded text-[10px] select-all overflow-x-auto">
                  {tool.installGuide || `winget install ${tool.id}`}
                </code>
              </div>
            </div>
          ))}
        </div>
      )}
      </div>
    </div>
  )
}
