import 'package:flutter/material.dart';
import '../../core/ipc/engine_client.dart';
import '../../core/ipc/models.dart';
import '../../core/theme/zitera_colors.dart';
import '../../widgets/status_badge.dart';
import '../../widgets/zitera_button.dart';
import '../../widgets/zitera_card.dart';
import '../../widgets/cute_anime_loading.dart';
import '../../widgets/hacker_tilix_entrance.dart';

class LabsView extends StatefulWidget {
  final Function(String) onSelectLab;

  const LabsView({
    super.key,
    required this.onSelectLab,
  });

  @override
  State<LabsView> createState() => _LabsViewState();
}

class _LabsViewState extends State<LabsView> {
  List<LabItem> _labs = [];
  List<CatalogEntry> _catalog = [];
  bool _isLoading = true;
  String? _error;
  String? _actionInProgressId;
  String _selectedFilter = 'ALL';
  String _searchQuery = '';

  @override
  void initState() {
    super.initState();
    _loadLabs();
  }

  Future<void> _loadLabs() async {
    setState(() {
      _isLoading = true;
      _error = null;
    });

    try {
      final results = await Future.wait([
        ZiteraEngineClient.getLabs(),
        ZiteraEngineClient.getCatalog().catchError((_) => <CatalogEntry>[]),
      ]);

      final installedLabs = results[0] as List<LabItem>;
      final catalogEntries = results[1] as List<CatalogEntry>;

      // Merge catalog items that are not yet installed
      final merged = List<LabItem>.from(installedLabs);
      for (final cat in catalogEntries) {
        if (!merged.any((l) => l.id.toUpperCase() == cat.id.toUpperCase())) {
          merged.add(
            LabItem(
              id: cat.id,
              title: cat.title,
              installed: false,
              running: false,
              port: 0,
              version: cat.version,
              status: 'NOT_INSTALLED',
              learnReadiness: 'READY',
              practiceReadiness: 'READY',
              challengeReadiness: 'READY',
            ),
          );
        }
      }

      if (mounted) {
        setState(() {
          _labs = merged;
          _catalog = catalogEntries;
          _isLoading = false;
        });
      }
    } catch (e) {
      if (mounted) {
        setState(() {
          _error = e.toString();
          _isLoading = false;
        });
      }
    }
  }

