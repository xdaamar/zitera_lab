import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import '../../core/ipc/engine_client.dart';
import '../../core/ipc/models.dart';
import '../../core/progress/progress_manager.dart';
import '../../core/theme/zitera_colors.dart';
import '../../widgets/status_badge.dart';
import '../../widgets/zitera_button.dart';
import '../../widgets/zitera_card.dart';
import '../../widgets/cute_anime_loading.dart';
import '../../widgets/hacker_tilix_entrance.dart';
import '../../widgets/zitera_rich_content.dart';
import '../../core/i18n/language_controller.dart';
import '../../core/i18n/lab_localization.dart';
import '../../widgets/terminal_console_widget.dart';

class LabDetailView extends StatefulWidget {
  final String labId;
  final VoidCallback onBack;

  const LabDetailView({
    super.key,
    required this.labId,
    required this.onBack,
  });

  @override
  State<LabDetailView> createState() => _LabDetailViewState();
}

class _LabDetailViewState extends State<LabDetailView> with SingleTickerProviderStateMixin {
  late TabController _tabController;
  LabItem? _status;
  LabContent? _content;
  bool _isLoading = true;
  String? _error;
  int _revealedHintTier = 0;
  final TextEditingController _flagController = TextEditingController();
  String? _flagFeedback;
  bool _flagSuccess = false;
  bool _isVerifyingFlag = false;

  bool _isChallengeSolved = false;
  bool _isPracticeDone = false;
  Set<String> _completedSections = {};
  bool _isVerifyingPractice = false;
  PracticeVerificationResult? _practiceResult;
  bool _dismissToolWarning = false;

  @override
  void initState() {
    super.initState();
    _tabController = TabController(length: 4, vsync: this);
    _refreshStatus();
  }

  @override
  void dispose() {
    _tabController.dispose();
    _flagController.dispose();
    super.dispose();
  }

