import 'package:flutter/material.dart';
import '../core/theme/zitera_colors.dart';

enum ButtonVariant { primary, secondary, danger, ghost }

class ZiteraButton extends StatelessWidget {
  final String label;
  final IconData? icon;
  final VoidCallback? onPressed;
  final ButtonVariant variant;
  final bool isLoading;

  const ZiteraButton({
    super.key,
    required this.label,
    this.icon,
    this.onPressed,
    this.variant = ButtonVariant.primary,
    this.isLoading = false,
  });

  @override
  Widget build(BuildContext context) {
    Color bg;
    Color fg;
    Color border;

    switch (variant) {
      case ButtonVariant.primary:
        bg = ZiteraColors.primary;
        fg = Colors.white;
        border = Colors.transparent;
        break;
      case ButtonVariant.secondary:
        bg = ZiteraColors.cardHover;
        fg = ZiteraColors.textPrimary;
        border = ZiteraColors.border;
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
    }

    return Material(
      color: Colors.transparent,
      child: InkWell(
        onTap: isLoading ? null : onPressed,
        borderRadius: BorderRadius.circular(6),
        child: Container(
          padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 10),
          decoration: BoxDecoration(
            color: bg,
            borderRadius: BorderRadius.circular(6),
            border: Border.all(color: border, width: 1),
          ),
          child: Row(
            mainAxisSize: MainAxisSize.min,
            mainAxisAlignment: MainAxisAlignment.center,
            children: [
              if (isLoading)
                const SizedBox(
                  width: 14,
                  height: 14,
                  child: CircularProgressIndicator(strokeWidth: 2, color: Colors.white),
                )
              else if (icon != null) ...[
                Icon(icon, size: 16, color: fg),
                const SizedBox(width: 8),
              ],
              Text(
                label,
                style: TextStyle(
                  color: fg,
                  fontSize: 13,
                  fontWeight: FontWeight.w600,
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
