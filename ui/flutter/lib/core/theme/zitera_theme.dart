import 'package:flutter/material.dart';
import 'zitera_colors.dart';

class ZiteraTheme {
  static ThemeData get darkTheme {
    return ThemeData(
      brightness: Brightness.dark,
      scaffoldBackgroundColor: ZiteraColors.background,
      primaryColor: ZiteraColors.primary,
      cardColor: ZiteraColors.card,
      dividerColor: ZiteraColors.border,
      fontFamily: 'Segoe UI',
      colorScheme: const ColorScheme.dark(
        primary: ZiteraColors.primary,
        secondary: ZiteraColors.cyan,
        surface: ZiteraColors.surface,
        error: ZiteraColors.error,
      ),
      appBarTheme: const AppBarTheme(
        backgroundColor: ZiteraColors.surface,
        elevation: 0,
        centerTitle: false,
        titleTextStyle: TextStyle(
          color: ZiteraColors.textPrimary,
          fontSize: 16,
          fontWeight: FontWeight.bold,
          letterSpacing: 1.2,
        ),
      ),
    );
  }
}