  Future<void> _refreshStatus() async {
    setState(() {
      _isLoading = true;
      _error = null;
    });

    try {
      final st = await ZiteraEngineClient.getLabStatus(widget.labId);
      LabContent? content;
      if (st.installed) {
        try {
          content = await ZiteraEngineClient.getLabContent(widget.labId);
        } catch (_) {}
      }

      final progressData = await ProgressManager.loadProgress();
      final challenges = List<String>.from(progressData['completed_challenges'] as List? ?? []);
      final practice = List<String>.from(progressData['completed_practice'] as List? ?? []);
      final sectionsMap = Map<String, dynamic>.from(progressData['completed_sections'] as Map? ?? {});
      final labSections = Set<String>.from(sectionsMap[widget.labId] as List? ?? []);

      // Record lab opened in structured progress model (Phase 19 Checkpoint 6)
      await ProgressManager.recordLabOpened(widget.labId);

      if (mounted) {
        setState(() {
          _status = st;
          _content = content;
          _isChallengeSolved = challenges.contains(widget.labId);
          _isPracticeDone = practice.contains(widget.labId);
          _completedSections = labSections;
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

  Future<void> _handleStart() async {
    try {
      await ZiteraEngineClient.startLab(widget.labId);
      _refreshStatus();
    } catch (e) {
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(content: Text('Start error: $e'), backgroundColor: ZiteraColors.error),
        );
      }
    }
  }

  Future<void> _handleStop() async {
    try {
      await ZiteraEngineClient.stopLab(widget.labId);
      _refreshStatus();
    } catch (e) {
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(content: Text('Stop error: $e'), backgroundColor: ZiteraColors.error),
        );
      }
    }
  }

  Future<void> _handleReset() async {
    final confirmed = await showDialog<bool>(
      context: context,
      builder: (ctx) => AlertDialog(
        backgroundColor: const Color(0xFFFCFBF8),
        shape: RoundedRectangleBorder(
          borderRadius: BorderRadius.circular(12),
          side: const BorderSide(color: ZiteraColors.border, width: 1.5),
        ),
        title: Text(
          'Reset Lab ${widget.labId}?',
          style: const TextStyle(
            fontFamily: 'SpaceGrotesk',
            fontWeight: FontWeight.bold,
            fontSize: 16,
            color: Color(0xFF1E1A14),
          ),
        ),
        content: Text(
          'This will reset the native sandboxed lab ${widget.labId} and restore its runtime memory and seed state. Local lesson reading progress will NOT be lost.',
          style: const TextStyle(
            fontFamily: 'JetBrainsMono',
            fontSize: 12,
            color: Color(0xFF5C5347),
          ),
        ),
        actions: [
          TextButton(
            onPressed: () => Navigator.of(ctx).pop(false),
            child: const Text('Cancel', style: TextStyle(fontFamily: 'SpaceGrotesk', color: Color(0xFF5C5347))),
          ),
          ZiteraButton(
            label: 'Yes, Reset Lab',
            icon: Icons.restore,
            variant: ButtonVariant.danger,
            onPressed: () => Navigator.of(ctx).pop(true),
          ),
        ],
      ),
    );

    if (confirmed != true) return;

    try {
      await ZiteraEngineClient.resetLab(widget.labId);
      _refreshStatus();
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(
            content: Text('Lab ${widget.labId} deterministically reset.'),
            backgroundColor: ZiteraColors.ready,
          ),
        );
      }
    } catch (e) {
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(content: Text('Reset error: $e'), backgroundColor: ZiteraColors.error),
        );
      }
    }
  }

  Future<void> _handleUpdate() async {
    try {
      final res = await ZiteraEngineClient.updateLab(widget.labId);
      _refreshStatus();
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(content: Text(res), backgroundColor: ZiteraColors.ready),
        );
      }
    } catch (e) {
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(content: Text('Update error: $e'), backgroundColor: ZiteraColors.error),
        );
      }
    }
  }

  Future<void> _handleRemove() async {
    final confirmed = await showDialog<bool>(
      context: context,
      builder: (ctx) => AlertDialog(
        backgroundColor: const Color(0xFFFCFBF8),
        shape: RoundedRectangleBorder(
          borderRadius: BorderRadius.circular(12),
          side: const BorderSide(color: ZiteraColors.border, width: 1.5),
        ),
        title: Text(
          'Remove Lab ${widget.labId}?',
          style: const TextStyle(
            fontFamily: 'SpaceGrotesk',
            fontWeight: FontWeight.bold,
            fontSize: 16,
            color: Color(0xFF1E1A14),
          ),
        ),
        content: Text(
          'This terminates the native sandboxed process and removes local package files for ${widget.labId}. You can reinstall it anytime from the catalog.',
          style: const TextStyle(
            fontFamily: 'JetBrainsMono',
            fontSize: 12,
            color: Color(0xFF5C5347),
          ),
        ),
        actions: [
          TextButton(
            onPressed: () => Navigator.of(ctx).pop(false),
            child: const Text('Cancel', style: TextStyle(fontFamily: 'SpaceGrotesk', color: Color(0xFF5C5347))),
          ),
          ZiteraButton(
            label: 'Yes, Remove Lab',
            icon: Icons.delete_forever,
            variant: ButtonVariant.danger,
            onPressed: () => Navigator.of(ctx).pop(true),
          ),
        ],
      ),
    );

    if (confirmed != true) return;

    try {
      await ZiteraEngineClient.removeLab(widget.labId);
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(content: Text('Lab ${widget.labId} removed.'), backgroundColor: ZiteraColors.ready),
        );
        widget.onBack();
      }
    } catch (e) {
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(content: Text('Remove error: $e'), backgroundColor: ZiteraColors.error),
        );
      }
    }
  }

  Future<void> _verifyPracticeTarget() async {
    setState(() {
      _isVerifyingPractice = true;
      _practiceResult = null;
    });

    try {
      final res = await ZiteraEngineClient.verifyPractice(widget.labId);
      if (mounted) {
        setState(() {
          _practiceResult = res;
          _isVerifyingPractice = false;
        });
      }
    } catch (e) {
      if (mounted) {
        setState(() {
          _practiceResult = PracticeVerificationResult(
            labId: widget.labId,
            status: 'error',
            message: 'Failed to execute practice probe: $e',
          );
          _isVerifyingPractice = false;
        });
      }
    }
  }

  Future<void> _toggleSectionUnderstood(String sectionKey) async {
    await ProgressManager.markSectionCompleted(widget.labId, sectionKey);
    setState(() {
      _completedSections.add(sectionKey);
    });
    if (mounted) {
      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(
          content: Text('Section "$sectionKey" marked as completed.'),
          backgroundColor: ZiteraColors.ready,
          duration: const Duration(seconds: 1),
        ),
      );
    }
  }

  Future<void> _markPracticeCompleted() async {
    await ProgressManager.markPracticeCompleted(widget.labId);
    setState(() {
      _isPracticeDone = true;
    });
    if (mounted) {
      ScaffoldMessenger.of(context).showSnackBar(
        const SnackBar(
          content: Text('Practice mode marked as completed!'),
          backgroundColor: ZiteraColors.ready,
        ),
      );
    }
  }

  Future<void> _verifyFlag() async {
    final input = _flagController.text.trim();
    if (input.isEmpty) return;

    setState(() {
      _isVerifyingFlag = true;
      _flagFeedback = null;
    });

    try {
      await ProgressManager.recordChallengeAttempt(widget.labId);
      final res = await ZiteraEngineClient.validateChallenge(widget.labId, input);
      final passed = res.status == 'passed';

      if (passed) {
        // Authoritative pass: record challenge completed WITHOUT saving flag text
        await ProgressManager.markChallengeCompleted(widget.labId);
      }

      if (mounted) {
        setState(() {
          _flagSuccess = passed;
          if (passed) _isChallengeSolved = true;
          _flagFeedback = res.message;
          _isVerifyingFlag = false;
        });

        if (passed) {
          _showChallengeCelebrationDialog(res.message);
        }
      }
    } catch (e) {
      if (mounted) {
        setState(() {
          _flagSuccess = false;
          _flagFeedback = 'Error verifying flag: $e';
          _isVerifyingFlag = false;
        });
      }
    }
  }

  void _showChallengeCelebrationDialog(String message) {
    if (!mounted) return;
    showDialog<void>(
      context: context,
      barrierDismissible: true,
      builder: (BuildContext dialogContext) {
        final labTitle = _status?.title ?? widget.labId;
        return Dialog(
          backgroundColor: Colors.transparent,
          child: Container(
            constraints: const BoxConstraints(maxWidth: 520),
            padding: const EdgeInsets.all(28),
            decoration: BoxDecoration(
              color: ZiteraColors.card,
              borderRadius: BorderRadius.circular(16),
              border: Border.all(color: ZiteraColors.borderDark, width: 2),
              boxShadow: const [
                BoxShadow(
                  color: Color(0x1A000000),
                  offset: Offset(4, 4),
                  blurRadius: 10,
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
                      padding: const EdgeInsets.all(12),
                      decoration: BoxDecoration(
                        color: ZiteraColors.labMintBg,
                        borderRadius: BorderRadius.circular(12),
                        border: Border.all(color: ZiteraColors.labMintBorder, width: 1.5),
                      ),
                      child: const Icon(
                        Icons.military_tech,
                        size: 32,
                        color: Color(0xFF166534),
                      ),
                    ),
                    const SizedBox(width: 16),
                    Expanded(
                      child: Column(
                        crossAxisAlignment: CrossAxisAlignment.start,
                        children: [
                          Container(
                            padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 3),
                            decoration: BoxDecoration(
                              color: ZiteraColors.readyMuted,
                              borderRadius: BorderRadius.circular(6),
                              border: Border.all(color: ZiteraColors.ready),
                            ),
                            child: const Text(
                              'FLAG VALIDATED • SOLVED',
                              style: TextStyle(
                                fontFamily: 'SpaceGrotesk',
                                fontSize: 11,
                                fontWeight: FontWeight.w800,
                                color: ZiteraColors.ready,
                                letterSpacing: 1.0,
                              ),
                            ),
                          ),
                          const SizedBox(height: 4),
                          const Text(
                            'CHALLENGE CONQUERED!',
                            style: TextStyle(
                              fontFamily: 'SpaceGrotesk',
                              fontSize: 20,
                              fontWeight: FontWeight.w900,
                              color: ZiteraColors.textPrimary,
                            ),
                          ),
                        ],
                      ),
                    ),
                    IconButton(
                      icon: const Icon(Icons.close, color: ZiteraColors.textSecondary),
                      onPressed: () => Navigator.of(dialogContext).pop(),
                    ),
                  ],
                ),
                const SizedBox(height: 20),
                Container(
                  padding: const EdgeInsets.all(16),
                  decoration: BoxDecoration(
                    color: ZiteraColors.background,
                    borderRadius: BorderRadius.circular(12),
                    border: Border.all(color: ZiteraColors.border, width: 1.5),
                  ),
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      const Row(
                        children: [
                          Icon(Icons.school, size: 18, color: ZiteraColors.primary),
                          SizedBox(width: 8),
                          Text(
                            'KEY DEFENSIVE TAKEAWAYS',
                            style: TextStyle(
                              fontFamily: 'SpaceGrotesk',
                              fontSize: 12,
                              fontWeight: FontWeight.w800,
                              color: ZiteraColors.primary,
                              letterSpacing: 0.5,
                            ),
                          ),
                        ],
                      ),
                      const SizedBox(height: 10),
                      Text(
                        'Lab: $labTitle',
                        style: const TextStyle(
                          fontFamily: 'SpaceGrotesk',
                          fontSize: 14,
                          fontWeight: FontWeight.w700,
                          color: ZiteraColors.textPrimary,
                        ),
                      ),
                      const SizedBox(height: 6),
                      Text(
                        message.isNotEmpty ? message : 'Congratulations on solving the security challenge for this lab!',
                        style: const TextStyle(
                          fontFamily: 'SpaceGrotesk',
                          fontSize: 13,
                          fontWeight: FontWeight.w500,
                          color: ZiteraColors.textSecondary,
                          height: 1.4,
                        ),
                      ),
                      const SizedBox(height: 10),
                      const Text(
                        'Always ensure secure defaults, least privilege, robust input sanitization, and continuous security audits in production environments.',
                        style: TextStyle(
                          fontFamily: 'SpaceGrotesk',
                          fontSize: 12,
                          fontStyle: FontStyle.italic,
                          color: ZiteraColors.textMuted,
                          height: 1.4,
                        ),
                      ),
                    ],
                  ),
                ),
                const SizedBox(height: 24),
                ElevatedButton(
                  style: ElevatedButton.styleFrom(
                    backgroundColor: ZiteraColors.labMintBtn,
                    foregroundColor: ZiteraColors.textPrimary,
                    padding: const EdgeInsets.symmetric(vertical: 14),
                    elevation: 0,
                    shape: RoundedRectangleBorder(
                      borderRadius: BorderRadius.circular(10),
                      side: const BorderSide(color: ZiteraColors.labMintBorder, width: 1.5),
                    ),
                  ),
                  onPressed: () => Navigator.of(dialogContext).pop(),
                  child: const Text(
                    'CONTINUE LEARNING',
                    style: TextStyle(
                      fontFamily: 'SpaceGrotesk',
                      fontSize: 14,
                      fontWeight: FontWeight.w800,
                      letterSpacing: 0.8,
                    ),
                  ),
                ),
              ],
            ),
          ),
        );
      },
    );
  }

  void _copyToClipboard(String text, String label) {
    Clipboard.setData(ClipboardData(text: text));
    ScaffoldMessenger.of(context).showSnackBar(
      SnackBar(content: Text('$label copied to clipboard!'), backgroundColor: ZiteraColors.ready),
    );
  }

  @override
  Widget build(BuildContext context) {
    if (_isLoading) {
      return const Scaffold(
        backgroundColor: ZiteraColors.background,
        body: Center(
          child: CuteAnimeLoading(
            message: 'LOADING LAB WORKSPACE & CURRICULUM...',
            subMessage: '( •̀ ω •́ )✧ PARSING ZERO-RECOMPILE MARKDOWN',
          ),
        ),
      );
    }

    if (_error != null && _status == null) {
      return Scaffold(
        backgroundColor: ZiteraColors.background,
        appBar: AppBar(leading: IconButton(icon: const Icon(Icons.arrow_back), onPressed: widget.onBack)),
        body: Center(
          child: ZiteraCard(
            borderColor: ZiteraColors.error,
            child: Text('Error loading lab: $_error', style: const TextStyle(color: ZiteraColors.error)),
          ),
        ),
      );
    }

    final manifest = _content?.manifest;
    final title = manifest?.title ?? _status?.title ?? widget.labId;
    final owaspCode = manifest?.owasp ?? 'OWASP';
    final port = manifest?.defaultPort ?? _status?.port ?? 0;
    final isRunning = _status?.running ?? false;
    final hasMissingRecommended = _status?.challengeReadiness == 'PARTIAL' && !_dismissToolWarning;

    return ValueListenableBuilder<String>(
      valueListenable: AppLanguageController.currentLanguage,
      builder: (context, _, _) {
        return Scaffold(
          backgroundColor: ZiteraColors.background,
          appBar: AppBar(
        leading: IconButton(
          icon: const Icon(Icons.arrow_back),
          onPressed: widget.onBack,
        ),
        title: Text('LAB $owaspCode // ${title.toUpperCase()}'),
        actions: [
          ValueListenableBuilder<String>(
            valueListenable: AppLanguageController.currentLanguage,
            builder: (context, lang, _) {
              final isId = lang == 'id';
              return InkWell(
                onTap: () => AppLanguageController.toggle(),
                borderRadius: BorderRadius.circular(6),
                child: Container(
                  padding: const EdgeInsets.symmetric(horizontal: 10, vertical: 4),
                  margin: const EdgeInsets.only(right: 12),
                  decoration: BoxDecoration(
                    color: isId ? const Color(0xFFCCFBF1) : const Color(0xFFF1F5F9),
                    borderRadius: BorderRadius.circular(6),
                    border: Border.all(
                      color: isId ? const Color(0xFF2DD4BF) : const Color(0xFFCBD5E1),
                      width: 1,
                    ),
                  ),
                  child: Row(
                    mainAxisSize: MainAxisSize.min,
                    children: [
                      const Icon(Icons.language, size: 14, color: Color(0xFF0F766E)),
                      const SizedBox(width: 5),
                      Text(
                        isId ? 'ID (BAHASA)' : 'EN (ENGLISH)',
                        style: const TextStyle(
                          fontFamily: 'JetBrainsMono',
                          fontSize: 10.5,
                          fontWeight: FontWeight.bold,
                          color: Color(0xFF0F766E),
                        ),
                      ),
                    ],
                  ),
                ),
              );
            },
          ),
          if (_isChallengeSolved)
            const Padding(
              padding: EdgeInsets.only(right: 12.0),
              child: StatusBadge(status: 'CHALLENGE SOLVED'),
            ),
          Padding(
            padding: const EdgeInsets.only(right: 16.0),
            child: StatusBadge(status: isRunning ? 'RUNNING' : (_status?.status ?? 'STOPPED')),
          ),
        ],
      ),
      body: Column(
        children: [
          // Non-blocking Recommended Tool Missing Banner (Phase 6D / 6L)
          if (hasMissingRecommended)
            Container(
              padding: const EdgeInsets.symmetric(horizontal: 24, vertical: 10),
              color: ZiteraColors.warning.withValues(alpha: 0.15),
              child: Row(
                children: [
                  const Icon(Icons.info_outline, color: ZiteraColors.warning, size: 20),
                  const SizedBox(width: 12),
                  Expanded(
                    child: Text(
                      'RECOMMENDED TOOLS MISSING (${_status!.recommendedTools.join(", ")}) — Learn & Practice modes are fully functional. You may install them via Tools Manager or continue.',
                      style: const TextStyle(color: ZiteraColors.warning, fontSize: 12),
                    ),
                  ),
                  TextButton(
                    onPressed: () => setState(() => _dismissToolWarning = true),
                    child: const Text('Dismiss', style: TextStyle(color: ZiteraColors.textPrimary, fontSize: 12)),
                  ),
                ],
              ),
            ),

          // Control & Runtime Banner
          HackerTilixEntrance(
            delay: Duration.zero,
            direction: TilixSlideDirection.down,
            child: Container(
              padding: const EdgeInsets.symmetric(horizontal: 24, vertical: 14),
              decoration: const BoxDecoration(
                color: ZiteraColors.surface,
                border: Border(bottom: BorderSide(color: ZiteraColors.border)),
              ),
              child: Row(
                mainAxisAlignment: MainAxisAlignment.spaceBetween,
                children: [
                  Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      Row(
                        children: [
                          Text(
                            port > 0 ? 'Target URL: http://127.0.0.1:$port' : 'Port: Unassigned',
                            style: const TextStyle(
                              fontFamily: 'monospace',
                              color: ZiteraColors.cyan,
                              fontWeight: FontWeight.bold,
                              fontSize: 13,
                            ),
                          ),
                          if (port > 0) ...[
                            const SizedBox(width: 8),
                            IconButton(
                              icon: const Icon(Icons.copy, size: 14, color: ZiteraColors.textMuted),
                              tooltip: 'Copy URL',
                              onPressed: () => _copyToClipboard('http://127.0.0.1:$port', 'Target URL'),
                            ),
                          ],
                        ],
                      ),
                      const Text(
                        'Security boundary: localhost only (no LAN exposure)',
                        style: TextStyle(color: ZiteraColors.textMuted, fontSize: 11),
                      ),
                    ],
                  ),
                  Row(
                    children: [
                      if (!isRunning) ...[
                        ZiteraButton(
                          label: 'Start Lab Environment',
                          icon: Icons.play_arrow,
                          variant: ButtonVariant.primary,
                          onPressed: _handleStart,
                        ),
                        const SizedBox(width: 8),
                        PopupMenuButton<String>(
                          icon: const Icon(Icons.more_vert, color: Color(0xFF5C5347)),
                          tooltip: 'Lab Management',
                          color: const Color(0xFFFCFBF8),
                          shape: RoundedRectangleBorder(
                            borderRadius: BorderRadius.circular(8),
                            side: const BorderSide(color: ZiteraColors.border),
                          ),
                          onSelected: (val) {
                            if (val == 'update') _handleUpdate();
                            if (val == 'remove') _handleRemove();
                          },
                          itemBuilder: (ctx) => [
                            const PopupMenuItem(
                              value: 'update',
                              child: Row(
                                children: [
                                  Icon(Icons.sync, size: 16, color: Color(0xFF0F766E)),
                                  SizedBox(width: 8),
                                  Text('Check & Update Lab', style: TextStyle(fontFamily: 'SpaceGrotesk', fontSize: 13)),
                                ],
                              ),
                            ),
                            const PopupMenuItem(
                              value: 'remove',
                              child: Row(
                                children: [
                                  Icon(Icons.delete_outline, size: 16, color: ZiteraColors.error),
                                  SizedBox(width: 8),
                                  Text('Remove Lab', style: TextStyle(fontFamily: 'SpaceGrotesk', fontSize: 13, color: ZiteraColors.error)),
                                ],
                              ),
                            ),
                          ],
                        ),
                      ] else ...[
                        ZiteraButton(
                          label: 'Stop Runtime',
                          icon: Icons.stop,
                          variant: ButtonVariant.danger,
                          onPressed: _handleStop,
                        ),
                        const SizedBox(width: 8),
                        ZiteraButton(
                          label: 'Deterministic Reset',
                          icon: Icons.restore,
                          variant: ButtonVariant.secondary,
                          onPressed: _handleReset,
                        ),
                      ],
                    ],
                  ),
                ],
              ),
            ),
          ),

          // Mode Tabs (Generic Learning Experience)
          HackerTilixEntrance(
            delay: const Duration(milliseconds: 50),
            direction: TilixSlideDirection.up,
            child: Container(
              color: ZiteraColors.surface,
              child: TabBar(
                controller: _tabController,
                indicatorColor: ZiteraColors.primary,
                labelColor: ZiteraColors.primary,
                unselectedLabelColor: ZiteraColors.textSecondary,
                tabs: [
                  Tab(icon: const Icon(Icons.menu_book_outlined, size: 18), text: 'MODE A: LEARN (${_completedSections.length} Done)'),
                  Tab(icon: Icon(_isPracticeDone ? Icons.check_circle : Icons.explore_outlined, size: 18), text: 'MODE B: PRACTICE'),
                  Tab(icon: Icon(_isChallengeSolved ? Icons.verified : Icons.flag_outlined, size: 18), text: 'MODE C: CHALLENGE / CTF'),
                  Tab(icon: const Icon(Icons.terminal, size: 18), text: 'TERMINAL'),
                ],
              ),
            ),
          ),

          // Tab Content
          Expanded(
            child: TabBarView(
              controller: _tabController,
              children: [
                _buildLearnTab(),
                _buildPracticeTab(port),
                _buildChallengeTab(),
                Padding(
                  padding: const EdgeInsets.all(16.0),
                  child: TerminalConsoleWidget(labId: widget.labId, labPort: port),
                ),
              ],
            ),
          ),
        ],
      ),
    );
      },
    );
  }

  Widget _buildLearnTab() {
    final lessons = _content?.lessons ?? {};

    if (lessons.isEmpty) {
      return Center(
        child: ZiteraCard(
          child: Text(
            _status?.installed == true
                ? 'Lesson content loading or not present in repository.'
                : 'Lab not installed. Please install the lab to view lesson materials.',
            style: const TextStyle(color: ZiteraColors.textSecondary),
          ),
        ),
      );
    }

    final isIndonesian = AppLanguageController.isIndonesian;
    // Dynamic section ordering - filter out locale keys (.id, _id)
    final keys = lessons.keys
        .where((k) => !k.endsWith('.id') && !k.endsWith('_id'))
        .toList();
    // Prioritize standard keys if present
    final preferredOrder = ['introduction', 'overview', 'analogy', 'concept', 'remediation', 'walkthrough', 'review'];
    keys.sort((a, b) {
      final indexA = preferredOrder.indexOf(a);
      final indexB = preferredOrder.indexOf(b);
      if (indexA != -1 && indexB != -1) return indexA.compareTo(indexB);
      if (indexA != -1) return -1;
      if (indexB != -1) return 1;
      return a.compareTo(b);
    });

    return SingleChildScrollView(
      padding: const EdgeInsets.all(32.0),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          // Phase 19 Checkpoint 4 & 5: Curriculum Specification & Learning Objectives
          if (_content != null) ...[
            HackerTilixEntrance(
              delay: const Duration(milliseconds: 40),
              direction: TilixSlideDirection.down,
              child: Container(
                margin: const EdgeInsets.only(bottom: 24.0),
                padding: const EdgeInsets.all(20.0),
                decoration: BoxDecoration(
                  color: const Color(0xFFFCFBF8),
                  borderRadius: BorderRadius.circular(10),
                  border: Border.all(color: const Color(0xFF2DD4BF), width: 1.4),
                  boxShadow: [
                    BoxShadow(
                      color: Colors.black.withValues(alpha: 0.02),
                      blurRadius: 8,
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
                        Row(
                          children: [
                            Container(
                              padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 3),
                              decoration: BoxDecoration(
                                color: const Color(0xFF0F766E),
                                borderRadius: BorderRadius.circular(4),
                              ),
                              child: Text(
                                '${_content!.manifest.standard.toUpperCase()} : ${_content!.manifest.standardVersion}',
                                style: const TextStyle(
                                  fontFamily: 'JetBrainsMono',
                                  fontSize: 10,
                                  fontWeight: FontWeight.bold,
                                  color: Colors.white,
                                ),
                              ),
                            ),
                            const SizedBox(width: 8),
                            Container(
                              padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 3),
                              decoration: BoxDecoration(
                                color: const Color(0xFFE6FFFA),
                                borderRadius: BorderRadius.circular(4),
                                border: Border.all(color: const Color(0xFF2DD4BF)),
                              ),
                              child: Text(
                                '${_content!.manifest.categoryId} • ${_content!.manifest.categoryName.toUpperCase()}',
                                style: const TextStyle(
                                  fontFamily: 'JetBrainsMono',
                                  fontSize: 10,
                                  fontWeight: FontWeight.bold,
                                  color: Color(0xFF0F766E),
                                ),
                              ),
                            ),
                          ],
                        ),
                        Container(
                          padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 3),
                          decoration: BoxDecoration(
                            color: const Color(0xFFF0EDE8),
                            borderRadius: BorderRadius.circular(4),
                            border: Border.all(color: ZiteraColors.border),
                          ),
                          child: Text(
                            'EST. ${_content!.manifest.estimatedMinutes} MIN',
                            style: const TextStyle(
                              fontFamily: 'JetBrainsMono',
                              fontSize: 10,
                              fontWeight: FontWeight.bold,
                              color: Color(0xFF5C5347),
                            ),
                          ),
                        ),
                      ],
                    ),
                    if (_content!.manifest.shortDescription.isNotEmpty) ...[
                      const SizedBox(height: 12),
                      Text(
                        _content!.manifest.shortDescription,
                        style: const TextStyle(
                          fontFamily: 'SpaceGrotesk',
                          fontSize: 13,
                          fontWeight: FontWeight.w500,
                          color: Color(0xFF1E1A14),
                          height: 1.4,
                        ),
                      ),
                    ],
                    if (_content!.manifest.learningObjectives.isNotEmpty) ...[
                      const SizedBox(height: 14),
                      const Text(
                        'LEARNING OBJECTIVES',
                        style: TextStyle(
                          fontFamily: 'SpaceGrotesk',
                          fontSize: 11,
                          fontWeight: FontWeight.bold,
                          letterSpacing: 0.6,
                          color: Color(0xFF0F766E),
                        ),
                      ),
                      const SizedBox(height: 6),
                      ..._content!.manifest.learningObjectives.map((obj) => Padding(
                        padding: const EdgeInsets.only(bottom: 4.0),
                        child: Row(
                          crossAxisAlignment: CrossAxisAlignment.start,
                          children: [
                            const Text('• ', style: TextStyle(color: Color(0xFF0F766E), fontWeight: FontWeight.bold)),
                            Expanded(
                              child: Text(
                                obj,
                                style: const TextStyle(
                                  fontFamily: 'SpaceGrotesk',
                                  fontSize: 12.5,
                                  color: Color(0xFF5C5347),
                                ),
                              ),
                            ),
                          ],
                        ),
                      )),
                    ],
                    if (_content!.manifest.skills.isNotEmpty || _content!.manifest.prerequisites.isNotEmpty) ...[
                      const SizedBox(height: 12),
                      Wrap(
                        spacing: 8,
                        runSpacing: 6,
                        children: [
                          ..._content!.manifest.skills.map((s) => Container(
                            padding: const EdgeInsets.symmetric(horizontal: 6, vertical: 2),
                            decoration: BoxDecoration(
                              color: const Color(0xFFEFF6FF),
                              borderRadius: BorderRadius.circular(4),
                              border: Border.all(color: const Color(0xFFBFDBFE)),
                            ),
                            child: Text(
                              'Skill: $s',
                              style: const TextStyle(
                                fontFamily: 'SpaceGrotesk',
                                fontSize: 10.5,
                                fontWeight: FontWeight.w600,
                                color: Color(0xFF1D4ED8),
                              ),
                            ),
                          )),
                          ..._content!.manifest.prerequisites.map((p) => Container(
                            padding: const EdgeInsets.symmetric(horizontal: 6, vertical: 2),
                            decoration: BoxDecoration(
                              color: const Color(0xFFFFFBEB),
                              borderRadius: BorderRadius.circular(4),
                              border: Border.all(color: const Color(0xFFFDE68A)),
                            ),
                            child: Text(
                              'Prereq: $p',
                              style: const TextStyle(
                                fontFamily: 'SpaceGrotesk',
                                fontSize: 10.5,
                                fontWeight: FontWeight.w600,
                                color: Color(0xFFB45309),
                              ),
                            ),
                          )),
                        ],
                      ),
                    ],
                  ],
                ),
              ),
            ),
          ],
          ...keys.asMap().entries.map((entry) {
          final index = entry.key;
          final key = entry.value;
          final content = LabLocalization.getLessonContent(
            labId: widget.labId,
            sectionKey: key,
            lessons: lessons,
            isIndonesian: isIndonesian,
          );
          final title = _formatSectionTitle(key);
          final isCompleted = _completedSections.contains(key);

          return Padding(
            padding: const EdgeInsets.only(bottom: 20.0),
            child: HackerTilixEntrance(
              delay: Duration(milliseconds: 80 + index * 50),
              direction: TilixSlideDirection.up,
              child: ZiteraCard(
                borderColor: isCompleted ? ZiteraColors.ready.withValues(alpha: 0.3) : ZiteraColors.border,
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Row(
                      mainAxisAlignment: MainAxisAlignment.spaceBetween,
                      children: [
                        Row(
                          children: [
                            Icon(_getSectionIcon(key), color: ZiteraColors.primary, size: 20),
                            const SizedBox(width: 10),
                            Text(title, style: const TextStyle(fontWeight: FontWeight.bold, fontSize: 15)),
                          ],
                        ),
                        if (isCompleted)
                          const Row(
                            children: [
                              Icon(Icons.check, color: ZiteraColors.ready, size: 16),
                              SizedBox(width: 4),
                              Text('UNDERSTOOD', style: TextStyle(color: ZiteraColors.ready, fontSize: 11, fontWeight: FontWeight.bold)),
                            ],
                          )
                        else
                          TextButton.icon(
                            icon: const Icon(Icons.done, size: 14),
                            label: const Text('Mark Understood', style: TextStyle(fontSize: 12)),
                            onPressed: () => _toggleSectionUnderstood(key),
                          ),
                      ],
                    ),
                    const SizedBox(height: 12),
                    ZiteraRichContent(
                      rawContent: content,
                    ),
                  ],
                ),
              ),
            ),
          );
        }).toList(),
      ),
    );
  }

  Widget _buildPracticeTab(int port) {
    final walkthroughText = _content?.lessons['walkthrough'] ?? _content?.lessons['practice'];

    return SingleChildScrollView(
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
                const Text(
                  'GUIDED PRACTICE INVESTIGATION (STEP-BY-STEP)',
                  style: TextStyle(
                    fontSize: 16,
                    fontWeight: FontWeight.bold,
                    letterSpacing: 1.2,
                    fontFamily: 'monospace',
                  ),
                ),
                Row(
                  children: [
                    if (_isPracticeDone)
                      const Padding(
                        padding: EdgeInsets.only(right: 8.0),
                        child: StatusBadge(status: 'PRACTICE DONE'),
                      ),
                    StatusBadge(status: 'PORT: $port'),
                  ],
                ),
              ],
            ),
          ),
          const SizedBox(height: 16),

          // Interactive Practice Target Verification Card (Phase 6F)
          HackerTilixEntrance(
            delay: const Duration(milliseconds: 60),
            direction: TilixSlideDirection.up,
            child: ZiteraCard(
              borderColor: ZiteraColors.cyan.withValues(alpha: 0.4),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Row(
                    mainAxisAlignment: MainAxisAlignment.spaceBetween,
                    children: [
                      const Row(
                        children: [
                          Icon(Icons.radar, color: ZiteraColors.cyan, size: 20),
                          SizedBox(width: 10),
                          Text(
                            'PRACTICE RUNTIME PROBE',
                            style: TextStyle(fontWeight: FontWeight.bold, fontSize: 14, fontFamily: 'monospace'),
                          ),
                        ],
                      ),
                      ZiteraButton(
                        label: _isVerifyingPractice ? 'Probing Target...' : 'Verify Local Service Health',
                        icon: Icons.network_check,
                        variant: ButtonVariant.secondary,
                        onPressed: _isVerifyingPractice ? () {} : _verifyPracticeTarget,
                      ),
                    ],
                  ),
                  if (_practiceResult != null) ...[
                    const SizedBox(height: 12),
                    Container(
                      padding: const EdgeInsets.all(10),
                      decoration: BoxDecoration(
                        color: _practiceResult!.status == 'passed' ? ZiteraColors.readyMuted : ZiteraColors.errorMuted,
                        borderRadius: BorderRadius.circular(4),
                        border: Border.all(
                          color: _practiceResult!.status == 'passed' ? ZiteraColors.ready : ZiteraColors.error,
                        ),
                      ),
                      child: Row(
                        children: [
                          Icon(
                            _practiceResult!.status == 'passed' ? Icons.check_circle : Icons.error_outline,
                            color: _practiceResult!.status == 'passed' ? ZiteraColors.ready : ZiteraColors.error,
                            size: 18,
                          ),
                          const SizedBox(width: 10),
                          Expanded(
                            child: Text(
                              _practiceResult!.message,
                              style: TextStyle(
                                color: _practiceResult!.status == 'passed' ? ZiteraColors.ready : ZiteraColors.error,
                                fontSize: 12,
                                fontWeight: FontWeight.bold,
                              ),
                            ),
                          ),
                        ],
                      ),
                    ),
                  ],
                ],
              ),
            ),
          ),

          const SizedBox(height: 16),

          HackerTilixEntrance(
            delay: const Duration(milliseconds: 120),
            direction: TilixSlideDirection.up,
            child: walkthroughText != null && walkthroughText.isNotEmpty
                ? ZiteraCard(
                    child: ZiteraRichContent(
                      rawContent: LabLocalization.getWalkthroughText(
                        labId: widget.labId,
                        lessons: _content?.lessons ?? {},
                        isIndonesian: AppLanguageController.isIndonesian,
                      ),
                    ),
                  )
                : const ZiteraCard(
                    child: Text(
                      'Practice walkthrough not specified in lab repository.',
                      style: TextStyle(color: ZiteraColors.textSecondary),
                    ),
                  ),
          ),

          const SizedBox(height: 20),

          // Completion action
          HackerTilixEntrance(
            delay: const Duration(milliseconds: 180),
            direction: TilixSlideDirection.up,
            child: Row(
              mainAxisAlignment: MainAxisAlignment.end,
              children: [
                if (!_isPracticeDone)
                  ZiteraButton(
                    label: 'Mark Practice as Completed',
                    icon: Icons.check_circle_outline,
                    variant: ButtonVariant.primary,
                    onPressed: _markPracticeCompleted,
                  )
                else
                  const Row(
                    children: [
                      Icon(Icons.verified, color: ZiteraColors.ready, size: 18),
                      SizedBox(width: 8),
                      Text('Practice Completed & Saved', style: TextStyle(color: ZiteraColors.ready, fontWeight: FontWeight.bold)),
                    ],
                  ),
              ],
            ),
          ),
        ],
      ),
    );
  }

  Widget _buildChallengeTab() {
    final isIndonesian = AppLanguageController.isIndonesian;
    final rawObjective = (_content != null && _content!.challengeObjective.isNotEmpty)
        ? _content!.challengeObjective
        : 'Complete the mission objective in the target application.';
    final objective = LabLocalization.getChallengeObjective(
      labId: widget.labId,
      defaultObjective: rawObjective,
      isIndonesian: isIndonesian,
    );
    final difficulty = _content?.manifest.difficulty.toUpperCase() ?? 'BEGINNER';
    final hints = LabLocalization.getHints(
      labId: widget.labId,
      defaultHints: _content?.hints ?? [],
      isIndonesian: isIndonesian,
    );

    return SingleChildScrollView(
      padding: const EdgeInsets.all(32.0),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          // Conquered Banner if solved
          if (_isChallengeSolved) ...[
            HackerTilixEntrance(
              delay: Duration.zero,
              direction: TilixSlideDirection.down,
              child: Container(
                padding: const EdgeInsets.all(16),
                margin: const EdgeInsets.only(bottom: 20),
                decoration: BoxDecoration(
                  color: ZiteraColors.readyMuted,
                  borderRadius: BorderRadius.circular(6),
                  border: Border.all(color: ZiteraColors.ready),
                ),
                child: const Row(
                  children: [
                    Icon(Icons.workspace_premium, color: ZiteraColors.ready, size: 28),
                    SizedBox(width: 14),
                    Expanded(
                      child: Column(
                        crossAxisAlignment: CrossAxisAlignment.start,
                        children: [
                          Text('CHALLENGE CONQUERED!', style: TextStyle(color: ZiteraColors.ready, fontWeight: FontWeight.bold, fontSize: 16)),
                          Text('You have successfully verified and completed this cybersecurity challenge objective.', style: TextStyle(color: ZiteraColors.textPrimary, fontSize: 12)),
                        ],
                      ),
                    ),
                  ],
                ),
              ),
            ),
          ],

          HackerTilixEntrance(
            delay: const Duration(milliseconds: 40),
            direction: TilixSlideDirection.up,
            child: ZiteraCard(
              borderColor: ZiteraColors.primary.withValues(alpha: 0.5),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Row(
                    mainAxisAlignment: MainAxisAlignment.spaceBetween,
                    children: [
                      const Text(
                        'MISSION OBJECTIVE',
                        style: TextStyle(
                          color: ZiteraColors.primary,
                          fontWeight: FontWeight.w900,
                          letterSpacing: 1.5,
                          fontFamily: 'monospace',
                        ),
                      ),
                      StatusBadge(status: 'DIFFICULTY: $difficulty'),
                    ],
                  ),
                  const SizedBox(height: 12),
                  ZiteraRichContent(
                    rawContent: objective,
                  ),
                ],
              ),
            ),
          ),

          const SizedBox(height: 24),

          // Progressive Hints (Sequential Unlock: Hint 1 -> Hint 2 -> Hint 3)
          if (hints.isNotEmpty) ...[
            HackerTilixEntrance(
              delay: const Duration(milliseconds: 90),
              direction: TilixSlideDirection.left,
              child: const Text(
                'PROGRESSIVE HINTS (SEQUENTIAL UNLOCK)',
                style: TextStyle(fontWeight: FontWeight.bold, fontSize: 13, letterSpacing: 1.0, fontFamily: 'monospace'),
              ),
            ),
            const SizedBox(height: 12),
            ...hints.asMap().entries.map((entry) {
              final index = entry.key;
              final h = entry.value;
              final isUnlocked = _revealedHintTier >= h.tier;
              final canUnlock = _revealedHintTier >= (h.tier - 1);

              return Padding(
                padding: const EdgeInsets.only(bottom: 8.0),
                child: HackerTilixEntrance(
                  delay: Duration(milliseconds: 120 + index * 40),
                  direction: TilixSlideDirection.up,
                  child: ZiteraCard(
                    padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 12),
                    child: Row(
                      mainAxisAlignment: MainAxisAlignment.spaceBetween,
                      children: [
                        Expanded(
                          child: Column(
                            crossAxisAlignment: CrossAxisAlignment.start,
                            children: [
                              Text(
                                'Hint ${h.tier}: ${h.type.toUpperCase()}',
                                style: TextStyle(
                                  fontFamily: 'SpaceGrotesk',
                                  fontWeight: FontWeight.bold,
                                  fontSize: 13,
                                  color: isUnlocked ? const Color(0xFF1E1A14) : const Color(0xFF786F62),
                                ),
                              ),
                              if (isUnlocked) ...[
                                const SizedBox(height: 6),
                                ZiteraRichContent(
                                  rawContent: h.hint,
                                  baseStyle: const TextStyle(
                                    fontFamily: 'JetBrainsMono',
                                    color: Color(0xFF0F766E),
                                    fontSize: 12,
                                    fontWeight: FontWeight.w600,
                                  ),
                                ),
                              ],
                            ],
                          ),
                        ),
                        if (!isUnlocked)
                          TextButton(
                            onPressed: canUnlock ? () => setState(() => _revealedHintTier = h.tier) : null,
                            child: Text(
                              canUnlock ? 'Unlock Hint ${h.tier}' : 'Locked (Unlock Hint ${h.tier - 1} First)',
                              style: TextStyle(
                                fontFamily: 'SpaceGrotesk',
                                fontWeight: FontWeight.bold,
                                color: canUnlock ? const Color(0xFF0F766E) : const Color(0xFF9C9080),
                                fontSize: 12,
                              ),
                            ),
                          ),
                      ],
                    ),
                  ),
                ),
              );
            }),
            const SizedBox(height: 28),
          ],

          // Flag Submission Card
          HackerTilixEntrance(
            delay: const Duration(milliseconds: 200),
            direction: TilixSlideDirection.up,
            child: ZiteraCard(
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  const Text(
                    'SUBMIT CHALLENGE FLAG',
                    style: TextStyle(fontFamily: 'SpaceGrotesk', fontWeight: FontWeight.bold, fontSize: 14, letterSpacing: 0.8),
                  ),
                  const SizedBox(height: 12),
                  Row(
                    children: [
                      Expanded(
                        child: TextField(
                          controller: _flagController,
                          style: const TextStyle(fontFamily: 'JetBrainsMono', color: Color(0xFF1E1A14), fontWeight: FontWeight.bold),
                          decoration: InputDecoration(
                            hintText: 'ZITERA{...}',
                            hintStyle: const TextStyle(fontFamily: 'JetBrainsMono', color: Color(0xFF9C9080)),
                            filled: true,
                            fillColor: const Color(0xFFF7F5F0),
                            border: OutlineInputBorder(
                              borderRadius: BorderRadius.circular(6),
                              borderSide: const BorderSide(color: ZiteraColors.border),
                            ),
                          ),
                        ),
                      ),
                      const SizedBox(width: 16),
                      ZiteraButton(
                        label: _isVerifyingFlag ? 'Verifying...' : 'Verify Flag',
                        icon: Icons.verified_outlined,
                        variant: ButtonVariant.primary,
                        onPressed: _isVerifyingFlag ? () {} : _verifyFlag,
                      ),
                    ],
                  ),
                  if (_flagFeedback != null) ...[
                    const SizedBox(height: 16),
                    Container(
                      padding: const EdgeInsets.all(12),
                      decoration: BoxDecoration(
                        color: _flagSuccess ? ZiteraColors.readyMuted : ZiteraColors.errorMuted,
                        borderRadius: BorderRadius.circular(6),
                        border: Border.all(color: _flagSuccess ? ZiteraColors.ready : ZiteraColors.error),
                      ),
                      child: Row(
                        children: [
                          Icon(
                            _flagSuccess ? Icons.check_circle : Icons.error,
                            color: _flagSuccess ? ZiteraColors.ready : ZiteraColors.error,
                          ),
                          const SizedBox(width: 12),
                          Expanded(
                            child: Text(
                              _flagFeedback!,
                              style: TextStyle(
                                color: _flagSuccess ? ZiteraColors.ready : ZiteraColors.error,
                                fontWeight: FontWeight.bold,
                              ),
                            ),
                          ),
                        ],
                      ),
                    ),
                  ],
                ],
              ),
            ),
          ),
        ],
      ),
    );
  }

  String _formatSectionTitle(String key) {
    switch (key) {
      case 'introduction':
      case 'overview':
        return 'Overview & Introduction';
      case 'analogy':
        return 'Mental Model / Real-World Analogy';
      case 'concept':
        return 'Technical Core Concept';
      case 'remediation':
        return 'Secure Remediation & Defense';
      case 'walkthrough':
        return 'Investigation Walkthrough';
      case 'practice':
        return 'Interactive Practice Objectives';
      case 'review':
        return 'Security Takeaways & Review';
      default:
        return key.replaceAll('_', ' ').toUpperCase();
    }
  }

  IconData _getSectionIcon(String key) {
    switch (key) {
      case 'introduction':
      case 'overview':
        return Icons.info_outline;
      case 'analogy':
        return Icons.lightbulb_outline;
      case 'concept':
        return Icons.code;
      case 'remediation':
        return Icons.security;
      case 'walkthrough':
        return Icons.format_list_numbered;
      case 'practice':
        return Icons.explore_outlined;
      case 'review':
        return Icons.rate_review_outlined;
      default:
        return Icons.article_outlined;
    }
  }
}
