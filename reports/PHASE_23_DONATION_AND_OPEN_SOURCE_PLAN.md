# PHASE 23: DONATION LINK & OPEN-SOURCE INTEGRATION PLAN
**Document Reference:** `reports/PHASE_23_DONATION_AND_OPEN_SOURCE_PLAN.md`  
**Phase:** 23 (CP05 — Donation Link & Open-Source Preparation)  
**Author:** AI Pair Programmer (Software Craftsmanship Active)  
**Date:** 2026-10-12  
**Status:** COMPLETE (STATUS: DONATION_URL_REQUIRED)

---

## 1. Executive Summary

This document establishes the open-source sponsorship and donation integration strategy for **ZITERA_LAB**. The integration strictly adheres to project integrity rules:
1. **Zero Fictitious URLs:** No fake, temporary, or third-party placeholder donation URLs have been introduced.
2. **Offline-First & Distraction-Free:** No popups, persistent banners, startup requests, telemetry, or donation modals will ever interrupt learner laboratory workflows.
3. **Centralized Configuration:** All external links are managed from a single source of truth (`ui/flutter/lib/core/config/app_config.dart`) defaulting to `null`, ensuring call-to-actions (CTAs) remain completely hidden until the owner explicitly provides a verified URL.

---

## 2. Repository Audit: Existing Links & Licenses

| Category | Inspected Asset | Audit Finding | Status |
|---|---|---|---|
| **Open-Source License** | `LICENSE` (Repo Root) | Standard MIT License (Copyright (c) 2026 ZITERA_LAB Contributors). Permissive, commercial-friendly, standard open-source license. | **VERIFIED (MIT)** |
| **Project Repository** | `README.md` & `Runner.rc` | GitHub repository URL: `https://github.com/xdaamar/zitera_lab` | **VERIFIED** |
| **Sponsorship Config** | `.github/FUNDING.yml` | Not present in repository. | **ABSENT** |
| **Existing Donation URL** | Codebase & Docs | Zero donation links (Ko-fi, GitHub Sponsors, Saweria, PayPal, Patreon) exist in source or documentation. | **DONATION_URL_REQUIRED** |

---

## 3. Centralized Configuration Architecture

To prevent scattered hardcoded URLs, configuration has been implemented in:
`ui/flutter/lib/core/config/app_config.dart`

```dart
class AppConfig {
  /// Default: null. Status: DONATION_URL_REQUIRED.
  static const String? donationUrl = null;

  /// Official Project Repository URL
  static const String repositoryUrl = 'https://github.com/xdaamar/zitera_lab';

  /// Validates whether a candidate URL conforms to strict HTTPS URI safety constraints.
  static bool isValidDonationUrl(String? url) {
    if (url == null || url.trim().isEmpty) return false;
    final uri = Uri.tryParse(url.trim());
    if (uri == null) return false;
    return uri.isScheme('https') && uri.host.isNotEmpty && !uri.host.contains('localhost') && uri.host != '127.0.0.1';
  }
}
```

### Safety & Invariant Guarantees:
- **Conditional Visibility:** UI components check `if (AppConfig.isValidDonationUrl(AppConfig.donationUrl))`. Because the default is `null`, zero donation buttons or broken links are shown to users today.
- **URI Scheme Enforcement:** Only strict `https://` URLs are allowed. Schemes like `http://`, `file://`, `javascript:`, or local port loopbacks (`127.0.0.1:8011..8020`) are rejected.
- **Broker Isolation:** External support links are opened in the host operating system's default browser via desktop OS shell launch; they are **never** proxied through the native lab broker or AppContainer sandbox.
- **Zero Startup Network Traffic:** The application generates zero outbound HTTP network requests during launch or idle execution.

---

## 4. Proposed UI Placement (Understated & Non-Intrusive)

When configured, the support entry point will be placed strictly in secondary informational screens:

1. **Settings / About View (`settings_view.dart`):**
   - An understated tile titled **"Support ZITERA_LAB (Open Source)"** placed alongside version details and diagnostic export.
   - Text: *"ZITERA_LAB is a free, offline-first cybersecurity training platform. Consider supporting ongoing lab development."*
   - Button: *"Sponsor Project (External Link)"*.
2. **README.md (Open-Source Contributors):**
   - An optional badge in `README.md` under a `## Sponsorship & Support` section.
3. **Prohibited UI Locations:**
   - NO banners on the Dashboard.
   - NO popups or reminders inside Lab Learn, Practice, or Challenge modes.
   - NO paywalls or feature-gating.

---

## 5. Next Steps for Repository Owner

To activate the donation / sponsor integration:
1. **Provide Authoritative URL:** Choose a platform (e.g. GitHub Sponsors `https://github.com/sponsors/xdaamar`, Ko-fi `https://ko-fi.com/...`, or Saweria `https://saweria.co/...`).
2. **Update AppConfig:** Set `AppConfig.donationUrl = 'https://...'`.
3. **Optional `.github/FUNDING.yml`:** Create `.github/FUNDING.yml` to display the "Sponsor this project" button on the GitHub repository homepage.
