import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import '../../core/ipc/engine_client.dart';
import '../../core/ipc/models.dart';
import '../../core/theme/zitera_colors.dart';
import '../../widgets/status_badge.dart';
import '../../widgets/zitera_button.dart';
import '../../widgets/zitera_card.dart';
import '../../widgets/cute_anime_loading.dart';
import '../../widgets/hacker_tilix_entrance.dart';

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
      return const Center(
        child: CuteAnimeLoading(
          message: 'AUDITING SYSTEM PREREQUISITES...',
          subMessage: '( •̀ ω •́ )✧ CHECKING DOCKER & SUBSYSTEMS',
        ),
      );
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

    return Stack(
      children: [
        Positioned.fill(
          child: Image.asset(
            'assets/images/bg_environment.jpg',
            fit: BoxFit.cover,
          ),
        ),
        Positioned.fill(
          child: Container(
            color: const Color(0xFFFCFBF8).withValues(alpha: 0.91),
          ),
        ),
        SingleChildScrollView(
          padding: const EdgeInsets.all(32.0),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              HackerTilixEntrance(
                delay: Duration.zero,
                direction: TilixSlideDirection.down,
                child: Row(
                  mainAxisAlignment: MainAxisAlignment.spaceBetween,
                  children: [
                    Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: const [
                        Text(
                          'SMART ENVIRONMENT SETUP V2',
                          style: TextStyle(
                            color: Color(0xFF1E1A14),
                            fontSize: 22,
                            fontWeight: FontWeight.w900,
                            letterSpacing: 1.2,
                            fontFamily: 'SpaceGrotesk',
                          ),
                        ),
                        SizedBox(height: 4),
                        Text(
                          'Detect, classify, explain, recommend, and validate system prerequisites & tools',
                          style: TextStyle(
                            color: Color(0xFF5C5347),
                            fontSize: 13,
                            fontFamily: 'JetBrainsMono',
                          ),
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
              ),

              const SizedBox(height: 24),

              // Overall Readiness Banner
              HackerTilixEntrance(
                delay: const Duration(milliseconds: 70),
                direction: TilixSlideDirection.up,
                child: ZiteraCard(
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
                              style: const TextStyle(
                                fontFamily: 'SpaceGrotesk',
                                fontWeight: FontWeight.bold,
                                fontSize: 15,
                                color: Color(0xFF1E1A14),
                              ),
                            ),
                            const SizedBox(height: 4),
                            Text(
                              diag.allReady
                                  ? 'Host environment is fully configured. All containerized external labs can be launched locally.'
                                  : 'One or more system components require attention. Review guided action steps below.',
                              style: const TextStyle(
                                fontFamily: 'JetBrainsMono',
                                color: Color(0xFF5C5347),
                                fontSize: 12,
                              ),
                            ),
                          ],
                        ),
                      ),
                      Row(
                        children: [
                          SizedBox(
                            width: 34,
                            height: 34,
                            child: Image.asset(
                              'assets/images/sticker_bunny.png',
                              fit: BoxFit.contain,
                              errorBuilder: (context, error, stackTrace) => const SizedBox(),
                            ),
                          ),
                          const SizedBox(width: 10),
                          StatusBadge(status: diag.allReady ? 'READY' : 'ACTION REQUIRED'),
                        ],
                      ),
                    ],
                  ),
                ),
              ),

              const SizedBox(height: 20),

              // Host Hardware Resources Card
              HackerTilixEntrance(
                delay: const Duration(milliseconds: 140),
                direction: TilixSlideDirection.up,
                child: Row(
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
                                const Text('HOST MEMORY (RAM)', style: TextStyle(color: Color(0xFF5C5347), fontSize: 11, fontWeight: FontWeight.bold, fontFamily: 'SpaceGrotesk')),
                                const SizedBox(height: 4),
                                Text('${diag.memoryGb.toStringAsFixed(1)} GB Total', style: const TextStyle(color: Color(0xFF1E1A14), fontWeight: FontWeight.bold, fontSize: 16, fontFamily: 'SpaceGrotesk')),
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
                                const Text('SYSTEM DRIVE FREE DISK', style: TextStyle(color: Color(0xFF5C5347), fontSize: 11, fontWeight: FontWeight.bold, fontFamily: 'SpaceGrotesk')),
                                const SizedBox(height: 4),
                                Text('${diag.diskFreeGb.toStringAsFixed(1)} GB Free', style: const TextStyle(color: Color(0xFF1E1A14), fontWeight: FontWeight.bold, fontSize: 16, fontFamily: 'SpaceGrotesk')),
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
                            Expanded(
                              child: Column(
                                crossAxisAlignment: CrossAxisAlignment.start,
                                children: [
                                  const Text('SECURITY TOOLS STATUS', style: TextStyle(color: Color(0xFF5C5347), fontSize: 11, fontWeight: FontWeight.bold, fontFamily: 'SpaceGrotesk')),
                                  const SizedBox(height: 4),
                                  Text('$readyTools of ${_tools.length} Tools Ready', style: const TextStyle(color: Color(0xFF1E1A14), fontWeight: FontWeight.bold, fontSize: 16, fontFamily: 'SpaceGrotesk')),
                                ],
                              ),
                            ),
                            SizedBox(
                              width: 32,
                              height: 32,
                              child: Image.asset(
                                'assets/images/sticker_cat_laptop.png',
                                fit: BoxFit.contain,
                                errorBuilder: (context, error, stackTrace) => const SizedBox(),
                              ),
                            ),
                          ],
                        ),
                      ),
                    ),
                  ],
                ),
              ),

              const SizedBox(height: 28),

              // Core System Prerequisites Section
              HackerTilixEntrance(
                delay: const Duration(milliseconds: 200),
                direction: TilixSlideDirection.left,
                child: const Text(
                  'CORE SYSTEM PREREQUISITES',
                  style: TextStyle(
                    color: Color(0xFF1E1A14),
                    fontSize: 14,
                    fontWeight: FontWeight.bold,
                    letterSpacing: 1.0,
                    fontFamily: 'SpaceGrotesk',
                  ),
                ),
              ),
              const SizedBox(height: 12),

              _buildComponentCard(diag.os, delay: const Duration(milliseconds: 240)),
              const SizedBox(height: 10),
              _buildComponentCard(diag.git, delay: const Duration(milliseconds: 280)),
              const SizedBox(height: 10),
              _buildComponentCard(diag.wsl, delay: const Duration(milliseconds: 320)),
              const SizedBox(height: 10),
              _buildComponentCard(diag.docker, delay: const Duration(milliseconds: 360)),
              const SizedBox(height: 10),
              _buildComponentCard(diag.dockerDaemon, delay: const Duration(milliseconds: 400)),
              const SizedBox(height: 10),
              _buildComponentCard(diag.powershell, delay: const Duration(milliseconds: 440)),
            ],
          ),
        ),
      ],
    );
  }

  Widget _buildComponentCard(ComponentItem comp, {Duration delay = Duration.zero}) {
    final isReady = comp.status == 'READY';
    final isBlocked = comp.status == 'BLOCKED';

    return HackerTilixEntrance(
      delay: delay,
      direction: TilixSlideDirection.up,
      child: ZiteraCard(
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
    ),
    );
  }
}
