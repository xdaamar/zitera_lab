import 'package:flutter/material.dart';
import '../../core/ipc/engine_client.dart';
import '../../core/ipc/models.dart';
import '../../core/theme/zitera_colors.dart';
import '../../widgets/status_badge.dart';
import '../../widgets/zitera_button.dart';
import '../../widgets/zitera_card.dart';

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
  bool _isLoading = true;
  String? _error;

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
      final labs = await ZiteraEngineClient.getLabs();
      setState(() {
        _labs = labs;
        _isLoading = false;
      });
    } catch (e) {
      setState(() {
        _error = e.toString();
        _isLoading = false;
      });
    }
  }

  Future<void> _handleStart(String id) async {
    try {
      await ZiteraEngineClient.startLab(id);
      _loadLabs();
    } catch (e) {
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(content: Text('Start error: $e'), backgroundColor: ZiteraColors.error),
        );
      }
    }
  }

  Future<void> _handleStop(String id) async {
    try {
      await ZiteraEngineClient.stopLab(id);
      _loadLabs();
    } catch (e) {
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(content: Text('Stop error: $e'), backgroundColor: ZiteraColors.error),
        );
      }
    }
  }

  Future<void> _handleReset(String id) async {
    try {
      await ZiteraEngineClient.resetLab(id);
      _loadLabs();
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
    }
  }

  @override
  Widget build(BuildContext context) {
    if (_isLoading) {
      return const Center(
        child: CircularProgressIndicator(color: ZiteraColors.primary),
      );
    }

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
                        'SECURITY LABS CATALOG',
                        style: TextStyle(
                          color: Color(0xFF1E1A14),
                          fontSize: 22,
                          fontWeight: FontWeight.w900,
                          letterSpacing: 1.2,
                          fontFamily: 'SpaceGrotesk',
                        ),
                      ),
                      SizedBox(height: 4),
                      Text(
                        'OWASP Top 10:2025 local isolated vulnerability environments',
                        style: TextStyle(
                          color: Color(0xFF5C5347),
                          fontSize: 13,
                          fontFamily: 'JetBrainsMono',
                        ),
                      ),
                    ],
                  ),
                  ZiteraButton(
                    label: 'Refresh Labs',
                    icon: Icons.refresh,
                    variant: ButtonVariant.secondary,
                    onPressed: _loadLabs,
                  ),
                ],
              ),

              const SizedBox(height: 24),

              if (_error != null)
                ZiteraCard(
                  borderColor: ZiteraColors.error,
                  child: Text(_error!, style: const TextStyle(color: ZiteraColors.error)),
                ),

              ListView.separated(
                shrinkWrap: true,
                physics: const NeverScrollableScrollPhysics(),
                itemCount: _labs.length,
                separatorBuilder: (_, _) => const SizedBox(height: 16),
                itemBuilder: (context, index) {
                  final lab = _labs[index];
                  final stickerList = [
                    'assets/images/sticker_pink.png',
                    'assets/images/sticker_duck.png',
                    'assets/images/sticker_bunny.png',
                  ];
                  final stickerAsset = stickerList[index % stickerList.length];

                  return ZiteraCard(
                    child: Row(
                      children: [
                        Container(
                          width: 54,
                          height: 54,
                          decoration: BoxDecoration(
                            color: ZiteraColors.primaryMuted,
                            borderRadius: BorderRadius.circular(6),
                            border: Border.all(color: ZiteraColors.primary.withValues(alpha: 0.5)),
                          ),
                          alignment: Alignment.center,
                          child: Text(
                            lab.id,
                            style: const TextStyle(
                              color: ZiteraColors.primary,
                              fontWeight: FontWeight.w900,
                              fontSize: 16,
                              fontFamily: 'JetBrainsMono',
                            ),
                          ),
                        ),
                        const SizedBox(width: 20),
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
                                  const SizedBox(width: 12),
                                  StatusBadge(status: lab.status),
                                  const SizedBox(width: 10),
                                  SizedBox(
                                    width: 28,
                                    height: 28,
                                    child: Image.asset(
                                      stickerAsset,
                                      fit: BoxFit.contain,
                                      errorBuilder: (context, error, stackTrace) => const SizedBox(),
                                    ),
                                  ),
                                ],
                              ),
                              const SizedBox(height: 6),
                              Text(
                                'Runtime: Docker (WSL2) | Port: ${lab.port} | Binding: 127.0.0.1 (Local Only)',
                                style: const TextStyle(
                                  color: Color(0xFF5C5347),
                                  fontSize: 12,
                                  fontFamily: 'JetBrainsMono',
                                ),
                              ),
                            ],
                          ),
                        ),
                        Row(
                          children: [
                            if (lab.running) ...[
                              ZiteraButton(
                                label: 'Stop',
                                icon: Icons.stop,
                                variant: ButtonVariant.danger,
                                onPressed: () => _handleStop(lab.id),
                              ),
                              const SizedBox(width: 8),
                              ZiteraButton(
                                label: 'Reset',
                                icon: Icons.restore,
                                variant: ButtonVariant.secondary,
                                onPressed: () => _handleReset(lab.id),
                              ),
                            ] else ...[
                              ZiteraButton(
                                label: 'Start Lab',
                                icon: Icons.play_arrow,
                                variant: ButtonVariant.secondary,
                                onPressed: () => _handleStart(lab.id),
                              ),
                            ],
                            const SizedBox(width: 12),
                            ZiteraButton(
                              label: 'Open Details',
                              icon: Icons.arrow_forward,
                              variant: ButtonVariant.primary,
                              onPressed: () => widget.onSelectLab(lab.id),
                            ),
                          ],
                        ),
                      ],
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
