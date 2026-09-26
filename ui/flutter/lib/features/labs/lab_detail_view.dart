import 'package:flutter/material.dart';
import '../../core/ipc/engine_client.dart';
import '../../core/ipc/models.dart';
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
  bool _isLoading = true;
  int _revealedHintTier = 0;
  final TextEditingController _flagController = TextEditingController();
  String? _flagFeedback;
  bool _flagSuccess = false;

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
    setState(() => _isLoading = true);
    try {
      final st = await ZiteraEngineClient.getLabStatus(widget.labId);
      setState(() {
        _status = st;
        _isLoading = false;
      });
    } catch (_) {
      setState(() => _isLoading = false);
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
          SnackBar(content: Text('Lab ${widget.labId} deterministically reset.'), backgroundColor: ZiteraColors.ready),
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

  void _verifyFlag() {
    final input = _flagController.text.trim();
    String expected;
    if (widget.labId == 'A01') {
      expected = 'ZITERA{b10k3n_4cc355_c0ntr01_m45t3r}';
    } else {
      expected = 'ZITERA{5q1_1nj3ct10n_m45t3r_2026}';
    }

    setState(() {
      if (input == expected) {
        _flagSuccess = true;
        _flagFeedback = 'EXCELLENT! Challenge Completed! Flag Verified.';
      } else {
        _flagSuccess = false;
        _flagFeedback = 'Invalid Flag. Keep investigating!';
      }
    });
  }

  @override
  Widget build(BuildContext context) {
    if (_isLoading) {
      return const Center(child: CircularProgressIndicator(color: ZiteraColors.primary));
    }

    final isA01 = widget.labId == 'A01';
    final title = isA01 ? 'Broken Access Control' : 'Injection (SQLi)';
    final owaspCode = isA01 ? 'A01:2025' : 'A05:2025';
    final port = isA01 ? 8011 : 8015;
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
            child: StatusBadge(status: isRunning ? 'RUNNING' : 'STOPPED'),
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
                      'Target URL: http://127.0.0.1:$port',
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
                _buildLearnTab(isA01),
                _buildPracticeTab(isA01, port),
                _buildChallengeTab(isA01, port),
              ],
            ),
          ),
        ],
      ),
    );
  }

  Widget _buildLearnTab(bool isA01) {
    return SingleChildScrollView(
      padding: const EdgeInsets.all(32.0),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          _sectionCard(
            title: isA01 ? 'Analogy: The Hotel Room Keycard' : 'Analogy: The Tampered Bank Check',
            icon: Icons.lightbulb_outline,
            accentColor: ZiteraColors.cyan,
            content: isA01
                ? 'Authentication checks your reservation at the front desk and hands you keycard #302. Insecure Direct Object Reference (IDOR) happens when room doors trust your phone app saying "open room #303" without verifying that room 303 belongs to you.'
                : 'Imagine writing a paper check with a memo line. If the recipient writes an instruction in the memo line and the bank cashier blindly executes it as a new money transfer, that is Injection! The database confuses user-provided data for executable database commands.',
          ),
          const SizedBox(height: 20),
          _sectionCard(
            title: 'Technical Core Concept',
            icon: Icons.code,
            accentColor: ZiteraColors.primary,
            content: isA01
                ? 'Broken Access Control occurs when authorization rules are not enforced on the server. If Alice views /invoice/1, and simply changes the URL to /invoice/2, a vulnerable server returns Bob\'s invoice because it queries the database by ID without checking whether invoice.user_id matches the active session.'
                : 'SQL Injection occurs when user input is concatenated directly into SQL queries using string formatting (e.g. f"SELECT * FROM items WHERE name LIKE \'%{input}%\'"). Injecting single quotes (\') breaks out of the string boundary, allowing attackers to append OR 1=1 or UNION SELECT to steal database records.',
          ),
          const SizedBox(height: 20),
          _sectionCard(
            title: 'Secure Remediation',
            icon: Icons.security,
            accentColor: ZiteraColors.ready,
            content: isA01
                ? 'Always verify ownership server-side before serving objects:\n\nSELECT * FROM invoices WHERE id = ? AND user_id = ?\n\nAdopt the Principle of Least Privilege: deny all access by default.'
                : 'Never use string concatenation or formatting for SQL queries. Always use Parameterized Queries (Prepared Statements). In parameterized queries, database drivers send query structure and user data over completely separate channels, making injection impossible.',
          ),
        ],
      ),
    );
  }

  Widget _buildPracticeTab(bool isA01, int port) {
    return SingleChildScrollView(
      padding: const EdgeInsets.all(32.0),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Text(
            'GUIDED PRACTICE INVESTIGATION (STEP-BY-STEP)',
            style: const TextStyle(
              fontSize: 16,
              fontWeight: FontWeight.bold,
              letterSpacing: 1.2,
              fontFamily: 'monospace',
            ),
          ),
          const SizedBox(height: 16),
          _stepCard(
            step: '01',
            action: 'Launch & Open Target',
            instruction: 'Start the lab environment and open http://127.0.0.1:$port in your browser.',
            observation: 'The target web application loads in your local browser.',
          ),
          const SizedBox(height: 16),
          _stepCard(
            step: '02',
            action: isA01 ? 'Log In as Alice' : 'Test Normal Query',
            instruction: isA01
                ? 'Sign in with credentials: username "alice", password "password123".'
                : 'Enter "Server" in the search box and observe standard filtered hardware items.',
            observation: isA01 ? 'You are redirected to /invoice/1' : 'Only Server hardware records appear in table.',
          ),
          const SizedBox(height: 16),
          _stepCard(
            step: '03',
            action: isA01 ? 'Test Object Identifier Tampering' : 'Inject Syntax Delimiter',
            instruction: isA01
                ? 'Edit the URL from /invoice/1 to /invoice/2 and hit Enter.'
                : 'Type a single apostrophe (\') into the search box and press Search.',
            observation: isA01
                ? 'Bob\'s confidential invoice details load without requiring his password!'
                : 'A raw database SQLite OperationalError appears, confirming syntax injection.',
          ),
        ],
      ),
    );
  }

  Widget _buildChallengeTab(bool isA01, int port) {
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
                    StatusBadge(status: 'DIFFICULTY: BEGINNER'),
                  ],
                ),
                const SizedBox(height: 12),
                Text(
                  isA01
                      ? 'An internal financial portal leaks records via authorization tampering. Locate the hidden master billing record (#42) to recover the project flag.'
                      : 'The hardware catalog portal query logic can be broken via UNION SQL injection. Extract the secret flag from the private "vault_secrets" table.',
                  style: const TextStyle(color: ZiteraColors.textPrimary, fontSize: 14, height: 1.4),
                ),
              ],
            ),
          ),

          const SizedBox(height: 24),

          const Text(
            'PROGRESSIVE HINTS (USE ONLY WHEN STUCK)',
            style: TextStyle(fontWeight: FontWeight.bold, fontSize: 13, letterSpacing: 1.0, fontFamily: 'monospace'),
          ),
          const SizedBox(height: 12),

          _buildHintAccordion(1, 'Hint 1: Conceptual', isA01 ? 'Inspect the URL parameters and think about how the server locates records.' : 'Determine how many columns are returned by the original SELECT query.'),
          _buildHintAccordion(2, 'Hint 2: Technical Area', isA01 ? 'Standard users start at 1 and 2. Administrative records have higher IDs.' : 'The original table has 3 columns: name, category, and price.'),
          _buildHintAccordion(3, 'Hint 3: Investigation Direction', isA01 ? 'Try inspecting invoice ID 42.' : 'Try: \' UNION SELECT secret_name, secret_data, 0 FROM vault_secrets --'),
          _buildHintAccordion(4, 'Hint 4: Solution Path', isA01 ? 'Navigate directly to http://127.0.0.1:8011/invoice/42 while logged in.' : 'Enter the UNION payload in the search field to dump the vault_secrets table contents.'),

          const SizedBox(height: 28),

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
                      label: 'Verify Flag',
                      icon: Icons.verified_outlined,
                      variant: ButtonVariant.primary,
                      onPressed: _verifyFlag,
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
                        Text(
                          _flagFeedback!,
                          style: TextStyle(
                            color: _flagSuccess ? ZiteraColors.ready : ZiteraColors.error,
                            fontWeight: FontWeight.bold,
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
                    Text(text, style: const TextStyle(color: ZiteraColors.cyan, fontSize: 12)),
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

  Widget _sectionCard({required String title, required IconData icon, required Color accentColor, required String content}) {
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
          Text(content, style: const TextStyle(color: ZiteraColors.textSecondary, fontSize: 13, height: 1.5)),
        ],
      ),
    );
  }

  Widget _stepCard({required String step, required String action, required String instruction, required String observation}) {
    return ZiteraCard(
      child: Row(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Container(
            padding: const EdgeInsets.all(8),
            decoration: BoxDecoration(
              color: ZiteraColors.primaryMuted,
              borderRadius: BorderRadius.circular(4),
            ),
            child: Text(step, style: const TextStyle(color: ZiteraColors.primary, fontWeight: FontWeight.w900, fontFamily: 'monospace')),
          ),
          const SizedBox(width: 16),
          Expanded(
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Text(action, style: const TextStyle(fontWeight: FontWeight.bold, fontSize: 14)),
                const SizedBox(height: 6),
                Text('Task: $instruction', style: const TextStyle(color: ZiteraColors.textSecondary, fontSize: 12)),
                const SizedBox(height: 4),
                Text('Expected Observation: $observation', style: const TextStyle(color: ZiteraColors.cyan, fontSize: 12)),
              ],
            ),
          ),
        ],
      ),
    );
  }
}
