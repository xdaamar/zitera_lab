import { useState, Children, isValidElement, type ReactNode } from 'react'
import ReactMarkdown from 'react-markdown'
import remarkGfm from 'remark-gfm'
import {
  Copy,
  Check,
  Terminal as TerminalIcon,
  Info,
  Lightbulb,
  AlertTriangle,
  AlertOctagon,
  ShieldAlert,
  ExternalLink,
  Sparkles,
} from 'lucide-react'

interface LessonMarkdownRendererProps {
  markdown: string
  onSendToTerminal?: (command: string) => void
}

interface CodeBlockProps {
  language: string
  code: string
  onSendToTerminal?: (command: string) => void
}

function CodeBlock({ language, code, onSendToTerminal }: CodeBlockProps) {
  const [copied, setCopied] = useState(false)
  const isTerminalCmd = ['bash', 'sh', 'shell', 'zsh', 'curl', 'http'].includes(
    language.toLowerCase()
  ) || code.trim().startsWith('curl ') || code.trim().startsWith('$ ')

  const handleCopy = async () => {
    try {
      // Clean leading '$ ' if copied
      const textToCopy = code
        .split('\n')
        .map((line) => (line.startsWith('$ ') ? line.slice(2) : line))
        .join('\n')
      await navigator.clipboard.writeText(textToCopy)
      setCopied(true)
      setTimeout(() => setCopied(false), 2000)
    } catch {}
  }

  const handleRunInTerminal = () => {
    if (!onSendToTerminal) return
    // Extract first executable line or full block
    const cleanedCmd = code
      .split('\n')
      .map((l) => (l.startsWith('$ ') ? l.slice(2) : l))
      .filter((l) => l.trim().length > 0 && !l.trim().startsWith('#'))
      .join(' && ')
    onSendToTerminal(cleanedCmd || code)
  }

  const getLangBadgeColor = (lang: string) => {
    switch (lang.toLowerCase()) {
      case 'bash':
      case 'sh':
      case 'curl':
        return 'bg-[#0F766E] text-white border-[#2DD4BF]/50'
      case 'python':
      case 'py':
        return 'bg-[#1D4ED8] text-white border-[#60A5FA]/50'
      case 'sql':
        return 'bg-[#B45309] text-white border-[#FBBF24]/50'
      case 'json':
        return 'bg-[#7C3AED] text-white border-[#C084FC]/50'
      case 'http':
        return 'bg-[#0E7490] text-white border-[#38BDF8]/50'
      case 'nginx':
        return 'bg-[#047857] text-white border-[#34D399]/50'
      case 'dockerfile':
      case 'docker':
        return 'bg-[#2563EB] text-white border-[#93C5FD]/50'
      default:
        return 'bg-[#334155] text-slate-200 border-slate-600'
    }
  }

  const lines = code.split('\n')

  return (
    <div className="my-5 rounded-xl border-2 border-[#1E293B] bg-[#090D16] shadow-[0_4px_0_0_#0F172A,0_10px_20px_rgba(0,0,0,0.3)] overflow-hidden">
      {/* Code Header Bar */}
      <div className="px-4 py-2.5 bg-[#101726] border-b border-[#1E293B] flex flex-wrap items-center justify-between gap-2 text-xs">
        <div className="flex items-center gap-2">
          {/* Mac/Tilix Window Dots */}
          <div className="flex items-center gap-1.5 mr-2">
            <span className="w-2.5 h-2.5 rounded-full bg-[#EF4444]/90 inline-block" />
            <span className="w-2.5 h-2.5 rounded-full bg-[#F59E0B]/90 inline-block" />
            <span className="w-2.5 h-2.5 rounded-full bg-[#10B981]/90 inline-block" />
          </div>

          <span
            className={`px-2 py-0.5 rounded text-[10px] font-['JetBrains_Mono'] font-bold uppercase tracking-wider border ${getLangBadgeColor(
              language
            )}`}
          >
            {language || 'TEXT'}
          </span>

          <span className="text-[11px] font-['JetBrains_Mono'] text-slate-400 hidden sm:inline">
            // {lines.length} baris
          </span>
        </div>

        <div className="flex items-center gap-2">
          {isTerminalCmd && onSendToTerminal && (
            <button
              onClick={handleRunInTerminal}
              className="px-2.5 py-1 rounded bg-[#0F766E] hover:bg-[#115E59] active:bg-[#094D47] text-white text-[11px] font-['Space_Grotesk'] font-bold flex items-center gap-1.5 shadow-2xs transition-all cursor-pointer"
              title="Muat perintah ini ke Terminal Tilix"
            >
              <TerminalIcon size={12} />
              <span>Kirim ke Terminal</span>
            </button>
          )}

          <button
            onClick={handleCopy}
            className={`px-2.5 py-1 rounded text-[11px] font-['Space_Grotesk'] font-bold flex items-center gap-1.5 transition-all cursor-pointer ${
              copied
                ? 'bg-[#10B981] text-white'
                : 'bg-[#1E293B] hover:bg-[#334155] text-slate-200 border border-slate-700'
            }`}
            title="Salin kode ke papan klip"
          >
            {copied ? (
              <>
                <Check size={12} className="text-white" />
                <span>Tersalin!</span>
              </>
            ) : (
              <>
                <Copy size={12} />
                <span>Salin</span>
              </>
            )}
          </button>
        </div>
      </div>

      {/* Code Body with Line Numbers */}
      <div className="p-4 overflow-x-auto font-['JetBrains_Mono'] text-xs text-[#E5E7EB] leading-relaxed">
        <div className="min-w-full inline-block">
          {lines.map((line, idx) => (
            <div key={idx} className="flex items-start hover:bg-[#1E293B]/40 px-1 rounded">
              <span className="select-none text-slate-600 text-right pr-4 shrink-0 w-8 text-[11px] mt-0.5">
                {idx + 1}
              </span>
              <span className="flex-1 whitespace-pre">{line || ' '}</span>
            </div>
          ))}
        </div>
      </div>
    </div>
  )
}

