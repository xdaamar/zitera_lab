import 'package:flutter/foundation.dart';
import '../auth/auth_service.dart';
import '../progress/progress_manager.dart';
import '../turso/turso_client.dart';

/// Single row entry in the live CTF Scoreboard.
class ScoreboardEntry {
  final int rank;
  final String userId;
  final String username;
  final int labsSolved;
  final int totalPoints;
  final DateTime? lastSolveAt;

  ScoreboardEntry({
    required this.rank,
    required this.userId,
    required this.username,
    required this.labsSolved,
    required this.totalPoints,
    this.lastSolveAt,
  });

  bool get isCurrentUser =>
      AuthService.currentUser != null && AuthService.currentUser!.id == userId;

  String get formattedLastSolve {
    if (lastSolveAt == null) return 'No solves yet';
    final diff = DateTime.now().difference(lastSolveAt!);
    if (diff.inSeconds < 60) return '${diff.inSeconds}s ago';
    if (diff.inMinutes < 60) return '${diff.inMinutes}m ago';
    if (diff.inHours < 24) return '${diff.inHours}h ago';
    return '${diff.inDays}d ago';
  }
}

/// Service managing remote solves and live leaderboard synchronization via Turso.
class ScoreboardService {
  /// Default points awarded per lab challenge solve.
  static const int defaultPointsPerLab = 100;

  /// Records a lab solve to Turso DB for the active authenticated user.
  /// Returns true if recorded or already recorded.
  static Future<bool> recordSolve({
    required String labId,
    int points = defaultPointsPerLab,
  }) async {
    final user = AuthService.currentUser;
    if (user == null) {
      debugPrint('[ScoreboardService] User not logged in, skipping remote solve.');
      return false;
    }

    final cleanLabId = ProgressManager.normalizeId(labId);
    final solveId = 's_${user.id}_$cleanLabId';
    final solvedAt = DateTime.now().millisecondsSinceEpoch;

    try {
      final res = await TursoClient.execute(
        'INSERT OR IGNORE INTO solves (id, user_id, username, lab_id, points, solved_at) '
        'VALUES (?, ?, ?, ?, ?, ?)',
        [solveId, user.id, user.username, cleanLabId, points, solvedAt],
      );

      debugPrint(
        '[ScoreboardService] Solve recorded for lab $cleanLabId by ${user.username} '
        '(affected: ${res.affectedRowCount})',
      );
      return true;
    } catch (e) {
      debugPrint('[ScoreboardService] Failed to record solve: $e');
      return false;
    }
  }

  /// Fetches the live CTF Scoreboard ranking from Turso DB.
  static Future<List<ScoreboardEntry>> getLeaderboard() async {
    const sql = '''
      SELECT 
        u.id AS user_id,
        u.username,
        COALESCE(COUNT(s.id), 0) AS labs_solved,
        COALESCE(SUM(s.points), 0) AS total_points,
        MAX(s.solved_at) AS last_solve_at
      FROM users u
      LEFT JOIN solves s ON u.id = s.user_id
      GROUP BY u.id, u.username
      ORDER BY total_points DESC, last_solve_at ASC, u.created_at ASC
    ''';

    final res = await TursoClient.execute(sql);
    final entries = <ScoreboardEntry>[];

    for (var i = 0; i < res.rows.length; i++) {
      final row = res.rows[i];
      final userId = row['user_id']?.toString() ?? '';
      final username = row['username']?.toString() ?? 'Anonymous';
      final labsSolved = (row['labs_solved'] as num?)?.toInt() ?? 0;
      final totalPoints = (row['total_points'] as num?)?.toInt() ?? 0;

      DateTime? lastSolve;
      final rawLastSolve = row['last_solve_at'];
      if (rawLastSolve != null) {
        final ms = (rawLastSolve as num?)?.toInt() ??
            int.tryParse(rawLastSolve.toString());
        if (ms != null && ms > 0) {
          lastSolve = DateTime.fromMillisecondsSinceEpoch(ms);
        }
      }

      entries.add(ScoreboardEntry(
        rank: i + 1,
        userId: userId,
        username: username,
        labsSolved: labsSolved,
        totalPoints: totalPoints,
        lastSolveAt: lastSolve,
      ));
    }

    return entries;
  }

  /// Synchronizes local completed challenges to Turso DB when user is logged in.
  static Future<int> syncLocalSolvesToRemote() async {
    final user = AuthService.currentUser;
    if (user == null) return 0;

    try {
      final progress = await ProgressManager.loadProgress();
      final completedChallenges =
          List<String>.from(progress['completed_challenges'] as List? ?? []);

      if (completedChallenges.isEmpty) return 0;

      var syncedCount = 0;
      for (final labId in completedChallenges) {
        final cleanLabId = ProgressManager.normalizeId(labId);
        final solveId = 's_${user.id}_$cleanLabId';
        final solvedAt = DateTime.now().millisecondsSinceEpoch;

        final res = await TursoClient.execute(
          'INSERT OR IGNORE INTO solves (id, user_id, username, lab_id, points, solved_at) '
          'VALUES (?, ?, ?, ?, ?, ?)',
          [solveId, user.id, user.username, cleanLabId, defaultPointsPerLab, solvedAt],
        );

        if (res.affectedRowCount > 0) {
          syncedCount++;
        }
      }

      if (syncedCount > 0) {
        debugPrint('[ScoreboardService] Synced $syncedCount local solves to Turso DB.');
      }
      return syncedCount;
    } catch (e) {
      debugPrint('[ScoreboardService] Error syncing local solves: $e');
      return 0;
    }
  }
}
