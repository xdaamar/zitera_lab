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
  final String installGuide;

  ToolItem({
    required this.id,
    required this.name,
    required this.category,
    required this.installed,
    this.version,
    required this.minVersion,
    required this.status,
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

  LabItem({
    required this.id,
    required this.title,
    required this.installed,
    required this.running,
    required this.port,
    this.url,
    required this.version,
    required this.status,
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
    );
  }
}

class CatalogEntry {
  final String id;
  final String title;
  final String repository;
  final String version;
  final String difficulty;
  final String owasp;
  final String description;

  CatalogEntry({
    required this.id,
    required this.title,
    required this.repository,
    required this.version,
    required this.difficulty,
    required this.owasp,
    required this.description,
  });

  factory CatalogEntry.fromJson(Map<String, dynamic> json) {
    return CatalogEntry(
      id: json['id'] as String? ?? '',
      title: json['title'] as String? ?? '',
      repository: json['repository'] as String? ?? '',
      version: json['version'] as String? ?? '1.0.0',
      difficulty: json['difficulty'] as String? ?? 'Beginner',
      owasp: json['owasp'] as String? ?? '',
      description: json['description'] as String? ?? '',
    );
  }
}

class LabManifest {
  final int schemaVersion;
  final String id;
  final String slug;
  final String title;
  final String owasp;
  final String version;
  final String difficulty;
  final String runtime;
  final String entrypoint;
  final int defaultPort;
  final int estimatedMinutes;
  final List<String> modes;

  LabManifest({
    required this.schemaVersion,
    required this.id,
    required this.slug,
    required this.title,
    required this.owasp,
    required this.version,
    required this.difficulty,
    required this.runtime,
    required this.entrypoint,
    required this.defaultPort,
    required this.estimatedMinutes,
    required this.modes,
  });

  factory LabManifest.fromJson(Map<String, dynamic> json) {
    return LabManifest(
      schemaVersion: json['schema_version'] as int? ?? 1,
      id: json['id'] as String? ?? '',
      slug: json['slug'] as String? ?? '',
      title: json['title'] as String? ?? '',
      owasp: json['owasp'] as String? ?? '',
      version: json['version'] as String? ?? '1.0.0',
      difficulty: json['difficulty'] as String? ?? 'Beginner',
      runtime: json['runtime'] as String? ?? 'docker',
      entrypoint: json['entrypoint'] as String? ?? '',
      defaultPort: json['default_port'] as int? ?? 0,
      estimatedMinutes: json['estimated_minutes'] as int? ?? 45,
      modes: (json['modes'] as List<dynamic>?)?.map((e) => e.toString()).toList() ?? [],
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
