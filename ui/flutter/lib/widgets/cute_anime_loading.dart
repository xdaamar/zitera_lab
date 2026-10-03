import 'dart:math' as math;
import 'package:flutter/material.dart';
import '../core/theme/zitera_colors.dart';

class CuteAnimeLoading extends StatefulWidget {
  final String? message;
  final String? subMessage;
  final double height;
  final bool compact;

  const CuteAnimeLoading({
    super.key,
    this.message = 'INITIALIZING CYBER MATRIX...',
    this.subMessage = '( •̀ ω •́ )✧ NEKO CORE ONLINE',
    this.height = 140,
    this.compact = false,
  });

  @override
  State<CuteAnimeLoading> createState() => _CuteAnimeLoadingState();
}

class _CuteAnimeLoadingState extends State<CuteAnimeLoading>
    with SingleTickerProviderStateMixin {
  late final AnimationController _controller;

  @override
  void initState() {
    super.initState();
    _controller = AnimationController(
      vsync: this,
      duration: const Duration(milliseconds: 2200),
    )..repeat();
  }

  @override
  void dispose() {
    _controller.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    if (widget.compact) {
      return SizedBox(
        width: 32,
        height: 32,
        child: AnimatedBuilder(
          animation: _controller,
          builder: (context, child) {
            final t = _controller.value;
            return Stack(
              alignment: Alignment.center,
              children: [
                Transform.rotate(
                  angle: t * 2 * math.pi,
                  child: Container(
                    width: 26,
                    height: 26,
                    decoration: BoxDecoration(
                      shape: BoxShape.circle,
                      border: Border.all(
                        color: const Color(0xFF2DD4BF).withValues(alpha: 0.6),
                        width: 2,
                      ),
                    ),
                  ),
                ),
                Transform.scale(
                  scale: 0.85 + 0.15 * math.sin(t * 4 * math.pi),
                  child: ClipOval(
                    child: Image.asset(
                      'assets/images/calico_cat.png',
                      width: 18,
                      height: 18,
                      fit: BoxFit.cover,
                      errorBuilder: (_, __, ___) => const Icon(
                        Icons.pets,
                        size: 14,
                        color: Color(0xFF2DD4BF),
                      ),
                    ),
                  ),
                ),
              ],
            );
          },
        ),
      );
    }

    return RepaintBoundary(
      child: Container(
        height: widget.height,
        constraints: const BoxConstraints(maxWidth: 420),
        padding: const EdgeInsets.symmetric(horizontal: 20, vertical: 16),
        decoration: BoxDecoration(
          color: const Color(0xFFFCFBF8).withValues(alpha: 0.94),
          borderRadius: BorderRadius.circular(16),
          border: Border.all(color: const Color(0xFF2DD4BF).withValues(alpha: 0.4), width: 1.5),
          boxShadow: [
            BoxShadow(
              color: const Color(0xFF2DD4BF).withValues(alpha: 0.12),
              blurRadius: 20,
              offset: const Offset(0, 4),
            ),
          ],
        ),
        child: AnimatedBuilder(
          animation: _controller,
          builder: (context, child) {
            final t = _controller.value;
            return Column(
              mainAxisAlignment: MainAxisAlignment.center,
              mainAxisSize: MainAxisSize.min,
              children: [
                // 1. Curved sinusoidal wavy track with animated mascot head
                SizedBox(
                  height: 60,
                  width: double.infinity,
                  child: LayoutBuilder(
                    builder: (context, constraints) {
                      final width = constraints.maxWidth;
                      final xProgress = (t * width) % width;
                      // Sinusoidal wave y position
                      const amplitude = 12.0;
                      const frequency = 3.5;
                      final waveY = 30.0 + amplitude * math.sin((xProgress / width) * 2 * math.pi * frequency + t * 2 * math.pi);
                      final tilt = math.cos((xProgress / width) * 2 * math.pi * frequency + t * 2 * math.pi) * 0.3;

                      return Stack(
                        clipBehavior: Clip.none,
                        children: [
                          // Wave Canvas painter
                          Positioned.fill(
                            child: CustomPaint(
                              painter: _CurvedWavePainter(
                                progress: t,
                                amplitude: amplitude,
                                frequency: frequency,
                              ),
                            ),
                          ),

                          // Floating Sparkle Particles
                          for (int i = 0; i < 4; i++) ...[
                            _buildSparkle(width, i, t),
                          ],

                          // Moving anime mascot head
                          Positioned(
                            left: (xProgress - 18).clamp(0.0, width - 36),
                            top: (waveY - 18).clamp(0.0, 42.0),
                            child: Transform.rotate(
                              angle: tilt,
                              child: Container(
                                width: 36,
                                height: 36,
                                decoration: BoxDecoration(
                                  shape: BoxShape.circle,
                                  color: Colors.white,
                                  border: Border.all(
                                    color: _getGlowColor(t),
                                    width: 2.2,
                                  ),
                                  boxShadow: [
                                    BoxShadow(
                                      color: _getGlowColor(t).withValues(alpha: 0.5),
                                      blurRadius: 10,
                                      spreadRadius: 2,
                                    ),
                                  ],
                                ),
                                child: ClipOval(
                                  child: Image.asset(
                                    'assets/images/zeta_avatar.png',
                                    fit: BoxFit.cover,
                                    errorBuilder: (_, __, ___) => Image.asset(
                                      'assets/images/calico_cat.png',
                                      fit: BoxFit.cover,
                                    ),
                                  ),
                                ),
                              ),
                            ),
                          ),
                        ],
                      );
                    },
                  ),
                ),

                const SizedBox(height: 10),

                // 2. Animated text
                if (widget.message != null)
                  Text(
                    widget.message!,
                    style: const TextStyle(
                      fontFamily: 'SpaceGrotesk',
                      fontSize: 12,
                      fontWeight: FontWeight.w700,
                      color: Color(0xFF1E1A14),
                      letterSpacing: 1.0,
                    ),
                    textAlign: TextAlign.center,
                  ),

                if (widget.subMessage != null) ...[
                  const SizedBox(height: 3),
                  Text(
                    widget.subMessage!,
                    style: const TextStyle(
                      fontFamily: 'JetBrainsMono',
                      fontSize: 10,
                      fontWeight: FontWeight.w600,
                      color: Color(0xFF0F766E),
                      letterSpacing: 0.5,
                    ),
                    textAlign: TextAlign.center,
                  ),
                ],
              ],
            );
          },
        ),
      ),
    );
  }

  Widget _buildSparkle(double width, int index, double t) {
    final seed = (index * 0.25 + t) % 1.0;
    final sx = seed * width;
    final sy = 15.0 + 25.0 * math.sin(seed * math.pi * 3);
    final size = 3.0 + 2.5 * math.sin(t * 4 * math.pi + index);
    final colors = [
      const Color(0xFFF472B6), // Pink
      const Color(0xFF2DD4BF), // Mint Cyan
      const Color(0xFFFBBF24), // Gold
      const Color(0xFFA78BFA), // Lavender
    ];

    return Positioned(
      left: sx,
      top: sy,
      child: Transform.rotate(
        angle: t * 4 * math.pi,
        child: Icon(
          Icons.auto_awesome,
          size: size.clamp(4.0, 10.0),
          color: colors[index % colors.length],
        ),
      ),
    );
  }

  Color _getGlowColor(double t) {
    if (t < 0.33) return const Color(0xFF2DD4BF); // Teal
    if (t < 0.66) return const Color(0xFFF472B6); // Pink
    return const Color(0xFF60A5FA); // Blue
  }
}

