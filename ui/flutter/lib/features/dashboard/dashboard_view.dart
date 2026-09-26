import 'package:flutter/material.dart';
import '../../core/ipc/engine_client.dart';
import '../../core/ipc/models.dart';
import '../../core/theme/zitera_colors.dart';
import '../../widgets/status_badge.dart';
import '../../widgets/zitera_button.dart';
import '../../widgets/zitera_card.dart';

class DashboardView extends StatefulWidget {
  final Function(int) onNavigate;
  final Function(String) onOpenLab;

  const DashboardView({
    super.key,
    required this.onNavigate,
    required this.onOpenLab,
  });

  @override
  State<DashboardView> createState() => _DashboardViewState();
}

class _DashboardViewState extends State<DashboardView> {
  DiagnosticsResult? _diagnostics;
  List<LabItem> _labs = [];
  bool _isLoading = true;
  String? _error;

  @override
  void initState() {
    super.initState();
    _loadDashboardData();
  }

  Future<void> _loadDashboardData() async {
    setState(() {
      _isLoading = true;
      _error = null;
    });

    try {
      final diag = await ZiteraEngineClient.runDoctor();
      final labs = await ZiteraEngineClient.getLabs();
      setState(() {
        _diagnostics = diag;
        _labs = labs;
        _isLoading = false;
      });
    } catch (e) {
      setState(() {
        _error = e.toString();
        _isLoading = false;
      });
    }
  }

