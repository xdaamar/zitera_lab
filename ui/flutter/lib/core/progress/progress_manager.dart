import 'dart:convert';
import 'dart:io';

class ProgressManager {
  static const String _fileName = 'zitera_progress.json';

  static File _getProgressFile() {
    return File(_fileName);
  }

  static Future<Map<String, dynamic>> loadProgress() async {
    final file = _getProgressFile();
    if (!file.existsSync()) {
      return {
        'solved_flags': <String>[],
        'completed_labs': <String>[],
        'last_updated': DateTime.now().toIso8601String(),
      };
    }
    try {
      final text = await file.readAsString();
      return jsonDecode(text) as Map<String, dynamic>;
    } catch (_) {
      return {
        'solved_flags': <String>[],
        'completed_labs': <String>[],
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

  static Future<void> markFlagSolved(String labId, String flag) async {
    final data = await loadProgress();
    final list = List<String>.from(data['solved_flags'] as List? ?? []);
    if (!list.contains(flag)) {
      list.add(flag);
      data['solved_flags'] = list;
      await _getProgressFile().writeAsString(jsonEncode(data));
    }
    await markLabCompleted(labId);
  }

  static Future<void> resetAll() async {
    final file = _getProgressFile();
    if (file.existsSync()) {
      await file.delete();
    }
  }
}
