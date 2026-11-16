import 'package:flutter/material.dart';
import '../core/auth/auth_service.dart';
import '../core/scoreboard/scoreboard_service.dart';
import '../core/theme/zitera_colors.dart';

/// Modal dialog for Operative Authentication (Login / Register / Profile Management).
class AuthDialog extends StatefulWidget {
  final bool initialIsRegister;

  const AuthDialog({
    super.key,
    this.initialIsRegister = false,
  });

  static Future<void> show(BuildContext context, {bool isRegister = false}) {
    return showDialog<void>(
      context: context,
      barrierColor: Colors.black.withValues(alpha: 0.5),
      builder: (ctx) => AuthDialog(initialIsRegister: isRegister),
    );
  }

  @override
  State<AuthDialog> createState() => _AuthDialogState();
}

class _AuthDialogState extends State<AuthDialog> {
  late bool _isRegister;
  bool _obscurePassword = true;
  bool _obscureConfirm = true;
  bool _isLoading = false;
  String? _errorMessage;
  String? _successMessage;

  final _usernameController = TextEditingController();
  final _passwordController = TextEditingController();
  final _confirmPasswordController = TextEditingController();

  @override
  void initState() {
    super.initState();
    _isRegister = widget.initialIsRegister;
  }

  @override
  void dispose() {
    _usernameController.dispose();
    _passwordController.dispose();
    _confirmPasswordController.dispose();
    super.dispose();
  }

  Future<void> _handleSubmit() async {
    final username = _usernameController.text.trim();
    final password = _passwordController.text;

    if (username.isEmpty || password.isEmpty) {
      setState(() => _errorMessage = 'Please fill in all required fields.');
      return;
    }

    if (_isRegister) {
      final confirm = _confirmPasswordController.text;
      if (password != confirm) {
        setState(() => _errorMessage = 'Passwords do not match.');
        return;
      }
    }

    setState(() {
      _isLoading = true;
      _errorMessage = null;
      _successMessage = null;
    });

    try {
      final res = _isRegister
          ? await AuthService.register(username, password)
          : await AuthService.login(username, password);

      if (!mounted) return;

      if (res.success) {
        setState(() {
          _isLoading = false;
          _successMessage = res.message;
        });

        // Sync local solves to Turso
        ScoreboardService.syncLocalSolvesToRemote();

        await Future.delayed(const Duration(milliseconds: 600));
        if (mounted) {
          Navigator.of(context).pop();
        }
      } else {
        setState(() {
          _isLoading = false;
          _errorMessage = res.message;
        });
      }
    } catch (e) {
      if (mounted) {
        setState(() {
          _isLoading = false;
          _errorMessage = 'An unexpected error occurred: $e';
        });
      }
    }
  }

  Future<void> _handleLogout() async {
    setState(() => _isLoading = true);
    await AuthService.logout();
    if (mounted) {
      setState(() => _isLoading = false);
      Navigator.of(context).pop();
    }
  }

  @override
  Widget build(BuildContext context) {
    final currentUser = AuthService.currentUser;

    return Dialog(
      backgroundColor: Colors.transparent,
      insetPadding: const EdgeInsets.symmetric(horizontal: 24, vertical: 24),
      child: Container(
        width: 440,
        padding: const EdgeInsets.all(24),
        decoration: BoxDecoration(
          color: ZiteraColors.card,
          borderRadius: BorderRadius.circular(14),
          border: Border.all(color: ZiteraColors.borderDark, width: 1.5),
          boxShadow: [
            BoxShadow(
              color: Colors.black.withValues(alpha: 0.16),
              blurRadius: 28,
              offset: const Offset(0, 10),
            ),
          ],
        ),
        child: currentUser != null ? _buildProfileView(currentUser) : _buildAuthForm(),
      ),
    );
  }

