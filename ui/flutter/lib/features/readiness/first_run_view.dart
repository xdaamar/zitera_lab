import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import '../../core/readiness/component_readiness.dart';
import '../../core/theme/zitera_colors.dart';
import '../../widgets/hacker_tilix_entrance.dart';
import '../../widgets/zitera_button.dart';
import '../../widgets/zitera_card.dart';

class FirstRunView extends StatefulWidget {
  final VoidCallback? onProceed;
  final bool standalone;
  final bool autoStart;

  const FirstRunView({
    super.key,
    this.onProceed,
    this.standalone = false,
    this.autoStart = true,
  });

  @override
  State<FirstRunView> createState() => _FirstRunViewState();
}

class _FirstRunViewState extends State<FirstRunView> {
  final ComponentReadinessChecker _checker = ComponentReadinessChecker();
  final ScrollController _logScrollController = ScrollController();

  List<ComponentCheckItem> _items = [
    ComponentCheckItem(
      id: 'bundled_engine',
      name: 'Bundled Rust Engine',
      description: 'Verifies zitera-engine.exe is present in the release package',
    ),
    ComponentCheckItem(
      id: 'engine_version',
      name: 'Engine Version Compatibility',
      description: 'Verifies compiled engine contract matches ZITERA 2.0 release',
    ),
    ComponentCheckItem(
      id: 'engine_health',
      name: 'Engine Diagnostic Health',
      description: 'Validates daemonless native runtime and OS prerequisites',
    ),
    ComponentCheckItem(
      id: 'flutter_runtime',
      name: 'Flutter Windows Runtime Files',
      description: 'Verifies flutter_windows.dll, ICU tables, and asset bundles',
    ),
    ComponentCheckItem(
      id: 'catalog_integrity',
      name: 'Signed Catalog Integrity',
      description: 'Validates cryptographic catalog.json and Ed25519 signature',
    ),
    ComponentCheckItem(
      id: 'lab_packages',
      name: 'Lab Packages Availability',
      description: 'Audits availability of canonical A01–A10 .zlab offline packages',
    ),
    ComponentCheckItem(
      id: 'windows_runtime',
      name: 'Required Windows Runtime',
      description: 'Verifies Visual C++ x64 universal runtime DLL availability',
    ),
  ];
  List<String> _logs = [];
  OverallReadiness _overall = OverallReadiness.checking;
  bool _isRunning = false;
  bool _isRepairing = false;
  bool _showLogs = false;
  bool _autoScroll = true;

  @override
  void initState() {
    super.initState();
    if (widget.autoStart) {
      _startAudit();
    }
  }

  @override
  void dispose() {
    _checker.cancel();
    _logScrollController.dispose();
    super.dispose();
  }

  Future<void> _startAudit() async {
    if (_isRunning) return;
    setState(() {
      _isRunning = true;
      _overall = OverallReadiness.checking;
    });

    final res = await _checker.runAudit(
      onComponentUpdate: (item) {
        if (mounted) {
          setState(() {
            final idx = _items.indexWhere((i) => i.id == item.id);
            if (idx >= 0) {
              _items[idx] = item;
            } else {
              _items.add(item);
            }
          });
        }
      },
      onLog: (line) {
        if (mounted) {
          setState(() {
            _logs = _checker.logs;
          });
          if (_autoScroll && _logScrollController.hasClients) {
            _logScrollController.animateTo(
              _logScrollController.position.maxScrollExtent,
              duration: const Duration(milliseconds: 150),
              curve: Curves.easeOut,
            );
          }
        }
      },
    );

    if (mounted) {
      setState(() {
        _items = (res['items'] as List<ComponentCheckItem>?) ?? _items;
        _overall = (res['overall'] as OverallReadiness?) ?? OverallReadiness.checkFailed;
        _logs = _checker.logs;
        _isRunning = false;
      });
    }
  }

  Future<void> _handleRepair() async {
    if (_isRepairing) return;
    setState(() => _isRepairing = true);

    await _checker.attemptRepair(
      onProgress: (msg) {
        if (mounted) {
          setState(() {
            _logs = _checker.logs;
          });
        }
      },
    );

    if (mounted) {
      setState(() => _isRepairing = false);
      await _startAudit();
    }
  }

