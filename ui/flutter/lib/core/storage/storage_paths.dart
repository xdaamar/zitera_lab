import 'dart:io';

/// Formalized Zitera Storage Boundaries & Path Resolver (Phase 20)
///
/// Enforces strict separation between:
/// 1. Application Binaries: %LOCALAPPDATA%\Programs\ZiteraLab
/// 2. User State & Progress: %LOCALAPPDATA%\ZiteraLab\user
/// 3. Installed Lab Packages: %LOCALAPPDATA%\ZiteraLab\labs
/// 4. Temporary Cache: %LOCALAPPDATA%\ZiteraLab\cache
/// 5. Execution Logs: %LOCALAPPDATA%\ZiteraLab\logs
/// 6. Sanitized Diagnostics: %LOCALAPPDATA%\ZiteraLab\diagnostics
class StoragePaths {
  static const String defaultProgressFileName = 'progress.json';
  static const String legacyProgressFileName = 'zitera_progress.json';

  /// Resolves the canonical user data directory (%LOCALAPPDATA%\ZiteraLab\user)
  static Directory getUserDataDir() {
    final customUserDir = Platform.environment['ZITERA_USER_DIR'];
    if (customUserDir != null && customUserDir.trim().isNotEmpty) {
      return Directory(customUserDir.trim());
    }

    final customDataDir = Platform.environment['ZITERA_DATA_DIR'];
    if (customDataDir != null && customDataDir.trim().isNotEmpty) {
      return Directory('${customDataDir.trim()}${Platform.pathSeparator}user');
    }

    final localAppData = Platform.environment['LOCALAPPDATA'];
    if (localAppData != null && localAppData.trim().isNotEmpty) {
      return Directory(
        '${localAppData.trim()}${Platform.pathSeparator}ZiteraLab${Platform.pathSeparator}user',
      );
    }

    return Directory('.zitera_user');
  }

  /// Resolves the progress file with graceful backward compatibility for legacy progress.
  static File getProgressFile() {
    // 1. Explicit environment override for testing or headless execution
    final explicitFile = Platform.environment['ZITERA_PROGRESS_FILE'] ??
        Platform.environment['ZITERA_USER_PROGRESS_FILE'];
    if (explicitFile != null && explicitFile.trim().isNotEmpty) {
      return File(explicitFile.trim());
    }

    // 2. If legacy progress file exists in current directory and canonical doesn't yet, preserve legacy
    final legacyFile = File(legacyProgressFileName);
    final userDir = getUserDataDir();
    final canonicalFile = File(
      '${userDir.path}${Platform.pathSeparator}$defaultProgressFileName',
    );

    // If canonical already exists or userDir is custom, prioritize canonical
    if (canonicalFile.existsSync()) {
      return canonicalFile;
    }

    // If running in development/local test where legacy file exists
    if (legacyFile.existsSync()) {
      return legacyFile;
    }

    // Otherwise, ensure directory exists and use canonical location
    try {
      if (!userDir.existsSync()) {
        userDir.createSync(recursive: true);
      }
      return canonicalFile;
    } catch (_) {
      // Fallback to local legacy file if user directory is inaccessible
      return legacyFile;
    }
  }
}
