import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import '../core/ipc/engine_client.dart';
import '../core/ipc/models.dart';
import '../core/theme/zitera_colors.dart';
import 'zitera_button.dart';

/// Models the 3-question recovery payload answering:
/// 1. What happened?
/// 2. Why?
/// 3. What can I do?
class FailureRecoveryInfo {
  final FailureCategory category;
  final String title;
  final String explanation;
  final String actionAdvice;
  final String code;
  final String rawMessage;
  final String? recoveryAction;
  final String? details;
  final IconData icon;
  final Color badgeBg;
  final Color badgeFg;
  final Color badgeBorder;

  const FailureRecoveryInfo({
    required this.category,
    required this.title,
    required this.explanation,
    required this.actionAdvice,
    required this.code,
    required this.rawMessage,
    this.recoveryAction,
    this.details,
    required this.icon,
    required this.badgeBg,
    required this.badgeFg,
    required this.badgeBorder,
  });

  factory FailureRecoveryInfo.fromError(Object error, {String? labId}) {
    String code = 'UNKNOWN_ERROR';
    String message = error.toString();
    String? recovery;
    String? details;
    FailureCategory category = FailureCategory.general;

    if (error is ZiteraException) {
      code = error.code;
      message = error.message;
      recovery = error.recoveryAction;
      details = error.details;
      category = error.category;
    } else {
      final m = message.toLowerCase();
      if (m.contains('crash') || m.contains('timed out waiting') || m.contains('died')) {
        category = FailureCategory.labCrash;
        code = 'LAB_CRASH';
      } else if (m.contains('rollback') || m.contains('rolled back')) {
        category = FailureCategory.rollbackOccurred;
        code = 'ROLLBACK_OCCURRED';
      } else if (m.contains('interrupted') || m.contains('staging')) {
        category = FailureCategory.updateInterrupted;
        code = 'UPDATE_INTERRUPTED';
      } else if (m.contains('runtime') || m.contains('appcontainer') || m.contains('executable not found')) {
        category = FailureCategory.runtimeUnavailable;
        code = 'RUNTIME_UNAVAILABLE';
      } else if (m.contains('verification') || m.contains('signature') || m.contains('checksum') || m.contains('downgrade')) {
        category = FailureCategory.packageVerificationFailure;
        code = 'PACKAGE_VERIFICATION_FAILED';
      } else if (m.contains('storage') || m.contains('permission denied') || m.contains('disk')) {
        category = FailureCategory.storageFailure;
        code = 'STORAGE_FAILURE';
      } else if (m.contains('broker') || m.contains('port') || m.contains('address already in use')) {
        category = FailureCategory.brokerUnavailable;
        code = 'BROKER_UNAVAILABLE';
      }
    }

    final target = labId != null ? 'Lab $labId' : 'The lab environment';

    switch (category) {
      case FailureCategory.labCrash:
        return FailureRecoveryInfo(
          category: category,
          title: '$target failed to start or crashed unexpectedly',
          explanation: 'The sandboxed lab process terminated unexpectedly or failed the initial startup readiness probe.',
          actionAdvice: 'Restart the lab, reset its runtime memory state to default seed, reinstall the package, or export diagnostics.',
          code: code,
          rawMessage: message,
          recoveryAction: recovery ?? 'Restart or reset the lab process to clear volatile runtime locks.',
          details: details,
          icon: Icons.error_outline_rounded,
          badgeBg: const Color(0xFFFEE2E2),
          badgeFg: const Color(0xFF991B1B),
          badgeBorder: const Color(0xFFFCA5A5),
        );

      case FailureCategory.runtimeUnavailable:
        return FailureRecoveryInfo(
          category: category,
          title: 'Native Sandbox Runtime is unavailable',
          explanation: 'Windows AppContainer isolation or native process boundaries could not be initialized for the current user.',
          actionAdvice: 'Verify runtime prerequisites using Zitera Doctor, ensure antivirus is not blocking loopback pipes, or export diagnostics.',
          code: code,
          rawMessage: message,
          recoveryAction: recovery ?? 'Run "zitera doctor" in Settings to inspect host capabilities.',
          details: details,
          icon: Icons.shield_outlined,
          badgeBg: const Color(0xFFFEF3C7),
          badgeFg: const Color(0xFF92400E),
          badgeBorder: const Color(0xFFFCD34D),
        );

      case FailureCategory.packageVerificationFailure:
        return FailureRecoveryInfo(
          category: category,
          title: 'Package verification failed',
          explanation: 'Digital Ed25519 signature, SHA256 integrity digest, or anti-downgrade security policy rejected the package bundle.',
          actionAdvice: 'Reinstall or re-download the authentic lab package from the catalog, verify package integrity, or export diagnostics.',
          code: code,
          rawMessage: message,
          recoveryAction: recovery ?? 'Ensure package file (.zlab) has not been tampered with or modified.',
          details: details,
          icon: Icons.verified_user_outlined,
          badgeBg: const Color(0xFFFEE2E2),
          badgeFg: const Color(0xFF991B1B),
          badgeBorder: const Color(0xFFFCA5A5),
        );

      case FailureCategory.updateInterrupted:
        return FailureRecoveryInfo(
          category: category,
          title: 'Package update was interrupted',
          explanation: 'The package update was halted before atomic version staging could complete. Your existing lab version was safely preserved.',
          actionAdvice: 'Retry the update, clean stale staging data, reset to the previous version, or export diagnostics.',
          code: code,
          rawMessage: message,
          recoveryAction: recovery ?? 'Retry the update operation to complete staging.',
          details: details,
          icon: Icons.update_disabled_rounded,
          badgeBg: const Color(0xFFFFEDD5),
          badgeFg: const Color(0xFF9A3412),
          badgeBorder: const Color(0xFFFDBA74),
        );

      case FailureCategory.rollbackOccurred:
        return FailureRecoveryInfo(
          category: category,
          title: 'Automated rollback executed',
          explanation: 'Post-update verification failed. The engine safely restored the previous stable version without student data loss.',
          actionAdvice: 'Restart the lab on its previous stable version, verify package compatibility, or export diagnostics.',
          code: code,
          rawMessage: message,
          recoveryAction: recovery ?? 'Continue learning on the previous stable version while diagnosing the update.',
          details: details,
          icon: Icons.restore_page_outlined,
          badgeBg: const Color(0xFFE0E7FF),
          badgeFg: const Color(0xFF3730A3),
          badgeBorder: const Color(0xFFA5B4FC),
        );

      case FailureCategory.storageFailure:
        return FailureRecoveryInfo(
          category: category,
          title: 'Storage subsystem access failure',
          explanation: 'File write access was denied, storage was locked by another process, or disk quota was exceeded in %LOCALAPPDATA%\\ZiteraLab.',
          actionAdvice: 'Purge cache files, ensure current user permissions on LocalAppData, or export diagnostics.',
          code: code,
          rawMessage: message,
          recoveryAction: recovery ?? 'Purge cache via Settings or verify disk space.',
          details: details,
          icon: Icons.storage_rounded,
          badgeBg: const Color(0xFFFEF3C7),
          badgeFg: const Color(0xFF92400E),
          badgeBorder: const Color(0xFFFCD34D),
        );

      case FailureCategory.brokerUnavailable:
        return FailureRecoveryInfo(
          category: category,
          title: 'Host broker service unavailable',
          explanation: 'The local loopback HTTP broker could not bind to its assigned port or lost connection with the client.',
          actionAdvice: 'Check that port is not occupied, restart the lab broker, or export diagnostics.',
          code: code,
          rawMessage: message,
          recoveryAction: recovery ?? 'Restart the lab to assign a fresh ephemeral port.',
          details: details,
          icon: Icons.cloud_off_rounded,
          badgeBg: const Color(0xFFFEE2E2),
          badgeFg: const Color(0xFF991B1B),
          badgeBorder: const Color(0xFFFCA5A5),
        );

      case FailureCategory.general:
        return FailureRecoveryInfo(
          category: category,
          title: 'Operation encountered an issue',
          explanation: 'The operation could not complete normally due to an unexpected engine error.',
          actionAdvice: 'Retry the action, reset the lab state, or export a privacy-sanitized diagnostic bundle.',
          code: code,
          rawMessage: message,
          recoveryAction: recovery ?? 'Retry the operation or inspect technical details.',
          details: details,
          icon: Icons.warning_amber_rounded,
          badgeBg: const Color(0xFFF3F4F6),
          badgeFg: const Color(0xFF374151),
          badgeBorder: const Color(0xFFD1D5DB),
        );
    }
  }
}