  void _copyLog() {
    final sanitizedLog = _logs.join('\n');
    Clipboard.setData(ClipboardData(text: sanitizedLog));
    ScaffoldMessenger.of(context).showSnackBar(
      const SnackBar(
        content: Text('Sanitized execution log copied to clipboard!'),
        backgroundColor: ZiteraColors.ready,
        duration: Duration(seconds: 2),
      ),
    );
  }

  Color _getStatusColor(CheckStatus status) {
    switch (status) {
      case CheckStatus.ready:
        return ZiteraColors.ready;
      case CheckStatus.checking:
        return ZiteraColors.primary;
      case CheckStatus.optionalUpdate:
        return const Color(0xFF0284C7);
      case CheckStatus.repairRequired:
      case CheckStatus.incompatible:
        return const Color(0xFFD97706);
      case CheckStatus.missing:
      case CheckStatus.corrupted:
      case CheckStatus.failed:
      case CheckStatus.timedOut:
        return ZiteraColors.error;
      case CheckStatus.pending:
        return ZiteraColors.textMuted;
    }
  }

  IconData _getStatusIcon(CheckStatus status) {
    switch (status) {
      case CheckStatus.ready:
        return Icons.check_circle_outline;
      case CheckStatus.checking:
        return Icons.sync;
      case CheckStatus.optionalUpdate:
        return Icons.info_outline;
      case CheckStatus.repairRequired:
      case CheckStatus.incompatible:
        return Icons.build_circle_outlined;
      case CheckStatus.missing:
        return Icons.search_off;
      case CheckStatus.corrupted:
        return Icons.broken_image_outlined;
      case CheckStatus.failed:
      case CheckStatus.timedOut:
        return Icons.highlight_off;
      case CheckStatus.pending:
        return Icons.radio_button_unchecked;
    }
  }

