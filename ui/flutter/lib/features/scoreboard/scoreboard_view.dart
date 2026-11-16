import 'package:flutter/material.dart';
import '../../core/auth/auth_service.dart';
import '../../core/auth/avatar_registry.dart';
import '../../core/scoreboard/scoreboard_service.dart';
import '../../core/theme/zitera_colors.dart';
import '../../widgets/auth_dialog.dart';

/// Live CTF Scoreboard & Operative Ranking View.
class ScoreboardView extends StatefulWidget {
  const ScoreboardView({super.key});

  @override
  State<ScoreboardView> createState() => _ScoreboardViewState();
}

class _ScoreboardViewState extends State<ScoreboardView> {
  bool _isLoading = true;
  String? _error;
  List<ScoreboardEntry> _entries = [];
  String _searchQuery = '';

  @override
  void initState() {
    super.initState();
    _fetchScoreboard();
    AuthService.currentUserNotifier.addListener(_onAuthChanged);
  }

  @override
  void dispose() {
    AuthService.currentUserNotifier.removeListener(_onAuthChanged);
    super.dispose();
  }

  void _onAuthChanged() {
    if (mounted) {
      _fetchScoreboard(silent: true);
    }
  }

  Future<void> _fetchScoreboard({bool silent = false}) async {
    if (!silent) {
      setState(() {
        _isLoading = true;
        _error = null;
      });
    }

    try {
      final list = await ScoreboardService.getLeaderboard();
      if (mounted) {
        setState(() {
          _entries = list;
          _isLoading = false;
          _error = null;
        });
      }
    } catch (e) {
      if (mounted) {
        setState(() {
          _isLoading = false;
          _error = 'Failed to synchronize with Turso DB: $e';
        });
      }
    }
  }

  @override
  Widget build(BuildContext context) {
    final filtered = _entries.where((e) {
      if (_searchQuery.isEmpty) return true;
      return e.username.toLowerCase().contains(_searchQuery.toLowerCase());
    }).toList();

    return Scaffold(
      backgroundColor: ZiteraColors.background,
      body: Stack(
        children: [
          // Background graphic tint
          Positioned.fill(
            child: Container(
              color: ZiteraColors.background,
            ),
          ),

          // Main Scrollable Body
          Column(
            children: [
              // Top Header Bar
              _buildHeaderBar(),

              // Content Area
              Expanded(
                child: _isLoading
                    ? _buildLoadingState()
                    : _error != null
                        ? _buildErrorState()
                        : _buildScoreboardContent(filtered),
              ),

              // Bottom Sticky Operative Status Bar
              _buildBottomOperativeBar(),
            ],
          ),
        ],
      ),
    );
  }

