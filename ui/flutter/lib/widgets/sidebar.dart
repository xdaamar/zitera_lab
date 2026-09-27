import 'package:flutter/material.dart';
import '../core/theme/zitera_colors.dart';

class Sidebar extends StatelessWidget {
  final int selectedIndex;
  final Function(int) onDestinationSelected;

  const Sidebar({
    super.key,
    required this.selectedIndex,
    required this.onDestinationSelected,
  });

  @override
  Widget build(BuildContext context) {
    return Container(
      width: 270,
      decoration: const BoxDecoration(
        border: Border(
          right: BorderSide(color: ZiteraColors.border, width: 1.5),
        ),
      ),
      child: Stack(
        children: [
          // 1. Sidebar Background Image
          Positioned.fill(
            child: Image.asset(
              'assets/images/sidebar_bg.jpg',
              fit: BoxFit.cover,
            ),
          ),
          // Warm cream wash overlay — matches off-white palette
          Positioned.fill(
            child: Container(
              color: const Color(0xFFFCFBF8).withValues(alpha: 0.46),
            ),
          ),

          // 2. Foreground Menu Elements
          Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              // Header Brand Card
              Padding(
                padding: const EdgeInsets.only(left: 14.0, right: 14.0, top: 16.0, bottom: 12.0),
                child: Container(
                  padding: const EdgeInsets.symmetric(horizontal: 10, vertical: 10),
                  decoration: BoxDecoration(
                    color: const Color(0xFFFCFBF8).withValues(alpha: 0.96),
                    borderRadius: BorderRadius.circular(10),
                    border: Border.all(color: ZiteraColors.border, width: 1.2),
                    boxShadow: [
                      BoxShadow(
                        color: Colors.black.withValues(alpha: 0.04),
                        blurRadius: 6,
                        offset: const Offset(0, 2),
                      ),
                    ],
                  ),
                  child: Row(
                    children: [
                      // ── Avatar: enlarged to 44×44 ──
                      Container(
                        width: 44,
                        height: 44,
                        decoration: BoxDecoration(
                          shape: BoxShape.circle,
                          border: Border.all(color: const Color(0xFF2DD4BF), width: 2.0),
                          boxShadow: [
                            BoxShadow(
                              color: const Color(0xFF2DD4BF).withValues(alpha: 0.25),
                              blurRadius: 8,
                            ),
                          ],
                        ),
                        child: ClipOval(
                          child: Image.asset(
                            'assets/images/zeta_avatar.png',
                            fit: BoxFit.cover,
                            errorBuilder: (context, error, stackTrace) =>
                                const Icon(Icons.face, size: 26, color: Color(0xFF0F766E)),
                          ),
                        ),
                      ),
                      const SizedBox(width: 10),
                      Expanded(
                        child: Column(
                          crossAxisAlignment: CrossAxisAlignment.start,
                          mainAxisSize: MainAxisSize.min,
                          children: [
                            Row(
                              children: [
                                // "ZITERA" — keep Silkscreen but readable size/weight
                                const Text(
                                  'ZITERA',
                                  style: TextStyle(
                                    fontFamily: 'Silkscreen',
                                    fontSize: 14,
                                    fontWeight: FontWeight.normal, // avoid w900 on pixel font
                                    color: Color(0xFF1E1A14),
                                    letterSpacing: 1.2,
                                  ),
                                ),
                                const SizedBox(width: 5),
                                Container(
                                  padding: const EdgeInsets.symmetric(horizontal: 5, vertical: 2),
                                  decoration: BoxDecoration(
                                    color: const Color(0xFFCCFBF1),
                                    borderRadius: BorderRadius.circular(4),
                                    border: Border.all(color: const Color(0xFF2DD4BF), width: 0.8),
                                  ),
                                  child: const Text(
                                    'LAB',
                                    style: TextStyle(
                                      fontFamily: 'Silkscreen',
                                      fontSize: 8,
                                      fontWeight: FontWeight.normal,
                                      color: Color(0xFF0F766E),
                                    ),
                                  ),
                                ),
                              ],
                            ),
                            const SizedBox(height: 2),
                            // Subtext — VT323 is fine for retro terminal feel at larger size
                            const Text(
                              'CYBER LAB // RETRO',
                              style: TextStyle(
                                color: Color(0xFF5C5347),
                                fontSize: 13,
                                fontFamily: 'VT323',
                                letterSpacing: 0.5,
                              ),
                            ),
                          ],
                        ),
                      ),
                    ],
                  ),
                ),
              ),

              const SizedBox(height: 4),

              // Navigation Menu Items
              _navItem(0, 'Dashboard',    Icons.dashboard_outlined,         'HOME_SYS',   null),
              _navItem(1, 'Labs Catalog', Icons.science_outlined,            'OWASP_25',   'assets/images/sticker_pink.png'),
              _navItem(2, 'Smart Setup',  Icons.health_and_safety_outlined,  'SYS_DIAG',   null),
              _navItem(3, 'Tool Manager', Icons.construction_outlined,        'SEC_TOOLS',  'assets/images/sticker_cat_laptop.png'),
              _navItem(4, 'Settings',     Icons.settings_outlined,            'PREFERENCES', null),

              const Spacer(),

              // Neko Status Card
              Padding(
                padding: const EdgeInsets.all(14.0),
                child: Container(
                  padding: const EdgeInsets.all(10),
                  decoration: BoxDecoration(
                    color: const Color(0xFFFCFBF8).withValues(alpha: 0.96),
                    borderRadius: BorderRadius.circular(10),
                    border: Border.all(color: ZiteraColors.border, width: 1.5),
                    boxShadow: [
                      BoxShadow(
                        color: Colors.black.withValues(alpha: 0.05),
                        blurRadius: 10,
                        offset: const Offset(0, 3),
                      ),
                    ],
                  ),
                  child: Row(
                    children: [
                      // Calico Cat — enlarged to 52×52 so it's clearly visible
                      SizedBox(
                        width: 52,
                        height: 52,
                        child: Image.asset(
                          'assets/images/calico_cat.png',
                          fit: BoxFit.contain,
                          errorBuilder: (context, error, stackTrace) =>
                              const Icon(Icons.pets, size: 32, color: Color(0xFFFDBA74)),
                        ),
                      ),
                      const SizedBox(width: 10),
                      const Expanded(
                        child: Column(
                          crossAxisAlignment: CrossAxisAlignment.start,
                          mainAxisSize: MainAxisSize.min,
                          children: [
                            Row(
                              mainAxisAlignment: MainAxisAlignment.spaceBetween,
                              children: [
                                Text(
                                  'NEKO STATUS',
                                  style: TextStyle(
                                    fontFamily: 'Silkscreen',
                                    fontSize: 9,
                                    fontWeight: FontWeight.normal,
                                    color: Color(0xFF1E1A14),
                                    letterSpacing: 0.4,
                                  ),
                                ),
                                Text(
                                  'READY',
                                  style: TextStyle(
                                    fontFamily: 'VT323',
                                    fontSize: 16,
                                    color: ZiteraColors.ready,
                                    letterSpacing: 0.3,
                                  ),
                                ),
                              ],
                            ),
                            SizedBox(height: 2),
                            Text(
                              '127.0.0.1 // WS-01',
                              style: TextStyle(
                                fontFamily: 'JetBrainsMono',
                                fontSize: 10,
                                color: Color(0xFF5C5347),
                              ),
                            ),
                          ],
                        ),
                      ),
                    ],
                  ),
                ),
              ),
            ],
          ),
        ],
      ),
    );
  }

  Widget _navItem(int index, String label, IconData icon, String tag, String? stickerAsset) {
    final isSelected = selectedIndex == index;
    return Padding(
      padding: const EdgeInsets.symmetric(horizontal: 14, vertical: 3),
      child: Stack(
        clipBehavior: Clip.none,
        children: [
          Material(
            color: Colors.transparent,
            child: InkWell(
              onTap: () => onDestinationSelected(index),
              borderRadius: BorderRadius.circular(8),
              child: AnimatedContainer(
                duration: const Duration(milliseconds: 150),
                padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 10),
                decoration: BoxDecoration(
                  color: isSelected
                      ? const Color(0xFFFCFBF8)
                      : const Color(0xFFFCFBF8).withValues(alpha: 0.88),
                  borderRadius: BorderRadius.circular(8),
                  border: Border.all(
                    color: isSelected ? const Color(0xFF2DD4BF) : ZiteraColors.border,
                    width: isSelected ? 1.5 : 1.0,
                  ),
                  boxShadow: [
                    BoxShadow(
                      color: isSelected
                          ? const Color(0xFF2DD4BF).withValues(alpha: 0.15)
                          : Colors.black.withValues(alpha: 0.03),
                      blurRadius: isSelected ? 8 : 4,
                      offset: const Offset(0, 2),
                    ),
                  ],
                ),
                child: Row(
                  children: [
                    Icon(
                      icon,
                      size: 18,
                      color: isSelected ? const Color(0xFF0F766E) : const Color(0xFF5C5347),
                    ),
                    const SizedBox(width: 10),
                    Expanded(
                      child: Text(
                        label,
                        style: TextStyle(
                          // Use JetBrainsMono for nav items (much more readable than Silkscreen)
                          fontFamily: 'JetBrainsMono',
                          color: isSelected ? const Color(0xFF0F766E) : const Color(0xFF1E1A14),
                          fontSize: 12,
                          fontWeight: isSelected ? FontWeight.bold : FontWeight.w500,
                        ),
                      ),
                    ),
                    // Tag — VT323 at 14px is very readable
                    Text(
                      tag,
                      style: TextStyle(
                        fontFamily: 'VT323',
                        fontSize: 14,
                        color: isSelected ? const Color(0xFF0F766E) : const Color(0xFF9C9080),
                        letterSpacing: 0.3,
                      ),
                    ),
                  ],
                ),
              ),
            ),
          ),
          if (stickerAsset != null)
            Positioned(
              right: -8,
              top: -8,
              child: SizedBox(
                width: 30,
                height: 30,
                child: Image.asset(
                  stickerAsset,
                  fit: BoxFit.contain,
                ),
              ),
            ),
        ],
      ),
    );
  }
}
