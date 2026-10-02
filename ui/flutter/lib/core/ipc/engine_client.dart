import 'dart:convert';
import 'dart:io';
import 'models.dart';

class ZiteraEngineClient {
  static String? _cachedEnginePath;

  static String findEngineExecutable() {
    if (_cachedEnginePath != null && File(_cachedEnginePath!).existsSync()) {
      return _cachedEnginePath!;
    }

    final exeDir = File(Platform.resolvedExecutable).parent.path;
    final currentDir = Directory.current.path;
    final candidates = [
      // Sibling in release package (relative to executable)
      '$exeDir\\zitera-engine.exe',
      '$exeDir\\engine\\zitera-engine.exe',
      // Sibling in working directory
      '$currentDir\\zitera-engine.exe',
      '$currentDir\\engine\\zitera-engine.exe',
      // Running from workspace root (dev)
      '$currentDir\\engine\\rust\\target\\release\\zitera-engine.exe',
      '$currentDir\\engine\\rust\\target\\debug\\zitera-engine.exe',
      // Running from ui/flutter (dev)
      '$currentDir\\..\\..\\engine\\rust\\target\\release\\zitera-engine.exe',
      '$currentDir\\..\\..\\engine\\rust\\target\\debug\\zitera-engine.exe',
      // Direct binary on PATH
      'zitera-engine.exe',
    ];

    for (final path in candidates) {
      if (File(path).existsSync()) {
        _cachedEnginePath = File(path).absolute.path;
        return _cachedEnginePath!;
      }
    }

    return 'zitera-engine.exe';
  }

  static Future<dynamic> executeCommandRaw(List<String> args) async {
    final engine = findEngineExecutable();
    final fullArgs = ['--json', ...args];

    try {
      final result = await Process.run(engine, fullArgs);
      if (result.stdout == null || result.stdout.toString().trim().isEmpty) {
        final err = result.stderr.toString().trim();
        throw Exception(err.isNotEmpty ? err : 'Engine produced empty response.');
      }

      final jsonMap = jsonDecode(result.stdout.toString().trim()) as Map<String, dynamic>;
      final success = jsonMap['success'] as bool? ?? false;
      if (!success) {
        final err = jsonMap['error'] as Map<String, dynamic>?;
        final msg = err?['message'] ?? 'Engine operation failed.';
        throw Exception(msg);
      }

      return jsonMap['data'];
    } on FormatException {
      throw Exception('Failed to parse response from ZITERA engine.');
    } catch (e) {
      rethrow;
    }
  }

  static Future<Map<String, dynamic>> executeCommand(List<String> args) async {
    final raw = await executeCommandRaw(args);
    if (raw is Map<String, dynamic>) {
      return raw;
    }
    return <String, dynamic>{};
  }

  static Future<DiagnosticsResult> runDoctor() async {
    final data = await executeCommand(['doctor']);
    return DiagnosticsResult.fromJson(data);
  }

  static Future<List<ToolItem>> getTools() async {
    final data = await executeCommandRaw(['tool', 'list']);
    final list = data as List<dynamic>? ?? [];
    return list.map((item) => ToolItem.fromJson(item as Map<String, dynamic>)).toList();
  }

  static Future<List<LabItem>> getLabs() async {
    final data = await executeCommandRaw(['lab', 'list']);
    final list = data as List<dynamic>? ?? [];
    return list.map((item) => LabItem.fromJson(item as Map<String, dynamic>)).toList();
  }

  static Future<LabItem> getLabStatus(String id) async {
    final data = await executeCommand(['lab', 'status', id]);
    return LabItem.fromJson(data);
  }

  static Future<String> startLab(String id) async {
    final data = await executeCommandRaw(['lab', 'start', id]);
    return data as String? ?? 'Lab started.';
  }

  static Future<String> stopLab(String id) async {
    final data = await executeCommandRaw(['lab', 'stop', id]);
    return data as String? ?? 'Lab stopped.';
  }

  static Future<String> resetLab(String id) async {
    final data = await executeCommandRaw(['lab', 'reset', id]);
    return data as String? ?? 'Lab reset.';
  }

  static Future<String> installLab(String id) async {
    final data = await executeCommandRaw(['lab', 'install', id]);
    return data as String? ?? 'Lab installed.';
  }

  static Future<String> updateLab(String id) async {
    final data = await executeCommandRaw(['lab', 'update', id]);
    return data as String? ?? 'Lab updated.';
  }

  static Future<String> removeLab(String id) async {
    final data = await executeCommandRaw(['lab', 'remove', id]);
    return data as String? ?? 'Lab removed.';
  }

  static Future<List<CatalogEntry>> getCatalog() async {
    final data = await executeCommandRaw(['catalog']);
    final catData = data as Map<String, dynamic>? ?? {};
    final list = catData['labs'] as List<dynamic>? ?? [];
    return list.map((item) => CatalogEntry.fromJson(item as Map<String, dynamic>)).toList();
  }

  static Future<LabContent> getLabContent(String id) async {
    final data = await executeCommand(['lab', 'content', id]);
    return LabContent.fromJson(data);
  }

  static Future<ChallengeResult> validateChallenge(String id, String flag) async {
    final data = await executeCommand(['lab', 'validate-challenge', id, flag]);
    return ChallengeResult.fromJson(data);
  }

  static Future<PracticeVerificationResult> verifyPractice(String id) async {
    final data = await executeCommand(['lab', 'practice-verify', id]);
    return PracticeVerificationResult.fromJson(data);
  }

  static Future<ToolItem> getToolStatus(String id) async {
    final data = await executeCommand(['tool', 'status', id]);
    return ToolItem.fromJson(data);
  }

  static Future<ToolInstallResult> installTool(String id) async {
    final data = await executeCommand(['tool', 'install', id]);
    return ToolInstallResult.fromJson(data);
  }
}
