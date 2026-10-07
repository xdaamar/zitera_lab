import 'dart:convert';
import 'dart:io';

/// Structured Lab Progress Record (Phase 19 Checkpoint 6).
/// Captures granular progress while preserving stable identity across package updates.
/// SEC rule: Flags and secrets are NEVER stored in progress.
class LabProgressRecord {
  final String labId;
  final String packageId;
  final String lessonStatus; // 'NOT_STARTED', 'IN_PROGRESS', 'COMPLETED'
  final String practiceStatus; // 'NOT_STARTED', 'COMPLETED'
  final String challengeStatus; // 'UNSOLVED', 'SOLVED'
  final int challengeAttempts;
  final String? completedAt;
  final String lastOpenedAt;

  LabProgressRecord({
    required this.labId,
    required this.packageId,
    this.lessonStatus = 'NOT_STARTED',
    this.practiceStatus = 'NOT_STARTED',
    this.challengeStatus = 'UNSOLVED',
    this.challengeAttempts = 0,
    this.completedAt,
    required this.lastOpenedAt,
  });

  Map<String, dynamic> toJson() => {
    'lab_id': labId,
    'package_id': packageId,
    'lesson_status': lessonStatus,
    'practice_status': practiceStatus,
    'challenge_status': challengeStatus,
    'challenge_attempts': challengeAttempts,
    if (completedAt != null) 'completed_at': completedAt,
    'last_opened_at': lastOpenedAt,
  };

  factory LabProgressRecord.fromJson(Map<String, dynamic> json, String defaultId) {
    final lid = (json['lab_id'] as String?)?.toUpperCase().trim() ?? defaultId.toUpperCase().trim();
    return LabProgressRecord(
      labId: lid,
      packageId: json['package_id'] as String? ?? 'zitera-lab-${lid.toLowerCase()}',
      lessonStatus: json['lesson_status'] as String? ?? 'NOT_STARTED',
      practiceStatus: json['practice_status'] as String? ?? 'NOT_STARTED',
      challengeStatus: json['challenge_status'] as String? ?? 'UNSOLVED',
      challengeAttempts: (json['challenge_attempts'] as num?)?.toInt() ?? 0,
      completedAt: json['completed_at'] as String?,
      lastOpenedAt: json['last_opened_at'] as String? ?? DateTime.now().toIso8601String(),
    );
  }
}

/// Local Progress Manager (Phase 6 & 19 Security & Extensibility principles).
/// Strictly adheres to SEC rules: The correct flag is NEVER stored in
/// Flutter local progress, UI state, normal logs, or UI assets.
/// Progress tracks interactions: completed labs, challenges, practice, sections, and attempts.
class ProgressManager {
  static const String _fileName = 'zitera_progress.json';

  static File _getProgressFile() {
    return File(_fileName);
  }

  static String normalizeId(String rawId) {
    var id = rawId.trim();
    if (id.toLowerCase().startsWith('zitera-lab-')) {
      id = id.substring('zitera-lab-'.length);
    } else if (id.toLowerCase().startsWith('zitera-')) {
      id = id.substring('zitera-'.length);
    }
    return id.toUpperCase();
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
    final tmpFile = File('${file.path}.tmp');

    String? rawJson;

    if (tmpFile.existsSync()) {
      try {
        final tmpText = tmpFile.readAsStringSync();
        if (tmpText.trim().isNotEmpty) {
          rawJson = tmpText;
          try {
            if (file.existsSync()) {
              try {
                file.deleteSync();
              } catch (_) {}
            }
            tmpFile.copySync(file.path);
            try {
              tmpFile.deleteSync();
            } catch (_) {}
          } catch (_) {
            try {
              tmpFile.renameSync(file.path);
            } catch (_) {}
          }
        }
      } catch (_) {}
    }

    if (rawJson == null) {
      if (!file.existsSync()) {
        return _defaultProgress();
      }
      try {
        rawJson = await file.readAsString();
      } catch (_) {
        return _defaultProgress();
      }
    }

    try {
      final text = rawJson;
      if (text.trim().isEmpty) {
        return _defaultProgress();
      }
      final decoded = jsonDecode(text) as Map<String, dynamic>;
      // Sanitize: ensure legacy 'solved_flags' with secret plain texts is purged
      decoded.remove('solved_flags');

      // Deduplicate lists and ensure valid schema
      final labs = (decoded['completed_labs'] as List? ?? [])
          .map((e) => normalizeId(e.toString()))
          .where((id) => id.isNotEmpty && id.length <= 16)
          .toSet()
          .toList();
      final challenges = (decoded['completed_challenges'] as List? ?? [])
          .map((e) => normalizeId(e.toString()))
          .where((id) => id.isNotEmpty && id.length <= 16)
          .toSet()
          .toList();
      final practice = (decoded['completed_practice'] as List? ?? [])
          .map((e) => normalizeId(e.toString()))
          .where((id) => id.isNotEmpty && id.length <= 16)
          .toSet()
          .toList();

      decoded['completed_labs'] = labs;
      decoded['completed_challenges'] = challenges;
      decoded['completed_practice'] = practice;

      // Ensure lab_records exists
      if (!decoded.containsKey('lab_records') || decoded['lab_records'] is! Map) {
        decoded['lab_records'] = <String, dynamic>{};
      }

      return decoded;
    } catch (_) {
      // Automatic corruption recovery: backup corrupted file and fallback to clean state
      try {
        if (file.existsSync()) {
          file.copySync('${file.path}.bak');
          try {
            file.deleteSync();
          } catch (_) {}
        }
      } catch (_) {}
      return _defaultProgress();
    }
  }

