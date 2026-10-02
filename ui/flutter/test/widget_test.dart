import 'dart:io';
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:zitera_lab/core/progress/progress_manager.dart';
import 'package:zitera_lab/main.dart';

void main() {
  testWidgets('ZITERA_LAB shell boots and displays branding in desktop viewport', (WidgetTester tester) async {
    tester.view.physicalSize = const Size(1280, 800);
    tester.view.devicePixelRatio = 1.0;
    addTearDown(tester.view.resetPhysicalSize);

    // Suppress asset loading errors in headless test environment
    final originalOnError = FlutterError.onError;
    FlutterError.onError = (details) {
      if (details.exception.toString().contains('Unable to load asset') ||
          details.exception.toString().contains('AssetBundleImageKey')) {
        return;
      }
      originalOnError?.call(details);
    };

    await tester.pumpWidget(const ZiteraLabApp());
    await tester.pump();

    expect(find.text('ZITERA'), findsAtLeastNWidgets(1));
    expect(find.text('LAB'), findsAtLeastNWidgets(1));
    expect(find.text('Dashboard'), findsOneWidget);
    expect(find.text('Labs Catalog'), findsOneWidget);
    expect(find.text('Smart Setup'), findsOneWidget);

    // Allow async microtasks to settle
    await tester.pump(const Duration(milliseconds: 500));
    FlutterError.onError = originalOnError;
  });

  group('ProgressManager Recovery & Integrity Tests', () {
    final progressFile = File('zitera_progress.json');
    final bakFile = File('zitera_progress.json.bak');
    final tmpFile = File('zitera_progress.json.tmp');

    tearDown(() {
      if (progressFile.existsSync()) progressFile.deleteSync();
      if (bakFile.existsSync()) bakFile.deleteSync();
      if (tmpFile.existsSync()) tmpFile.deleteSync();
    });

    test('Corrupted progress file creates .bak and returns clean default', () async {
      await progressFile.writeAsString('{{{INVALID JSON CORRUPTED DATA!!!');
      final data = await ProgressManager.loadProgress();
      expect(data['completed_labs'], isEmpty);
      expect(bakFile.existsSync(), isTrue);
    });

    test('Orphaned .tmp file from interrupted write is safely restored', () async {
      final validJson = '{"completed_labs":["A01"],"completed_challenges":[],"completed_practice":[],"completed_sections":{}}';
      await tmpFile.writeAsString(validJson);
      final data = await ProgressManager.loadProgress();
      expect(data['completed_labs'], contains('A01'));
      expect(progressFile.existsSync(), isTrue);
    });
  });
}
