import 'package:flutter/material.dart';
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
  bool _isLoading = true;

  @override
  void initState() {
    super.initState();
    _runCheck();
  }

  Future<void> _runCheck() async {
    setState(() => _isLoading = true);
    try {
      final diag = await ZiteraEngineClient.runDoctor();
      setState(() {
        _diagnostics = diag;
        _isLoading = false;
      });
    } catch (_) {
      setState(() => _isLoading = false);
    }
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
              const Text('Unable to query system diagnostics.', style: TextStyle(color: ZiteraColors.error)),
              const SizedBox(height: 12),
              ZiteraButton(label: 'Retry Check', icon: Icons.refresh, onPressed: _runCheck),
            ],
          ),
        ),
      );
    }

    final diag = _diagnostics!;

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
                    'SMART SETUP & ENVIRONMENT DIAGNOSTICS',
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
                    'Detect, classify, explain, recommend, and validate system prerequisites',
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

          ZiteraCard(
            borderColor: diag.allReady ? ZiteraColors.ready.withValues(alpha: 0.4) : ZiteraColors.warning.withValues(alpha: 0.4),
            child: Row(
              children: [
                Icon(
                  diag.allReady ? Icons.check_circle : Icons.warning_amber,
                  color: diag.allReady ? ZiteraColors.ready : ZiteraColors.warning,
                  size: 32,
                ),
                const SizedBox(width: 16),
                Expanded(
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      Text(
                        diag.allReady ? 'ALL SYSTEM PREREQUISITES READY' : 'ACTION REQUIRED TO COMPLETE SETUP',
                        style: const TextStyle(fontWeight: FontWeight.bold, fontSize: 15),
                      ),
                      const SizedBox(height: 4),
                      Text(
                        diag.allReady
                            ? 'Your Windows machine is fully configured to host, start, and reset all isolated Docker labs.'
                            : 'Some components require setup or service startup. Review the recommendations below.',
                        style: const TextStyle(color: ZiteraColors.textSecondary, fontSize: 12),
                      ),
                    ],
                  ),
                ),
                StatusBadge(status: diag.allReady ? 'READY' : 'ACTION REQUIRED'),
              ],
            ),
          ),

          const SizedBox(height: 28),

          _buildComponentCard(diag.os),
          const SizedBox(height: 12),
          _buildComponentCard(diag.git),
          const SizedBox(height: 12),
          _buildComponentCard(diag.wsl),
          const SizedBox(height: 12),
          _buildComponentCard(diag.docker),
          const SizedBox(height: 12),
          _buildComponentCard(diag.dockerDaemon),
          const SizedBox(height: 12),
          _buildComponentCard(diag.powershell),
        ],
      ),
    );
  }

  Widget _buildComponentCard(ComponentItem comp) {
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
                    comp.installed ? Icons.check_circle_outline : Icons.error_outline,
                    color: comp.installed ? ZiteraColors.ready : ZiteraColors.warning,
                    size: 20,
                  ),
                  const SizedBox(width: 12),
                  Text(
                    comp.name,
                    style: const TextStyle(fontWeight: FontWeight.bold, fontSize: 15),
                  ),
                ],
              ),
              StatusBadge(status: comp.status),
            ],
          ),
          const SizedBox(height: 8),
          Text(comp.message, style: const TextStyle(color: ZiteraColors.textSecondary, fontSize: 13)),
          if (comp.recommendation != null) ...[
            const SizedBox(height: 10),
            Container(
              padding: const EdgeInsets.all(10),
              decoration: BoxDecoration(
                color: ZiteraColors.surface,
                borderRadius: BorderRadius.circular(4),
                border: Border.all(color: ZiteraColors.border),
              ),
              child: Row(
                children: [
                  const Icon(Icons.info_outline, size: 16, color: ZiteraColors.cyan),
                  const SizedBox(width: 8),
                  Expanded(
                    child: Text(
                      'Recommendation: ${comp.recommendation!}',
                      style: const TextStyle(color: ZiteraColors.cyan, fontSize: 12, fontFamily: 'monospace'),
                    ),
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