function extractText(node: ReactNode): string {
  if (typeof node === 'string') return node
  if (typeof node === 'number') return String(node)
  if (Array.isArray(node)) return node.map(extractText).join('')
  if (isValidElement(node) && (node.props as { children?: ReactNode })?.children) {
    return extractText((node.props as { children?: ReactNode }).children)
  }
  return ''
}

function AlertBlockquote({ children }: { children: ReactNode }) {
  const fullText = extractText(children).trim()
  const match = fullText.match(/^\[!(NOTE|TIP|WARNING|IMPORTANT|CAUTION)\]/i)

  if (!match) {
    return (
      <blockquote className="my-5 border-l-4 border-[#2DD4BF] bg-[#F7F5F0] rounded-r-xl p-4 text-[#5C5347] font-['Plus_Jakarta_Sans'] italic shadow-2xs leading-relaxed">
        {children}
      </blockquote>
    )
  }

  const alertType = match[1].toUpperCase()

  // Clean the alert tag from the children rendering
  const cleanedChildren = Children.map(children, (child) => {
    if (isValidElement(child) && (child.props as { children?: ReactNode })?.children) {
      const pText = extractText(child)
      if (pText.startsWith(`[!${alertType}]`)) {
        const remaining = pText.slice(`[!${alertType}]`.length).trimStart()
        return <p className="mb-0 text-xs md:text-sm leading-relaxed">{remaining}</p>
      }
    }
    return child
  })

  switch (alertType) {
    case 'NOTE':
      return (
        <div className="my-5 rounded-xl border-2 border-[#38BDF8] bg-[#F0F9FF] shadow-[0_3px_0_0_#BAE6FD] p-4 text-[#0369A1] space-y-2">
          <div className="flex items-center gap-2 font-['Space_Grotesk'] font-bold text-xs uppercase tracking-wider text-[#0284C7]">
            <Info size={16} className="text-[#0284C7] shrink-0" />
            <span>CATATAN PENTING // NOTE</span>
          </div>
          <div className="text-xs md:text-sm font-['Plus_Jakarta_Sans'] text-[#075985] leading-relaxed">
            {cleanedChildren}
          </div>
        </div>
      )
    case 'TIP':
      return (
        <div className="my-5 rounded-xl border-2 border-[#34D399] bg-[#F0FDF4] shadow-[0_3px_0_0_#BBF7D0] p-4 text-[#15803D] space-y-2">
          <div className="flex items-center gap-2 font-['Space_Grotesk'] font-bold text-xs uppercase tracking-wider text-[#16A34A]">
            <Lightbulb size={16} className="text-[#16A34A] shrink-0" />
            <span>TIPS & REKOMENDASI // PRO-TIP</span>
          </div>
          <div className="text-xs md:text-sm font-['Plus_Jakarta_Sans'] text-[#14532D] leading-relaxed">
            {cleanedChildren}
          </div>
        </div>
      )
    case 'WARNING':
      return (
        <div className="my-5 rounded-xl border-2 border-[#FBBF24] bg-[#FFFBEB] shadow-[0_3px_0_0_#FDE68A] p-4 text-[#B45309] space-y-2">
          <div className="flex items-center gap-2 font-['Space_Grotesk'] font-bold text-xs uppercase tracking-wider text-[#D97706]">
            <AlertTriangle size={16} className="text-[#D97706] shrink-0" />
            <span>PERINGATAN KEAMANAN // WARNING</span>
          </div>
          <div className="text-xs md:text-sm font-['Plus_Jakarta_Sans'] text-[#78350F] leading-relaxed">
            {cleanedChildren}
          </div>
        </div>
      )
    case 'CAUTION':
      return (
        <div className="my-5 rounded-xl border-2 border-[#F87171] bg-[#FEF2F2] shadow-[0_3px_0_0_#FECACA] p-4 text-[#B91C1C] space-y-2">
          <div className="flex items-center gap-2 font-['Space_Grotesk'] font-bold text-xs uppercase tracking-wider text-[#DC2626]">
            <AlertOctagon size={16} className="text-[#DC2626] shrink-0" />
            <span>PERHATIAN KRITIS // CAUTION</span>
          </div>
          <div className="text-xs md:text-sm font-['Plus_Jakarta_Sans'] text-[#7F1D1D] leading-relaxed">
            {cleanedChildren}
          </div>
        </div>
      )
    case 'IMPORTANT':
    default:
      return (
        <div className="my-5 rounded-xl border-2 border-[#C084FC] bg-[#FAF5FF] shadow-[0_3px_0_0_#E9D5FF] p-4 text-[#7E22CE] space-y-2">
          <div className="flex items-center gap-2 font-['Space_Grotesk'] font-bold text-xs uppercase tracking-wider text-[#9333EA]">
            <ShieldAlert size={16} className="text-[#9333EA] shrink-0" />
            <span>POIN UTAMA // IMPORTANT</span>
          </div>
          <div className="text-xs md:text-sm font-['Plus_Jakarta_Sans'] text-[#581C87] leading-relaxed">
            {cleanedChildren}
          </div>
        </div>
      )
  }
}

