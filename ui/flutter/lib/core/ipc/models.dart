class ComponentItem {
  final String name;
  final bool installed;
  final String? version;
  final String status;
  final String message;
  final String? recommendation;

  ComponentItem({
    required this.name,
    required this.installed,
    this.version,
    required this.status,
    required this.message,
    this.recommendation,
  });

  factory ComponentItem.fromJson(Map<String, dynamic> json) {
    return ComponentItem(
      name: json['name'] as String? ?? 'Unknown',
      installed: json['installed'] as bool? ?? false,
      version: json['version'] as String?,
      status: json['status'] as String? ?? 'MISSING',
      message: json['message'] as String? ?? '',
      recommendation: json['recommendation'] as String?,
    );
  }
}

class DiagnosticsResult {
  final ComponentItem os;
  final ComponentItem git;
  final ComponentItem wsl;
  final ComponentItem docker;
  final ComponentItem dockerDaemon;
  final ComponentItem powershell;
  final double memoryGb;
  final double diskFreeGb;
  final bool allReady;

  DiagnosticsResult({
    required this.os,
    required this.git,
    required this.wsl,
    required this.docker,
    required this.dockerDaemon,
    required this.powershell,
    required this.memoryGb,
    required this.diskFreeGb,
    required this.allReady,
  });

  factory DiagnosticsResult.fromJson(Map<String, dynamic> json) {
    return DiagnosticsResult(
      os: ComponentItem.fromJson(json['os'] as Map<String, dynamic>? ?? {}),
      git: ComponentItem.fromJson(json['git'] as Map<String, dynamic>? ?? {}),
      wsl: ComponentItem.fromJson(json['wsl'] as Map<String, dynamic>? ?? {}),
      docker: ComponentItem.fromJson(json['docker'] as Map<String, dynamic>? ?? {}),
      dockerDaemon: ComponentItem.fromJson(json['docker_daemon'] as Map<String, dynamic>? ?? {}),
      powershell: ComponentItem.fromJson(json['powershell'] as Map<String, dynamic>? ?? {}),
      memoryGb: (json['memory_gb'] as num?)?.toDouble() ?? 16.0,
      diskFreeGb: (json['disk_free_gb'] as num?)?.toDouble() ?? 50.0,
      allReady: json['all_ready'] as bool? ?? false,
    );
  }
}

class ToolItem {
  final String id;
  final String name;
  final String category;
  final bool installed;
  final String? version;
  final String minVersion;
  final String status;
  final String? path;
  final List<String> capabilities;
  final String installMethod;
  final String installGuide;

  ToolItem({
    required this.id,
    required this.name,
    required this.category,
    required this.installed,
    this.version,
    required this.minVersion,
    required this.status,
    this.path,
    this.capabilities = const [],
    this.installMethod = 'manual',
    required this.installGuide,
  });

  factory ToolItem.fromJson(Map<String, dynamic> json) {
    return ToolItem(
      id: json['id'] as String? ?? '',
      name: json['name'] as String? ?? '',
      category: json['category'] as String? ?? '',
      installed: json['installed'] as bool? ?? false,
      version: json['version'] as String?,
      minVersion: json['min_version'] as String? ?? '',
      status: json['status'] as String? ?? 'MISSING',
      path: json['path'] as String?,
      capabilities: (json['capabilities'] as List<dynamic>?)?.map((e) => e.toString()).toList() ?? [],
      installMethod: json['install_method'] as String? ?? 'manual',
      installGuide: json['install_guide'] as String? ?? '',
    );
  }
}

class LabItem {
  final String id;
  final String title;
  final bool installed;
  final bool running;
  final int port;
  final String? url;
  final String version;
  final String status;
  final String learnReadiness;
  final String practiceReadiness;
  final String challengeReadiness;
  final List<String> recommendedTools;

  LabItem({
    required this.id,
    required this.title,
    required this.installed,
    required this.running,
    required this.port,
    this.url,
    required this.version,
    required this.status,
    this.learnReadiness = 'READY',
    this.practiceReadiness = 'READY',
    this.challengeReadiness = 'READY',
    this.recommendedTools = const [],
  });

