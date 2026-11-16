import 'dart:convert';
import 'dart:io';
import 'dart:math';
import 'package:crypto/crypto.dart';
import 'package:flutter/foundation.dart';
import '../storage/storage_paths.dart';
import '../turso/turso_client.dart';

/// Represents a Zitera Lab authenticated user.
class UserModel {
  final String id;
  final String username;
  final int createdAt;

  UserModel({
    required this.id,
    required this.username,
    required this.createdAt,
  });

  Map<String, dynamic> toJson() => {
        'id': id,
        'username': username,
        'created_at': createdAt,
      };

  factory UserModel.fromJson(Map<String, dynamic> json) {
    return UserModel(
      id: json['id'] as String? ?? '',
      username: json['username'] as String? ?? '',
      createdAt: (json['created_at'] as num?)?.toInt() ?? 0,
    );
  }
}

/// Authentication result container.
class AuthResult {
  final bool success;
  final String message;
  final UserModel? user;

  const AuthResult({
    required this.success,
    required this.message,
    this.user,
  });

  factory AuthResult.ok(String message, UserModel user) =>
      AuthResult(success: true, message: message, user: user);

  factory AuthResult.fail(String message) =>
      AuthResult(success: false, message: message);
}

/// Authentication & User Session Service powered by Turso Cloud DB.
class AuthService {
  static final ValueNotifier<UserModel?> currentUserNotifier =
      ValueNotifier<UserModel?>(null);

  static UserModel? get currentUser => currentUserNotifier.value;
  static bool get isLoggedIn => currentUserNotifier.value != null;

  static const String _sessionFileName = 'auth_session.json';

  static File _getSessionFile() {
    final userDir = StoragePaths.getUserDataDir();
    if (!userDir.existsSync()) {
      try {
        userDir.createSync(recursive: true);
      } catch (_) {}
    }
    return File('${userDir.path}${Platform.pathSeparator}$_sessionFileName');
  }

  /// Initializes session from local storage on app launch.
  static Future<void> init() async {
    try {
      final file = _getSessionFile();
      if (file.existsSync()) {
        final content = await file.readAsString();
        if (content.trim().isNotEmpty) {
          final data = jsonDecode(content) as Map<String, dynamic>;
          final user = UserModel.fromJson(data);
          if (user.id.isNotEmpty && user.username.isNotEmpty) {
            currentUserNotifier.value = user;
          }
        }
      }
    } catch (e) {
      debugPrint('[AuthService] Error loading session: $e');
    }
  }

  /// Hashes a password with salt using SHA-256.
  static String hashPassword(String password, [String? existingSalt]) {
    final salt = existingSalt ?? _generateSalt();
    final bytes = utf8.encode('$salt:$password');
    final digest = sha256.convert(bytes);
    return '$salt:${digest.toString()}';
  }

  /// Verifies password against stored salt:hash string.
  static bool verifyPassword(String password, String storedHash) {
    final parts = storedHash.split(':');
    if (parts.length != 2) return false;
    final salt = parts[0];
    final expectedHash = parts[1];

    final bytes = utf8.encode('$salt:$password');
    final actualHash = sha256.convert(bytes).toString();

    return actualHash == expectedHash;
  }

  static String _generateSalt([int length = 16]) {
    final rnd = Random.secure();
    final values = List<int>.generate(length, (_) => rnd.nextInt(256));
    return values.map((b) => b.toRadixString(16).padLeft(2, '0')).join();
  }

  static String _generateId() {
    final rnd = Random.secure();
    final hexPart = List<int>.generate(8, (_) => rnd.nextInt(256))
        .map((b) => b.toRadixString(16).padLeft(2, '0'))
        .join();
    return 'u_${DateTime.now().millisecondsSinceEpoch}_$hexPart';
  }

  /// Registers a new user in Turso DB and signs in immediately.
  static Future<AuthResult> register(String username, String password) async {
    final cleanUsername = username.trim();
    if (cleanUsername.length < 3) {
      return AuthResult.fail('Username must be at least 3 characters long.');
    }
    if (!RegExp(r'^[a-zA-Z0-9_]+$').hasMatch(cleanUsername)) {
      return AuthResult.fail('Username can only contain letters, numbers, and underscores.');
    }
    if (password.length < 4) {
      return AuthResult.fail('Password must be at least 4 characters long.');
    }

    try {
      // 1. Check if username is taken
      final checkRes = await TursoClient.execute(
        'SELECT id FROM users WHERE username = ?',
        [cleanUsername],
      );
      if (checkRes.isNotEmpty) {
        return AuthResult.fail('Username "$cleanUsername" is already registered.');
      }

      // 2. Hash password & Insert user
      final userId = _generateId();
      final passHash = hashPassword(password);
      final createdAt = DateTime.now().millisecondsSinceEpoch;

      await TursoClient.execute(
        'INSERT INTO users (id, username, password_hash, created_at) VALUES (?, ?, ?, ?)',
        [userId, cleanUsername, passHash, createdAt],
      );

      final user = UserModel(
        id: userId,
        username: cleanUsername,
        createdAt: createdAt,
      );

      await _persistSession(user);
      currentUserNotifier.value = user;

      return AuthResult.ok('Account registered successfully!', user);
    } catch (e) {
      return AuthResult.fail('Registration failed: $e');
    }
  }

  /// Signs in an existing user with username & password.
  static Future<AuthResult> login(String username, String password) async {
    final cleanUsername = username.trim();
    if (cleanUsername.isEmpty || password.isEmpty) {
      return AuthResult.fail('Username and password are required.');
    }

    try {
      final res = await TursoClient.execute(
        'SELECT id, username, password_hash, created_at FROM users WHERE username = ?',
        [cleanUsername],
      );

      if (res.isEmpty) {
        return AuthResult.fail('User "$cleanUsername" not found.');
      }

      final row = res.firstRow!;
      final storedHash = row['password_hash']?.toString() ?? '';
      if (!verifyPassword(password, storedHash)) {
        return AuthResult.fail('Incorrect password.');
      }

      final user = UserModel(
        id: row['id']?.toString() ?? '',
        username: row['username']?.toString() ?? cleanUsername,
        createdAt: (row['created_at'] as num?)?.toInt() ?? 0,
      );

      await _persistSession(user);
      currentUserNotifier.value = user;

      return AuthResult.ok('Logged in as $cleanUsername', user);
    } catch (e) {
      return AuthResult.fail('Login failed: $e');
    }
  }

  /// Logs out the active user and clears local session.
  static Future<void> logout() async {
    currentUserNotifier.value = null;
    try {
      final file = _getSessionFile();
      if (file.existsSync()) {
        await file.delete();
      }
    } catch (e) {
      debugPrint('[AuthService] Error clearing session: $e');
    }
  }

  static Future<void> _persistSession(UserModel user) async {
    try {
      final file = _getSessionFile();
      await file.writeAsString(jsonEncode(user.toJson()), flush: true);
    } catch (e) {
      debugPrint('[AuthService] Failed to persist session: $e');
    }
  }
}
