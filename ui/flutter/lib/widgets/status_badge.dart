import 'package:flutter/material.dart';
import '../core/theme/zitera_colors.dart';

class StatusBadge extends StatelessWidget {
  final String status;
  final double fontSize;

  const StatusBadge({
    super.key,
    required this.status,
    this.fontSize = 11,
  });

  @override
  Widget build(BuildContext context) {
    Color bg;
    Color fg;
    Color border;

    final norm = status.toUpperCase();

    if (norm == 'READY' || norm == 'RUNNING' || norm == 'OK') {
      bg = ZiteraColors.readyMuted;
      fg = ZiteraColors.ready;
      border = ZiteraColors.ready.withValues(alpha: 0.4);
    } else if (norm == 'WARNING' || norm == 'OUTDATED' || norm == 'ATTENTION REQUIRED') {
      bg = ZiteraColors.warningMuted;
      fg = ZiteraColors.warning;
      border = ZiteraColors.warning.withValues(alpha: 0.4);
    } else if (norm == 'BLOCKED' || norm == 'ERROR' || norm == 'FAILED') {
      bg = ZiteraColors.errorMuted;
      fg = ZiteraColors.error;
      border = ZiteraColors.error.withValues(alpha: 0.4);
    } else if (norm == 'STOPPED') {
      bg = ZiteraColors.cyanMuted;
      fg = ZiteraColors.cyan;
      border = ZiteraColors.cyan.withValues(alpha: 0.4);
    } else {
      bg = ZiteraColors.missingMuted;
      fg = ZiteraColors.missing;
      border = ZiteraColors.missing.withValues(alpha: 0.3);
    }

    return Container(
      padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 4),
      decoration: BoxDecoration(
        color: bg,
        borderRadius: BorderRadius.circular(4),
        border: Border.all(color: border, width: 1),
      ),
      child: Row(
        mainAxisSize: MainAxisSize.min,
        children: [
          Container(
            width: 6,
            height: 6,
            decoration: BoxDecoration(
              color: fg,
              shape: BoxShape.circle,
            ),
          ),
          const SizedBox(width: 6),
          Text(
            norm,
            style: TextStyle(
              color: fg,
              fontSize: fontSize,
              fontWeight: FontWeight.w700,
              letterSpacing: 0.8,
            ),
          ),
        ],
      ),
    );
  }
}