  factory LabItem.fromJson(Map<String, dynamic> json) {
    return LabItem(
      id: json['id'] as String? ?? '',
      title: json['title'] as String? ?? '',
      installed: json['installed'] as bool? ?? false,
      running: json['running'] as bool? ?? false,
      port: json['port'] as int? ?? 0,
      url: json['url'] as String?,
      version: json['version'] as String? ?? '1.0.0',
      status: json['status'] as String? ?? 'NOT_INSTALLED',
      learnReadiness: json['learn_readiness'] as String? ?? 'READY',
      practiceReadiness: json['practice_readiness'] as String? ?? 'READY',
      challengeReadiness: json['challenge_readiness'] as String? ?? 'READY',
      recommendedTools: (json['recommended_tools'] as List<dynamic>?)?.map((e) => e.toString()).toList() ?? [],
    );
  }
}

class CatalogEntry {
  final String id;
  final String packageId;
  final String title;
  final String repository;
  final String version;
  final String difficulty;
  final String owasp;
  final String standard;
  final String standardVersion;
  final String categoryId;
  final String categoryName;
  final String description;
  final String shortDescription;
  final int securityVersion;
  final String minimumCoreVersion;
  final int estimatedTime;
  final List<String> skills;
  final List<String> learningObjectives;
  final List<String> prerequisites;

  CatalogEntry({
    required this.id,
    required this.packageId,
    required this.title,
    required this.repository,
    required this.version,
    required this.difficulty,
    required this.owasp,
    this.standard = 'owasp-top10',
    this.standardVersion = '2025',
    required this.categoryId,
    required this.categoryName,
    required this.description,
    required this.shortDescription,
    this.securityVersion = 1,
    this.minimumCoreVersion = '2.0.0',
    this.estimatedTime = 45,
    this.skills = const [],
    this.learningObjectives = const [],
    this.prerequisites = const [],
  });

  factory CatalogEntry.fromJson(Map<String, dynamic> json) {
    final rawId = json['id'] as String? ?? '';
    final rawTitle = json['title'] as String? ?? '';
    final rawDesc = json['description'] as String? ?? '';
    return CatalogEntry(
      id: rawId,
      packageId: json['package_id'] as String? ?? 'zitera-lab-${rawId.toLowerCase()}',
      title: rawTitle,
      repository: json['repository'] as String? ?? '',
      version: json['version'] as String? ?? '1.0.0',
      difficulty: json['difficulty'] as String? ?? 'Beginner',
      owasp: json['owasp'] as String? ?? '',
      standard: json['standard'] as String? ?? 'owasp-top10',
      standardVersion: json['standard_version'] as String? ?? '2025',
      categoryId: json['category_id'] as String? ?? rawId,
      categoryName: json['category_name'] as String? ?? rawTitle,
      description: rawDesc,
      shortDescription: json['short_description'] as String? ?? rawDesc,
      securityVersion: (json['security_version'] as num?)?.toInt() ?? 1,
      minimumCoreVersion: json['minimum_core_version'] as String? ?? '2.0.0',
      estimatedTime: (json['estimated_time'] as num?)?.toInt() ?? 45,
      skills: (json['skills'] as List<dynamic>?)?.map((e) => e.toString()).toList() ?? [],
      learningObjectives: (json['learning_objectives'] as List<dynamic>?)?.map((e) => e.toString()).toList() ?? [],
      prerequisites: (json['prerequisites'] as List<dynamic>?)?.map((e) => e.toString()).toList() ?? [],
    );
  }
}

class LabManifest {
  final int schemaVersion;
  final String id;
  final String packageId;
  final String slug;
  final String title;
  final String owasp;
  final String standard;
  final String standardVersion;
  final String categoryId;
  final String categoryName;
  final String version;
  final int securityVersion;
  final String minimumCoreVersion;
  final String difficulty;
  final String runtime;
  final String entrypoint;
  final int defaultPort;
  final int estimatedMinutes;
  final List<String> modes;
  final String shortDescription;
  final List<String> skills;
  final List<String> learningObjectives;
  final List<String> prerequisites;

  LabManifest({
    required this.schemaVersion,
    required this.id,
    required this.packageId,
    required this.slug,
    required this.title,
    required this.owasp,
    this.standard = 'owasp-top10',
    this.standardVersion = '2025',
    required this.categoryId,
    required this.categoryName,
    required this.version,
    this.securityVersion = 1,
    this.minimumCoreVersion = '2.0.0',
    required this.difficulty,
    required this.runtime,
    required this.entrypoint,
    required this.defaultPort,
    required this.estimatedMinutes,
    required this.modes,
    this.shortDescription = '',
    this.skills = const [],
    this.learningObjectives = const [],
    this.prerequisites = const [],
  });

