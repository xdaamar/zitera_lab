import 'package:flutter/material.dart';

enum TilixSlideDirection { up, down, left, right }

/// High-tech hacker Tilix-style entrance animation.
/// Emulates the cybernetic pane deployment, matrix grid snap, and scanline flash
/// characteristic of the Tilix terminal emulator.
class HackerTilixEntrance extends StatefulWidget {
  final Widget child;
  final Duration delay;
  final Duration duration;
  final TilixSlideDirection direction;
  final double offsetDistance;
  final bool showScanBeam;
  final Color beamColor;

  const HackerTilixEntrance({
    super.key,
    required this.child,
    this.delay = Duration.zero,
    this.duration = const Duration(milliseconds: 420),
    this.direction = TilixSlideDirection.up,
    this.offsetDistance = 20.0,
    this.showScanBeam = true,
    this.beamColor = const Color(0xFF2DD4BF), // Neon Mint / Cyber Teal
  });

  @override
  State<HackerTilixEntrance> createState() => _HackerTilixEntranceState();
}

class _HackerTilixEntranceState extends State<HackerTilixEntrance>
    with SingleTickerProviderStateMixin {
  late final AnimationController _controller;
  late final Animation<double> _fadeAnim;
  late final Animation<double> _slideAnim;
  late final Animation<double> _scaleAnim;
  late final Animation<double> _beamAnim;
  bool _started = false;

  @override
  void initState() {
    super.initState();
    _controller = AnimationController(
      vsync: this,
      duration: widget.duration,
    );

    _fadeAnim = CurvedAnimation(
      parent: _controller,
      curve: const Interval(0.0, 0.75, curve: Curves.easeOut),
    );

    _slideAnim = CurvedAnimation(
      parent: _controller,
      curve: const Interval(0.0, 0.9, curve: Curves.easeOutCubic),
    );

    _scaleAnim = Tween<double>(begin: 0.96, end: 1.0).animate(
      CurvedAnimation(
        parent: _controller,
        curve: const Interval(0.1, 1.0, curve: Curves.easeOutBack),
      ),
    );

    // Neon beam sweep across pane top border
    _beamAnim = Tween<double>(begin: 0.0, end: 1.0).animate(
      CurvedAnimation(
        parent: _controller,
        curve: const Interval(0.0, 0.8, curve: Curves.easeInOut),
      ),
    );

    _startWithDelay();
  }

  void _startWithDelay() {
    if (widget.delay == Duration.zero) {
      if (mounted) {
        setState(() => _started = true);
        _controller.forward();
      }
    } else {
      Future.delayed(widget.delay, () {
        if (mounted) {
          setState(() => _started = true);
          _controller.forward();
        }
      });
    }
  }

  @override
  void dispose() {
    _controller.dispose();
    super.dispose();
  }

  Offset _getSlideOffset() {
    final dist = widget.offsetDistance * (1.0 - _slideAnim.value);
    switch (widget.direction) {
      case TilixSlideDirection.up:
        return Offset(0.0, dist);
      case TilixSlideDirection.down:
        return Offset(0.0, -dist);
      case TilixSlideDirection.left:
        return Offset(dist, 0.0);
      case TilixSlideDirection.right:
        return Offset(-dist, 0.0);
    }
  }

  @override
  Widget build(BuildContext context) {
    if (!_started) {
      // Invisible before delay to guarantee staggered matrix appearance without layout shifts
      return Opacity(
        opacity: 0.0,
        child: widget.child,
      );
    }

    return AnimatedBuilder(
      animation: _controller,
      builder: (context, child) {
        final offset = _getSlideOffset();
        final opacity = _fadeAnim.value.clamp(0.0, 1.0);
        final scale = _scaleAnim.value;
        final beamT = _beamAnim.value;

        return Opacity(
          opacity: opacity,
          child: Transform.translate(
            offset: offset,
            child: Transform.scale(
              scale: scale,
              alignment: Alignment.center,
              child: Stack(
                clipBehavior: Clip.none,
                children: [
                  widget.child,

                  // Hacker Tilix scanline beam sweep across pane
                  if (widget.showScanBeam && _controller.value < 0.95)
                    Positioned.fill(
                      child: IgnorePointer(
                        child: CustomPaint(
                          painter: _TilixScanBeamPainter(
                            progress: beamT,
                            beamColor: widget.beamColor,
                          ),
                        ),
                      ),
                    ),
                ],
              ),
            ),
          ),
        );
      },
    );
  }
}

class _TilixScanBeamPainter extends CustomPainter {
  final double progress;
  final Color beamColor;

  _TilixScanBeamPainter({
    required this.progress,
    required this.beamColor,
  });

  @override
  void paint(Canvas canvas, Size size) {
    if (progress <= 0.0 || progress >= 1.0) return;

    // Glowing scanline laser beam sweeping from left to right along top edge
    final beamWidth = size.width * 0.45;
    final beamLeft = (progress * (size.width + beamWidth)) - beamWidth;

    final rect = Rect.fromLTWH(beamLeft, 0, beamWidth, 2.5);
    final paint = Paint()
      ..shader = LinearGradient(
        colors: [
          Colors.transparent,
          beamColor.withValues(alpha: 0.8),
          Colors.white,
          beamColor.withValues(alpha: 0.8),
          Colors.transparent,
        ],
        stops: const [0.0, 0.35, 0.5, 0.65, 1.0],
      ).createShader(rect)
      ..style = PaintingStyle.fill;

    canvas.drawRRect(
      RRect.fromRectAndRadius(rect, const Radius.circular(2)),
      paint,
    );

    // Subtle laser beam glow
    final glowPaint = Paint()
      ..shader = LinearGradient(
        colors: [
          Colors.transparent,
          beamColor.withValues(alpha: 0.4),
          Colors.transparent,
        ],
      ).createShader(Rect.fromLTWH(beamLeft - 10, -2, beamWidth + 20, 7))
      ..maskFilter = const MaskFilter.blur(BlurStyle.normal, 3.0);

    canvas.drawRect(Rect.fromLTWH(beamLeft - 10, -2, beamWidth + 20, 7), glowPaint);
  }

  @override
  bool shouldRepaint(covariant _TilixScanBeamPainter oldDelegate) {
    return oldDelegate.progress != progress;
  }
}
