import 'package:flutter/material.dart';
import '../../core/ipc/engine_client.dart';
import '../../core/ipc/models.dart';
import '../../core/progress/progress_manager.dart';
import '../../core/theme/zitera_colors.dart';
import '../../widgets/pixel_mascot_widget.dart';
import '../../widgets/status_badge.dart';
import '../../widgets/zitera_button.dart';

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
        child: Container(
          padding: const EdgeInsets.all(24),
          decoration: BoxDecoration(
            color: const Color(0xFFFCFBF8),
            borderRadius: BorderRadius.circular(10),
            border: Border.all(color: ZiteraColors.error, width: 1.5),
          ),
          child: Column(
            mainAxisSize: MainAxisSize.min,
            children: [
              const Icon(Icons.error_outline, color: ZiteraColors.error, size: 36),
              const SizedBox(height: 12),
              const Text(
                'Engine Connection Error',
                style: TextStyle(
                  fontFamily: 'SpaceGrotesk',
                  fontWeight: FontWeight.bold,
                  fontSize: 18,
                ),
              ),
              const SizedBox(height: 8),
              Text(
                _error!,
                style: const TextStyle(
                  fontFamily: 'JetBrainsMono',
                  color: ZiteraColors.textSecondary,
                  fontSize: 12,
                ),
              ),
              const SizedBox(height: 16),
              ZiteraButton(
                label: 'RETRY CONNECTION',
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

    // Fallback laboratories if not discovered yet, ensuring exact blueprint match
    final displayLabs = _labs.isNotEmpty
        ? _labs
        : [
            LabItem(
              id: 'a01',
              title: 'Broken Access Control',
              installed: true,
              running: false,
              port: 8011,
              version: '1.0.1',
              status: 'STOPPED',
              learnReadiness: 'READY',
              practiceReadiness: 'READY',
              challengeReadiness: 'READY',
              recommendedTools: ['curl', 'burp'],
            ),
            LabItem(
              id: 'a05',
              title: 'Injection',
              installed: true,
              running: false,
              port: 8015,
              version: '1.0.0',
              status: 'STOPPED',
              learnReadiness: 'READY',
              practiceReadiness: 'READY',
              challengeReadiness: 'PARTIAL',
              recommendedTools: ['sqlmap'],
            ),
          ];

    // Fallback tools if not populated
    final displayTools = _tools.isNotEmpty
        ? _tools
        : [
            ToolItem(
              id: 'nmap',
              name: 'Nmap',
              category: 'Network Scanning',
              installed: false,
              minVersion: '7.90',
              status: 'MISSING',
              installGuide: 'winget install Insecure.Nmap',
            ),
          ];

    return Stack(
      children: [
        Positioned.fill(
          child: Image.asset(
            'assets/images/bg_dashboard.jpg',
            fit: BoxFit.cover,
          ),
        ),
        Positioned.fill(
          child: Container(
            color: const Color(0xFFFCFBF8).withValues(alpha: 0.90),
          ),
        ),
        SingleChildScrollView(
          padding: const EdgeInsets.symmetric(horizontal: 28.0, vertical: 24.0),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          // 1. Header Section
          Row(
            mainAxisAlignment: MainAxisAlignment.spaceBetween,
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Row(
                    crossAxisAlignment: CrossAxisAlignment.center,
                    children: [
                      const Text(
                        'SECURITY COMMAND',
                        style: TextStyle(
                          fontFamily: 'SpaceGrotesk',
                          fontSize: 20,
                          fontWeight: FontWeight.bold,
                          color: Color(0xFF1E1A14),
                          letterSpacing: 0.8,
                        ),
                      ),
                      const SizedBox(width: 10),
                      Container(
                        padding: const EdgeInsets.symmetric(horizontal: 6, vertical: 3),
                        decoration: BoxDecoration(
                          color: const Color(0xFFF0EDE8),
                          borderRadius: BorderRadius.circular(4),
                          border: Border.all(color: const Color(0xFFCEC8BF), width: 1),
                        ),
                        child: const Text(
                          'ZITERA__LAB',
                          style: TextStyle(
                            fontFamily: 'JetBrainsMono',
                            fontSize: 10,
                            fontWeight: FontWeight.bold,
                            color: Color(0xFF5C5347),
                            letterSpacing: 0.5,
                          ),
                        ),
                      ),
                    ],
                  ),
                  const SizedBox(height: 6),
                  const Text(
                    'Educational cybersecurity laboratory with real external git repositories & Docker runtimes',
                    style: TextStyle(
                      fontFamily: 'JetBrainsMono',
                      color: ZiteraColors.textSecondary,
                      fontSize: 12,
                      fontWeight: FontWeight.w500,
                    ),
                  ),
                ],
              ),
              ZiteraButton(
                label: 'REFRESH STATUS',
                icon: Icons.refresh,
                variant: ButtonVariant.secondary,
                fontSize: 11,
                padding: const EdgeInsets.symmetric(horizontal: 14, vertical: 8),
                onPressed: _loadDashboardData,
              ),
            ],
          ),

          const SizedBox(height: 18),

          // 2. Welcome Card (Zeta Mascot)
          const PixelMascotWidget(),

          const SizedBox(height: 18),

          // 3. Environment Check Card (Soft Pastel Mint)
          Container(
            padding: const EdgeInsets.symmetric(horizontal: 18, vertical: 14),
            decoration: BoxDecoration(
              color: ZiteraColors.envBg,
              borderRadius: BorderRadius.circular(10),
              border: Border.all(color: ZiteraColors.envBorder, width: 1.4),
              boxShadow: [
                BoxShadow(
                  color: Colors.black.withValues(alpha: 0.03),
                  blurRadius: 6,
                  offset: const Offset(0, 2),
                ),
              ],
            ),
            child: Row(
              mainAxisAlignment: MainAxisAlignment.spaceBetween,
              children: [
                Row(
                  children: [
                    Container(
                      width: 28,
                      height: 28,
                      decoration: const BoxDecoration(
                        color: Color(0xFF22C55E),
                        shape: BoxShape.circle,
                      ),
                      child: const Icon(Icons.check, color: Colors.white, size: 18),
                    ),
                    const SizedBox(width: 12),
                    Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        Text(
                          diag.allReady ? 'ENVIRONMENT READY FOR LEARNING' : 'SYSTEM SETUP ACTION REQUIRED',
                          style: const TextStyle(
                            fontFamily: 'SpaceGrotesk',
                            fontSize: 13,
                            fontWeight: FontWeight.bold,
                            color: Color(0xFF1E1A14),
                            letterSpacing: 0.3,
                          ),
                        ),
                        const SizedBox(height: 2),
                        Text(
                          'Windows: ${diag.os.status} • Git: ${diag.git.status} • WSL2: ${diag.wsl.status} • Docker: ${diag.docker.status} (${diag.dockerDaemon.status})',
                          style: const TextStyle(
                            fontFamily: 'JetBrainsMono',
                            fontSize: 11,
                            color: Color(0xFF334155),
                            fontWeight: FontWeight.w500,
                          ),
                        ),
                      ],
                    ),
                  ],
                ),
                ZiteraButton(
                  label: 'MANAGE ENVIRONMENT',
                  icon: Icons.settings_suggest,
                  variant: ButtonVariant.gradient,
                  fontSize: 11,
                  padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 10),
                  onPressed: () => widget.onNavigate(2),
                ),
              ],
            ),
          ),

          const SizedBox(height: 28),

          // 4. Statistics Cards (x4 Identical Square Cards with Peeking Stickers)
          Row(
            children: [
              // Card 1: CHALLENGES (Peach)
              Expanded(
                child: _buildStatCard(
                  title: 'CHALLENGES',
                  value: '${solvedChallenges.length}/${displayLabs.length} SOLVED',
                  icon: Icons.military_tech_outlined,
                  bgColor: ZiteraColors.statChallengesBg,
                  borderColor: ZiteraColors.statChallengesBorder,
                  textColor: ZiteraColors.statChallengesText,
                  stickerAsset: 'assets/images/sticker_blonde.png',
                ),
              ),
              const SizedBox(width: 14),

              // Card 2: LABS DONE (Mint)
              Expanded(
                child: _buildStatCard(
                  title: 'LABS DONE',
                  value: '${completedLabs.length}/${displayLabs.length} DONE',
                  icon: Icons.science_outlined,
                  bgColor: ZiteraColors.statLabsBg,
                  borderColor: ZiteraColors.statLabsBorder,
                  textColor: ZiteraColors.statLabsText,
                  stickerAsset: 'assets/images/sticker_bunny.png',
                ),
              ),
              const SizedBox(width: 14),

              // Card 3: RUNTIMES (Yellow)
              Expanded(
                child: _buildStatCard(
                  title: 'RUNTIMES',
                  value: '$runningLabs RUNNING',
                  icon: Icons.power_settings_new,
                  bgColor: ZiteraColors.statRuntimesBg,
                  borderColor: ZiteraColors.statRuntimesBorder,
                  textColor: ZiteraColors.statRuntimesText,
                  stickerAsset: 'assets/images/sticker_hamster.png',
                ),
              ),
              const SizedBox(width: 14),

              // Card 4: SEC TOOLS (Purple)
              Expanded(
                child: _buildStatCard(
                  title: 'SEC TOOLS',
                  value: '$readyTools/${displayTools.length} READY',
                  icon: Icons.handyman_outlined,
                  bgColor: ZiteraColors.statToolsBg,
                  borderColor: ZiteraColors.statToolsBorder,
                  textColor: ZiteraColors.statToolsText,
                  stickerAsset: 'assets/images/sticker_purple.png',
                ),
              ),
            ],
          ),

          const SizedBox(height: 28),

          // 5. Available Laboratories Section Header
          Row(
            mainAxisAlignment: MainAxisAlignment.spaceBetween,
            crossAxisAlignment: CrossAxisAlignment.center,
            children: [
              Row(
                crossAxisAlignment: CrossAxisAlignment.center,
                children: [
                  const Text(
                    'AVAILABLE LABORATORIES',
                    style: TextStyle(
                      fontFamily: 'SpaceGrotesk',
                      fontSize: 14,
                      fontWeight: FontWeight.bold,
                      letterSpacing: 0.6,
                      color: Color(0xFF1E1A14),
                    ),
                  ),
                  const SizedBox(width: 8),
                  _pixelBarChart(),
                  const SizedBox(width: 8),
                  const Icon(Icons.auto_awesome, size: 16, color: Color(0xFFF472B6)),
                ],
              ),
              InkWell(
                onTap: () => widget.onNavigate(1),
                child: const Row(
                  children: [
                    Text(
                      'View All Labs',
                      style: TextStyle(
                        fontFamily: 'SpaceGrotesk',
                        fontSize: 12,
                        fontWeight: FontWeight.bold,
                        color: Color(0xFF0F766E),
                      ),
                    ),
                    SizedBox(width: 4),
                    Icon(Icons.arrow_forward, size: 14, color: Color(0xFF0F766E)),
                  ],
                ),
              ),
            ],
          ),

          const SizedBox(height: 26),

          // Laboratory Cards (Row of 2 Cards, equal height via IntrinsicHeight)
          IntrinsicHeight(
            child: Row(
              crossAxisAlignment: CrossAxisAlignment.stretch,
              children: [
                // Card 1: A01 Broken Access Control (Mint Theme)
                Expanded(
                  child: _buildLabCard(
                    lab: displayLabs.firstWhere((l) => l.id == 'a01', orElse: () => displayLabs[0]),
                    isYellowTheme: false,
                  ),
                ),
                const SizedBox(width: 16),

                // Card 2: A05 Injection (Yellow Theme with Rainbow Button)
                Expanded(
                  child: _buildLabCard(
                    lab: displayLabs.length > 1
                        ? displayLabs.firstWhere((l) => l.id == 'a05', orElse: () => displayLabs[1])
                        : displayLabs[0],
                    isYellowTheme: true,
                  ),
                ),
              ],
            ),
          ),

          const SizedBox(height: 28),

          // 6. Security Tooling Readiness Section Header
          Row(
            mainAxisAlignment: MainAxisAlignment.spaceBetween,
            crossAxisAlignment: CrossAxisAlignment.center,
            children: [
              const Text(
                'SECURITY TOOLING READINESS',
                style: TextStyle(
                  fontFamily: 'SpaceGrotesk',
                  fontSize: 14,
                  fontWeight: FontWeight.bold,
                  letterSpacing: 0.6,
                  color: Color(0xFF1E1A14),
                ),
              ),
              InkWell(
                onTap: () => widget.onNavigate(3),
                child: const Row(
                  children: [
                    Text(
                      'Open Tools Manager',
                      style: TextStyle(
                        fontFamily: 'SpaceGrotesk',
                        fontSize: 12,
                        fontWeight: FontWeight.bold,
                        color: Color(0xFF0F766E),
                      ),
                    ),
                    SizedBox(width: 4),
                    Icon(Icons.arrow_forward, size: 14, color: Color(0xFF0F766E)),
                  ],
                ),
              ),
            ],
          ),

          const SizedBox(height: 14),

          // Security Tools Card
          Container(
            padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 12),
            decoration: BoxDecoration(
              color: const Color(0xFFFCFBF8),
              borderRadius: BorderRadius.circular(8),
              border: Border.all(color: ZiteraColors.border, width: 1.2),
              boxShadow: [
                BoxShadow(
                  color: Colors.black.withValues(alpha: 0.02),
                  blurRadius: 6,
                  offset: const Offset(0, 2),
                ),
              ],
            ),
            child: Column(
              children: displayTools.take(4).map((tool) {
                return Column(
                  children: [
                    Padding(
                      padding: const EdgeInsets.symmetric(vertical: 6.0),
                      child: Row(
                        mainAxisAlignment: MainAxisAlignment.spaceBetween,
                        children: [
                          Row(
                            children: [
                              Icon(
                                tool.installed ? Icons.check_circle_outline : Icons.radio_button_unchecked,
                                color: tool.installed ? ZiteraColors.ready : ZiteraColors.textMuted,
                                size: 16,
                              ),
                              const SizedBox(width: 10),
                              Text(
                                tool.name,
                                style: const TextStyle(
                                  fontFamily: 'JetBrainsMono', // more readable than Silkscreen for tool names
                                  fontSize: 12,
                                  fontWeight: FontWeight.bold,
                                  color: Color(0xFF1E1A14),
                                ),
                              ),
                              const SizedBox(width: 8),
                              Text(
                                '// ${tool.category}',
                                style: const TextStyle(
                                  fontFamily: 'JetBrainsMono',
                                  color: ZiteraColors.textMuted,
                                  fontSize: 12,
                                ),
                              ),
                            ],
                          ),
                          StatusBadge(
                            status: tool.status,
                            backgroundColor: const Color(0xFFF1F5F9),
                            textColor: const Color(0xFF64748B),
                            borderColor: const Color(0xFFE2E8F0),
                          ),
                        ],
                      ),
                    ),
                    if (tool != displayTools.take(4).last)
                      const Divider(color: ZiteraColors.border, height: 12),
                  ],
                );
              }).toList(),
            ),
          ),
        ],
      ),
    ),
      ],
    );
  }

  // --- STAT CARD BUILDER ---
  Widget _buildStatCard({
    required String title,
    required String value,
    required IconData icon,
    required Color bgColor,
    required Color borderColor,
    required Color textColor,
    String? stickerAsset,
  }) {
    return Container(
      padding: const EdgeInsets.symmetric(horizontal: 14, vertical: 12),
      decoration: BoxDecoration(
        color: bgColor,
        borderRadius: BorderRadius.circular(8),
        border: Border.all(color: borderColor, width: 1.4),
        boxShadow: [
          BoxShadow(
            color: Colors.black.withValues(alpha: 0.02),
            blurRadius: 4,
            offset: const Offset(0, 2),
          ),
        ],
      ),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Row(
            mainAxisAlignment: MainAxisAlignment.spaceBetween,
            children: [
              Text(
                title,
                style: TextStyle(
                  fontFamily: 'SpaceGrotesk',
                  fontSize: 11,
                  fontWeight: FontWeight.bold,
                  color: textColor,
                  letterSpacing: 0.5,
                ),
              ),
              if (stickerAsset != null)
                SizedBox(
                  width: 32,
                  height: 32,
                  child: Image.asset(
                    stickerAsset,
                    fit: BoxFit.contain,
                    errorBuilder: (context, error, stackTrace) => const SizedBox(),
                  ),
                ),
            ],
          ),
          const SizedBox(height: 8),
          Row(
            mainAxisAlignment: MainAxisAlignment.spaceBetween,
            crossAxisAlignment: CrossAxisAlignment.end,
            children: [
              Text(
                value,
                style: const TextStyle(
                  fontFamily: 'SpaceGrotesk',
                  fontSize: 13,
                  fontWeight: FontWeight.bold,
                  color: Color(0xFF1E1A14),
                ),
              ),
              Icon(icon, color: textColor, size: 20),
            ],
          ),
        ],
      ),
    );
  }

  // --- LAB CARD BUILDER (A01 Mint & A05 Yellow) ---
  Widget _buildLabCard({
    required LabItem lab,
    required bool isYellowTheme,
  }) {
    final cardBg = isYellowTheme ? const Color(0xFFFEF9C3) : const Color(0xFFDCFCE7);
    final cardBorder = isYellowTheme ? const Color(0xFFFDE047) : const Color(0xFF86EFAC);
    final badgeBg = isYellowTheme ? const Color(0xFFFDE68A) : const Color(0xFFBBF7D0);
    final badgeFg = isYellowTheme ? const Color(0xFF92400E) : const Color(0xFF166534);

    return Stack(
      clipBehavior: Clip.none,
      children: [
        Container(
          padding: const EdgeInsets.all(16),
          decoration: BoxDecoration(
            color: cardBg,
            borderRadius: BorderRadius.circular(10),
            border: Border.all(color: cardBorder, width: 1.5),
            boxShadow: [
              BoxShadow(
                color: Colors.black.withValues(alpha: 0.03),
                blurRadius: 6,
                offset: const Offset(0, 2),
              ),
            ],
          ),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              // Top Badges & Cute Inside Sticker
              Row(
                mainAxisAlignment: MainAxisAlignment.spaceBetween,
                children: [
                  StatusBadge(
                    status: lab.status,
                    backgroundColor: badgeBg,
                    textColor: badgeFg,
                    borderColor: badgeFg.withValues(alpha: 0.3),
                  ),
                  Row(
                    children: [
                      SizedBox(
                        width: 32,
                        height: 32,
                        child: Image.asset(
                          isYellowTheme ? 'assets/images/sticker_pink.png' : 'assets/images/sticker_duck.png',
                          fit: BoxFit.contain,
                          errorBuilder: (context, error, stackTrace) => const SizedBox(),
                        ),
                      ),
                      const SizedBox(width: 8),
                      StatusBadge(
                        status: 'PORT ${lab.port}',
                        backgroundColor: badgeBg,
                        textColor: badgeFg,
                        borderColor: badgeFg.withValues(alpha: 0.3),
                      ),
                    ],
                  ),
                ],
              ),
              const SizedBox(height: 12),

              // Title & ID
              Text(
                lab.title,
                style: const TextStyle(
                  fontFamily: 'SpaceGrotesk',
                  fontSize: 16,
                  fontWeight: FontWeight.bold,
                  color: Color(0xFF1E1A14),
                ),
              ),
              const SizedBox(height: 4),
              Text(
                'ID: ${lab.id.toUpperCase()} • Version: v${lab.version}',
                style: const TextStyle(
                  fontFamily: 'JetBrainsMono',
                  fontSize: 11,
                  color: Color(0xFF475569),
                  fontWeight: FontWeight.w500,
                ),
              ),
              const SizedBox(height: 12),

              // Readiness Row + Hazard Tag
              Container(
                padding: const EdgeInsets.symmetric(horizontal: 10, vertical: 8),
                decoration: BoxDecoration(
                  color: const Color(0xFFFCFBF8).withValues(alpha: 0.85),
                  borderRadius: BorderRadius.circular(6),
                  border: Border.all(color: cardBorder, width: 1),
                ),
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Row(
                      children: [
                        _readinessDot(lab.learnReadiness),
                        const SizedBox(width: 4),
                        Text(
                          'Learn: ${lab.learnReadiness}',
                          style: TextStyle(
                            fontFamily: 'JetBrainsMono',
                            fontSize: 10,
                            fontWeight: FontWeight.bold,
                            color: _readinessColor(lab.learnReadiness),
                          ),
                        ),
                        const SizedBox(width: 6),
                        _readinessDot(lab.practiceReadiness),
                        const SizedBox(width: 4),
                        Text(
                          'Practice: ${lab.practiceReadiness}',
                          style: TextStyle(
                            fontFamily: 'JetBrainsMono',
                            fontSize: 10,
                            fontWeight: FontWeight.bold,
                            color: _readinessColor(lab.practiceReadiness),
                          ),
                        ),
                        const SizedBox(width: 6),
                        _readinessDot(lab.challengeReadiness),
                        const SizedBox(width: 4),
                        Text(
                          'Challenge',
                          style: TextStyle(
                            fontFamily: 'JetBrainsMono',
                            fontSize: 10,
                            fontWeight: FontWeight.bold,
                            color: _readinessColor(lab.challengeReadiness),
                          ),
                        ),
                      ],
                    ),
                    if (lab.recommendedTools.isNotEmpty) ...[
                      const SizedBox(height: 4),
                      Text(
                        'Recommended Tool: ${lab.recommendedTools.join(", ")}',
                        style: const TextStyle(
                          fontFamily: 'JetBrainsMono',
                          fontSize: 10,
                          fontWeight: FontWeight.bold,
                          color: Color(0xFFB45309),
                        ),
                      ),
                    ],
                  ],
                ),
              ),

              const SizedBox(height: 14),

              // Buttons Row
              Row(
                children: [
                  Expanded(
                    child: isYellowTheme
                        ? ZiteraButton(
                            label: 'ENTER LABORATORY',
                            icon: Icons.login,
                            variant: ButtonVariant.gradient,
                            fontSize: 10,
                            padding: const EdgeInsets.symmetric(vertical: 8),
                            onPressed: () => widget.onOpenLab(lab.id),
                          )
                        : ZiteraButton(
                            label: 'ENTER LABORATORY',
                            icon: Icons.login,
                            variant: ButtonVariant.mint,
                            fontSize: 10,
                            padding: const EdgeInsets.symmetric(vertical: 8),
                            onPressed: () => widget.onOpenLab(lab.id),
                          ),
                  ),
                  const SizedBox(width: 8),
                  ZiteraButton(
                    label: 'View Details',
                    variant: ButtonVariant.secondary,
                    fontSize: 10,
                    padding: const EdgeInsets.symmetric(horizontal: 10, vertical: 8),
                    onPressed: () => _showLabDetailsDialog(context, lab),
                  ),
                ],
              ),
            ],
          ),
        ),

        // Corner Sparkle Accent
        Positioned(
          bottom: -8,
          right: -8,
          child: const Icon(Icons.auto_awesome, size: 16, color: Color(0xFFA78BFA)),
        ),
      ],
    );
  }

  // --- MODAL DIALOG ON "VIEW DETAILS" ---
  void _showLabDetailsDialog(BuildContext context, LabItem lab) {
    showDialog(
      context: context,
      builder: (ctx) {
        return Dialog(
          backgroundColor: Colors.transparent,
          child: Container(
            width: 540,
            padding: const EdgeInsets.all(22),
            decoration: BoxDecoration(
              color: const Color(0xFFFCFBF8),
              borderRadius: BorderRadius.circular(12),
              border: Border.all(color: ZiteraColors.borderDark, width: 1.5),
              boxShadow: [
                BoxShadow(
                  color: Colors.black.withValues(alpha: 0.12),
                  blurRadius: 20,
                  offset: const Offset(0, 8),
                ),
              ],
            ),
            child: Column(
              mainAxisSize: MainAxisSize.min,
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                // Modal Header
                Row(
                  mainAxisAlignment: MainAxisAlignment.spaceBetween,
                  children: [
                    Row(
                      children: [
                        Container(
                          padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 4),
                          decoration: BoxDecoration(
                            color: ZiteraColors.statToolsBg,
                            borderRadius: BorderRadius.circular(4),
                            border: Border.all(color: ZiteraColors.statToolsBorder),
                          ),
                          child: Text(
                            'LAB SPEC // ${lab.id.toUpperCase()}',
                            style: const TextStyle(
                              fontFamily: 'JetBrainsMono',
                              fontSize: 11,
                              fontWeight: FontWeight.bold,
                              color: ZiteraColors.statToolsText,
                            ),
                          ),
                        ),
                        const SizedBox(width: 8),
                        Text(
                          'v${lab.version}',
                          style: const TextStyle(
                            fontFamily: 'JetBrainsMono',
                            fontSize: 11,
                            color: ZiteraColors.textMuted,
                          ),
                        ),
                      ],
                    ),
                    IconButton(
                      icon: const Icon(Icons.close, size: 18),
                      padding: EdgeInsets.zero,
                      constraints: const BoxConstraints(),
                      onPressed: () => Navigator.of(ctx).pop(),
                    ),
                  ],
                ),
                const SizedBox(height: 14),

                // Lab Title & Port
                Text(
                  lab.title,
                  style: const TextStyle(
                    fontFamily: 'SpaceGrotesk',
                    fontSize: 20,
                    fontWeight: FontWeight.bold,
                    color: ZiteraColors.textPrimary,
                  ),
                ),
                const SizedBox(height: 4),
                Text(
                  'Local Docker Port: ${lab.port > 0 ? lab.port : "Auto"} • Status: ${lab.status}',
                  style: const TextStyle(
                    fontFamily: 'JetBrainsMono',
                    fontSize: 12,
                    color: ZiteraColors.textSecondary,
                  ),
                ),
                const SizedBox(height: 16),

                // Modes Overview
                Container(
                  padding: const EdgeInsets.all(12),
                  decoration: BoxDecoration(
                    color: const Color(0xFFF8FAFC),
                    borderRadius: BorderRadius.circular(8),
                    border: Border.all(color: ZiteraColors.border),
                  ),
                  child: Column(
                    children: [
                      _modalModeRow('1. LEARN MODE', 'Study vulnerability mechanics, code patterns, and OWASP references.', lab.learnReadiness),
                      const Divider(height: 16),
                      _modalModeRow('2. PRACTICE MODE', 'Follow guided step-by-step walkthroughs with live telemetry.', lab.practiceReadiness),
                      const Divider(height: 16),
                      _modalModeRow('3. CHALLENGE MODE', 'Execute real-world exploit payload to capture the victory flag.', lab.challengeReadiness),
                    ],
                  ),
                ),

                if (lab.recommendedTools.isNotEmpty) ...[
                  const SizedBox(height: 12),
                  Row(
                    children: [
                      const Text(
                        'RECOMMENDED TOOLS: ',
                        style: TextStyle(
                          fontFamily: 'SpaceGrotesk',
                          fontSize: 11,
                          fontWeight: FontWeight.bold,
                          color: ZiteraColors.textSecondary,
                        ),
                      ),
                      Text(
                        lab.recommendedTools.join(', '),
                        style: const TextStyle(
                          fontFamily: 'JetBrainsMono',
                          fontSize: 12,
                          fontWeight: FontWeight.bold,
                          color: Color(0xFFB45309),
                        ),
                      ),
                    ],
                  ),
                ],

                const SizedBox(height: 18),

                // Actions
                Row(
                  mainAxisAlignment: MainAxisAlignment.end,
                  children: [
                    ZiteraButton(
                      label: 'CLOSE',
                      variant: ButtonVariant.secondary,
                      onPressed: () => Navigator.of(ctx).pop(),
                    ),
                    const SizedBox(width: 10),
                    ZiteraButton(
                      label: 'ENTER LABORATORY',
                      icon: Icons.login,
                      variant: ButtonVariant.gradient,
                      onPressed: () {
                        Navigator.of(ctx).pop();
                        widget.onOpenLab(lab.id);
                      },
                    ),
                  ],
                ),
              ],
            ),
          ),
        );
      },
    );
  }

  Widget _modalModeRow(String title, String description, String status) {
    return Row(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        _readinessDot(status),
        const SizedBox(width: 8),
        Expanded(
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Row(
                mainAxisAlignment: MainAxisAlignment.spaceBetween,
                children: [
                  Text(
                    title,
                    style: const TextStyle(
                      fontFamily: 'SpaceGrotesk',
                      fontSize: 12,
                      fontWeight: FontWeight.bold,
                      color: ZiteraColors.textPrimary,
                    ),
                  ),
                  Text(
                    status,
                    style: TextStyle(
                      fontFamily: 'JetBrainsMono',
                      fontSize: 10,
                      fontWeight: FontWeight.bold,
                      color: _readinessColor(status),
                    ),
                  ),
                ],
              ),
              const SizedBox(height: 2),
              Text(
                description,
                style: const TextStyle(
                  fontFamily: 'JetBrainsMono',
                  fontSize: 11,
                  color: ZiteraColors.textSecondary,
                ),
              ),
            ],
          ),
        ),
      ],
    );
  }

  Widget _pixelBarChart() {
    return Row(
      crossAxisAlignment: CrossAxisAlignment.end,
      children: [
        Container(
          width: 5,
          height: 10,
          decoration: BoxDecoration(
            color: const Color(0xFF2DD4BF),
            borderRadius: BorderRadius.circular(1),
          ),
        ),
        const SizedBox(width: 2),
        Container(
          width: 5,
          height: 18,
          decoration: BoxDecoration(
            color: const Color(0xFFF472B6),
            borderRadius: BorderRadius.circular(1),
          ),
        ),
        const SizedBox(width: 2),
        Container(
          width: 5,
          height: 14,
          decoration: BoxDecoration(
            color: const Color(0xFFFBBF24),
            borderRadius: BorderRadius.circular(1),
          ),
        ),
      ],
    );
  }

  Widget _readinessDot(String status) {
    return Container(
      width: 6,
      height: 6,
      margin: const EdgeInsets.only(top: 4),
      decoration: BoxDecoration(
        color: _readinessColor(status),
        borderRadius: BorderRadius.circular(1),
      ),
    );
  }

  Color _readinessColor(String status) {
    switch (status.toUpperCase()) {
      case 'READY':
        return const Color(0xFF16A34A);
      case 'PARTIAL':
        return const Color(0xFFD97706);
      default:
        return const Color(0xFFDC2626);
    }
  }
}
