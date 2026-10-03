import 'package:flutter/material.dart';
import '../core/theme/zitera_colors.dart';
import 'cute_anime_cursor.dart';

class ZiteraCard extends StatefulWidget {
  final Widget child;
  final EdgeInsetsGeometry padding;
  final VoidCallback? onTap;
  final Color? borderColor;
  final Color? backgroundColor;
  final double borderRadius;
  final List<BoxShadow>? boxShadow;
  final bool enableHover;

  const ZiteraCard({
    super.key,
    required this.child,
    this.padding = const EdgeInsets.all(20),
    this.onTap,
    this.borderColor,
    this.backgroundColor,
    this.borderRadius = 10,
    this.boxShadow,
    this.enableHover = true,
  });

  @override
  State<ZiteraCard> createState() => _ZiteraCardState();
}

class _ZiteraCardState extends State<ZiteraCard> {
  bool _isHovered = false;

  @override
  Widget build(BuildContext context) {
    final hasHover = widget.enableHover;
    final isClickable = widget.onTap != null;

    final defaultBorder = widget.borderColor ?? ZiteraColors.border;
    final activeBorder = widget.borderColor != null
        ? widget.borderColor!
        : const Color(0xFF2DD4BF).withValues(alpha: 0.8);

    final defaultShadow = widget.boxShadow ?? [
      BoxShadow(
        color: Colors.black.withValues(alpha: 0.04),
        blurRadius: 6,
        offset: const Offset(0, 2),
      ),
    ];

    final hoverShadow = [
      BoxShadow(
        color: const Color(0xFF2DD4BF).withValues(alpha: 0.16),
        blurRadius: 18,
        spreadRadius: 1,
        offset: const Offset(0, 6),
      ),
      BoxShadow(
        color: Colors.black.withValues(alpha: 0.04),
        blurRadius: 8,
        offset: const Offset(0, 2),
      ),
    ];

    final cardContent = AnimatedContainer(
      duration: const Duration(milliseconds: 180),
      curve: Curves.easeOutCubic,
      transform: Matrix4.identity()
        ..translate(0.0, (hasHover && _isHovered) ? -4.0 : 0.0),
      padding: widget.padding,
      decoration: BoxDecoration(
        color: widget.backgroundColor ??
            ((hasHover && _isHovered && isClickable)
                ? const Color(0xFFFCFDFD)
                : ZiteraColors.card),
        borderRadius: BorderRadius.circular(widget.borderRadius),
        border: Border.all(
          color: (hasHover && _isHovered) ? activeBorder : defaultBorder,
          width: (hasHover && _isHovered) ? 1.5 : 1.2,
        ),
        boxShadow: (hasHover && _isHovered) ? hoverShadow : defaultShadow,
      ),
      child: widget.child,
    );

    return MouseRegion(
      cursor: isClickable ? CuteCursorController.cursor : CuteCursorController.defaultCursor,
      onEnter: (_) {
        if (hasHover) setState(() => _isHovered = true);
        if (isClickable) CuteCursorController.setHover(true);
      },
      onExit: (_) {
        if (hasHover) setState(() => _isHovered = false);
        if (isClickable) CuteCursorController.setHover(false);
      },
      child: isClickable
          ? GestureDetector(
              onTap: widget.onTap,
              child: cardContent,
            )
          : cardContent,
    );
  }
}