  String _formatStatusLabel(CheckStatus status) {
    switch (status) {
      case CheckStatus.ready:
        return 'READY';
      case CheckStatus.checking:
        return 'CHECKING';
      case CheckStatus.missing:
        return 'MISSING';
      case CheckStatus.corrupted:
        return 'CORRUPTED';
      case CheckStatus.incompatible:
        return 'INCOMPATIBLE';
      case CheckStatus.repairRequired:
        return 'REPAIR REQUIRED';
      case CheckStatus.failed:
        return 'FAILED';
      case CheckStatus.optionalUpdate:
        return 'OPTIONAL UPDATE';
      case CheckStatus.timedOut:
        return 'TIMED OUT';
      case CheckStatus.pending:
        return 'PENDING';
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: ZiteraColors.background,
      body: SafeArea(
        child: SingleChildScrollView(
          padding: const EdgeInsets.symmetric(horizontal: 32.0, vertical: 24.0),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              // Header
              HackerTilixEntrance(
                delay: const Duration(milliseconds: 30),
                direction: TilixSlideDirection.down,
                child: Row(
                  mainAxisAlignment: MainAxisAlignment.spaceBetween,
                  children: [
                    Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        Row(
                          children: [
                            Container(
                              padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 3),
                              decoration: BoxDecoration(
                                color: ZiteraColors.primary.withValues(alpha: 0.15),
                                borderRadius: BorderRadius.circular(4),
                                border: Border.all(color: ZiteraColors.primary, width: 1),
                              ),
                              child: const Text(
                                'ZITERA 2.0 // FIRST-RUN READINESS',
                                style: TextStyle(
                                  fontFamily: 'JetBrainsMono',
                                  fontSize: 11,
                                  fontWeight: FontWeight.bold,
                                  color: ZiteraColors.primary,
                                  letterSpacing: 0.8,
                                ),
                              ),
                            ),
                          ],
                        ),
                        const SizedBox(height: 8),
                        const Text(
                          'System & Component Verification',
                          style: TextStyle(
                            fontFamily: 'SpaceGrotesk',
                            fontSize: 24,
                            fontWeight: FontWeight.w800,
                            color: ZiteraColors.textPrimary,
                          ),
                        ),
                      ],
                    ),
                    if (widget.onProceed != null)
                      ElevatedButton.icon(
                        style: ElevatedButton.styleFrom(
                          backgroundColor: _overall == OverallReadiness.ready
                              ? ZiteraColors.ready
                              : const Color(0xFF334155),
                          foregroundColor: Colors.white,
                          padding: const EdgeInsets.symmetric(horizontal: 20, vertical: 14),
                          shape: RoundedRectangleBorder(
                            borderRadius: BorderRadius.circular(8),
                          ),
                        ),
                        icon: const Icon(Icons.arrow_forward, size: 18),
                        label: const Text(
                          'PROCEED TO DASHBOARD',
                          style: TextStyle(
                            fontFamily: 'SpaceGrotesk',
                            fontWeight: FontWeight.bold,
                            letterSpacing: 0.8,
                          ),
                        ),
                        onPressed: _overall == OverallReadiness.ready ? widget.onProceed : null,
                      ),
                  ],
                ),
              ),

              const SizedBox(height: 20),

              // Overall Status Banner
              HackerTilixEntrance(
                delay: const Duration(milliseconds: 60),
                direction: TilixSlideDirection.up,
                child: Container(
                  padding: const EdgeInsets.all(16.0),
                  decoration: BoxDecoration(
                    color: _overall == OverallReadiness.ready
                        ? ZiteraColors.readyMuted
                        : _overall == OverallReadiness.needsRepair
                            ? const Color(0xFFFEF3C7)
                            : _overall == OverallReadiness.checking
                                ? const Color(0xFFF0FDFA)
                                : const Color(0xFFFEE2E2),
                    borderRadius: BorderRadius.circular(8),
                    border: Border.all(
                      color: _overall == OverallReadiness.ready
                          ? ZiteraColors.ready
                          : _overall == OverallReadiness.needsRepair
                              ? const Color(0xFFD97706)
                              : _overall == OverallReadiness.checking
                                  ? ZiteraColors.primary
                                  : ZiteraColors.error,
                      width: 1.5,
                    ),
                  ),
                  child: Row(
                    children: [
                      Icon(
                        _overall == OverallReadiness.ready
                            ? Icons.verified
                            : _overall == OverallReadiness.needsRepair
                                ? Icons.warning_amber_rounded
                                : _overall == OverallReadiness.checking
                                    ? Icons.hourglass_top
                                    : Icons.error_outline,
                        color: _overall == OverallReadiness.ready
                            ? ZiteraColors.ready
                            : _overall == OverallReadiness.needsRepair
                                ? const Color(0xFFD97706)
                                : _overall == OverallReadiness.checking
                                    ? ZiteraColors.primary
                                    : ZiteraColors.error,
                        size: 28,
                      ),
                      const SizedBox(width: 14),
                      Expanded(
                        child: Column(
                          crossAxisAlignment: CrossAxisAlignment.start,
                          children: [
                            Text(
                              _overall == OverallReadiness.ready
                                  ? 'ALL CRITICAL COMPONENTS READY'
                                  : _overall == OverallReadiness.needsRepair
                                      ? 'ACTION REQUIRED: REPAIR OR CONFIGURATION NEEDED'
                                      : _overall == OverallReadiness.checking
                                          ? 'PERFORMING HARDWARE & COMPONENT AUDIT...'
                                          : 'COMPONENT VERIFICATION FAILED',
                              style: TextStyle(
                                fontFamily: 'SpaceGrotesk',
                                fontWeight: FontWeight.w800,
                                fontSize: 14,
                                color: _overall == OverallReadiness.ready
                                    ? ZiteraColors.ready
                                    : _overall == OverallReadiness.needsRepair
                                        ? const Color(0xFF92400E)
                                        : _overall == OverallReadiness.checking
                                            ? ZiteraColors.primary
                                            : ZiteraColors.error,
                              ),
                            ),
                            const SizedBox(height: 2),
                            Text(
                              _overall == OverallReadiness.ready
                                  ? 'The bundled Rust engine, signed catalog, Windows runtime, and package trust boundaries are fully verified.'
                                  : _overall == OverallReadiness.needsRepair
                                      ? 'One or more components require repair or optional updates before full curriculum interaction.'
                                      : _overall == OverallReadiness.checking
                                          ? 'Scanning release bundle directories, engine process contracts, and security descriptors...'
                                          : 'Critical components are missing or corrupted. Review details and recommendations below.',
                              style: const TextStyle(
                                fontFamily: 'SpaceGrotesk',
                                fontSize: 12,
                                color: ZiteraColors.textSecondary,
                              ),
                            ),
                          ],
                        ),
                      ),
                      const SizedBox(width: 12),
                      Row(
                        children: [
                          ZiteraButton(
                            label: _isRunning ? 'Auditing...' : 'Re-Run Audit',
                            icon: Icons.refresh,
                            onPressed: _isRunning ? null : _startAudit,
                          ),
                          if (_overall == OverallReadiness.needsRepair ||
                              _overall == OverallReadiness.checkFailed) ...[
                            const SizedBox(width: 8),
                            ZiteraButton(
                              label: _isRepairing ? 'Repairing...' : 'Repair Setup',
                              icon: Icons.build,
                              onPressed: (_isRunning || _isRepairing) ? null : _handleRepair,
                            ),
                          ],
                        ],
                      ),
                    ],
                  ),
                ),
              ),

              const SizedBox(height: 24),

              // Layer A — Visual Progress Cards
              const Text(
                'COMPONENT AUDIT CHECKLIST',
                style: TextStyle(
                  fontFamily: 'SpaceGrotesk',
                  fontWeight: FontWeight.bold,
                  fontSize: 12,
                  letterSpacing: 1.0,
                  color: ZiteraColors.textMuted,
                ),
              ),
              const SizedBox(height: 12),

              ListView.separated(
                shrinkWrap: true,
                physics: const NeverScrollableScrollPhysics(),
                itemCount: _items.length,
                separatorBuilder: (ctx, i) => const SizedBox(height: 10),
                itemBuilder: (ctx, idx) {
                  final item = _items[idx];
                  final statusColor = _getStatusColor(item.status);
                  final statusIcon = _getStatusIcon(item.status);
                  final statusLabel = _formatStatusLabel(item.status);

                  return HackerTilixEntrance(
                    delay: Duration(milliseconds: 50 + idx * 30),
                    direction: TilixSlideDirection.up,
                    child: ZiteraCard(
                      borderColor: statusColor.withValues(alpha: 0.35),
                      child: Padding(
                        padding: const EdgeInsets.all(14.0),
                        child: Row(
                          crossAxisAlignment: CrossAxisAlignment.start,
                          children: [
                            Container(
                              padding: const EdgeInsets.all(8),
                              decoration: BoxDecoration(
                                color: statusColor.withValues(alpha: 0.1),
                                shape: BoxShape.circle,
                              ),
                              child: Icon(statusIcon, color: statusColor, size: 20),
                            ),
                            const SizedBox(width: 14),
                            Expanded(
                              child: Column(
                                crossAxisAlignment: CrossAxisAlignment.start,
                                children: [
                                  Row(
                                    mainAxisAlignment: MainAxisAlignment.spaceBetween,
                                    children: [
                                      Text(
                                        item.name,
                                        style: const TextStyle(
                                          fontFamily: 'SpaceGrotesk',
                                          fontWeight: FontWeight.bold,
                                          fontSize: 14,
                                          color: ZiteraColors.textPrimary,
                                        ),
                                      ),
                                      Container(
                                        padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 3),
                                        decoration: BoxDecoration(
                                          color: statusColor.withValues(alpha: 0.12),
                                          borderRadius: BorderRadius.circular(4),
                                          border: Border.all(color: statusColor, width: 1),
                                        ),
                                        child: Text(
                                          statusLabel,
                                          style: TextStyle(
                                            fontFamily: 'JetBrainsMono',
                                            fontSize: 10.5,
                                            fontWeight: FontWeight.bold,
                                            color: statusColor,
                                            letterSpacing: 0.6,
                                          ),
                                        ),
                                      ),
                                    ],
                                  ),
                                  const SizedBox(height: 4),
                                  Text(
                                    item.description,
                                    style: const TextStyle(
                                      fontFamily: 'SpaceGrotesk',
                                      fontSize: 12,
                                      color: ZiteraColors.textMuted,
                                    ),
                                  ),
                                  const SizedBox(height: 6),
                                  Container(
                                    padding: const EdgeInsets.symmetric(horizontal: 10, vertical: 6),
                                    decoration: BoxDecoration(
                                      color: const Color(0xFFF5F3EF),
                                      borderRadius: BorderRadius.circular(4),
                                    ),
                                    child: Text(
                                      item.details,
                                      style: const TextStyle(
                                        fontFamily: 'JetBrainsMono',
                                        fontSize: 11,
                                        color: Color(0xFF475569),
                                      ),
                                    ),
                                  ),
                                  if (item.recoveryAction != null) ...[
                                    const SizedBox(height: 6),
                                    Row(
                                      children: [
                                        const Icon(Icons.help_outline, size: 14, color: Color(0xFFD97706)),
                                        const SizedBox(width: 6),
                                        Expanded(
                                          child: Text(
                                            'Recommendation: ${item.recoveryAction!}',
                                            style: const TextStyle(
                                              fontFamily: 'SpaceGrotesk',
                                              fontSize: 11.5,
                                              fontWeight: FontWeight.w600,
                                              color: Color(0xFFB45309),
                                            ),
                                          ),
                                        ),
                                      ],
                                    ),
                                  ],
                                ],
                              ),
                            ),
                          ],
                        ),
                      ),
                    ),
                  );
                },
              ),

