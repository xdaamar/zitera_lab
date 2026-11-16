import 'package:flutter_test/flutter_test.dart';
import 'package:zitera_lab/core/auth/avatar_registry.dart';
import 'package:zitera_lab/core/auth/auth_service.dart';
import 'package:zitera_lab/core/scoreboard/scoreboard_service.dart';
import 'package:zitera_lab/core/turso/turso_client.dart';

void main() {
  group('Avatar Registry Contract & Fallback Tests', () {
    test('1. Registry contains exactly 14 avatars with non-empty attributes', () {
      expect(AvatarRegistry.all.length, 14);

      final expectedIds = [
        'zeta',
        'neko',
        'girl_cheerful',
        'girl_cool',
        'girl_smug',
        'girl_serious',
        'girl_pouty',
        'girl_fierce',
        'boy_cheerful',
        'boy_cool',
        'boy_smug',
        'boy_serious',
        'boy_pouty',
        'boy_fierce',
      ];

      for (final id in expectedIds) {
        expect(AvatarRegistry.isValidId(id), isTrue, reason: 'Missing id $id');
        final avatar = AvatarRegistry.getById(id);
        expect(avatar.id, id);
        expect(avatar.name.isNotEmpty, isTrue);
        expect(avatar.gender.isNotEmpty, isTrue);
        expect(avatar.expression.isNotEmpty, isTrue);
        expect(avatar.assetPath.isNotEmpty, isTrue);
      }
    });

    test('2. Defensive Fallback to Zeta for null, empty, or unknown IDs', () {
      expect(AvatarRegistry.getById(null).id, 'zeta');
      expect(AvatarRegistry.getById('').id, 'zeta');
      expect(AvatarRegistry.getById('   ').id, 'zeta');
      expect(AvatarRegistry.getById('non_existent_avatar_999').id, 'zeta');

      expect(
        AvatarRegistry.getAssetPath('unknown_id'),
        'assets/images/zeta_avatar.png',
      );
    });

    test('3. Case-insensitivity in Avatar lookup', () {
      expect(AvatarRegistry.getById('GIRL_CHEERFUL').id, 'girl_cheerful');
      expect(AvatarRegistry.getById('Boy_Cool').id, 'boy_cool');
    });
  });

  group('Auth & Scoreboard Avatar E2E Integration', () {
    final timestamp = DateTime.now().millisecondsSinceEpoch;
    final testUsername = 'avatar_user_$timestamp';
    final testPassword = 'AvatarPassword123!';
    const chosenAvatarId = 'girl_cool';
    String? createdUserId;

    test('4. UserModel / UserSession serialization contract', () {
      final user = UserModel(
        id: 'u_123',
        username: 'test_op',
        createdAt: 1000,
        avatarId: 'boy_serious',
      );

      final json = user.toJson();
      expect(json['avatar_id'], 'boy_serious');

      final fromJson = UserModel.fromJson(json);
      expect(fromJson.avatarId, 'boy_serious');

      // Compatibility fallback when json has no avatar_id
      final legacy = UserModel.fromJson({'id': 'u_456', 'username': 'legacy', 'created_at': 2000});
      expect(legacy.avatarId, 'zeta');

      // UserSession alias contract
      const isAliasValid = UserSession == UserModel;
      expect(isAliasValid, isTrue);
    });

    test('5. User Registration with Custom Avatar & Turso DB Persistence', () async {
      final regRes = await AuthService.register(
        testUsername,
        testPassword,
        avatarId: chosenAvatarId,
      );

      expect(regRes.success, isTrue, reason: regRes.message);
      expect(regRes.user, isNotNull);
      expect(regRes.user!.avatarId, chosenAvatarId);
      createdUserId = regRes.user!.id;

      // Verify in Turso users table
      final dbRes = await TursoClient.execute(
        'SELECT avatar_id FROM users WHERE id = ?',
        [createdUserId],
      );
      expect(dbRes.isNotEmpty, isTrue);
      expect(dbRes.firstRow!['avatar_id'], chosenAvatarId);
    });

    test('6. User Login restores chosen avatar into session', () async {
      await AuthService.logout();
      expect(AuthService.isLoggedIn, isFalse);

      final loginRes = await AuthService.login(testUsername, testPassword);
      expect(loginRes.success, isTrue);
      expect(loginRes.user?.avatarId, chosenAvatarId);
      expect(AuthService.currentUser?.avatarId, chosenAvatarId);
    });

    test('7. Update Avatar updates DB and currentUserNotifier', () async {
      expect(AuthService.isLoggedIn, isTrue);
      const newAvatarId = 'boy_fierce';

      final updateOk = await AuthService.updateAvatar(newAvatarId);
      expect(updateOk, isTrue);
      expect(AuthService.currentUser?.avatarId, newAvatarId);

      // Verify DB update
      final dbRes = await TursoClient.execute(
        'SELECT avatar_id FROM users WHERE id = ?',
        [createdUserId],
      );
      expect(dbRes.firstRow!['avatar_id'], newAvatarId);
    });

    test('8. Record Solve stores avatar_id in solves and reflects in Leaderboard', () async {
      expect(AuthService.isLoggedIn, isTrue);
      final currentAvatar = AuthService.currentUser!.avatarId;

      final solveOk = await ScoreboardService.recordSolve(
        labId: 'A03',
        points: 150,
      );
      expect(solveOk, isTrue);

      final leaderboard = await ScoreboardService.getLeaderboard();
      expect(leaderboard.isNotEmpty, isTrue);

      final myEntry = leaderboard.firstWhere(
        (e) => e.username == testUsername,
        orElse: () => throw Exception('Test user entry missing from leaderboard'),
      );

      expect(myEntry.avatarId, currentAvatar);
      expect(myEntry.labsSolved >= 1, isTrue);
    });

    tearDownAll(() async {
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
