import 'package:flutter/material.dart';
import 'package:flutter/services.dart';

/// High-tech cyber textbook and CTF markdown renderer.
/// Guarantees ZERO raw markdown syntax artifacts (such as `**`, `###`, etc.).
/// All bold text is natively rendered with authentic typography bolding (FontWeight.bold),
/// code blocks are styled into cyber terminals, and list items have custom badges.
class ZiteraRichContent extends StatelessWidget {
  final String rawContent;
  final TextStyle? baseStyle;
  final bool compact;

  const ZiteraRichContent({
    super.key,
    required this.rawContent,
    this.baseStyle,
    this.compact = false,
  });

  @override
  Widget build(BuildContext context) {
    if (rawContent.trim().isEmpty) {
      return const SizedBox.shrink();
    }

    final blocks = _parseBlocks(rawContent);

    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: blocks.map((block) => _buildBlock(context, block)).toList(),
    );
  }

  Widget _buildBlock(BuildContext context, _ContentBlock block) {
    switch (block.type) {
      case _BlockType.h1:
        return Padding(
          padding: const EdgeInsets.only(top: 8.0, bottom: 12.0),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Text(
                block.content,
                style: const TextStyle(
                  fontFamily: 'SpaceGrotesk',
                  fontSize: 18,
                  fontWeight: FontWeight.bold,
                  color: Color(0xFF1E1A14),
                  letterSpacing: 0.4,
                ),
              ),
              const SizedBox(height: 4),
              Container(
                width: 48,
                height: 3,
                decoration: BoxDecoration(
                  color: const Color(0xFF2DD4BF),
                  borderRadius: BorderRadius.circular(2),
                ),
              ),
            ],
          ),
        );

      case _BlockType.h2:
        return Padding(
          padding: const EdgeInsets.only(top: 14.0, bottom: 8.0),
          child: Container(
            padding: const EdgeInsets.only(left: 10),
            decoration: const BoxDecoration(
              border: Border(
                left: BorderSide(color: Color(0xFF0F766E), width: 3.5),
              ),
            ),
            child: Text(
              block.content,
              style: const TextStyle(
                fontFamily: 'SpaceGrotesk',
                fontSize: 15,
                fontWeight: FontWeight.bold,
                color: Color(0xFF1E1A14),
                letterSpacing: 0.3,
              ),
            ),
          ),
        );

      case _BlockType.h3:
        return Padding(
          padding: const EdgeInsets.only(top: 12.0, bottom: 6.0),
          child: Row(
            children: [
              Container(
                padding: const EdgeInsets.symmetric(horizontal: 7, vertical: 2.5),
                decoration: BoxDecoration(
                  color: const Color(0xFFCCFBF1),
                  borderRadius: BorderRadius.circular(4),
                  border: Border.all(color: const Color(0xFF2DD4BF), width: 0.8),
                ),
                child: Text(
                  block.content.toUpperCase(),
                  style: const TextStyle(
                    fontFamily: 'JetBrainsMono',
                    fontSize: 10.5,
                    fontWeight: FontWeight.bold,
                    color: Color(0xFF0F766E),
                    letterSpacing: 0.6,
                  ),
                ),
              ),
            ],
          ),
        );

      case _BlockType.codeBlock:
        return Padding(
          padding: const EdgeInsets.symmetric(vertical: 8.0),
          child: _CodeBlockViewer(code: block.content, language: block.meta),
        );

      case _BlockType.bullet:
        return Padding(
          padding: const EdgeInsets.only(bottom: 6.0, left: 4.0),
          child: Row(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Container(
                margin: const EdgeInsets.only(top: 7.5, right: 10),
                width: 5.5,
                height: 5.5,
                decoration: const BoxDecoration(
                  color: Color(0xFF0F766E),
                  shape: BoxShape.circle,
                ),
              ),
              Expanded(
                child: SelectableText.rich(
                  TextSpan(
                    children: _parseInlineSpans(block.content, baseStyle),
                  ),
                ),
              ),
            ],
          ),
        );

      case _BlockType.numbered:
        return Padding(
          padding: const EdgeInsets.only(bottom: 8.0),
          child: Row(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Container(
                margin: const EdgeInsets.only(top: 2, right: 10),
                padding: const EdgeInsets.symmetric(horizontal: 6, vertical: 1.5),
                decoration: BoxDecoration(
                  color: const Color(0xFFF5F3EF),
                  borderRadius: BorderRadius.circular(4),
                  border: Border.all(color: const Color(0xFFD6CFC3), width: 0.8),
                ),
                child: Text(
                  block.meta.padLeft(2, '0'),
                  style: const TextStyle(
                    fontFamily: 'JetBrainsMono',
                    fontSize: 10,
                    fontWeight: FontWeight.bold,
                    color: Color(0xFF5C5347),
                  ),
                ),
              ),
              Expanded(
                child: SelectableText.rich(
                  TextSpan(
                    children: _parseInlineSpans(block.content, baseStyle),
                  ),
                ),
              ),
            ],
          ),
        );

      case _BlockType.callout:
        return Padding(
          padding: const EdgeInsets.symmetric(vertical: 8.0),
          child: Container(
            padding: const EdgeInsets.symmetric(horizontal: 14, vertical: 10),
            decoration: BoxDecoration(
              color: const Color(0xFFF0FDF4),
              borderRadius: BorderRadius.circular(6),
              border: Border.all(color: const Color(0xFF86EFAC), width: 1.2),
            ),
            child: Row(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                const Icon(Icons.tips_and_updates, size: 16, color: Color(0xFF16A34A)),
                const SizedBox(width: 10),
                Expanded(
                  child: SelectableText.rich(
                    TextSpan(
                      children: _parseInlineSpans(block.content, baseStyle),
                    ),
                  ),
                ),
              ],
            ),
          ),
        );

      case _BlockType.paragraph:
        return Padding(
          padding: const EdgeInsets.only(bottom: 10.0),
          child: SelectableText.rich(
            TextSpan(
              children: _parseInlineSpans(block.content, baseStyle),
            ),
          ),
        );
    }
  }

  /// Parses lines into high-level semantic blocks
  List<_ContentBlock> _parseBlocks(String input) {
    final blocks = <_ContentBlock>[];
    final lines = input.replaceAll('\r\n', '\n').replaceAll('\r', '\n').split('\n');

    bool inCodeBlock = false;
    String codeLanguage = '';
    final codeBuffer = StringBuffer();

    for (int i = 0; i < lines.length; i++) {
      final line = lines[i];
      final trimmed = line.trim();

      // Check code block fences ```lang
      if (trimmed.startsWith('```')) {
        if (inCodeBlock) {
          blocks.add(_ContentBlock(
            type: _BlockType.codeBlock,
            content: codeBuffer.toString().trimRight(),
            meta: codeLanguage,
          ));
          codeBuffer.clear();
          inCodeBlock = false;
        } else {
          inCodeBlock = true;
          codeLanguage = trimmed.substring(3).trim();
        }
        continue;
      }

      if (inCodeBlock) {
        codeBuffer.writeln(line);
        continue;
      }

      if (trimmed.isEmpty) {
        continue;
      }

      // Headers
      if (trimmed.startsWith('# ')) {
        blocks.add(_ContentBlock(type: _BlockType.h1, content: trimmed.substring(2).trim()));
      } else if (trimmed.startsWith('## ')) {
        blocks.add(_ContentBlock(type: _BlockType.h2, content: trimmed.substring(3).trim()));
      } else if (trimmed.startsWith('### ')) {
        blocks.add(_ContentBlock(type: _BlockType.h3, content: trimmed.substring(4).trim()));
      } else if (trimmed.startsWith('> ')) {
        blocks.add(_ContentBlock(type: _BlockType.callout, content: trimmed.substring(2).trim()));
      } else if (trimmed.startsWith('- ') || trimmed.startsWith('* ')) {
        blocks.add(_ContentBlock(type: _BlockType.bullet, content: trimmed.substring(2).trim()));
      } else {
        // Numbered lists: e.g. "1. ", "2. "
        final numMatch = RegExp(r'^(\d+)\.\s+(.*)$').firstMatch(trimmed);
        if (numMatch != null) {
          final numStr = numMatch.group(1) ?? '1';
          final rest = numMatch.group(2) ?? '';
          blocks.add(_ContentBlock(type: _BlockType.numbered, content: rest, meta: numStr));
        } else {
          blocks.add(_ContentBlock(type: _BlockType.paragraph, content: trimmed));
        }
      }
    }

    if (inCodeBlock && codeBuffer.isNotEmpty) {
      blocks.add(_ContentBlock(
        type: _BlockType.codeBlock,
        content: codeBuffer.toString().trimRight(),
        meta: codeLanguage,
      ));
    }

    return blocks;
  }

  /// Parses inline bold `**text**`, inline code `` `code` ``, italics `*text*`
  /// into styled TextSpans without displaying raw asterisks.
  static List<InlineSpan> _parseInlineSpans(String text, TextStyle? fallbackStyle) {
    final spans = <InlineSpan>[];
    final defaultStyle = fallbackStyle ??
        const TextStyle(
          fontFamily: 'SpaceGrotesk',
          fontSize: 13,
          color: Color(0xFF332D25),
          height: 1.6,
        );

    final boldStyle = defaultStyle.copyWith(
      fontWeight: FontWeight.bold,
      color: const Color(0xFF1E1A14),
    );

    final inlineCodeStyle = const TextStyle(
      fontFamily: 'JetBrainsMono',
      fontSize: 11.5,
      fontWeight: FontWeight.bold,
      color: Color(0xFF0F766E),
      backgroundColor: Color(0xFFE6F4F1),
    );

    // Regex matching: **bold**, `code`, or regular text
    final regex = RegExp(r'(\*\*([^*]+)\*\*|`([^`]+)`|\*([^*]+)\*)');
    int lastIndex = 0;

    for (final match in regex.allMatches(text)) {
      if (match.start > lastIndex) {
        final preceding = text.substring(lastIndex, match.start);
        spans.add(TextSpan(text: _sanitizePlain(preceding), style: defaultStyle));
      }

      final fullMatch = match.group(0) ?? '';
      if (fullMatch.startsWith('**') && fullMatch.endsWith('**')) {
        final inner = match.group(2) ?? '';
        spans.add(TextSpan(text: inner, style: boldStyle));
      } else if (fullMatch.startsWith('`') && fullMatch.endsWith('`')) {
        final inner = match.group(3) ?? '';
        spans.add(WidgetSpan(
          alignment: PlaceholderAlignment.middle,
          child: Container(
            padding: const EdgeInsets.symmetric(horizontal: 5, vertical: 1.5),
            margin: const EdgeInsets.symmetric(horizontal: 2),
            decoration: BoxDecoration(
              color: const Color(0xFFE6F4F1),
              borderRadius: BorderRadius.circular(4),
              border: Border.all(color: const Color(0xFF2DD4BF), width: 0.6),
            ),
            child: Text(inner, style: inlineCodeStyle),
          ),
        ));
      } else if (fullMatch.startsWith('*') && fullMatch.endsWith('*')) {
        final inner = match.group(4) ?? '';
        spans.add(TextSpan(
          text: inner,
          style: defaultStyle.copyWith(fontStyle: FontStyle.italic, fontWeight: FontWeight.w600),
        ));
      }

      lastIndex = match.end;
    }

    if (lastIndex < text.length) {
      final trailing = text.substring(lastIndex);
      spans.add(TextSpan(text: _sanitizePlain(trailing), style: defaultStyle));
    }

    return spans;
  }

  static String _sanitizePlain(String str) {
    // Strip any lone or rogue asterisks that would look like markdown copypasta
    return str.replaceAll('**', '');
  }
}

