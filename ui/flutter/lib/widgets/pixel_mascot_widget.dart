import 'package:flutter/material.dart';
import '../core/theme/zitera_colors.dart';

class PixelMascotWidget extends StatefulWidget {
  final bool compact;

  const PixelMascotWidget({
    super.key,
    this.compact = false,
  });

  @override
  State<PixelMascotWidget> createState() => _PixelMascotWidgetState();
}

class _PixelMascotWidgetState extends State<PixelMascotWidget> {
  bool _isCatMode = false;
  int _tipIndex = 0;

  final List<String> _girlTips = [
    "Selamat datang di ZITERA_LAB! Siap eksplorasi kerentanan OWASP Top 10?",
    "Tips: Selalu cek input validation dan sanitasi query SQL di layer repository.",
    "Practice Mode dilengkapi langkah walkthrough lengkap jika kamu butuh panduan!",
    "Mesin zitera-engine berjalan ringan di latar belakang tanpa daemon berat.",
    "Mode Challenge menguji kemampuan analisis tanpa bocoran solusi langsung!",
    "Semua lab berjalan di container Docker terisolasi pada port 127.0.0.1 lokal.",
  ];

  final List<String> _catTips = [
    "Meow! Sensor Docker menunjukkan status container sangat prima!",
    "Purrr... Neko merekomendasikan cek Smart Setup sebelum memulai lab baru.",
    "Meow-ware detected? Jangan khawatir, lab ini aman dan terisolasi!",
    "Klik aku lagi untuk tips cyber security berikutnya, meow!",
  ];

  void _nextTip() {
    setState(() {
      final tips = _isCatMode ? _catTips : _girlTips;
      _tipIndex = (_tipIndex + 1) % tips.length;
    });
  }

  void _toggleCharacter() {
    setState(() {
      _isCatMode = !_isCatMode;
      _tipIndex = 0;
    });
  }

  @override
  Widget build(BuildContext context) {
    final currentTips = _isCatMode ? _catTips : _girlTips;
    final activeTip = currentTips[_tipIndex % currentTips.length];
    final characterName = _isCatMode ? "NEKO // CYBER_COMPANION" : "ZETA // LAB_OPERATOR";
    final characterAsset = _isCatMode ? 'assets/images/calico_cat.png' : 'assets/images/zeta_avatar.png';

    return Container(
      padding: const EdgeInsets.all(14),
      decoration: BoxDecoration(
        color: const Color(0xFFFCFBF8),
        borderRadius: BorderRadius.circular(10),
        border: Border.all(color: ZiteraColors.border, width: 1.2),
        boxShadow: [
          BoxShadow(
            color: Colors.black.withValues(alpha: 0.04),
            blurRadius: 8,
            offset: const Offset(0, 2),
          ),
        ],
      ),
      child: Row(
        crossAxisAlignment: CrossAxisAlignment.center,
        children: [
          // ── Avatar column: enlarged to 88×88 ──
          Column(
            mainAxisSize: MainAxisSize.min,
            children: [
              Container(
                width: 88,
                height: 88,
                decoration: BoxDecoration(
                  color: const Color(0xFFFFEDD5),
                  borderRadius: BorderRadius.circular(12),
                  border: Border.all(color: const Color(0xFFFDBA74), width: 2.0),
                  boxShadow: [
                    BoxShadow(
                      color: const Color(0xFFFDBA74).withValues(alpha: 0.25),
                      blurRadius: 8,
                    ),
                  ],
                ),
                child: ClipRRect(
                  borderRadius: BorderRadius.circular(10),
                  child: Image.asset(
                    characterAsset,
                    fit: BoxFit.cover,
                    errorBuilder: (context, error, stackTrace) =>
                        const Icon(Icons.face, size: 48, color: Color(0xFF5C5347)),
                  ),
                ),
              ),
              const SizedBox(height: 8),
              // SWAP pill button
              InkWell(
                onTap: _toggleCharacter,
                borderRadius: BorderRadius.circular(6),
                child: Container(
                  padding: const EdgeInsets.symmetric(horizontal: 10, vertical: 4),
                  decoration: BoxDecoration(
                    color: const Color(0xFFFCFBF8),
                    borderRadius: BorderRadius.circular(6),
                    border: Border.all(color: ZiteraColors.border, width: 1),
                  ),
                  child: Row(
                    mainAxisSize: MainAxisSize.min,
                    children: [
                      const Icon(Icons.swap_horiz, size: 12, color: Color(0xFF0F766E)),
                      const SizedBox(width: 4),
                      Text(
                        _isCatMode ? 'ZETA' : 'SWAP',
                        style: const TextStyle(
                          fontFamily: 'SpaceGrotesk',
                          fontSize: 10,
                          fontWeight: FontWeight.w700,
                          color: Color(0xFF1E1A14),
                        ),
                      ),
                    ],
                  ),
                ),
              ),
            ],
          ),

          const SizedBox(width: 16),

          // Speech Bubble Card
          Expanded(
            child: Container(
              padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 14),
              decoration: BoxDecoration(
                color: const Color(0xFFF7F5F0),
                borderRadius: BorderRadius.circular(10),
                border: Border.all(color: ZiteraColors.border, width: 1),
              ),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                mainAxisSize: MainAxisSize.min,
                children: [
                  Row(
                    mainAxisAlignment: MainAxisAlignment.spaceBetween,
                    children: [
                      // Character name — SpaceGrotesk
                      Text(
                        characterName,
                        style: const TextStyle(
                          fontFamily: 'SpaceGrotesk',
                          fontSize: 11,
                          fontWeight: FontWeight.w700,
                          color: Color(0xFF1E1A14),
                          letterSpacing: 0.5,
                        ),
                      ),
                      // CLICK FOR TIPS button
                      InkWell(
                        onTap: _nextTip,
                        borderRadius: BorderRadius.circular(6),
                        child: Container(
                          padding: const EdgeInsets.symmetric(horizontal: 10, vertical: 5),
                          decoration: BoxDecoration(
                            color: const Color(0xFFFCFBF8),
                            borderRadius: BorderRadius.circular(6),
                            border: Border.all(color: ZiteraColors.borderDark, width: 0.8),
                          ),
                          child: const Row(
                            mainAxisSize: MainAxisSize.min,
                            children: [
                              Icon(Icons.touch_app, size: 12, color: Color(0xFF5C5347)),
                              SizedBox(width: 5),
                              Text(
                                'CLICK FOR TIPS',
                                style: TextStyle(
                                  fontFamily: 'SpaceGrotesk',
                                  fontSize: 9,
                                  fontWeight: FontWeight.w600,
                                  color: Color(0xFF5C5347),
                                ),
                              ),
                            ],
                          ),
                        ),
                      ),
                    ],
                  ),
                  const SizedBox(height: 10),
                  // Tip text — PlusJakartaSans is super clean and natural for speech
                  Text(
                    activeTip,
                    style: const TextStyle(
                      fontFamily: 'PlusJakartaSans',
                      fontSize: 13,
                      color: Color(0xFF1E1A14),
                      height: 1.5,
                      fontWeight: FontWeight.w500,
                    ),
                  ),
                ],
              ),
            ),
          ),
        ],
      ),
    );
  }
}
