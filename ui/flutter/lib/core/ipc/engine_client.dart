import 'dart:convert';
import 'dart:io';
import 'models.dart';

class ZiteraEngineClient {
  static String? _cachedEnginePath;

  static String findEngineExecutable() {
    if (_cachedEnginePath != null && File(_cachedEnginePath!).existsSync()) {
      return _cachedEnginePath!;
    }

    // 1. Primary: Search bundled distribution paths relative to Platform.resolvedExecutable
    try {
      final exeDir = File(Platform.resolvedExecutable).parent;
      final bundledCandidates = [
        File('${exeDir.path}${Platform.pathSeparator}engine${Platform.pathSeparator}zitera-engine.exe'),
        File('${exeDir.path}${Platform.pathSeparator}zitera-engine.exe'),
      ];
      for (final f in bundledCandidates) {
        if (f.existsSync() && f.lengthSync() > 50000) {
          _cachedEnginePath = f.absolute.path;
          return _cachedEnginePath!;
        }
      }
    } catch (_) {}

    // 2. Secondary: Search relative to Directory.current (for workspace execution, test runner, CLI)
    try {
      Directory? dir = Directory.current;
      for (int i = 0; i < 6 && dir != null; i++) {
        final candidates = [
          File('${dir.path}${Platform.pathSeparator}engine${Platform.pathSeparator}zitera-engine.exe'),
          File('${dir.path}${Platform.pathSeparator}zitera-engine.exe'),
          File('${dir.path}${Platform.pathSeparator}engine${Platform.pathSeparator}rust${Platform.pathSeparator}target${Platform.pathSeparator}release${Platform.pathSeparator}zitera-engine.exe'),
        ];
        for (final f in candidates) {
          if (f.existsSync() && f.lengthSync() > 50000) {
            _cachedEnginePath = f.absolute.path;
            return _cachedEnginePath!;
          }
        }
        final parent = dir.parent;
        if (parent.path == dir.path) break;
        dir = parent;
      }
    } catch (_) {}

    // Never secretly fallback to arbitrary developer machine paths.
    // Return relative binary name so caller can detect missing state cleanly.
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
        final code = err?['code'] as String? ?? 'OPERATION_FAILED';
        final msg = err?['message'] as String? ?? 'Engine operation failed.';
        final recovery = err?['recovery_action'] as String?;
        final recoverable = err?['recoverable'] as bool? ?? true;
        final details = err?['details'] as String?;
        throw ZiteraException(
          code: code,
          message: msg,
          recoveryAction: recovery,
          recoverable: recoverable,
          details: details,
        );
      }

      return jsonMap['data'];
    } on FormatException {
      throw ZiteraException(
        code: 'PARSE_ERROR',
        message: 'Failed to parse response from ZITERA engine.',
        recoveryAction: 'Verify that the engine executable is intact and not corrupted.',
      );
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

  static Future<Map<String, dynamic>> exportDiagnostics([String? outputPath]) async {
    final args = ['diagnostics', 'export'];
    if (outputPath != null && outputPath.trim().isNotEmpty) {
      args.add(outputPath.trim());
    }
    return await executeCommand(args);
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

  static List<CatalogEntry>? _cachedCatalog;

  static Future<List<CatalogEntry>> getCatalog({bool forceRefresh = false}) async {
    if (!forceRefresh && _cachedCatalog != null) {
      return _cachedCatalog!;
    }
    final data = await executeCommandRaw(['catalog']);
    final catData = data as Map<String, dynamic>? ?? {};
    final list = catData['labs'] as List<dynamic>? ?? [];
    _cachedCatalog = list.map((item) => CatalogEntry.fromJson(item as Map<String, dynamic>)).toList();
    return _cachedCatalog!;
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

  static Future<TerminalResult> executeTerminal(String command, {String? labId}) async {
    final args = <String>['terminal'];
    if (labId != null && labId.isNotEmpty) {
      args.addAll(['--lab', labId]);
    }
    args.add(command);
    final data = await executeCommand(args);
    return TerminalResult.fromJson(data);
  }

  static Future<LabValidationReport> validateLab(String id) async {
    final data = await executeCommand(['lab', 'validate', id]);
    return LabValidationReport.fromJson(data);
  }
}
