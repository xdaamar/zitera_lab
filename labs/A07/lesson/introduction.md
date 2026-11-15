# A07: Identification and Authentication Failures

## 1. What are Authentication Failures?
In the OWASP Top 10:2025 standard, **Authentication Failures** (formerly known as *Identification and Authentication Failures*) encompasses defects in how an application confirms digital identity, establishes authenticated user sessions, and defends credential evaluation boundaries against automated exploitation.

Authentication represents the primary perimeter guard of any system: compromises at this layer permit threat actors to masquerade as authentic users, seize high-privilege administrative accounts (*account takeover*), and manipulate sensitive corporate assets without needing to penetrate underlying network firewalls.

## 2. Why Authentication Failures Persist in 2025/2026
Threats targeting digital identity have expanded exponentially driven by low-cost cloud automation and massive credential leak corpuses on the dark web:
- **Absence of Rate-Limiting & Lockout Controls**: Unprotected login endpoints accept thousands of rapid candidate guesses per second without imposing backoff delays, IP throttling, or CAPTCHA verification challenges.
- **Permissive Password Policies**: Allowing low-entropy credentials (such as 4-digit numeric PINs) or default manufacturer credentials that can be searched exhaustively within seconds.
- **Automated Credential Stuffing**: Threat actors deploy botnets to replay billions of username/password pairs harvested from external data breaches against login interfaces, capitalizing on cross-service password reuse.
- **Flawed Session Lifecycles**: Failing to regenerate session tokens upon successful login (*Session Fixation*), omitting strict session expiration timers, or transmitting session identifiers over unencrypted channels.

## 3. Notable Real-World Authentication Incidents
- **Reddit Administrative Breach (2018)**: Threat actors compromised staff employee accounts through targeted credential stuffing, bypassing weak SMS-based two-factor authentication.
- **Zoom Credential Stuffing Surge (2020)**: Over 500,000 user credentials were harvested and circulated across criminal forums because public API endpoints lacked adaptive request-rate throttling.
- **Mirai IoT Botnet Infiltration**: Hundreds of thousands of commercial cameras and routers were weaponized within hours due to unchangeable, hardcoded factory login defaults (`admin:admin`, `root:123456`).

## 4. Learning Objectives in Lab A07
Through the *OmniAuth Admin Gateway* hands-on environment on port 8017, you will:
1. Detect the total absence of rate-limiting and lockout controls across web forms and JSON API handlers.
2. Analyze the mathematical feasibility of exhaustive search against constrained keyspaces (4-digit numeric PINs).
3. Execute targeted automated credential guessing using terminal-based cURL routines to recover authentic administrative PINs.
4. Authenticate into the administrative view and capture the challenge token in standard `ZITERA{...}` format.
5. Implement multi-layered defensive authentication architectures utilizing *Exponential Backoff*, *Account Lockout*, and *Multi-Factor Authentication (MFA)*.