class _CurvedWavePainter extends CustomPainter {
  final double progress;
  final double amplitude;
  final double frequency;

  _CurvedWavePainter({
    required this.progress,
    required this.amplitude,
    required this.frequency,
  });

  @override
  void paint(Canvas canvas, Size size) {
    final midY = size.height / 2;

    // 1. Draw subtle background wavy dashed track
    final bgPaint = Paint()
      ..color = const Color(0xFFE2E8F0).withValues(alpha: 0.7)
      ..style = PaintingStyle.stroke
      ..strokeWidth = 2.0;

    final bgPath = Path();
    for (double x = 0; x <= size.width; x += 3) {
      final y = midY + amplitude * math.sin((x / size.width) * 2 * math.pi * frequency);
      if (x == 0) {
        bgPath.moveTo(x, y);
      } else {
        bgPath.lineTo(x, y);
      }
    }
    canvas.drawPath(bgPath, bgPaint);

    // 2. Draw glowing animated rainbow gradient wave
    final wavePaint = Paint()
      ..shader = const LinearGradient(
        colors: [
          Color(0xFF2DD4BF), // Mint
          Color(0xFF60A5FA), // Sky Blue
          Color(0xFFF472B6), // Pink
          Color(0xFFFBBF24), // Peach/Gold
        ],
      ).createShader(Rect.fromLTWH(0, 0, size.width, size.height))
      ..style = PaintingStyle.stroke
      ..strokeCap = StrokeCap.round
      ..strokeWidth = 3.5;

    final activePath = Path();
    final activeWidth = size.width;

    for (double x = 0; x <= activeWidth; x += 2) {
      final y = midY + amplitude * math.sin((x / size.width) * 2 * math.pi * frequency + progress * 2 * math.pi);
      if (x == 0) {
        activePath.moveTo(x, y);
      } else {
        activePath.lineTo(x, y);
      }
    }

    // Shadow glow
    final glowPaint = Paint()
      ..color = const Color(0xFF2DD4BF).withValues(alpha: 0.3)
      ..style = PaintingStyle.stroke
      ..strokeCap = StrokeCap.round
      ..strokeWidth = 7.0
      ..maskFilter = const MaskFilter.blur(BlurStyle.normal, 4.0);

    canvas.drawPath(activePath, glowPaint);
    canvas.drawPath(activePath, wavePaint);
  }

  @override
  bool shouldRepaint(covariant _CurvedWavePainter oldDelegate) {
    return oldDelegate.progress != progress;
  }
}
