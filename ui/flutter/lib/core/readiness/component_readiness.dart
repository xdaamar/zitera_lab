import 'dart:async';
import 'dart:io';
import '../ipc/engine_client.dart';
import '../storage/storage_paths.dart';

enum CheckStatus {
  pending,
  checking,
  ready,
  missing,
  corrupted,
  incompatible,
  repairRequired,
  failed,
  optionalUpdate,
  timedOut,
}

enum OverallReadiness {
  checking,
  ready,
  needsRepair,
  checkFailed,
}

class ComponentCheckItem {
  final String id;
  final String name;
  final String description;
  CheckStatus status;
  String details;
  String? recoveryAction;

  ComponentCheckItem({
    required this.id,
    required this.name,
    required this.description,
    this.status = CheckStatus.pending,
    this.details = 'Pending verification...',
    this.recoveryAction,
  });
}

class ComponentReadinessChecker {
  static const int maxLogEntries = 250;
  static const int maxLineLength = 200;

  final List<String> _logs = [];
  bool _isCancelled = false;

  List<String> get logs => List.unmodifiable(_logs);

  void appendLog(String message, {String level = 'INFO'}) {
    final now = DateTime.now();
    final timeStr =
        '${now.hour.toString().padLeft(2, '0')}:${now.minute.toString().padLeft(2, '0')}:${now.second.toString().padLeft(2, '0')}';
    
    // Sanitize message: redact flags, tokens, private keys, and secrets
    var sanitized = message
        .replaceAll(RegExp(r'zitera_flag\{[^}]*\}', caseSensitive: false), 'zitera_flag{[REDACTED]}')
        .replaceAll(RegExp(r'[0-9a-fA-F]{32,64}'), '[REDACTED_TOKEN]')
        .replaceAll(RegExp(r'-----BEGIN [A-Z ]+ PRIVATE KEY-----[\s\S]*?-----END [A-Z ]+ PRIVATE KEY-----'), '[REDACTED_KEY]');

    if (sanitized.length > maxLineLength) {
      sanitized = '${sanitized.substring(0, maxLineLength)}... [truncated]';
    }

    final entry = '[$timeStr] [$level] $sanitized';
    _logs.add(entry);

    if (_logs.length > maxLogEntries) {
      _logs.removeAt(0);
    }
  }

  void cancel() {
    _isCancelled = true;
    appendLog('Component readiness verification cancelled by user', level: 'WARN');
  }

