import 'package:flutter_test/flutter_test.dart';
import 'package:zitera_lab/core/ipc/models.dart';

void main() {
  group('Dynamic Catalog & 1-Click Update System Contract Tests', () {
    test('LabItem parses dynamic update and latest_version fields correctly', () {
      final jsonWithUpdate = {
        'id': 'A01',
        'title': 'Broken Access Control',
        'installed': true,
        'running': false,
        'port': 8011,
        'version': '1.0.0',
        'status': 'STOPPED',
        'has_update': true,
        'latest_version': '1.0.1',
      };

      final lab = LabItem.fromJson(jsonWithUpdate);
      expect(lab.id, 'A01');
      expect(lab.installed, isTrue);
      expect(lab.version, '1.0.0');
      expect(lab.hasUpdate, isTrue);
      expect(lab.latestVersion, '1.0.1');

      final jsonUpToDate = {
        'id': 'A02',
        'title': 'Security Misconfiguration',
        'installed': true,
        'running': false,
        'port': 8012,
        'version': '1.0.1',
        'status': 'STOPPED',
        'has_update': false,
        'latest_version': '1.0.1',
      };

      final labUpToDate = LabItem.fromJson(jsonUpToDate);
      expect(labUpToDate.hasUpdate, isFalse);
      expect(labUpToDate.version, '1.0.1');
      expect(labUpToDate.latestVersion, '1.0.1');

      final jsonNotInstalled = {
        'id': 'A03',
        'title': 'Software Supply Chain Failures',
        'installed': false,
        'running': false,
        'port': 0,
        'version': 'N/A',
        'status': 'NOT_INSTALLED',
        'has_update': false,
        'latest_version': '1.0.1',
      };

      final labNotInstalled = LabItem.fromJson(jsonNotInstalled);
      expect(labNotInstalled.installed, isFalse);
      expect(labNotInstalled.hasUpdate, isFalse);
      expect(labNotInstalled.latestVersion, '1.0.1');
    });

    test('CatalogEntry parses installed status and update availability from engine', () {
      final catalogItemJson = {
        'id': 'A01',
        'package_id': 'zitera-lab-a01',
        'title': 'Broken Access Control',
        'repository': 'xdaamar/zitera_lab_a01',
        'version': '1.0.1',
        'difficulty': 'Beginner',
        'owasp': 'A01:2025',
        'description': 'Authorization flaws.',
        'installed': true,
        'installed_version': '1.0.0',
        'has_update': true,
        'latest_version': '1.0.1',
      };

      final entry = CatalogEntry.fromJson(catalogItemJson);
      expect(entry.id, 'A01');
      expect(entry.version, '1.0.1');
      expect(entry.installed, isTrue);
      expect(entry.installedVersion, '1.0.0');
      expect(entry.hasUpdate, isTrue);
      expect(entry.latestVersion, '1.0.1');
    });

    test('Button label action contracts for 1-Click update states', () {
      String getActionLabel(LabItem lab) {
        if (!lab.installed) {
          return '[INSTALL LAB]';
        }
        if (lab.running) {
          return 'Stop';
        }
        if (lab.hasUpdate) {
          return '[UPDATE LAB (v${lab.latestVersion})]';
        }
        return '[RUN LAB]';
      }

      final uninstalled = LabItem(
        id: 'A01',
        title: 'Lab 1',
        installed: false,
        running: false,
        port: 0,
        version: 'N/A',
        status: 'NOT_INSTALLED',
        latestVersion: '1.0.1',
      );
      expect(getActionLabel(uninstalled), '[INSTALL LAB]');

      final updateAvailable = LabItem(
        id: 'A01',
        title: 'Lab 1',
        installed: true,
        running: false,
        port: 8011,
        version: '1.0.0',
        status: 'STOPPED',
        hasUpdate: true,
        latestVersion: '1.0.1',
      );
      expect(getActionLabel(updateAvailable), '[UPDATE LAB (v1.0.1)]');

      final upToDate = LabItem(
        id: 'A01',
        title: 'Lab 1',
        installed: true,
        running: false,
        port: 8011,
        version: '1.0.1',
        status: 'STOPPED',
        hasUpdate: false,
        latestVersion: '1.0.1',
      );
      expect(getActionLabel(upToDate), '[RUN LAB]');

      final running = LabItem(
        id: 'A01',
        title: 'Lab 1',
        installed: true,
        running: true,
        port: 8011,
        version: '1.0.1',
        status: 'RUNNING',
        hasUpdate: false,
        latestVersion: '1.0.1',
      );
      expect(getActionLabel(running), 'Stop');
    });
  });
}
