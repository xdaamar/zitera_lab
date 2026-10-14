import 'package:flutter_test/flutter_test.dart';
import 'package:zitera_lab/core/i18n/language_controller.dart';
import 'package:zitera_lab/core/models/curriculum_models.dart';

void main() {
  TestWidgetsFlutterBinding.ensureInitialized();

  group('Phase 1 Locale & Language Contract Tests', () {
    setUp(() {
      AppLanguageController.currentLanguage.value = AppLanguageController.defaultLocale;
    });

    test('Default locale is strictly "id" (Indonesian)', () {
      expect(AppLanguageController.defaultLocale, equals('id'));
      expect(AppLanguageController.supportedLocales, containsAll(['id', 'en']));
      expect(AppLanguageController.supportedLocales.length, equals(2));
      expect(AppLanguageController.currentLanguage.value, equals('id'));
      expect(AppLanguageController.isIndonesian, isTrue);
      expect(AppLanguageController.isEnglish, isFalse);
    });

    test('Locale normalizer enforces strict binary choice (fallback to id)', () {
      expect(AppLanguageController.normalizeLocale(null), equals('id'));
      expect(AppLanguageController.normalizeLocale(''), equals('id'));
      expect(AppLanguageController.normalizeLocale('fr'), equals('id'));
      expect(AppLanguageController.normalizeLocale('de'), equals('id'));
      expect(AppLanguageController.normalizeLocale('es'), equals('id'));
      expect(AppLanguageController.normalizeLocale('ID'), equals('id'));
      expect(AppLanguageController.normalizeLocale('  id  '), equals('id'));
      expect(AppLanguageController.normalizeLocale('en'), equals('en'));
      expect(AppLanguageController.normalizeLocale('EN'), equals('en'));
    });

    test('Setting language toggles state predictably', () async {
      await AppLanguageController.setLanguage('en');
      expect(AppLanguageController.currentLanguage.value, equals('en'));
      expect(AppLanguageController.isEnglish, isTrue);
      expect(AppLanguageController.isIndonesian, isFalse);

      await AppLanguageController.setLanguage('invalid_lang');
      expect(AppLanguageController.currentLanguage.value, equals('id'));
      expect(AppLanguageController.isIndonesian, isTrue);

      await AppLanguageController.toggle();
      expect(AppLanguageController.currentLanguage.value, equals('en'));

      await AppLanguageController.toggle();
      expect(AppLanguageController.currentLanguage.value, equals('id'));
    });
  });

  group('Phase 1 Curriculum Content & Flag Contract Tests', () {
    test('CurriculumContentRef resolves canonical bilingual filenames with stable ID', () {
      const ref = CurriculumContentRef(
        id: 'A01-L01-M01',
        labId: 'A01',
        section: 'walkthrough',
        tier: MissionProgressionTier.guidedPractice,
      );

      expect(ref.id, equals('A01-L01-M01'));
      expect(ref.labId, equals('A01'));
      expect(ref.tier, equals(MissionProgressionTier.guidedPractice));
      expect(ref.resolveFileName('id'), equals('walkthrough.id.md'));
      expect(ref.resolveFileName('en'), equals('walkthrough.en.md'));
      expect(ref.resolveFileName('unsupported'), equals('walkthrough.id.md'));
    });

    test('FlagContract enforces canonical ZITERA{...} regex and redaction', () {
      expect(FlagContract.isValidFormat('ZITERA{b10k3n_4cc355_c0ntr01}'), isTrue);
      expect(FlagContract.isValidFormat('ZITERA{sq1_1nj3ct10n_m45t3r_2026}'), isTrue);
      expect(FlagContract.isValidFormat('ZITERA{short}'), isFalse); // < 8 chars payload
      expect(FlagContract.isValidFormat('CTF{wrong_prefix}'), isFalse);
      expect(FlagContract.isValidFormat('ZITERA{}'), isFalse);
      expect(FlagContract.isValidFormat(null), isFalse);

      final leaked = 'Congratulations, your flag is ZITERA{b10k3n_4cc355_c0ntr01} submit now';
      final redacted = FlagContract.redactFlag(leaked);
      expect(redacted, equals('Congratulations, your flag is ZITERA{REDACTED_FLAG} submit now'));
    });
  });
}