  factory LabManifest.fromJson(Map<String, dynamic> json) {
    final rawId = json['id'] as String? ?? '';
    final rawTitle = json['title'] as String? ?? '';
    return LabManifest(
      schemaVersion: json['schema_version'] as int? ?? 1,
      id: rawId,
      packageId: json['package_id'] as String? ?? 'zitera-lab-${rawId.toLowerCase()}',
      slug: json['slug'] as String? ?? '',
      title: rawTitle,
      owasp: json['owasp'] as String? ?? '',
      standard: json['standard'] as String? ?? 'owasp-top10',
      standardVersion: json['standard_version'] as String? ?? '2025',
      categoryId: json['category_id'] as String? ?? rawId,
      categoryName: json['category_name'] as String? ?? rawTitle,
      version: json['version'] as String? ?? '1.0.0',
      securityVersion: (json['security_version'] as num?)?.toInt() ?? 1,
      minimumCoreVersion: json['minimum_core_version'] as String? ?? '2.0.0',
      difficulty: json['difficulty'] as String? ?? 'Beginner',
      runtime: json['runtime'] as String? ?? 'native_sandboxed',
      entrypoint: json['entrypoint'] as String? ?? '',
      defaultPort: json['default_port'] as int? ?? 0,
      estimatedMinutes: json['estimated_minutes'] as int? ?? 45,
      modes: (json['modes'] as List<dynamic>?)?.map((e) => e.toString()).toList() ?? [],
      shortDescription: json['short_description'] as String? ?? '',
      skills: (json['skills'] as List<dynamic>?)?.map((e) => e.toString()).toList() ?? [],
      learningObjectives: (json['learning_objectives'] as List<dynamic>?)?.map((e) => e.toString()).toList() ?? [],
      prerequisites: (json['prerequisites'] as List<dynamic>?)?.map((e) => e.toString()).toList() ?? [],
    );
  }
}

class ProgressiveHint {
  final int tier;
  final String type;
  final String hint;

  ProgressiveHint({
    required this.tier,
    required this.type,
    required this.hint,
  });

  factory ProgressiveHint.fromJson(Map<String, dynamic> json) {
    return ProgressiveHint(
      tier: json['tier'] as int? ?? 1,
      type: json['type'] as String? ?? 'general',
      hint: json['hint'] as String? ?? '',
    );
  }
}

class LabContent {
  final LabManifest manifest;
  final Map<String, String> lessons;
  final String challengeObjective;
  final List<ProgressiveHint> hints;

  LabContent({
    required this.manifest,
    required this.lessons,
    required this.challengeObjective,
    required this.hints,
  });

  factory LabContent.fromJson(Map<String, dynamic> json) {
    final rawLessons = json['lessons'] as Map<String, dynamic>? ?? {};
    final parsedLessons = rawLessons.map((k, v) => MapEntry(k, v.toString()));

    final rawHints = json['hints'] as List<dynamic>? ?? [];
    final parsedHints = rawHints
        .map((h) => ProgressiveHint.fromJson(h as Map<String, dynamic>))
        .toList();

    return LabContent(
      manifest: LabManifest.fromJson(json['manifest'] as Map<String, dynamic>? ?? {}),
      lessons: parsedLessons,
      challengeObjective: json['challenge_objective'] as String? ?? '',
      hints: parsedHints,
    );
  }
}

class ChallengeResult {
  final String labId;
  final String status;
  final String message;

  ChallengeResult({
    required this.labId,
    required this.status,
    required this.message,
  });

  factory ChallengeResult.fromJson(Map<String, dynamic> json) {
    return ChallengeResult(
      labId: json['lab_id'] as String? ?? '',
      status: json['status'] as String? ?? 'failed',
      message: json['message'] as String? ?? '',
    );
  }
}

class PracticeVerificationResult {
  final String labId;
  final String status; // passed, failed, unavailable, error
  final String message;

  PracticeVerificationResult({
    required this.labId,
    required this.status,
    required this.message,
  });

  factory PracticeVerificationResult.fromJson(Map<String, dynamic> json) {
    return PracticeVerificationResult(
      labId: json['lab_id'] as String? ?? '',
      status: json['status'] as String? ?? 'unavailable',
      message: json['message'] as String? ?? '',
    );
  }
}

