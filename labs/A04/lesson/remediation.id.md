# Remediasi & Rekomendasi Kriptografi Aman

Pedoman implementasi kriptografi standar industri:

## 1. Gunakan Algoritma Hash Kata Sandi Modern
Gunakan fungsi hashing yang memiliki faktor kerja adaptif (*work factor*) dan tahan terhadap serangan akselerasi perangkat keras GPU/ASIC:
- Pilihan Utama: **Argon2id** (Pemenang Password Hashing Competition)
- Pilihan Alternatif Standar: **bcrypt** dengan cost factor minimal 12, atau **PBKDF2** dengan minimal 600.000 iterasi untuk SHA-256.

```python
# IMPLEMENTASI HASHING AMAN MENGGUNAKAN BCRYPT
import bcrypt

# Menyimpan password baru saat registrasi
salt = bcrypt.gensalt(rounds=12)
hashed_password = bcrypt.hashpw(plain_password.encode('utf-8'), salt)

# Memverifikasi saat pengguna login
if bcrypt.checkpw(entered_password.encode('utf-8'), hashed_password):
    # Password cocok dan terverifikasi aman
    login_success()
```

## 2. Enkripsi Data Saat Transit dan Saat Istirahat
- Wajib gunakan TLS 1.3 (atau minimal TLS 1.2) dengan cipher suite modern yang mendukung *Forward Secrecy* (ECDHE).
- Untuk enkripsi data di database (*data at rest*), gunakan algoritma terautentikasi seperti **AES-256-GCM** atau **ChaCha20-Poly1305**.
- Kelola kunci kriptografi menggunakan layanan KMS terisolasi (seperti HashiCorp Vault, AWS KMS, atau Azure Key Vault).