  static Map<String, dynamic> _defaultProgress() {
    return {
      'language': 'en',
      'completed_labs': <String>[],
      'completed_challenges': <String>[],
      'completed_practice': <String>[],
      'completed_sections': <String, dynamic>{},
      'lab_records': <String, dynamic>{},
      'last_updated': DateTime.now().toIso8601String(),
    };
  }

  static Future<String> getLanguage() async {
    final data = await loadProgress();
    return (data['language'] as String?) ?? 'en';
  }

  static Future<void> setLanguage(String code) async {
    final data = await loadProgress();
    data['language'] = code;
    data['last_updated'] = DateTime.now().toIso8601String();
    await _atomicWrite(data);
  }

  static Future<void> recordLabOpened(String labId) async {
    final cleanId = normalizeId(labId);
    final data = await loadProgress();
    final records = Map<String, dynamic>.from(data['lab_records'] as Map? ?? {});
    final existing = records[cleanId] is Map<String, dynamic>
        ? LabProgressRecord.fromJson(records[cleanId] as Map<String, dynamic>, cleanId)
        : LabProgressRecord(
            labId: cleanId,
            packageId: 'zitera-lab-${cleanId.toLowerCase()}',
            lastOpenedAt: DateTime.now().toIso8601String(),
          );

    records[cleanId] = LabProgressRecord(
      labId: existing.labId,
      packageId: existing.packageId,
      lessonStatus: existing.lessonStatus,
      practiceStatus: existing.practiceStatus,
      challengeStatus: existing.challengeStatus,
      challengeAttempts: existing.challengeAttempts,
      completedAt: existing.completedAt,
      lastOpenedAt: DateTime.now().toIso8601String(),
    ).toJson();

    data['lab_records'] = records;
    data['last_updated'] = DateTime.now().toIso8601String();
    await _atomicWrite(data);
  }

  static Future<void> recordChallengeAttempt(String labId) async {
    final cleanId = normalizeId(labId);
    final data = await loadProgress();
    final records = Map<String, dynamic>.from(data['lab_records'] as Map? ?? {});
    final existing = records[cleanId] is Map<String, dynamic>
        ? LabProgressRecord.fromJson(records[cleanId] as Map<String, dynamic>, cleanId)
        : LabProgressRecord(
            labId: cleanId,
            packageId: 'zitera-lab-${cleanId.toLowerCase()}',
            lastOpenedAt: DateTime.now().toIso8601String(),
          );

    records[cleanId] = LabProgressRecord(
      labId: existing.labId,
      packageId: existing.packageId,
      lessonStatus: existing.lessonStatus,
      practiceStatus: existing.practiceStatus,
      challengeStatus: existing.challengeStatus,
      challengeAttempts: existing.challengeAttempts + 1,
      completedAt: existing.completedAt,
      lastOpenedAt: existing.lastOpenedAt,
    ).toJson();

    data['lab_records'] = records;
    data['last_updated'] = DateTime.now().toIso8601String();
    await _atomicWrite(data);
  }

