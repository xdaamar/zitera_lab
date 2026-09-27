import 'package:flutter/material.dart';

class StatusBadge extends StatelessWidget {

  final String status;
  final double fontSize;
  final Color? backgroundColor;
  final Color? textColor;
  final Color? borderColor;

  const StatusBadge({
    super.key,
    required this.status,
    this.fontSize = 10,
    this.backgroundColor,
    this.textColor,
    this.borderColor,
  });

  @override
  Widget build(BuildContext context) {
    Color bg;
    Color fg;
    Color border;

    final norm = status.toUpperCase();

    if (norm == 'READY' || norm == 'RUNNING' || norm == 'OK') {
      bg = const Color(0xFFDCFCE7);
      fg = const Color(0xFF166534);
      border = const Color(0xFF86EFAC);
    } else if (norm == 'WARNING' || norm == 'OUTDATED' || norm == 'ATTENTION REQUIRED') {
      bg = const Color(0xFFFEF3C7);
      fg = const Color(0xFFB45309);
      border = const Color(0xFFFCD34D);
    } else if (norm == 'BLOCKED' || norm == 'ERROR' || norm == 'FAILED') {
      bg = const Color(0xFFFEE2E2);
      fg = const Color(0xFFB91C1C);
      border = const Color(0xFFFCA5A5);
    } else if (norm == 'STOPPED') {
      bg = const Color(0xFFE2E8F0);
      fg = const Color(0xFF475569);
      border = const Color(0xFFCBD5E1);
    } else if (norm.startsWith('PORT')) {
      bg = const Color(0xFFF1F5F9);
      fg = const Color(0xFF334155);
      border = const Color(0xFFCBD5E1);
    } else {
      bg = const Color(0xFFF1F5F9);
      fg = const Color(0xFF64748B);
      border = const Color(0xFFE2E8F0);
    }

    if (backgroundColor != null) bg = backgroundColor!;
    if (textColor != null) fg = textColor!;
    if (borderColor != null) border = borderColor!;

    return Container(
      padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 3),
      decoration: BoxDecoration(
        color: bg,
        borderRadius: BorderRadius.circular(4),
        border: Border.all(color: border, width: 1),
      ),
      child: Row(
        mainAxisSize: MainAxisSize.min,
        crossAxisAlignment: CrossAxisAlignment.center,
        children: [
          // Square pixel dot
          Container(
            width: 5,
            height: 5,
            decoration: BoxDecoration(
              color: fg,
              borderRadius: BorderRadius.circular(1),
            ),
          ),
          const SizedBox(width: 5),
          Text(
            norm,
            style: TextStyle(
              color: fg,
              fontSize: fontSize,
              fontWeight: FontWeight.w600,
              fontFamily: 'JetBrainsMono',
              letterSpacing: 0.3,
            ),
          ),
        ],
      ),
    );
  }
}

