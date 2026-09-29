# CTF Challenge: Administrative Gateway Brute-Force

## Scenario
The OmniAuth Control Gateway exposes an administrative login interface at `http://127.0.0.1:8017/login`. The developer failed to implement login rate limiting, CAPTCHA, or temporary account lockouts.

Additionally, the administrator chose a simple 4-digit PIN based on the current release year.

## Objective
1. Inspect the login portal running at `http://127.0.0.1:8017`.
2. Discover the lack of brute-force mitigation mechanisms.
3. Test or brute-force the 4-digit PIN for user `admin`.
4. Gain access to `/admin` to retrieve the CTF flag.
