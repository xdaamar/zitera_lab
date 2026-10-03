import 'package:flutter/material.dart';
import 'zitera_colors.dart';
import '../../widgets/cute_anime_cursor.dart';

class ZiteraTheme {
  static ThemeData get lightTheme {
    return ThemeData(
      brightness: Brightness.light,
      scaffoldBackgroundColor: ZiteraColors.background,
      primaryColor: ZiteraColors.primary,
      cardColor: ZiteraColors.card,
      dividerColor: ZiteraColors.border,
      fontFamily: 'SpaceGrotesk',
      colorScheme: const ColorScheme.light(
        primary: ZiteraColors.primary,
        secondary: ZiteraColors.cyan,
        surface: ZiteraColors.surface,
        error: ZiteraColors.error,
      ),
      iconButtonTheme: IconButtonThemeData(
        style: ButtonStyle(
          mouseCursor: WidgetStateProperty.resolveWith(
            (_) => CuteCursorController.cursor,
          ),
        ),
      ),
      textButtonTheme: TextButtonThemeData(
        style: ButtonStyle(
          mouseCursor: WidgetStateProperty.resolveWith(
            (_) => CuteCursorController.cursor,
          ),
        ),
      ),
      elevatedButtonTheme: ElevatedButtonThemeData(
        style: ButtonStyle(
          mouseCursor: WidgetStateProperty.resolveWith(
            (_) => CuteCursorController.cursor,
          ),
        ),
      ),
      outlinedButtonTheme: OutlinedButtonThemeData(
        style: ButtonStyle(
          mouseCursor: WidgetStateProperty.resolveWith(
            (_) => CuteCursorController.cursor,
          ),
        ),
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
          fontFamily: 'SpaceGrotesk',
        ),
      ),
    );
  }

  static ThemeData get darkTheme => lightTheme;
}