export function LessonMarkdownRenderer({
  markdown,
  onSendToTerminal,
}: LessonMarkdownRendererProps) {
  return (
    <div className="lesson-markdown-enhanced text-[#1E1A14]">
      <ReactMarkdown
        remarkPlugins={[remarkGfm]}
        components={{
          // Code & Pre block handling
          pre({ children }) {
            return <div className="my-4">{children}</div>
          },
          code({ className, children }) {
            const match = /language-(\w+)/.exec(className || '')
            const codeString = String(children).replace(/\n$/, '')
            const isMultiline = codeString.includes('\n')
            const isBlock = match || isMultiline

            if (isBlock) {
              const lang = match ? match[1] : 'text'
              return (
                <CodeBlock
                  language={lang}
                  code={codeString}
                  onSendToTerminal={onSendToTerminal}
                />
              )
            }

            return (
              <code className="px-1.5 py-0.5 mx-0.5 rounded-md bg-[#F0EDE8] border border-[#E6E2D8] text-[#0F766E] font-['JetBrains_Mono'] text-[13px] font-semibold tracking-tight shadow-2xs">
                {children}
              </code>
            )
          },

          // Alert callout blockquotes
          blockquote({ children }) {
            return <AlertBlockquote>{children}</AlertBlockquote>
          },

          // Responsive styled tables
          table({ children }) {
            return (
              <div className="my-6 overflow-x-auto rounded-xl border-2 border-[#E6E2D8] bg-white shadow-xs">
                <table className="w-full text-left border-collapse text-xs md:text-sm font-['Plus_Jakarta_Sans']">
                  {children}
                </table>
              </div>
            )
          },
          thead({ children }) {
            return (
              <thead className="bg-[#F7F5F0] border-b-2 border-[#E6E2D8] text-xs font-['Space_Grotesk'] font-bold text-[#1E1A14] uppercase tracking-wider">
                {children}
              </thead>
            )
          },
          th({ children }) {
            return <th className="py-3 px-4 text-[#1E1A14] font-bold">{children}</th>
          },
          tbody({ children }) {
            return <tbody className="divide-y divide-[#E6E2D8]">{children}</tbody>
          },
          tr({ children }) {
            return (
              <tr className="hover:bg-[#F0FDF4]/50 transition-colors even:bg-[#FCFBF8]">
                {children}
              </tr>
            )
          },
          td({ children }) {
            return <td className="py-2.5 px-4 text-[#5C5347]">{children}</td>
          },

          // Checkmarks & styled lists
          ul({ children }) {
            return (
              <ul className="my-4 space-y-2 text-[#3D372E] text-[14.5px] leading-relaxed list-none pl-0">
                {children}
              </ul>
            )
          },
          ol({ children }) {
            return (
              <ol className="my-4 space-y-2 text-[#3D372E] text-[14.5px] leading-relaxed list-none pl-0 [counter-reset:lesson-step]">
                {children}
              </ol>
            )
          },
          li({ children }) {
            return (
              <li className="flex items-start gap-2.5 text-[#3D372E]">
                <span className="w-1.5 h-1.5 rounded-full bg-[#0F766E] shrink-0 mt-2.5 inline-block" />
                <div className="flex-1">{children}</div>
              </li>
            )
          },

          // Anti-fatigue typography headings
          h1({ children }) {
            return (
              <h1 className="text-xl md:text-2xl font-bold font-['Space_Grotesk'] text-[#1E1A14] mt-8 mb-4 pb-2.5 border-b-2 border-[#E6E2D8] flex items-center gap-2.5">
                <Sparkles size={20} className="text-[#0F766E] shrink-0" />
                <span>{children}</span>
              </h1>
            )
          },
          h2({ children }) {
            return (
              <h2 className="text-lg md:text-xl font-bold font-['Space_Grotesk'] text-[#1E1A14] mt-7 mb-3 pb-1.5 border-b border-[#F0EDE8] flex items-center gap-2">
                <span className="w-2 h-2 rounded-full bg-[#2DD4BF] shrink-0" />
                <span>{children}</span>
              </h2>
            )
          },
          h3({ children }) {
            return (
              <h3 className="text-base font-bold font-['Space_Grotesk'] text-[#0F766E] mt-5 mb-2 flex items-center gap-2">
                <span>{children}</span>
              </h3>
            )
          },
          h4({ children }) {
            return (
              <h4 className="text-sm font-bold font-['Space_Grotesk'] text-[#5C5347] mt-4 mb-2">
                {children}
              </h4>
            )
          },

          // Paragraphs with comfortable line-height
          p({ children }) {
            return (
              <p className="text-[14.5px] md:text-base text-[#3D372E] leading-relaxed font-['Plus_Jakarta_Sans'] mb-4">
                {children}
              </p>
            )
          },

          // Links
          a({ href, children }) {
            return (
              <a
                href={href}
                target="_blank"
                rel="noopener noreferrer"
                className="text-[#0F766E] hover:text-[#115E59] font-medium underline underline-offset-2 inline-flex items-center gap-1 transition-colors"
              >
                <span>{children}</span>
                <ExternalLink size={12} className="inline opacity-70" />
              </a>
            )
          },

          // Horizontal rule
          hr() {
            return (
              <div className="my-7 flex items-center justify-center gap-2 text-[#CBD5E1]">
                <div className="h-px bg-[#E6E2D8] flex-1" />
                <span className="w-1.5 h-1.5 rounded-full bg-[#94A3B8]" />
                <span className="w-2 h-2 rounded-full bg-[#0F766E]" />
                <span className="w-1.5 h-1.5 rounded-full bg-[#94A3B8]" />
                <div className="h-px bg-[#E6E2D8] flex-1" />
              </div>
            )
          },
        }}
      >
        {markdown}
      </ReactMarkdown>
    </div>
  )
}
