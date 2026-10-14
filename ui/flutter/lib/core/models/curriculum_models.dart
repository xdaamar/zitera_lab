/// ZITERA_LAB Canonical Curriculum & Learning Contract Models (Phase 1)
/// Strictly adheres to locale-invariance: Content identity and flags are language-independent.
library;

enum CurriculumTrack {
  trackA, // Web Security Fundamentals (Guided Learning)
  trackB, // Secure Systems & Application Architecture (Independent Challenge)
  sharedFoundation, // Core prerequisites (HTTP, DevTools, Sessions)
}

enum MissionProgressionTier {
  foundation, // Level 1: Recognition & mental analogy
  guidedPractice, // Level 2: Step-by-step observable hands-on practice
  appliedAnalysis, // Level 3: Evidence analysis & root-cause determination
  advancedRemediation, // Level 4: Autonomous CTF resolution & secure code patching
}

class CurriculumContentRef {
  final String id; // Stable, locale-invariant ID (e.g. A01-L01-M01)
  final String labId; // e.g. A01
  final String section; // e.g. introduction, concept, analogy, walkthrough, remediation
  final MissionProgressionTier tier;

  const CurriculumContentRef({
    required this.id,
    required this.labId,
    required this.section,
    this.tier = MissionProgressionTier.foundation,
  });

  /// Resolves canonical file name for a given locale (defaults to 'id').
  String resolveFileName(String locale) {
    final clean = (locale.trim().toLowerCase() == 'en') ? 'en' : 'id';
    return '$section.$clean.md';
  }
}

class FlagContract {
  static final RegExp flagPattern = RegExp(r'^ZITERA\{[a-zA-Z0-9_\-]{8,64}\}$');
  static final RegExp flagExtractionPattern = RegExp(r'ZITERA\{[a-zA-Z0-9_\-]{8,64}\}');

  /// Validates format compliance of a challenge completion flag.
  static bool isValidFormat(String? candidate) {
    if (candidate == null) return false;
    return flagPattern.hasMatch(candidate.trim());
  }

  /// Sanitizes log output: never print raw flags.
  static String redactFlag(String input) {
    return input.replaceAll(flagExtractionPattern, 'ZITERA{REDACTED_FLAG}');
  }
}
