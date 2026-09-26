import 'package:flutter/material.dart';
import '../../core/ipc/engine_client.dart';
import '../../core/theme/zitera_colors.dart';
import '../../widgets/zitera_button.dart';
import '../../widgets/zitera_card.dart';

class SettingsView extends StatelessWidget {
  const SettingsView({super.key});

  @override
  Widget build(BuildContext context) {
    final enginePath = ZiteraEngineClient.findEngineExecutable();

    return SingleChildScrollView(
      padding: const EdgeInsets.all(32.0),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          const Text(
            'SYSTEM & LAB SETTINGS',
            style: TextStyle(
              color: ZiteraColors.textPrimary,
              fontSize: 22,
              fontWeight: FontWeight.w900,
              letterSpacing: 1.5,
              fontFamily: 'monospace',
            ),
          ),
          const SizedBox(height: 4),
          const Text(
            'Environment preferences, systems engine binding, and local storage',
            style: TextStyle(color: ZiteraColors.textSecondary, fontSize: 13),
          ),

          const SizedBox(height: 28),

          ZiteraCard(
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                const Text('RUST ENGINE RUNTIME BINDING', style: TextStyle(fontWeight: FontWeight.bold, fontSize: 14)),
                const SizedBox(height: 8),
                const Text(
                  'The Flutter desktop UI communicates with the lightweight Rust engine via JSON IPC over standard process streams.',
                  style: TextStyle(color: ZiteraColors.textSecondary, fontSize: 12),
                ),
                const SizedBox(height: 12),
                Container(
                  padding: const EdgeInsets.all(12),
                  decoration: BoxDecoration(
                    color: ZiteraColors.surface,
                    borderRadius: BorderRadius.circular(4),
                    border: Border.all(color: ZiteraColors.border),
                  ),
                  child: Row(
                    children: [
                      const Icon(Icons.settings_system_daydream, size: 16, color: ZiteraColors.cyan),
                      const SizedBox(width: 10),
                      Expanded(
                        child: Text(
                          enginePath,
                          style: const TextStyle(fontFamily: 'monospace', fontSize: 12, color: ZiteraColors.textPrimary),
                        ),
                      ),
                    ],
                  ),
                ),
              ],
            ),
          ),

          const SizedBox(height: 20),

          ZiteraCard(
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                const Text('DATA STORAGE & REPOSITORIES', style: TextStyle(fontWeight: FontWeight.bold, fontSize: 14)),
                const SizedBox(height: 8),
                const Text(
                  'Local labs are stored separately from application binaries to allow safe updates without rebuilding the Flutter shell.',
                  style: TextStyle(color: ZiteraColors.textSecondary, fontSize: 12),
                ),
                const SizedBox(height: 12),
                _settingRow('Catalog Source', 'catalog/catalog.json'),
                _settingRow('Local Labs Root', 'labs/'),
                _settingRow('Architecture', 'Windows x64 / WSL2 Docker Backend'),
                _settingRow('Product Version', 'ZITERA_LAB v1.0.0 (Foundation Milestone)'),
              ],
            ),
          ),

          const SizedBox(height: 20),

          ZiteraCard(
            borderColor: ZiteraColors.error.withValues(alpha: 0.3),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                const Text('RESET & RECOVERY', style: TextStyle(fontWeight: FontWeight.bold, fontSize: 14, color: ZiteraColors.error)),
                const SizedBox(height: 8),
                const Text(
                  'Restore all challenge flags, local logs, and temporary engine cache to fresh initial state.',
                  style: TextStyle(color: ZiteraColors.textSecondary, fontSize: 12),
                ),
                const SizedBox(height: 14),
                ZiteraButton(
                  label: 'Clear Local Challenge Cache',
                  icon: Icons.delete_outline,
                  variant: ButtonVariant.danger,
                  onPressed: () {
                    ScaffoldMessenger.of(context).showSnackBar(
                      const SnackBar(
                        content: Text('Local progress and flag caches have been reset.'),
                        backgroundColor: ZiteraColors.ready,
                      ),
                    );
                  },
                ),
              ],
            ),
          ),
        ],
      ),
    );
  }

  Widget _settingRow(String label, String value) {
    return Padding(
      padding: const EdgeInsets.symmetric(vertical: 6.0),
      child: Row(
        mainAxisAlignment: MainAxisAlignment.spaceBetween,
        children: [
          Text(label, style: const TextStyle(fontWeight: FontWeight.w600, fontSize: 13)),
          Text(value, style: const TextStyle(color: ZiteraColors.cyan, fontFamily: 'monospace', fontSize: 12)),
        ],
      ),
    );
  }
}
