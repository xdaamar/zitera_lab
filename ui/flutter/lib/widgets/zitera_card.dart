import 'package:flutter/material.dart';
import '../core/theme/zitera_colors.dart';

class ZiteraCard extends StatelessWidget {
  final Widget child;
  final EdgeInsetsGeometry padding;
  final VoidCallback? onTap;
  final Color? borderColor;

  const ZiteraCard({
    super.key,
    required this.child,
    this.padding = const EdgeInsets.all(20),
    this.onTap,
    this.borderColor,
  });

  @override
  Widget build(BuildContext context) {
    Widget content = Container(
      padding: padding,
      decoration: BoxDecoration(
        color: ZiteraColors.card,
        borderRadius: BorderRadius.circular(8),
        border: Border.all(
          color: borderColor ?? ZiteraColors.border,
          width: 1,
        ),
      ),
      child: child,
    );

    if (onTap != null) {
      return InkWell(
        onTap: onTap,
        borderRadius: BorderRadius.circular(8),
        hoverColor: ZiteraColors.cardHover,
        child: content,
      );
    }

    return content;
  }
}
