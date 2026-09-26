import 'package:flutter/material.dart';
import '../../core/ipc/engine_client.dart';
import '../../core/ipc/models.dart';
import '../../core/theme/zitera_colors.dart';
import '../../widgets/status_badge.dart';
import '../../widgets/zitera_button.dart';
import '../../widgets/zitera_card.dart';

class ToolsView extends StatefulWidget {
  const ToolsView({super.key});

  @override
  State<ToolsView> createState() => _ToolsViewState();
}

class _ToolsViewState extends State<ToolsView> {
  List<ToolItem> _tools = [];
  bool _isLoading = true;

  @override
  void initState() {
    super.initState();
    _loadTools();
  }

  Future<void> _loadTools() async {
    setState(() => _isLoading = true);
    try {
      final t = await ZiteraEngineClient.getTools();
      setState(() {
        _tools = t;
        _isLoading = false;
      });
    } catch (_) {
      setState(() => _isLoading = false);
    }
  }

  @override
  Widget build(BuildContext context) {
    if (_isLoading) {
      return const Center(child: CircularProgressIndicator(color: ZiteraColors.primary));
    }

    return SingleChildScrollView(
      padding: const EdgeInsets.all(32.0),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Row(
            mainAxisAlignment: MainAxisAlignment.spaceBetween,
            children: [
              Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: const [
                  Text(
                    'SECURITY TOOLS MANAGER',
                    style: TextStyle(
                      color: ZiteraColors.textPrimary,
                      fontSize: 22,
                      fontWeight: FontWeight.w900,
                      letterSpacing: 1.5,
                      fontFamily: 'monospace',
                    ),
                  ),
                  SizedBox(height: 4),
                  Text(
                    'Lightweight security tooling profiles, detection, and official installation guides',
                    style: TextStyle(color: ZiteraColors.textSecondary, fontSize: 13),
                  ),
                ],
              ),
              ZiteraButton(
                label: 'Recheck Tools',
                icon: Icons.refresh,
                variant: ButtonVariant.secondary,
                onPressed: _loadTools,
              ),
            ],
          ),

          const SizedBox(height: 24),

          ListView.separated(
            shrinkWrap: true,
            physics: const NeverScrollableScrollPhysics(),
            itemCount: _tools.length,
            separatorBuilder: (_, _) => const SizedBox(height: 12),
            itemBuilder: (context, index) {
              final tool = _tools[index];
              return ZiteraCard(
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Row(
                      mainAxisAlignment: MainAxisAlignment.spaceBetween,
                      children: [
                        Row(
                          children: [
                            Container(
                              padding: const EdgeInsets.all(8),
                              decoration: BoxDecoration(
                                color: ZiteraColors.surface,
                                borderRadius: BorderRadius.circular(4),
                                border: Border.all(color: ZiteraColors.border),
                              ),
                              child: const Icon(Icons.terminal, size: 16, color: ZiteraColors.cyan),
                            ),
                            const SizedBox(width: 12),
                            Text(
                              tool.name,
                              style: const TextStyle(fontWeight: FontWeight.bold, fontSize: 16),
                            ),
                            const SizedBox(width: 8),
                            Text(
                              '// ${tool.category}',
                              style: const TextStyle(color: ZiteraColors.textMuted, fontSize: 12),
                            ),
                          ],
                        ),
                        StatusBadge(status: tool.status),
                      ],
                    ),
                    const SizedBox(height: 12),
                    if (tool.version != null) ...[
                      Text('Detected Version: ${tool.version!}', style: const TextStyle(fontFamily: 'monospace', fontSize: 12, color: ZiteraColors.ready)),
                      const SizedBox(height: 4),
                    ],
                    Text(
                      'Guide / Source: ${tool.installGuide}',
                      style: const TextStyle(color: ZiteraColors.textSecondary, fontSize: 12),
                    ),
                  ],
                ),
              );
            },
          ),
        ],
      ),
    );
  }
}
