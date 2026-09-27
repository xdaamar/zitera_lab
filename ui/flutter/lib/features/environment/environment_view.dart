import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import '../../core/ipc/engine_client.dart';
import '../../core/ipc/models.dart';
import '../../core/theme/zitera_colors.dart';
import '../../widgets/status_badge.dart';
import '../../widgets/zitera_button.dart';
import '../../widgets/zitera_card.dart';

class EnvironmentView extends StatefulWidget {
  const EnvironmentView({super.key});

  @override
  State<EnvironmentView> createState() => _EnvironmentViewState();
}

class _EnvironmentViewState extends State<EnvironmentView> {
  DiagnosticsResult? _diagnostics;
  List<ToolItem> _tools = [];
  bool _isLoading = true;

  @override
  void initState() {
    super.initState();
    _runCheck();
  }

  Future<void> _runCheck() async {
    setState(() => _isLoading = true);
    try {
      final results = await Future.wait([
        ZiteraEngineClient.runDoctor(),
        ZiteraEngineClient.getTools(),
      ]);
      if (mounted) {
        setState(() {
          _diagnostics = results[0] as DiagnosticsResult;
          _tools = results[1] as List<ToolItem>;
          _isLoading = false;
        });
      }
    } catch (_) {
      if (mounted) {
        setState(() => _isLoading = false);
      }
    }
  }

