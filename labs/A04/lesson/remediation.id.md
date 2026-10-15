# Panduan Remediasi: Rekomendasi Standar Kriptografi Industri

Untuk memperbaiki kegagalan kriptografi secara menyeluruh dan mencegah kebocoran data sensitif, terapkan standar industri berikut:

---

## 1. Gunakan Fungsi Hash Kata Sandi Adaptif & Memory-Hard
Jangan pernah menggunakan MD5, SHA-1, atau plain SHA-256 untuk menyimpan kata sandi pengguna. Gunakan algoritma yang sengaja dirancang lambat dan membutuhkan alokasi memori dinamis untuk menggagalkan akselerasi GPU/ASIC:

### Rekomendasi Utama: Argon2id
```python
from argon2 import PasswordHasher

# Inisialisasi hasher dengan parameter keamanan standar OWASP
# time_cost=3, memory_cost=65536 (64 MB), parallelism=4
ph = PasswordHasher(time_cost=3, memory_cost=65536, parallelism=4)

# Membuat hash baru (secara otomatis menghasilkan salt acak unik)
secure_hash = ph.hash("UserSecretPassword123!")

# Memverifikasi kata sandi saat pengguna login
try:
    ph.verify(secure_hash, "UserSecretPassword123!")
    # Verifikasi sukses
except Exception:
    # Verifikasi gagal
    pass
```

### Rekomendasi Alternatif: bcrypt (Cost Factor Minimal 12)
```python
import bcrypt

# Registrasi: Hash kata sandi dengan salt unik 12 rounds
salt = bcrypt.gensalt(rounds=12)
hashed_password = bcrypt.hashpw(raw_password.encode('utf-8'), salt)

# Login: Verifikasi kata sandi
if bcrypt.checkpw(user_input.encode('utf-8'), hashed_password):
    # Autentikasi berhasil
    grant_access()
```

---

## 2. Enkripsi Terautentikasi Saat Istirahat (AEAD Mode)
Ketika mengenkripsi data sensitif (seperti nomor identitas, riwayat medis, atau token API) di dalam basis data:
- Gunakan algoritma **Authenticated Encryption with Associated Data (AEAD)** seperti `AES-256-GCM` atau `ChaCha20-Poly1305`.
- Pastikan setiap pesan menggunakan **Nonce / Initialization Vector (IV)** yang unik dan dihasilkan dari CSPRNG.

```python
import os
from cryptography.hazmat.primitives.ciphers.aead import AESGCM

# Menghasilkan kunci simetris 256-bit (simpan kunci ini di KMS)
key = AESGCM.generate_key(bit_length=256)
aesgcm = AESGCM(key)

# Generate nonce unik 96-bit untuk setiap enkripsi
nonce = os.urandom(12)
plaintext = b"Nomor Rekening Rahasia: 1234-5678-9012"

# Enkripsi menghasilkan ciphertext + tag integritas terpadu
ciphertext = aesgcm.encrypt(nonce, plaintext, None)

# Dekripsi data
decrypted = aesgcm.decrypt(nonce, ciphertext, None)
```

---

## 3. Gunakan Perbandingan Konstan Waktu (Constant-Time Comparison)
Untuk mencegah serangan *timing attack* saat membandingkan token autentikasi, HMAC, atau tanda tangan digital:
```python
import hmac

# JANGAN PERNAH gunakan operator == biasa:
# if user_token == expected_token: ...

# GUNAKAN perbandingan waktu konstan:
if hmac.compare_digest(user_token, expected_token):
    # Autentikasi aman dari analisis durasi CPU
    pass
```

---

## 4. Tegakkan Enkripsi Saat Transit (Transport Layer Security)
- Wajibkan protokol minimal **TLS 1.3** (atau TLS 1.2 sebagai fallback transisi) dan nonaktifkan secara permanen SSLv3, TLS 1.0, dan TLS 1.1.
- Aktifkan header **HTTP Strict Transport Security (HSTS)** untuk memaksa peramban berkomunikasi secara terenkripsi:
  ```http
  Strict-Transport-Security: max-age=63072000; includeSubDomains; preload
  ```

---

## 5. Tata Kelola Kunci Terpusat (Key Management Systems / KMS)
- Jangan pernah menyimpan kunci enkripsi atau salt master di dalam kode sumber (*hardcoded*) atau file `.env` di repositori Git.
- Simpan dan putar (*rotate*) kunci secara berkala menggunakan layanan KMS berstandar FIPS 140-2 Level 3 (seperti HashiCorp Vault, AWS KMS, Google Cloud KMS, atau Azure Key Vault).
