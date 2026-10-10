import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import '../core/ipc/engine_client.dart';

class TerminalConsoleWidget extends StatefulWidget {
  final String? labId;
  final int? labPort;

  const TerminalConsoleWidget({
    super.key,
    this.labId,
    this.labPort,
  });

  @override
  State<TerminalConsoleWidget> createState() => _TerminalConsoleWidgetState();
}

class _TerminalConsoleWidgetState extends State<TerminalConsoleWidget> {
  final TextEditingController _inputController = TextEditingController();
  final ScrollController _scrollController = ScrollController();
  final FocusNode _focusNode = FocusNode();

  final List<Map<String, dynamic>> _history = [];
  final List<String> _commandHistory = [];
  int _historyIndex = -1;

  String _currentCwd = '/';
  bool _isExecuting = false;

  @override
  void initState() {
    super.initState();
    _history.add({
      'type': 'system',
      'text': 'Zitera Educational Terminal v2.0 (OWASP:2025 Edition)\n'
          'Isolated in-memory Virtual Filesystem. Zero host shell passthrough.\n'
          'Type "help" or click command shortcuts below to get started.\n',
    });
  }

  @override
  void dispose() {
    _inputController.dispose();
    _scrollController.dispose();
    _focusNode.dispose();
    super.dispose();
  }

  void _scrollToBottom() {
    WidgetsBinding.instance.addPostFrameCallback((_) {
      if (_scrollController.hasClients) {
        _scrollController.animateTo(
          _scrollController.position.maxScrollExtent,
          duration: const Duration(milliseconds: 200),
          curve: Curves.easeOut,
        );
      }
    });
  }

  Future<void> _runCommand(String command) async {
    final cmd = command.trim();
    if (cmd.isEmpty) return;

    _commandHistory.add(cmd);
    _historyIndex = _commandHistory.length;

    setState(() {
      _history.add({
        'type': 'command',
        'cwd': _currentCwd,
        'command': cmd,
      });
      _isExecuting = true;
    });
    _inputController.clear();
    _scrollToBottom();

    if (cmd == 'clear') {
      setState(() {
        _history.clear();
        _isExecuting = false;
      });
      return;
    }

    try {
      final res = await ZiteraEngineClient.executeTerminal(cmd, labId: widget.labId);
      if (mounted) {
        setState(() {
          _currentCwd = res.cwd;
          _history.add({
            'type': 'output',
            'stdout': res.stdout,
            'stderr': res.stderr,
            'exitCode': res.exitCode,
          });
          _isExecuting = false;
        });
        _scrollToBottom();
      }
    } catch (e) {
      if (mounted) {
        setState(() {
          _history.add({
            'type': 'output',
            'stdout': '',
            'stderr': 'Execution error: $e\n',
            'exitCode': 1,
          });
          _isExecuting = false;
        });
        _scrollToBottom();
      }
    }
  }

  void _navigateHistory(bool up) {
    if (_commandHistory.isEmpty) return;
    if (up) {
      if (_historyIndex > 0) {
        _historyIndex--;
        _inputController.text = _commandHistory[_historyIndex];
        _inputController.selection = TextSelection.fromPosition(
          TextPosition(offset: _inputController.text.length),
        );
      }
    } else {
      if (_historyIndex < _commandHistory.length - 1) {
        _historyIndex++;
        _inputController.text = _commandHistory[_historyIndex];
        _inputController.selection = TextSelection.fromPosition(
          TextPosition(offset: _inputController.text.length),
        );
      } else {
        _historyIndex = _commandHistory.length;
        _inputController.clear();
      }
    }
  }

