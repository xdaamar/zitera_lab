import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
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
}
