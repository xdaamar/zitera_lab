import 'dart:math' as math;
import 'package:flutter/material.dart';

/// Global controller to inform the custom cursor when hovering over interactive elements
class CuteCursorController {
  static final ValueNotifier<bool> isHoveringClickable = ValueNotifier<bool>(false);
  static final ValueNotifier<bool> isEnabled = ValueNotifier<bool>(true);

  static void setHover(bool hover) {
    if (isHoveringClickable.value != hover) {
      isHoveringClickable.value = hover;
    }
  }

  /// When custom cursor is enabled, returns SystemMouseCursors.none so Windows
  /// completely removes/hides its default pointing hand / arrow cursor,
  /// letting our custom kawaii cat paw cursor render full and unobstructed!
  static MouseCursor get cursor =>
      isEnabled.value ? SystemMouseCursors.none : SystemMouseCursors.click;

  static MouseCursor get defaultCursor =>
      isEnabled.value ? SystemMouseCursors.none : MouseCursor.defer;
}

class CuteAnimeCursor extends StatefulWidget {
  final Widget child;

  const CuteAnimeCursor({
    super.key,
    required this.child,
  });

  @override
  State<CuteAnimeCursor> createState() => _CuteAnimeCursorState();
}

class _CuteAnimeCursorState extends State<CuteAnimeCursor>
    with SingleTickerProviderStateMixin {
  Offset _cursorPos = const Offset(-100, -100);
  bool _isInside = false;
  bool _isPressed = false;
  late final AnimationController _animController;
  final List<_TrailParticle> _particles = [];

  @override
  void initState() {
    super.initState();
    _animController = AnimationController(
      vsync: this,
      duration: const Duration(milliseconds: 1800),
    )..repeat();

    _animController.addListener(_updateParticles);
  }

  void _updateParticles() {
    if (_particles.isNotEmpty) {
      setState(() {
        _particles.removeWhere((p) {
          p.age += 0.05;
          p.opacity = (1.0 - p.age).clamp(0.0, 1.0);
          return p.age >= 1.0;
        });
      });
    }
  }

  void _addParticle(Offset pos, bool isHover) {
    if (_particles.length < 12) {
      _particles.add(
        _TrailParticle(
          x: pos.dx + (math.Random().nextDouble() - 0.5) * 12,
          y: pos.dy + (math.Random().nextDouble() - 0.5) * 12,
          color: isHover
              ? (math.Random().nextBool() ? const Color(0xFFF472B6) : const Color(0xFF2DD4BF))
              : const Color(0xFF38BDF8),
          size: isHover ? 4.5 : 3.0,
        ),
      );
    }
  }

  @override
  void dispose() {
    _animController.removeListener(_updateParticles);
    _animController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return ValueListenableBuilder<bool>(
      valueListenable: CuteCursorController.isEnabled,
      builder: (context, enabled, _) {
        if (!enabled) return widget.child;

        return ValueListenableBuilder<bool>(
          valueListenable: CuteCursorController.isHoveringClickable,
          builder: (context, isHover, _) {
            return MouseRegion(
              cursor: _isInside ? SystemMouseCursors.none : MouseCursor.defer,
              onEnter: (_) => setState(() => _isInside = true),
              onExit: (_) => setState(() => _isInside = false),
              onHover: (event) {
                setState(() {
                  _cursorPos = event.position;
                  _isInside = true;
                });
                _addParticle(event.position, isHover);
              },
              child: Listener(
                behavior: HitTestBehavior.translucent,
                onPointerDown: (_) => setState(() => _isPressed = true),
                onPointerUp: (_) => setState(() => _isPressed = false),
                onPointerCancel: (_) => setState(() => _isPressed = false),
                child: Stack(
                  children: [
                    widget.child,

                    // Custom anime cursor overlay
                    if (_isInside)
                      IgnorePointer(
                        child: AnimatedBuilder(
                          animation: _animController,
                          builder: (context, _) {
                            return CustomPaint(
                              size: Size.infinite,
                              painter: _CuteAnimeCursorPainter(
                                position: _cursorPos,
                                isHover: isHover,
                                isPressed: _isPressed,
                                animationValue: _animController.value,
                                particles: _particles,
                              ),
                            );
                          },
                        ),
                      ),
                  ],
                ),
              ),
            );
          },
        );
      },
    );
  }
}

class _TrailParticle {
  final double x;
  final double y;
  final Color color;
  final double size;
  double age = 0.0;
  double opacity = 1.0;

  _TrailParticle({
    required this.x,
    required this.y,
    required this.color,
    required this.size,
  });
}

class _CuteAnimeCursorPainter extends CustomPainter {
  final Offset position;
  final bool isHover;
  final bool isPressed;
  final double animationValue;
  final List<_TrailParticle> particles;

  _CuteAnimeCursorPainter({
    required this.position,
    required this.isHover,
    required this.isPressed,
    required this.animationValue,
    required this.particles,
  });

  @override
  void paint(Canvas canvas, Size size) {
    if (position.dx < -10 || position.dy < -10) return;

    // 1. Draw trailing particles
    for (final p in particles) {
      if (p.opacity <= 0) continue;
      final pPaint = Paint()
        ..color = p.color.withValues(alpha: p.opacity * 0.8)
        ..style = PaintingStyle.fill;
      _drawSparkle(canvas, Offset(p.x, p.y), p.size * (1.0 - p.age * 0.3), pPaint);
    }

    canvas.save();
    canvas.translate(position.dx, position.dy);

    // Scale animation
    final scale = isPressed
        ? 0.85
        : (isHover
            ? 1.15 + 0.05 * math.sin(animationValue * 4 * math.pi)
            : 1.0);
    canvas.scale(scale);

    if (isHover) {
      _paintHoverCatPaw(canvas);
    } else {
      _paintNormalAnimeArrow(canvas);
    }

    canvas.restore();
  }

