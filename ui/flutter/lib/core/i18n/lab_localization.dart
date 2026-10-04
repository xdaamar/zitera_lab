import 'dart:convert';
import 'dart:io';
import '../ipc/models.dart';

/// Centralized localization resolver for Zitera Lab learning materials.
/// Fulfills PRD requirements:
/// - Only lesson materials, walkthrough steps, challenge objectives, and hints
///   are localized into educational Indonesian.
/// - Buttons, cards, lab titles, system statuses, and navigation remain in standard English.
/// - Technical terms (SQL Injection, IDOR, Broken Access Control, JWT, etc.) remain in English
///   accompanied by natural Indonesian explanations.
class LabLocalization {
  /// Resolves lesson section markdown according to the active language
  static String getLessonContent({
    required String labId,
    required String sectionKey,
    required Map<String, String> lessons,
    required bool isIndonesian,
  }) {
    if (!isIndonesian) {
      return lessons[sectionKey] ?? '';
    }

    // 1. Check in-memory engine dictionary
    final localizedKeyDot = '$sectionKey.id';
    final localizedKeyUnder = '${sectionKey}_id';
    if (lessons.containsKey(localizedKeyDot) && lessons[localizedKeyDot]!.trim().isNotEmpty) {
      return lessons[localizedKeyDot]!;
    }
    if (lessons.containsKey(localizedKeyUnder) && lessons[localizedKeyUnder]!.trim().isNotEmpty) {
      return lessons[localizedKeyUnder]!;
    }

    // 2. Try reading from local repository directory
    try {
      final localFile = File('labs/$labId/lesson/$sectionKey.id.md');
      if (localFile.existsSync()) {
        final text = localFile.readAsStringSync();
        if (text.trim().isNotEmpty) return text;
      }
    } catch (_) {}

    // 3. Fallback to English original
    return lessons[sectionKey] ?? '';
  }

  /// Resolves CTF challenge mission objective according to active language
  static String getChallengeObjective({
    required String labId,
    required String defaultObjective,
    required bool isIndonesian,
  }) {
    if (!isIndonesian) {
      return defaultObjective;
    }

    try {
      final localFile = File('labs/$labId/challenge/challenge.id.md');
      if (localFile.existsSync()) {
        final text = localFile.readAsStringSync();
        if (text.trim().isNotEmpty) return text;
      }
    } catch (_) {}

    return defaultObjective;
  }

  /// Resolves progressive hints according to active language
  static List<ProgressiveHint> getHints({
    required String labId,
    required List<ProgressiveHint> defaultHints,
    required bool isIndonesian,
  }) {
    if (!isIndonesian) {
      return defaultHints;
    }

    try {
      final localFile = File('labs/$labId/challenge/hints.id.json');
      if (localFile.existsSync()) {
        final raw = localFile.readAsStringSync();
        final parsed = jsonDecode(raw) as Map<String, dynamic>;
        final rawHints = parsed['hints'] as List<dynamic>? ?? [];
        if (rawHints.isNotEmpty) {
          return rawHints
              .map((h) => ProgressiveHint.fromJson(h as Map<String, dynamic>))
              .toList();
        }
      }
    } catch (_) {}

    return defaultHints;
  }

  /// Resolves guided practice walkthrough text
  static String getWalkthroughText({
    required String labId,
    required Map<String, String> lessons,
    required bool isIndonesian,
  }) {
    if (isIndonesian) {
      if (lessons.containsKey('walkthrough.id') && lessons['walkthrough.id']!.trim().isNotEmpty) {
        return lessons['walkthrough.id']!;
      }
      if (lessons.containsKey('walkthrough_id') && lessons['walkthrough_id']!.trim().isNotEmpty) {
        return lessons['walkthrough_id']!;
      }
      try {
        final localFile = File('labs/$labId/lesson/walkthrough.id.md');
        if (localFile.existsSync()) {
          final text = localFile.readAsStringSync();
          if (text.trim().isNotEmpty) return text;
        }
      } catch (_) {}
    }

    return lessons['walkthrough'] ?? lessons['practice'] ?? '';
  }
}