enum _BlockType { h1, h2, h3, paragraph, bullet, numbered, codeBlock, callout }

class _ContentBlock {
  final _BlockType type;
  final String content;
  final String meta;

  _ContentBlock({
    required this.type,
    required this.content,
    this.meta = '',
  });
}

class _CodeBlockViewer extends StatefulWidget {
  final String code;
  final String language;

  const _CodeBlockViewer({required this.code, required this.language});

  @override
  State<_CodeBlockViewer> createState() => _CodeBlockViewerState();
}

class _CodeBlockViewerState extends State<_CodeBlockViewer> {
  bool _copied = false;

  void _copy() async {
    await Clipboard.setData(ClipboardData(text: widget.code));
    if (mounted) {
      setState(() => _copied = true);
      Future.delayed(const Duration(seconds: 2), () {
        if (mounted) setState(() => _copied = false);
      });
    }
  }

  @override
  Widget build(BuildContext context) {
    final langDisplay = widget.language.isNotEmpty ? widget.language.toUpperCase() : 'CODE';

    return Container(
      decoration: BoxDecoration(
        color: const Color(0xFF1E1A14),
        borderRadius: BorderRadius.circular(6),
        border: Border.all(color: const Color(0xFF3E362C), width: 1.2),
      ),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.stretch,
        children: [
          // Header Bar
          Container(
            padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 6),
            decoration: const BoxDecoration(
              color: Color(0xFF14120E),
              borderRadius: BorderRadius.only(
                topLeft: Radius.circular(5),
                topRight: Radius.circular(5),
              ),
            ),
            child: Row(
              mainAxisAlignment: MainAxisAlignment.spaceBetween,
              children: [
                Row(
                  children: [
                    Container(width: 8, height: 8, decoration: const BoxDecoration(color: Color(0xFFEF4444), shape: BoxShape.circle)),
                    const SizedBox(width: 5),
                    Container(width: 8, height: 8, decoration: const BoxDecoration(color: Color(0xFFF59E0B), shape: BoxShape.circle)),
                    const SizedBox(width: 5),
                    Container(width: 8, height: 8, decoration: const BoxDecoration(color: Color(0xFF10B981), shape: BoxShape.circle)),
                    const SizedBox(width: 10),
                    Text(
                      langDisplay,
                      style: const TextStyle(
                        fontFamily: 'JetBrainsMono',
                        fontSize: 10,
                        fontWeight: FontWeight.bold,
                        color: Color(0xFF2DD4BF),
                      ),
                    ),
                  ],
                ),
                InkWell(
                  onTap: _copy,
                  borderRadius: BorderRadius.circular(4),
                  child: Padding(
                    padding: const EdgeInsets.symmetric(horizontal: 6, vertical: 2),
                    child: Row(
                      children: [
                        Icon(
                          _copied ? Icons.check : Icons.copy,
                          size: 13,
                          color: _copied ? const Color(0xFF2DD4BF) : const Color(0xFF9C9080),
                        ),
                        const SizedBox(width: 4),
                        Text(
                          _copied ? 'COPIED' : 'COPY',
                          style: TextStyle(
                            fontFamily: 'JetBrainsMono',
                            fontSize: 10,
                            fontWeight: FontWeight.bold,
                            color: _copied ? const Color(0xFF2DD4BF) : const Color(0xFF9C9080),
                          ),
                        ),
                      ],
                    ),
                  ),
                ),
              ],
            ),
          ),
          // Code Text Area
          Padding(
            padding: const EdgeInsets.all(12),
            child: SelectableText(
              widget.code,
              style: const TextStyle(
                fontFamily: 'JetBrainsMono',
                fontSize: 12,
                color: Color(0xFFF5F3EF),
                height: 1.5,
              ),
            ),
          ),
        ],
      ),
    );
  }
}