  @override
  Widget build(BuildContext context) {
    if (_isLoading) {
      return const Center(
        child: CircularProgressIndicator(color: ZiteraColors.primary),
      );
    }

    if (_error != null) {
      return Center(
        child: ZiteraCard(
          borderColor: ZiteraColors.error,
          child: Column(
            mainAxisSize: MainAxisSize.min,
            children: [
              const Icon(Icons.error_outline, color: ZiteraColors.error, size: 36),
              const SizedBox(height: 12),
              const Text(
                'Engine Connection Error',
                style: TextStyle(fontWeight: FontWeight.bold, fontSize: 16),
              ),
              const SizedBox(height: 8),
              Text(_error!, style: const TextStyle(color: ZiteraColors.textSecondary, fontSize: 12)),
              const SizedBox(height: 16),
              ZiteraButton(
                label: 'Retry Connection',
                icon: Icons.refresh,
                onPressed: _loadDashboardData,
              ),
            ],
          ),
        ),
      );
    }

    final diag = _diagnostics!;
    final runningLabs = _labs.where((l) => l.running).length;

    return SingleChildScrollView(
      padding: const EdgeInsets.all(32.0),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          // Header
          Row(
            mainAxisAlignment: MainAxisAlignment.spaceBetween,
            children: [
              Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: const [
                  Text(
                    'SECURITY COMMAND DASHBOARD',
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
                    'Local isolated cyber security laboratory environment',
                    style: TextStyle(color: ZiteraColors.textSecondary, fontSize: 13),
                  ),
                ],
              ),
              ZiteraButton(
                label: 'Refresh Status',
                icon: Icons.refresh,
                variant: ButtonVariant.secondary,
                onPressed: _loadDashboardData,
              ),
            ],
          ),

          const SizedBox(height: 28),

          // Top Stats Cards
          Row(
            children: [
              Expanded(
                child: ZiteraCard(
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      const Text('PLATFORM READINESS', style: TextStyle(color: ZiteraColors.textMuted, fontSize: 11, fontWeight: FontWeight.bold)),
                      const SizedBox(height: 10),
                      Row(
                        mainAxisAlignment: MainAxisAlignment.spaceBetween,
                        children: [
                          StatusBadge(status: diag.allReady ? 'READY' : 'ATTENTION REQUIRED'),
                          Icon(
                            diag.allReady ? Icons.check_circle_outline : Icons.warning_amber_outlined,
                            color: diag.allReady ? ZiteraColors.ready : ZiteraColors.warning,
                          ),
                        ],
                      ),
                    ],
                  ),
                ),
              ),
              const SizedBox(width: 16),
              Expanded(
                child: ZiteraCard(
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      const Text('INSTALLED LABS', style: TextStyle(color: ZiteraColors.textMuted, fontSize: 11, fontWeight: FontWeight.bold)),
                      const SizedBox(height: 10),
                      Row(
                        mainAxisAlignment: MainAxisAlignment.spaceBetween,
                        children: [
                          Text('${_labs.length} Available', style: const TextStyle(color: ZiteraColors.textPrimary, fontSize: 18, fontWeight: FontWeight.bold)),
                          const Icon(Icons.science_outlined, color: ZiteraColors.cyan),
                        ],
                      ),
                    ],
                  ),
                ),
              ),
              const SizedBox(width: 16),
              Expanded(
                child: ZiteraCard(
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      const Text('ACTIVE RUNTIMES', style: TextStyle(color: ZiteraColors.textMuted, fontSize: 11, fontWeight: FontWeight.bold)),
                      const SizedBox(height: 10),
                      Row(
                        mainAxisAlignment: MainAxisAlignment.spaceBetween,
                        children: [
                          Text('$runningLabs Running', style: TextStyle(color: runningLabs > 0 ? ZiteraColors.ready : ZiteraColors.textSecondary, fontSize: 18, fontWeight: FontWeight.bold)),
                          Icon(Icons.power_settings_new, color: runningLabs > 0 ? ZiteraColors.ready : ZiteraColors.missing),
                        ],
                      ),
                    ],
                  ),
                ),
              ),
            ],
          ),

          const SizedBox(height: 32),

          // Reference Labs section
          Row(
            mainAxisAlignment: MainAxisAlignment.spaceBetween,
            children: [
              const Text(
                'FEATURED REFERENCE LABS (OWASP TOP 10:2025)',
                style: TextStyle(
                  color: ZiteraColors.textPrimary,
                  fontSize: 14,
                  fontWeight: FontWeight.bold,
                  letterSpacing: 1.0,
                  fontFamily: 'monospace',
                ),
              ),
              TextButton(
                onPressed: () => widget.onNavigate(1),
                child: const Text('View All Labs →', style: TextStyle(color: ZiteraColors.cyan)),
              ),
            ],
          ),
          const SizedBox(height: 16),

          Row(
            children: [
              Expanded(
                child: ZiteraCard(
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      Row(
                        mainAxisAlignment: MainAxisAlignment.spaceBetween,
                        children: const [
                          StatusBadge(status: 'A01:2025'),
                          StatusBadge(status: 'PORT 8011'),
                        ],
                      ),
                      const SizedBox(height: 12),
                      const Text(
                        'Broken Access Control',
                        style: TextStyle(color: ZiteraColors.textPrimary, fontSize: 17, fontWeight: FontWeight.bold),
                      ),
                      const SizedBox(height: 6),
                      const Text(
                        'Exploit horizontal IDOR parameter flaws to recover confidential financial accounts.',
                        style: TextStyle(color: ZiteraColors.textSecondary, fontSize: 12),
                      ),
                      const SizedBox(height: 16),
                      ZiteraButton(
                        label: 'Enter Laboratory',
                        icon: Icons.login,
                        onPressed: () => widget.onOpenLab('A01'),
                      ),
                    ],
                  ),
                ),
              ),
              const SizedBox(width: 16),
              Expanded(
                child: ZiteraCard(
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      Row(
                        mainAxisAlignment: MainAxisAlignment.spaceBetween,
                        children: const [
                          StatusBadge(status: 'A05:2025'),
                          StatusBadge(status: 'PORT 8015'),
                        ],
                      ),
                      const SizedBox(height: 12),
                      const Text(
                        'Injection (SQLi)',
                        style: TextStyle(color: ZiteraColors.textPrimary, fontSize: 17, fontWeight: FontWeight.bold),
                      ),
                      const SizedBox(height: 6),
                      const Text(
                        'Bypass query boundaries with UNION injection to extract hidden database vault flags.',
                        style: TextStyle(color: ZiteraColors.textSecondary, fontSize: 12),
                      ),
                      const SizedBox(height: 16),
                      ZiteraButton(
                        label: 'Enter Laboratory',
                        icon: Icons.login,
                        onPressed: () => widget.onOpenLab('A05'),
                      ),
                    ],
                  ),
                ),
              ),
            ],
          ),

          const SizedBox(height: 32),

          // Environment Diagnostics preview
          const Text(
            'SYSTEM ENVIRONMENT STATUS',
            style: TextStyle(
              color: ZiteraColors.textPrimary,
              fontSize: 14,
              fontWeight: FontWeight.bold,
              letterSpacing: 1.0,
              fontFamily: 'monospace',
            ),
          ),
          const SizedBox(height: 16),

          ZiteraCard(
            child: Column(
              children: [
                _diagnosticRow('Git Version Control', diag.git.installed, diag.git.version ?? 'Not Found', diag.git.status),
                const Divider(color: ZiteraColors.border),
                _diagnosticRow('WSL2 Linux Environment', diag.wsl.installed, diag.wsl.message, diag.wsl.status),
                const Divider(color: ZiteraColors.border),
                _diagnosticRow('Docker CLI', diag.docker.installed, diag.docker.version ?? 'Not Found', diag.docker.status),
                const Divider(color: ZiteraColors.border),
                _diagnosticRow('Docker Daemon Engine', diag.dockerDaemon.installed, diag.dockerDaemon.message, diag.dockerDaemon.status),
              ],
            ),
          ),
        ],
      ),
    );
  }

  Widget _diagnosticRow(String title, bool ok, String details, String status) {
    return Padding(
      padding: const EdgeInsets.symmetric(vertical: 8.0),
      child: Row(
        mainAxisAlignment: MainAxisAlignment.spaceBetween,
        children: [
          Row(
            children: [
              Icon(ok ? Icons.check_circle_outline : Icons.error_outline, color: ok ? ZiteraColors.ready : ZiteraColors.warning, size: 18),
              const SizedBox(width: 12),
              Text(title, style: const TextStyle(fontWeight: FontWeight.w600, fontSize: 13)),
            ],
          ),
          Row(
            children: [
              Text(details, style: const TextStyle(color: ZiteraColors.textMuted, fontSize: 12)),
              const SizedBox(width: 16),
              StatusBadge(status: status),
            ],
          ),
        ],
      ),
    );
  }
}
