# CTF Challenge: The Broken Hash Vault

### MISSION BRIEF
You are auditing an internal cryptographic key management service running at `http://127.0.0.1:8014`. Legacy infrastructure logs suggest the organization migrated old user accounts but continues to store credentials using obsolete hashing schemes without cryptographic salts.

### OBJECTIVE
Recover the administrator's credentials from the exposed hash audit log, authenticate to the administrative vault, and retrieve the master challenge flag.

### TARGET ENVIRONMENT
- **Base URL:** `http://127.0.0.1:8014`
- **Scope:** Host local-only `127.0.0.1:8014`.

### SUBMISSION FORMAT
The flag follows the standard ZITERA format:
`ZITERA{...}`
