/// Centralized application configuration for ZITERA_LAB.
/// 
/// Manages open-source project links, support URLs, and external resources.
/// Follows strict offline-first and defensive URI security constraints.
class AppConfig {
  /// Dedicated configuration point for open-source project donation/support.
  /// 
  /// Default: null (CTA hidden in UI).
  /// Status: DONATION_URL_REQUIRED (awaiting authoritative owner link).
  /// 
  /// Hard Rule: Do not populate with fictitious or third-party placeholder URLs.
  /// When configured by the repository owner, set to a valid HTTPS URL
  /// (e.g. GitHub Sponsors, Ko-fi, Saweria, or OpenCollective).
  static const String? donationUrl = null;

  /// Official Project Repository URL
  static const String repositoryUrl = 'https://github.com/xdaamar/zitera_lab';

  /// Validates whether a candidate URL conforms to strict HTTPS URI safety constraints.
  static bool isValidDonationUrl(String? url) {
    if (url == null || url.trim().isEmpty) return false;
    final uri = Uri.tryParse(url.trim());
    if (uri == null) return false;
    // Strict security policy: Only HTTPS scheme is permitted; reject localhost, file, and non-web schemes
    return uri.isScheme('https') && uri.host.isNotEmpty && !uri.host.contains('localhost') && uri.host != '127.0.0.1';
  }
}
