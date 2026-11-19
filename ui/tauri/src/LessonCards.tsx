import { useState } from 'react'
import {
  Lightbulb,
  Cpu,
  Footprints,
  ShieldCheck,
  ExternalLink,
  Copy,
  Check,
  ChevronDown,
  ChevronUp,
  Terminal as TerminalIcon,
  CheckCircle2,
  TrendingUp,
  Layers,
  Sparkles,
  Lock,
  Unlock,
  Split,
  SquareCode,
  DollarSign,
  ShieldAlert,
} from 'lucide-react'
import type {
  LabInteractiveCurriculum,
  WalkthroughStep,
  BugBountyReportData,
} from './lessonInteractiveData'

// ── 1. ANALOGY CARD COMPONENT ──
export function AnalogyCard({
  data,
}: {
  data: LabInteractiveCurriculum['analogy']
}) {
  return (
    <div className="rounded-2xl border-2 border-[#2DD4BF] bg-white shadow-[0_5px_0_0_#0F766E,0_16px_32px_rgba(15,118,110,0.08)] p-6 md:p-7 space-y-6 animate-in fade-in duration-200">
      {/* Top Banner with Real-World Icon */}
      <div className="flex flex-wrap items-center justify-between gap-4 pb-4 border-b border-[#E6E2D8]">
        <div className="flex items-center gap-3.5">
          <div className="w-13 h-13 rounded-2xl bg-gradient-to-br from-[#CCFBF1] to-[#99F6E4] border-2 border-[#2DD4BF] flex items-center justify-center text-2xl shadow-xs shrink-0">
            <span>{data.icon || '🏨 🔑'}</span>
          </div>
          <div>
            <span className="text-[10px] font-bold font-['JetBrains_Mono'] px-2.5 py-0.5 rounded-full bg-[#CCFBF1] text-[#0F766E] border border-[#2DD4BF]/60 uppercase tracking-wider">
              ANALOGI KEHIDUPAN NYATA // MENTAL MODEL
            </span>
            <h3 className="text-lg md:text-xl font-bold font-['Space_Grotesk'] text-[#1E1A14] mt-1">
              {data.storyTitle}
            </h3>
          </div>
        </div>

        <div className="flex items-center gap-1.5 px-3 py-1 rounded-lg bg-[#F7F5F0] border border-[#E6E2D8] text-[11px] font-['Space_Grotesk'] font-bold text-[#5C5347]">
          <Sparkles size={14} className="text-[#0F766E]" />
          <span>Konsep Tanpa Jargon</span>
        </div>
      </div>

      {/* Friendly Real-World Scenario Narrative */}
      <div className="p-4 md:p-5 rounded-xl bg-gradient-to-r from-[#F0FDF4]/50 via-white to-[#F7F5F0] border border-[#E6E2D8] leading-relaxed text-[#3D372E] text-[14.5px] font-['Plus_Jakarta_Sans'] shadow-2xs">
        <p className="whitespace-pre-line">{data.realWorldScenario}</p>
      </div>

      {/* Comparison Matrix: Dunia Fisik vs Dunia Digital */}
      <div className="space-y-3">
        <div className="flex items-center justify-between text-xs font-['Space_Grotesk'] font-bold text-[#1E1A14]">
          <span className="uppercase tracking-wider">
            TABEL PERBANDINGAN: DUNIA FISIK VS DUNIA DIGITAL
          </span>
          <span className="text-[11px] text-[#78716C] font-['JetBrains_Mono']">
            {data.comparisonMatrix.length} Poin Kunci
          </span>
        </div>

        <div className="overflow-x-auto rounded-xl border-2 border-[#E6E2D8] bg-white shadow-xs">
          <table className="w-full text-left border-collapse text-xs md:text-sm font-['Plus_Jakarta_Sans']">
            <thead>
              <tr className="bg-[#F7F5F0] border-b-2 border-[#E6E2D8] text-xs font-['Space_Grotesk'] font-bold text-[#1E1A14] uppercase tracking-wider">
                <th className="py-3 px-4 w-1/3">Dunia Fisik (Kehidupan Nyata)</th>
                <th className="py-3 px-4 w-1/3">Dunia Digital (Web & Server)</th>
                <th className="py-3 px-4 w-1/3">Pelajaran Keamanan</th>
              </tr>
            </thead>
            <tbody className="divide-y divide-[#E6E2D8]">
              {data.comparisonMatrix.map((row, idx) => (
                <tr
                  key={idx}
                  className="hover:bg-[#F0FDF4]/50 transition-colors even:bg-[#FCFBF8]"
                >
                  <td className="py-3 px-4 text-[#5C5347] font-medium align-top">
                    <span className="inline-block mr-1.5 text-base">🏢</span>
                    {row.physical}
                  </td>
                  <td className="py-3 px-4 text-[#0F766E] font-medium font-['JetBrains_Mono'] text-xs align-top">
                    <span className="inline-block mr-1.5 text-base">💻</span>
                    {row.digital}
                  </td>
                  <td className="py-3 px-4 text-[#1E1A14] font-semibold align-top">
                    <span className="inline-block mr-1.5 text-base">🛡️</span>
                    {row.takeaway}
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      </div>

      {/* Intuitive Takeaway Card */}
      <div className="rounded-xl border-2 border-[#FCD34D] bg-gradient-to-r from-[#FEF3C7] to-[#FFFBEB] p-4 md:p-5 shadow-[0_3px_0_0_#F59E0B] flex items-start gap-3.5">
        <div className="w-10 h-10 rounded-xl bg-[#F59E0B] text-white flex items-center justify-center shrink-0 shadow-xs mt-0.5">
          <Lightbulb size={20} />
        </div>
        <div className="space-y-1">
          <span className="text-[10px] font-bold font-['JetBrains_Mono'] uppercase tracking-wider text-[#B45309]">
            INTUITIVE TAKEAWAY // PRINSIP UTAMA
          </span>
          <p className="text-xs md:text-sm font-['Plus_Jakarta_Sans'] font-bold text-[#78350F] leading-relaxed">
            {data.intuitiveTakeaway}
          </p>
        </div>
      </div>
    </div>
  )
}

// ── 2. CORE CONCEPT CARD COMPONENT ──
export function CoreConceptCard({
  data,
}: {
  data: LabInteractiveCurriculum['concept']
}) {
  const [activeMindset, setActiveMindset] = useState<'attacker' | 'defender'>('attacker')

  return (
    <div className="rounded-2xl border-2 border-[#E6E2D8] bg-white shadow-[0_5px_0_0_#CEC8BF,0_16px_32px_rgba(0,0,0,0.04)] p-6 md:p-7 space-y-6 animate-in fade-in duration-200">
      {/* Concept Header */}
      <div className="flex flex-wrap items-center justify-between gap-3 pb-4 border-b border-[#E6E2D8]">
        <div className="flex items-center gap-3">
          <div className="w-11 h-11 rounded-xl bg-[#0F766E] text-white flex items-center justify-center shadow-xs shrink-0">
            <Cpu size={22} />
          </div>
          <div>
            <span className="text-[10px] font-bold font-['JetBrains_Mono'] px-2 py-0.5 rounded bg-[#CCFBF1] text-[#0F766E] border border-[#2DD4BF]/50 uppercase tracking-wider">
              BEDAH TEKNIS & MEKANISME
            </span>
            <h3 className="text-base md:text-lg font-bold font-['Space_Grotesk'] text-[#1E1A14]">
              Mengapa Celah Ini Ada & Dampak Bisnis
            </h3>
          </div>
        </div>
      </div>

      {/* Grid: 2 Main Pillars */}
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-5">
        {/* Pillar 1: Mengapa Celah Ini Ada? (Root Causes) */}
        <div className="rounded-xl border-2 border-[#FCA5A5] bg-gradient-to-b from-[#FEF2F2]/60 to-white p-5 shadow-[0_3px_0_0_#EF4444] space-y-4">
          <div className="flex items-center gap-2 pb-2 border-b border-[#FECACA]">
            <span className="w-3 h-3 rounded-full bg-[#EF4444] inline-block animate-pulse" />
            <h4 className="text-xs md:text-sm font-bold font-['Space_Grotesk'] text-[#991B1B] uppercase tracking-wider">
              🔴 Mengapa Celah Ini Ada? (Jebakan Developer)
            </h4>
          </div>

          <div className="space-y-3">
            {data.whyItExists.map((item, idx) => (
              <div
                key={idx}
                className="p-3.5 rounded-lg bg-white border border-[#FECACA] shadow-2xs space-y-1.5"
              >
                <div className="text-xs font-bold font-['Space_Grotesk'] text-[#B91C1C] flex items-center gap-1.5">
                  <span className="w-4 h-4 rounded-full bg-[#FEE2E2] text-[#DC2626] font-mono text-[10px] flex items-center justify-center shrink-0">
                    {idx + 1}
                  </span>
                  <span>{item.title}</span>
                </div>
                <p className="text-xs text-[#5C5347] font-['Plus_Jakarta_Sans'] leading-relaxed">
                  {item.explanation}
                </p>
                <div className="p-2 rounded bg-[#FFF1F2] border border-[#FDE2E4] text-[11px] font-['JetBrains_Mono'] text-[#BE123C]">
                  ⚠️ Kesalahan: {item.mistake}
                </div>
              </div>
            ))}
          </div>
        </div>

        {/* Pillar 2: Dampak Nyata pada Bisnis (Commercial & Legal Impact) */}
        <div className="rounded-xl border-2 border-[#93C5FD] bg-gradient-to-b from-[#EFF6FF]/60 to-white p-5 shadow-[0_3px_0_0_#3B82F6] space-y-4">
          <div className="flex items-center gap-2 pb-2 border-b border-[#BFDBFE]">
            <TrendingUp size={16} className="text-[#1D4ED8]" />
            <h4 className="text-xs md:text-sm font-bold font-['Space_Grotesk'] text-[#1E40AF] uppercase tracking-wider">
              💼 Dampak Nyata pada Bisnis & Regulasi
            </h4>
          </div>

          <div className="grid grid-cols-1 sm:grid-cols-2 gap-3">
            {/* Regulatory Fine Card */}
            <div className="p-3.5 rounded-lg bg-white border border-[#BFDBFE] shadow-2xs space-y-1">
              <span className="text-[10px] font-bold font-['JetBrains_Mono'] text-[#1D4ED8] uppercase">
                ⚖️ Kepatuhan Regulasi
              </span>
              <p className="text-xs text-[#1E3A8A] font-['Plus_Jakarta_Sans'] leading-relaxed">
                {data.businessImpact.regulatory}
              </p>
            </div>

            {/* Financial Losses Card */}
            <div className="p-3.5 rounded-lg bg-white border border-[#BFDBFE] shadow-2xs space-y-1">
              <span className="text-[10px] font-bold font-['JetBrains_Mono'] text-[#1D4ED8] uppercase">
                💰 Kerugian Finansial
              </span>
              <p className="text-xs text-[#1E3A8A] font-['Plus_Jakarta_Sans'] leading-relaxed">
                {data.businessImpact.financial}
              </p>
            </div>

            {/* Reputational Loss Card */}
            <div className="p-3.5 rounded-lg bg-white border border-[#BFDBFE] shadow-2xs space-y-1">
              <span className="text-[10px] font-bold font-['JetBrains_Mono'] text-[#1D4ED8] uppercase">
                📉 Reputasi & Kepercayaan
              </span>
              <p className="text-xs text-[#1E3A8A] font-['Plus_Jakarta_Sans'] leading-relaxed">
                {data.businessImpact.reputational}
              </p>
            </div>

            {/* Bounty Reward Card */}
            <div className="p-3.5 rounded-lg bg-gradient-to-br from-[#FEF3C7] to-white border border-[#FCD34D] shadow-2xs space-y-1">
              <span className="text-[10px] font-bold font-['JetBrains_Mono'] text-[#B45309] uppercase">
                🎯 Nilai Bug Bounty
              </span>
              <p className="text-xs text-[#78350F] font-['Plus_Jakarta_Sans'] font-bold leading-relaxed">
                {data.businessImpact.bountyReward}
              </p>
            </div>
          </div>
        </div>
      </div>

      {/* Attacker vs Defender Mindset Interactive Box */}
      <div className="rounded-xl border-2 border-[#E6E2D8] bg-[#F7F5F0] p-4 md:p-5 space-y-3">
        <div className="flex flex-wrap items-center justify-between gap-3">
          <span className="text-xs font-bold font-['Space_Grotesk'] text-[#1E1A14] uppercase tracking-wider">
            🧠 SIMULASI POLA PIKIR: ATTACKER VS DEFENDER
          </span>

          <div className="flex items-center gap-1.5 bg-white p-1 rounded-lg border border-[#E6E2D8]">
            <button
              onClick={() => setActiveMindset('attacker')}
              className={`px-3 py-1 rounded text-xs font-['Space_Grotesk'] font-bold transition-all cursor-pointer ${
                activeMindset === 'attacker'
                  ? 'bg-[#EF4444] text-white shadow-2xs'
                  : 'text-[#5C5347] hover:bg-[#F0EDE8]'
              }`}
            >
              Pola Pikir Peretas (Attacker)
            </button>
            <button
              onClick={() => setActiveMindset('defender')}
              className={`px-3 py-1 rounded text-xs font-['Space_Grotesk'] font-bold transition-all cursor-pointer ${
                activeMindset === 'defender'
                  ? 'bg-[#0F766E] text-white shadow-2xs'
                  : 'text-[#5C5347] hover:bg-[#F0EDE8]'
              }`}
            >
              Pola Pikir Pembela (Defender)
            </button>
          </div>
        </div>

        <div className="p-4 rounded-lg bg-white border border-[#E6E2D8] text-xs md:text-sm font-['Plus_Jakarta_Sans'] leading-relaxed shadow-2xs">
          {activeMindset === 'attacker' ? (
            <div className="space-y-1.5 text-[#991B1B]">
              <span className="font-bold font-['Space_Grotesk'] flex items-center gap-1.5 text-xs text-[#DC2626]">
                <Unlock size={14} />
                PERSPEKTIF PERETAS // BUG BOUNTY HUNTER:
              </span>
              <p>{data.attackerVsDefender.attackerPerspective}</p>
            </div>
          ) : (
            <div className="space-y-1.5 text-[#0F766E]">
              <span className="font-bold font-['Space_Grotesk'] flex items-center gap-1.5 text-xs text-[#0F766E]">
                <Lock size={14} />
                PERSPEKTIF ARSITEK KEAMANAN // DEFENSIVE DEV:
              </span>
              <p>{data.attackerVsDefender.defenderPerspective}</p>
            </div>
          )}
        </div>
      </div>
    </div>
  )
}

// ── 3. INTERACTIVE WALKTHROUGH CARD COMPONENT ──
export function InteractiveWalkthroughCard({
  data,
  targetPort,
  onSendToTerminal,
}: {
  data: LabInteractiveCurriculum['walkthrough']
  targetPort: number
  onSendToTerminal?: (cmd: string) => void
}) {
  const [selectedStepIdx, setSelectedStepIdx] = useState(0)
  const [expandedAccordions, setExpandedAccordions] = useState<Record<number, boolean>>({
    0: true, // First step accordion open by default
  })
  const [copiedStep, setCopiedStep] = useState<number | null>(null)

  const activeStep: WalkthroughStep | undefined = data.steps[selectedStepIdx]

  const toggleAccordion = (idx: number) => {
    setExpandedAccordions((prev) => ({
      ...prev,
      [idx]: !prev[idx],
    }))
  }

  const handleCopyCommand = async (cmd: string, idx: number) => {
    try {
      await navigator.clipboard.writeText(cmd)
      setCopiedStep(idx)
      setTimeout(() => setCopiedStep(null), 2000)
    } catch {}
  }

  return (
    <div className="rounded-2xl border-2 border-[#0F766E] bg-white shadow-[0_5px_0_0_#094D47,0_16px_32px_rgba(15,118,110,0.1)] p-6 md:p-7 space-y-6 animate-in fade-in duration-200">
      {/* Walkthrough Header Bar */}
      <div className="flex flex-wrap items-center justify-between gap-3 pb-4 border-b border-[#E6E2D8]">
        <div className="flex items-center gap-3">
          <div className="w-11 h-11 rounded-xl bg-[#0F766E] text-white flex items-center justify-center shadow-xs shrink-0">
            <Footprints size={22} />
          </div>
          <div>
            <span className="text-[10px] font-bold font-['JetBrains_Mono'] px-2.5 py-0.5 rounded-full bg-[#CCFBF1] text-[#0F766E] border border-[#2DD4BF]/60 uppercase tracking-wider">
              INTERACTIVE STEP-BY-STEP WALKTHROUGH
            </span>
            <h3 className="text-base md:text-lg font-bold font-['Space_Grotesk'] text-[#1E1A14]">
              {data.summary}
            </h3>
          </div>
        </div>

        <div className="flex items-center gap-2">
          <span className="text-[11px] font-['JetBrains_Mono'] font-bold px-2.5 py-1 rounded bg-[#F0EDE8] text-[#5C5347] border border-[#E6E2D8]">
            TARGET PORT: {targetPort}
          </span>
        </div>
      </div>

      {/* Step Navigation Pills */}
      <div className="flex items-center gap-2 overflow-x-auto pb-1 scrollbar-none">
        {data.steps.map((step, idx) => {
          const isSelected = selectedStepIdx === idx
          return (
            <button
              key={idx}
              onClick={() => setSelectedStepIdx(idx)}
              className={`flex items-center gap-2 px-3.5 py-2 rounded-xl text-xs font-['Space_Grotesk'] font-bold border-2 transition-all cursor-pointer whitespace-nowrap ${
                isSelected
                  ? 'bg-[#0F766E] border-[#094D47] text-white shadow-[0_3px_0_0_#063934]'
                  : 'bg-[#F7F5F0] border-[#E6E2D8] text-[#5C5347] hover:bg-[#F0EDE8]'
              }`}
            >
              <span
                className={`w-5 h-5 rounded-full text-[11px] font-mono flex items-center justify-center font-bold ${
                  isSelected ? 'bg-white text-[#0F766E]' : 'bg-[#E6E2D8] text-[#5C5347]'
                }`}
              >
                {step.stepNumber}
              </span>
              <span>{step.badge}</span>
            </button>
          )
        })}
      </div>

      {/* Active Step Dedicated Card */}
      {activeStep && (
        <div className="rounded-xl border-2 border-[#2DD4BF] bg-gradient-to-b from-[#F0FDF4]/30 to-white p-5 md:p-6 shadow-xs space-y-4">
          {/* Step Headline & Action */}
          <div className="flex flex-wrap items-center justify-between gap-3 pb-3 border-b border-[#E6E2D8]">
            <div className="space-y-1">
              <div className="flex items-center gap-2">
                <span className="px-2 py-0.5 rounded bg-[#CCFBF1] text-[#0F766E] text-[10px] font-bold font-['JetBrains_Mono']">
                  LANGKAH {activeStep.stepNumber} DARI {data.steps.length}
                </span>
                <span className="text-[11px] font-bold font-['Space_Grotesk'] text-[#78716C]">
                  // {activeStep.badge}
                </span>
              </div>
              <h4 className="text-base font-bold font-['Space_Grotesk'] text-[#1E1A14]">
                {activeStep.title}
              </h4>
            </div>

            {activeStep.endpoint && (
              <div className="flex items-center gap-2">
                <code className="px-2.5 py-1 rounded bg-[#F0EDE8] text-[#0F766E] font-['JetBrains_Mono'] text-xs font-bold border border-[#E6E2D8]">
                  {activeStep.method || 'GET'} {activeStep.endpoint}
                </code>
                <button
                  onClick={() => window.open(`http://127.0.0.1:${targetPort}${activeStep.endpoint}`, '_blank')}
                  className="px-2 py-1 rounded bg-[#F0EDE8] hover:bg-[#E6E2D8] text-[#0F766E] text-xs font-['Space_Grotesk'] font-bold flex items-center gap-1 transition-colors cursor-pointer"
                  title="Buka endpoint ini di browser"
                >
                  <span className="hidden sm:inline">Buka</span>
                  <ExternalLink size={12} />
                </button>
              </div>
            )}
          </div>

          {/* Step Description */}
          <p className="text-xs md:text-sm text-[#3D372E] font-['Plus_Jakarta_Sans'] leading-relaxed">
            {activeStep.description}
          </p>

          {/* Click-to-Copy curl / API Command Box */}
          {activeStep.command && (
            <div className="space-y-2">
              <div className="flex items-center justify-between text-xs font-['Space_Grotesk'] font-bold text-[#5C5347]">
                <span>COMMAND / REQUEST TERMINAL</span>
                <span className="text-[11px] font-['JetBrains_Mono'] text-[#9C9080]">
                  One-Click Copy & Run
                </span>
              </div>

              <div className="rounded-xl border-2 border-[#1E293B] bg-[#090D16] p-3.5 shadow-xs flex flex-col sm:flex-row sm:items-center justify-between gap-3">
                <div className="flex items-center gap-2 overflow-x-auto text-xs font-['JetBrains_Mono'] text-[#2DD4BF] scrollbar-none">
                  <span className="text-[#9CA3AF] select-none">$</span>
                  <span className="whitespace-pre">{activeStep.command}</span>
                </div>

                <div className="flex items-center gap-2 shrink-0">
                  {onSendToTerminal && (
                    <button
                      onClick={() => onSendToTerminal(activeStep.command || '')}
                      className="px-2.5 py-1.5 rounded-lg bg-[#0F766E] hover:bg-[#115E59] active:bg-[#094D47] text-white text-xs font-['Space_Grotesk'] font-bold flex items-center gap-1.5 shadow-xs transition-all cursor-pointer"
                      title="Kirim perintah ini ke Terminal Tilix"
                    >
                      <TerminalIcon size={13} />
                      <span className="hidden sm:inline">Kirim ke Terminal</span>
                    </button>
                  )}

                  <button
                    onClick={() => handleCopyCommand(activeStep.command || '', selectedStepIdx)}
                    className={`px-3 py-1.5 rounded-lg text-xs font-['Space_Grotesk'] font-bold flex items-center gap-1.5 transition-all cursor-pointer ${
                      copiedStep === selectedStepIdx
                        ? 'bg-[#10B981] text-white'
                        : 'bg-[#1E293B] hover:bg-[#334155] text-slate-200 border border-slate-700'
                    }`}
                  >
                    {copiedStep === selectedStepIdx ? (
                      <>
                        <Check size={13} className="text-white" />
                        <span>Tersalin!</span>
                      </>
                    ) : (
                      <>
                        <Copy size={13} />
                        <span>Salin Command</span>
                      </>
                    )}
                  </button>
                </div>
              </div>
            </div>
          )}

          {/* Expected Response Preview Card */}
          {activeStep.expectedResponse && (
            <div className="space-y-1.5">
              <div className="flex items-center justify-between text-xs font-['Space_Grotesk'] font-bold text-[#5C5347]">
                <span>RESPONS YANG DIHARAPKAN</span>
                {activeStep.expectedStatus && (
                  <span className="px-2 py-0.5 rounded bg-[#DCFCE7] text-[#15803D] text-[10px] font-bold font-['JetBrains_Mono']">
                    {activeStep.expectedStatus}
                  </span>
                )}
              </div>

              <div className="p-3 rounded-lg bg-[#1E1A14] border border-[#3D372E] text-xs font-['JetBrains_Mono'] text-[#A7F3D0] leading-relaxed overflow-x-auto">
                <pre className="whitespace-pre-wrap">{activeStep.expectedResponse}</pre>
              </div>
            </div>
          )}

          {/* Collapsible Accordion: Di Balik Layar: Apa yang terjadi di server? */}
          <div className="rounded-xl border border-[#2DD4BF]/60 bg-[#CCFBF1]/30 overflow-hidden transition-all">
            <button
              onClick={() => toggleAccordion(selectedStepIdx)}
              className="w-full px-4 py-3 flex items-center justify-between text-left text-xs font-['Space_Grotesk'] font-bold text-[#0F766E] hover:bg-[#CCFBF1]/50 transition-colors cursor-pointer"
            >
              <div className="flex items-center gap-2">
                <span className="text-base">💡</span>
                <span>Di Balik Layar: Apa yang terjadi di sisi server?</span>
              </div>
              {expandedAccordions[selectedStepIdx] ? (
                <ChevronUp size={16} />
              ) : (
                <ChevronDown size={16} />
              )}
            </button>

            {expandedAccordions[selectedStepIdx] && (
              <div className="px-4 pb-4 pt-1 text-xs text-[#065F46] font-['Plus_Jakarta_Sans'] leading-relaxed border-t border-[#2DD4BF]/30 bg-white/70 animate-in fade-in duration-150">
                <p>{activeStep.underTheHood}</p>
              </div>
            )}
          </div>
        </div>
      )}
    </div>
  )
}

// ── 4. REMEDIATION CARD COMPONENT (SIDE-BY-SIDE / TABBED) ──
export function RemediationCard({
  data,
}: {
  data: LabInteractiveCurriculum['remediation']
}) {
  const [viewMode, setViewMode] = useState<'sideBySide' | 'tabbed'>('sideBySide')
  const [activeTab, setActiveTab] = useState<'vulnerable' | 'secure'>('secure')
  const [copiedVulnerable, setCopiedVulnerable] = useState(false)
  const [copiedSecure, setCopiedSecure] = useState(false)

  const handleCopy = async (code: string, isVulnerable: boolean) => {
    try {
      await navigator.clipboard.writeText(code)
      if (isVulnerable) {
        setCopiedVulnerable(true)
        setTimeout(() => setCopiedVulnerable(false), 2000)
      } else {
        setCopiedSecure(true)
        setTimeout(() => setCopiedSecure(false), 2000)
      }
    } catch {}
  }

  return (
    <div className="rounded-2xl border-2 border-[#10B981] bg-white shadow-[0_5px_0_0_#059669,0_16px_32px_rgba(16,185,129,0.1)] p-6 md:p-7 space-y-6 animate-in fade-in duration-200">
      {/* Remediation Header Bar */}
      <div className="flex flex-wrap items-center justify-between gap-3 pb-4 border-b border-[#E6E2D8]">
        <div className="flex items-center gap-3">
          <div className="w-11 h-11 rounded-xl bg-[#059669] text-white flex items-center justify-center shadow-xs shrink-0">
            <ShieldCheck size={22} />
          </div>
          <div>
            <span className="text-[10px] font-bold font-['JetBrains_Mono'] px-2.5 py-0.5 rounded-full bg-[#DCFCE7] text-[#15803D] border border-[#86EFAC] uppercase tracking-wider">
              DEFENSIVE REMEDIATION & SECURE CODING
            </span>
            <h3 className="text-base md:text-lg font-bold font-['Space_Grotesk'] text-[#1E1A14]">
              {data.title}
            </h3>
          </div>
        </div>

        {/* View Mode Toggle: Side-by-Side vs Tabs */}
        <div className="flex items-center gap-1.5 bg-[#F7F5F0] p-1 rounded-xl border border-[#E6E2D8]">
          <button
            onClick={() => setViewMode('sideBySide')}
            className={`px-3 py-1.5 rounded-lg text-xs font-['Space_Grotesk'] font-bold flex items-center gap-1.5 transition-all cursor-pointer ${
              viewMode === 'sideBySide'
                ? 'bg-white text-[#1E1A14] shadow-xs border border-[#E6E2D8]'
                : 'text-[#78716C] hover:text-[#1E1A14]'
            }`}
            title="Tampilkan Berdampingan"
          >
            <Split size={14} />
            <span className="hidden sm:inline">Berdampingan</span>
          </button>
          <button
            onClick={() => setViewMode('tabbed')}
            className={`px-3 py-1.5 rounded-lg text-xs font-['Space_Grotesk'] font-bold flex items-center gap-1.5 transition-all cursor-pointer ${
              viewMode === 'tabbed'
                ? 'bg-white text-[#1E1A14] shadow-xs border border-[#E6E2D8]'
                : 'text-[#78716C] hover:text-[#1E1A14]'
            }`}
            title="Tampilkan Tab Bergantian"
          >
            <SquareCode size={14} />
            <span className="hidden sm:inline">Tab Bergantian</span>
          </button>
        </div>
      </div>

      {/* Tab Selectors when in Tabbed View Mode */}
      {viewMode === 'tabbed' && (
        <div className="flex items-center gap-2 border-b border-[#E6E2D8] pb-3">
          <button
            onClick={() => setActiveTab('vulnerable')}
            className={`px-4 py-2 rounded-xl text-xs font-['Space_Grotesk'] font-bold flex items-center gap-2 transition-all cursor-pointer border-2 ${
              activeTab === 'vulnerable'
                ? 'bg-[#FEF2F2] border-[#EF4444] text-[#B91C1C] shadow-xs'
                : 'bg-[#F7F5F0] border-transparent text-[#78716C] hover:bg-[#F0EDE8]'
            }`}
          >
            <span className="w-2.5 h-2.5 rounded-full bg-[#EF4444]" />
            <span>🔴 Kode Rentan (Vulnerable)</span>
          </button>
          <button
            onClick={() => setActiveTab('secure')}
            className={`px-4 py-2 rounded-xl text-xs font-['Space_Grotesk'] font-bold flex items-center gap-2 transition-all cursor-pointer border-2 ${
              activeTab === 'secure'
                ? 'bg-[#F0FDF4] border-[#10B981] text-[#15803D] shadow-xs'
                : 'bg-[#F7F5F0] border-transparent text-[#78716C] hover:bg-[#F0EDE8]'
            }`}
          >
            <span className="w-2.5 h-2.5 rounded-full bg-[#10B981]" />
            <span>🟢 Kode Aman (Secure Implementation)</span>
          </button>
        </div>
      )}

      {/* Code Comparison Container */}
      <div
        className={
          viewMode === 'sideBySide'
            ? 'grid grid-cols-1 lg:grid-cols-2 gap-5'
            : 'space-y-4'
        }
      >
        {/* Vulnerable Code Card */}
        {(viewMode === 'sideBySide' || activeTab === 'vulnerable') && (
          <div className="rounded-xl border-2 border-[#EF4444] bg-[#090D16] shadow-[0_4px_0_0_#991B1B] overflow-hidden flex flex-col">
            <div className="px-4 py-2.5 bg-[#1F1212] border-b border-[#7F1D1D] flex items-center justify-between text-xs">
              <div className="flex items-center gap-2">
                <span className="w-2.5 h-2.5 rounded-full bg-[#EF4444]" />
                <span className="font-['Space_Grotesk'] font-bold text-[#FCA5A5]">
                  🔴 KODE RENTAN ({data.vulnerableSnippet.filename})
                </span>
              </div>
              <button
                onClick={() => handleCopy(data.vulnerableSnippet.code, true)}
                className="px-2 py-0.5 rounded text-[11px] font-['Space_Grotesk'] font-bold bg-[#7F1D1D] hover:bg-[#991B1B] text-white flex items-center gap-1 transition-all cursor-pointer"
              >
                {copiedVulnerable ? <Check size={11} /> : <Copy size={11} />}
                <span>{copiedVulnerable ? 'Tersalin' : 'Salin'}</span>
              </button>
            </div>

            <div className="p-4 font-['JetBrains_Mono'] text-xs text-[#FCA5A5] leading-relaxed overflow-x-auto flex-1 bg-[#090D16]">
              <pre className="whitespace-pre">{data.vulnerableSnippet.code}</pre>
            </div>

            <div className="p-3 bg-[#1F1212] border-t border-[#7F1D1D] text-xs font-['Plus_Jakarta_Sans'] text-[#FCA5A5]">
              ⚠️ <span className="font-bold">Celah:</span>{' '}
              {data.vulnerableSnippet.flawExplanation}
            </div>
          </div>
        )}

        {/* Secure Code Card */}
        {(viewMode === 'sideBySide' || activeTab === 'secure') && (
          <div className="rounded-xl border-2 border-[#10B981] bg-[#090D16] shadow-[0_4px_0_0_#065F46] overflow-hidden flex flex-col">
            <div className="px-4 py-2.5 bg-[#0D241E] border-b border-[#065F46] flex items-center justify-between text-xs">
              <div className="flex items-center gap-2">
                <span className="w-2.5 h-2.5 rounded-full bg-[#10B981]" />
                <span className="font-['Space_Grotesk'] font-bold text-[#6EE7B7]">
                  🟢 KODE AMAN TERMEDIASI ({data.secureSnippet.filename})
                </span>
              </div>
              <button
                onClick={() => handleCopy(data.secureSnippet.code, false)}
                className="px-2 py-0.5 rounded text-[11px] font-['Space_Grotesk'] font-bold bg-[#065F46] hover:bg-[#047857] text-white flex items-center gap-1 transition-all cursor-pointer"
              >
                {copiedSecure ? <Check size={11} /> : <Copy size={11} />}
                <span>{copiedSecure ? 'Tersalin' : 'Salin'}</span>
              </button>
            </div>

            <div className="p-4 font-['JetBrains_Mono'] text-xs text-[#6EE7B7] leading-relaxed overflow-x-auto flex-1 bg-[#090D16]">
              <pre className="whitespace-pre">{data.secureSnippet.code}</pre>
            </div>

            <div className="p-3 bg-[#0D241E] border-t border-[#065F46] text-xs font-['Plus_Jakarta_Sans'] text-[#6EE7B7]">
              🛡️ <span className="font-bold">Perbaikan:</span>{' '}
              {data.secureSnippet.fixExplanation}
            </div>
          </div>
        )}
      </div>

      {/* Security Diffs Matrix */}
      {data.securityDiffs.length > 0 && (
        <div className="space-y-3">
          <div className="flex items-center justify-between text-xs font-['Space_Grotesk'] font-bold text-[#1E1A14]">
            <span className="uppercase tracking-wider">
              HIGHLIGHT PERUBAHAN KEAMANAN (SECURITY DIFFS)
            </span>
          </div>

          <div className="overflow-x-auto rounded-xl border-2 border-[#E6E2D8] bg-white shadow-xs">
            <table className="w-full text-left border-collapse text-xs md:text-sm font-['Plus_Jakarta_Sans']">
              <thead>
                <tr className="bg-[#F7F5F0] border-b-2 border-[#E6E2D8] text-xs font-['Space_Grotesk'] font-bold text-[#1E1A14] uppercase tracking-wider">
                  <th className="py-2.5 px-4 w-1/4">Komponen</th>
                  <th className="py-2.5 px-4 w-1/4 text-[#DC2626]">Sebelum (Rentan)</th>
                  <th className="py-2.5 px-4 w-1/4 text-[#16A34A]">Sesudah (Aman)</th>
                  <th className="py-2.5 px-4 w-1/4">Dampak Keamanan</th>
                </tr>
              </thead>
              <tbody className="divide-y divide-[#E6E2D8]">
                {data.securityDiffs.map((diff, idx) => (
                  <tr key={idx} className="hover:bg-[#F0FDF4]/50 transition-colors">
                    <td className="py-2.5 px-4 font-bold text-[#1E1A14] font-['Space_Grotesk']">
                      {diff.component}
                    </td>
                    <td className="py-2.5 px-4 font-['JetBrains_Mono'] text-xs text-[#DC2626] bg-[#FEF2F2]/50">
                      {diff.before}
                    </td>
                    <td className="py-2.5 px-4 font-['JetBrains_Mono'] text-xs text-[#16A34A] bg-[#F0FDF4]/50 font-bold">
                      {diff.after}
                    </td>
                    <td className="py-2.5 px-4 text-[#5C5347]">{diff.impact}</td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        </div>
      )}

      {/* 4 Defense-in-Depth Principles */}
      <div className="space-y-3">
        <span className="text-xs font-bold font-['Space_Grotesk'] text-[#1E1A14] uppercase tracking-wider">
          🛡️ PRINSIP PERTAHANAN BERLAPIS (DEFENSE IN DEPTH)
        </span>

        <div className="grid grid-cols-1 sm:grid-cols-2 gap-3">
          {data.defensePrinciples.map((dp, idx) => (
            <div
              key={idx}
              className="p-4 rounded-xl border border-[#E6E2D8] bg-[#F7F5F0] hover:border-[#10B981] transition-all shadow-2xs space-y-1"
            >
              <div className="flex items-center gap-2 text-xs font-bold font-['Space_Grotesk'] text-[#065F46]">
                <CheckCircle2 size={15} className="text-[#10B981] shrink-0" />
                <span>{dp.title}</span>
              </div>
              <p className="text-xs text-[#5C5347] font-['Plus_Jakarta_Sans'] leading-relaxed">
                {dp.desc}
              </p>
            </div>
          ))}
        </div>
      </div>
    </div>
  )
}

// ── 5. ARCHITECTURE FLOW CARD COMPONENT ──
export function ArchitectureFlowCard({
  targetPort,
}: {
  targetPort: number
}) {
  const [activeLayer, setActiveLayer] = useState<number>(3)

  const layers = [
    {
      step: 1,
      title: 'Klien / Peramban Web',
      subtitle: 'Pengguna mengirim request HTTP (misal: GET /wire/2)',
      detail:
        'Pengguna atau peretas mengirimkan parameter langsung melalui bilah alamat peramban atau script cURL terminal.',
      badge: 'CLIENT LAYER',
      color: 'border-[#38BDF8] text-[#0284C7]',
    },
    {
      step: 2,
      title: 'Reverse Proxy / API Gateway',
      subtitle: `Listening on localhost:${targetPort} (Rate limit & TLS termination)`,
      detail:
        'Meneruskan koneksi HTTP mentah ke backend. Layer ini memverifikasi format request namun tidak mengetahui konteks bisnis data.',
      badge: 'GATEWAY LAYER',
      color: 'border-[#A855F7] text-[#7E22CE]',
    },
    {
      step: 3,
      title: 'Middleware Autentikasi (AuthN)',
      subtitle: 'Memverifikasi cookie sesi (Apakah token sah?)',
      detail:
        'Memvalidasi tanda tangan JWT atau cookie session_user. Pada celah Broken Access Control, layer ini BERHASIL lolos karena user memang memiliki akun sah.',
      badge: 'AUTHN LAYER',
      color: 'border-[#F59E0B] text-[#B45309]',
    },
    {
      step: 4,
      title: 'Policy Enforcement Point (PEP - AuthZ)',
      subtitle: 'TITIK KRITIS: Pengecekan Hak Kepemilikan Objek',
      detail:
        'Inilah titik di mana celah IDOR terjadi: jika server melewatkan pengecekan "Apakah user_id == session.user_id?", data terlarang akan bocor!',
      badge: 'AUTHZ PEP LAYER',
      color: 'border-[#EF4444] text-[#B91C1C]',
    },
    {
      step: 5,
      title: 'Database Terisolasi (Data Access Layer)',
      subtitle: 'Eksekusi Query SQL & Tenant Scoping',
      detail:
        'Sistem aman selalu membatasi query dengan klausul AND user_id = :session_user_id untuk menjamin isolasi multi-tenant.',
      badge: 'DATABASE LAYER',
      color: 'border-[#10B981] text-[#047857]',
    },
  ]

  return (
    <div className="rounded-2xl border-2 border-[#A855F7] bg-white shadow-[0_5px_0_0_#7E22CE,0_16px_32px_rgba(168,85,247,0.08)] p-6 md:p-7 space-y-6 animate-in fade-in duration-200">
      {/* Architecture Header */}
      <div className="flex flex-wrap items-center justify-between gap-3 pb-4 border-b border-[#E6E2D8]">
        <div className="flex items-center gap-3">
          <div className="w-11 h-11 rounded-xl bg-[#7E22CE] text-white flex items-center justify-center shadow-xs shrink-0">
            <Layers size={22} />
          </div>
          <div>
            <span className="text-[10px] font-bold font-['JetBrains_Mono'] px-2.5 py-0.5 rounded-full bg-[#EDE9FE] text-[#7C3AED] border border-[#C084FC]/60 uppercase tracking-wider">
              INTERACTIVE SYSTEM ARCHITECTURE
            </span>
            <h3 className="text-base md:text-lg font-bold font-['Space_Grotesk'] text-[#1E1A14]">
              Alur Arsitektur Penegakan Kontrol Akses Multi-Lapis
            </h3>
          </div>
        </div>

        <span className="text-xs font-['JetBrains_Mono'] text-[#78716C]">
          Klik layer untuk menginspeksi alur
        </span>
      </div>

      {/* Layer Flow Stepper */}
      <div className="space-y-3">
        {layers.map((l) => {
          const isSelected = activeLayer === l.step
          return (
            <div
              key={l.step}
              onClick={() => setActiveLayer(l.step)}
              className={`p-4 rounded-xl border-2 transition-all cursor-pointer ${
                isSelected
                  ? `bg-[#FAF5FF] ${l.color} shadow-xs scale-[1.01]`
                  : 'bg-white border-[#E6E2D8] hover:border-[#CBD5E1] text-[#5C5347]'
              }`}
            >
              <div className="flex flex-wrap items-center justify-between gap-2">
                <div className="flex items-center gap-3">
                  <span
                    className={`w-7 h-7 rounded-lg text-xs font-mono font-bold flex items-center justify-center shrink-0 ${
                      isSelected
                        ? 'bg-[#7E22CE] text-white'
                        : 'bg-[#F0EDE8] text-[#5C5347]'
                    }`}
                  >
                    {l.step}
                  </span>
                  <div>
                    <h5 className="text-xs md:text-sm font-bold font-['Space_Grotesk'] text-[#1E1A14]">
                      {l.title}
                    </h5>
                    <p className="text-xs text-[#78716C] font-['Plus_Jakarta_Sans']">
                      {l.subtitle}
                    </p>
                  </div>
                </div>

                <span className="text-[10px] font-bold font-['JetBrains_Mono'] px-2 py-0.5 rounded bg-white border border-[#E6E2D8]">
                  {l.badge}
                </span>
              </div>

              {isSelected && (
                <div className="mt-3 pt-3 border-t border-[#E9D5FF] text-xs font-['Plus_Jakarta_Sans'] leading-relaxed text-[#581C87] animate-in fade-in duration-150">
                  <p>{l.detail}</p>
                </div>
              )}
            </div>
          )
        })}
      </div>
    </div>
  )
}

// ── 6. BUG BOUNTY & HACKERONE REPORT CARD COMPONENT ──
export function BugBountyReportCard({
  report,
  onSendToTerminal,
}: {
  report: BugBountyReportData
  onSendToTerminal?: (cmd: string) => void
}) {
  const [copiedReport, setCopiedReport] = useState(false)
  const [copiedCurl, setCopiedCurl] = useState(false)

  const handleCopyReport = () => {
    const markdown = `# ${report.reportTitle}

**Severity**: ${report.severity} (${report.cvssScore} - \`${report.cvssVector}\`)
**Asset / Scope**: \`${report.assetScope}\`
**Estimated Bounty**: ${report.bountyRange}

---

## 1. Vulnerability Summary
${report.summary}

## 2. Steps to Reproduce
${report.stepsToReproduce.join('\n')}

## 3. Proof of Concept (cURL)
\`\`\`bash
${report.pocCurl}
\`\`\`

## 4. Impact Assessment
${report.impact}

## 5. Remediation Recommendation
${report.remediationRecommendation}

---
*Reported via Zitera Lab Web Pentest & Bug Bounty Bootcamp*
`
    navigator.clipboard.writeText(markdown)
    setCopiedReport(true)
    setTimeout(() => setCopiedReport(false), 2000)
  }

  const handleCopyCurl = () => {
    navigator.clipboard.writeText(report.pocCurl)
    setCopiedCurl(true)
    setTimeout(() => setCopiedCurl(false), 2000)
  }

  const severityColor =
    report.severity === 'Critical'
      ? 'bg-[#FEF2F2] border-[#FECACA] text-[#DC2626]'
      : report.severity === 'High'
      ? 'bg-[#FFFBEB] border-[#FDE68A] text-[#D97706]'
      : 'bg-[#EFF6FF] border-[#BFDBFE] text-[#2563EB]'

  return (
    <div className="rounded-2xl border-2 border-[#F59E0B] bg-white shadow-[0_5px_0_0_#B45309,0_16px_32px_rgba(245,158,11,0.08)] p-6 md:p-7 space-y-6 animate-in fade-in duration-200">
      {/* Top Banner with HackerOne / Bounty Branding */}
      <div className="flex flex-wrap items-center justify-between gap-4 pb-4 border-b border-[#E6E2D8]">
        <div className="flex items-center gap-3.5">
          <div className="w-12 h-12 rounded-2xl bg-gradient-to-br from-[#FEF3C7] to-[#FDE68A] border-2 border-[#F59E0B] flex items-center justify-center text-2xl shadow-xs shrink-0">
            <span>🏆 💰</span>
          </div>
          <div>
            <span className="text-[10px] font-bold font-['JetBrains_Mono'] px-2.5 py-0.5 rounded-full bg-[#FEF3C7] text-[#B45309] border border-[#FDE68A] uppercase tracking-wider">
              HACKERONE & BUGCROWD // REPORT BLUEPRINT
            </span>
            <h3 className="text-base md:text-lg font-bold font-['Space_Grotesk'] text-[#1E1A14] mt-1">
              {report.reportTitle}
            </h3>
          </div>
        </div>

        <div className="flex items-center gap-2">
          <div className={`px-3 py-1 rounded-lg border text-xs font-['Space_Grotesk'] font-bold flex items-center gap-1.5 ${severityColor}`}>
            <ShieldAlert size={14} />
            <span>{report.severity.toUpperCase()} ({report.cvssScore})</span>
          </div>
          <div className="px-3 py-1 rounded-lg bg-[#DCFCE7] border border-[#86EFAC] text-xs font-['Space_Grotesk'] font-bold text-[#15803D] flex items-center gap-1.5">
            <DollarSign size={14} />
            <span>Bounty: {report.bountyRange}</span>
          </div>
        </div>
      </div>

      {/* Scope & CVSS Meta Grid */}
      <div className="grid grid-cols-1 md:grid-cols-2 gap-3">
        <div className="p-3.5 rounded-xl bg-[#F7F5F0] border border-[#E6E2D8] space-y-1">
          <span className="text-[10px] font-mono font-bold text-[#78716C] uppercase tracking-wider">
            TARGET IN-SCOPE ASSET
          </span>
          <div className="text-xs font-mono font-bold text-[#1E1A14] truncate">
            {report.assetScope}
          </div>
        </div>
        <div className="p-3.5 rounded-xl bg-[#F7F5F0] border border-[#E6E2D8] space-y-1">
          <span className="text-[10px] font-mono font-bold text-[#78716C] uppercase tracking-wider">
            CVSS v3.1 VECTOR
          </span>
          <div className="text-xs font-mono text-[#0F766E] truncate">
            {report.cvssVector}
          </div>
        </div>
      </div>

      {/* Executive Summary */}
      <div className="space-y-2">
        <h4 className="text-xs font-bold font-['Space_Grotesk'] text-[#1E1A14] uppercase tracking-wider">
          1. Vulnerability Summary & Root Cause
        </h4>
        <div className="p-4 rounded-xl bg-white border border-[#E6E2D8] text-xs sm:text-sm text-[#3D372E] leading-relaxed font-['Plus_Jakarta_Sans'] shadow-2xs">
          {report.summary}
        </div>
      </div>

      {/* Steps to Reproduce */}
      <div className="space-y-2">
        <h4 className="text-xs font-bold font-['Space_Grotesk'] text-[#1E1A14] uppercase tracking-wider">
          2. Step-by-Step Proof of Concept (Triage Checklist)
        </h4>
        <div className="p-4 rounded-xl bg-[#F7F5F0] border border-[#E6E2D8] space-y-2">
          {report.stepsToReproduce.map((step, idx) => (
            <div key={idx} className="text-xs text-[#5C5347] font-['Plus_Jakarta_Sans'] leading-relaxed flex items-start gap-2">
              <span className="w-1.5 h-1.5 rounded-full bg-[#D97706] mt-1.5 shrink-0" />
              <span>{step}</span>
            </div>
          ))}
        </div>
      </div>

      {/* PoC cURL Terminal Box */}
      <div className="space-y-2">
        <div className="flex items-center justify-between text-xs font-['Space_Grotesk'] font-bold text-[#1E1A14]">
          <span className="uppercase tracking-wider">3. Minimal Reproducing Command (PoC cURL)</span>
          <div className="flex items-center gap-1.5">
            <button
              onClick={handleCopyCurl}
              className="px-2 py-1 text-[11px] rounded bg-white border border-[#E6E2D8] hover:bg-[#F0EDE8] text-[#5C5347] flex items-center gap-1 transition-all cursor-pointer"
            >
              {copiedCurl ? <Check size={12} className="text-[#10B981]" /> : <Copy size={12} />}
              <span>{copiedCurl ? 'Tersalin' : 'Salin cURL'}</span>
            </button>
            {onSendToTerminal && (
              <button
                onClick={() => onSendToTerminal(report.pocCurl)}
                className="px-2 py-1 text-[11px] rounded bg-[#0F766E] hover:bg-[#0D655E] text-white flex items-center gap-1 transition-all cursor-pointer font-bold"
              >
                <TerminalIcon size={12} />
                <span>Kirim ke Tilix</span>
              </button>
            )}
          </div>
        </div>
        <div className="p-3.5 rounded-xl bg-[#090D16] border border-[#1E293B] text-[#38BDF8] font-mono text-xs overflow-x-auto shadow-inner">
          <code>{report.pocCurl}</code>
        </div>
      </div>

      {/* Impact & Remediation Grid */}
      <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
        <div className="p-4 rounded-xl bg-[#FEF2F2]/50 border border-[#FECACA] space-y-1.5">
          <h5 className="text-xs font-bold font-['Space_Grotesk'] text-[#991B1B] uppercase tracking-wider">
            4. Business & Regulatory Impact
          </h5>
          <p className="text-xs text-[#7F1D1D] leading-relaxed font-['Plus_Jakarta_Sans']">
            {report.impact}
          </p>
        </div>

        <div className="p-4 rounded-xl bg-[#F0FDF4]/50 border border-[#BBF7D0] space-y-1.5">
          <h5 className="text-xs font-bold font-['Space_Grotesk'] text-[#166534] uppercase tracking-wider">
            5. Remediation for Engineering
          </h5>
          <p className="text-xs text-[#14532D] leading-relaxed font-['Plus_Jakarta_Sans']">
            {report.remediationRecommendation}
          </p>
        </div>
      </div>

      {/* Main Copy Action Button */}
      <div className="pt-2">
        <button
          onClick={handleCopyReport}
          className="w-full py-3 px-4 rounded-xl font-bold font-['Space_Grotesk'] text-xs sm:text-sm bg-gradient-to-r from-[#D97706] to-[#B45309] hover:from-[#B45309] hover:to-[#92400E] text-white shadow-xs transition-all flex items-center justify-center gap-2 cursor-pointer"
        >
          {copiedReport ? (
            <>
              <Check size={16} />
              <span>Format Laporan Berhasil Disalin ke Clipboard!</span>
            </>
          ) : (
            <>
              <Copy size={16} />
              <span>Salin Format Laporan Bug Bounty Lengkap (Markdown)</span>
            </>
          )}
        </button>
      </div>
    </div>
  )
}

