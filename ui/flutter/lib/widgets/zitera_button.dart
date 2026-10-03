import 'package:flutter/material.dart';
import '../core/theme/zitera_colors.dart';
import 'cute_anime_loading.dart';
import 'cute_anime_cursor.dart';

enum ButtonVariant { primary, secondary, danger, ghost, gradient, mint }

class ZiteraButton extends StatefulWidget {
  final String label;
  final IconData? icon;
  final VoidCallback? onPressed;
  final ButtonVariant variant;
  final bool isLoading;
  final Gradient? gradient;
  final Color? backgroundColor;
  final Color? textColor;
  final Color? borderColor;
  final EdgeInsetsGeometry padding;
  final double fontSize;

  const ZiteraButton({
    super.key,
    required this.label,
    this.icon,
    this.onPressed,
    this.variant = ButtonVariant.primary,
    this.isLoading = false,
    this.gradient,
    this.backgroundColor,
    this.textColor,
    this.borderColor,
    this.padding = const EdgeInsets.symmetric(horizontal: 16, vertical: 10),
    this.fontSize = 12,
  });

  // Rainbow pastel gradient from blueprint
  static const Gradient rainbowGradient = LinearGradient(
    colors: [
      Color(0xFFFED7AA), // Peach
      Color(0xFFF472B6), // Pink
      Color(0xFF93C5FD), // Sky Blue
      Color(0xFFA7F3D0), // Mint
    ],
    begin: Alignment.centerLeft,
    end: Alignment.centerRight,
  );

  @override
  State<ZiteraButton> createState() => _ZiteraButtonState();
}

class _ZiteraButtonState extends State<ZiteraButton> {
  bool _isHovered = false;
  bool _isPressed = false;

  @override
  Widget build(BuildContext context) {
    Color bg;
    Color fg;
    Color border;
    Gradient? effGradient = widget.gradient;

    switch (widget.variant) {
      case ButtonVariant.primary:
        bg = ZiteraColors.primary;
        fg = Colors.white;
        border = _isHovered ? const Color(0xFF14B8A6) : Colors.transparent;
        break;
      case ButtonVariant.secondary:
        bg = _isHovered ? const Color(0xFFF8FAFC) : Colors.white;
        fg = _isHovered ? const Color(0xFF0F766E) : ZiteraColors.textPrimary;
        border = _isHovered ? const Color(0xFF2DD4BF) : ZiteraColors.borderDark;
        break;
      case ButtonVariant.danger:
        bg = _isHovered ? const Color(0xFFFEE2E2) : ZiteraColors.errorMuted;
        fg = ZiteraColors.error;
        border = _isHovered ? ZiteraColors.error : ZiteraColors.error.withValues(alpha: 0.4);
        break;
      case ButtonVariant.ghost:
        bg = _isHovered ? const Color(0xFFCCFBF1).withValues(alpha: 0.3) : Colors.transparent;
        fg = _isHovered ? const Color(0xFF0F766E) : ZiteraColors.cyan;
        border = _isHovered ? const Color(0xFF2DD4BF).withValues(alpha: 0.4) : Colors.transparent;
        break;
      case ButtonVariant.gradient:
        bg = Colors.transparent;
        fg = ZiteraColors.textPrimary;
        border = _isHovered ? const Color(0xFFF472B6) : ZiteraColors.borderDark;
        effGradient ??= ZiteraButton.rainbowGradient;
        break;
      case ButtonVariant.mint:
        bg = _isHovered ? const Color(0xFF6EE7B7) : const Color(0xFFA7F3D0);
        fg = const Color(0xFF065F46);
        border = _isHovered ? const Color(0xFF059669) : const Color(0xFF6EE7B7);
        break;
    }

    if (widget.backgroundColor != null) bg = widget.backgroundColor!;
    if (widget.textColor != null) fg = widget.textColor!;
    if (widget.borderColor != null) border = widget.borderColor!;

    final isClickable = widget.onPressed != null && !widget.isLoading;

    Color glowColor;
    switch (widget.variant) {
      case ButtonVariant.danger:
        glowColor = ZiteraColors.error.withValues(alpha: 0.35);
        break;
      case ButtonVariant.gradient:
        glowColor = const Color(0xFFF472B6).withValues(alpha: 0.4);
        break;
      case ButtonVariant.mint:
        glowColor = const Color(0xFF34D399).withValues(alpha: 0.4);
        break;
      default:
        glowColor = const Color(0xFF2DD4BF).withValues(alpha: 0.35);
    }

    return MouseRegion(
      cursor: isClickable ? CuteCursorController.cursor : CuteCursorController.defaultCursor,
      onEnter: (_) {
        if (isClickable) {
          setState(() => _isHovered = true);
          CuteCursorController.setHover(true);
        }
      },
      onExit: (_) {
        if (isClickable) {
          setState(() {
            _isHovered = false;
            _isPressed = false;
          });
          CuteCursorController.setHover(false);
        }
      },
      child: GestureDetector(
        onTapDown: (_) {
          if (isClickable) setState(() => _isPressed = true);
        },
        onTapUp: (_) {
          if (isClickable) setState(() => _isPressed = false);
        },
        onTapCancel: () {
          if (isClickable) setState(() => _isPressed = false);
        },
        onTap: isClickable ? widget.onPressed : null,
        child: AnimatedContainer(
          duration: const Duration(milliseconds: 140),
          curve: Curves.easeOutCubic,
          transform: Matrix4.identity()
            ..translate(0.0, _isPressed ? 1.2 : (_isHovered ? -2.2 : 0.0))
            ..scale(_isHovered ? 1.02 : 1.0),
          padding: widget.padding,
          decoration: BoxDecoration(
            color: effGradient == null ? bg : null,
            gradient: effGradient,
            borderRadius: BorderRadius.circular(8),
            border: Border.all(
              color: border,
              width: _isHovered ? 1.5 : 1.2,
            ),
            boxShadow: [
              if (_isHovered)
                BoxShadow(
                  color: glowColor,
                  blurRadius: 12,
                  spreadRadius: 1,
                  offset: const Offset(0, 3),
                )
              else
                BoxShadow(
                  color: Colors.black.withValues(alpha: 0.05),
                  blurRadius: 4,
                  offset: const Offset(0, 1),
                ),
            ],
          ),
          child: Row(
            mainAxisSize: MainAxisSize.min,
            mainAxisAlignment: MainAxisAlignment.center,
            children: [
              if (widget.isLoading)
                const Padding(
                  padding: EdgeInsets.only(right: 8.0),
                  child: CuteAnimeLoading(compact: true),
                )
              else if (widget.icon != null) ...[
                Icon(widget.icon, size: widget.fontSize + 4, color: fg),
                const SizedBox(width: 8),
              ],
              Text(
                widget.label,
                style: TextStyle(
                  color: fg,
                  fontSize: widget.fontSize,
                  fontWeight: FontWeight.w700,
                  fontFamily: 'SpaceGrotesk',
                  letterSpacing: 0.5,
                ),
              ),
              if (_isHovered && isClickable) ...[
                const SizedBox(width: 5),
                Icon(
                  Icons.auto_awesome,
                  size: widget.fontSize + 1,
                  color: fg.withValues(alpha: 0.8),
                ),
              ],
            ],
          ),
        ),
      ),
    );
  }
}
