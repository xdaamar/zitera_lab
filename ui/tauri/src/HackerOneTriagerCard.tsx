import { useState, useEffect } from 'react'
import {
  Send,
  Sparkles,
  AlertTriangle,
  DollarSign,
  FileCheck,
  RefreshCw,
  CheckCircle2,
  ShieldCheck,
} from 'lucide-react'
import confetti from 'canvas-confetti'
import type { BugBountyReportData } from './lessonInteractiveData'

interface HackerOneTriagerCardProps {
  labId: string
  referenceReport: BugBountyReportData
  targetPort: number
}

interface TriageResult {
  score: number
  status: 'TRIAGED' | 'NEEDS_MORE_INFO' | 'SPAM_REJECTED'
  payoutAwarded: string
  feedback: string[]
  triagedAt: string
}

export function HackerOneTriagerCard({
  labId,
  referenceReport,
  targetPort,
}: HackerOneTriagerCardProps) {
  const [reportTitle, setReportTitle] = useState('')
  const [assetScope, setAssetScope] = useState('')
  const [pocCommand, setPocCommand] = useState('')
  const [impactStatement, setImpactStatement] = useState('')
  const [evaluating, setEvaluating] = useState(false)
  const [result, setResult] = useState<TriageResult | null>(null)

  // Load previously triaged result if exists
  useEffect(() => {
    try {
      const saved = localStorage.getItem(`zitera_triaged_${labId}`)
      if (saved) {
        setResult(JSON.parse(saved))
      }
    } catch {
      // Ignored
    }
  }, [labId])

  const handleAutofill = () => {
    setReportTitle(referenceReport.reportTitle)
    setAssetScope(referenceReport.assetScope)
    setPocCommand(referenceReport.pocCurl)
    setImpactStatement(referenceReport.impact)
  }

  const handleEvaluateReport = (e: React.FormEvent) => {
    e.preventDefault()
    setEvaluating(true)

    setTimeout(() => {
      let score = 0
      const feedback: string[] = []

      // 1. Evaluate Title & Vulnerability Specification (25 pts)
      const titleLower = reportTitle.toLowerCase()
      if (
        titleLower.length > 15 &&
        (titleLower.includes('idor') ||
          titleLower.includes('ssrf') ||
          titleLower.includes('sql') ||
          titleLower.includes('supply') ||
          titleLower.includes('key') ||
          titleLower.includes('price') ||
          titleLower.includes('auth') ||
          titleLower.includes('integrity') ||
          titleLower.includes('log') ||
          titleLower.includes('exception') ||
          titleLower.includes('bypass') ||
          titleLower.includes('flaw'))
      ) {
        score += 25
        feedback.push('[PASS] Judul laporan profesional & mengidentifikasi kelas kerentanan secara presisi (+25).')
      } else {
        feedback.push('[REVISION] Judul laporan kurang spesifik. Sebutkan nama aset, kelas bug, dan dampak ringkas.')
      }

      // 2. Evaluate In-Scope Asset / Endpoint (25 pts)
      const assetLower = assetScope.toLowerCase()
      const portStr = String(targetPort)
      if (
        assetLower.length > 5 &&
        (assetLower.includes(portStr) ||
          assetLower.includes('apexfin') ||
          assetLower.includes('cloudpulse') ||
          assetLower.includes('packnexus') ||
          assetLower.includes('safekey') ||
          assetLower.includes('dataflow') ||
          assetLower.includes('nexastore') ||
          assetLower.includes('nexhealth') ||
          assetLower.includes('fleettrack') ||
          assetLower.includes('templatecloud') ||
          assetLower.includes('127.0.0.1'))
      ) {
        score += 25
        feedback.push('[PASS] Target aset berada di dalam cakupan resmi (In-Scope Verification Pass) (+25).')
      } else {
        feedback.push('[REVISION] Format target asset tidak valid atau di luar cakupan program bug bounty.')
      }

      // 3. Evaluate Reproducible PoC / cURL (25 pts)
      const pocLower = pocCommand.toLowerCase()
      if (
        pocLower.includes('curl') ||
        pocLower.includes('http') ||
        pocLower.includes('post') ||
        pocLower.includes('get')
      ) {
        score += 25
        feedback.push('[PASS] Proof of Concept (PoC) memenuhi standar triage: langkah reproduksi deterministik (+25).')
      } else {
        feedback.push('[REVISION] PoC lemah atau tidak menyertakan perintah cURL/HTTP request yang dapat diverifikasi.')
      }

      // 4. Evaluate Business & Regulatory Impact (25 pts)
      const impactLower = impactStatement.toLowerCase()
      if (
        impactLower.length > 25 &&
        (impactLower.includes('exfiltration') ||
          impactLower.includes('breach') ||
          impactLower.includes('unauthorized') ||
          impactLower.includes('bocor') ||
          impactLower.includes('financial') ||
          impactLower.includes('kerugian') ||
          impactLower.includes('gdpr') ||
          impactLower.includes('pdp') ||
          impactLower.includes('sox') ||
          impactLower.includes('compromise') ||
          impactLower.includes('bypass') ||
          impactLower.includes('drain'))
      ) {
        score += 25
        feedback.push('[PASS] Analisis dampak bisnis & risiko regulasi meyakinkan bagi tim CISO/Security (+25).')
      } else {
        feedback.push('[REVISION] Uraian dampak terlalu umum. Jelaskan dampak finansial, data breach, atau kepatuhan regulasi.')
      }

      const isTriaged = score >= 75
      const triageResult: TriageResult = {
        score,
        status: isTriaged ? 'TRIAGED' : 'NEEDS_MORE_INFO',
        payoutAwarded: isTriaged ? referenceReport.bountyRange : '$0 USD',
        feedback,
        triagedAt: new Date().toLocaleTimeString('id-ID', { hour: '2-digit', minute: '2-digit' }),
      }

      setResult(triageResult)
      setEvaluating(false)

      if (isTriaged) {
        try {
          localStorage.setItem(`zitera_triaged_${labId}`, JSON.stringify(triageResult))
        } catch {
          // Ignored
        }
        confetti({
          particleCount: 160,
          spread: 90,
          origin: { y: 0.65 },
          colors: ['#F59E0B', '#10B981', '#2DD4BF', '#6366F1'],
        })
      }
    }, 900)
  }

  const handleReset = () => {
    try {
      localStorage.removeItem(`zitera_triaged_${labId}`)
    } catch {
      // Ignored
    }
    setResult(null)
    setReportTitle('')
    setAssetScope('')
    setPocCommand('')
    setImpactStatement('')
  }

  return (
    <div className="rounded-2xl border-2 border-[#F59E0B] bg-white shadow-[0_5px_0_0_#B45309,0_16px_32px_rgba(245,158,11,0.08)] p-6 md:p-7 space-y-5 animate-in fade-in duration-200">
      {/* Header */}
      <div className="flex flex-wrap items-center justify-between gap-3 pb-4 border-b border-[#E6E2D8]">
        <div className="flex items-center gap-3">
          <div className="w-11 h-11 rounded-xl bg-gradient-to-br from-[#FEF3C7] to-[#FDE68A] border-2 border-[#F59E0B] flex items-center justify-center shadow-xs shrink-0 text-[#B45309]">
            <ShieldCheck size={22} className="text-[#B45309]" />
          </div>
          <div>
            <div className="flex items-center gap-2">
              <span className="text-[10px] font-bold font-['JetBrains_Mono'] px-2 py-0.5 rounded-full bg-[#FEF3C7] text-[#B45309] border border-[#FDE68A] uppercase">
                TRIAGE & GRADING ENGINE
              </span>
              <span className="text-xs font-['JetBrains_Mono'] text-[#78716C]">
                // HACKERONE STANDARDS
              </span>
            </div>
            <h3 className="text-base font-bold font-['Space_Grotesk'] text-[#1E1A14] mt-0.5">
              Simulasi Triage & Penilaian Laporan Bug Bounty
            </h3>
          </div>
        </div>

        <div className="flex items-center gap-2">
          <button
            onClick={handleAutofill}
            className="px-3 py-1.5 text-xs font-['Space_Grotesk'] font-bold rounded-lg border border-[#FDE68A] bg-[#FFFBEB] hover:bg-[#FEF3C7] text-[#B45309] transition-all flex items-center gap-1.5 cursor-pointer"
            title="Isi form dengan draf laporan resmi untuk menguji grading"
          >
            <Sparkles size={14} />
            <span>Isi Draf Otomatis</span>
          </button>
          {result && (
            <button
              onClick={handleReset}
              className="p-1.5 rounded-lg border border-[#E6E2D8] bg-[#F7F5F0] hover:bg-[#F0EDE8] text-[#78716C] transition-all cursor-pointer"
              title="Reset Hasil Penilaian"
            >
              <RefreshCw size={14} />
            </button>
          )}
        </div>
      </div>

      <p className="text-xs text-[#5C5347] leading-relaxed font-['Plus_Jakarta_Sans']">
        Pada platform bug bounty resmi (HackerOne/Bugcrowd), menemukan celah hanyalah 50% pekerjaan. 50% sisanya adalah menyusun laporan komprehensif yang lolos proses validasi triager keamanan. Uji kualitas laporanmu di bawah ini:
      </p>

      {/* Form Submission */}
      <form onSubmit={handleEvaluateReport} className="space-y-4">
        <div className="space-y-1">
          <label className="block text-xs font-['Space_Grotesk'] font-bold text-[#1E1A14]">
            Judul Laporan (*Report Title*)
          </label>
          <input
            type="text"
            placeholder="e.g. [Aset] Kelas Kerentanan pada endpoint /path menyebabkan Dampak..."
            value={reportTitle}
            onChange={(e) => setReportTitle(e.target.value)}
            required
            className="w-full px-3.5 py-2 text-xs bg-[#FCFBF8] border border-[#E6E2D8] rounded-lg font-['Space_Grotesk'] focus:outline-none focus:border-[#F59E0B]"
          />
        </div>

        <div className="space-y-1">
          <label className="block text-xs font-['Space_Grotesk'] font-bold text-[#1E1A14]">
            Target Aset / In-Scope Scope URL
          </label>
          <input
            type="text"
            placeholder={`e.g. http://127.0.0.1:${targetPort}/ atau *.target.com`}
            value={assetScope}
            onChange={(e) => setAssetScope(e.target.value)}
            required
            className="w-full px-3.5 py-2 text-xs bg-[#FCFBF8] border border-[#E6E2D8] rounded-lg font-mono focus:outline-none focus:border-[#F59E0B]"
          />
        </div>

        <div className="space-y-1">
          <label className="block text-xs font-['Space_Grotesk'] font-bold text-[#1E1A14]">
            Proof of Concept (Perintah cURL atau Langkah Reproduksi)
          </label>
          <input
            type="text"
            placeholder={`e.g. curl -s -i "http://127.0.0.1:${targetPort}/..."`}
            value={pocCommand}
            onChange={(e) => setPocCommand(e.target.value)}
            required
            className="w-full px-3.5 py-2 text-xs bg-[#FCFBF8] border border-[#E6E2D8] rounded-lg font-mono text-[#0F766E] focus:outline-none focus:border-[#F59E0B]"
          />
        </div>

        <div className="space-y-1">
          <label className="block text-xs font-['Space_Grotesk'] font-bold text-[#1E1A14]">
            Uraian Dampak Bisnis & Kepatuhan Regulasi (*Business Impact*)
          </label>
          <textarea
            rows={3}
            placeholder="Jelaskan dampak kebocoran data, kerugian finansial, atau pelanggaran privasi jika celah ini dieksploitasi penyerang nyata..."
            value={impactStatement}
            onChange={(e) => setImpactStatement(e.target.value)}
            required
            className="w-full px-3.5 py-2 text-xs bg-[#FCFBF8] border border-[#E6E2D8] rounded-lg font-['Plus_Jakarta_Sans'] focus:outline-none focus:border-[#F59E0B]"
          />
        </div>

        <button
          type="submit"
          disabled={evaluating}
          className="w-full py-2.5 px-4 rounded-xl font-bold font-['Space_Grotesk'] text-xs bg-[#0F766E] hover:bg-[#115E59] text-white shadow-xs transition-all flex items-center justify-center gap-2 cursor-pointer disabled:opacity-50"
        >
          {evaluating ? (
            <>
              <Sparkles size={14} className="animate-spin" />
              <span>Memproses Penilaian Triage HackerOne...</span>
            </>
          ) : (
            <>
              <Send size={14} />
              <span>Kirim ke Triager & Nilai Kualitas Laporan</span>
            </>
          )}
        </button>
      </form>

      {/* Triage Result Card */}
      {result && (
        <div
          className={`p-5 rounded-xl border-2 space-y-3.5 animate-in fade-in duration-200 ${
            result.status === 'TRIAGED'
              ? 'bg-[#F0FDF4] border-[#86EFAC]'
              : 'bg-[#FFFBEB] border-[#FDE68A]'
          }`}
        >
          <div className="flex flex-wrap items-center justify-between gap-2 pb-2.5 border-b border-black/10">
            <div className="flex items-center gap-2">
              {result.status === 'TRIAGED' ? (
                <FileCheck size={20} className="text-[#16A34A]" />
              ) : (
                <AlertTriangle size={20} className="text-[#D97706]" />
              )}
              <div>
                <span className="text-[10px] font-mono font-bold tracking-wider uppercase text-[#5C5347]">
                  STATUS TRIAGE // {result.triagedAt}
                </span>
                <div
                  className={`text-sm font-bold font-['Space_Grotesk'] flex items-center gap-1.5 ${
                    result.status === 'TRIAGED' ? 'text-[#15803D]' : 'text-[#B45309]'
                  }`}
                >
                  {result.status === 'TRIAGED' ? (
                    <>
                      <CheckCircle2 size={16} className="text-[#16A34A]" />
                      <span>TRIAGED: LAPORAN DITERIMA TIM KEAMANAN</span>
                    </>
                  ) : (
                    <>
                      <AlertTriangle size={16} className="text-[#D97706]" />
                      <span>PERLU PERBAIKAN: INFORMASI BELUM LENGKAP</span>
                    </>
                  )}
                </div>
              </div>
            </div>

            <div className="flex items-center gap-2">
              <span className="px-2.5 py-1 rounded bg-white border border-[#E6E2D8] text-xs font-bold font-mono text-[#1E1A14]">
                SKOR: {result.score} / 100
              </span>
              {result.status === 'TRIAGED' && (
                <span className="px-2.5 py-1 rounded bg-[#DCFCE7] border border-[#86EFAC] text-xs font-bold font-['Space_Grotesk'] text-[#15803D] flex items-center gap-1">
                  <DollarSign size={13} />
                  <span>Bounty: {result.payoutAwarded}</span>
                </span>
              )}
            </div>
          </div>

          <div className="space-y-1.5">
            <span className="text-[11px] font-bold font-['Space_Grotesk'] text-[#1E1A14] uppercase tracking-wider">
              Feedback Evaluasi Triager:
            </span>
            <ul className="text-xs space-y-1.5 font-['Plus_Jakarta_Sans']">
              {result.feedback.map((fb, idx) => {
                const isPass = fb.startsWith('[PASS]')
                const cleanText = fb.replace(/^\[(PASS|REVISION)\]\s*/, '')
                return (
                  <li
                    key={idx}
                    className={`flex items-start gap-2 p-2 rounded-lg border ${
                      isPass
                        ? 'bg-[#F0FDF4] border-[#BBF7D0] text-[#166534]'
                        : 'bg-[#FFFBEB] border-[#FDE68A] text-[#92400E]'
                    }`}
                  >
                    {isPass ? (
                      <CheckCircle2 size={14} className="text-[#16A34A] shrink-0 mt-0.5" />
                    ) : (
                      <AlertTriangle size={14} className="text-[#D97706] shrink-0 mt-0.5" />
                    )}
                    <span>{cleanText}</span>
                  </li>
                )
              })}
            </ul>
          </div>
        </div>
      )}
    </div>
  )
}
