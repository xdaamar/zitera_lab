import 'package:flutter/material.dart';
import '../core/theme/zitera_colors.dart';

class Sidebar extends StatefulWidget {
  final int selectedIndex;
  final Function(int) onDestinationSelected;

  const Sidebar({
    super.key,
    required this.selectedIndex,
    required this.onDestinationSelected,
  });

  @override
  State<Sidebar> createState() => _SidebarState();
}

class _SidebarState extends State<Sidebar> {
  bool _isMenuVisible = true;

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
          // 1. Sidebar Background Video/Animation (Original Colors - Infinite Looping)
          Positioned.fill(
            child: Image.asset(
              'assets/images/sidebar_bg.webp',
              fit: BoxFit.cover,
              gaplessPlayback: true,
              errorBuilder: (context, error, stackTrace) {
                return Image.asset(
                  'assets/images/sidebar_bg.jpg',
                  fit: BoxFit.cover,
                );
              },
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
                                // "ZITERA" — SpaceGrotesk bold
                                const Text(
                                  'ZITERA',
                                  style: TextStyle(
                                    fontFamily: 'SpaceGrotesk',
                                    fontSize: 16,
                                    fontWeight: FontWeight.bold,
                                    color: Color(0xFF1E1A14),
                                    letterSpacing: 1.0,
                                  ),
                                ),
                                const SizedBox(width: 6),
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
                                      fontFamily: 'SpaceGrotesk',
                                      fontSize: 9,
                                      fontWeight: FontWeight.bold,
                                      color: Color(0xFF0F766E),
                                    ),
                                  ),
                                ),
                              ],
                            ),
                            const SizedBox(height: 2),
                            // Subtext — JetBrainsMono (RETRO removed)
                            const Text(
                              'CYBER LAB',
                              style: TextStyle(
                                color: Color(0xFF5C5347),
                                fontSize: 11,
                                fontFamily: 'JetBrainsMono',
                                fontWeight: FontWeight.w500,
                                letterSpacing: 0.3,
                              ),
                            ),
                          ],
                        ),
                      ),
                      // Show / Hide Toggle Button
                      Material(
                        color: Colors.transparent,
                        child: InkWell(
                          onTap: () {
                            setState(() {
                              _isMenuVisible = !_isMenuVisible;
                            });
                          },
                          borderRadius: BorderRadius.circular(6),
                          child: Container(
                            padding: const EdgeInsets.all(5),
                            decoration: BoxDecoration(
                              color: _isMenuVisible ? const Color(0xFFE6FFFA) : const Color(0xFFF8FAFC),
                              borderRadius: BorderRadius.circular(6),
                              border: Border.all(
                                color: _isMenuVisible ? const Color(0xFF2DD4BF) : ZiteraColors.border,
                                width: 1.0,
                              ),
                            ),
                            child: Icon(
                              _isMenuVisible ? Icons.keyboard_arrow_up : Icons.keyboard_arrow_down,
                              size: 18,
                              color: _isMenuVisible ? const Color(0xFF0F766E) : const Color(0xFF5C5347),
                            ),
                          ),
                        ),
                      ),
                    ],
                  ),
                ),
              ),

              const SizedBox(height: 4),

              // Navigation Menu Items with Show/Hide Animated Crossfade
              AnimatedCrossFade(
                firstChild: Column(
                  mainAxisSize: MainAxisSize.min,
                  children: [
                    _navItem(0, 'Dashboard',    Icons.dashboard_outlined,         'HOME_SYS',   null),
                    _navItem(1, 'Labs Catalog', Icons.science_outlined,            'OWASP_25',   'assets/images/sticker_pink.png'),
                    _navItem(2, 'Smart Setup',  Icons.health_and_safety_outlined,  'SYS_DIAG',   'assets/images/sticker_bunny.png'),
                    _navItem(3, 'Tool Manager', Icons.construction_outlined,        'SEC_TOOLS',  'assets/images/sticker_cat_laptop.png'),
                    _navItem(4, 'Settings',     Icons.settings_outlined,            'PREFERENCES', 'assets/images/sticker_shiba.png'),
                  ],
                ),
                secondChild: Padding(
                  padding: const EdgeInsets.symmetric(horizontal: 14, vertical: 8),
                  child: Container(
                    width: double.infinity,
                    padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 10),
                    decoration: BoxDecoration(
                      color: const Color(0xFFFCFBF8).withValues(alpha: 0.85),
                      borderRadius: BorderRadius.circular(8),
                      border: Border.all(color: ZiteraColors.border, width: 1.0),
                    ),
                    child: const Row(
                      mainAxisAlignment: MainAxisAlignment.center,
                      children: [
                        Icon(Icons.visibility_off_outlined, size: 14, color: Color(0xFF9C9080)),
                        SizedBox(width: 8),
                        Text(
                          'MENU HIDDEN',
                          style: TextStyle(
                            fontFamily: 'JetBrainsMono',
                            fontSize: 10,
                            fontWeight: FontWeight.bold,
                            color: Color(0xFF9C9080),
                            letterSpacing: 0.5,
                          ),
                        ),
                      ],
                    ),
                  ),
                ),
                crossFadeState: _isMenuVisible ? CrossFadeState.showFirst : CrossFadeState.showSecond,
                duration: const Duration(milliseconds: 200),
              ),

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
                      // Calico Cat — enlarged to 58×58 so it's clearly visible
                      SizedBox(
                        width: 58,
                        height: 58,
                        child: Image.asset(
                          'assets/images/calico_cat.png',
                          fit: BoxFit.contain,
                          errorBuilder: (context, error, stackTrace) =>
                              const Icon(Icons.pets, size: 36, color: Color(0xFFFDBA74)),
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
                                    fontFamily: 'SpaceGrotesk',
                                    fontSize: 10,
                                    fontWeight: FontWeight.bold,
                                    color: Color(0xFF1E1A14),
                                    letterSpacing: 0.4,
                                  ),
                                ),
                                Text(
                                  'READY',
                                  style: TextStyle(
                                    fontFamily: 'JetBrainsMono',
                                    fontSize: 12,
                                    fontWeight: FontWeight.bold,
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
    final isSelected = widget.selectedIndex == index;
    return Padding(
      padding: const EdgeInsets.symmetric(horizontal: 14, vertical: 3),
      child: Material(
        color: Colors.transparent,
        child: InkWell(
          onTap: () => widget.onDestinationSelected(index),
          borderRadius: BorderRadius.circular(8),
          child: AnimatedContainer(
            duration: const Duration(milliseconds: 150),
            padding: EdgeInsets.symmetric(
              horizontal: 12,
              vertical: stickerAsset != null ? 5 : 9,
            ),
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
                      fontFamily: 'SpaceGrotesk',
                      color: isSelected ? const Color(0xFF0F766E) : const Color(0xFF1E1A14),
                      fontSize: 13,
                      fontWeight: isSelected ? FontWeight.bold : FontWeight.w500,
                    ),
                  ),
                ),
                if (stickerAsset != null)
                  SizedBox(
                    width: 34,
                    height: 34,
                    child: Image.asset(
                      stickerAsset,
                      fit: BoxFit.contain,
                      errorBuilder: (context, error, stackTrace) => const SizedBox(),
                    ),
                  )
                else
                  Text(
                    tag,
                    style: TextStyle(
                      fontFamily: 'JetBrainsMono',
                      fontSize: 10,
                      fontWeight: FontWeight.w600,
                      color: isSelected ? const Color(0xFF0F766E) : const Color(0xFF9C9080),
                      letterSpacing: 0.3,
                    ),
                  ),
              ],
            ),
          ),
        ),
      ),
    );
  }
}
