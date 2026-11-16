import 'package:flutter_test/flutter_test.dart';
import 'package:zitera_lab/core/auth/auth_service.dart';
import 'package:zitera_lab/core/scoreboard/scoreboard_service.dart';
import 'package:zitera_lab/core/turso/turso_client.dart';

void main() {
  group('Turso Auth & Scoreboard E2E Test Suite', () {
    final testUsername = 'test_runner_${DateTime.now().millisecondsSinceEpoch}';
    final testPassword = 'SecurePassword123!';
    String? createdUserId;

    test('1. Database Connection & Schema Verification', () async {
      final res = await TursoClient.execute(
        "SELECT name FROM sqlite_master WHERE type='table' AND name IN ('users', 'solves')",
      );
      expect(res.rows.length, 2);
      final tableNames = res.rows.map((r) => r['name']).toSet();
      expect(tableNames.contains('users'), isTrue);
      expect(tableNames.contains('solves'), isTrue);
    });

    test('2. Password Hashing & Verification', () {
      final hash = AuthService.hashPassword(testPassword);
      expect(hash.contains(':'), isTrue);
      expect(AuthService.verifyPassword(testPassword, hash), isTrue);
      expect(AuthService.verifyPassword('WrongPassword', hash), isFalse);
    });

    test('3. User Registration via AuthService', () async {
      final regRes = await AuthService.register(testUsername, testPassword);
      expect(regRes.success, isTrue, reason: regRes.message);
      expect(regRes.user, isNotNull);
      expect(regRes.user!.username, testUsername);
      expect(regRes.user!.id.isNotEmpty, isTrue);

      createdUserId = regRes.user!.id;
      expect(AuthService.isLoggedIn, isTrue);
      expect(AuthService.currentUser?.username, testUsername);

      // Verify duplicate registration rejection
      final dupRes = await AuthService.register(testUsername, 'AnotherPassword');
      expect(dupRes.success, isFalse);
      expect(dupRes.message.contains('already registered'), isTrue);
    });

    test('4. User Login & Session Persistence', () async {
      // Logout first
      await AuthService.logout();
      expect(AuthService.isLoggedIn, isFalse);

      // Invalid password attempt
      final badLogin = await AuthService.login(testUsername, 'WrongPass');
      expect(badLogin.success, isFalse);
      expect(badLogin.message, 'Incorrect password.');

      // Valid login
      final goodLogin = await AuthService.login(testUsername, testPassword);
      expect(goodLogin.success, isTrue);
      expect(goodLogin.user?.username, testUsername);
      expect(AuthService.isLoggedIn, isTrue);
    });

    test('5. Record Solve to Turso & Idempotency', () async {
      expect(AuthService.isLoggedIn, isTrue);

      // Record first solve (A01)
      final solve1 = await ScoreboardService.recordSolve(
        labId: 'A01',
        points: 100,
      );
      expect(solve1, isTrue);

      // Record duplicate solve (A01) - should be idempotent
      final solve1Dup = await ScoreboardService.recordSolve(
        labId: 'A01',
        points: 100,
      );
      expect(solve1Dup, isTrue);

      // Record second solve (A02)
      final solve2 = await ScoreboardService.recordSolve(
        labId: 'A02',
        points: 100,
      );
      expect(solve2, isTrue);
    });

    test('6. Fetch Scoreboard Leaderboard & Validate Ranks', () async {
      final leaderboard = await ScoreboardService.getLeaderboard();
      expect(leaderboard.isNotEmpty, isTrue);

      final myEntry = leaderboard.firstWhere(
        (e) => e.username == testUsername,
        orElse: () => throw Exception('Test user entry missing from scoreboard'),
      );

      expect(myEntry.labsSolved, 2);
      expect(myEntry.totalPoints, 200);
      expect(myEntry.lastSolveAt, isNotNull);
      expect(myEntry.isCurrentUser, isTrue);
    });

    tearDownAll(() async {
      // Clean up test user & solves from Turso DB
      if (createdUserId != null) {
        try {
          await TursoClient.batch([
            TursoStatement("DELETE FROM solves WHERE user_id = ?", [createdUserId]),
            TursoStatement("DELETE FROM users WHERE id = ?", [createdUserId]),
          ]);
        } catch (_) {}
      }
      await AuthService.logout();
    });
  });
}