  static Future<void> markLabCompleted(String labId) async {
    final cleanId = normalizeId(labId);
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
    final cleanId = normalizeId(labId);
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

    // Update structured record
    final records = Map<String, dynamic>.from(data['lab_records'] as Map? ?? {});
    final existing = records[cleanId] is Map<String, dynamic>
        ? LabProgressRecord.fromJson(records[cleanId] as Map<String, dynamic>, cleanId)
        : LabProgressRecord(
            labId: cleanId,
            packageId: 'zitera-lab-${cleanId.toLowerCase()}',
            lastOpenedAt: DateTime.now().toIso8601String(),
          );

    records[cleanId] = LabProgressRecord(
      labId: existing.labId,
      packageId: existing.packageId,
      lessonStatus: existing.lessonStatus,
      practiceStatus: existing.practiceStatus,
      challengeStatus: 'SOLVED',
      challengeAttempts: existing.challengeAttempts > 0 ? existing.challengeAttempts : 1,
      completedAt: DateTime.now().toIso8601String(),
      lastOpenedAt: DateTime.now().toIso8601String(),
    ).toJson();
    data['lab_records'] = records;

    if (changed || true) {
      data['last_updated'] = DateTime.now().toIso8601String();
      await _atomicWrite(data);
    }
  }

  /// Mark practice mode completed for a given lab.
  static Future<void> markPracticeCompleted(String labId) async {
    final cleanId = normalizeId(labId);
    final data = await loadProgress();
    final practice = List<String>.from(data['completed_practice'] as List? ?? []);
    if (!practice.contains(cleanId)) {
      practice.add(cleanId);
      data['completed_practice'] = practice;
    }

    final records = Map<String, dynamic>.from(data['lab_records'] as Map? ?? {});
    final existing = records[cleanId] is Map<String, dynamic>
        ? LabProgressRecord.fromJson(records[cleanId] as Map<String, dynamic>, cleanId)
        : LabProgressRecord(
            labId: cleanId,
            packageId: 'zitera-lab-${cleanId.toLowerCase()}',
            lastOpenedAt: DateTime.now().toIso8601String(),
          );

    records[cleanId] = LabProgressRecord(
      labId: existing.labId,
      packageId: existing.packageId,
      lessonStatus: existing.lessonStatus,
      practiceStatus: 'COMPLETED',
      challengeStatus: existing.challengeStatus,
      challengeAttempts: existing.challengeAttempts,
      completedAt: existing.completedAt,
      lastOpenedAt: DateTime.now().toIso8601String(),
    ).toJson();
    data['lab_records'] = records;

    data['last_updated'] = DateTime.now().toIso8601String();
    await _atomicWrite(data);
  }

  /// Mark learning section viewed/completed for interaction-based progress.
  static Future<void> markSectionCompleted(String labId, String section) async {
    final cleanId = normalizeId(labId);
    final data = await loadProgress();
    final sectionsMap = Map<String, dynamic>.from(data['completed_sections'] as Map? ?? {});
    final labSections = List<String>.from(sectionsMap[cleanId] as List? ?? []);

    if (!labSections.contains(section)) {
      labSections.add(section);
      sectionsMap[cleanId] = labSections;
      data['completed_sections'] = sectionsMap;
    }

    final records = Map<String, dynamic>.from(data['lab_records'] as Map? ?? {});
    final existing = records[cleanId] is Map<String, dynamic>
        ? LabProgressRecord.fromJson(records[cleanId] as Map<String, dynamic>, cleanId)
        : LabProgressRecord(
            labId: cleanId,
            packageId: 'zitera-lab-${cleanId.toLowerCase()}',
            lastOpenedAt: DateTime.now().toIso8601String(),
          );

    records[cleanId] = LabProgressRecord(
      labId: existing.labId,
      packageId: existing.packageId,
      lessonStatus: labSections.length >= 3 ? 'COMPLETED' : 'IN_PROGRESS',
      practiceStatus: existing.practiceStatus,
      challengeStatus: existing.challengeStatus,
      challengeAttempts: existing.challengeAttempts,
      completedAt: existing.completedAt,
      lastOpenedAt: DateTime.now().toIso8601String(),
    ).toJson();
    data['lab_records'] = records;

    data['last_updated'] = DateTime.now().toIso8601String();
    await _atomicWrite(data);
  }

  static Future<bool> isChallengeCompleted(String labId) async {
    final data = await loadProgress();
    final cleanId = normalizeId(labId);
    final list = List<String>.from(data['completed_challenges'] as List? ?? []);
    return list.contains(cleanId);
  }

  static Future<bool> isPracticeCompleted(String labId) async {
    final data = await loadProgress();
    final cleanId = normalizeId(labId);
    final list = List<String>.from(data['completed_practice'] as List? ?? []);
    return list.contains(cleanId);
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
