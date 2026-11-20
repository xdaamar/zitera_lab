import { useEffect, useState, useRef, useCallback } from 'react'

interface Sparkle {
  id: number
  x: number
  y: number
  size: number
  color: string
}

const SPARKLE_COLORS = ['#2DD4BF', '#F59E0B', '#A855F7', '#38BDF8', '#FB7185', '#10B981']

export function KawaiiCursor({ enabled = true }: { enabled?: boolean }) {
  const pawRef = useRef<HTMLDivElement | null>(null)
  const posRef = useRef({ x: -100, y: -100 })
  const isHoverRef = useRef(false)
  const isClickingRef = useRef(false)
  const rafIdRef = useRef<number | null>(null)

  const [isHover, setIsHover] = useState(false)
  const [sparkles, setSparkles] = useState<Sparkle[]>([])
  const nextId = useRef(0)
  const lastSparkleTime = useRef(0)

  const removeSparkle = useCallback((id: number) => {
    setSparkles((prev) => prev.filter((s) => s.id !== id))
  }, [])

  const updatePawTransform = useCallback(() => {
    if (!pawRef.current) return
    const { x, y } = posRef.current
    const clicking = isClickingRef.current
    const hover = isHoverRef.current
    const scale = clicking ? 0.85 : hover ? 1.25 : 1
    const rot = hover ? '-12deg' : '0deg'
    pawRef.current.style.transform = `translate3d(${x - 6}px, ${y - 4}px, 0) scale(${scale}) rotate(${rot})`
  }, [])

  useEffect(() => {
    if (!enabled) return

    const handleMouseMove = (e: MouseEvent) => {
      posRef.current = { x: e.clientX, y: e.clientY }

      // Check if target is interactive (only trigger React state update if changed)
      const target = e.target as HTMLElement | null
      const interactive = !!target?.closest('button, a, input, select, textarea, [role="button"], .clickable')
      if (interactive !== isHoverRef.current) {
        isHoverRef.current = interactive
        setIsHover(interactive)
      }

      // Schedule high-performance hardware-accelerated transform update via RAF
      if (!rafIdRef.current) {
        rafIdRef.current = requestAnimationFrame(() => {
          updatePawTransform()
          rafIdRef.current = null
        })
      }

      // Throttle sparkle emission (max 1 sparkle per 65ms, capped to 12 active)
      const now = Date.now()
      if (now - lastSparkleTime.current > 65) {
        lastSparkleTime.current = now
        const newSparkle: Sparkle = {
          id: nextId.current++,
          x: e.clientX + (Math.random() * 16 - 8),
          y: e.clientY + (Math.random() * 16 - 8),
          size: Math.floor(Math.random() * 8) + 8,
          color: SPARKLE_COLORS[Math.floor(Math.random() * SPARKLE_COLORS.length)],
        }
        setSparkles((prev) => [...prev.slice(-11), newSparkle])
      }
    }

    const handleMouseDown = () => {
      isClickingRef.current = true
      updatePawTransform()
    }

    const handleMouseUp = () => {
      isClickingRef.current = false
      updatePawTransform()
    }

    window.addEventListener('mousemove', handleMouseMove, { passive: true })
    window.addEventListener('mousedown', handleMouseDown, { passive: true })
    window.addEventListener('mouseup', handleMouseUp, { passive: true })

    return () => {
      window.removeEventListener('mousemove', handleMouseMove)
      window.removeEventListener('mousedown', handleMouseDown)
      window.removeEventListener('mouseup', handleMouseUp)
      if (rafIdRef.current) {
        cancelAnimationFrame(rafIdRef.current)
      }
    }
  }, [enabled, updatePawTransform])

  if (!enabled) return null

  return (
    <div className="pointer-events-none fixed inset-0 z-50 overflow-hidden select-none">
      {/* Sparkles with native CSS animation decay and zero-state timeouts */}
      {sparkles.map((s) => (
        <span
          key={s.id}
          onAnimationEnd={() => removeSparkle(s.id)}
          className="absolute transform -translate-x-1/2 -translate-y-1/2"
          style={{
            left: s.x,
            top: s.y,
            color: s.color,
            width: `${s.size}px`,
            height: `${s.size}px`,
            animation: 'sparkle-pop 0.45s ease-out forwards',
          }}
        >
          <svg viewBox="0 0 24 24" fill="currentColor" className="w-full h-full">
            <path d="M12 0L14.5 9.5L24 12L14.5 14.5L12 24L9.5 14.5L0 12L9.5 9.5L12 0Z" />
          </svg>
        </span>
      ))}

      {/* Cat Paw Pointer with direct hardware GPU transform */}
      <div
        ref={pawRef}
        className="absolute top-0 left-0 will-change-transform pointer-events-none"
        style={{
          transform: `translate3d(-100px, -100px, 0)`,
        }}
      >
        <svg
          width="26"
          height="26"
          viewBox="0 0 32 32"
          fill="none"
          xmlns="http://www.w3.org/2000/svg"
          className="drop-shadow-md"
        >
          {/* Main paw pad */}
          <ellipse
            cx="16"
            cy="20"
            rx="8"
            ry="6"
            fill={isHover ? '#2DD4BF' : '#FCFBF8'}
            stroke="#0F766E"
            strokeWidth="2"
          />
          {/* Inner pink cushion */}
          <ellipse
            cx="16"
            cy="20"
            rx="5"
            ry="3.5"
            fill={isHover ? '#0F766E' : '#FB7185'}
            opacity="0.9"
          />
          {/* Toe 1 (Left) */}
          <circle
            cx="8.5"
            cy="12"
            r="3"
            fill={isHover ? '#2DD4BF' : '#FCFBF8'}
            stroke="#0F766E"
            strokeWidth="1.8"
          />
          <circle cx="8.5" cy="12" r="1.6" fill={isHover ? '#0F766E' : '#FB7185'} />
          {/* Toe 2 (Middle Left) */}
          <circle
            cx="13"
            cy="9"
            r="3.2"
            fill={isHover ? '#2DD4BF' : '#FCFBF8'}
            stroke="#0F766E"
            strokeWidth="1.8"
          />
          <circle cx="13" cy="9" r="1.8" fill={isHover ? '#0F766E' : '#FB7185'} />
          {/* Toe 3 (Middle Right) */}
          <circle
            cx="19"
            cy="9"
            r="3.2"
            fill={isHover ? '#2DD4BF' : '#FCFBF8'}
            stroke="#0F766E"
            strokeWidth="1.8"
          />
          <circle cx="19" cy="9" r="1.8" fill={isHover ? '#0F766E' : '#FB7185'} />
          {/* Toe 4 (Right) */}
          <circle
            cx="23.5"
            cy="12"
            r="3"
            fill={isHover ? '#2DD4BF' : '#FCFBF8'}
            stroke="#0F766E"
            strokeWidth="1.8"
          />
          <circle cx="23.5" cy="12" r="1.6" fill={isHover ? '#0F766E' : '#FB7185'} />
        </svg>
      </div>
    </div>
  )
}
