import 'package:flutter/material.dart';
import '../core/theme/zitera_colors.dart';

enum ButtonVariant { primary, secondary, danger, ghost, gradient, mint }

class ZiteraButton extends StatelessWidget {
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
  Widget build(BuildContext context) {
    Color bg;
    Color fg;
    Color border;
    Gradient? effGradient = gradient;

    switch (variant) {
      case ButtonVariant.primary:
        bg = ZiteraColors.primary;
        fg = Colors.white;
        border = Colors.transparent;
        break;
      case ButtonVariant.secondary:
        bg = Colors.white;
        fg = ZiteraColors.textPrimary;
        border = ZiteraColors.borderDark;
        break;
      case ButtonVariant.danger:
        bg = ZiteraColors.errorMuted;
        fg = ZiteraColors.error;
        border = ZiteraColors.error.withValues(alpha: 0.4);
        break;
      case ButtonVariant.ghost:
        bg = Colors.transparent;
        fg = ZiteraColors.cyan;
        border = Colors.transparent;
        break;
      case ButtonVariant.gradient:
        bg = Colors.transparent;
        fg = ZiteraColors.textPrimary;
        border = ZiteraColors.borderDark;
        effGradient ??= rainbowGradient;
        break;
      case ButtonVariant.mint:
        bg = const Color(0xFFA7F3D0);
        fg = const Color(0xFF065F46);
        border = const Color(0xFF6EE7B7);
        break;
    }

    if (backgroundColor != null) bg = backgroundColor!;
    if (textColor != null) fg = textColor!;
    if (borderColor != null) border = borderColor!;

    return Material(
      color: Colors.transparent,
      child: InkWell(
        onTap: isLoading ? null : onPressed,
        borderRadius: BorderRadius.circular(6),
        child: Container(
          padding: padding,
          decoration: BoxDecoration(
            color: effGradient == null ? bg : null,
            gradient: effGradient,
            borderRadius: BorderRadius.circular(6),
            border: Border.all(color: border, width: 1.2),
            boxShadow: [
              BoxShadow(
                color: Colors.black.withValues(alpha: 0.04),
                blurRadius: 4,
                offset: const Offset(0, 1),
              ),
            ],
          ),
          child: Row(
            mainAxisSize: MainAxisSize.min,
            mainAxisAlignment: MainAxisAlignment.center,
            children: [
              if (isLoading)
                SizedBox(
                  width: fontSize + 2,
                  height: fontSize + 2,
                  child: const CircularProgressIndicator(strokeWidth: 2, color: ZiteraColors.textPrimary),
                )
              else if (icon != null) ...[
                Icon(icon, size: fontSize + 4, color: fg),
                const SizedBox(width: 8),
              ],
              Text(
                label,
                style: TextStyle(
                  color: fg,
                  fontSize: fontSize,
                  fontWeight: FontWeight.w700,
                  fontFamily: 'SpaceGrotesk',
                  letterSpacing: 0.5,
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }
}

