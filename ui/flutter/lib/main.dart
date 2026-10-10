import 'package:flutter/material.dart';
import 'core/theme/zitera_theme.dart';
import 'features/dashboard/dashboard_view.dart';
import 'features/environment/environment_view.dart';
import 'features/labs/lab_detail_view.dart';
import 'features/labs/labs_view.dart';
import 'features/settings/settings_view.dart';
import 'features/tools/tools_view.dart';
import 'widgets/sidebar.dart';
import 'widgets/cute_anime_cursor.dart';
import 'core/i18n/language_controller.dart';

void main() async {
  WidgetsFlutterBinding.ensureInitialized();
  await AppLanguageController.init();
  runApp(const ZiteraLabApp());
}

class ZiteraLabApp extends StatelessWidget {
  const ZiteraLabApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'ZITERA_LAB // Cyber Security Laboratory',
      debugShowCheckedModeBanner: false,
      theme: ZiteraTheme.darkTheme,
      builder: (context, child) {
        return CuteAnimeCursor(child: child ?? const SizedBox());
      },
      home: const MainShell(),
    );
  }
}

class MainShell extends StatefulWidget {
  const MainShell({super.key});

  @override
  State<MainShell> createState() => _MainShellState();
}

class _MainShellState extends State<MainShell> {
  int _selectedIndex = 0;
  String? _activeLabId;

  void _navigateToTab(int index) {
    setState(() {
      _selectedIndex = index;
      _activeLabId = null; // Clear active lab when switching primary navigation tabs
    });
  }

  void _openLab(String id) {
    setState(() {
      _selectedIndex = 1;
      _activeLabId = id;
    });
  }

  void _closeLab() {
    setState(() {
      _activeLabId = null;
    });
  }

  @override
  Widget build(BuildContext context) {
    Widget activeContent;

    if (_activeLabId != null) {
      activeContent = LabDetailView(
        labId: _activeLabId!,
        onBack: _closeLab,
      );
    } else {
      switch (_selectedIndex) {
        case 0:
          activeContent = DashboardView(
            onNavigate: _navigateToTab,
            onOpenLab: _openLab,
          );
          break;
        case 1:
          activeContent = LabsView(
            onSelectLab: _openLab,
          );
          break;
        case 2:
          activeContent = EnvironmentView(
            onNavigateToDashboard: () => _navigateToTab(0),
          );
          break;
        case 3:
          activeContent = const ToolsView();
          break;
        case 4:
          activeContent = const SettingsView();
          break;
        default:
          activeContent = DashboardView(
            onNavigate: _navigateToTab,
            onOpenLab: _openLab,
          );
      }
    }

    return Scaffold(
      body: Row(
        children: [
          RepaintBoundary(
            child: Sidebar(
              selectedIndex: _selectedIndex,
              onDestinationSelected: _navigateToTab,
            ),
          ),
          Expanded(
            child: KeyedSubtree(
              key: ValueKey('tab-$_selectedIndex-lab-$_activeLabId'),
              child: activeContent,
            ),
          ),
        ],
      ),
    );
  }
}
