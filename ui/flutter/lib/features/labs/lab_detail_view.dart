import 'package:flutter/material.dart';
import '../../core/ipc/engine_client.dart';
import '../../core/ipc/models.dart';
import '../../core/progress/progress_manager.dart';
import '../../core/theme/zitera_colors.dart';
import '../../widgets/status_badge.dart';
import '../../widgets/zitera_button.dart';
import '../../widgets/zitera_card.dart';

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

  @override
  void initState() {
    super.initState();
    _tabController = TabController(length: 3, vsync: this);
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
        } catch (_) {
          // If reading content fails, graceful fallback
        }
      }

      if (mounted) {
        setState(() {
          _status = st;
          _content = content;
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

  Future<void> _verifyFlag() async {
    final input = _flagController.text.trim();
    if (input.isEmpty) return;

    setState(() {
      _isVerifyingFlag = true;
      _flagFeedback = null;
    });

    try {
      final res = await ZiteraEngineClient.validateChallenge(widget.labId, input);
      final passed = res.status == 'passed';

      if (passed) {
        await ProgressManager.markFlagSolved(widget.labId, input);
      }

      if (mounted) {
        setState(() {
          _flagSuccess = passed;
          _flagFeedback = res.message;
          _isVerifyingFlag = false;
        });
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

  @override
  Widget build(BuildContext context) {
    if (_isLoading) {
      return const Scaffold(
        backgroundColor: ZiteraColors.background,
        body: Center(child: CircularProgressIndicator(color: ZiteraColors.primary)),
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

    return Scaffold(
      backgroundColor: ZiteraColors.background,
      appBar: AppBar(
        leading: IconButton(
          icon: const Icon(Icons.arrow_back),
          onPressed: widget.onBack,
        ),
        title: Text('LAB $owaspCode // ${title.toUpperCase()}'),
        actions: [
          Padding(
            padding: const EdgeInsets.only(right: 16.0),
            child: StatusBadge(status: isRunning ? 'RUNNING' : (_status?.status ?? 'STOPPED')),
          ),
        ],
      ),
      body: Column(
        children: [
          // Control Banner
          Container(
            padding: const EdgeInsets.symmetric(horizontal: 24, vertical: 16),
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
                    Text(
                      port > 0 ? 'Target URL: http://127.0.0.1:$port' : 'Port: Unassigned',
                      style: const TextStyle(
                        fontFamily: 'monospace',
                        color: ZiteraColors.cyan,
                        fontWeight: FontWeight.bold,
                        fontSize: 13,
                      ),
                    ),
                    const SizedBox(height: 2),
                    const Text(
                      'Security boundary: localhost only (no LAN exposure)',
                      style: TextStyle(color: ZiteraColors.textMuted, fontSize: 11),
                    ),
                  ],
                ),
                Row(
                  children: [
                    if (!isRunning)
                      ZiteraButton(
                        label: 'Start Lab Environment',
                        icon: Icons.play_arrow,
                        variant: ButtonVariant.primary,
                        onPressed: _handleStart,
                      )
                    else ...[
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

          // Mode Tabs
          Container(
            color: ZiteraColors.surface,
            child: TabBar(
              controller: _tabController,
              indicatorColor: ZiteraColors.primary,
              labelColor: ZiteraColors.primary,
              unselectedLabelColor: ZiteraColors.textSecondary,
              tabs: const [
                Tab(icon: Icon(Icons.menu_book_outlined, size: 18), text: 'MODE A: LEARN'),
                Tab(icon: Icon(Icons.explore_outlined, size: 18), text: 'MODE B: PRACTICE'),
                Tab(icon: Icon(Icons.flag_outlined, size: 18), text: 'MODE C: CHALLENGE / CTF'),
              ],
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
              ],
            ),
          ),
        ],
      ),
    );
  }

  Widget _buildLearnTab() {
    final lessons = _content?.lessons ?? {};
    final analogyText = lessons['analogy'];
    final conceptText = lessons['concept'];
    final remediationText = lessons['remediation'];
    final introText = lessons['introduction'];

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

    return SingleChildScrollView(
      padding: const EdgeInsets.all(32.0),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          if (introText != null && introText.isNotEmpty) ...[
            _sectionCard(
              title: 'Overview & Introduction',
              icon: Icons.info_outline,
              accentColor: ZiteraColors.primary,
              content: introText,
            ),
            const SizedBox(height: 20),
          ],
          if (analogyText != null && analogyText.isNotEmpty) ...[
            _sectionCard(
              title: 'Mental Model / Real-World Analogy',
              icon: Icons.lightbulb_outline,
              accentColor: ZiteraColors.cyan,
              content: analogyText,
            ),
            const SizedBox(height: 20),
          ],
          if (conceptText != null && conceptText.isNotEmpty) ...[
            _sectionCard(
              title: 'Technical Core Concept',
              icon: Icons.code,
              accentColor: ZiteraColors.primary,
              content: conceptText,
            ),
            const SizedBox(height: 20),
          ],
          if (remediationText != null && remediationText.isNotEmpty) ...[
            _sectionCard(
              title: 'Secure Remediation & Defense',
              icon: Icons.security,
              accentColor: ZiteraColors.ready,
              content: remediationText,
            ),
          ],
        ],
      ),
    );
  }

  Widget _buildPracticeTab(int port) {
    final walkthroughText = _content?.lessons['walkthrough'];

    return SingleChildScrollView(
      padding: const EdgeInsets.all(32.0),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Row(
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
              StatusBadge(status: 'PORT: $port'),
            ],
          ),
          const SizedBox(height: 16),
          if (walkthroughText != null && walkthroughText.isNotEmpty)
            ZiteraCard(
              child: SelectableText(
                walkthroughText,
                style: const TextStyle(
                  color: ZiteraColors.textPrimary,
                  fontSize: 13,
                  height: 1.6,
                  fontFamily: 'monospace',
                ),
              ),
            )
          else
            const ZiteraCard(
              child: Text(
                'Practice walkthrough not specified in lab repository.',
                style: TextStyle(color: ZiteraColors.textSecondary),
              ),
            ),
        ],
      ),
    );
  }

  Widget _buildChallengeTab() {
    final objective = (_content != null && _content!.challengeObjective.isNotEmpty)
        ? _content!.challengeObjective
        : 'Complete the mission objective in the target application.';
    final difficulty = _content?.manifest.difficulty.toUpperCase() ?? 'BEGINNER';
    final hints = _content?.hints ?? [];

    return SingleChildScrollView(
      padding: const EdgeInsets.all(32.0),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          ZiteraCard(
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
                SelectableText(
                  objective,
                  style: const TextStyle(color: ZiteraColors.textPrimary, fontSize: 13, height: 1.5),
                ),
              ],
            ),
          ),

          const SizedBox(height: 24),

          if (hints.isNotEmpty) ...[
            const Text(
              'PROGRESSIVE HINTS (USE ONLY WHEN STUCK)',
              style: TextStyle(fontWeight: FontWeight.bold, fontSize: 13, letterSpacing: 1.0, fontFamily: 'monospace'),
            ),
            const SizedBox(height: 12),
            ...hints.map((h) => _buildHintAccordion(
                  h.tier,
                  'Hint ${h.tier}: ${h.type.toUpperCase()}',
                  h.hint,
                )),
            const SizedBox(height: 28),
          ],

          // Flag Submission
          ZiteraCard(
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                const Text(
                  'SUBMIT CHALLENGE FLAG',
                  style: TextStyle(fontWeight: FontWeight.bold, fontSize: 14, letterSpacing: 1.0, fontFamily: 'monospace'),
                ),
                const SizedBox(height: 12),
                Row(
                  children: [
                    Expanded(
                      child: TextField(
                        controller: _flagController,
                        style: const TextStyle(fontFamily: 'monospace', color: Colors.white),
                        decoration: InputDecoration(
                          hintText: 'ZITERA{...}',
                          hintStyle: const TextStyle(color: ZiteraColors.textMuted),
                          filled: true,
                          fillColor: ZiteraColors.surface,
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
        ],
      ),
    );
  }

  Widget _buildHintAccordion(int tier, String title, String text) {
    final isRevealed = _revealedHintTier >= tier;
    return Padding(
      padding: const EdgeInsets.only(bottom: 8.0),
      child: ZiteraCard(
        padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 12),
        child: Row(
          mainAxisAlignment: MainAxisAlignment.spaceBetween,
          children: [
            Expanded(
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Text(title, style: const TextStyle(fontWeight: FontWeight.bold, fontSize: 13)),
                  if (isRevealed) ...[
                    const SizedBox(height: 6),
                    SelectableText(text, style: const TextStyle(color: ZiteraColors.cyan, fontSize: 12)),
                  ],
                ],
              ),
            ),
            if (!isRevealed)
              TextButton(
                onPressed: () => setState(() => _revealedHintTier = tier),
                child: const Text('Unlock Hint', style: TextStyle(color: ZiteraColors.primary, fontSize: 12)),
              ),
          ],
        ),
      ),
    );
  }

  Widget _sectionCard({
    required String title,
    required IconData icon,
    required Color accentColor,
    required String content,
  }) {
    return ZiteraCard(
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Row(
            children: [
              Icon(icon, color: accentColor, size: 20),
              const SizedBox(width: 10),
              Text(title, style: const TextStyle(fontWeight: FontWeight.bold, fontSize: 15)),
            ],
          ),
          const SizedBox(height: 12),
          SelectableText(
            content,
            style: const TextStyle(color: ZiteraColors.textSecondary, fontSize: 13, height: 1.5),
          ),
        ],
      ),
    );
  }
}