  /// Normal state: Cute anime cyber-arrow with cat-ear silhouette & glowing tip star
  void _paintNormalAnimeArrow(Canvas canvas) {
    // Glowing shadow
    final glowPaint = Paint()
      ..color = const Color(0xFF2DD4BF).withValues(alpha: 0.45)
      ..maskFilter = const MaskFilter.blur(BlurStyle.normal, 5.0)
      ..style = PaintingStyle.fill;

    // White body
    final fillPaint = Paint()
      ..color = Colors.white
      ..style = PaintingStyle.fill;

    // Cyber border
    final borderPaint = Paint()
      ..color = const Color(0xFF0F766E)
      ..strokeWidth = 1.8
      ..style = PaintingStyle.stroke
      ..strokeJoin = StrokeJoin.round;

    final path = Path()
      ..moveTo(0, 0)
      ..lineTo(0, 22)
      ..lineTo(6, 17)
      ..lineTo(10, 24)
      ..lineTo(14, 22)
      ..lineTo(10, 15)
      ..lineTo(17, 15)
      ..close();

    // Draw glow
    canvas.drawPath(path, glowPaint);
    // Draw body
    canvas.drawPath(path, fillPaint);
    // Draw border
    canvas.drawPath(path, borderPaint);

    // Cat ear decoration on arrow
    final earPaint = Paint()
      ..color = const Color(0xFF2DD4BF)
      ..style = PaintingStyle.fill;

    final earPath = Path()
      ..moveTo(3, 4)
      ..lineTo(8, 7)
      ..lineTo(4, 9)
      ..close();
    canvas.drawPath(earPath, earPaint);

    // Tiny anime twinkle star at tip
    final starPaint = Paint()
      ..color = const Color(0xFF38BDF8)
      ..style = PaintingStyle.fill;
    final starPulse = 2.5 + 1.0 * math.sin(animationValue * 6 * math.pi);
    _drawSparkle(canvas, const Offset(-2, -2), starPulse, starPaint);
  }

  /// Hover state: Kawaii Cat Paw with pastel pink jelly pads & cyber halo!
  void _paintHoverCatPaw(Canvas canvas) {
    // Glowing halo
    final haloPaint = Paint()
      ..color = const Color(0xFFF472B6).withValues(alpha: 0.5)
      ..maskFilter = const MaskFilter.blur(BlurStyle.normal, 8.0)
      ..style = PaintingStyle.fill;
    canvas.drawCircle(const Offset(4, 4), 18, haloPaint);

    // Paw base (White/Cream body)
    final pawBodyPaint = Paint()
      ..color = const Color(0xFFFCFBF8)
      ..style = PaintingStyle.fill;

    final pawBorderPaint = Paint()
      ..color = const Color(0xFFF472B6)
      ..strokeWidth = 2.0
      ..style = PaintingStyle.stroke;

    // Draw main palm circle
    canvas.drawRRect(
      RRect.fromRectAndRadius(const Rect.fromLTWH(-6, -4, 20, 18), const Radius.circular(10)),
      pawBodyPaint,
    );
    canvas.drawRRect(
      RRect.fromRectAndRadius(const Rect.fromLTWH(-6, -4, 20, 18), const Radius.circular(10)),
      pawBorderPaint,
    );

    // Pink jelly bean main pad
    final mainPadPaint = Paint()
      ..color = const Color(0xFFFB7185)
      ..style = PaintingStyle.fill;
    canvas.drawOval(const Rect.fromLTWH(-1, 2, 10, 8), mainPadPaint);

    // 3 small toe pads
    final toePadPaint = Paint()
      ..color = const Color(0xFFF472B6)
      ..style = PaintingStyle.fill;

    canvas.drawCircle(const Offset(-2, -1), 2.5, toePadPaint);
    canvas.drawCircle(const Offset(4, -3), 2.8, toePadPaint);
    canvas.drawCircle(const Offset(10, -1), 2.5, toePadPaint);

    // Twinkling stars bursting around paw
    final sparklePaint = Paint()
      ..color = const Color(0xFFFBBF24)
      ..style = PaintingStyle.fill;
    _drawSparkle(canvas, const Offset(14, -8), 3.5, sparklePaint);
    _drawSparkle(canvas, const Offset(-10, 12), 2.5, sparklePaint);
  }

  void _drawSparkle(Canvas canvas, Offset center, double size, Paint paint) {
    final path = Path()
      ..moveTo(center.dx, center.dy - size)
      ..quadraticBezierTo(center.dx, center.dy, center.dx + size, center.dy)
      ..quadraticBezierTo(center.dx, center.dy, center.dx, center.dy + size)
      ..quadraticBezierTo(center.dx, center.dy, center.dx - size, center.dy)
      ..quadraticBezierTo(center.dx, center.dy, center.dx, center.dy - size)
      ..close();
    canvas.drawPath(path, paint);
  }

  @override
  bool shouldRepaint(covariant _CuteAnimeCursorPainter oldDelegate) {
    return true;
  }
}
