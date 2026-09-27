import 'package:flutter/material.dart';
import '../../core/ipc/engine_client.dart';
import '../../core/ipc/models.dart';
import '../../core/progress/progress_manager.dart';
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
  List<ToolItem> _tools = [];
  Map<String, dynamic> _progress = {};
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
      // Parallel batch query for high performance without persistent daemon overhead
      final results = await Future.wait([
        ZiteraEngineClient.runDoctor(),
        ZiteraEngineClient.getLabs(),
        ZiteraEngineClient.getTools(),
        ProgressManager.loadProgress(),
      ]);

      if (mounted) {
        setState(() {
          _diagnostics = results[0] as DiagnosticsResult;
          _labs = results[1] as List<LabItem>;
          _tools = results[2] as List<ToolItem>;
          _progress = results[3] as Map<String, dynamic>;
          _isLoading = false;
        });
      }
    } catch (e) {
      if (mounted) {
        setState(() {
          _error = e.toString();
          _isLoading = false;
        });
      }
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
    final readyTools = _tools.where((t) => t.installed).length;
    final solvedChallenges = List<String>.from(_progress['completed_challenges'] as List? ?? []);
    final completedLabs = List<String>.from(_progress['completed_labs'] as List? ?? []);

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
                    'Educational cybersecurity laboratory with real external git repositories & Docker runtimes',
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

          // 1. PRIORITY 1: "Is my environment ready?"
          ZiteraCard(
            borderColor: diag.allReady ? ZiteraColors.ready.withValues(alpha: 0.4) : ZiteraColors.warning.withValues(alpha: 0.4),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Row(
                  mainAxisAlignment: MainAxisAlignment.spaceBetween,
                  children: [
                    Row(
                      children: [
                        Icon(
                          diag.allReady ? Icons.check_circle : Icons.warning_amber,
                          color: diag.allReady ? ZiteraColors.ready : ZiteraColors.warning,
                          size: 28,
                        ),
                        const SizedBox(width: 12),
                        Column(
                          crossAxisAlignment: CrossAxisAlignment.start,
                          children: [
                            Text(
                              diag.allReady ? 'ENVIRONMENT READY FOR LEARNING' : 'SYSTEM SETUP ACTION REQUIRED',
                              style: const TextStyle(fontWeight: FontWeight.bold, fontSize: 15),
                            ),
                            Text(
                              'Windows: ${diag.os.status} • Git: ${diag.git.status} • WSL2: ${diag.wsl.status} • Docker: ${diag.docker.status} (${diag.dockerDaemon.status})',
                              style: const TextStyle(color: ZiteraColors.textSecondary, fontSize: 12),
                            ),
                          ],
                        ),
                      ],
                    ),
                    ZiteraButton(
                      label: 'Manage Environment',
                      icon: Icons.settings_suggest_outlined,
                      variant: ButtonVariant.secondary,
                      onPressed: () => widget.onNavigate(2),
                    ),
                  ],
                ),
              ],
            ),
          ),

          const SizedBox(height: 20),

          // High-level Metrics Row (Priority 3: "What's my progress?")
          Row(
            children: [
              Expanded(
                child: ZiteraCard(
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      const Text('CHALLENGES CONQUERED', style: TextStyle(color: ZiteraColors.textMuted, fontSize: 11, fontWeight: FontWeight.bold)),
                      const SizedBox(height: 10),
                      Row(
                        mainAxisAlignment: MainAxisAlignment.spaceBetween,
                        children: [
                          Text('${solvedChallenges.length} of ${_labs.length} Solved', style: const TextStyle(color: ZiteraColors.textPrimary, fontSize: 18, fontWeight: FontWeight.bold)),
                          const Icon(Icons.workspace_premium, color: ZiteraColors.ready),
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
                      const Text('LABS COMPLETED', style: TextStyle(color: ZiteraColors.textMuted, fontSize: 11, fontWeight: FontWeight.bold)),
                      const SizedBox(height: 10),
                      Row(
                        mainAxisAlignment: MainAxisAlignment.spaceBetween,
                        children: [
                          Text('${completedLabs.length} of ${_labs.length} Done', style: const TextStyle(color: ZiteraColors.textPrimary, fontSize: 18, fontWeight: FontWeight.bold)),
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
              const SizedBox(width: 16),
              Expanded(
                child: ZiteraCard(
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      const Text('SECURITY TOOLS', style: TextStyle(color: ZiteraColors.textMuted, fontSize: 11, fontWeight: FontWeight.bold)),
                      const SizedBox(height: 10),
                      Row(
                        mainAxisAlignment: MainAxisAlignment.spaceBetween,
                        children: [
                          Text('$readyTools of ${_tools.length} Ready', style: const TextStyle(color: ZiteraColors.textPrimary, fontSize: 18, fontWeight: FontWeight.bold)),
                          const Icon(Icons.terminal, color: ZiteraColors.primary),
                        ],
                      ),
                    ],
                  ),
                ),
              ),
            ],
          ),

          const SizedBox(height: 32),

          // 2. PRIORITY 2: "What can I learn?"
          Row(
            mainAxisAlignment: MainAxisAlignment.spaceBetween,
            children: [
              const Text(
                'AVAILABLE REFERENCE LABORATORIES',
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

          if (_labs.isEmpty)
            const ZiteraCard(
              child: Text(
                'No laboratories discovered yet. Check catalog or run setup.',
                style: TextStyle(color: ZiteraColors.textSecondary),
              ),
            )
          else
            Wrap(
              spacing: 16,
              runSpacing: 16,
              children: _labs.map((lab) {
                final isSolved = solvedChallenges.contains(lab.id);
                final hasMissingTools = lab.challengeReadiness == 'PARTIAL';

                return SizedBox(
                  width: 380,
                  child: ZiteraCard(
                    borderColor: isSolved ? ZiteraColors.ready.withValues(alpha: 0.4) : ZiteraColors.border,
                    child: Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        Row(
                          mainAxisAlignment: MainAxisAlignment.spaceBetween,
                          children: [
                            StatusBadge(status: lab.status),
                            if (isSolved)
                              const StatusBadge(status: 'SOLVED')
                            else if (lab.port > 0)
                              StatusBadge(status: 'PORT ${lab.port}'),
                          ],
                        ),
                        const SizedBox(height: 12),
                        Text(
                          lab.title,
                          style: const TextStyle(
                            color: ZiteraColors.textPrimary,
                            fontSize: 17,
                            fontWeight: FontWeight.bold,
                          ),
                        ),
                        const SizedBox(height: 6),
                        Text(
                          'ID: ${lab.id} • Version: v${lab.version}',
                          style: const TextStyle(color: ZiteraColors.textSecondary, fontSize: 12),
                        ),
                        const SizedBox(height: 12),

                        // Phase 6K: Lab Readiness Card
                        Container(
                          padding: const EdgeInsets.all(10),
                          decoration: BoxDecoration(
                            color: ZiteraColors.surface,
                            borderRadius: BorderRadius.circular(4),
                            border: Border.all(color: ZiteraColors.border),
                          ),
                          child: Column(
                            crossAxisAlignment: CrossAxisAlignment.start,
                            children: [
                              Row(
                                mainAxisAlignment: MainAxisAlignment.spaceBetween,
                                children: [
                                  _readinessPill('Learn', lab.learnReadiness),
                                  _readinessPill('Practice', lab.practiceReadiness),
                                  _readinessPill('Challenge', lab.challengeReadiness),
                                ],
                              ),
                              if (hasMissingTools && lab.recommendedTools.isNotEmpty) ...[
                                const SizedBox(height: 6),
                                Text(
                                  'Recommended Tool: ${lab.recommendedTools.join(", ")}',
                                  style: const TextStyle(color: ZiteraColors.warning, fontSize: 11),
                                ),
                              ],
                            ],
                          ),
                        ),

                        const SizedBox(height: 16),
                        ZiteraButton(
                          label: 'Enter Laboratory',
                          icon: Icons.login,
                          onPressed: () => widget.onOpenLab(lab.id),
                        ),
                      ],
                    ),
                  ),
                );
              }).toList(),
            ),

          const SizedBox(height: 32),

          // 4. PRIORITY 4: "Which tools are missing?"
          Row(
            mainAxisAlignment: MainAxisAlignment.spaceBetween,
            children: [
              const Text(
                'SECURITY TOOLING READINESS',
                style: TextStyle(
                  color: ZiteraColors.textPrimary,
                  fontSize: 14,
                  fontWeight: FontWeight.bold,
                  letterSpacing: 1.0,
                  fontFamily: 'monospace',
                ),
              ),
              TextButton(
                onPressed: () => widget.onNavigate(3),
                child: const Text('Open Tools Manager →', style: TextStyle(color: ZiteraColors.cyan)),
              ),
            ],
          ),
          const SizedBox(height: 16),

          ZiteraCard(
            child: Column(
              children: _tools.take(4).map((tool) {
                return Column(
                  children: [
                    Padding(
                      padding: const EdgeInsets.symmetric(vertical: 8.0),
                      child: Row(
                        mainAxisAlignment: MainAxisAlignment.spaceBetween,
                        children: [
                          Row(
                            children: [
                              Icon(
                                tool.installed ? Icons.check_circle_outline : Icons.radio_button_unchecked,
                                color: tool.installed ? ZiteraColors.ready : ZiteraColors.textMuted,
                                size: 18,
                              ),
                              const SizedBox(width: 10),
                              Text(tool.name, style: const TextStyle(fontWeight: FontWeight.bold, fontSize: 14)),
                              const SizedBox(width: 8),
                              Text('// ${tool.category}', style: const TextStyle(color: ZiteraColors.textMuted, fontSize: 12)),
                            ],
                          ),
                          StatusBadge(status: tool.status),
                        ],
                      ),
                    ),
                    if (tool != _tools.take(4).last)
                      const Divider(color: ZiteraColors.border),
                  ],
                );
              }).toList(),
            ),
          ),
        ],
      ),
    );
  }

  Widget _readinessPill(String label, String status) {
    Color color;
    switch (status) {
      case 'READY':
        color = ZiteraColors.ready;
        break;
      case 'PARTIAL':
        color = ZiteraColors.warning;
        break;
      default:
        color = ZiteraColors.error;
    }

    return Row(
      children: [
        Container(
          width: 7,
          height: 7,
          decoration: BoxDecoration(color: color, shape: BoxShape.circle),
        ),
        const SizedBox(width: 5),
        Text(
          '$label: $status',
          style: TextStyle(color: color, fontSize: 11, fontWeight: FontWeight.bold),
        ),
      ],
    );
  }
}
