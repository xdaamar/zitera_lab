# Core Concepts of Authentication Attacks

## Attack Methodologies

1. **Brute-Force Attacks**: Systematically checking all possible combinations (e.g. 4-digit PINs `0000` to `9999` takes only 10,000 attempts, which takes less than 30 seconds over a fast local network).
2. **Credential Stuffing**: Using automated botnets to test billions of username/password pairs leaked from past third-party data breaches.
3. **Session Fixation**: Tricking an unsuspecting user into authenticating using a session identifier established by the attacker.
4. **Weak Password Reset**: Insecure security questions or easily guessable reset tokens sent via unauthenticated channels.
