import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:zitera_lab/core/readiness/component_readiness.dart';
import 'package:zitera_lab/features/readiness/first_run_view.dart';

void main() {
  group('First-Run Component Readiness Unit & Security Tests', () {
    test('ComponentReadinessChecker log sanitization redacts flags and tokens', () {
      final checker = ComponentReadinessChecker();
      checker.appendLog('Normal info message');
      checker.appendLog('Leak check: zitera_flag{secret_super_flag_12345}');
      checker.appendLog('Token check: 88f73194b90fb93f7cbd13fc5755c5ebe78451043d8b038496b1d0650c11ab1b');

      final logs = checker.logs;
      expect(logs.length, 3);
      expect(logs[0], contains('Normal info message'));
      expect(logs[1], contains('zitera_flag{[REDACTED]}'));
      expect(logs[1], isNot(contains('secret_super_flag_12345')));
      expect(logs[2], contains('[REDACTED_TOKEN]'));
    });

    test('ComponentReadinessChecker initializes and runs audit producing 7 items', () async {
      final checker = ComponentReadinessChecker();
      final report = await checker.runAudit();

      expect(report.containsKey('overall'), isTrue);
      expect(report.containsKey('items'), isTrue);
      final items = report['items'] as List<ComponentCheckItem>;
      expect(items.length, 7);

      final ids = items.map((i) => i.id).toList();
      expect(ids, contains('bundled_engine'));
      expect(ids, contains('engine_version'));
      expect(ids, contains('engine_health'));
      expect(ids, contains('flutter_runtime'));
      expect(ids, contains('catalog_integrity'));
      expect(ids, contains('lab_packages'));
      expect(ids, contains('windows_runtime'));
    });
  });

  group('FirstRunView Widget Tests', () {
    testWidgets('FirstRunView renders checklist, banner, and expandable logs', (WidgetTester tester) async {
      tester.view.physicalSize = const Size(1280, 800);
      tester.view.devicePixelRatio = 1.0;
      addTearDown(tester.view.resetPhysicalSize);

      await tester.pumpWidget(
        const MaterialApp(
          home: FirstRunView(standalone: true, autoStart: false),
        ),
      );
      await tester.pump();

      expect(find.text('ZITERA 2.0 // FIRST-RUN READINESS'), findsOneWidget);
      expect(find.text('System & Component Verification'), findsOneWidget);
      expect(find.text('COMPONENT AUDIT CHECKLIST'), findsOneWidget);
      expect(find.text('Bundled Rust Engine'), findsOneWidget);
      expect(find.text('Re-Run Audit'), findsOneWidget);

      // Drag scroll view to reveal logs section
      await tester.drag(find.byType(SingleChildScrollView), const Offset(0, -800));
      await tester.pump();

      final viewLogsButton = find.byWidgetPredicate((w) =>
          w is Text && (w.data?.contains('VIEW DETAILED EXECUTION LOGS') ?? false)).first;
      await tester.tap(viewLogsButton);
      await tester.pump();

      expect(find.text('HIDE EXECUTION LOGS'), findsOneWidget);
      expect(find.text('Copy Sanitized Log'), findsOneWidget);

      // Settle HackerTilixEntrance animation microtasks and timers
      await tester.pump(const Duration(milliseconds: 600));
    });
  });
}
