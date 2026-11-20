import { useState, useRef } from 'react'
import {
  X,
  Award,
  ShieldCheck,
  CheckCircle2,
  Printer,
  Coins,
} from 'lucide-react'

interface CertificateModalProps {
  isOpen: boolean
  onClose: () => void
  studentName: string
  completionDate?: string
  totalBountyEarned: string
}

export function CertificateModal({
  isOpen,
  onClose,
  studentName,
  completionDate = new Date().toLocaleDateString('id-ID', {
    day: 'numeric',
    month: 'long',
    year: 'numeric',
  }),
  totalBountyEarned,
}: CertificateModalProps) {
  const [copiedLink, setCopiedLink] = useState(false)
  const certRef = useRef<HTMLDivElement>(null)

  if (!isOpen) return null

  const certHash = `ZITERA-CWPT-2026-${Math.abs(
    studentName.split('').reduce((acc, char) => acc + char.charCodeAt(0), 1337)
  ).toString(16).toUpperCase()}`

  const handleCopyHash = () => {
    navigator.clipboard.writeText(`Certificate ID: ${certHash} | Verified by Zitera Security Academy 2026`)
    setCopiedLink(true)
    setTimeout(() => setCopiedLink(false), 2000)
  }

  const handlePrint = () => {
    window.print()
  }

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center p-3 sm:p-5 bg-black/80 backdrop-blur-md animate-in fade-in duration-200 select-none">
      <div
        className="w-full max-w-3xl max-h-[94vh] flex flex-col rounded-2xl border-2 border-[#2DD4BF] bg-[#FCFBF8] shadow-[0_16px_50px_rgba(0,0,0,0.4)] overflow-hidden"
        onClick={(e) => e.stopPropagation()}
      >
        {/* Modal Top Bar */}
        <div className="flex items-center justify-between px-5 py-3.5 border-b border-[#E6E2D8] bg-white shrink-0">
          <div className="flex items-center gap-2">
            <Award className="text-[#0F766E]" size={20} />
            <span className="text-xs font-bold font-['Space_Grotesk'] text-[#1E1A14]">
              OFFICIAL BOOTCAMP CREDENTIAL // ZITERA ACADEMY
            </span>
          </div>

          <div className="flex items-center gap-2">
            <button
              onClick={handlePrint}
              className="px-2.5 py-1.5 text-xs font-bold font-['Space_Grotesk'] bg-[#F7F5F0] hover:bg-[#EFECE6] text-[#5C5347] border border-[#E6E2D8] rounded-lg flex items-center gap-1.5 transition-all cursor-pointer"
              title="Cetak Sertifikat"
            >
              <Printer size={13} />
              <span className="hidden sm:inline">Cetak</span>
            </button>
            <button
              onClick={handleCopyHash}
              className="px-2.5 py-1.5 text-xs font-bold font-['Space_Grotesk'] bg-[#CCFBF1] hover:bg-[#99F6E4] text-[#0F766E] border border-[#2DD4BF] rounded-lg flex items-center gap-1.5 transition-all cursor-pointer"
              title="Salin Verifikasi"
            >
              <CheckCircle2 size={13} />
              <span>{copiedLink ? 'Tersalin' : 'Verifikasi ID'}</span>
            </button>
            <button
              onClick={onClose}
              className="p-1.5 rounded-lg border border-[#E6E2D8] bg-[#F7F5F0] hover:bg-[#FEE2E2] text-[#78716C] hover:text-[#DC2626] transition-all cursor-pointer"
            >
              <X size={18} />
            </button>
          </div>
        </div>

        {/* Certificate Canvas Container */}
        <div className="flex-1 overflow-y-auto p-4 sm:p-6 bg-[#F7F5F0]">
          <div
            ref={certRef}
            className="relative rounded-2xl border-4 border-[#0F766E] bg-gradient-to-b from-[#FFFFFF] via-[#FCFBF8] to-[#F0FDFA] p-6 sm:p-10 shadow-lg text-center space-y-6 overflow-hidden"
          >
            {/* Subtle Guilloche Pattern Background */}
            <div className="absolute inset-2 border-2 border-dashed border-[#2DD4BF]/50 rounded-xl pointer-events-none" />
            <div className="absolute top-0 right-0 w-48 h-48 bg-[radial-gradient(#0f766e15_1px,transparent_1px)] [background-size:12px_12px] pointer-events-none" />
            <div className="absolute bottom-0 left-0 w-48 h-48 bg-[radial-gradient(#0f766e15_1px,transparent_1px)] [background-size:12px_12px] pointer-events-none" />

            {/* Header / Brand */}
            <div className="relative z-10 space-y-2">
              <div className="flex items-center justify-center gap-2">
                <div className="w-8 h-8 rounded-lg bg-[#0F766E] text-white flex items-center justify-center shadow-xs">
                  <ShieldCheck size={18} />
                </div>
                <span className="text-base sm:text-lg font-black font-['Space_Grotesk'] tracking-widest text-[#0F766E]">
                  ZITERA SECURITY ACADEMY
                </span>
              </div>
              <div className="text-[10px] font-mono tracking-widest text-[#78716C] uppercase">
                CENTER OF EXCELLENCE IN OFFENSIVE APPLICATION SECURITY & BUG BOUNTY
              </div>
            </div>

            {/* Main Title */}
            <div className="relative z-10 space-y-1">
              <span className="text-xs font-mono font-bold text-[#B45309] uppercase tracking-wider">
                SERTIFIKAT KELULUSAN & KEAHLIAN PROFESIONAL
              </span>
              <h2 className="text-xl sm:text-2xl md:text-3xl font-black font-['Space_Grotesk'] text-[#1E1A14] tracking-tight">
                Zitera Certified Web Penetration Tester & Bug Hunter
              </h2>
              <div className="text-xs font-mono font-bold text-[#0F766E]">
                (CREDENTIAL DESIGNATION: Z-CWPT // 2026-2027)
              </div>
            </div>

            {/* Recipient */}
            <div className="relative z-10 py-3 space-y-2">
              <p className="text-xs font-sans text-[#78716C]">
                Dengan ini menerangkan bahwa peserta bootcamp di bawah ini:
              </p>
              <div className="text-2xl sm:text-3xl font-black font-['Space_Grotesk'] text-[#0F766E] border-b-2 border-[#2DD4BF] max-w-md mx-auto pb-2">
                {studentName || 'Security Researcher'}
              </div>
              <p className="text-xs leading-relaxed text-[#5C5347] max-w-xl mx-auto font-['Plus_Jakarta_Sans'] pt-1">
                Telah berhasil menyelesaikan seluruh rangkaian materi kurikulum <strong>OWASP Top 10:2025/2026</strong>, membuktikan eksploitasi pada 10 target entitas komersial nyata, menguasai audit manual parameter HTTP, dan menyusun laporan standar <strong>HackerOne & Bugcrowd</strong> dengan simulasi total bounty:
              </p>
              <div className="inline-flex items-center gap-1.5 px-3 py-1 rounded-full bg-[#DCFCE7] border border-[#86EFAC] text-xs font-mono font-bold text-[#15803D]">
                <Coins className="w-3.5 h-3.5 text-[#15803D]" />
                <span>Total Bounty Portfolio Terakumulasi: {totalBountyEarned}</span>
              </div>
            </div>

            {/* Signatures & Seal */}
            <div className="relative z-10 grid grid-cols-2 gap-6 pt-4 border-t border-[#E6E2D8] text-xs">
              <div className="text-left space-y-1 font-mono">
                <div className="text-[10px] text-[#9C9080] uppercase">TANGGAL PENYELESAIAN</div>
                <div className="font-bold text-[#1E1A14]">{completionDate}</div>
                <div className="text-[9px] text-[#78716C] break-all">{certHash}</div>
              </div>

              <div className="text-right space-y-1">
                <div className="text-[10px] text-[#9C9080] uppercase font-mono">DEWAN SERTIFIKASI</div>
                <div className="font-bold font-['Space_Grotesk'] text-[#1E1A14]">
                  Zitera Core Security Team
                </div>
                <div className="text-[10px] text-[#0F766E] font-bold flex items-center justify-end gap-1">
                  <CheckCircle2 className="w-3.5 h-3.5 text-[#0F766E]" />
                  <span>VERIFIED CRYPTOGRAPHIC PASS</span>
                </div>
              </div>
            </div>
          </div>
        </div>

        {/* Footer */}
        <div className="px-5 py-3 border-t border-[#E6E2D8] bg-white flex items-center justify-between text-xs font-['Space_Grotesk'] shrink-0">
          <span className="text-[11px] font-mono text-[#78716C]">
            CREDENTIAL HASH: {certHash}
          </span>
          <button
            onClick={onClose}
            className="px-4 py-1.5 rounded-lg border border-[#E6E2D8] bg-[#F7F5F0] hover:bg-[#EFECE6] text-[#1E1A14] font-bold transition-all cursor-pointer"
          >
            Tutup Sertifikat
          </button>
        </div>
      </div>
    </div>
  )
}
