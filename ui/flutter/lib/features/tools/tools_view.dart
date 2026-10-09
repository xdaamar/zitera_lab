import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import '../../core/ipc/engine_client.dart';
import '../../core/ipc/models.dart';
import '../../core/theme/zitera_colors.dart';
import '../../widgets/status_badge.dart';
import '../../widgets/zitera_button.dart';
import '../../widgets/zitera_card.dart';
import '../../widgets/cute_anime_loading.dart';
import '../../widgets/hacker_tilix_entrance.dart';

class ToolsView extends StatefulWidget {
  const ToolsView({super.key});

  @override
  State<ToolsView> createState() => _ToolsViewState();
}

class _ToolsViewState extends State<ToolsView> {
  List<ToolItem> _tools = [];
  bool _isLoading = true;
  String? _installingToolId;
  String _selectedCategory = 'ALL';

  @override
  void initState() {
    super.initState();
    _loadTools();
  }

  Future<void> _loadTools() async {
    setState(() => _isLoading = true);
    try {
      final t = await ZiteraEngineClient.getTools();
      if (mounted) {
        setState(() {
          _tools = t;
          _isLoading = false;
        });
      }
    } catch (_) {
      if (mounted) {
        setState(() => _isLoading = false);
      }
    }
  }

  Future<void> _handleInstall(ToolItem tool) async {
    final confirmed = await showDialog<bool>(
      context: context,
      builder: (ctx) => AlertDialog(
        backgroundColor: const Color(0xFFFCFBF8),
        shape: RoundedRectangleBorder(
          borderRadius: BorderRadius.circular(12),
          side: const BorderSide(color: ZiteraColors.border, width: 1.5),
        ),
        title: Text(
          'Install ${tool.name}',
          style: const TextStyle(
            color: Color(0xFF1E1A14),
            fontFamily: 'SpaceGrotesk',
            fontWeight: FontWeight.bold,
            fontSize: 16,
          ),
        ),
        content: Column(
          mainAxisSize: MainAxisSize.min,
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Text(
              'Package method: ${tool.installMethod.toUpperCase()}',
              style: const TextStyle(
                fontWeight: FontWeight.bold,
                color: Color(0xFF0F766E),
                fontFamily: 'JetBrainsMono',
                fontSize: 13,
              ),
            ),
            const SizedBox(height: 8),
            const Text(
              'Zitera will run the verified package manager command for this tool. No arbitrary external scripts will be executed.',
              style: TextStyle(
                color: Color(0xFF5C5347),
                fontSize: 12,
                fontFamily: 'JetBrainsMono',
              ),
            ),
            const SizedBox(height: 12),
            Container(
              padding: const EdgeInsets.all(10),
              decoration: BoxDecoration(
                color: const Color(0xFFF0EDE8),
                borderRadius: BorderRadius.circular(6),
                border: Border.all(color: ZiteraColors.border),
              ),
              child: SelectableText(
                tool.installGuide,
                style: const TextStyle(
                  fontFamily: 'JetBrainsMono',
                  fontSize: 11,
                  color: Color(0xFF1E1A14),
                ),
              ),
            ),
          ],
        ),
        actions: [
          TextButton(
            onPressed: () => Navigator.of(ctx).pop(false),
            child: const Text(
              'Cancel',
              style: TextStyle(
                fontFamily: 'SpaceGrotesk',
                color: Color(0xFF5C5347),
                fontWeight: FontWeight.w600,
              ),
            ),
          ),
          ZiteraButton(
            label: 'Proceed with Install',
            variant: ButtonVariant.primary,
            onPressed: () => Navigator.of(ctx).pop(true),
          ),
        ],
      ),
    );

    if (confirmed != true) return;

