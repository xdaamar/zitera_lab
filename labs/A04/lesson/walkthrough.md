# Practice Walkthrough: Cracking Unsalted Hashes

### Objective
In this exercise, you will investigate how obsolete unsalted password hashing allows attackers to recover administrator credentials from the CryptoVault service at `http://127.0.0.1:8014`.

### Step 1: Discovering the Audit Hash Log
- Open your browser to `http://127.0.0.1:8014`.
- Notice the link to `/api/audit/hashes`.
- Open `http://127.0.0.1:8014/api/audit/hashes` in a new tab.
- **Observation:** The endpoint returns JSON containing the user table with stored hashes:
  - `admin`: `21232f297a57a5a743894a0e4a801fc3` (Type: MD5 Unsalted)

### Step 2: Analyzing the Hash Format
- Notice the hash length: 32 hexadecimal characters (128 bits).
- A 32-character hexadecimal digest without salt prefix strongly indicates MD5.

### Step 3: Reversing the MD5 Digest
- Because MD5 has no salt and is deterministic, it has been precomputed in public rainbow tables.
- Query a public hash lookup or test common passwords:
  - `echo -n "admin" | md5sum` -> `21232f297a57a5a743894a0e4a801fc3`
  - Match confirmed! The plaintext password for `admin` is `admin`.

### Step 4: Gaining Administrative Access
- Navigate back to `http://127.0.0.1:8014/login`.
- Log in with:
  - Username: `admin`
  - Password: `admin`
- **Result:** You are redirected to `/vault`, exposing the protected cryptographic secret key!
