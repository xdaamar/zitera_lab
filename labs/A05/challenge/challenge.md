# CTF Challenge: The Hidden Vault Extraction

### MISSION
The military hardware warehouse portal at `http://127.0.0.1:8015` allows employees to query item inventories. Internal security audits suggest that classified asset records containing a master project flag are stored in a private database table named `vault_secrets`.

### OBJECTIVE
Perform a SQL Injection attack (e.g. UNION-based injection) to extract the secret key flag stored in the database.

### TARGET ENVIRONMENT
- **Base URL:** `http://127.0.0.1:8015`
- **Vulnerable Surface:** Product Search query input

### SUBMISSION FORMAT
The flag follows the standard format:
`ZITERA{...}`
