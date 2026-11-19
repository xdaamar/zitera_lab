import { useState, useEffect } from 'react'
import {
  X,
  Play,
  ExternalLink,
  Sparkles,
  Save,
  Film,
  Video,
  CheckCircle2,
  Info,
} from 'lucide-react'

export function YouTubeIcon({
  size = 20,
  className = '',
}: {
  size?: number
  className?: string
}) {
  return (
    <svg
      width={size}
      height={size}
      viewBox="0 0 24 24"
      fill="currentColor"
      className={className}
    >
      <path d="M23.498 6.186a3.016 3.016 0 0 0-2.122-2.136C19.505 3.545 12 3.545 12 3.545s-7.505 0-9.377.505A3.017 3.017 0 0 0 .502 6.186C0 8.07 0 12 0 12s0 3.93.502 5.814a3.016 3.016 0 0 0 2.122 2.136c1.871.505 9.376.505 9.376.505s7.505 0 9.377-.505a3.015 3.015 0 0 0 2.122-2.136C24 15.93 24 12 24 12s0-3.93-.502-5.814zM9.545 15.568V8.432L15.818 12l-6.273 3.568z" />
    </svg>
  )
}

interface VideoDrawerModalProps {
  isOpen: boolean
  onClose: () => void
  labId: string
  labTitle: string
  defaultYoutubeUrl?: string
}

function extractYouTubeVideoId(urlOrId: string): string | null {
  if (!urlOrId) return null
  const trimmed = urlOrId.trim()

  // If already 11 alphanumeric chars like dQw4w9WgXcQ
  if (/^[a-zA-Z0-9_-]{11}$/.test(trimmed)) {
    return trimmed
  }

  // Regex matches standard watch?v=, youtu.be/, embed/
  const match = trimmed.match(
    /(?:youtu\.be\/|youtube\.com\/(?:embed\/|v\/|watch\?v=|watch\?.+&v=))([\w-]{11})/
  )
  return match ? match[1] : null
}

