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

  static Future<void> _atomicWrite(Map<String, dynamic> data) async {
    final file = _getProgressFile();
    final tmpFile = File('${file.path}.tmp');
    final jsonStr = jsonEncode(data);
    await tmpFile.writeAsString(jsonStr, flush: true);
    if (file.existsSync()) {
      await file.delete();
    }
    await tmpFile.rename(file.path);
  }

  static Future<Map<String, dynamic>> loadProgress() async {
    final file = _getProgressFile();
    if (!file.existsSync()) {
      return _defaultProgress();
    }
    try {
      final text = await file.readAsString();
      if (text.trim().isEmpty) {
        return _defaultProgress();
      }
      final decoded = jsonDecode(text) as Map<String, dynamic>;
      // Sanitize: ensure legacy 'solved_flags' with secret plain texts is purged
      decoded.remove('solved_flags');

      // Deduplicate lists and ensure valid schema
      final labs = (decoded['completed_labs'] as List? ?? [])
          .map((e) => e.toString().toUpperCase().trim())
          .where((id) => id.isNotEmpty && id.length <= 16)
          .toSet()
          .toList();
      final challenges = (decoded['completed_challenges'] as List? ?? [])
          .map((e) => e.toString().toUpperCase().trim())
          .where((id) => id.isNotEmpty && id.length <= 16)
          .toSet()
          .toList();
      final practice = (decoded['completed_practice'] as List? ?? [])
          .map((e) => e.toString().toUpperCase().trim())
          .where((id) => id.isNotEmpty && id.length <= 16)
          .toSet()
          .toList();

      decoded['completed_labs'] = labs;
      decoded['completed_challenges'] = challenges;
      decoded['completed_practice'] = practice;
      return decoded;
    } catch (_) {
      // Automatic corruption recovery: backup corrupted file and fallback to default
      try {
        if (file.existsSync()) {
          file.copySync('${file.path}.bak');
        }
      } catch (_) {}
      return _defaultProgress();
    }
  }

  static Map<String, dynamic> _defaultProgress() {
    return {
      'completed_labs': <String>[],
      'completed_challenges': <String>[],
      'completed_practice': <String>[],
      'completed_sections': <String, dynamic>{},
      'last_updated': DateTime.now().toIso8601String(),
    };
  }

  static Future<void> markLabCompleted(String labId) async {
    final cleanId = labId.toUpperCase().trim();
    final data = await loadProgress();
    final list = List<String>.from(data['completed_labs'] as List? ?? []);
    if (!list.contains(cleanId)) {
      list.add(cleanId);
      data['completed_labs'] = list;
      data['last_updated'] = DateTime.now().toIso8601String();
      await _atomicWrite(data);
    }
  }

  /// Authoritative challenge pass recorded without ever saving the flag string.
  static Future<void> markChallengeCompleted(String labId) async {
    final cleanId = labId.toUpperCase().trim();
    final data = await loadProgress();
    final challenges = List<String>.from(data['completed_challenges'] as List? ?? []);
    final labs = List<String>.from(data['completed_labs'] as List? ?? []);
    var changed = false;

    if (!challenges.contains(cleanId)) {
      challenges.add(cleanId);
      data['completed_challenges'] = challenges;
      changed = true;
    }
    if (!labs.contains(cleanId)) {
      labs.add(cleanId);
      data['completed_labs'] = labs;
      changed = true;
    }

    if (changed) {
      data['last_updated'] = DateTime.now().toIso8601String();
      await _atomicWrite(data);
    }
  }

  /// Mark practice mode completed for a given lab.
  static Future<void> markPracticeCompleted(String labId) async {
    final cleanId = labId.toUpperCase().trim();
    final data = await loadProgress();
    final practice = List<String>.from(data['completed_practice'] as List? ?? []);
    if (!practice.contains(cleanId)) {
      practice.add(cleanId);
      data['completed_practice'] = practice;
      data['last_updated'] = DateTime.now().toIso8601String();
      await _atomicWrite(data);
    }
  }

  /// Mark learning section viewed/completed for interaction-based progress.
  static Future<void> markSectionCompleted(String labId, String section) async {
    final cleanId = labId.toUpperCase().trim();
    final data = await loadProgress();
    final sectionsMap = Map<String, dynamic>.from(data['completed_sections'] as Map? ?? {});
    final labSections = List<String>.from(sectionsMap[cleanId] as List? ?? []);

    if (!labSections.contains(section)) {
      labSections.add(section);
      sectionsMap[cleanId] = labSections;
      data['completed_sections'] = sectionsMap;
      data['last_updated'] = DateTime.now().toIso8601String();
      await _atomicWrite(data);
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