  void _copyAllOutput() {
    final buffer = StringBuffer();
    for (final item in _history) {
      if (item['type'] == 'command') {
        buffer.writeln('learner@zitera-lab:${item['cwd']}\$ ${item['command']}');
      } else if (item['type'] == 'output') {
        if (item['stdout'] != null && (item['stdout'] as String).isNotEmpty) {
          buffer.write(item['stdout']);
        }
        if (item['stderr'] != null && (item['stderr'] as String).isNotEmpty) {
          buffer.write(item['stderr']);
        }
      } else if (item['type'] == 'system') {
        buffer.writeln(item['text']);
      }
    }
    Clipboard.setData(ClipboardData(text: buffer.toString()));
    ScaffoldMessenger.of(context).showSnackBar(
      const SnackBar(
        content: Text('Terminal output copied to clipboard.'),
        duration: Duration(seconds: 2),
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    final defaultPort = widget.labPort ?? 8090;

    return Container(
      decoration: BoxDecoration(
        color: const Color(0xFF0F172A), // Slate 900
        borderRadius: BorderRadius.circular(10),
        border: Border.all(color: const Color(0xFF334155), width: 1.5),
        boxShadow: [
          BoxShadow(
            color: Colors.black.withValues(alpha: 0.3),
            blurRadius: 10,
            offset: const Offset(0, 4),
          ),
        ],
      ),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.stretch,
        children: [
          // Header Bar
          Container(
            padding: const EdgeInsets.symmetric(horizontal: 14, vertical: 8),
            decoration: const BoxDecoration(
              color: Color(0xFF1E293B), // Slate 800
              borderRadius: BorderRadius.only(
                topLeft: Radius.circular(8),
                topRight: Radius.circular(8),
              ),
              border: Border(
                bottom: BorderSide(color: Color(0xFF334155), width: 1),
              ),
            ),
            child: Row(
              children: [
                // Window dots
                Row(
                  children: [
                    Container(width: 10, height: 10, decoration: const BoxDecoration(color: Color(0xFFEF4444), shape: BoxShape.circle)),
                    const SizedBox(width: 6),
                    Container(width: 10, height: 10, decoration: const BoxDecoration(color: Color(0xFFF59E0B), shape: BoxShape.circle)),
                    const SizedBox(width: 6),
                    Container(width: 10, height: 10, decoration: const BoxDecoration(color: Color(0xFF10B981), shape: BoxShape.circle)),
                  ],
                ),
                const SizedBox(width: 12),
                const Icon(Icons.terminal, size: 16, color: Color(0xFF2DD4BF)),
                const SizedBox(width: 6),
                Text(
                  widget.labId != null
                      ? 'TERMINAL // [LAB: ${widget.labId}]'
                      : 'ZITERA SECURE EDUCATIONAL TERMINAL',
                  style: const TextStyle(
                    fontFamily: 'JetBrainsMono',
                    fontSize: 12,
                    fontWeight: FontWeight.bold,
                    color: Color(0xFFE2E8F0),
                  ),
                ),
                const Spacer(),
                Container(
                  padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 2),
                  decoration: BoxDecoration(
                    color: const Color(0xFF0F766E).withValues(alpha: 0.3),
                    borderRadius: BorderRadius.circular(4),
                    border: Border.all(color: const Color(0xFF2DD4BF), width: 0.8),
                  ),
                  child: Text(
                    'cwd: $_currentCwd',
                    style: const TextStyle(
                      fontFamily: 'JetBrainsMono',
                      fontSize: 11,
                      color: Color(0xFF2DD4BF),
                    ),
                  ),
                ),
                const SizedBox(width: 8),
                IconButton(
                  tooltip: 'Copy Output',
                  icon: const Icon(Icons.copy, size: 15, color: Color(0xFF94A3B8)),
                  onPressed: _copyAllOutput,
                  splashRadius: 16,
                  padding: EdgeInsets.zero,
                  constraints: const BoxConstraints(),
                ),
                const SizedBox(width: 8),
                IconButton(
                  tooltip: 'Clear Output',
                  icon: const Icon(Icons.block, size: 15, color: Color(0xFF94A3B8)),
                  onPressed: () => setState(() => _history.clear()),
                  splashRadius: 16,
                  padding: EdgeInsets.zero,
                  constraints: const BoxConstraints(),
                ),
              ],
            ),
          ),

          // Shortcut buttons
          Container(
            padding: const EdgeInsets.symmetric(horizontal: 10, vertical: 6),
            color: const Color(0xFF161F30),
            child: SingleChildScrollView(
              scrollDirection: Axis.horizontal,
              child: Row(
                children: [
                  const Text(
                    'COMMANDS: ',
                    style: TextStyle(
                      fontFamily: 'JetBrainsMono',
                      fontSize: 10.5,
                      fontWeight: FontWeight.bold,
                      color: Color(0xFF64748B),
                    ),
                  ),
                  _buildQuickPill('help', () => _runCommand('help')),
                  _buildQuickPill('help grep', () => _runCommand('help grep')),
                  _buildQuickPill('ls -la', () => _runCommand('ls -la')),
                  _buildQuickPill('ps -ef', () => _runCommand('ps -ef')),
                  _buildQuickPill('pwd', () => _runCommand('pwd')),
                  _buildQuickPill('whoami', () => _runCommand('whoami')),
                  _buildQuickPill('curl :$defaultPort', () => _runCommand('curl http://localhost:$defaultPort/')),
                  _buildQuickPill('clear', () => _runCommand('clear')),
                ],
              ),
            ),
          ),

          // Terminal Output Body
          Expanded(
            child: Container(
              padding: const EdgeInsets.all(12),
              child: ListView.builder(
                controller: _scrollController,
                itemCount: _history.length,
                itemBuilder: (context, index) {
                  final item = _history[index];
                  if (item['type'] == 'system') {
                    return Text(
                      item['text'] as String,
                      style: const TextStyle(
                        fontFamily: 'JetBrainsMono',
                        fontSize: 12,
                        color: Color(0xFF94A3B8),
                        height: 1.4,
                      ),
                    );
                  }
                  if (item['type'] == 'command') {
                    return Padding(
                      padding: const EdgeInsets.symmetric(vertical: 2.0),
                      child: Row(
                        crossAxisAlignment: CrossAxisAlignment.start,
                        children: [
                          Text(
                            'learner@zitera-lab:${item['cwd']}\$ ',
                            style: const TextStyle(
                              fontFamily: 'JetBrainsMono',
                              fontSize: 12,
                              fontWeight: FontWeight.bold,
                              color: Color(0xFF2DD4BF),
                            ),
                          ),
                          Expanded(
                            child: Text(
                              item['command'] as String,
                              style: const TextStyle(
                                fontFamily: 'JetBrainsMono',
                                fontSize: 12,
                                fontWeight: FontWeight.w600,
                                color: Color(0xFFF8FAFC),
                              ),
                            ),
                          ),
                        ],
                      ),
                    );
                  }
                  if (item['type'] == 'output') {
                    final stdout = item['stdout'] as String? ?? '';
                    final stderr = item['stderr'] as String? ?? '';
                    return Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        if (stdout.isNotEmpty)
                          Text(
                            stdout,
                            style: const TextStyle(
                              fontFamily: 'JetBrainsMono',
                              fontSize: 12,
                              color: Color(0xFFCBD5E1),
                              height: 1.35,
                            ),
                          ),
                        if (stderr.isNotEmpty)
                          Text(
                            stderr,
                            style: const TextStyle(
                              fontFamily: 'JetBrainsMono',
                              fontSize: 12,
                              color: Color(0xFFF87171),
                              height: 1.35,
                            ),
                          ),
                      ],
                    );
                  }
                  return const SizedBox();
                },
              ),
            ),
          ),

          // Interactive Input Prompt
          Container(
            padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 8),
            decoration: const BoxDecoration(
              color: Color(0xFF0F172A),
              border: Border(
                top: BorderSide(color: Color(0xFF334155), width: 1),
              ),
            ),
            child: KeyboardListener(
              focusNode: FocusNode(),
              onKeyEvent: (event) {
                if (event is KeyDownEvent) {
                  if (event.logicalKey == LogicalKeyboardKey.arrowUp) {
                    _navigateHistory(true);
                  } else if (event.logicalKey == LogicalKeyboardKey.arrowDown) {
                    _navigateHistory(false);
                  }
                }
              },
              child: Row(
                children: [
                  Text(
                    'learner@zitera-lab:$_currentCwd\$ ',
                    style: const TextStyle(
                      fontFamily: 'JetBrainsMono',
                      fontSize: 12.5,
                      fontWeight: FontWeight.bold,
                      color: Color(0xFF2DD4BF),
                    ),
                  ),
                  Expanded(
                    child: TextField(
                      controller: _inputController,
                      focusNode: _focusNode,
                      style: const TextStyle(
                        fontFamily: 'JetBrainsMono',
                        fontSize: 12.5,
                        color: Color(0xFFF8FAFC),
                      ),
                      cursorColor: const Color(0xFF2DD4BF),
                      decoration: const InputDecoration(
                        isDense: true,
                        contentPadding: EdgeInsets.zero,
                        border: InputBorder.none,
                        hintText: 'type a command (e.g. help, ls, ps)...',
                        hintStyle: TextStyle(
                          fontFamily: 'JetBrainsMono',
                          fontSize: 12,
                          color: Color(0xFF475569),
                        ),
                      ),
                      onSubmitted: (val) => _runCommand(val),
                    ),
                  ),
                  if (_isExecuting)
                    const SizedBox(
                      width: 14,
                      height: 14,
                      child: CircularProgressIndicator(
                        strokeWidth: 2,
                        valueColor: AlwaysStoppedAnimation(Color(0xFF2DD4BF)),
                      ),
                    )
                  else
                    IconButton(
                      tooltip: 'Run',
                      icon: const Icon(Icons.send_rounded, size: 16, color: Color(0xFF2DD4BF)),
                      onPressed: () => _runCommand(_inputController.text),
                      splashRadius: 16,
                      padding: EdgeInsets.zero,
                      constraints: const BoxConstraints(),
                    ),
                ],
              ),
            ),
          ),
        ],
      ),
    );
  }

  Widget _buildQuickPill(String label, VoidCallback onTap) {
    return Container(
      margin: const EdgeInsets.only(right: 6),
      child: InkWell(
        onTap: onTap,
        borderRadius: BorderRadius.circular(4),
        child: Container(
          padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 3),
          decoration: BoxDecoration(
            color: const Color(0xFF1E293B),
            borderRadius: BorderRadius.circular(4),
            border: Border.all(color: const Color(0xFF475569), width: 0.8),
          ),
          child: Text(
            label,
            style: const TextStyle(
              fontFamily: 'JetBrainsMono',
              fontSize: 11,
              color: Color(0xFFE2E8F0),
            ),
          ),
        ),
      ),
    );
  }
}