/// Human-Actionable Failure Recovery Dialog answering:
/// - What happened?
/// - Why?
/// - What can I do?
class FailureRecoveryDialog extends StatefulWidget {
  final Object error;
  final String? labId;
  final String? actionContext;
  final VoidCallback? onRestart;
  final VoidCallback? onReset;
  final VoidCallback? onReinstall;
  final VoidCallback? onPurgeCache;
  final VoidCallback? onRefresh;

  const FailureRecoveryDialog({
    super.key,
    required this.error,
    this.labId,
    this.actionContext,
    this.onRestart,
    this.onReset,
    this.onReinstall,
    this.onPurgeCache,
    this.onRefresh,
  });

  static Future<void> show(
    BuildContext context, {
    required Object error,
    String? labId,
    String? actionContext,
    VoidCallback? onRestart,
    VoidCallback? onReset,
    VoidCallback? onReinstall,
    VoidCallback? onPurgeCache,
    VoidCallback? onRefresh,
  }) {
    return showDialog<void>(
      context: context,
      barrierDismissible: true,
      builder: (ctx) => FailureRecoveryDialog(
        error: error,
        labId: labId,
        actionContext: actionContext,
        onRestart: onRestart,
        onReset: onReset,
        onReinstall: onReinstall,
        onPurgeCache: onPurgeCache,
        onRefresh: onRefresh,
      ),
    );
  }