  Widget _buildHeaderBar() {
    return Container(
      padding: const EdgeInsets.symmetric(horizontal: 28, vertical: 20),
      decoration: const BoxDecoration(
        color: ZiteraColors.card,
        border: Border(
          bottom: BorderSide(color: ZiteraColors.border, width: 1.5),
        ),
      ),
      child: Row(
        mainAxisAlignment: MainAxisAlignment.spaceBetween,
        children: [
          // Title Section
          Row(
            children: [
              Container(
                width: 44,
                height: 44,
                decoration: BoxDecoration(
                  color: ZiteraColors.statChallengesBg,
                  borderRadius: BorderRadius.circular(10),
                  border: Border.all(color: ZiteraColors.statChallengesBorder, width: 1.2),
                ),
                child: const Icon(
                  Icons.military_tech_rounded,
                  color: ZiteraColors.statChallengesText,
                  size: 26,
                ),
              ),
              const SizedBox(width: 14),
              const Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Row(
                    children: [
                      Text(
                        'LIVE CTF // SCOREBOARD',
                        style: TextStyle(
                          fontFamily: 'SpaceGrotesk',
                          fontSize: 20,
                          fontWeight: FontWeight.bold,
                          color: Color(0xFF1E1A14),
                          letterSpacing: 0.8,
                        ),
                      ),
                      SizedBox(width: 8),
                      _LivePill(),
                    ],
                  ),
                  SizedBox(height: 2),
                  Text(
                    'Real-time decentralized ranking synchronized via Turso Cloud DB',
                    style: TextStyle(
                      fontFamily: 'JetBrainsMono',
                      fontSize: 11,
                      color: Color(0xFF78716C),
                    ),
                  ),
                ],
              ),
            ],
          ),

          // Action Buttons: Search + Refresh + Auth Pill
          Row(
            children: [
              // Search input
              Container(
                width: 220,
                height: 38,
                decoration: BoxDecoration(
                  color: Colors.white,
                  borderRadius: BorderRadius.circular(8),
                  border: Border.all(color: ZiteraColors.border),
                ),
                child: TextField(
                  onChanged: (val) => setState(() => _searchQuery = val.trim()),
                  style: const TextStyle(fontFamily: 'JetBrainsMono', fontSize: 12),
                  decoration: const InputDecoration(
                    hintText: 'Filter operatives...',
                    hintStyle: TextStyle(
                      fontFamily: 'JetBrainsMono',
                      fontSize: 11,
                      color: Color(0xFFA8A29E),
                    ),
                    prefixIcon: Icon(Icons.search, size: 16, color: Color(0xFF78716C)),
                    border: InputBorder.none,
                    contentPadding: EdgeInsets.symmetric(vertical: 10),
                  ),
                ),
              ),
              const SizedBox(width: 12),

              // Refresh Button
              IconButton(
                tooltip: 'Sync Leaderboard',
                style: IconButton.styleFrom(
                  backgroundColor: ZiteraColors.surface,
                  shape: RoundedRectangleBorder(
                    borderRadius: BorderRadius.circular(8),
                    side: const BorderSide(color: ZiteraColors.border),
                  ),
                ),
                icon: const Icon(Icons.refresh, size: 18, color: Color(0xFF44403C)),
                onPressed: () => _fetchScoreboard(),
              ),
              const SizedBox(width: 12),

              // Auth Pill
              ValueListenableBuilder<UserModel?>(
                valueListenable: AuthService.currentUserNotifier,
                builder: (context, user, _) {
                  if (user != null) {
                    return ElevatedButton.icon(
                      style: ElevatedButton.styleFrom(
                        backgroundColor: ZiteraColors.statLabsBg,
                        foregroundColor: ZiteraColors.statLabsText,
                        elevation: 0,
                        side: const BorderSide(color: ZiteraColors.statLabsBorder),
                        padding: const EdgeInsets.symmetric(horizontal: 14, vertical: 10),
                        shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(8)),
                      ),
                      onPressed: () => AuthDialog.show(context),
                      icon: const Icon(Icons.person, size: 16, color: ZiteraColors.statLabsText),
                      label: Text(
                        'OP: ${user.username}',
                        style: const TextStyle(
                          fontFamily: 'JetBrainsMono',
                          fontSize: 12,
                          fontWeight: FontWeight.bold,
                        ),
                      ),
                    );
                  } else {
                    return ElevatedButton.icon(
                      style: ElevatedButton.styleFrom(
                        backgroundColor: const Color(0xFF0D9488),
                        foregroundColor: Colors.white,
                        elevation: 0,
                        padding: const EdgeInsets.symmetric(horizontal: 14, vertical: 10),
                        shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(8)),
                      ),
                      onPressed: () => AuthDialog.show(context),
                      icon: const Icon(Icons.login, size: 16),
                      label: const Text(
                        'LOGIN TO CTF',
                        style: TextStyle(
                          fontFamily: 'JetBrainsMono',
                          fontSize: 12,
                          fontWeight: FontWeight.bold,
                        ),
                      ),
                    );
                  }
                },
              ),
            ],
          ),
        ],
      ),
    );
  }

  Widget _buildScoreboardContent(List<ScoreboardEntry> filtered) {
    final totalSolves = _entries.fold<int>(0, (sum, e) => sum + e.labsSolved);
    final topScore = _entries.isNotEmpty ? _entries.first.totalPoints : 0;
    final topUsername = _entries.isNotEmpty ? _entries.first.username : 'None';

    return SingleChildScrollView(
      padding: const EdgeInsets.all(28),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          // Stat Overview Cards
          Row(
            children: [
              _buildStatCard(
                title: 'TOTAL OPERATIVES',
                value: '${_entries.length}',
                subtitle: 'Registered contenders',
                icon: Icons.groups_outlined,
                bg: ZiteraColors.statLabsBg,
                border: ZiteraColors.statLabsBorder,
                textColor: ZiteraColors.statLabsText,
              ),
              const SizedBox(width: 16),
              _buildStatCard(
                title: 'TOTAL SOLVES',
                value: '$totalSolves',
                subtitle: 'Challenge flags submitted',
                icon: Icons.flag_outlined,
                bg: ZiteraColors.statChallengesBg,
                border: ZiteraColors.statChallengesBorder,
                textColor: ZiteraColors.statChallengesText,
              ),
              const SizedBox(width: 16),
              _buildStatCard(
                title: 'LEADERBOARD APEX',
                value: '$topScore PTS',
                subtitle: 'Leader: $topUsername',
                icon: Icons.workspace_premium_outlined,
                bg: ZiteraColors.statRuntimesBg,
                border: ZiteraColors.statRuntimesBorder,
                textColor: ZiteraColors.statRuntimesText,
              ),
            ],
          ),
          const SizedBox(height: 24),

          // Leaderboard Table Card
          Container(
            decoration: BoxDecoration(
              color: ZiteraColors.card,
              borderRadius: BorderRadius.circular(12),
              border: Border.all(color: ZiteraColors.borderDark, width: 1.5),
              boxShadow: [
                BoxShadow(
                  color: Colors.black.withValues(alpha: 0.03),
                  blurRadius: 10,
                  offset: const Offset(0, 3),
                ),
              ],
            ),
            child: Column(
              children: [
                // Table Header Row
                Container(
                  padding: const EdgeInsets.symmetric(horizontal: 20, vertical: 14),
                  decoration: const BoxDecoration(
                    color: Color(0xFFF5F3EF),
                    borderRadius: BorderRadius.only(
                      topLeft: Radius.circular(10),
                      topRight: Radius.circular(10),
                    ),
                    border: Border(
                      bottom: BorderSide(color: ZiteraColors.border, width: 1.2),
                    ),
                  ),
                  child: const Row(
                    children: [
                      SizedBox(
                        width: 70,
                        child: Text(
                          'RANK',
                          style: TextStyle(
                            fontFamily: 'JetBrainsMono',
                            fontSize: 11,
                            fontWeight: FontWeight.bold,
                            color: Color(0xFF57534E),
                          ),
                        ),
                      ),
                      Expanded(
                        flex: 4,
                        child: Text(
                          'OPERATIVE',
                          style: TextStyle(
                            fontFamily: 'JetBrainsMono',
                            fontSize: 11,
                            fontWeight: FontWeight.bold,
                            color: Color(0xFF57534E),
                          ),
                        ),
                      ),
                      Expanded(
                        flex: 3,
                        child: Text(
                          'LABS SOLVED',
                          style: TextStyle(
                            fontFamily: 'JetBrainsMono',
                            fontSize: 11,
                            fontWeight: FontWeight.bold,
                            color: Color(0xFF57534E),
                          ),
                        ),
                      ),
                      Expanded(
                        flex: 3,
                        child: Text(
                          'TOTAL POINTS',
                          style: TextStyle(
                            fontFamily: 'JetBrainsMono',
                            fontSize: 11,
                            fontWeight: FontWeight.bold,
                            color: Color(0xFF57534E),
                          ),
                        ),
                      ),
                      Expanded(
                        flex: 3,
                        child: Text(
                          'LAST SOLVE TIME',
                          style: TextStyle(
                            fontFamily: 'JetBrainsMono',
                            fontSize: 11,
                            fontWeight: FontWeight.bold,
                            color: Color(0xFF57534E),
                          ),
                        ),
                      ),
                    ],
                  ),
                ),

                // Table Rows
                if (filtered.isEmpty)
                  Padding(
                    padding: const EdgeInsets.symmetric(vertical: 48),
                    child: Center(
                      child: Column(
                        mainAxisSize: MainAxisSize.min,
                        children: [
                          Icon(
                            Icons.sentiment_dissatisfied_outlined,
                            size: 40,
                            color: Colors.grey.shade400,
                          ),
                          const SizedBox(height: 10),
                          const Text(
                            'No contenders found matching filter.',
                            style: TextStyle(
                              fontFamily: 'JetBrainsMono',
                              fontSize: 13,
                              color: Color(0xFF78716C),
                            ),
                          ),
                        ],
                      ),
                    ),
                  )
                else
                  ListView.separated(
                    shrinkWrap: true,
                    physics: const NeverScrollableScrollPhysics(),
                    itemCount: filtered.length,
                    separatorBuilder: (context, index) => const Divider(
                      height: 1,
                      thickness: 1,
                      color: ZiteraColors.border,
                    ),
                    itemBuilder: (context, index) {
                      final entry = filtered[index];
                      return _buildTableRow(entry);
                    },
                  ),
              ],
            ),
          ),
        ],
      ),
    );
  }

  Widget _buildTableRow(ScoreboardEntry entry) {
    final isMe = entry.isCurrentUser;

    return Container(
      padding: const EdgeInsets.symmetric(horizontal: 20, vertical: 14),
      decoration: BoxDecoration(
        color: isMe ? const Color(0xFFF0FDF4) : Colors.transparent,
      ),
      child: Row(
        children: [
          // Rank Badge
          SizedBox(
            width: 70,
            child: _buildRankBadge(entry.rank),
          ),

          // Operative Name & Avatar
          Expanded(
            flex: 4,
            child: Row(
              children: [
                Container(
                  width: 32,
                  height: 32,
                  decoration: BoxDecoration(
                    shape: BoxShape.circle,
                    color: isMe ? ZiteraColors.statLabsBg : ZiteraColors.surface,
                    border: Border.all(
                      color: isMe ? const Color(0xFF0D9488) : ZiteraColors.border,
                      width: isMe ? 1.5 : 1,
                    ),
                    boxShadow: isMe
                        ? [
                            BoxShadow(
                              color: const Color(0xFF0D9488).withValues(alpha: 0.25),
                              blurRadius: 4,
                            ),
                          ]
                        : null,
                  ),
                  child: ClipOval(
                    child: AvatarRegistry.buildAvatarImage(
                      entry.avatarId,
                      width: 32,
                      height: 32,
                    ),
                  ),
                ),
                const SizedBox(width: 10),
                Flexible(
                  child: Text(
                    entry.username,
                    style: TextStyle(
                      fontFamily: 'SpaceGrotesk',
                      fontSize: 14,
                      fontWeight: isMe ? FontWeight.bold : FontWeight.w600,
                      color: isMe ? const Color(0xFF0F766E) : const Color(0xFF1E1A14),
                    ),
                    overflow: TextOverflow.ellipsis,
                  ),
                ),
                if (isMe) ...[
                  const SizedBox(width: 8),
                  Container(
                    padding: const EdgeInsets.symmetric(horizontal: 6, vertical: 2),
                    decoration: BoxDecoration(
                      color: const Color(0xFF16A34A),
                      borderRadius: BorderRadius.circular(4),
                    ),
                    child: const Text(
                      'YOU',
                      style: TextStyle(
                        fontFamily: 'JetBrainsMono',
                        fontSize: 9,
                        fontWeight: FontWeight.bold,
                        color: Colors.white,
                      ),
                    ),
                  ),
                ],
              ],
            ),
          ),

          // Labs Solved
          Expanded(
            flex: 3,
            child: Row(
              children: [
                Container(
                  padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 3),
                  decoration: BoxDecoration(
                    color: entry.labsSolved > 0
                        ? ZiteraColors.statChallengesBg
                        : ZiteraColors.surface,
                    borderRadius: BorderRadius.circular(4),
                    border: Border.all(
                      color: entry.labsSolved > 0
                          ? ZiteraColors.statChallengesBorder
                          : ZiteraColors.border,
                    ),
                  ),
                  child: Text(
                    '${entry.labsSolved} / 10',
                    style: TextStyle(
                      fontFamily: 'JetBrainsMono',
                      fontSize: 11,
                      fontWeight: FontWeight.bold,
                      color: entry.labsSolved > 0
                          ? ZiteraColors.statChallengesText
                          : const Color(0xFF78716C),
                    ),
                  ),
                ),
              ],
            ),
          ),

          // Total Points
          Expanded(
            flex: 3,
            child: Text(
              '${entry.totalPoints} PTS',
              style: const TextStyle(
                fontFamily: 'JetBrainsMono',
                fontSize: 13,
                fontWeight: FontWeight.bold,
                color: Color(0xFF0D9488),
              ),
            ),
          ),

          // Last Solve Time
          Expanded(
            flex: 3,
            child: Row(
              children: [
                const Icon(Icons.schedule, size: 14, color: Color(0xFF78716C)),
                const SizedBox(width: 6),
                Text(
                  entry.formattedLastSolve,
                  style: const TextStyle(
                    fontFamily: 'JetBrainsMono',
                    fontSize: 11,
                    color: Color(0xFF78716C),
                  ),
                ),
              ],
            ),
          ),
        ],
      ),
    );
  }

  Widget _buildRankBadge(int rank) {
    if (rank == 1) {
      return Container(
        padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 3),
        decoration: BoxDecoration(
          color: const Color(0xFFFEF08A),
          borderRadius: BorderRadius.circular(6),
          border: Border.all(color: const Color(0xFFEAB308), width: 1.2),
        ),
        child: const Row(
          mainAxisSize: MainAxisSize.min,
          children: [
            Icon(Icons.emoji_events, size: 14, color: Color(0xFF854D0E)),
            SizedBox(width: 4),
            Text(
              '#1',
              style: TextStyle(
                fontFamily: 'JetBrainsMono',
                fontSize: 11,
                fontWeight: FontWeight.bold,
                color: Color(0xFF854D0E),
              ),
            ),
          ],
        ),
      );
    } else if (rank == 2) {
      return Container(
        padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 3),
        decoration: BoxDecoration(
          color: const Color(0xFFE2E8F0),
          borderRadius: BorderRadius.circular(6),
          border: Border.all(color: const Color(0xFF94A3B8), width: 1.2),
        ),
        child: const Text(
          '#2',
          style: TextStyle(
            fontFamily: 'JetBrainsMono',
            fontSize: 11,
            fontWeight: FontWeight.bold,
            color: Color(0xFF334155),
          ),
        ),
      );
    } else if (rank == 3) {
      return Container(
        padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 3),
        decoration: BoxDecoration(
          color: const Color(0xFFFFEDD5),
          borderRadius: BorderRadius.circular(6),
          border: Border.all(color: const Color(0xFFFB923C), width: 1.2),
        ),
        child: const Text(
          '#3',
          style: TextStyle(
            fontFamily: 'JetBrainsMono',
            fontSize: 11,
            fontWeight: FontWeight.bold,
            color: Color(0xFF9A3412),
          ),
        ),
      );
    }

    return Text(
      '#$rank',
      style: const TextStyle(
        fontFamily: 'JetBrainsMono',
        fontSize: 12,
        fontWeight: FontWeight.bold,
        color: Color(0xFF78716C),
      ),
    );
  }

  Widget _buildStatCard({
    required String title,
    required String value,
    required String subtitle,
    required IconData icon,
    required Color bg,
    required Color border,
    required Color textColor,
  }) {
    return Expanded(
      child: Container(
        padding: const EdgeInsets.all(18),
        decoration: BoxDecoration(
          color: bg,
          borderRadius: BorderRadius.circular(10),
          border: Border.all(color: border, width: 1.2),
        ),
        child: Row(
          children: [
            Container(
              padding: const EdgeInsets.all(10),
              decoration: BoxDecoration(
                color: Colors.white.withValues(alpha: 0.8),
                shape: BoxShape.circle,
              ),
              child: Icon(icon, size: 22, color: textColor),
            ),
            const SizedBox(width: 14),
            Expanded(
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Text(
                    title,
                    style: TextStyle(
                      fontFamily: 'JetBrainsMono',
                      fontSize: 10,
                      fontWeight: FontWeight.bold,
                      color: textColor.withValues(alpha: 0.85),
                      letterSpacing: 0.5,
                    ),
                  ),
                  const SizedBox(height: 4),
                  Text(
                    value,
                    style: TextStyle(
                      fontFamily: 'SpaceGrotesk',
                      fontSize: 20,
                      fontWeight: FontWeight.bold,
                      color: textColor,
                    ),
                  ),
                  const SizedBox(height: 2),
                  Text(
                    subtitle,
                    style: TextStyle(
                      fontFamily: 'JetBrainsMono',
                      fontSize: 10,
                      color: textColor.withValues(alpha: 0.7),
                    ),
                    overflow: TextOverflow.ellipsis,
                  ),
                ],
              ),
            ),
          ],
        ),
      ),
    );
  }

  Widget _buildBottomOperativeBar() {
    return ValueListenableBuilder<UserModel?>(
      valueListenable: AuthService.currentUserNotifier,
      builder: (context, user, _) {
        if (user == null) {
          return Container(
            padding: const EdgeInsets.symmetric(horizontal: 24, vertical: 12),
            decoration: const BoxDecoration(
              color: Color(0xFFFEF3C7),
              border: Border(top: BorderSide(color: Color(0xFFFCD34D), width: 1.5)),
            ),
            child: Row(
              mainAxisAlignment: MainAxisAlignment.spaceBetween,
              children: [
                const Row(
                  children: [
                    Icon(Icons.info_outline, size: 18, color: Color(0xFFB45309)),
                    SizedBox(width: 10),
                    Text(
                      'Operative session not active. Register or sign in to record your challenge flags on the live CTF scoreboard.',
                      style: TextStyle(
                        fontFamily: 'JetBrainsMono',
                        fontSize: 11,
                        color: Color(0xFF92400E),
                      ),
                    ),
                  ],
                ),
                ElevatedButton(
                  style: ElevatedButton.styleFrom(
                    backgroundColor: const Color(0xFFD97706),
                    foregroundColor: Colors.white,
                    padding: const EdgeInsets.symmetric(horizontal: 14, vertical: 8),
                    shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(6)),
                    elevation: 0,
                  ),
                  onPressed: () => AuthDialog.show(context),
                  child: const Text(
                    'SIGN IN NOW',
                    style: TextStyle(
                      fontFamily: 'JetBrainsMono',
                      fontSize: 11,
                      fontWeight: FontWeight.bold,
                    ),
                  ),
                ),
              ],
            ),
          );
        }

        // Find my entry
        ScoreboardEntry? myEntry;
        for (final e in _entries) {
          if (e.userId == user.id) {
            myEntry = e;
            break;
          }
        }

        return Container(
          padding: const EdgeInsets.symmetric(horizontal: 24, vertical: 12),
          decoration: const BoxDecoration(
            color: Color(0xFFCCFBF1),
            border: Border(top: BorderSide(color: Color(0xFF5EEAD4), width: 1.5)),
          ),
          child: Row(
            mainAxisAlignment: MainAxisAlignment.spaceBetween,
            children: [
              Row(
                children: [
                  Container(
                    width: 24,
                    height: 24,
                    decoration: BoxDecoration(
                      shape: BoxShape.circle,
                      border: Border.all(color: const Color(0xFF0F766E), width: 1.5),
                    ),
                    child: ClipOval(
                      child: AvatarRegistry.buildAvatarImage(
                        user.avatarId,
                        width: 24,
                        height: 24,
                      ),
                    ),
                  ),
                  const SizedBox(width: 10),
                  Text(
                    'OPERATIVE: ${user.username}',
                    style: const TextStyle(
                      fontFamily: 'SpaceGrotesk',
                      fontSize: 13,
                      fontWeight: FontWeight.bold,
                      color: Color(0xFF0F766E),
                    ),
                  ),
                  const SizedBox(width: 16),
                  Text(
                    myEntry != null
                        ? 'CURRENT RANK: #${myEntry.rank} • ${myEntry.totalPoints} PTS • ${myEntry.labsSolved} SOLVED'
                        : 'CURRENT RANK: UNRANKED (Solve challenges to appear!)',
                    style: const TextStyle(
                      fontFamily: 'JetBrainsMono',
                      fontSize: 11,
                      fontWeight: FontWeight.bold,
                      color: Color(0xFF115E59),
                    ),
                  ),
                ],
              ),
              OutlinedButton.icon(
                style: OutlinedButton.styleFrom(
                  foregroundColor: const Color(0xFF0F766E),
                  side: const BorderSide(color: Color(0xFF0F766E)),
                  padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 6),
                  shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(6)),
                ),
                onPressed: () => AuthDialog.show(context),
                icon: const Icon(Icons.badge_outlined, size: 14),
                label: const Text(
                  'MANAGE PROFILE',
                  style: TextStyle(
                    fontFamily: 'JetBrainsMono',
                    fontSize: 11,
                    fontWeight: FontWeight.bold,
                  ),
                ),
              ),
            ],
          ),
        );
      },
    );
  }

  Widget _buildLoadingState() {
    return const Center(
      child: Column(
        mainAxisSize: MainAxisSize.min,
        children: [
          SizedBox(
            width: 32,
            height: 32,
            child: CircularProgressIndicator(
              strokeWidth: 2.5,
              valueColor: AlwaysStoppedAnimation<Color>(Color(0xFF0D9488)),
            ),
          ),
          SizedBox(height: 16),
          Text(
            'SYNCHRONIZING WITH TURSO CLOUD...',
            style: TextStyle(
              fontFamily: 'JetBrainsMono',
              fontSize: 12,
              fontWeight: FontWeight.bold,
              color: Color(0xFF78716C),
              letterSpacing: 0.5,
            ),
          ),
        ],
      ),
    );
  }

  Widget _buildErrorState() {
    return Center(
      child: Container(
        constraints: const BoxConstraints(maxWidth: 480),
        padding: const EdgeInsets.all(24),
        decoration: BoxDecoration(
          color: ZiteraColors.card,
          borderRadius: BorderRadius.circular(12),
          border: Border.all(color: const Color(0xFFFCA5A5), width: 1.5),
        ),
        child: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            const Icon(Icons.cloud_off, size: 44, color: Color(0xFFDC2626)),
            const SizedBox(height: 14),
            const Text(
              'TURSO CLOUD SYNC ERROR',
              style: TextStyle(
                fontFamily: 'SpaceGrotesk',
                fontSize: 16,
                fontWeight: FontWeight.bold,
                color: Color(0xFF991B1B),
              ),
            ),
            const SizedBox(height: 8),
            Text(
              _error ?? 'Unable to reach Turso database.',
              textAlign: TextAlign.center,
              style: const TextStyle(
                fontFamily: 'JetBrainsMono',
                fontSize: 11,
                color: Color(0xFF7F1D1D),
              ),
            ),
            const SizedBox(height: 18),
            ElevatedButton.icon(
              style: ElevatedButton.styleFrom(
                backgroundColor: const Color(0xFF0D9488),
                foregroundColor: Colors.white,
                shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(8)),
              ),
              onPressed: () => _fetchScoreboard(),
              icon: const Icon(Icons.refresh, size: 16),
              label: const Text(
                'RETRY SYNC',
                style: TextStyle(
                  fontFamily: 'JetBrainsMono',
                  fontSize: 12,
                  fontWeight: FontWeight.bold,
                ),
              ),
            ),
          ],
        ),
      ),
    );
  }
}

class _LivePill extends StatelessWidget {
  const _LivePill();

  @override
  Widget build(BuildContext context) {
    return Container(
      padding: const EdgeInsets.symmetric(horizontal: 6, vertical: 2),
      decoration: BoxDecoration(
        color: const Color(0xFFDCFCE7),
        borderRadius: BorderRadius.circular(4),
        border: Border.all(color: const Color(0xFF86EFAC)),
      ),
      child: const Row(
        mainAxisSize: MainAxisSize.min,
        children: [
          Icon(Icons.circle, size: 7, color: Color(0xFF16A34A)),
          SizedBox(width: 4),
          Text(
            'LIVE',
            style: TextStyle(
              fontFamily: 'JetBrainsMono',
              fontSize: 9,
              fontWeight: FontWeight.bold,
              color: Color(0xFF166534),
            ),
          ),
        ],
      ),
    );
  }
}
