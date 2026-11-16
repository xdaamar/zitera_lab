import 'package:flutter/material.dart';

/// Represents a selectable Operative Avatar profile.
class AvatarDefinition {
  final String id;
  final String name;
  final String gender;
  final String expression;
  final String assetPath;

  const AvatarDefinition({
    required this.id,
    required this.name,
    required this.gender,
    required this.expression,
    required this.assetPath,
  });

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is AvatarDefinition &&
          runtimeType == other.runtimeType &&
          id == other.id;

  @override
  int get hashCode => id.hashCode;
}

/// Central registry of Zitera Lab Operative Avatars with defensive fallbacks.
class AvatarRegistry {
  static const String defaultId = 'zeta';
  static const String fallbackAsset = 'assets/images/zeta_avatar.png';

  /// Standard 14 Zitera Operative Avatars (2 Classics + 6 Cyber Girls + 6 Cyber Boys).
  static const List<AvatarDefinition> all = [
    // ── Classic Mascot & Operator ──
    AvatarDefinition(
      id: 'zeta',
      name: 'Zeta',
      gender: 'Operator',
      expression: 'Operator Classic',
      assetPath: 'assets/images/zeta_avatar.png',
    ),
    AvatarDefinition(
      id: 'neko',
      name: 'Neko',
      gender: 'Mascot',
      expression: 'Cyber Calico Cat',
      assetPath: 'assets/images/calico_cat.png',
    ),

    // ── Cyber Girls (Operatives) ──
    AvatarDefinition(
      id: 'girl_cheerful',
      name: 'Aoi',
      gender: 'Female',
      expression: 'Cheerful / Ceria',
      assetPath: 'assets/images/avatars/girl_cheerful.png',
    ),
    AvatarDefinition(
      id: 'girl_cool',
      name: 'Rin',
      gender: 'Female',
      expression: 'Cool / Kalem',
      assetPath: 'assets/images/avatars/girl_cool.png',
    ),
    AvatarDefinition(
      id: 'girl_smug',
      name: 'Mia',
      gender: 'Female',
      expression: 'Tengil / Smug',
      assetPath: 'assets/images/avatars/girl_smug.png',
    ),
    AvatarDefinition(
      id: 'girl_serious',
      name: 'Hana',
      gender: 'Female',
      expression: 'Serius / Cyber Analyst',
      assetPath: 'assets/images/avatars/girl_serious.png',
    ),
    AvatarDefinition(
      id: 'girl_pouty',
      name: 'Yuki',
      gender: 'Female',
      expression: 'Pouty / Cemberut Lucu',
      assetPath: 'assets/images/avatars/girl_pouty.png',
    ),
    AvatarDefinition(
      id: 'girl_fierce',
      name: 'Asuka',
      gender: 'Female',
      expression: 'Fierce / Ambisius',
      assetPath: 'assets/images/avatars/girl_fierce.png',
    ),

    // ── Cyber Boys (Operatives) ──
    AvatarDefinition(
      id: 'boy_cheerful',
      name: 'Ren',
      gender: 'Male',
      expression: 'Cheerful / Enerjik',
      assetPath: 'assets/images/avatars/boy_cheerful.png',
    ),
    AvatarDefinition(
      id: 'boy_cool',
      name: 'Kaito',
      gender: 'Male',
      expression: 'Cool / Hacker Hoodie',
      assetPath: 'assets/images/avatars/boy_cool.png',
    ),
    AvatarDefinition(
      id: 'boy_smug',
      name: 'Shin',
      gender: 'Male',
      expression: 'Tengil / Smirk',
      assetPath: 'assets/images/avatars/boy_smug.png',
    ),
    AvatarDefinition(
      id: 'boy_serious',
      name: 'Daiki',
      gender: 'Male',
      expression: 'Serius / Tactical Visor',
      assetPath: 'assets/images/avatars/boy_serious.png',
    ),
    AvatarDefinition(
      id: 'boy_pouty',
      name: 'Sora',
      gender: 'Male',
      expression: 'Pouty / Tsundere',
      assetPath: 'assets/images/avatars/boy_pouty.png',
    ),
    AvatarDefinition(
      id: 'boy_fierce',
      name: 'Ryu',
      gender: 'Male',
      expression: 'Fierce / Competitive',
      assetPath: 'assets/images/avatars/boy_fierce.png',
    ),
  ];

  static final Map<String, AvatarDefinition> _byId = {
    for (final a in all) a.id: a,
  };

  /// Default fallback avatar ('zeta').
  static AvatarDefinition get defaultAvatar => _byId[defaultId]!;

  /// Returns avatar by id with defensive fallback to 'zeta' if not found or empty.
  static AvatarDefinition getById(String? id) {
    if (id == null || id.trim().isEmpty) {
      return defaultAvatar;
    }
    return _byId[id.trim().toLowerCase()] ?? defaultAvatar;
  }

  /// Returns asset path for an avatar ID with defensive fallback to 'zeta'.
  static String getAssetPath(String? id) {
    return getById(id).assetPath;
  }

  /// Verifies whether the specified avatar ID exists in registry.
  static bool isValidId(String? id) {
    if (id == null) return false;
    return _byId.containsKey(id.trim().toLowerCase());
  }

  /// Builds an Avatar Image widget with robust fallback on load errors.
  static Widget buildAvatarImage(
    String? id, {
    double? width,
    double? height,
    BoxFit fit = BoxFit.cover,
    BorderRadius? borderRadius,
  }) {
    final avatar = getById(id);
    Widget img = Image.asset(
      avatar.assetPath,
      width: width,
      height: height,
      fit: fit,
      errorBuilder: (context, error, stackTrace) {
        return Image.asset(
          fallbackAsset,
          width: width,
          height: height,
          fit: fit,
          errorBuilder: (c, e, s) => Container(
            width: width,
            height: height,
            decoration: BoxDecoration(
              color: const Color(0xFFCCFBF1),
              borderRadius: borderRadius,
            ),
            child: Icon(
              avatar.gender == 'Female' ? Icons.face_3 : Icons.face_6,
              size: (width != null) ? width * 0.6 : 24,
              color: const Color(0xFF0F766E),
            ),
          ),
        );
      },
    );

    if (borderRadius != null) {
      return ClipRRect(borderRadius: borderRadius, child: img);
    }
    return img;
  }
}