  @override
  State<FailureRecoveryDialog> createState() => _FailureRecoveryDialogState();
}

class _FailureRecoveryDialogState extends State<FailureRecoveryDialog> {
  late FailureRecoveryInfo _info;
  bool _isExporting = false;
  String? _exportedPath;
  bool _isResetting = false;
  bool _isRestarting = false;
  bool _isReinstalling = false;
  bool _isPurging = false;
  bool _showTechnicalDetails = false;
  String? _operationFeedback;

  @override
  void initState() {
    super.initState();
    _info = FailureRecoveryInfo.fromError(widget.error, labId: widget.labId);
  }

  Future<void> _handleExportDiagnostics() async {
    setState(() {
      _isExporting = true;
      _operationFeedback = null;
    });

    try {
      final res = await ZiteraEngineClient.exportDiagnostics();
      final path = res['path'] as String? ?? 'diagnostics.zip';
      if (mounted) {
        setState(() {
          _isExporting = false;
          _exportedPath = path;
          _operationFeedback = 'Diagnostic bundle exported successfully.';
        });
      }
    } catch (e) {
      if (mounted) {
        setState(() {
          _isExporting = false;
          _operationFeedback = 'Diagnostics export failed: $e';
        });
      }
    }
  }

  Future<void> _handleReset() async {
    if (widget.labId == null && widget.onReset == null) return;
    setState(() {
      _isResetting = true;
      _operationFeedback = null;
    });

    try {
      if (widget.onReset != null) {
        widget.onReset!();
      } else if (widget.labId != null) {
        await ZiteraEngineClient.resetLab(widget.labId!);
      }
      if (widget.onRefresh != null) widget.onRefresh!();
      if (mounted) {
        Navigator.of(context).pop();
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(
            content: Text('Lab ${widget.labId ?? ""} state deterministically reset.'),
            backgroundColor: ZiteraColors.ready,
          ),
        );
      }
    } catch (e) {
      if (mounted) {
        setState(() {
          _isResetting = false;
          _operationFeedback = 'Reset error: $e';
        });
      }
    }
  }

  Future<void> _handleRestart() async {
    if (widget.onRestart == null && widget.labId == null) return;
    setState(() {
      _isRestarting = true;
      _operationFeedback = null;
    });

    try {
      if (widget.onRestart != null) {
        widget.onRestart!();
      } else if (widget.labId != null) {
        await ZiteraEngineClient.startLab(widget.labId!);
      }
      if (widget.onRefresh != null) widget.onRefresh!();
      if (mounted) {
        Navigator.of(context).pop();
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(
            content: Text('Lab ${widget.labId ?? ""} restarted.'),
            backgroundColor: ZiteraColors.ready,
          ),
        );
      }
    } catch (e) {
      if (mounted) {
        setState(() {
          _isRestarting = false;
          _operationFeedback = 'Restart error: $e';
          _info = FailureRecoveryInfo.fromError(e, labId: widget.labId);
        });
      }
    }
  }

  Future<void> _handleReinstall() async {
    if (widget.labId == null && widget.onReinstall == null) return;
    setState(() {
      _isReinstalling = true;
      _operationFeedback = null;
    });

    try {
      if (widget.onReinstall != null) {
        widget.onReinstall!();
      } else if (widget.labId != null) {
        await ZiteraEngineClient.installLab(widget.labId!);
      }
      if (widget.onRefresh != null) widget.onRefresh!();
      if (mounted) {
        Navigator.of(context).pop();
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(
            content: Text('Lab ${widget.labId ?? ""} reinstalled successfully.'),
            backgroundColor: ZiteraColors.ready,
          ),
        );
      }
    } catch (e) {
      if (mounted) {
        setState(() {
          _isReinstalling = false;
          _operationFeedback = 'Reinstall error: $e';
        });
      }
    }
  }

  Future<void> _handlePurgeCache() async {
    setState(() {
      _isPurging = true;
      _operationFeedback = null;
    });

    try {
      if (widget.onPurgeCache != null) {
        widget.onPurgeCache!();
      } else {
        await ZiteraEngineClient.executeCommand(['storage', 'purge-cache']);
      }
      if (mounted) {
        setState(() {
          _isPurging = false;
          _operationFeedback = 'Cache purged successfully.';
        });
      }
    } catch (e) {
      if (mounted) {
        setState(() {
          _isPurging = false;
          _operationFeedback = 'Purge cache error: $e';
        });
      }
    }
  }

  @override
  Widget build(BuildContext context) {
    return Dialog(
      backgroundColor: Colors.transparent,
      insetPadding: const EdgeInsets.symmetric(horizontal: 24, vertical: 24),
      child: Container(
        constraints: const BoxConstraints(maxWidth: 580),
        decoration: BoxDecoration(
          color: ZiteraColors.card,
          borderRadius: BorderRadius.circular(16),
          border: Border.all(color: ZiteraColors.borderDark, width: 1.5),
          boxShadow: const [
            BoxShadow(
              color: Color(0x14000000),
              offset: Offset(4, 6),
              blurRadius: 16,
            ),
          ],
        ),
        padding: const EdgeInsets.all(24),
        child: SingleChildScrollView(
          child: Column(
            mainAxisSize: MainAxisSize.min,
            crossAxisAlignment: CrossAxisAlignment.stretch,
            children: [
              // Top Bar: Badge & Action Context
              Row(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Container(
                    padding: const EdgeInsets.all(10),
                    decoration: BoxDecoration(
                      color: _info.badgeBg,
                      borderRadius: BorderRadius.circular(10),
                      border: Border.all(color: _info.badgeBorder, width: 1.2),
                    ),
                    child: Icon(_info.icon, color: _info.badgeFg, size: 24),
                  ),
                  const SizedBox(width: 14),
                  Expanded(
                    child: Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        Row(
                          children: [
                            Container(
                              padding: const EdgeInsets.symmetric(horizontal: 7, vertical: 2),
                              decoration: BoxDecoration(
                                color: _info.badgeBg,
                                borderRadius: BorderRadius.circular(5),
                                border: Border.all(color: _info.badgeBorder),
                              ),
                              child: Text(
                                'FAILURE RECOVERY ASSISTANT',
                                style: TextStyle(
                                  fontFamily: 'SpaceGrotesk',
                                  fontSize: 10,
                                  fontWeight: FontWeight.w800,
                                  color: _info.badgeFg,
                                  letterSpacing: 0.8,
                                ),
                              ),
                            ),
                            const Spacer(),
                            IconButton(
                              icon: const Icon(Icons.close, size: 18, color: ZiteraColors.textMuted),
                              onPressed: () => Navigator.of(context).pop(),
                              splashRadius: 16,
                              padding: EdgeInsets.zero,
                              constraints: const BoxConstraints(),
                            ),
                          ],
                        ),
                        const SizedBox(height: 6),
                        // 1. WHAT HAPPENED?
                        Text(
                          _info.title,
                          style: const TextStyle(
                            fontFamily: 'SpaceGrotesk',
                            fontSize: 18,
                            fontWeight: FontWeight.bold,
                            color: ZiteraColors.textPrimary,
                            height: 1.2,
                          ),
                        ),
                        if (widget.actionContext != null) ...[
                          const SizedBox(height: 3),
                          Text(
                            'Action: ${widget.actionContext}',
                            style: const TextStyle(
                              fontFamily: 'JetBrainsMono',
                              fontSize: 11,
                              color: ZiteraColors.textSecondary,
                            ),
                          ),
                        ],
                      ],
                    ),
                  ),
                ],
              ),
              const SizedBox(height: 18),

              // 2. WHY? (Container explaining root cause)
              Container(
                padding: const EdgeInsets.all(14),
                decoration: BoxDecoration(
                  color: ZiteraColors.surface,
                  borderRadius: BorderRadius.circular(10),
                  border: Border.all(color: ZiteraColors.border, width: 1),
                ),
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Row(
                      children: [
                        const Icon(Icons.help_outline, size: 14, color: ZiteraColors.textSecondary),
                        const SizedBox(width: 6),
                        const Text(
                          'WHY DID THIS HAPPEN?',
                          style: TextStyle(
                            fontFamily: 'SpaceGrotesk',
                            fontSize: 11,
                            fontWeight: FontWeight.bold,
                            color: ZiteraColors.textSecondary,
                            letterSpacing: 0.5,
                          ),
                        ),
                        const Spacer(),
                        Container(
                          padding: const EdgeInsets.symmetric(horizontal: 6, vertical: 1.5),
                          decoration: BoxDecoration(
                            color: const Color(0xFFF1F5F9),
                            borderRadius: BorderRadius.circular(4),
                            border: Border.all(color: const Color(0xFFCBD5E1)),
                          ),
                          child: Text(
                            'CODE: ${_info.code}',
                            style: const TextStyle(
                              fontFamily: 'JetBrainsMono',
                              fontSize: 9.5,
                              fontWeight: FontWeight.w600,
                              color: Color(0xFF475569),
                            ),
                          ),
                        ),
                      ],
                    ),
                    const SizedBox(height: 8),
                    Text(
                      _info.explanation,
                      style: const TextStyle(
                        fontFamily: 'SpaceGrotesk',
                        fontSize: 12.5,
                        color: ZiteraColors.textPrimary,
                        height: 1.35,
                      ),
                    ),
                    const SizedBox(height: 8),

                    // Expandable Technical Details
                    InkWell(
                      onTap: () {
                        setState(() {
                          _showTechnicalDetails = !_showTechnicalDetails;
                        });
                      },
                      child: Row(
                        mainAxisSize: MainAxisSize.min,
                        children: [
                          Icon(
                            _showTechnicalDetails ? Icons.expand_less : Icons.expand_more,
                            size: 16,
                            color: ZiteraColors.primary,
                          ),
                          const SizedBox(width: 4),
                          Text(
                            _showTechnicalDetails ? 'Hide technical error details' : 'Show technical error details',
                            style: const TextStyle(
                              fontFamily: 'SpaceGrotesk',
                              fontSize: 11.5,
                              fontWeight: FontWeight.bold,
                              color: ZiteraColors.primary,
                            ),
                          ),
                        ],
                      ),
                    ),
                    if (_showTechnicalDetails) ...[
                      const SizedBox(height: 8),
                      Container(
                        width: double.infinity,
                        padding: const EdgeInsets.all(10),
                        decoration: BoxDecoration(
                          color: const Color(0xFF1E1A14),
                          borderRadius: BorderRadius.circular(6),
                        ),
                        child: SelectableText(
                          'Error: ${_info.rawMessage}\n'
                          'Remediation: ${_info.recoveryAction ?? "None"}'
                          '${_info.details != null ? "\nDetails: ${_info.details}" : ""}',
                          style: const TextStyle(
                            fontFamily: 'JetBrainsMono',
                            fontSize: 10.5,
                            color: Color(0xFFE2E8F0),
                            height: 1.3,
                          ),
                        ),
                      ),
                    ],
                  ],
                ),
              ),
              const SizedBox(height: 16),

              // 3. WHAT CAN I DO? (Action Guidance)
              const Text(
                'WHAT CAN YOU DO?',
                style: TextStyle(
                  fontFamily: 'SpaceGrotesk',
                  fontSize: 11,
                  fontWeight: FontWeight.bold,
                  color: ZiteraColors.textSecondary,
                  letterSpacing: 0.5,
                ),
              ),
              const SizedBox(height: 4),
              Text(
                _info.actionAdvice,
                style: const TextStyle(
                  fontFamily: 'SpaceGrotesk',
                  fontSize: 12.5,
                  color: ZiteraColors.textPrimary,
                  height: 1.3,
                ),
              ),

              // Inline Operation Feedback
              if (_operationFeedback != null) ...[
                const SizedBox(height: 12),
                Container(
                  padding: const EdgeInsets.all(10),
                  decoration: BoxDecoration(
                    color: _operationFeedback!.contains('failed') || _operationFeedback!.contains('error')
                        ? ZiteraColors.errorMuted
                        : ZiteraColors.readyMuted,
                    borderRadius: BorderRadius.circular(8),
                    border: Border.all(
                      color: _operationFeedback!.contains('failed') || _operationFeedback!.contains('error')
                          ? ZiteraColors.error
                          : ZiteraColors.ready,
                    ),
                  ),
                  child: Row(
                    children: [
                      Icon(
                        _operationFeedback!.contains('failed') || _operationFeedback!.contains('error')
                            ? Icons.error_outline
                            : Icons.check_circle_outline,
                        size: 16,
                        color: _operationFeedback!.contains('failed') || _operationFeedback!.contains('error')
                            ? ZiteraColors.error
                            : ZiteraColors.ready,
                      ),
                      const SizedBox(width: 8),
                      Expanded(
                        child: Text(
                          _operationFeedback!,
                          style: TextStyle(
                            fontFamily: 'SpaceGrotesk',
                            fontSize: 11.5,
                            fontWeight: FontWeight.bold,
                            color: _operationFeedback!.contains('failed') || _operationFeedback!.contains('error')
                                ? ZiteraColors.error
                                : ZiteraColors.ready,
                          ),
                        ),
                      ),
                    ],
                  ),
                ),
              ],

              // Exported Diagnostics Path Box
              if (_exportedPath != null) ...[
                const SizedBox(height: 10),
                Container(
                  padding: const EdgeInsets.all(10),
                  decoration: BoxDecoration(
                    color: const Color(0xFFCCFBF1),
                    borderRadius: BorderRadius.circular(8),
                    border: Border.all(color: const Color(0xFF2DD4BF)),
                  ),
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      const Text(
                        'DIAGNOSTIC BUNDLE READY (REDACTED)',
                        style: TextStyle(
                          fontFamily: 'SpaceGrotesk',
                          fontSize: 10,
                          fontWeight: FontWeight.bold,
                          color: Color(0xFF0F766E),
                          letterSpacing: 0.5,
                        ),
                      ),
                      const SizedBox(height: 4),
                      Row(
                        children: [
                          Expanded(
                            child: SelectableText(
                              _exportedPath!,
                              style: const TextStyle(
                                fontFamily: 'JetBrainsMono',
                                fontSize: 11,
                                color: Color(0xFF134E4A),
                              ),
                            ),
                          ),
                          const SizedBox(width: 8),
                          IconButton(
                            icon: const Icon(Icons.copy, size: 16, color: Color(0xFF0F766E)),
                            tooltip: 'Copy path',
                            onPressed: () {
                              Clipboard.setData(ClipboardData(text: _exportedPath!));
                              ScaffoldMessenger.of(context).showSnackBar(
                                const SnackBar(
                                  content: Text('Diagnostics path copied to clipboard!'),
                                  backgroundColor: Color(0xFF0F766E),
                                  duration: Duration(seconds: 2),
                                ),
                              );
                            },
                            splashRadius: 16,
                            padding: EdgeInsets.zero,
                            constraints: const BoxConstraints(),
                          ),
                        ],
                      ),
                    ],
                  ),
                ),
              ],
              const SizedBox(height: 20),

              // Action Buttons Row
              Wrap(
                alignment: WrapAlignment.end,
                spacing: 8,
                runSpacing: 8,
                children: [
                  // Dismiss
                  ZiteraButton(
                    label: 'Dismiss',
                    variant: ButtonVariant.ghost,
                    onPressed: () => Navigator.of(context).pop(),
                  ),

                  // Purge Cache (only if storage failure)
                  if (_info.category == FailureCategory.storageFailure)
                    ZiteraButton(
                      label: 'Purge Cache',
                      icon: Icons.cleaning_services_outlined,
                      variant: ButtonVariant.secondary,
                      isLoading: _isPurging,
                      onPressed: _isPurging ? null : _handlePurgeCache,
                    ),

                  // Reinstall Lab
                  if (widget.labId != null || widget.onReinstall != null)
                    ZiteraButton(
                      label: 'Reinstall Lab',
                      icon: Icons.file_download_outlined,
                      variant: ButtonVariant.secondary,
                      isLoading: _isReinstalling,
                      onPressed: _isReinstalling ? null : _handleReinstall,
                    ),

                  // Reset Lab
                  if (widget.labId != null || widget.onReset != null)
                    ZiteraButton(
                      label: 'Reset Lab',
                      icon: Icons.restore,
                      variant: ButtonVariant.danger,
                      isLoading: _isResetting,
                      onPressed: _isResetting ? null : _handleReset,
                    ),

                  // Export Diagnostics
                  ZiteraButton(
                    label: 'Export Diagnostics',
                    icon: Icons.shield_outlined,
                    variant: ButtonVariant.mint,
                    isLoading: _isExporting,
                    onPressed: _isExporting ? null : _handleExportDiagnostics,
                  ),

                  // Restart / Retry (Primary)
                  if (widget.onRestart != null || widget.labId != null)
                    ZiteraButton(
                      label: 'Restart Lab',
                      icon: Icons.refresh,
                      variant: ButtonVariant.primary,
                      isLoading: _isRestarting,
                      onPressed: _isRestarting ? null : _handleRestart,
                    ),
                ],
              ),
            ],
          ),
        ),
      ),
    );
  }
}

