import 'package:flutter/material.dart';
import '../core/auth/auth_service.dart';
import '../core/auth/avatar_registry.dart';
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
  String _selectedAvatarId = 'zeta';
  String _avatarCategoryFilter = 'ALL';
  bool _isChangingAvatar = false;

  final _usernameController = TextEditingController();
  final _passwordController = TextEditingController();
  final _confirmPasswordController = TextEditingController();

  @override
  void initState() {
    super.initState();
    _isRegister = widget.initialIsRegister;
    final current = AuthService.currentUser;
    if (current != null && current.avatarId.isNotEmpty) {
      _selectedAvatarId = current.avatarId;
    }
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
          ? await AuthService.register(username, password, avatarId: _selectedAvatarId)
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
        width: 480,
        constraints: const BoxConstraints(maxHeight: 700),
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
        child: SingleChildScrollView(
          child: currentUser != null ? _buildProfileView(currentUser) : _buildAuthForm(),
        ),
      ),
    );
  }

  Widget _buildProfileView(UserModel user) {
    final avatar = AvatarRegistry.getById(user.avatarId);

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
                width: 56,
                height: 56,
                decoration: BoxDecoration(
                  shape: BoxShape.circle,
                  border: Border.all(color: const Color(0xFF0D9488), width: 2),
                  boxShadow: [
                    BoxShadow(
                      color: const Color(0xFF0D9488).withValues(alpha: 0.25),
                      blurRadius: 8,
                    ),
                  ],
                ),
                child: ClipOval(
                  child: AvatarRegistry.buildAvatarImage(user.avatarId),
                ),
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
                      'AVATAR: ${avatar.name} • ${avatar.expression}',
                      style: const TextStyle(
                        fontFamily: 'JetBrainsMono',
                        fontSize: 11,
                        fontWeight: FontWeight.bold,
                        color: Color(0xFF0F766E),
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
        const SizedBox(height: 14),

        // Avatar change / picker toggle
        if (_isChangingAvatar) ...[
          _buildAvatarPicker(
            selectedId: _selectedAvatarId,
            onSelected: (id) => setState(() => _selectedAvatarId = id),
          ),
          const SizedBox(height: 12),
          Row(
            children: [
              Expanded(
                child: OutlinedButton(
                  style: OutlinedButton.styleFrom(
                    foregroundColor: const Color(0xFF78716C),
                    padding: const EdgeInsets.symmetric(vertical: 10),
                    shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(8)),
                  ),
                  onPressed: () => setState(() => _isChangingAvatar = false),
                  child: const Text(
                    'CANCEL',
                    style: TextStyle(fontFamily: 'JetBrainsMono', fontSize: 11, fontWeight: FontWeight.bold),
                  ),
                ),
              ),
              const SizedBox(width: 10),
              Expanded(
                child: ElevatedButton(
                  style: ElevatedButton.styleFrom(
                    backgroundColor: const Color(0xFF0D9488),
                    foregroundColor: Colors.white,
                    padding: const EdgeInsets.symmetric(vertical: 10),
                    shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(8)),
                  ),
                  onPressed: () async {
                    setState(() => _isLoading = true);
                    final ok = await AuthService.updateAvatar(_selectedAvatarId);
                    if (mounted) {
                      setState(() {
                        _isLoading = false;
                        _isChangingAvatar = false;
                        if (ok) _successMessage = 'Avatar updated successfully!';
                      });
                    }
                  },
                  child: const Text(
                    'SAVE AVATAR',
                    style: TextStyle(fontFamily: 'JetBrainsMono', fontSize: 11, fontWeight: FontWeight.bold),
                  ),
                ),
              ),
            ],
          ),
        ] else ...[
          SizedBox(
            width: double.infinity,
            child: OutlinedButton.icon(
              style: OutlinedButton.styleFrom(
                foregroundColor: const Color(0xFF0D9488),
                side: const BorderSide(color: Color(0xFF0D9488), width: 1.2),
                padding: const EdgeInsets.symmetric(vertical: 10),
                shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(8)),
              ),
              onPressed: () => setState(() {
                _selectedAvatarId = user.avatarId;
                _isChangingAvatar = true;
              }),
              icon: const Icon(Icons.palette_outlined, size: 16),
              label: const Text(
                'CHANGE OPERATIVE AVATAR',
                style: TextStyle(fontFamily: 'JetBrainsMono', fontSize: 11, fontWeight: FontWeight.bold),
              ),
            ),
          ),
        ],
        const SizedBox(height: 16),

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
          const SizedBox(height: 14),
          const Text(
            'OPERATIVE AVATAR // IDENTITY',
            style: TextStyle(
              fontFamily: 'JetBrainsMono',
              fontSize: 11,
              fontWeight: FontWeight.bold,
              color: Color(0xFF44403C),
            ),
          ),
          const SizedBox(height: 6),
          _buildAvatarPicker(
            selectedId: _selectedAvatarId,
            onSelected: (id) => setState(() => _selectedAvatarId = id),
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

  Widget _avatarFilterPill(String label, String category) {
    final isSelected = _avatarCategoryFilter == category;
    return InkWell(
      onTap: () => setState(() => _avatarCategoryFilter = category),
      borderRadius: BorderRadius.circular(4),
      child: Container(
        padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 3),
        decoration: BoxDecoration(
          color: isSelected ? const Color(0xFF0D9488) : Colors.white,
          borderRadius: BorderRadius.circular(4),
          border: Border.all(
            color: isSelected ? const Color(0xFF0D9488) : ZiteraColors.border,
          ),
        ),
        child: Text(
          label,
          style: TextStyle(
            fontFamily: 'JetBrainsMono',
            fontSize: 9,
            fontWeight: isSelected ? FontWeight.bold : FontWeight.w500,
            color: isSelected ? Colors.white : const Color(0xFF5C5347),
          ),
        ),
      ),
    );
  }

  Widget _buildAvatarPicker({
    required String selectedId,
    required ValueChanged<String> onSelected,
  }) {
    final selectedAvatar = AvatarRegistry.getById(selectedId);
    final filteredAvatars = AvatarRegistry.all.where((a) {
      if (_avatarCategoryFilter == 'GIRLS') return a.gender == 'Female';
      if (_avatarCategoryFilter == 'BOYS') return a.gender == 'Male';
      if (_avatarCategoryFilter == 'CLASSIC') return a.gender == 'Operator' || a.gender == 'Mascot';
      return true;
    }).toList();

    return Container(
      padding: const EdgeInsets.all(12),
      decoration: BoxDecoration(
        color: ZiteraColors.surface,
        borderRadius: BorderRadius.circular(10),
        border: Border.all(color: ZiteraColors.border),
      ),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        mainAxisSize: MainAxisSize.min,
        children: [
          // Header / Active Preview
          Row(
            children: [
              Container(
                width: 48,
                height: 48,
                decoration: BoxDecoration(
                  shape: BoxShape.circle,
                  border: Border.all(color: const Color(0xFF0D9488), width: 2),
                  boxShadow: [
                    BoxShadow(
                      color: const Color(0xFF0D9488).withValues(alpha: 0.3),
                      blurRadius: 8,
                    ),
                  ],
                ),
                child: ClipOval(
                  child: AvatarRegistry.buildAvatarImage(selectedId),
                ),
              ),
              const SizedBox(width: 12),
              Expanded(
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Row(
                      children: [
                        Text(
                          selectedAvatar.name,
                          style: const TextStyle(
                            fontFamily: 'SpaceGrotesk',
                            fontSize: 14,
                            fontWeight: FontWeight.bold,
                            color: Color(0xFF1E1A14),
                          ),
                        ),
                        const SizedBox(width: 6),
                        Container(
                          padding: const EdgeInsets.symmetric(horizontal: 5, vertical: 1.5),
                          decoration: BoxDecoration(
                            color: const Color(0xFFCCFBF1),
                            borderRadius: BorderRadius.circular(4),
                          ),
                          child: Text(
                            selectedAvatar.gender.toUpperCase(),
                            style: const TextStyle(
                              fontFamily: 'JetBrainsMono',
                              fontSize: 9,
                              fontWeight: FontWeight.bold,
                              color: Color(0xFF0F766E),
                            ),
                          ),
                        ),
                      ],
                    ),
                    const SizedBox(height: 2),
                    Text(
                      selectedAvatar.expression,
                      style: const TextStyle(
                        fontFamily: 'JetBrainsMono',
                        fontSize: 10,
                        color: Color(0xFF78716C),
                      ),
                    ),
                  ],
                ),
              ),
            ],
          ),
          const SizedBox(height: 10),

          // Category filter pills
          Row(
            children: [
              _avatarFilterPill('ALL (14)', 'ALL'),
              const SizedBox(width: 6),
              _avatarFilterPill('GIRLS (6)', 'GIRLS'),
              const SizedBox(width: 6),
              _avatarFilterPill('BOYS (6)', 'BOYS'),
              const SizedBox(width: 6),
              _avatarFilterPill('CLASSIC (2)', 'CLASSIC'),
            ],
          ),
          const SizedBox(height: 10),

          // Avatars grid
          Wrap(
            spacing: 8,
            runSpacing: 8,
            children: filteredAvatars.map((a) {
              final isSelected = a.id == selectedId;
              return Tooltip(
                message: '${a.name} (${a.expression})',
                child: InkWell(
                  onTap: () => onSelected(a.id),
                  borderRadius: BorderRadius.circular(8),
                  child: AnimatedContainer(
                    duration: const Duration(milliseconds: 150),
                    padding: const EdgeInsets.all(3),
                    decoration: BoxDecoration(
                      borderRadius: BorderRadius.circular(8),
                      color: isSelected ? const Color(0xFFCCFBF1) : Colors.white,
                      border: Border.all(
                        color: isSelected ? const Color(0xFF0D9488) : ZiteraColors.border,
                        width: isSelected ? 2 : 1,
                      ),
                      boxShadow: isSelected
                          ? [
                              BoxShadow(
                                color: const Color(0xFF0D9488).withValues(alpha: 0.25),
                                blurRadius: 6,
                              )
                            ]
                          : null,
                    ),
                    child: Stack(
                      children: [
                        ClipRRect(
                          borderRadius: BorderRadius.circular(6),
                          child: AvatarRegistry.buildAvatarImage(
                            a.id,
                            width: 44,
                            height: 44,
                          ),
                        ),
                        if (isSelected)
                          Positioned(
                            bottom: 0,
                            right: 0,
                            child: Container(
                              padding: const EdgeInsets.all(2),
                              decoration: const BoxDecoration(
                                color: Color(0xFF0D9488),
                                shape: BoxShape.circle,
                              ),
                              child: const Icon(Icons.check, size: 10, color: Colors.white),
                            ),
                          ),
                      ],
                    ),
                  ),
                ),
              );
            }).toList(),
          ),
        ],
      ),
    );
  }
}
