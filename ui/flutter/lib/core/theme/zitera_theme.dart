import 'package:flutter/material.dart';
import 'zitera_colors.dart';

class ZiteraTheme {
  static ThemeData get lightTheme {
    return ThemeData(
      brightness: Brightness.light,
      scaffoldBackgroundColor: ZiteraColors.background,
      primaryColor: ZiteraColors.primary,
      cardColor: ZiteraColors.card,
      dividerColor: ZiteraColors.border,
      fontFamily: 'JetBrainsMono',
      colorScheme: const ColorScheme.light(
        primary: ZiteraColors.primary,
        secondary: ZiteraColors.cyan,
        surface: ZiteraColors.surface,
        error: ZiteraColors.error,
      ),
      appBarTheme: const AppBarTheme(
        backgroundColor: ZiteraColors.background,
        elevation: 0,
        centerTitle: false,
        titleTextStyle: TextStyle(
          color: ZiteraColors.textPrimary,
          fontSize: 16,
          fontWeight: FontWeight.bold,
          letterSpacing: 1.2,
          fontFamily: 'Silkscreen',
        ),
      ),
    );
  }

  static ThemeData get darkTheme => lightTheme;
}