/// In-place Failure Recovery Card for inline views answering:
/// - What happened?
/// - Why?
/// - What can I do?
class FailureRecoveryCard extends StatelessWidget {
  final Object error;
  final String? labId;
  final VoidCallback? onRetry;
  final VoidCallback? onReset;
  final VoidCallback? onExportDiagnostics;

  const FailureRecoveryCard({
    super.key,
    required this.error,
    this.labId,
    this.onRetry,
    this.onReset,
    this.onExportDiagnostics,
  });

  @override
  Widget build(BuildContext context) {
    final info = FailureRecoveryInfo.fromError(error, labId: labId);

    return Container(
      constraints: const BoxConstraints(maxWidth: 620),
      margin: const EdgeInsets.symmetric(horizontal: 16, vertical: 24),
      padding: const EdgeInsets.all(22),
      decoration: BoxDecoration(
        color: ZiteraColors.card,
        borderRadius: BorderRadius.circular(14),
        border: Border.all(color: ZiteraColors.borderDark, width: 1.5),
        boxShadow: const [
          BoxShadow(
            color: Color(0x10000000),
            offset: Offset(0, 4),
            blurRadius: 12,
          ),
        ],
      ),
      child: Column(
        mainAxisSize: MainAxisSize.min,
        crossAxisAlignment: CrossAxisAlignment.stretch,
        children: [
          Row(
            children: [
              Container(
                padding: const EdgeInsets.all(8),
                decoration: BoxDecoration(
                  color: info.badgeBg,
                  borderRadius: BorderRadius.circular(8),
                  border: Border.all(color: info.badgeBorder),
                ),
                child: Icon(info.icon, color: info.badgeFg, size: 22),
              ),
              const SizedBox(width: 12),
              Expanded(
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Text(
                      'FAILURE DETECTED // RECOVERY AVAILABLE',
                      style: TextStyle(
                        fontFamily: 'SpaceGrotesk',
                        fontSize: 10,
                        fontWeight: FontWeight.w800,
                        color: info.badgeFg,
                        letterSpacing: 0.8,
                      ),
                    ),
                    const SizedBox(height: 2),
                    Text(
                      info.title,
                      style: const TextStyle(
                        fontFamily: 'SpaceGrotesk',
                        fontSize: 16,
                        fontWeight: FontWeight.bold,
                        color: ZiteraColors.textPrimary,
                      ),
                    ),
                  ],
                ),
              ),
            ],
          ),
          const SizedBox(height: 14),

          // Why
          Container(
            padding: const EdgeInsets.all(12),
            decoration: BoxDecoration(
              color: ZiteraColors.surface,
              borderRadius: BorderRadius.circular(8),
              border: Border.all(color: ZiteraColors.border),
            ),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                const Text(
                  'WHY DID THIS HAPPEN?',
                  style: TextStyle(
                    fontFamily: 'SpaceGrotesk',
                    fontSize: 10.5,
                    fontWeight: FontWeight.bold,
                    color: ZiteraColors.textSecondary,
                  ),
                ),
                const SizedBox(height: 4),
                Text(
                  info.explanation,
                  style: const TextStyle(
                    fontFamily: 'SpaceGrotesk',
                    fontSize: 12,
                    color: ZiteraColors.textPrimary,
                  ),
                ),
              ],
            ),
          ),
          const SizedBox(height: 12),

          // What can I do
          Text(
            info.actionAdvice,
            style: const TextStyle(
              fontFamily: 'SpaceGrotesk',
              fontSize: 12,
              color: ZiteraColors.textSecondary,
            ),
          ),
          const SizedBox(height: 16),

          // Buttons
          Wrap(
            alignment: WrapAlignment.end,
            spacing: 8,
            runSpacing: 8,
            children: [
              if (onExportDiagnostics != null)
                ZiteraButton(
                  label: 'Export Diagnostics',
                  icon: Icons.shield_outlined,
                  variant: ButtonVariant.mint,
                  onPressed: onExportDiagnostics,
                ),
              if (onReset != null)
                ZiteraButton(
                  label: 'Reset Lab',
                  icon: Icons.restore,
                  variant: ButtonVariant.danger,
                  onPressed: onReset,
                ),
              if (onRetry != null)
                ZiteraButton(
                  label: 'Retry',
                  icon: Icons.refresh,
                  variant: ButtonVariant.primary,
                  onPressed: onRetry,
                ),
            ],
          ),
        ],
      ),
    );
  }
}
