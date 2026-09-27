import 'dart:convert';
import 'dart:io';

/// Local Progress Manager (Phase 6 Security & Ponytail principles).
/// Strictly adheres to SEC rules: The correct flag is NEVER stored in
/// Flutter local progress, UI state, normal logs, or UI assets.
/// Progress tracks interactions: completed labs, challenges, practice, and sections.
class ProgressManager {
  static const String _fileName = 'zitera_progress.json';

  static File _getProgressFile() {
    return File(_fileName);
  }

  static Future<Map<String, dynamic>> loadProgress() async {
    final file = _getProgressFile();
    if (!file.existsSync()) {
      return {
        'completed_labs': <String>[],
        'completed_challenges': <String>[],
        'completed_practice': <String>[],
        'completed_sections': <String, dynamic>{},
        'last_updated': DateTime.now().toIso8601String(),
      };
    }
    try {
      final text = await file.readAsString();
      final decoded = jsonDecode(text) as Map<String, dynamic>;
      // Sanitize: ensure legacy 'solved_flags' with secret plain texts is purged
      decoded.remove('solved_flags');
      return decoded;
    } catch (_) {
      return {
        'completed_labs': <String>[],
        'completed_challenges': <String>[],
        'completed_practice': <String>[],
        'completed_sections': <String, dynamic>{},
        'last_updated': DateTime.now().toIso8601String(),
      };
    }
  }

  static Future<void> markLabCompleted(String labId) async {
    final data = await loadProgress();
    final list = List<String>.from(data['completed_labs'] as List? ?? []);
    if (!list.contains(labId)) {
      list.add(labId);
      data['completed_labs'] = list;
      data['last_updated'] = DateTime.now().toIso8601String();
      await _getProgressFile().writeAsString(jsonEncode(data));
    }
  }

  /// Authoritative challenge pass recorded without ever saving the flag string.
  static Future<void> markChallengeCompleted(String labId) async {
    final data = await loadProgress();
    final challenges = List<String>.from(data['completed_challenges'] as List? ?? []);
    final labs = List<String>.from(data['completed_labs'] as List? ?? []);
    var changed = false;

    if (!challenges.contains(labId)) {
      challenges.add(labId);
      data['completed_challenges'] = challenges;
      changed = true;
    }
    if (!labs.contains(labId)) {
      labs.add(labId);
      data['completed_labs'] = labs;
      changed = true;
    }

    if (changed) {
      data['last_updated'] = DateTime.now().toIso8601String();
      await _getProgressFile().writeAsString(jsonEncode(data));
    }
  }

  /// Mark practice mode completed for a given lab.
  static Future<void> markPracticeCompleted(String labId) async {
    final data = await loadProgress();
    final practice = List<String>.from(data['completed_practice'] as List? ?? []);
    if (!practice.contains(labId)) {
      practice.add(labId);
      data['completed_practice'] = practice;
      data['last_updated'] = DateTime.now().toIso8601String();
      await _getProgressFile().writeAsString(jsonEncode(data));
    }
  }

  /// Mark learning section viewed/completed for interaction-based progress.
  static Future<void> markSectionCompleted(String labId, String section) async {
    final data = await loadProgress();
    final sectionsMap = Map<String, dynamic>.from(data['completed_sections'] as Map? ?? {});
    final labSections = List<String>.from(sectionsMap[labId] as List? ?? []);

    if (!labSections.contains(section)) {
      labSections.add(section);
      sectionsMap[labId] = labSections;
      data['completed_sections'] = sectionsMap;
      data['last_updated'] = DateTime.now().toIso8601String();
      await _getProgressFile().writeAsString(jsonEncode(data));
    }
  }

  static Future<bool> isChallengeCompleted(String labId) async {
    final data = await loadProgress();
    final list = List<String>.from(data['completed_challenges'] as List? ?? []);
    return list.contains(labId);
  }

  static Future<bool> isPracticeCompleted(String labId) async {
    final data = await loadProgress();
    final list = List<String>.from(data['completed_practice'] as List? ?? []);
    return list.contains(labId);
  }

  /// Backward-compatible alias for existing call-sites: does NOT store flag string
  static Future<void> markFlagSolved(String labId, [String? flag]) async {
    await markChallengeCompleted(labId);
  }

  static Future<void> resetAll() async {
    final file = _getProgressFile();
    if (file.existsSync()) {
      await file.delete();
    }
  }
}
