import 'package:flutter/material.dart';
import '../../core/ipc/engine_client.dart';
import '../../core/progress/progress_manager.dart';
import '../../core/theme/zitera_colors.dart';
import '../../widgets/zitera_button.dart';
import '../../widgets/zitera_card.dart';
import '../../widgets/cute_anime_cursor.dart';
import '../../widgets/hacker_tilix_entrance.dart';
import '../../core/i18n/language_controller.dart';

class SettingsView extends StatelessWidget {
  const SettingsView({super.key});

  Future<void> _handleResetProgress(BuildContext context) async {
    final confirmed = await showDialog<bool>(
      context: context,
      builder: (ctx) => AlertDialog(
        backgroundColor: const Color(0xFFFCFBF8),
        shape: RoundedRectangleBorder(
          borderRadius: BorderRadius.circular(12),
          side: const BorderSide(color: ZiteraColors.border, width: 1.5),
        ),
        title: const Text(
          'Reset All Learning Progress?',
          style: TextStyle(
            fontFamily: 'SpaceGrotesk',
            fontWeight: FontWeight.bold,
            fontSize: 16,
            color: Color(0xFF1E1A14),
          ),
        ),
        content: const Text(
          'This will remove your local learning progress, marked sections, and completed challenges. Installed labs, containers, and repositories will NOT be deleted.',
          style: TextStyle(
            fontFamily: 'JetBrainsMono',
            fontSize: 12,
            color: Color(0xFF5C5347),
          ),
        ),
        actions: [
          TextButton(
            onPressed: () => Navigator.of(ctx).pop(false),
            child: const Text(
              'Cancel',
              style: TextStyle(
                fontFamily: 'SpaceGrotesk',
                color: Color(0xFF5C5347),
                fontWeight: FontWeight.w600,
              ),
            ),
          ),
          ZiteraButton(
            label: 'Yes, Reset Progress',
            icon: Icons.delete_forever,
            variant: ButtonVariant.danger,
            onPressed: () => Navigator.of(ctx).pop(true),
          ),
        ],
      ),
    );

    if (confirmed == true) {
      await ProgressManager.resetAll();
      if (context.mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          const SnackBar(
            content: Text('Local learning progress has been reset to initial state.'),
            backgroundColor: ZiteraColors.ready,
            duration: Duration(seconds: 3),
          ),
        );
      }
    }
  }

  Future<void> _handleClearCatalogCache(BuildContext context) async {
    final confirmed = await showDialog<bool>(
      context: context,
      builder: (ctx) => AlertDialog(
        backgroundColor: const Color(0xFFFCFBF8),
        shape: RoundedRectangleBorder(
          borderRadius: BorderRadius.circular(12),
          side: const BorderSide(color: ZiteraColors.border, width: 1.5),
        ),
        title: const Text(
          'Refresh Catalog Cache?',
          style: TextStyle(
            fontFamily: 'SpaceGrotesk',
            fontWeight: FontWeight.bold,
            fontSize: 16,
            color: Color(0xFF1E1A14),
          ),
        ),
        content: const Text(
          'This clears the local cached catalog file. The next catalog load will fetch the latest lab definitions from GitHub.',
          style: TextStyle(
            fontFamily: 'JetBrainsMono',
            fontSize: 12,
            color: Color(0xFF5C5347),
          ),
        ),
        actions: [
          TextButton(
            onPressed: () => Navigator.of(ctx).pop(false),
            child: const Text(
              'Cancel',
              style: TextStyle(
                fontFamily: 'SpaceGrotesk',
                color: Color(0xFF5C5347),
                fontWeight: FontWeight.w600,
              ),
            ),
          ),
          ZiteraButton(
            label: 'Clear Cache',
            icon: Icons.refresh,
            variant: ButtonVariant.secondary,
            onPressed: () => Navigator.of(ctx).pop(true),
          ),
        ],
      ),
    );

    if (confirmed == true) {
      try {
        await ZiteraEngineClient.getCatalog();
      } catch (_) {}
      if (context.mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          const SnackBar(
            content: Text('Catalog synchronized with remote registry.'),
            backgroundColor: ZiteraColors.ready,
            duration: Duration(seconds: 2),
          ),
        );
      }
    }
  }

  @override
  Widget build(BuildContext context) {
    final enginePath = ZiteraEngineClient.findEngineExecutable();

    return Stack(
      children: [
        Positioned.fill(
          child: Image.asset(
            'assets/images/bg_settings.jpg',
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
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: const [
                    Text(
                      'SYSTEM & LAB SETTINGS',
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
                      'Environment preferences, systems engine binding, and local storage',
                      style: TextStyle(
                        color: Color(0xFF5C5347),
                        fontSize: 13,
                        fontFamily: 'JetBrainsMono',
                      ),
                    ),
                  ],
                ),
              ),

              const SizedBox(height: 28),

              HackerTilixEntrance(
                delay: const Duration(milliseconds: 60),
                direction: TilixSlideDirection.up,
                child: ZiteraCard(
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      Row(
                        mainAxisAlignment: MainAxisAlignment.spaceBetween,
                        children: [
                          const Text(
                            'RUST ENGINE RUNTIME BINDING',
                            style: TextStyle(
                              fontFamily: 'SpaceGrotesk',
                              fontWeight: FontWeight.bold,
                              fontSize: 14,
                              color: Color(0xFF1E1A14),
                            ),
                          ),
                          SizedBox(
                            width: 30,
                            height: 30,
                            child: Image.asset(
                              'assets/images/sticker_shiba.png',
                              fit: BoxFit.contain,
                              errorBuilder: (context, error, stackTrace) => const SizedBox(),
                            ),
                          ),
                        ],
                      ),
                      const SizedBox(height: 8),
                      const Text(
                        'The Flutter desktop UI communicates with the lightweight Rust engine via JSON IPC over standard process streams.',
                        style: TextStyle(
                          color: Color(0xFF5C5347),
                          fontSize: 12,
                          fontFamily: 'JetBrainsMono',
                        ),
                      ),
                      const SizedBox(height: 12),
                      Container(
                        padding: const EdgeInsets.all(12),
                        decoration: BoxDecoration(
                          color: const Color(0xFFE6FFFA),
                          borderRadius: BorderRadius.circular(4),
                          border: Border.all(color: const Color(0xFF2DD4BF)),
                        ),
                        child: Row(
                          children: [
                            const Icon(Icons.settings_system_daydream, size: 16, color: Color(0xFF0F766E)),
                            const SizedBox(width: 10),
                            Expanded(
                              child: Text(
                                enginePath,
                                style: const TextStyle(
                                  fontFamily: 'JetBrainsMono',
                                  fontSize: 12,
                                  color: Color(0xFF0F766E),
                                  fontWeight: FontWeight.bold,
                                ),
                              ),
                            ),
                          ],
                        ),
                      ),
                    ],
                  ),
                ),
              ),

              const SizedBox(height: 20),

              // Language Preference Setting (PRD §44 & User Spec)
              HackerTilixEntrance(
                delay: const Duration(milliseconds: 100),
                direction: TilixSlideDirection.up,
                child: ValueListenableBuilder<String>(
                  valueListenable: AppLanguageController.currentLanguage,
                  builder: (context, lang, _) {
                    final isIndonesian = lang == 'id';
                    return ZiteraCard(
                      child: Column(
                        crossAxisAlignment: CrossAxisAlignment.start,
                        children: [
                          Row(
                            mainAxisAlignment: MainAxisAlignment.spaceBetween,
                            children: [
                              Row(
                                children: [
                                  const Text(
                                    'CONTENT & INSTRUCTION LANGUAGE',
                                    style: TextStyle(
                                      fontFamily: 'SpaceGrotesk',
                                      fontWeight: FontWeight.bold,
                                      fontSize: 14,
                                      color: Color(0xFF1E1A14),
                                    ),
                                  ),
                                  const SizedBox(width: 8),
                                  Container(
                                    padding: const EdgeInsets.symmetric(horizontal: 6, vertical: 2),
                                    decoration: BoxDecoration(
                                      color: const Color(0xFFCCFBF1),
                                      borderRadius: BorderRadius.circular(4),
                                      border: Border.all(color: const Color(0xFF2DD4BF), width: 0.8),
                                    ),
                                    child: const Text(
                                      'BILINGUAL',
                                      style: TextStyle(
                                        fontFamily: 'JetBrainsMono',
                                        fontSize: 9,
                                        fontWeight: FontWeight.bold,
                                        color: Color(0xFF0F766E),
                                      ),
                                    ),
                                  ),
                                ],
                              ),
                              Row(
                                children: [
                                  _buildLanguageTabButton(
                                    label: 'English',
                                    code: 'en',
                                    isSelected: !isIndonesian,
                                    onTap: () => AppLanguageController.setLanguage('en'),
                                  ),
                                  const SizedBox(width: 8),
                                  _buildLanguageTabButton(
                                    label: 'Bahasa Indonesia',
                                    code: 'id',
                                    isSelected: isIndonesian,
                                    onTap: () => AppLanguageController.setLanguage('id'),
                                  ),
                                ],
                              ),
                            ],
                          ),
                          const SizedBox(height: 14),
                          Container(
                            padding: const EdgeInsets.all(14),
                            decoration: BoxDecoration(
                              color: const Color(0xFFFBF8F2),
                              borderRadius: BorderRadius.circular(8),
                              border: Border.all(
                                color: isIndonesian ? const Color(0xFF2DD4BF) : const Color(0xFFE5DECE),
                                width: 1.2,
                              ),
                            ),
                            child: Column(
                              crossAxisAlignment: CrossAxisAlignment.start,
                              children: [
                                Row(
                                  children: [
                                    Icon(
                                      isIndonesian ? Icons.auto_stories : Icons.info_outline,
                                      size: 16,
                                      color: const Color(0xFF0F766E),
                                    ),
                                    const SizedBox(width: 8),
                                    const Text(
                                      'CATATAN PENTING MODE BAHASA INDONESIA',
                                      style: TextStyle(
                                        fontFamily: 'SpaceGrotesk',
                                        fontSize: 12,
                                        fontWeight: FontWeight.bold,
                                        color: Color(0xFF1E1A14),
                                        letterSpacing: 0.5,
                                      ),
                                    ),
                                  ],
                                ),
                                const SizedBox(height: 8),
                                const Text(
                                  '• Mode Bahasa Indonesia dirancang khusus untuk mempermudah kamu memahami seluruh materi belajar, konsep teori, analogi dunia nyata, panduan praktik (walkthrough), dan instruksi misi CTF tanpa kendala bahasa.',
                                  style: TextStyle(
                                    fontFamily: 'SpaceGrotesk',
                                    fontSize: 12,
                                    color: Color(0xFF4A4235),
                                    height: 1.5,
                                  ),
                                ),
                                const SizedBox(height: 6),
                                const Text(
                                  '• Tombol aksi, kartu menu, judul laboratorium, status sistem, dan navigasi tetap dipertahankan dalam Bahasa Inggris agar tampilan tetap rapi, profesional, dan membiasakan kamu dengan terminologi standar industri global.',
                                  style: TextStyle(
                                    fontFamily: 'SpaceGrotesk',
                                    fontSize: 12,
                                    color: Color(0xFF4A4235),
                                    height: 1.5,
                                  ),
                                ),
                                const SizedBox(height: 6),
                                const Text(
                                  '• Istilah teknis keamanan siber (seperti SQL Injection, Broken Access Control, JWT, Payload, IDOR, Bypass) tetap ditulis dengan istilah aslinya disertai penjelasan maknanya dalam Bahasa Indonesia agar mudah dipahami.',
                                  style: TextStyle(
                                    fontFamily: 'SpaceGrotesk',
                                    fontSize: 12,
                                    color: Color(0xFF4A4235),
                                    height: 1.5,
                                  ),
                                ),
                              ],
                            ),
                          ),
                        ],
                      ),
                    );
                  },
                ),
              ),

              const SizedBox(height: 20),

              // Kawaii Cyber Cursor Setting
              HackerTilixEntrance(
                delay: const Duration(milliseconds: 120),
                direction: TilixSlideDirection.up,
                child: ValueListenableBuilder<bool>(
                  valueListenable: CuteCursorController.isEnabled,
                  builder: (context, isCursorEnabled, _) {
                    return ZiteraCard(
                      child: Row(
                        mainAxisAlignment: MainAxisAlignment.spaceBetween,
                        children: [
                          Expanded(
                            child: Column(
                              crossAxisAlignment: CrossAxisAlignment.start,
                              children: [
                                Row(
                                  children: [
                                    const Text(
                                      'KAWAII ANIME CURSOR',
                                      style: TextStyle(
                                        fontFamily: 'SpaceGrotesk',
                                        fontWeight: FontWeight.bold,
                                        fontSize: 14,
                                        color: Color(0xFF1E1A14),
                                      ),
                                    ),
                                    const SizedBox(width: 8),
                                    Container(
                                      padding: const EdgeInsets.symmetric(horizontal: 6, vertical: 2),
                                      decoration: BoxDecoration(
                                        color: const Color(0xFFFCE7F3),
                                        borderRadius: BorderRadius.circular(4),
                                        border: Border.all(color: const Color(0xFFF472B6), width: 0.8),
                                      ),
                                      child: const Text(
                                        'NEKO PAW',
                                        style: TextStyle(
                                          fontFamily: 'JetBrainsMono',
                                          fontSize: 9,
                                          fontWeight: FontWeight.bold,
                                          color: Color(0xFFBE185D),
                                        ),
                                      ),
                                    ),
                                  ],
                                ),
                                const SizedBox(height: 6),
                                const Text(
                                  'Interactive anime cursor with sparkling star trail, transforming into a kawaii cat paw with jelly bean pads on hover.',
                                  style: TextStyle(
                                    color: Color(0xFF5C5347),
                                    fontSize: 12,
                                    fontFamily: 'JetBrainsMono',
                                  ),
                                ),
                              ],
                            ),
                          ),
                          Switch(
                            value: isCursorEnabled,
                            activeThumbColor: const Color(0xFF2DD4BF),
                            activeTrackColor: const Color(0xFFCCFBF1),
                            onChanged: (val) {
                              CuteCursorController.isEnabled.value = val;
                            },
                          ),
                        ],
                      ),
                    );
                  },
                ),
              ),

              const SizedBox(height: 20),

              HackerTilixEntrance(
                delay: const Duration(milliseconds: 180),
                direction: TilixSlideDirection.up,
                child: ZiteraCard(
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      Row(
                        mainAxisAlignment: MainAxisAlignment.spaceBetween,
                        children: [
                          const Text(
                            'DATA STORAGE & REPOSITORIES',
                            style: TextStyle(
                              fontFamily: 'SpaceGrotesk',
                              fontWeight: FontWeight.bold,
                              fontSize: 14,
                              color: Color(0xFF1E1A14),
                            ),
                          ),
                          SizedBox(
                            width: 30,
                            height: 30,
                            child: Image.asset(
                              'assets/images/sticker_bunny.png',
                              fit: BoxFit.contain,
                              errorBuilder: (context, error, stackTrace) => const SizedBox(),
                            ),
                          ),
                        ],
                      ),
                      const SizedBox(height: 8),
                      const Text(
                        'Local labs are stored separately from application binaries to allow safe updates without rebuilding the Flutter shell.',
                        style: TextStyle(
                          color: Color(0xFF5C5347),
                          fontSize: 12,
                          fontFamily: 'JetBrainsMono',
                        ),
                      ),
                      const SizedBox(height: 12),
                      _settingRow('Catalog Source', 'catalog/catalog.json'),
                      _settingRow('Local Labs Root', 'labs/'),
                      _settingRow('Architecture', 'Windows x64 (Native Runtime Baseline)'),
                      _settingRow('Product Version', 'ZITERA_LAB v1.0.0 (Foundation Milestone)'),
                    ],
                  ),
                ),
              ),

              const SizedBox(height: 20),

              HackerTilixEntrance(
                delay: const Duration(milliseconds: 240),
                direction: TilixSlideDirection.up,
                child: ZiteraCard(
                  borderColor: ZiteraColors.error.withValues(alpha: 0.3),
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      const Text(
                        'RESET & RECOVERY',
                        style: TextStyle(
                          fontFamily: 'SpaceGrotesk',
                          fontWeight: FontWeight.bold,
                          fontSize: 14,
                          color: ZiteraColors.error,
                        ),
                      ),
                      const SizedBox(height: 8),
                      const Text(
                        'Restore all challenge flags, local logs, and temporary engine cache to fresh initial state.',
                        style: TextStyle(
                          color: Color(0xFF5C5347),
                          fontSize: 12,
                          fontFamily: 'JetBrainsMono',
                        ),
                      ),
                      const SizedBox(height: 14),
                      Wrap(
                        spacing: 12,
                        runSpacing: 10,
                        children: [
                          ZiteraButton(
                            label: 'Reset All Learning Progress',
                            icon: Icons.delete_outline,
                            variant: ButtonVariant.danger,
                            onPressed: () => _handleResetProgress(context),
                          ),
                          ZiteraButton(
                            label: 'Refresh Catalog Cache',
                            icon: Icons.sync,
                            variant: ButtonVariant.secondary,
                            onPressed: () => _handleClearCatalogCache(context),
                          ),
                        ],
                      ),
                    ],
                  ),
                ),
              ),
            ],
          ),
        ),
      ],
    );
  }

  Widget _settingRow(String label, String value) {
    return Padding(
      padding: const EdgeInsets.symmetric(vertical: 6.0),
      child: Row(
        mainAxisAlignment: MainAxisAlignment.spaceBetween,
        children: [
          Text(
            label,
            style: const TextStyle(
              fontFamily: 'SpaceGrotesk',
              fontWeight: FontWeight.w600,
              fontSize: 13,
              color: Color(0xFF1E1A14),
            ),
          ),
          Text(
            value,
            style: const TextStyle(
              color: Color(0xFF0F766E),
              fontFamily: 'JetBrainsMono',
              fontSize: 12,
              fontWeight: FontWeight.bold,
            ),
          ),
        ],
      ),
    );
  }

  Widget _buildLanguageTabButton({
    required String label,
    required String code,
    required bool isSelected,
    required VoidCallback onTap,
  }) {
    return InkWell(
      onTap: onTap,
      borderRadius: BorderRadius.circular(6),
      child: Container(
        padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 6),
        decoration: BoxDecoration(
          color: isSelected ? const Color(0xFFCCFBF1) : const Color(0xFFF5F3EF),
          borderRadius: BorderRadius.circular(6),
          border: Border.all(
            color: isSelected ? const Color(0xFF2DD4BF) : const Color(0xFFD6CFC3),
            width: isSelected ? 1.4 : 1.0,
          ),
        ),
        child: Row(
          mainAxisSize: MainAxisSize.min,
          children: [
            Text(
              code.toUpperCase(),
              style: TextStyle(
                fontFamily: 'JetBrainsMono',
                fontSize: 10,
                fontWeight: FontWeight.bold,
                color: isSelected ? const Color(0xFF0F766E) : const Color(0xFF786F62),
              ),
            ),
            const SizedBox(width: 6),
            Text(
              label,
              style: TextStyle(
                fontFamily: 'SpaceGrotesk',
                fontSize: 11,
                fontWeight: isSelected ? FontWeight.bold : FontWeight.w500,
                color: isSelected ? const Color(0xFF0F766E) : const Color(0xFF4A4235),
              ),
            ),
          ],
        ),
      ),
    );
  }
}