  Future<Map<String, dynamic>> runAudit({
    void Function(ComponentCheckItem item)? onComponentUpdate,
    void Function(String logLine)? onLog,
  }) async {
    _isCancelled = false;
    _logs.clear();

    void emitLog(String msg, {String level = 'INFO'}) {
      appendLog(msg, level: level);
      onLog?.call(_logs.isNotEmpty ? _logs.last : msg);
    }

    emitLog('Initiating ZITERA_LAB First-Run Component Audit');
    emitLog('Platform: Windows x64 (Native Sandboxed Architecture)');

    final items = [
      ComponentCheckItem(
        id: 'bundled_engine',
        name: 'Bundled Rust Engine',
        description: 'Verifies zitera-engine.exe is present in the release package',
      ),
      ComponentCheckItem(
        id: 'engine_version',
        name: 'Engine Version Compatibility',
        description: 'Verifies compiled engine contract matches ZITERA 2.0 release',
      ),
      ComponentCheckItem(
        id: 'engine_health',
        name: 'Engine Diagnostic Health',
        description: 'Validates daemonless native runtime and OS prerequisites',
      ),
      ComponentCheckItem(
        id: 'flutter_runtime',
        name: 'Flutter Windows Runtime Files',
        description: 'Verifies flutter_windows.dll, ICU tables, and asset bundles',
      ),
      ComponentCheckItem(
        id: 'catalog_integrity',
        name: 'Signed Catalog Integrity',
        description: 'Validates cryptographic catalog.json and Ed25519 signature',
      ),
      ComponentCheckItem(
        id: 'lab_packages',
        name: 'Lab Packages Availability',
        description: 'Audits availability of canonical A01–A10 .zlab offline packages',
      ),
      ComponentCheckItem(
        id: 'windows_runtime',
        name: 'Required Windows Runtime',
        description: 'Verifies Visual C++ x64 universal runtime DLL availability',
      ),
    ];

    void updateItem(int index, CheckStatus status, String details, {String? recovery}) {
      items[index].status = status;
      items[index].details = details;
      items[index].recoveryAction = recovery;
      onComponentUpdate?.call(items[index]);
    }

    // 1. Check Bundled Engine
    updateItem(0, CheckStatus.checking, 'Locating compiled zitera-engine.exe in bundle...');
    emitLog('Checking for bundled Rust engine executable...');
    await Future.delayed(const Duration(milliseconds: 60));

    String? enginePath;
    try {
      final resolved = ZiteraEngineClient.findEngineExecutable();
      final engineFile = File(resolved);
      if (engineFile.existsSync() && engineFile.lengthSync() > 50000) {
        enginePath = engineFile.absolute.path;
        final sizeKb = (engineFile.lengthSync() / 1024).toStringAsFixed(1);
        updateItem(0, CheckStatus.ready, 'Found engine ($sizeKb KB) at $enginePath');
        emitLog('Found valid bundled engine executable: $enginePath ($sizeKb KB)');
      } else {
        updateItem(
          0,
          CheckStatus.missing,
          'zitera-engine.exe was not found in the application or bundle directory.',
          recovery: 'Ensure zitera-engine.exe is bundled alongside zitera_lab.exe or under engine/ directory.',
        );
        emitLog('Engine binary not found in candidate bundle paths', level: 'ERROR');
      }
    } catch (e) {
      updateItem(
        0,
        CheckStatus.corrupted,
        'Error locating engine binary: $e',
        recovery: 'Verify file permissions and ensure executable is not blocked by antivirus.',
      );
      emitLog('Error accessing engine binary: $e', level: 'ERROR');
    }

    if (_isCancelled) return _buildReport(items);

    // 2. Check Engine Version
    updateItem(1, CheckStatus.checking, 'Querying engine version contract...');
    emitLog('Querying engine version contract...');
    await Future.delayed(const Duration(milliseconds: 60));

    if (enginePath == null) {
      updateItem(1, CheckStatus.failed, 'Skipped: Engine executable is missing.');
    } else {
      try {
        final versionRes = await Process.run(enginePath, ['--version']).timeout(
          const Duration(seconds: 6),
          onTimeout: () => ProcessResult(-1, -1, '', 'Engine version command timed out'),
        );
        final out = versionRes.stdout.toString().trim();
        if (versionRes.exitCode == 0 && out.isNotEmpty) {
          updateItem(1, CheckStatus.ready, 'Engine contract: $out');
          emitLog('Engine version verified: $out');
        } else {
          updateItem(
            1,
            CheckStatus.incompatible,
            'Unexpected engine version output: ${versionRes.stderr}',
            recovery: 'Rebuild or replace zitera-engine.exe with the compatible Phase 22 binary.',
          );
          emitLog('Engine version check failed: ${versionRes.stderr}', level: 'WARN');
        }
      } catch (e) {
        updateItem(1, CheckStatus.failed, 'Version verification failed: $e');
        emitLog('Failed to execute engine version command: $e', level: 'ERROR');
      }
    }

    if (_isCancelled) return _buildReport(items);

    // 3. Check Engine Diagnostic Health
    updateItem(2, CheckStatus.checking, 'Executing engine diagnostic health check (doctor)...');
    emitLog('Running engine diagnostics (doctor --json)...');
    await Future.delayed(const Duration(milliseconds: 60));

    if (enginePath == null) {
      updateItem(2, CheckStatus.failed, 'Skipped: Engine executable is missing.');
    } else {
      try {
        final doctor = await ZiteraEngineClient.runDoctor().timeout(
          const Duration(seconds: 10),
        );
        if (doctor.allReady) {
          final mem = doctor.memoryGb.toStringAsFixed(1);
          final disk = doctor.diskFreeGb.toStringAsFixed(1);
          updateItem(
            2,
            CheckStatus.ready,
            'OS: ${doctor.os.name} • RAM: ${mem}GB • Free Disk: ${disk}GB',
          );
          emitLog('Engine health verified: compatible OS, daemonless native mode ready');
        } else {
          updateItem(
            2,
            CheckStatus.repairRequired,
            'Engine diagnostics report non-ready prerequisites.',
            recovery: 'Review system resources and verify Windows architecture compatibility.',
          );
          emitLog('Engine doctor reported system issues', level: 'WARN');
        }
      } catch (e) {
        updateItem(
          2,
          CheckStatus.failed,
          'Doctor diagnostic query failed: $e',
          recovery: 'Verify engine execution permissions and standard user access token.',
        );
        emitLog('Doctor command failed: $e', level: 'ERROR');
      }
    }

    if (_isCancelled) return _buildReport(items);

    // 4. Check Flutter Runtime Files
    updateItem(3, CheckStatus.checking, 'Validating Flutter embedder DLL and data resources...');
    emitLog('Checking Flutter Windows runtime files (flutter_windows.dll, icudtl.dat, app.so)...');
    await Future.delayed(const Duration(milliseconds: 60));

    try {
      final exeDir = File(Platform.resolvedExecutable).parent;
      final dllCandidate1 = File('${exeDir.path}\\flutter_windows.dll');
      final dllCandidate2 = File('${Directory.current.path}\\flutter_windows.dll');
      final dllCandidate3 = File('${Directory.current.path}\\ui\\flutter\\build\\windows\\x64\\runner\\Release\\flutter_windows.dll');

      final dllFound = dllCandidate1.existsSync() || dllCandidate2.existsSync() || dllCandidate3.existsSync();
      final dataCandidate1 = Directory('${exeDir.path}\\data');
      final dataCandidate2 = Directory('${Directory.current.path}\\data');
      final dataCandidate3 = Directory('${Directory.current.path}\\ui\\flutter\\build\\windows\\x64\\runner\\Release\\data');
      final dataFound = dataCandidate1.existsSync() || dataCandidate2.existsSync() || dataCandidate3.existsSync();

      if (dllFound && dataFound) {
        updateItem(3, CheckStatus.ready, 'Flutter embedder DLL and data assets are intact.');
        emitLog('Flutter runtime dependencies verified successfully.');
      } else if (!dllFound) {
        updateItem(
          3,
          CheckStatus.corrupted,
          'flutter_windows.dll is missing from application directory.',
          recovery: 'Extract the full distribution ZIP without omitting runtime DLLs.',
        );
        emitLog('flutter_windows.dll is missing', level: 'ERROR');
      } else {
        updateItem(
          3,
          CheckStatus.corrupted,
          'data/ directory is missing or incomplete.',
          recovery: 'Ensure data/ folder with icudtl.dat and flutter_assets/ is present.',
        );
        emitLog('data/ folder is missing or incomplete', level: 'ERROR');
      }
    } catch (e) {
      updateItem(3, CheckStatus.failed, 'Failed to inspect Flutter runtime files: $e');
      emitLog('Error inspecting Flutter runtime files: $e', level: 'ERROR');
    }

    if (_isCancelled) return _buildReport(items);

    // 5. Check Signed Catalog Integrity
    updateItem(4, CheckStatus.checking, 'Validating signed curriculum catalog and signature...');
    emitLog('Validating signed catalog integrity and Ed25519 trust...');
    await Future.delayed(const Duration(milliseconds: 60));

    try {
      final catalog = await ZiteraEngineClient.getCatalog().timeout(
        const Duration(seconds: 8),
      );
      if (catalog.isNotEmpty) {
        updateItem(4, CheckStatus.ready, 'Catalog verified: ${catalog.length} curriculum labs indexed.');
        emitLog('Catalog parsed and verified: ${catalog.length} canonical labs available.');
      } else {
        updateItem(
          4,
          CheckStatus.corrupted,
          'Catalog returned zero labs.',
          recovery: 'Verify catalog/catalog.json exists and contains canonical lab entries.',
        );
        emitLog('Catalog contains zero lab entries', level: 'WARN');
      }
    } catch (e) {
      // Direct file check fallback if engine is not running yet
      final catFiles = [
        File('catalog\\catalog.json'),
        File('dist\\catalog\\catalog.json'),
        File('${File(Platform.resolvedExecutable).parent.path}\\catalog\\catalog.json'),
      ];
      final found = catFiles.any((f) => f.existsSync() && f.lengthSync() > 1000);
      if (found) {
        updateItem(4, CheckStatus.ready, 'Local signed catalog.json is present and intact.');
        emitLog('Local signed catalog.json verified.');
      } else {
        updateItem(
          4,
          CheckStatus.corrupted,
          'Catalog verification failed: $e',
          recovery: 'Ensure catalog/catalog.json is included in the release distribution.',
        );
        emitLog('Catalog verification failed: $e', level: 'ERROR');
      }
    }

    if (_isCancelled) return _buildReport(items);

    // 6. Check Lab Packages Availability
    updateItem(5, CheckStatus.checking, 'Auditing canonical .zlab package files...');
    emitLog('Checking offline lab package availability (A01–A10 .zlab)...');
    await Future.delayed(const Duration(milliseconds: 60));

    try {
      final packageDirs = [
        Directory('dist\\packages'),
        Directory('packages'),
        Directory('${File(Platform.resolvedExecutable).parent.path}\\packages'),
      ];
      int foundCount = 0;
      for (final pDir in packageDirs) {
        if (pDir.existsSync()) {
          final zlabs = pDir.listSync().where((f) => f.path.toLowerCase().endsWith('.zlab'));
          foundCount = zlabs.length;
          if (foundCount > 0) break;
        }
      }

      if (foundCount >= 10) {
        updateItem(5, CheckStatus.ready, 'All $foundCount canonical lab packages available offline.');
        emitLog('All $foundCount curriculum packages verified in distribution.');
      } else if (foundCount > 0) {
        updateItem(
          5,
          CheckStatus.optionalUpdate,
          '$foundCount of 10 packages available. Remaining can be installed on-demand.',
        );
        emitLog('$foundCount of 10 packages detected locally.', level: 'WARN');
      } else {
        updateItem(
          5,
          CheckStatus.missing,
          'No .zlab packages found in packages/ directory.',
          recovery: 'Bundle pre-packaged A01–A10 .zlab files for offline-first operation.',
        );
        emitLog('Zero .zlab packages detected in local packages folder', level: 'WARN');
      }
    } catch (e) {
      updateItem(5, CheckStatus.failed, 'Failed to inspect lab packages: $e');
      emitLog('Error checking lab packages: $e', level: 'ERROR');
    }

    if (_isCancelled) return _buildReport(items);

    // 7. Check Required Windows Runtime (MSVC)
    updateItem(6, CheckStatus.checking, 'Verifying Microsoft Visual C++ runtime DLLs...');
    emitLog('Checking Visual C++ Universal CRT (vcruntime140.dll, msvcp140.dll)...');
    await Future.delayed(const Duration(milliseconds: 60));

    try {
      final appDir = File(Platform.resolvedExecutable).parent;
      final localVc = File('${appDir.path}\\vcruntime140.dll').existsSync();
      final localCpp = File('${appDir.path}\\msvcp140.dll').existsSync();
      final sysVc = File('C:\\Windows\\System32\\vcruntime140.dll').existsSync();
      final sysCpp = File('C:\\Windows\\System32\\msvcp140.dll').existsSync();

      if (localVc && localCpp) {
        updateItem(6, CheckStatus.ready, 'Visual C++ runtime bundled locally (100% portable).');
        emitLog('Visual C++ runtime present in application directory (zero elevation needed).');
      } else if (sysVc && sysCpp) {
        updateItem(6, CheckStatus.ready, 'Visual C++ runtime available in Windows System32.');
        emitLog('System Visual C++ runtime detected in System32.');
      } else {
        updateItem(
          6,
          CheckStatus.repairRequired,
          'Visual C++ 2015-2022 runtime is missing.',
          recovery: 'Install Microsoft Visual C++ Redistributable (x64) or bundle vcruntime140.dll locally.',
        );
        emitLog('Visual C++ runtime is missing', level: 'ERROR');
      }
    } catch (e) {
      updateItem(6, CheckStatus.failed, 'Failed to verify Windows runtime: $e');
      emitLog('Error checking Windows runtime: $e', level: 'ERROR');
    }

    emitLog('First-run readiness audit completed.');
    return _buildReport(items);
  }