    setState(() => _installingToolId = tool.id);
    try {
      final res = await ZiteraEngineClient.installTool(tool.id);
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(
            content: Text(res.message),
            backgroundColor: res.success ? ZiteraColors.ready : ZiteraColors.error,
            duration: const Duration(seconds: 4),
          ),
        );
        _loadTools();
      }
    } catch (e) {
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(content: Text('Installation error: $e'), backgroundColor: ZiteraColors.error),
        );
      }
    } finally {
      if (mounted) {
        setState(() => _installingToolId = null);
      }
    }
  }

  void _showGuideDialog(ToolItem tool) {
    showDialog(
      context: context,
      builder: (ctx) => AlertDialog(
        backgroundColor: const Color(0xFFFCFBF8),
        shape: RoundedRectangleBorder(
          borderRadius: BorderRadius.circular(12),
          side: const BorderSide(color: ZiteraColors.border, width: 1.5),
        ),
        title: Text(
          '${tool.name} — Installation Guide',
          style: const TextStyle(
            color: Color(0xFF1E1A14),
            fontFamily: 'SpaceGrotesk',
            fontWeight: FontWeight.bold,
            fontSize: 16,
          ),
        ),
        content: Column(
          mainAxisSize: MainAxisSize.min,
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            const Text(
              'Per Zitera Tool Security Policy, heavy GUI tools are never bundled or silently installed. Please use the verified official guide below:',
              style: TextStyle(
                color: Color(0xFF5C5347),
                fontSize: 12,
                fontFamily: 'JetBrainsMono',
              ),
            ),
            const SizedBox(height: 12),
            Container(
              padding: const EdgeInsets.all(12),
              decoration: BoxDecoration(
                color: const Color(0xFFF0EDE8),
                borderRadius: BorderRadius.circular(6),
                border: Border.all(color: ZiteraColors.border),
              ),
              child: SelectableText(
                tool.installGuide,
                style: const TextStyle(
                  color: Color(0xFF0F766E),
                  fontFamily: 'JetBrainsMono',
                  fontSize: 12,
                ),
              ),
            ),
          ],
        ),
        actions: [
          TextButton.icon(
            icon: const Icon(Icons.copy, size: 16, color: Color(0xFF0F766E)),
            label: const Text(
              'Copy Guide',
              style: TextStyle(
                fontFamily: 'SpaceGrotesk',
                color: Color(0xFF0F766E),
                fontWeight: FontWeight.bold,
              ),
            ),
            onPressed: () {
              Clipboard.setData(ClipboardData(text: tool.installGuide));
              Navigator.of(ctx).pop();
              ScaffoldMessenger.of(context).showSnackBar(
                const SnackBar(content: Text('Guide copied to clipboard!'), backgroundColor: ZiteraColors.ready),
              );
            },
          ),
          TextButton(
            onPressed: () => Navigator.of(ctx).pop(),
            child: const Text(
              'Close',
              style: TextStyle(
                fontFamily: 'SpaceGrotesk',
                color: Color(0xFF5C5347),
                fontWeight: FontWeight.w600,
              ),
            ),
          ),
        ],
      ),
    );
  }

  void _showToolDetails(ToolItem tool) {
    showDialog(
      context: context,
      builder: (ctx) => AlertDialog(
        backgroundColor: const Color(0xFFFCFBF8),
        shape: RoundedRectangleBorder(
          borderRadius: BorderRadius.circular(12),
          side: const BorderSide(color: ZiteraColors.border, width: 1.5),
        ),
        title: Row(
          children: [
            Text(
              tool.name,
              style: const TextStyle(
                fontFamily: 'SpaceGrotesk',
                fontWeight: FontWeight.bold,
                fontSize: 18,
                color: Color(0xFF1E1A14),
              ),
            ),
            const SizedBox(width: 10),
            StatusBadge(status: tool.status),
          ],
        ),
        content: SizedBox(
          width: 480,
          child: SingleChildScrollView(
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              mainAxisSize: MainAxisSize.min,
              children: [
                _detailRow('Category', tool.category),
                _detailRow('Status', tool.status),
                if (tool.version != null) _detailRow('Detected Version', tool.version!),
                if (tool.minVersion.isNotEmpty) _detailRow('Minimum Required', tool.minVersion),
                if (tool.path != null) _detailRow('Executable Path', tool.path!),
                _detailRow('Install Method', tool.installMethod.toUpperCase()),
                const SizedBox(height: 12),
                const Text(
                  'Capabilities & Use Cases:',
                  style: TextStyle(
                    fontFamily: 'SpaceGrotesk',
                    fontWeight: FontWeight.bold,
                    fontSize: 13,
                    color: Color(0xFF1E1A14),
                  ),
                ),
                const SizedBox(height: 6),
                Wrap(
                  spacing: 6,
                  runSpacing: 6,
                  children: tool.capabilities.map((c) {
                    return Container(
                      padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 4),
                      decoration: BoxDecoration(
                        color: const Color(0xFFE6FFFA),
                        borderRadius: BorderRadius.circular(4),
                        border: Border.all(color: const Color(0xFF2DD4BF)),
                      ),
                      child: Text(
                        c,
                        style: const TextStyle(
                          fontSize: 11,
                          fontFamily: 'JetBrainsMono',
                          color: Color(0xFF0F766E),
                          fontWeight: FontWeight.w600,
                        ),
                      ),
                    );
                  }).toList(),
                ),
                const SizedBox(height: 14),
                const Text(
                  'Installation / Launch Instruction:',
                  style: TextStyle(
                    fontFamily: 'SpaceGrotesk',
                    fontWeight: FontWeight.bold,
                    fontSize: 13,
                    color: Color(0xFF1E1A14),
                  ),
                ),
                const SizedBox(height: 6),
                Container(
                  width: double.infinity,
                  padding: const EdgeInsets.all(10),
                  decoration: BoxDecoration(
                    color: const Color(0xFFF0EDE8),
                    borderRadius: BorderRadius.circular(6),
                    border: Border.all(color: ZiteraColors.border),
                  ),
                  child: SelectableText(
                    tool.installGuide,
                    style: const TextStyle(
                      fontFamily: 'JetBrainsMono',
                      fontSize: 11,
                      color: Color(0xFF1E1A14),
                    ),
                  ),
                ),
              ],
            ),
          ),
        ),
        actions: [
          TextButton(
            onPressed: () => Navigator.of(ctx).pop(),
            child: const Text(
              'Done',
              style: TextStyle(
                fontFamily: 'SpaceGrotesk',
                color: Color(0xFF5C5347),
                fontWeight: FontWeight.bold,
              ),
            ),
          ),
        ],
      ),
    );
  }

  Widget _detailRow(String label, String value) {
    return Padding(
      padding: const EdgeInsets.symmetric(vertical: 4.0),
      child: Row(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          SizedBox(
            width: 140,
            child: Text(
              label,
              style: const TextStyle(
                fontFamily: 'SpaceGrotesk',
                fontWeight: FontWeight.w600,
                fontSize: 12,
                color: Color(0xFF5C5347),
              ),
            ),
          ),
          Expanded(
            child: Text(
              value,
              style: const TextStyle(
                fontFamily: 'JetBrainsMono',
                fontSize: 12,
                fontWeight: FontWeight.bold,
                color: Color(0xFF1E1A14),
              ),
            ),
          ),
        ],
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    if (_isLoading) {
      return const Center(
        child: CuteAnimeLoading(
          message: 'INITIALIZING CYBER TOOLS...',
          subMessage: '( •̀ ω •́ )✧ VERIFYING LOCAL SECURITY PACKAGES',
        ),
      );
    }

    final readyCount = _tools.where((t) => t.installed).length;

    return Stack(
      children: [
        Positioned.fill(
          child: RepaintBoundary(
            child: Image.asset(
              'assets/images/bg_tools.jpg',
              fit: BoxFit.cover,
            ),
          ),
        ),
        Positioned.fill(
          child: Container(
            color: const Color(0xFFFCFBF8).withValues(alpha: 0.91),
          ),
        ),
        SingleChildScrollView(
          padding: const EdgeInsets.all(32.0),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              HackerTilixEntrance(
                delay: Duration.zero,
                direction: TilixSlideDirection.down,
                child: Row(
                  mainAxisAlignment: MainAxisAlignment.spaceBetween,
                  children: [
                    Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        const Text(
                          'SECURITY TOOLS CAPABILITY MANAGER',
                          style: TextStyle(
                            color: Color(0xFF1E1A14),
                            fontSize: 22,
                            fontWeight: FontWeight.w900,
                            letterSpacing: 1.2,
                            fontFamily: 'SpaceGrotesk',
                          ),
                        ),
                        const SizedBox(height: 4),
                        Text(
                          'Detection, bounded versioning, and safe explicit installation contracts ($readyCount/${_tools.length} Ready)',
                          style: const TextStyle(
                            color: Color(0xFF5C5347),
                            fontSize: 13,
                            fontFamily: 'JetBrainsMono',
                          ),
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
              ),

              const SizedBox(height: 18),

              // Category Filter Chips
              HackerTilixEntrance(
                delay: const Duration(milliseconds: 60),
                direction: TilixSlideDirection.right,
                child: SingleChildScrollView(
                  scrollDirection: Axis.horizontal,
                  child: Row(
                    children: [
                      'ALL',
                      'Scanning',
                      'Vulnerability',
                      'Fuzzing',
                      'Discovery',
                      'Privilege Escalation',
                      'Proxy',
                    ].map((cat) {
                      final isSelected = _selectedCategory == cat;
                      return Padding(
                        padding: const EdgeInsets.only(right: 8.0),
                        child: InkWell(
                          onTap: () => setState(() => _selectedCategory = cat),
                          borderRadius: BorderRadius.circular(6),
                          child: Container(
                            padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 6),
                            decoration: BoxDecoration(
                              color: isSelected ? const Color(0xFFCCFBF1) : const Color(0xFFFCFBF8),
                              borderRadius: BorderRadius.circular(6),
                              border: Border.all(
                                color: isSelected ? const Color(0xFF2DD4BF) : ZiteraColors.border,
                                width: 1.0,
                              ),
                            ),
                            child: Text(
                              cat,
                              style: TextStyle(
                                fontFamily: 'SpaceGrotesk',
                                fontWeight: isSelected ? FontWeight.bold : FontWeight.w500,
                                fontSize: 12,
                                color: isSelected ? const Color(0xFF0F766E) : const Color(0xFF5C5347),
                              ),
                            ),
                          ),
                        ),
                      );
                    }).toList(),
                  ),
                ),
              ),

              const SizedBox(height: 18),

              ListView.separated(
                shrinkWrap: true,
                physics: const NeverScrollableScrollPhysics(),
                itemCount: _tools.where((t) {
                  if (_selectedCategory == 'ALL') return true;
                  return t.category.toLowerCase().contains(_selectedCategory.toLowerCase());
                }).length,
                separatorBuilder: (_, _) => const SizedBox(height: 14),
                itemBuilder: (context, index) {
                  final filteredList = _tools.where((t) {
                    if (_selectedCategory == 'ALL') return true;
                    return t.category.toLowerCase().contains(_selectedCategory.toLowerCase());
                  }).toList();
                  final tool = filteredList[index];
                  final isInstalling = _installingToolId == tool.id;
                  final toolSticker = index % 3 == 0
                      ? 'assets/images/sticker_hamster.png'
                      : (index % 3 == 1
                          ? 'assets/images/sticker_shiba.png'
                          : 'assets/images/sticker_cat_laptop.png');

                  return HackerTilixEntrance(
                    delay: Duration(milliseconds: 100 + index * 40),
                    direction: TilixSlideDirection.up,
                    child: ZiteraCard(
                    borderColor: tool.installed ? ZiteraColors.ready.withValues(alpha: 0.3) : ZiteraColors.border,
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
                                    color: const Color(0xFFE6FFFA),
                                    borderRadius: BorderRadius.circular(4),
                                    border: Border.all(color: const Color(0xFF2DD4BF)),
                                  ),
                                  child: const Icon(Icons.terminal, size: 18, color: Color(0xFF0F766E)),
                                ),
                                const SizedBox(width: 14),
                                Column(
                                  crossAxisAlignment: CrossAxisAlignment.start,
                                  children: [
                                    Row(
                                      children: [
                                        Text(
                                          tool.name,
                                          style: const TextStyle(
                                            fontFamily: 'SpaceGrotesk',
                                            fontWeight: FontWeight.bold,
                                            fontSize: 16,
                                            color: Color(0xFF1E1A14),
                                          ),
                                        ),
                                        const SizedBox(width: 10),
                                        Text(
                                          '// ${tool.category}',
                                          style: const TextStyle(
                                            fontFamily: 'JetBrainsMono',
                                            color: Color(0xFF786F62),
                                            fontSize: 12,
                                          ),
                                        ),
                                      ],
                                    ),
                                    if (tool.version != null) ...[
                                      const SizedBox(height: 3),
                                      Text(
                                        tool.version!,
                                        style: const TextStyle(
                                          color: Color(0xFF0F766E),
                                          fontSize: 12,
                                          fontFamily: 'JetBrainsMono',
                                        ),
                                      ),
                                    ],
                                  ],
                                ),
                              ],
                            ),
                            Row(
                              children: [
                                SizedBox(
                                  width: 28,
                                  height: 28,
                                  child: Image.asset(
                                    toolSticker,
                                    fit: BoxFit.contain,
                                    errorBuilder: (context, error, stackTrace) => const SizedBox(),
                                  ),
                                ),
                                const SizedBox(width: 8),
                                StatusBadge(status: tool.status),
                              ],
                            ),
                          ],
                        ),

                    if (tool.capabilities.isNotEmpty) ...[
                      const SizedBox(height: 12),
                      Wrap(
                        spacing: 8,
                        runSpacing: 6,
                        children: tool.capabilities.map((c) {
                          return Container(
                            padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 3),
                            decoration: BoxDecoration(
                              color: ZiteraColors.surface,
                              borderRadius: BorderRadius.circular(3),
                              border: Border.all(color: ZiteraColors.border),
                            ),
                            child: Text(c, style: const TextStyle(fontSize: 11, color: ZiteraColors.textSecondary)),
                          );
                        }).toList(),
                      ),
                    ],

                    if (tool.path != null) ...[
                      const SizedBox(height: 10),
                      Text(
                        'Binary: ${tool.path}',
                        style: const TextStyle(color: ZiteraColors.textMuted, fontSize: 11, fontFamily: 'monospace'),
                      ),
                    ],

                    const SizedBox(height: 16),
                    const Divider(color: ZiteraColors.border),
                    const SizedBox(height: 8),

                    Row(
                      mainAxisAlignment: MainAxisAlignment.spaceBetween,
                      children: [
                        Expanded(
                          child: Text(
                            tool.installed ? 'Verified ready for active lab practice and challenges.' : tool.installGuide,
                            style: TextStyle(
                              color: tool.installed ? ZiteraColors.ready : ZiteraColors.textSecondary,
                              fontSize: 12,
                            ),
                            maxLines: 2,
                            overflow: TextOverflow.ellipsis,
                          ),
                        ),
                        const SizedBox(width: 16),
                        Row(
                          mainAxisSize: MainAxisSize.min,
                          children: [
                            ZiteraButton(
                              label: 'Details',
                              icon: Icons.info_outline,
                              variant: ButtonVariant.secondary,
                              onPressed: () => _showToolDetails(tool),
                            ),
                            const SizedBox(width: 8),
                            if (!tool.installed) ...[
                              if (tool.installMethod == 'guide')
                                ZiteraButton(
                                  label: 'View Guide',
                                  icon: Icons.menu_book,
                                  variant: ButtonVariant.secondary,
                                  onPressed: () => _showGuideDialog(tool),
                                )
                              else
                                ZiteraButton(
                                  label: isInstalling ? 'Installing...' : 'Install (${tool.installMethod.toUpperCase()})',
                                  icon: isInstalling ? Icons.hourglass_top : Icons.download,
                                  variant: ButtonVariant.primary,
                                  onPressed: isInstalling ? () {} : () => _handleInstall(tool),
                                ),
                            ] else ...[
                              const Row(
                                children: [
                                  Icon(Icons.check, size: 16, color: ZiteraColors.ready),
                                  SizedBox(width: 6),
                                  Text('READY', style: TextStyle(color: ZiteraColors.ready, fontSize: 12, fontWeight: FontWeight.bold)),
                                ],
                              ),
                            ],
                          ],
                        ),
                      ],
                    ),
                  ],
                ),
                ),
              );
            },
          ),
        ],
      ),
    ),
      ],
    );
  }
}
