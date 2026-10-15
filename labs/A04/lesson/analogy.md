# Real-World Analogy: Cheap Luggage Padlocks vs Bank Vaults

To intuitively understand the gap between broken legacy cryptography and modern defensive standards, consider how one might protect a billion-dollar proprietary manufacturing formula:

---

### 1. Plaintext Storage (Sticky Note on the Lobby Desk)
Persisting credentials or secrets in cleartext is equivalent to scribbling the formula on a notepad and leaving it on an unattended reception counter. Every visitor walking through the building can effortlessly read, photograph, or copy the data without triggering a single alarm.

---

### 2. Unsalted MD5 Hashing (Cheap 3-Dial Plastic Luggage Padlock)
To protect the secret, you place the document in a lightweight suitcase locked with a 3-wheel plastic rotary lock:
- **Trivial Search Space**: With only 1,000 possible combinations (000 through 999), an observer sitting in the terminal can test every single sequence within three minutes.
- **Missing Salt (Identical Factory Master Keys)**: The manufacturer ships every unit with identical internal tumblers. Adversaries do not even need to manipulate the dials; they simply consult the public factory manual to open the latch immediately.

This mirrors fast legacy hashing: because MD5 without salt is deterministic and computationally lightweight, modern processors can search billions of candidate passwords per second or instantly match hashes using public rainbow lookup databases.

---

### 3. Modern Salted, Memory-Hard Storage (Reinforced Bank Vault)
Now consider securing the blueprint inside a state-of-the-art bank vault:
- **Cryptographic Salt (Unique Tumbler Castings)**: Each vault chamber is cast with a completely distinct, unrepeatable mechanical keyway. Master key catalogs provide zero utility.
- **Argon2 / Work Factor (Weighted Hydraulic Delay)**: When an authorized operator enters a combination, internal hydraulic pistons deliberately require 300 milliseconds of deliberate mechanical rotation to engage.
- **Economic Impracticability**: Even if an automated robotic arm attempts to brute-force millions of candidate combinations, the deliberate mechanical delay renders exhaustive search computationally and economically unfeasible across human lifetimes.

---

### Core Security Takeaway
Never assume that transformed data is secure simply because it appears as an obscure alphanumeric string. If that string was produced by a high-speed, unsalted legacy hash, the protection is merely an illusion waiting to collapse.