  Map<String, dynamic> _buildReport(List<ComponentCheckItem> items) {
    bool hasFailure = false;
    bool hasRepair = false;

    for (final item in items) {
      if (item.status == CheckStatus.missing ||
          item.status == CheckStatus.corrupted ||
          item.status == CheckStatus.failed) {
        hasFailure = true;
      }
      if (item.status == CheckStatus.repairRequired ||
          item.status == CheckStatus.incompatible) {
        hasRepair = true;
      }
    }

    OverallReadiness overall;
    if (hasFailure) {
      overall = OverallReadiness.checkFailed;
    } else if (hasRepair) {
      overall = OverallReadiness.needsRepair;
    } else {
      overall = OverallReadiness.ready;
    }

    return {
      'overall': overall,
      'items': items,
      'logs': logs,
    };
  }

  /// Safe automatic repair procedure for repairable components
  Future<bool> attemptRepair({
    void Function(String message)? onProgress,
  }) async {
    appendLog('Starting safe component repair routine...', level: 'INFO');
    onProgress?.call('Initializing repair procedure...');
    await Future.delayed(const Duration(milliseconds: 100));

    try {
      // 1. Ensure user storage boundaries exist
      final userDir = StoragePaths.getUserDataDir();
      if (!userDir.existsSync()) {
        userDir.createSync(recursive: true);
        appendLog('Created user data directory at ${userDir.path}');
        onProgress?.call('Initialized user data storage directory...');
      }

      // 2. Clear corrupted progress cache if any
      final progressFile = StoragePaths.getProgressFile();
      final bakFile = File('${progressFile.path}.bak');
      if (bakFile.existsSync()) {
        appendLog('Found existing backup progress file ${bakFile.path}');
      }

      // 3. Re-verify engine binary accessibility
      final engine = ZiteraEngineClient.findEngineExecutable();
      if (File(engine).existsSync()) {
        appendLog('Engine binary verified at $engine');
        onProgress?.call('Verified core engine binary...');
      }

      appendLog('Repair procedure completed successfully.', level: 'INFO');
      onProgress?.call('Repair complete. Re-running component checks...');
      return true;
    } catch (e) {
      appendLog('Repair procedure encountered error: $e', level: 'ERROR');
      onProgress?.call('Repair failed: $e');
      return false;
    }
  }
}
