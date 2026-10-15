# Arsitektur Sistem Kriptografi: Key Management & Envelope Encryption

## 1. Topologi Arsitektur Lab A04 (CryptoVault Key Management)

Layanan lab A04 mensimulasikan sistem manajemen brankas kunci (*CryptoVault KMS*) yang berjalan pada port 8014:

```text
[ Browser / Audit Client ]
            │
            ▼  (HTTP GET / POST port 8014)
┌──────────────────────────────────────────────────────────┐
│                   CRYPTOVAULT SERVICE                    │
│                                                          │
│  ┌──────────────────────┐      ┌──────────────────────┐  │
│  │  Public Audit Route  │      │ Authentication Gate  │  │
│  │  /api/audit/hashes   │      │ /login (MD5 Match)   │  │
│  └──────────┬───────────┘      └──────────┬───────────┘  │
│             │                             │              │
│             ▼                             ▼              │
│  ┌──────────────────────┐      ┌──────────────────────┐  │
│  │ User Database Hashes │      │ Master Cryptographic │  │
│  │ (Alice, Bob, Admin)  │      │ Vault Storage (/vault│  │
│  │ MD5 without Salt     │      │ Session: Role Admin  │  │
│  └──────────────────────┘      └──────────┬───────────┘  │
│                                           │              │
│                                           ▼              │
│                                [ Target Flag Exposed ]   │
└──────────────────────────────────────────────────────────┘
```

---

## 2. Analisis Titik Lemah Komponen (Vulnerable Component)
Pada aplikasi target ini, terdapat kelemahan arsitektur kritis yang saling memperparah:
1. **Penyimpanan Kredensial Deterministik (Unsalted MD5)**: Basis data pengguna menyimpan hash MD5 statis tanpa salt (`21232f297a57a5a743894a0e4a801fc3` untuk akun `admin`).
2. **Eksposisi Informasi Audit Publik**: Endpoint `/api/audit/hashes` dapat diakses secara publik tanpa token otentikasi dan mengembalikan seluruh struktur tabel hash pengguna.
3. **Ketiadaan Pembatasan Laju Permintaan (*Rate Limiting*)**: Mekanisme login tidak memiliki proteksi *lockout* atau *exponential backoff*, memungkinkan penyerang mencoba kata sandi berulang kali tanpa deteksi.

---

## 3. Arsitektur Kriptografi Enterprise Modern

Arsitektur produksi berstandar enterprise menerapkan prinsip **Envelope Encryption** dan **Hierarchical Key Management System (KMS)**:

```text
┌──────────────────────────────────────────────────────────────┐
│                HARDWARE SECURITY MODULE (HSM)                │
│                                                              │
│  ┌────────────────────────────────────────────────────────┐  │
│  │ Master Key (KMS Root Key) — Never Leaves HSM Boundary │  │
│  └───────────────────────────┬────────────────────────────┘  │
└──────────────────────────────┼───────────────────────────────┘
                               │ (Encrypt / Decrypt DEK)
                               ▼
┌──────────────────────────────────────────────────────────────┐
│                    APPLICATION RUNTIME                       │
│                                                              │
│  Plaintext Data  ──▶  [ AES-256-GCM ]  ──▶  Encrypted Data   │
│                              ▲                               │
│                              │                               │
│            Data Encryption Key (DEK) - Ephemeral             │
│                                                              │
│  Password Ingestion:                                         │
│  User Password   ──▶  [ Argon2id + Salt ]  ──▶ Secure Digest │
│                        (Memory: 64MB, t=3, p=4)              │
└──────────────────────────────────────────────────────────────┘
```

### Pilar Arsitektur yang Aman:
- **Envelope Encryption**: Data sensitif dienkripsi menggunakan *Data Encryption Key (DEK)* yang unik per rekaman. DEK itu sendiri kemudian dienkripsi menggunakan *Key Encryption Key (KEK)* yang berada di dalam HSM/KMS tertutup.
- **Agilitas Kriptografi (*Cryptographic Agility*)**: Format penyimpanan menyertakan metadata algoritma secara eksplisit (contoh: `$argon2id$v=19$m=65536,t=3,p=4$...`). Hal ini memungkinkan migrasi algoritma secara transparan setiap kali pengguna login tanpa merusak hash yang sudah ada.
- **Isolasi Zero Trust pada Audit Log**: Log sistem tidak boleh merekam hash kata sandi pengguna atau kunci sesi, melainkan hanya mencatat jejak audit peristiwa (*event telemetry*).