  Future<void> _handleStart(String id) async {
    setState(() => _actionInProgressId = id);
    try {
      await ZiteraEngineClient.startLab(id);
      await _loadLabs();
    } catch (e) {
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(content: Text('Start error: $e'), backgroundColor: ZiteraColors.error),
        );
      }
    } finally {
      if (mounted) setState(() => _actionInProgressId = null);
    }
  }

  Future<void> _handleStop(String id) async {
    setState(() => _actionInProgressId = id);
    try {
      await ZiteraEngineClient.stopLab(id);
      await _loadLabs();
    } catch (e) {
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(content: Text('Stop error: $e'), backgroundColor: ZiteraColors.error),
        );
      }
    } finally {
      if (mounted) setState(() => _actionInProgressId = null);
    }
  }

  Future<void> _handleReset(String id) async {
    final confirmed = await showDialog<bool>(
      context: context,
      builder: (ctx) => AlertDialog(
        backgroundColor: const Color(0xFFFCFBF8),
        shape: RoundedRectangleBorder(
          borderRadius: BorderRadius.circular(12),
          side: const BorderSide(color: ZiteraColors.border, width: 1.5),
        ),
        title: Text(
          'Reset Lab $id?',
          style: const TextStyle(
            fontFamily: 'SpaceGrotesk',
            fontWeight: FontWeight.bold,
            fontSize: 16,
            color: Color(0xFF1E1A14),
          ),
        ),
        content: Text(
          'This will restart Docker containers for $id and restore its database to initial seed state. External lesson notes will remain intact.',
          style: const TextStyle(
            fontFamily: 'JetBrainsMono',
            fontSize: 12,
            color: Color(0xFF5C5347),
          ),
        ),
        actions: [
          TextButton(
            onPressed: () => Navigator.of(ctx).pop(false),
            child: const Text('Cancel', style: TextStyle(fontFamily: 'SpaceGrotesk', color: Color(0xFF5C5347))),
          ),
          ZiteraButton(
            label: 'Yes, Reset Lab',
            icon: Icons.restore,
            variant: ButtonVariant.danger,
            onPressed: () => Navigator.of(ctx).pop(true),
          ),
        ],
      ),
    );

    if (confirmed != true) return;

    setState(() => _actionInProgressId = id);
    try {
      await ZiteraEngineClient.resetLab(id);
      await _loadLabs();
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(content: Text('Lab $id has been deterministically reset.'), backgroundColor: ZiteraColors.ready),
        );
      }
    } catch (e) {
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(content: Text('Reset error: $e'), backgroundColor: ZiteraColors.error),
        );
      }
    } finally {
      if (mounted) setState(() => _actionInProgressId = null);
    }
  }

  Future<void> _handleInstall(String id) async {
    setState(() => _actionInProgressId = id);
    try {
      final res = await ZiteraEngineClient.installLab(id);
      await _loadLabs();
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(content: Text(res), backgroundColor: ZiteraColors.ready),
        );
      }
    } catch (e) {
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(content: Text('Install error: $e'), backgroundColor: ZiteraColors.error),
        );
      }
    } finally {
      if (mounted) setState(() => _actionInProgressId = null);
    }
  }

  @override
  Widget build(BuildContext context) {
    if (_isLoading) {
      return const Center(
        child: CuteAnimeLoading(
          message: 'EXPLORING OWASP CYBER LABS...',
          subMessage: '( •̀ ω •́ )✧ DISCOVERING LAB REPOSITORIES',
        ),
      );
    }

    // Filter labs by search and category chip
    final filteredLabs = _labs.where((lab) {
      if (_searchQuery.isNotEmpty) {
        final q = _searchQuery.toLowerCase();
        final matchesTitle = lab.title.toLowerCase().contains(q);
        final matchesId = lab.id.toLowerCase().contains(q);
        if (!matchesTitle && !matchesId) return false;
      }

      switch (_selectedFilter) {
        case 'RUNNING':
          return lab.running;
        case 'INSTALLED':
          return lab.installed;
        case 'NOT INSTALLED':
          return !lab.installed;
        case 'BEGINNER':
          return true; // All baseline labs are beginner
        case 'ALL':
        default:
          return true;
      }
    }).toList();

    return Stack(
      children: [
        Positioned.fill(
          child: Image.asset(
            'assets/images/bg_labs.jpg',
            fit: BoxFit.cover,
          ),
        ),
        Positioned.fill(
          child: Container(
            color: const Color(0xFFFCFBF8).withValues(alpha: 0.91),
          ),
        ),
        SingleChildScrollView(
          padding: const EdgeInsets.symmetric(horizontal: 28.0, vertical: 24.0),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              // Header
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
                          'SECURITY LABS CATALOG',
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
                          'OWASP Top 10:2025 local isolated vulnerability environments (${_labs.length} available)',
                          style: const TextStyle(
                            color: Color(0xFF5C5347),
                            fontSize: 13,
                            fontFamily: 'JetBrainsMono',
                          ),
                        ),
                      ],
                    ),
                    Row(
                      children: [
                        Container(
                          padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 4),
                          decoration: BoxDecoration(
                            color: const Color(0xFFE6FFFA),
                            borderRadius: BorderRadius.circular(4),
                            border: Border.all(color: const Color(0xFF2DD4BF)),
                          ),
                          child: const Row(
                            children: [
                              Icon(Icons.wifi, size: 14, color: Color(0xFF0F766E)),
                              SizedBox(width: 6),
                              Text(
                                'CATALOG SYNCED',
                                style: TextStyle(
                                  fontFamily: 'JetBrainsMono',
                                  fontSize: 10,
                                  fontWeight: FontWeight.bold,
                                  color: Color(0xFF0F766E),
                                ),
                              ),
                            ],
                          ),
                        ),
                        const SizedBox(width: 10),
                        ZiteraButton(
                          label: 'Refresh Labs',
                          icon: Icons.refresh,
                          variant: ButtonVariant.secondary,
                          onPressed: _loadLabs,
                        ),
                      ],
                    ),
                  ],
                ),
              ),

              const SizedBox(height: 18),

              // Search & Filter Toolbar
              HackerTilixEntrance(
                delay: const Duration(milliseconds: 60),
                child: Row(
                children: [
                  Expanded(
                    child: Container(
                      height: 40,
                      decoration: BoxDecoration(
                        color: const Color(0xFFFCFBF8),
                        borderRadius: BorderRadius.circular(8),
                        border: Border.all(color: ZiteraColors.border),
                      ),
                      child: TextField(
                        onChanged: (val) => setState(() => _searchQuery = val),
                        style: const TextStyle(
                          fontFamily: 'SpaceGrotesk',
                          fontSize: 13,
                          color: Color(0xFF1E1A14),
                        ),
                        decoration: const InputDecoration(
                          hintText: 'Search labs by name or ID (e.g. A01, Injection)...',
                          hintStyle: TextStyle(
                            fontFamily: 'SpaceGrotesk',
                            fontSize: 12,
                            color: Color(0xFF9C9080),
                          ),
                          prefixIcon: Icon(Icons.search, size: 18, color: Color(0xFF786F62)),
                          border: InputBorder.none,
                          contentPadding: EdgeInsets.symmetric(vertical: 10),
                        ),
                      ),
                    ),
                  ),
                  const SizedBox(width: 14),
                  SingleChildScrollView(
                    scrollDirection: Axis.horizontal,
                    child: Row(
                      children: ['ALL', 'RUNNING', 'INSTALLED', 'NOT INSTALLED'].map((filter) {
                        final isSelected = _selectedFilter == filter;
                        return Padding(
                          padding: const EdgeInsets.only(left: 6.0),
                          child: InkWell(
                            onTap: () => setState(() => _selectedFilter = filter),
                            borderRadius: BorderRadius.circular(6),
                            child: Container(
                              padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 8),
                              decoration: BoxDecoration(
                                color: isSelected ? const Color(0xFFCCFBF1) : const Color(0xFFFCFBF8),
                                borderRadius: BorderRadius.circular(6),
                                border: Border.all(
                                  color: isSelected ? const Color(0xFF2DD4BF) : ZiteraColors.border,
                                  width: 1.0,
                                ),
                              ),
                              child: Text(
                                filter,
                                style: TextStyle(
                                  fontFamily: 'SpaceGrotesk',
                                  fontWeight: isSelected ? FontWeight.bold : FontWeight.w500,
                                  fontSize: 11,
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
              ),

              const SizedBox(height: 20),

              if (_error != null)
                Padding(
                  padding: const EdgeInsets.only(bottom: 16.0),
                  child: ZiteraCard(
                    borderColor: ZiteraColors.error,
                    child: Text(_error!, style: const TextStyle(color: ZiteraColors.error)),
                  ),
                ),

              if (filteredLabs.isEmpty)
                ZiteraCard(
                  child: Center(
                    child: Padding(
                      padding: const EdgeInsets.symmetric(vertical: 24.0),
                      child: Column(
                        children: [
                          const Icon(Icons.search_off, size: 36, color: Color(0xFF9C9080)),
                          const SizedBox(height: 10),
                          const Text(
                            'No laboratories matched your search or filter.',
                            style: TextStyle(
                              fontFamily: 'SpaceGrotesk',
                              fontWeight: FontWeight.bold,
                              fontSize: 14,
                              color: Color(0xFF1E1A14),
                            ),
                          ),
                          const SizedBox(height: 6),
                          ZiteraButton(
                            label: 'Reset Filters',
                            variant: ButtonVariant.secondary,
                            onPressed: () => setState(() {
                              _searchQuery = '';
                              _selectedFilter = 'ALL';
                            }),
                          ),
                        ],
                      ),
                    ),
                  ),
                )
              else
                ListView.separated(
                  shrinkWrap: true,
                  physics: const NeverScrollableScrollPhysics(),
                  itemCount: filteredLabs.length,
                  separatorBuilder: (_, _) => const SizedBox(height: 14),
                  itemBuilder: (context, index) {
                    final lab = filteredLabs[index];
                    final isBusy = _actionInProgressId == lab.id;
                    final stickerList = [
                      'assets/images/sticker_pink.png',
                      'assets/images/sticker_duck.png',
                      'assets/images/sticker_bunny.png',
                    ];
                    final stickerAsset = stickerList[index % stickerList.length];

                    // Find catalog description
                    final catDesc = _catalog
                        .firstWhere(
                          (c) => c.id.toUpperCase() == lab.id.toUpperCase(),
                          orElse: () => CatalogEntry(
                            id: lab.id,
                            title: lab.title,
                            repository: 'xdaamar/zitera_lab_${lab.id.toLowerCase()}',
                            version: lab.version,
                            difficulty: 'Beginner',
                            owasp: '${lab.id}:2025',
                            description: 'Interactive containerized security environment.',
                          ),
                        )
                        .description;

                    return HackerTilixEntrance(
                      delay: Duration(milliseconds: 100 + index * 45),
                      child: ZiteraCard(
                      borderColor: lab.running ? const Color(0xFF2DD4BF) : ZiteraColors.border,
                      child: Row(
                        children: [
                          Container(
                            width: 54,
                            height: 54,
                            decoration: BoxDecoration(
                              color: const Color(0xFFE6FFFA),
                              borderRadius: BorderRadius.circular(6),
                              border: Border.all(color: const Color(0xFF2DD4BF)),
                            ),
                            alignment: Alignment.center,
                            child: Text(
                              lab.id,
                              style: const TextStyle(
                                color: Color(0xFF0F766E),
                                fontWeight: FontWeight.w900,
                                fontSize: 16,
                                fontFamily: 'JetBrainsMono',
                              ),
                            ),
                          ),
                          const SizedBox(width: 18),
                          Expanded(
                            child: Column(
                              crossAxisAlignment: CrossAxisAlignment.start,
                              children: [
                                Row(
                                  children: [
                                    Text(
                                      lab.title,
                                      style: const TextStyle(
                                        color: Color(0xFF1E1A14),
                                        fontSize: 16,
                                        fontWeight: FontWeight.bold,
                                        fontFamily: 'SpaceGrotesk',
                                      ),
                                    ),
                                    const SizedBox(width: 10),
                                    StatusBadge(status: lab.status),
                                    const SizedBox(width: 8),
                                    Container(
                                      padding: const EdgeInsets.symmetric(horizontal: 6, vertical: 2),
                                      decoration: BoxDecoration(
                                        color: const Color(0xFFF0EDE8),
                                        borderRadius: BorderRadius.circular(4),
                                        border: Border.all(color: ZiteraColors.border),
                                      ),
                                      child: Text(
                                        'v${lab.version}',
                                        style: const TextStyle(
                                          fontFamily: 'JetBrainsMono',
                                          fontSize: 10,
                                          fontWeight: FontWeight.bold,
                                          color: Color(0xFF5C5347),
                                        ),
                                      ),
                                    ),
                                    const SizedBox(width: 8),
                                    SizedBox(
                                      width: 26,
                                      height: 26,
                                      child: Image.asset(
                                        stickerAsset,
                                        fit: BoxFit.contain,
                                        errorBuilder: (context, error, stackTrace) => const SizedBox(),
                                      ),
                                    ),
                                  ],
                                ),
                                const SizedBox(height: 4),
                                Text(
                                  catDesc,
                                  style: const TextStyle(
                                    color: Color(0xFF5C5347),
                                    fontSize: 12,
                                    fontFamily: 'SpaceGrotesk',
                                  ),
                                  maxLines: 2,
                                  overflow: TextOverflow.ellipsis,
                                ),
                                const SizedBox(height: 6),
                                Text(
                                  lab.installed
                                      ? 'Runtime: Docker (WSL2) | Port: ${lab.port > 0 ? lab.port : 'Dynamic'} | Binding: 127.0.0.1 (Local Only)'
                                      : 'Source: GitHub Remote Repository | Status: Ready to Install',
                                  style: const TextStyle(
                                    color: Color(0xFF786F62),
                                    fontSize: 11,
                                    fontFamily: 'JetBrainsMono',
                                  ),
                                ),
                              ],
                            ),
                          ),
                          const SizedBox(width: 16),
                          Row(
                            mainAxisSize: MainAxisSize.min,
                            children: [
                              if (!lab.installed) ...[
                                ZiteraButton(
                                  label: isBusy ? 'Installing...' : 'Install Lab',
                                  icon: isBusy ? Icons.hourglass_top : Icons.download,
                                  variant: ButtonVariant.primary,
                                  onPressed: isBusy ? () {} : () => _handleInstall(lab.id),
                                ),
                              ] else if (lab.running) ...[
                                ZiteraButton(
                                  label: isBusy ? 'Stopping...' : 'Stop',
                                  icon: Icons.stop,
                                  variant: ButtonVariant.danger,
                                  onPressed: isBusy ? () {} : () => _handleStop(lab.id),
                                ),
                                const SizedBox(width: 8),
                                ZiteraButton(
                                  label: isBusy ? 'Resetting...' : 'Reset',
                                  icon: Icons.restore,
                                  variant: ButtonVariant.secondary,
                                  onPressed: isBusy ? () {} : () => _handleReset(lab.id),
                                ),
                                const SizedBox(width: 8),
                                ZiteraButton(
                                  label: 'Open Details',
                                  icon: Icons.arrow_forward,
                                  variant: ButtonVariant.primary,
                                  onPressed: () => widget.onSelectLab(lab.id),
                                ),
                              ] else ...[
                                ZiteraButton(
                                  label: isBusy ? 'Starting...' : 'Start Lab',
                                  icon: Icons.play_arrow,
                                  variant: ButtonVariant.secondary,
                                  onPressed: isBusy ? () {} : () => _handleStart(lab.id),
                                ),
                                const SizedBox(width: 8),
                                ZiteraButton(
                                  label: 'Open Details',
                                  icon: Icons.arrow_forward,
                                  variant: ButtonVariant.primary,
                                  onPressed: () => widget.onSelectLab(lab.id),
                                ),
                              ],
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
