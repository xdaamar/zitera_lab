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
