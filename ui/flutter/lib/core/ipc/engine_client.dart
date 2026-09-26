import 'dart:convert';
import 'dart:io';
import 'models.dart';

class ZiteraEngineClient {
  static String? _cachedEnginePath;

  static String findEngineExecutable() {
    if (_cachedEnginePath != null && File(_cachedEnginePath!).existsSync()) {
      return _cachedEnginePath!;
    }

    final currentDir = Directory.current.path;
    final candidates = [
      // Running from workspace root
      '$currentDir\\engine\\rust\\target\\release\\zitera-engine.exe',
      '$currentDir\\engine\\rust\\target\\debug\\zitera-engine.exe',
      // Running from ui/flutter
      '$currentDir\\..\\..\\engine\\rust\\target\\release\\zitera-engine.exe',
      '$currentDir\\..\\..\\engine\\rust\\target\\debug\\zitera-engine.exe',
      // Direct binary
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

  static Future<Map<String, dynamic>> executeCommand(List<String> args) async {
    final engine = findEngineExecutable();
    final fullArgs = ['--json', ...args];

    try {
      final result = await Process.run(engine, fullArgs);
      if (result.stdout == null || result.stdout.toString().trim().isEmpty) {
        throw Exception('Engine produced empty response. Stderr: ${result.stderr}');
      }

      final jsonMap = jsonDecode(result.stdout.toString().trim()) as Map<String, dynamic>;
      final success = jsonMap['success'] as bool? ?? false;
      if (!success) {
        final err = jsonMap['error'] as Map<String, dynamic>?;
        final msg = err?['message'] ?? 'Unknown engine error occurred';
        throw Exception(msg);
      }

      return jsonMap['data'] as Map<String, dynamic>? ?? {};
    } catch (e) {
      rethrow;
    }
  }

  static Future<DiagnosticsResult> runDoctor() async {
    final data = await executeCommand(['doctor']);
    return DiagnosticsResult.fromJson(data);
  }

  static Future<List<ToolItem>> getTools() async {
    final engine = findEngineExecutable();
    final result = await Process.run(engine, ['--json', 'tool', 'list']);
    final jsonMap = jsonDecode(result.stdout.toString().trim()) as Map<String, dynamic>;
    final list = jsonMap['data'] as List<dynamic>? ?? [];
    return list.map((item) => ToolItem.fromJson(item as Map<String, dynamic>)).collect();
  }

  static Future<List<LabItem>> getLabs() async {
    final engine = findEngineExecutable();
    final result = await Process.run(engine, ['--json', 'lab', 'list']);
    final jsonMap = jsonDecode(result.stdout.toString().trim()) as Map<String, dynamic>;
    final list = jsonMap['data'] as List<dynamic>? ?? [];
    return list.map((item) => LabItem.fromJson(item as Map<String, dynamic>)).collect();
  }

  static Future<LabItem> getLabStatus(String id) async {
    final data = await executeCommand(['lab', 'status', id]);
    return LabItem.fromJson(data);
  }

  static Future<String> startLab(String id) async {
    final engine = findEngineExecutable();
    final result = await Process.run(engine, ['--json', 'lab', 'start', id]);
    final jsonMap = jsonDecode(result.stdout.toString().trim()) as Map<String, dynamic>;
    if (jsonMap['success'] == true) {
      return jsonMap['data'] as String? ?? 'Lab started.';
    }
    final err = jsonMap['error'] as Map<String, dynamic>?;
    throw Exception(err?['message'] ?? 'Failed to start lab.');
  }

  static Future<String> stopLab(String id) async {
    final engine = findEngineExecutable();
    final result = await Process.run(engine, ['--json', 'lab', 'stop', id]);
    final jsonMap = jsonDecode(result.stdout.toString().trim()) as Map<String, dynamic>;
    if (jsonMap['success'] == true) {
      return jsonMap['data'] as String? ?? 'Lab stopped.';
    }
    final err = jsonMap['error'] as Map<String, dynamic>?;
    throw Exception(err?['message'] ?? 'Failed to stop lab.');
  }

  static Future<String> resetLab(String id) async {
    final engine = findEngineExecutable();
    final result = await Process.run(engine, ['--json', 'lab', 'reset', id]);
    final jsonMap = jsonDecode(result.stdout.toString().trim()) as Map<String, dynamic>;
    if (jsonMap['success'] == true) {
      return jsonMap['data'] as String? ?? 'Lab reset.';
    }
    final err = jsonMap['error'] as Map<String, dynamic>?;
    throw Exception(err?['message'] ?? 'Failed to reset lab.');
  }

  static Future<List<CatalogEntry>> getCatalog() async {
    final engine = findEngineExecutable();
    final result = await Process.run(engine, ['--json', 'catalog']);
    final jsonMap = jsonDecode(result.stdout.toString().trim()) as Map<String, dynamic>;
    final catData = jsonMap['data'] as Map<String, dynamic>? ?? {};
    final list = catData['labs'] as List<dynamic>? ?? [];
    return list.map((item) => CatalogEntry.fromJson(item as Map<String, dynamic>)).collect();
  }
}

extension IterableExtension<T> on Iterable<T> {
  List<T> collect() => toList();
}
