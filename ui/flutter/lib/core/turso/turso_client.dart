import 'dart:convert';
import 'dart:io';

/// Result of an individual Turso query execution.
class TursoQueryResult {
  final int affectedRowCount;
  final List<String> columns;
  final List<Map<String, dynamic>> rows;

  TursoQueryResult({
    required this.affectedRowCount,
    required this.columns,
    required this.rows,
  });

  bool get isEmpty => rows.isEmpty;
  bool get isNotEmpty => rows.isNotEmpty;
  Map<String, dynamic>? get firstRow => rows.isNotEmpty ? rows.first : null;
}

/// A prepared statement with SQL and parameter bindings.
class TursoStatement {
  final String sql;
  final List<dynamic>? args;

  const TursoStatement(this.sql, [this.args]);
}

/// Official Turso HTTP Pipeline (v2) Client for Zitera Lab.
/// Manages high-performance remote database operations without native SQLite driver overhead.
class TursoClient {
  static const String _defaultUrl =
      'https://ziteralab-xdaamar.aws-ap-northeast-1.turso.io/v2/pipeline';
  static const String _defaultToken =
      'eyJhbGciOiJFZERTQSIsInR5cCI6IkpXVCJ9.eyJhIjoicnciLCJpYXQiOjE3OTQ4MzAxMTMsImlkIjoiMDFhMWU0MzUtYmMwMS03OTk0LWFmZjctMDhhYmM5ODZhY2MyIiwia2lkIjoiUGlqYXZ5dzQwbVB0cTVUdnQyMzd6Q29ZRHpqQ0JTa3p3WVdVdGtGTnhqayIsInJpZCI6IjFmZmIyZjFhLTM0MmEtNDAwMS1hNTZmLTdiYmM1OGJlMzExZSJ9.Y-tIm1M0tlUAEzPIJXC7w8PRsh016Y8sSpbE24EoBzjJX9z9p1AJXK2rYTFYgILBvZvzGRmNy_0n2JqVxv6OCw';

  static Uri get _url {
    final env = Platform.environment['TURSO_DATABASE_URL'];
    return Uri.parse(env != null && env.trim().isNotEmpty ? env.trim() : _defaultUrl);
  }

  static String get _token {
    final env = Platform.environment['TURSO_AUTH_TOKEN'];
    return env != null && env.trim().isNotEmpty ? env.trim() : _defaultToken;
  }

  /// Encodes Dart value to Turso's typed parameter object.
  static Map<String, dynamic> _encodeArg(dynamic val) {
    if (val == null) {
      return {'type': 'null'};
    } else if (val is int || val is BigInt) {
      return {'type': 'integer', 'value': val.toString()};
    } else if (val is double) {
      return {'type': 'float', 'value': val.toString()};
    } else if (val is bool) {
      return {'type': 'integer', 'value': val ? '1' : '0'};
    } else {
      return {'type': 'text', 'value': val.toString()};
    }
  }

  /// Parses raw Turso value cell into appropriate Dart primitive.
  static dynamic _decodeValue(Map<String, dynamic>? cell) {
    if (cell == null) return null;
    final type = cell['type'] as String?;
    final val = cell['value'];
    if (type == 'null' || val == null) return null;

    if (type == 'integer') {
      if (val is int) return val;
      return int.tryParse(val.toString()) ?? val;
    } else if (type == 'float') {
      if (val is double) return val;
      return double.tryParse(val.toString()) ?? val;
    }
    return val.toString();
  }

  /// Executes a single SQL query against Turso v2 pipeline.
  static Future<TursoQueryResult> execute(
    String sql, [
    List<dynamic>? args,
  ]) async {
    final results = await batch([TursoStatement(sql, args)]);
    return results.first;
  }

  /// Executes a batch of statements in a single pipeline roundtrip.
  static Future<List<TursoQueryResult>> batch(
    List<TursoStatement> statements,
  ) async {
    final client = HttpClient();
    client.connectionTimeout = const Duration(seconds: 10);

    try {
      final req = await client.postUrl(_url);
      req.headers.set('Authorization', 'Bearer $_token');
      req.headers.set('Content-Type', 'application/json');

      final requests = <Map<String, dynamic>>[];
      for (final stmt in statements) {
        final stmtPayload = <String, dynamic>{'sql': stmt.sql};
        if (stmt.args != null && stmt.args!.isNotEmpty) {
          stmtPayload['args'] = stmt.args!.map(_encodeArg).toList();
        }
        requests.add({
          'type': 'execute',
          'stmt': stmtPayload,
        });
      }
      requests.add({'type': 'close'});

      final body = jsonEncode({'requests': requests});
      req.add(utf8.encode(body));

      final resp = await req.close().timeout(const Duration(seconds: 15));
      final respText = await resp.transform(utf8.decoder).join();

      if (resp.statusCode != 200) {
        throw HttpException(
          'Turso HTTP error ${resp.statusCode}: $respText',
          uri: _url,
        );
      }

      final decoded = jsonDecode(respText) as Map<String, dynamic>;
      final rawResults = decoded['results'] as List? ?? [];

      final queryResults = <TursoQueryResult>[];
      for (var i = 0; i < statements.length; i++) {
        if (i >= rawResults.length) {
          throw Exception('Turso returned fewer results than statements executed.');
        }

        final resItem = rawResults[i] as Map<String, dynamic>;
        final resType = resItem['type'] as String?;

        if (resType == 'error') {
          final err = resItem['error'] as Map<String, dynamic>?;
          final msg = err?['message'] ?? 'Unknown Turso error';
          throw Exception('Turso SQL Error: $msg');
        }

        final responseObj = resItem['response'] as Map<String, dynamic>? ?? {};
        final resultObj = responseObj['result'] as Map<String, dynamic>? ?? {};

        final colsList = resultObj['cols'] as List? ?? [];
        final colNames = colsList
            .map((c) => (c as Map<String, dynamic>)['name']?.toString() ?? '')
            .toList();

        final affectedCount = (resultObj['affected_row_count'] as num?)?.toInt() ?? 0;
        final rawRows = resultObj['rows'] as List? ?? [];

        final rows = <Map<String, dynamic>>[];
        for (final r in rawRows) {
          final cells = r as List? ?? [];
          final rowMap = <String, dynamic>{};
          for (var j = 0; j < colNames.length; j++) {
            if (j < cells.length) {
              final cell = cells[j] as Map<String, dynamic>?;
              rowMap[colNames[j]] = _decodeValue(cell);
            } else {
              rowMap[colNames[j]] = null;
            }
          }
          rows.add(rowMap);
        }

        queryResults.add(TursoQueryResult(
          affectedRowCount: affectedCount,
          columns: colNames,
          rows: rows,
        ));
      }

      return queryResults;
    } finally {
      client.close();
    }
  }
}