  Widget _buildProfileView(UserModel user) {
    return Column(
      mainAxisSize: MainAxisSize.min,
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        // Header
        Row(
          mainAxisAlignment: MainAxisAlignment.spaceBetween,
          children: [
            Row(
              children: [
                Container(
                  padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 4),
                  decoration: BoxDecoration(
                    color: ZiteraColors.statLabsBg,
                    borderRadius: BorderRadius.circular(4),
                    border: Border.all(color: ZiteraColors.statLabsBorder),
                  ),
                  child: const Text(
                    'OPERATIVE PROFILE',
                    style: TextStyle(
                      fontFamily: 'JetBrainsMono',
                      fontSize: 11,
                      fontWeight: FontWeight.bold,
                      color: ZiteraColors.statLabsText,
                    ),
                  ),
                ),
                const SizedBox(width: 8),
                Container(
                  width: 8,
                  height: 8,
                  decoration: const BoxDecoration(
                    color: ZiteraColors.ready,
                    shape: BoxShape.circle,
                  ),
                ),
              ],
            ),
            IconButton(
              icon: const Icon(Icons.close, size: 20, color: Color(0xFF5C5347)),
              splashRadius: 18,
              onPressed: () => Navigator.of(context).pop(),
            ),
          ],
        ),
        const SizedBox(height: 18),

        // User Identity Card
        Container(
          padding: const EdgeInsets.all(16),
          decoration: BoxDecoration(
            color: ZiteraColors.surface,
            borderRadius: BorderRadius.circular(10),
            border: Border.all(color: ZiteraColors.border),
          ),
          child: Row(
            children: [
              Container(
                width: 52,
                height: 52,
                decoration: BoxDecoration(
                  shape: BoxShape.circle,
                  color: ZiteraColors.statToolsBg,
                  border: Border.all(color: ZiteraColors.cyanLight, width: 2),
                ),
                child: const Icon(Icons.shield_outlined, color: ZiteraColors.cyan, size: 28),
              ),
              const SizedBox(width: 14),
              Expanded(
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Text(
                      user.username,
                      style: const TextStyle(
                        fontFamily: 'SpaceGrotesk',
                        fontSize: 18,
                        fontWeight: FontWeight.bold,
                        color: Color(0xFF1E1A14),
                      ),
                    ),
                    const SizedBox(height: 3),
                    Text(
                      'ID: ${user.id}',
                      style: const TextStyle(
                        fontFamily: 'JetBrainsMono',
                        fontSize: 11,
                        color: Color(0xFF78716C),
                      ),
                      overflow: TextOverflow.ellipsis,
                    ),
                    const SizedBox(height: 2),
                    Text(
                      'TURSO SYNC: ACTIVE',
                      style: const TextStyle(
                        fontFamily: 'JetBrainsMono',
                        fontSize: 10,
                        fontWeight: FontWeight.bold,
                        color: ZiteraColors.ready,
                        letterSpacing: 0.5,
                      ),
                    ),
                  ],
                ),
              ),
            ],
          ),
        ),
        const SizedBox(height: 20),

        // Actions
        Row(
          children: [
            Expanded(
              child: OutlinedButton.icon(
                style: OutlinedButton.styleFrom(
                  foregroundColor: ZiteraColors.coral,
                  side: const BorderSide(color: ZiteraColors.coral),
                  padding: const EdgeInsets.symmetric(vertical: 12),
                  shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(8)),
                ),
                onPressed: _isLoading ? null : _handleLogout,
                icon: const Icon(Icons.logout, size: 16),
                label: const Text(
                  'LOGOUT',
                  style: TextStyle(
                    fontFamily: 'JetBrainsMono',
                    fontSize: 12,
                    fontWeight: FontWeight.bold,
                  ),
                ),
              ),
            ),
            const SizedBox(width: 12),
            Expanded(
              child: ElevatedButton.icon(
                style: ElevatedButton.styleFrom(
                  backgroundColor: ZiteraColors.cyan,
                  foregroundColor: Colors.white,
                  padding: const EdgeInsets.symmetric(vertical: 12),
                  shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(8)),
                ),
                onPressed: () => Navigator.of(context).pop(),
                icon: const Icon(Icons.check, size: 16),
                label: const Text(
                  'DONE',
                  style: TextStyle(
                    fontFamily: 'JetBrainsMono',
                    fontSize: 12,
                    fontWeight: FontWeight.bold,
                  ),
                ),
              ),
            ),
          ],
        ),
      ],
    );
  }

  Widget _buildAuthForm() {
    return Column(
      mainAxisSize: MainAxisSize.min,
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        // Header & Close
        Row(
          mainAxisAlignment: MainAxisAlignment.spaceBetween,
          children: [
            Row(
              children: [
                Container(
                  padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 4),
                  decoration: BoxDecoration(
                    color: ZiteraColors.statChallengesBg,
                    borderRadius: BorderRadius.circular(4),
                    border: Border.all(color: ZiteraColors.statChallengesBorder),
                  ),
                  child: const Text(
                    'ZITERA AUTH // TURSO',
                    style: TextStyle(
                      fontFamily: 'JetBrainsMono',
                      fontSize: 11,
                      fontWeight: FontWeight.bold,
                      color: ZiteraColors.statChallengesText,
                    ),
                  ),
                ),
              ],
            ),
            IconButton(
              icon: const Icon(Icons.close, size: 20, color: Color(0xFF5C5347)),
              splashRadius: 18,
              onPressed: () => Navigator.of(context).pop(),
            ),
          ],
        ),
        const SizedBox(height: 14),

        // Title
        Text(
          _isRegister ? 'Register Operative Account' : 'Login to Zitera Lab',
          style: const TextStyle(
            fontFamily: 'SpaceGrotesk',
            fontSize: 20,
            fontWeight: FontWeight.bold,
            color: Color(0xFF1E1A14),
          ),
        ),
        const SizedBox(height: 4),
        Text(
          _isRegister
              ? 'Create a decentralized profile to record solves on the live CTF scoreboard.'
              : 'Sign in with your credentials to synchronize progress with the live scoreboard.',
          style: const TextStyle(
            fontFamily: 'JetBrainsMono',
            fontSize: 11,
            color: Color(0xFF78716C),
          ),
        ),
        const SizedBox(height: 16),

        // Mode Switcher Tabs
        Container(
          padding: const EdgeInsets.all(4),
          decoration: BoxDecoration(
            color: ZiteraColors.surface,
            borderRadius: BorderRadius.circular(8),
            border: Border.all(color: ZiteraColors.border),
          ),
          child: Row(
            children: [
              Expanded(
                child: _modeTab(
                  label: 'SIGN IN',
                  selected: !_isRegister,
                  onTap: () => setState(() {
                    _isRegister = false;
                    _errorMessage = null;
                  }),
                ),
              ),
              Expanded(
                child: _modeTab(
                  label: 'CREATE ACCOUNT',
                  selected: _isRegister,
                  onTap: () => setState(() {
                    _isRegister = true;
                    _errorMessage = null;
                  }),
                ),
              ),
            ],
          ),
        ),
        const SizedBox(height: 16),

        // Feedback messages
        if (_errorMessage != null) ...[
          Container(
            padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 8),
            decoration: BoxDecoration(
              color: const Color(0xFFFEE2E2),
              borderRadius: BorderRadius.circular(6),
              border: Border.all(color: const Color(0xFFFCA5A5)),
            ),
            child: Row(
              children: [
                const Icon(Icons.error_outline, size: 16, color: Color(0xFFDC2626)),
                const SizedBox(width: 8),
                Expanded(
                  child: Text(
                    _errorMessage!,
                    style: const TextStyle(
                      fontFamily: 'JetBrainsMono',
                      fontSize: 11,
                      color: Color(0xFF991B1B),
                    ),
                  ),
                ),
              ],
            ),
          ),
          const SizedBox(height: 12),
        ],
        if (_successMessage != null) ...[
          Container(
            padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 8),
            decoration: BoxDecoration(
              color: const Color(0xFFDCFCE7),
              borderRadius: BorderRadius.circular(6),
              border: Border.all(color: const Color(0xFF86EFAC)),
            ),
            child: Row(
              children: [
                const Icon(Icons.check_circle_outline, size: 16, color: Color(0xFF16A34A)),
                const SizedBox(width: 8),
                Expanded(
                  child: Text(
                    _successMessage!,
                    style: const TextStyle(
                      fontFamily: 'JetBrainsMono',
                      fontSize: 11,
                      fontWeight: FontWeight.bold,
                      color: Color(0xFF166534),
                    ),
                  ),
                ),
              ],
            ),
          ),
          const SizedBox(height: 12),
        ],

        // Username Field
        const Text(
          'OPERATIVE USERNAME',
          style: TextStyle(
            fontFamily: 'JetBrainsMono',
            fontSize: 11,
            fontWeight: FontWeight.bold,
            color: Color(0xFF44403C),
          ),
        ),
        const SizedBox(height: 6),
        TextField(
          controller: _usernameController,
          enabled: !_isLoading,
          style: const TextStyle(fontFamily: 'JetBrainsMono', fontSize: 13),
          decoration: _inputDecoration(
            hintText: 'e.g. ghost_operative',
            icon: Icons.person_outline,
          ),
        ),
        const SizedBox(height: 12),

        // Password Field
        const Text(
          'ACCESS PASSCODE',
          style: TextStyle(
            fontFamily: 'JetBrainsMono',
            fontSize: 11,
            fontWeight: FontWeight.bold,
            color: Color(0xFF44403C),
          ),
        ),
        const SizedBox(height: 6),
        TextField(
          controller: _passwordController,
          enabled: !_isLoading,
          obscureText: _obscurePassword,
          style: const TextStyle(fontFamily: 'JetBrainsMono', fontSize: 13),
          decoration: _inputDecoration(
            hintText: '••••••••',
            icon: Icons.lock_outline,
            suffix: IconButton(
              icon: Icon(
                _obscurePassword ? Icons.visibility_outlined : Icons.visibility_off_outlined,
                size: 18,
                color: const Color(0xFF78716C),
              ),
              onPressed: () => setState(() => _obscurePassword = !_obscurePassword),
            ),
          ),
          onSubmitted: (_) {
            if (!_isRegister) _handleSubmit();
          },
        ),

        // Confirm Password Field (Register mode only)
        if (_isRegister) ...[
          const SizedBox(height: 12),
          const Text(
            'CONFIRM PASSCODE',
            style: TextStyle(
              fontFamily: 'JetBrainsMono',
              fontSize: 11,
              fontWeight: FontWeight.bold,
              color: Color(0xFF44403C),
            ),
          ),
          const SizedBox(height: 6),
          TextField(
            controller: _confirmPasswordController,
            enabled: !_isLoading,
            obscureText: _obscureConfirm,
            style: const TextStyle(fontFamily: 'JetBrainsMono', fontSize: 13),
            decoration: _inputDecoration(
              hintText: '••••••••',
              icon: Icons.lock_reset,
              suffix: IconButton(
                icon: Icon(
                  _obscureConfirm ? Icons.visibility_outlined : Icons.visibility_off_outlined,
                  size: 18,
                  color: const Color(0xFF78716C),
                ),
                onPressed: () => setState(() => _obscureConfirm = !_obscureConfirm),
              ),
            ),
            onSubmitted: (_) => _handleSubmit(),
          ),
        ],
        const SizedBox(height: 20),

        // Submit Button
        SizedBox(
          width: double.infinity,
          height: 44,
          child: ElevatedButton(
            style: ElevatedButton.styleFrom(
              backgroundColor: const Color(0xFF0D9488),
              foregroundColor: Colors.white,
              shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(8)),
              elevation: 0,
            ),
            onPressed: _isLoading ? null : _handleSubmit,
            child: _isLoading
                ? const SizedBox(
                    width: 20,
                    height: 20,
                    child: CircularProgressIndicator(
                      strokeWidth: 2,
                      valueColor: AlwaysStoppedAnimation<Color>(Colors.white),
                    ),
                  )
                : Text(
                    _isRegister ? 'REGISTER & ENTER LAB' : 'SIGN IN OPERATIVE',
                    style: const TextStyle(
                      fontFamily: 'JetBrainsMono',
                      fontSize: 12,
                      fontWeight: FontWeight.bold,
                      letterSpacing: 0.5,
                    ),
                  ),
          ),
        ),
      ],
    );
  }

  Widget _modeTab({
    required String label,
    required bool selected,
    required VoidCallback onTap,
  }) {
    return GestureDetector(
      onTap: onTap,
      child: AnimatedContainer(
        duration: const Duration(milliseconds: 150),
        padding: const EdgeInsets.symmetric(vertical: 8),
        decoration: BoxDecoration(
          color: selected ? Colors.white : Colors.transparent,
          borderRadius: BorderRadius.circular(6),
          boxShadow: selected
              ? [
                  BoxShadow(
                    color: Colors.black.withValues(alpha: 0.06),
                    blurRadius: 4,
                    offset: const Offset(0, 1),
                  ),
                ]
              : null,
        ),
        child: Center(
          child: Text(
            label,
            style: TextStyle(
              fontFamily: 'JetBrainsMono',
              fontSize: 11,
              fontWeight: selected ? FontWeight.bold : FontWeight.w500,
              color: selected ? const Color(0xFF0F766E) : const Color(0xFF78716C),
            ),
          ),
        ),
      ),
    );
  }

  InputDecoration _inputDecoration({
    required String hintText,
    required IconData icon,
    Widget? suffix,
  }) {
    return InputDecoration(
      hintText: hintText,
      hintStyle: const TextStyle(
        fontFamily: 'JetBrainsMono',
        fontSize: 12,
        color: Color(0xFFA8A29E),
      ),
      prefixIcon: Icon(icon, size: 18, color: const Color(0xFF78716C)),
      suffixIcon: suffix,
      filled: true,
      fillColor: Colors.white,
      contentPadding: const EdgeInsets.symmetric(horizontal: 12, vertical: 12),
      border: OutlineInputBorder(
        borderRadius: BorderRadius.circular(8),
        borderSide: const BorderSide(color: ZiteraColors.border),
      ),
      enabledBorder: OutlineInputBorder(
        borderRadius: BorderRadius.circular(8),
        borderSide: const BorderSide(color: ZiteraColors.border),
      ),
      focusedBorder: OutlineInputBorder(
        borderRadius: BorderRadius.circular(8),
        borderSide: const BorderSide(color: Color(0xFF0D9488), width: 1.5),
      ),
    );
  }
}