              const SizedBox(height: 24),

              // Layer B — Expandable Execution Log (CP03)
              HackerTilixEntrance(
                delay: const Duration(milliseconds: 120),
                direction: TilixSlideDirection.up,
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Row(
                      mainAxisAlignment: MainAxisAlignment.spaceBetween,
                      children: [
                        TextButton.icon(
                          onPressed: () => setState(() => _showLogs = !_showLogs),
                          icon: Icon(
                            _showLogs ? Icons.keyboard_arrow_up : Icons.keyboard_arrow_down,
                            color: ZiteraColors.primary,
                          ),
                          label: Text(
                            _showLogs ? 'HIDE EXECUTION LOGS' : 'VIEW DETAILED EXECUTION LOGS (${_logs.length})',
                            style: const TextStyle(
                              fontFamily: 'JetBrainsMono',
                              fontWeight: FontWeight.bold,
                              fontSize: 12,
                              color: ZiteraColors.primary,
                              letterSpacing: 0.8,
                            ),
                          ),
                        ),
                        if (_showLogs)
                          Row(
                            children: [
                              TextButton.icon(
                                onPressed: () => setState(() => _autoScroll = !_autoScroll),
                                icon: Icon(
                                  _autoScroll ? Icons.lock : Icons.lock_open,
                                  size: 14,
                                  color: ZiteraColors.textMuted,
                                ),
                                label: Text(
                                  _autoScroll ? 'Auto-scroll On' : 'Auto-scroll Off',
                                  style: const TextStyle(
                                    fontFamily: 'SpaceGrotesk',
                                    fontSize: 11,
                                    color: ZiteraColors.textMuted,
                                  ),
                                ),
                              ),
                              const SizedBox(width: 8),
                              ZiteraButton(
                                label: 'Copy Sanitized Log',
                                icon: Icons.copy,
                                onPressed: _copyLog,
                              ),
                            ],
                          ),
                      ],
                    ),

                    if (_showLogs) ...[
                      const SizedBox(height: 8),
                      Container(
                        height: 240,
                        width: double.infinity,
                        padding: const EdgeInsets.all(14.0),
                        decoration: BoxDecoration(
                          color: const Color(0xFF0F172A),
                          borderRadius: BorderRadius.circular(8),
                          border: Border.all(color: const Color(0xFF334155), width: 1.2),
                        ),
                        child: _logs.isEmpty
                            ? const Center(
                                child: Text(
                                  'No log events recorded yet.',
                                  style: TextStyle(
                                    fontFamily: 'JetBrainsMono',
                                    fontSize: 12,
                                    color: Color(0xFF64748B),
                                  ),
                                ),
                              )
                            : ListView.builder(
                                controller: _logScrollController,
                                itemCount: _logs.length,
                                itemBuilder: (ctx, i) {
                                  final line = _logs[i];
                                  final isError = line.contains('[ERROR]');
                                  final isWarn = line.contains('[WARN]');
                                  return Padding(
                                    padding: const EdgeInsets.symmetric(vertical: 1.5),
                                    child: Text(
                                      line,
                                      style: TextStyle(
                                        fontFamily: 'JetBrainsMono',
                                        fontSize: 11,
                                        height: 1.35,
                                        color: isError
                                            ? const Color(0xFFF87171)
                                            : isWarn
                                                ? const Color(0xFFFBBF24)
                                                : const Color(0xFFCBD5E1),
                                      ),
                                    ),
                                  );
                                },
                              ),
                      ),
                    ],
                  ],
                ),
              ),

              const SizedBox(height: 32),
            ],
          ),
        ),
      ),
    );
  }
}