export function VideoDrawerModal({
  isOpen,
  onClose,
  labId,
  labTitle,
  defaultYoutubeUrl = '',
}: VideoDrawerModalProps) {
  const [videoInput, setVideoInput] = useState('')
  const [activeVideoId, setActiveVideoId] = useState<string | null>(null)
  const [isEditing, setIsEditing] = useState(false)
  const [savedSuccess, setSavedSuccess] = useState(false)

  // Load configured video URL from localStorage or default
  useEffect(() => {
    try {
      const stored = localStorage.getItem(`zitera_video_${labId}`)
      const initial = stored || defaultYoutubeUrl || ''
      setVideoInput(initial)
      setActiveVideoId(extractYouTubeVideoId(initial))
    } catch {
      setActiveVideoId(extractYouTubeVideoId(defaultYoutubeUrl))
    }
  }, [labId, defaultYoutubeUrl, isOpen])

  const handleSaveVideo = () => {
    const extracted = extractYouTubeVideoId(videoInput)
    try {
      if (videoInput.trim()) {
        localStorage.setItem(`zitera_video_${labId}`, videoInput.trim())
      } else {
        localStorage.removeItem(`zitera_video_${labId}`)
      }
    } catch {
      // Ignored
    }
    setActiveVideoId(extracted)
    setIsEditing(false)
    setSavedSuccess(true)
    setTimeout(() => setSavedSuccess(false), 2500)
  }

  if (!isOpen) return null

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center p-3 sm:p-5 bg-black/75 backdrop-blur-sm animate-in fade-in duration-200">
      <div
        className="w-full max-w-4xl max-h-[92vh] flex flex-col rounded-2xl border-2 border-[#2DD4BF] bg-[#FCFBF8] shadow-[0_12px_40px_rgba(0,0,0,0.35)] overflow-hidden"
        onClick={(e) => e.stopPropagation()}
      >
        {/* Header Bar */}
        <div className="flex items-center justify-between px-5 py-4 border-b border-[#E6E2D8] bg-white shrink-0">
          <div className="flex items-center gap-3">
            <div className="w-10 h-10 rounded-xl bg-gradient-to-br from-[#EF4444] to-[#DC2626] text-white flex items-center justify-center shadow-xs">
              <YouTubeIcon size={22} />
            </div>
            <div>
              <div className="flex items-center gap-2">
                <span className="text-[10px] font-bold font-['JetBrains_Mono'] px-2 py-0.5 rounded-full bg-[#FEF2F2] text-[#DC2626] border border-[#FECACA] uppercase">
                  BOOTCAMP VIDEO
                </span>
                <span className="text-xs font-['JetBrains_Mono'] text-[#78716C]">
                  // LAB {labId}
                </span>
              </div>
              <h3 className="text-sm sm:text-base font-bold font-['Space_Grotesk'] text-[#1E1A14] mt-0.5">
                {labTitle}
              </h3>
            </div>
          </div>

          <div className="flex items-center gap-2">
            <button
              onClick={() => setIsEditing(!isEditing)}
              className="px-2.5 py-1.5 text-xs font-['Space_Grotesk'] font-bold rounded-lg border border-[#E6E2D8] bg-[#F7F5F0] hover:bg-[#EFECE6] text-[#5C5347] transition-all flex items-center gap-1.5 cursor-pointer"
              title="Atur Tautan Video YouTube"
            >
              <Film size={14} className="text-[#DC2626]" />
              <span className="hidden sm:inline">Pengaturan URL</span>
            </button>
            <button
              onClick={onClose}
              className="p-1.5 rounded-lg border border-[#E6E2D8] bg-[#F7F5F0] hover:bg-[#FEE2E2] text-[#78716C] hover:text-[#DC2626] transition-all cursor-pointer"
            >
              <X size={18} />
            </button>
          </div>
        </div>

        {/* Modal Body */}
        <div className="flex-1 overflow-y-auto p-4 sm:p-6 space-y-4">
          {/* Optional URL Configuration Bar */}
          {isEditing && (
            <div className="p-4 rounded-xl border border-[#2DD4BF] bg-[#F0FDF4]/60 space-y-3 animate-in fade-in duration-150">
              <div className="flex items-center gap-2 text-xs font-bold font-['Space_Grotesk'] text-[#0F766E]">
                <Sparkles size={15} />
                <span>Konfigurasi Tautan Video YouTube untuk Lab Ini</span>
              </div>
              <p className="text-xs text-[#5C5347]">
                Tempelkan URL YouTube dari video pembahasan materi atau walkthrough lab ini. Video akan langsung tersemat dan dapat ditonton oleh peserta bootcamp.
              </p>
              <div className="flex flex-col sm:flex-row gap-2">
                <input
                  type="text"
                  placeholder="https://www.youtube.com/watch?v=... atau https://youtu.be/..."
                  value={videoInput}
                  onChange={(e) => setVideoInput(e.target.value)}
                  className="flex-1 px-3 py-2 text-xs bg-white border border-[#E6E2D8] rounded-lg font-mono focus:outline-none focus:border-[#2DD4BF]"
                />
                <button
                  onClick={handleSaveVideo}
                  className="px-4 py-2 text-xs font-bold font-['Space_Grotesk'] bg-[#0F766E] hover:bg-[#0D655E] text-white rounded-lg flex items-center justify-center gap-1.5 transition-all cursor-pointer"
                >
                  <Save size={14} />
                  <span>Simpan & Terapkan</span>
                </button>
              </div>
              {savedSuccess && (
                <div className="flex items-center gap-1.5 text-xs text-[#16A34A] font-bold">
                  <CheckCircle2 size={14} />
                  <span>Tautan video berhasil diperbarui!</span>
                </div>
              )}
            </div>
          )}

          {/* Video Player or Placeholder */}
          {activeVideoId ? (
            <div className="space-y-3">
              {/* Responsive 16:9 YouTube Embed */}
              <div className="relative w-full aspect-video rounded-xl overflow-hidden border-2 border-[#1E1A14] bg-black shadow-md">
                <iframe
                  src={`https://www.youtube-nocookie.com/embed/${activeVideoId}?autoplay=1&rel=0&modestbranding=1`}
                  title={`Zitera Bootcamp Video: ${labTitle}`}
                  allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share"
                  allowFullScreen
                  className="absolute inset-0 w-full h-full border-0"
                />
              </div>

              {/* Action pill below video */}
              <div className="flex flex-wrap items-center justify-between gap-2 pt-2">
                <div className="flex items-center gap-2 text-xs text-[#5C5347]">
                  <span className="w-2 h-2 rounded-full bg-[#10B981] animate-pulse" />
                  <span className="font-['JetBrains_Mono'] font-bold text-[#0F766E]">
                    YouTube Stream Active (ID: {activeVideoId})
                  </span>
                </div>
                <a
                  href={`https://www.youtube.com/watch?v=${activeVideoId}`}
                  target="_blank"
                  rel="noopener noreferrer"
                  className="text-xs text-[#DC2626] hover:underline flex items-center gap-1 font-['Space_Grotesk'] font-bold"
                >
                  <span>Buka di Aplikasi YouTube</span>
                  <ExternalLink size={13} />
                </a>
              </div>
            </div>
          ) : (
            /* Elegant Placeholder Card when video is not yet configured */
            <div className="rounded-xl border-2 border-dashed border-[#E6E2D8] bg-white p-8 sm:p-12 text-center space-y-4">
              <div className="w-16 h-16 rounded-2xl bg-gradient-to-br from-[#FEE2E2] to-[#FECACA] text-[#DC2626] mx-auto flex items-center justify-center shadow-xs">
                <Video size={30} />
              </div>
              <div className="max-w-md mx-auto space-y-1.5">
                <h4 className="text-base sm:text-lg font-bold font-['Space_Grotesk'] text-[#1E1A14]">
                  Video Bootcamp Sedang Dalam Produksi
                </h4>
                <p className="text-xs sm:text-sm text-[#78716C] leading-relaxed">
                  Materi rekaman audio-visual terstruktur untuk modul ini akan segera diunggah ke YouTube. Kamu dapat menempelkan tautan video YouTube secara mandiri kapan saja.
                </p>
              </div>

              <div className="pt-2">
                <button
                  onClick={() => setIsEditing(true)}
                  className="px-4 py-2.5 text-xs font-bold font-['Space_Grotesk'] bg-[#DC2626] hover:bg-[#B91C1C] text-white rounded-xl shadow-xs transition-all flex items-center gap-2 mx-auto cursor-pointer"
                >
                  <Play size={15} />
                  <span>Masukkan Tautan Video YouTube</span>
                </button>
              </div>

              <div className="pt-3 border-t border-[#F0EDE8] max-w-lg mx-auto flex items-center justify-center gap-2 text-[11px] text-[#A8A29E]">
                <Info size={13} />
                <span>
                  Video dapat ditonton langsung tanpa meninggalkan aplikasi dan tanpa distorsi iklan.
                </span>
              </div>
            </div>
          )}

          {/* Quick Learning Guide & Takeaway Notes */}
          <div className="p-4 rounded-xl border border-[#E6E2D8] bg-white space-y-2.5">
            <div className="flex items-center gap-2 text-xs font-bold font-['Space_Grotesk'] text-[#1E1A14]">
              <Sparkles size={14} className="text-[#0F766E]" />
              <span>Panduan Menonton Efektif untuk Web Pentester:</span>
            </div>
            <ul className="text-xs text-[#5C5347] space-y-1.5 list-disc pl-5 leading-relaxed font-['Plus_Jakarta_Sans']">
              <li>
                <strong>Fokus pada Logika HTTP:</strong> Amati alur request dan response di tab Network atau proxy Burp Suite saat instruktur mendemonstrasikan eksploitasi.
              </li>
              <li>
                <strong>Jalankan Bersamaan:</strong> Buka terminal atau browser lab di samping modul untuk menguji coba payload langsung secara *live*.
              </li>
              <li>
                <strong>Pahami Vektor Bisnis:</strong> Catat bagaimana celah ini berdampak finansial pada entitas komersial agar dapat menyusun laporan bug bounty bernilai tinggi.
              </li>
            </ul>
          </div>
        </div>

        {/* Footer */}
        <div className="px-5 py-3 border-t border-[#E6E2D8] bg-[#F7F5F0] flex items-center justify-between text-xs font-['Space_Grotesk'] shrink-0">
          <span className="text-[#78716C] text-[11px] font-['JetBrains_Mono']">
            ZITERA BOOTCAMP // MULTIMEDIA ENGINE
          </span>
          <button
            onClick={onClose}
            className="px-4 py-1.5 rounded-lg border border-[#E6E2D8] bg-white hover:bg-[#F0EDE8] text-[#1E1A14] font-bold transition-all cursor-pointer"
          >
            Tutup Pemutar
          </button>
        </div>
      </div>
    </div>
  )
}