class ToolInstallResult {
  final String id;
  final bool success;
  final String message;
  final String method;

  ToolInstallResult({
    required this.id,
    required this.success,
    required this.message,
    required this.method,
  });

  factory ToolInstallResult.fromJson(Map<String, dynamic> json) {
    return ToolInstallResult(
      id: json['id'] as String? ?? '',
      success: json['success'] as bool? ?? false,
      message: json['message'] as String? ?? '',
      method: json['method'] as String? ?? 'manual',
    );
  }
}

class TerminalResult {
  final String stdout;
  final String stderr;
  final int exitCode;
  final String cwd;

  TerminalResult({
    required this.stdout,
    required this.stderr,
    required this.exitCode,
    required this.cwd,
  });

  factory TerminalResult.fromJson(Map<String, dynamic> json) {
    return TerminalResult(
      stdout: json['stdout'] as String? ?? '',
      stderr: json['stderr'] as String? ?? '',
      exitCode: json['exit_code'] as int? ?? 0,
      cwd: json['cwd'] as String? ?? '/',
    );
  }
}

class ValidationCheckItem {
  final String name;
  final bool passed;
  final String message;
  final String? remediation;

  ValidationCheckItem({
    required this.name,
    required this.passed,
    required this.message,
    this.remediation,
  });

  factory ValidationCheckItem.fromJson(Map<String, dynamic> json) {
    return ValidationCheckItem(
      name: json['name'] as String? ?? '',
      passed: json['passed'] as bool? ?? false,
      message: json['message'] as String? ?? '',
      remediation: json['remediation'] as String?,
    );
  }
}

class LabValidationReport {
  final String labId;
  final bool valid;
  final List<ValidationCheckItem> checks;
  final String summary;

  LabValidationReport({
    required this.labId,
    required this.valid,
    required this.checks,
    required this.summary,
  });

  factory LabValidationReport.fromJson(Map<String, dynamic> json) {
    final rawChecks = json['checks'] as List<dynamic>? ?? [];
    return LabValidationReport(
      labId: json['lab_id'] as String? ?? '',
      valid: json['valid'] as bool? ?? false,
      checks: rawChecks
          .map((c) => ValidationCheckItem.fromJson(c as Map<String, dynamic>))
          .toList(),
      summary: json['summary'] as String? ?? '',
    );
  }
}

enum FailureCategory {
  labCrash,
  runtimeUnavailable,
  packageVerificationFailure,
  updateInterrupted,
  rollbackOccurred,
  storageFailure,
  brokerUnavailable,
  general,
}

class ZiteraException implements Exception {
  final String code;
  final String message;
  final String? recoveryAction;
  final bool recoverable;
  final String? details;

  ZiteraException({
    required this.code,
    required this.message,
    this.recoveryAction,
    this.recoverable = true,
    this.details,
  });

  FailureCategory get category {
    final c = code.toUpperCase();
    final m = message.toLowerCase();
    if (c.contains('CRASH') || m.contains('crash') || m.contains('timed out waiting') || c == 'LAB_START_FAILED') {
      return FailureCategory.labCrash;
    }
    if (c.contains('ROLLBACK') || m.contains('rollback') || m.contains('rolled back')) {
      return FailureCategory.rollbackOccurred;
    }
    if (c.contains('INTERRUPTED') || m.contains('interrupted') || m.contains('staging')) {
      return FailureCategory.updateInterrupted;
    }
    if (c.contains('RUNTIME') || m.contains('runtime') || m.contains('appcontainer') || m.contains('executable not found')) {
      return FailureCategory.runtimeUnavailable;
    }
    if (c.contains('VERIFICATION') || c.contains('SIGNATURE') || c.contains('DOWNGRADE') || m.contains('verification') || m.contains('signature') || m.contains('checksum')) {
      return FailureCategory.packageVerificationFailure;
    }
    if (c.contains('STORAGE') || m.contains('storage') || m.contains('disk') || m.contains('permission denied')) {
      return FailureCategory.storageFailure;
    }
    if (c.contains('BROKER') || m.contains('broker') || m.contains('port') || m.contains('address already in use')) {
      return FailureCategory.brokerUnavailable;
    }
    return FailureCategory.general;
  }

  @override
  String toString() {
    if (recoveryAction != null && recoveryAction!.isNotEmpty) {
      return '$message\n\nRemediation: $recoveryAction';
    }
    return message;
  }
}
