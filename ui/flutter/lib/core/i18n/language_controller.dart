import 'package:flutter/foundation.dart';
import '../progress/progress_manager.dart';

/// Global controller managing user language preference for Zitera Lab.
/// Follows PRD specifications:
/// - Only learning materials (lessons, concepts, analogies, walkthroughs)
///   and task instructions (challenge objectives, hints, practice steps)
///   are translated into Indonesian.
/// - Technical terminology (e.g. SQL Injection, IDOR, Broken Access Control,
///   JWT, Payload, Bypass) remains in standard English accompanied by clear
///   conceptual Indonesian explanations.
/// - Buttons, cards, lab titles, navigation menus, and system metrics remain
///   in English for industry-standard consistency.
class AppLanguageController {
  static final ValueNotifier<String> currentLanguage = ValueNotifier<String>('en');

  static bool get isIndonesian => currentLanguage.value == 'id';
  static bool get isEnglish => currentLanguage.value == 'en';

  static Future<void> init() async {
    try {
      final savedLang = await ProgressManager.getLanguage();
      currentLanguage.value = savedLang;
    } catch (_) {
      currentLanguage.value = 'en';
    }
  }

  static Future<void> setLanguage(String langCode) async {
    final cleanCode = (langCode == 'id') ? 'id' : 'en';
    if (currentLanguage.value != cleanCode) {
      currentLanguage.value = cleanCode;
      try {
        await ProgressManager.setLanguage(cleanCode);
      } catch (_) {}
    }
  }

  static Future<void> toggle() async {
    await setLanguage(isIndonesian ? 'en' : 'id');
  }
}
