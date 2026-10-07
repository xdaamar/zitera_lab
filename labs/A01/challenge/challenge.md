# CTF Challenge: The Master Invoice Leak

### MISSION
You are reviewing a company's internal accounting and billing portal. Security intelligence indicates that an administrator created a hidden master billing record containing the system activation flag.

### OBJECTIVE
Exploit an access control flaw in the target billing system running at `http://127.0.0.1:8011` to uncover the secret master invoice and retrieve the challenge flag.

### TARGET ENVIRONMENT
- Base URL: `http://127.0.0.1:8011`
- Initial Credentials: `alice` / `password123`

### CONSTRAINTS
- Attack ONLY `127.0.0.1:8011`.
- Do not attempt brute-force attacks against system passwords. The vulnerability lies in access control parameters.

### SUBMISSION FORMAT
The flag follows the standard format:
`ZITERA{...}`