  void _copyToClipboard(String text, String label) {
    Clipboard.setData(ClipboardData(text: text));
    ScaffoldMessenger.of(context).showSnackBar(
      SnackBar(
        content: Text('$label copied to clipboard!'),
        backgroundColor: ZiteraColors.ready,
        duration: const Duration(seconds: 2),
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    if (_isLoading) {
      return const Center(child: CircularProgressIndicator(color: ZiteraColors.primary));
    }

    if (_diagnostics == null) {
      return Center(
        child: ZiteraCard(
          borderColor: ZiteraColors.error,
          child: Column(
            mainAxisSize: MainAxisSize.min,
            children: [
              const Icon(Icons.error_outline, color: ZiteraColors.error, size: 36),
              const SizedBox(height: 12),
              const Text('Unable to query system diagnostics.', style: TextStyle(color: ZiteraColors.error)),
              const SizedBox(height: 12),
              ZiteraButton(label: 'Retry Check', icon: Icons.refresh, onPressed: _runCheck),
            ],
          ),
        ),
      );
    }

    final diag = _diagnostics!;
    final readyTools = _tools.where((t) => t.installed).length;

    return SingleChildScrollView(
      padding: const EdgeInsets.all(32.0),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Row(
            mainAxisAlignment: MainAxisAlignment.spaceBetween,
            children: [
              Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: const [
                  Text(
                    'SMART ENVIRONMENT SETUP V2',
                    style: TextStyle(
                      color: ZiteraColors.textPrimary,
                      fontSize: 22,
                      fontWeight: FontWeight.w900,
                      letterSpacing: 1.5,
                      fontFamily: 'monospace',
                    ),
                  ),
                  SizedBox(height: 4),
                  Text(
                    'Detect, classify, explain, recommend, and validate system prerequisites & tools',
                    style: TextStyle(color: ZiteraColors.textSecondary, fontSize: 13),
                  ),
                ],
              ),
              ZiteraButton(
                label: 'Recheck System',
                icon: Icons.refresh,
                variant: ButtonVariant.primary,
                onPressed: _runCheck,
              ),
            ],
          ),

          const SizedBox(height: 24),

          // Overall Readiness Banner
          ZiteraCard(
            borderColor: diag.allReady
                ? ZiteraColors.ready.withValues(alpha: 0.4)
                : ZiteraColors.warning.withValues(alpha: 0.4),
            child: Row(
              children: [
                Icon(
                  diag.allReady ? Icons.check_circle : Icons.warning_amber,
                  color: diag.allReady ? ZiteraColors.ready : ZiteraColors.warning,
                  size: 36,
                ),
                const SizedBox(width: 16),
                Expanded(
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      Text(
                        diag.allReady
                            ? 'ALL CORE SYSTEM PREREQUISITES VERIFIED READY'
                            : 'ACTION REQUIRED TO COMPLETE ENVIRONMENT SETUP',
                        style: const TextStyle(fontWeight: FontWeight.bold, fontSize: 15),
                      ),
                      const SizedBox(height: 4),
                      Text(
                        diag.allReady
                            ? 'Host environment is fully configured. All containerized external labs can be launched locally.'
                            : 'One or more system components require attention. Review guided action steps below.',
                        style: const TextStyle(color: ZiteraColors.textSecondary, fontSize: 12),
                      ),
                    ],
                  ),
                ),
                StatusBadge(status: diag.allReady ? 'READY' : 'ACTION REQUIRED'),
              ],
            ),
          ),

          const SizedBox(height: 20),

          // Host Hardware Resources Card
          Row(
            children: [
              Expanded(
                child: ZiteraCard(
                  child: Row(
                    children: [
                      const Icon(Icons.memory, color: ZiteraColors.cyan, size: 28),
                      const SizedBox(width: 14),
                      Column(
                        crossAxisAlignment: CrossAxisAlignment.start,
                        children: [
                          const Text('HOST MEMORY (RAM)', style: TextStyle(color: ZiteraColors.textMuted, fontSize: 11, fontWeight: FontWeight.bold)),
                          const SizedBox(height: 4),
                          Text('${diag.memoryGb.toStringAsFixed(1)} GB Total', style: const TextStyle(color: ZiteraColors.textPrimary, fontWeight: FontWeight.bold, fontSize: 16)),
                        ],
                      ),
                    ],
                  ),
                ),
              ),
              const SizedBox(width: 16),
              Expanded(
                child: ZiteraCard(
                  child: Row(
                    children: [
                      const Icon(Icons.storage, color: ZiteraColors.primary, size: 28),
                      const SizedBox(width: 14),
                      Column(
                        crossAxisAlignment: CrossAxisAlignment.start,
                        children: [
                          const Text('SYSTEM DRIVE FREE DISK', style: TextStyle(color: ZiteraColors.textMuted, fontSize: 11, fontWeight: FontWeight.bold)),
                          const SizedBox(height: 4),
                          Text('${diag.diskFreeGb.toStringAsFixed(1)} GB Free', style: const TextStyle(color: ZiteraColors.textPrimary, fontWeight: FontWeight.bold, fontSize: 16)),
                        ],
                      ),
                    ],
                  ),
                ),
              ),
              const SizedBox(width: 16),
              Expanded(
                child: ZiteraCard(
                  child: Row(
                    children: [
                      const Icon(Icons.security, color: ZiteraColors.ready, size: 28),
                      const SizedBox(width: 14),
                      Column(
                        crossAxisAlignment: CrossAxisAlignment.start,
                        children: [
                          const Text('SECURITY TOOLS STATUS', style: TextStyle(color: ZiteraColors.textMuted, fontSize: 11, fontWeight: FontWeight.bold)),
                          const SizedBox(height: 4),
                          Text('$readyTools of ${_tools.length} Tools Ready', style: const TextStyle(color: ZiteraColors.textPrimary, fontWeight: FontWeight.bold, fontSize: 16)),
                        ],
                      ),
                    ],
                  ),
                ),
              ),
            ],
          ),

          const SizedBox(height: 28),

          // Core System Prerequisites Section
          const Text(
            'CORE SYSTEM PREREQUISITES',
            style: TextStyle(
              color: ZiteraColors.textPrimary,
              fontSize: 14,
              fontWeight: FontWeight.bold,
              letterSpacing: 1.0,
              fontFamily: 'monospace',
            ),
          ),
          const SizedBox(height: 12),

          _buildComponentCard(diag.os),
          const SizedBox(height: 10),
          _buildComponentCard(diag.git),
          const SizedBox(height: 10),
          _buildComponentCard(diag.wsl),
          const SizedBox(height: 10),
          _buildComponentCard(diag.docker),
          const SizedBox(height: 10),
          _buildComponentCard(diag.dockerDaemon),
          const SizedBox(height: 10),
          _buildComponentCard(diag.powershell),
        ],
      ),
    );
  }

  Widget _buildComponentCard(ComponentItem comp) {
    final isReady = comp.status == 'READY';
    final isBlocked = comp.status == 'BLOCKED';

    return ZiteraCard(
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Row(
            mainAxisAlignment: MainAxisAlignment.spaceBetween,
            children: [
              Row(
                children: [
                  Icon(
                    isReady
                        ? Icons.check_circle_outline
                        : (isBlocked ? Icons.block : Icons.error_outline),
                    color: isReady
                        ? ZiteraColors.ready
                        : (isBlocked ? ZiteraColors.error : ZiteraColors.warning),
                    size: 20,
                  ),
                  const SizedBox(width: 12),
                  Text(
                    comp.name,
                    style: const TextStyle(fontWeight: FontWeight.bold, fontSize: 15),
                  ),
                  if (comp.version != null) ...[
                    const SizedBox(width: 8),
                    Text(
                      '(${comp.version})',
                      style: const TextStyle(color: ZiteraColors.textMuted, fontSize: 12, fontFamily: 'monospace'),
                    ),
                  ],
                ],
              ),
              StatusBadge(status: comp.status),
            ],
          ),
          const SizedBox(height: 8),
          Text(comp.message, style: const TextStyle(color: ZiteraColors.textSecondary, fontSize: 13)),
          if (comp.recommendation != null) ...[
            const SizedBox(height: 12),
            Container(
              padding: const EdgeInsets.symmetric(horizontal: 14, vertical: 10),
              decoration: BoxDecoration(
                color: ZiteraColors.surface,
                borderRadius: BorderRadius.circular(4),
                border: Border.all(color: ZiteraColors.border),
              ),
              child: Row(
                children: [
                  const Icon(Icons.build_circle_outlined, size: 18, color: ZiteraColors.cyan),
                  const SizedBox(width: 10),
                  Expanded(
                    child: Text(
                      'Recommended Action: ${comp.recommendation!}',
                      style: const TextStyle(color: ZiteraColors.cyan, fontSize: 12, fontFamily: 'monospace'),
                    ),
                  ),
                  const SizedBox(width: 8),
                  TextButton.icon(
                    icon: const Icon(Icons.copy, size: 14, color: ZiteraColors.textPrimary),
                    label: const Text('Copy Action', style: TextStyle(color: ZiteraColors.textPrimary, fontSize: 12)),
                    onPressed: () => _copyToClipboard(comp.recommendation!, comp.name),
                  ),
                ],
              ),
            ),
          ],
        ],
      ),
    );
  }
}
