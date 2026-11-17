import 'dart:io';

/// Native URL and browser launcher without external dependencies.
/// Dispatches URL protocols directly to the host operating system's default browser.
class BrowserLauncher {
  static Future<void> openUrl(String url) async {
    final cleanUrl = url.trim();
    if (cleanUrl.isEmpty) return;

    try {
      if (Platform.isWindows) {
        // Primary method on Windows: cmd /c start "" <url>
        final res = await Process.run('cmd', ['/c', 'start', '', cleanUrl]);
        if (res.exitCode != 0) {
          await Process.run('cmd', ['/c', 'start', cleanUrl]);
        }
      } else if (Platform.isMacOS) {
        await Process.run('open', [cleanUrl]);
      } else if (Platform.isLinux) {
        await Process.run('xdg-open', [cleanUrl]);
      }
    } catch (_) {
      try {
        if (Platform.isWindows) {
          await Process.run('cmd', ['/c', 'start', cleanUrl]);
        }
      } catch (_) {}
    }
  }
}
