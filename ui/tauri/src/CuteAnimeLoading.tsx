import { useEffect, useRef } from 'react'

export interface CuteAnimeLoadingProps {
  message?: string
  subMessage?: string
  height?: number
  compact?: boolean
}

export function CuteAnimeLoading({
  message = 'INITIALIZING CYBER MATRIX...',
  subMessage = '( •̀ ω •́ )✧ NEKO CORE ONLINE',
  compact = false,
}: CuteAnimeLoadingProps) {
  const canvasRef = useRef<HTMLCanvasElement | null>(null)
  const animFrameIdRef = useRef<number | null>(null)

  useEffect(() => {
    if (compact) return

    const canvas = canvasRef.current
    if (!canvas) return
    const ctx = canvas.getContext('2d')
    if (!ctx) return

    let startTime = performance.now()
    const width = 320
    const height = 64
    const amplitude = 12.0
    const frequency = 3.5
    const midY = height / 2

    const catImg = new Image()
    catImg.src = '/assets/calico_cat.png'

    const render = (now: number) => {
      const elapsed = (now - startTime) / 1000
      // Cycle duration is 2.2s (matches Flutter controller)
      const t = (elapsed / 2.2) % 1.0

      ctx.clearRect(0, 0, width, height)

      // 1. Background dashed track
      ctx.beginPath()
      ctx.strokeStyle = 'rgba(226, 232, 240, 0.75)'
      ctx.lineWidth = 2
      ctx.setLineDash([4, 4])
      for (let x = 0; x <= width; x += 3) {
        const y = midY + amplitude * Math.sin((x / width) * 2 * Math.PI * frequency)
        if (x === 0) ctx.moveTo(x, y)
        else ctx.lineTo(x, y)
      }
      ctx.stroke()
      ctx.setLineDash([])

      // 2. Glowing animated rainbow gradient wave
      ctx.beginPath()
      for (let x = 0; x <= width; x += 2) {
        const y = midY + amplitude * Math.sin((x / width) * 2 * Math.PI * frequency + t * 2 * Math.PI)
        if (x === 0) ctx.moveTo(x, y)
        else ctx.lineTo(x, y)
      }

      // Outer Glow
      ctx.save()
      ctx.strokeStyle = 'rgba(45, 212, 191, 0.35)'
      ctx.lineWidth = 7
      ctx.lineCap = 'round'
      ctx.shadowColor = '#2DD4BF'
      ctx.shadowBlur = 10
      ctx.stroke()
      ctx.restore()

      // Rainbow Core
      const grad = ctx.createLinearGradient(0, 0, width, 0)
      grad.addColorStop(0.0, '#2DD4BF') // Mint
      grad.addColorStop(0.33, '#60A5FA') // Sky Blue
      grad.addColorStop(0.66, '#F472B6') // Pink
      grad.addColorStop(1.0, '#FBBF24') // Gold

      ctx.save()
      ctx.strokeStyle = grad
      ctx.lineWidth = 3.5
      ctx.lineCap = 'round'
      ctx.stroke()
      ctx.restore()

      // 3. Floating sparkling stars (Pink, Mint, Gold, Lavender)
      const sparkleColors = ['#F472B6', '#2DD4BF', '#FBBF24', '#A78BFA']
      for (let i = 0; i < 4; i++) {
        const seed = (i * 0.25 + t) % 1.0
        const sx = seed * width
        const sy = 16.0 + 26.0 * Math.sin(seed * Math.PI * 3)
        const size = Math.max(3, 5 + 3 * Math.sin(t * 4 * Math.PI + i))
        const color = sparkleColors[i % sparkleColors.length]

        ctx.save()
        ctx.translate(sx, sy)
        ctx.rotate(t * 4 * Math.PI)
        ctx.fillStyle = color
        ctx.shadowColor = color
        ctx.shadowBlur = 6

        // Draw 4-point sparkle star
        ctx.beginPath()
        for (let p = 0; p < 8; p++) {
          const r = p % 2 === 0 ? size : size * 0.35
          const angle = (p * Math.PI) / 4
          const px = r * Math.cos(angle)
          const py = r * Math.sin(angle)
          if (p === 0) ctx.moveTo(px, py)
          else ctx.lineTo(px, py)
        }
        ctx.closePath()
        ctx.fill()
        ctx.restore()
      }

      // 4. Surfing Calico Cat Mascot Head
      const mascotX = (t * width) % width
      const mascotY = midY + amplitude * Math.sin((mascotX / width) * 2 * Math.PI * frequency + t * 2 * Math.PI)
      const tilt = Math.cos((mascotX / width) * 2 * Math.PI * frequency + t * 2 * Math.PI) * 0.35

      // Glow color cycle (Teal -> Pink -> Blue)
      let glowColor = '#2DD4BF'
      if (t < 0.33) glowColor = '#2DD4BF'
      else if (t < 0.66) glowColor = '#F472B6'
      else glowColor = '#60A5FA'

      ctx.save()
      ctx.translate(mascotX, mascotY)
      ctx.rotate(tilt)

      // Outer glowing ring
      ctx.beginPath()
      ctx.arc(0, 0, 16, 0, 2 * Math.PI)
      ctx.fillStyle = '#FCFBF8'
      ctx.strokeStyle = glowColor
      ctx.lineWidth = 2
      ctx.shadowColor = glowColor
      ctx.shadowBlur = 12
      ctx.fill()
      ctx.stroke()

      // Mascot image in circular clip
      if (catImg.complete && catImg.naturalWidth > 0) {
        ctx.save()
        ctx.beginPath()
        ctx.arc(0, 0, 14, 0, 2 * Math.PI)
        ctx.clip()
        ctx.drawImage(catImg, -14, -14, 28, 28)
        ctx.restore()
      } else {
        // Fallback paw text if image loading
        ctx.fillStyle = '#0F766E'
        ctx.font = 'bold 12px sans-serif'
        ctx.textAlign = 'center'
        ctx.textBaseline = 'middle'
        ctx.fillText('🐾', 0, 0)
      }

      ctx.restore()

      animFrameIdRef.current = requestAnimationFrame(render)
    }

    animFrameIdRef.current = requestAnimationFrame(render)

    return () => {
      if (animFrameIdRef.current) {
        cancelAnimationFrame(animFrameIdRef.current)
      }
    }
  }, [compact])

  // Compact Mode (for buttons & small pills)
  if (compact) {
    return (
      <div className="relative w-7 h-7 flex items-center justify-center shrink-0">
        <div className="absolute inset-0 rounded-full border-2 border-[#2DD4BF] border-t-transparent animate-spin" />
        <div className="w-4 h-4 rounded-full overflow-hidden bg-white shadow-xs animate-pulse">
          <img
            src="/assets/calico_cat.png"
            alt=""
            className="w-full h-full object-cover"
            onError={(e) => {
              ;(e.target as HTMLElement).style.display = 'none'
            }}
          />
        </div>
      </div>
    )
  }

  // Full Widget Mode (matches Flutter CuteAnimeLoading)
  return (
    <div className="flex flex-col items-center justify-center py-6 px-4">
      <div className="relative max-w-sm w-full bg-[#FCFBF8] border-2 border-[#2DD4BF]/60 rounded-2xl p-5 shadow-[0_4px_0_0_#2DD4BF,0_8px_24px_rgba(45,212,191,0.22)] flex flex-col items-center text-center select-none overflow-hidden">
        {/* Subtle background ambient cyber light */}
        <div className="absolute top-0 right-0 w-32 h-32 bg-gradient-to-bl from-[#2DD4BF]/15 via-transparent to-transparent pointer-events-none rounded-full blur-xl" />

        {/* Sinusoidal Wave Canvas */}
        <div className="w-[320px] h-[64px] relative">
          <canvas
            ref={canvasRef}
            width={320}
            height={64}
            className="w-full h-full block"
          />
        </div>

        {/* Animated Cyber Typography */}
        <div className="mt-3 space-y-1">
          {message && (
            <div className="text-xs font-bold font-['Space_Grotesk'] text-[#1E1A14] tracking-wider uppercase drop-shadow-xs">
              {message}
            </div>
          )}
          {subMessage && (
            <div className="text-[11px] font-bold font-['JetBrains_Mono'] text-[#0F766E] tracking-tight flex items-center justify-center gap-1.5">
              <span>{subMessage}</span>
            </div>
          )}
        </div>
      </div>
    </div>
  )
}
