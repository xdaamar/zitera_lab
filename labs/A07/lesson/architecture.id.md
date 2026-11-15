# Arsitektur Sistem: Desain Pertahanan Lapisan Autentikasi & Manajemen Sesi

## 1. Topologi Arsitektur Lab A07 (OmniAuth Admin Gateway)

Layanan lab A07 mensimulasikan gerbang login administrasi (*OmniAuth Admin Gateway*) yang berjalan pada port 8017:

```text
[ Browser / Brute-Force Bot ]
             │
             ▼  HTTP GET / POST (Port 8017)
┌──────────────────────────────────────────────────────────┐
│              OMNIAUTH RUNTIME SERVICE                    │
│                                                          │
│  ┌────────────────────────────────────────────────────┐  │
│  │ HTTP Request Dispatcher                            │  │
│  │ • GET  /health          (Health Check)             │  │
│  │ • GET  /login           (HTML Login View)          │  │
│  │ • POST /login           (Form Authentication)      │  │
│  │ • POST /api/login       (JSON API Credential Check)│  │
│  │ • GET  /admin           (Protected View & Flag)    │  │
│  │ • GET  /logout          (Session Teardown)         │  │
│  └────────────────────────┬───────────────────────────┘  │
│                           │                              │
│                           ▼ (Synchronous Pin Matching)   │
│  ┌────────────────────────────────────────────────────┐  │
│  │ Unthrottled Evaluation Engine                      │  │
│  │ • Target: admin:2026                               │  │
│  │ • ZERO Rate Limiting / ZERO Cooldown Delay         │  │
│  └────────────────────────┬───────────────────────────┘  │
│                           │                              │
│                           ▼                              │
│  ┌────────────────────────────────────────────────────┐  │
│  │ Authenticated Administrative Session Context       │  │
│  │ • session["authenticated"] = True                  │  │
│  │ • Exposes Master Challenge Flag                    │  │
│  └────────────────────────────────────────────────────┘  │
└──────────────────────────────────────────────────────────┘
```

---

## 2. Analisis Titik Lemah Arsitektur (Architectural Deficiencies)
Pada sistem target ini, terdapat kelemahan arsitektur struktural yang saling memperparah:
1. **Ketiadaan Lapisan Pembatas Laju (Missing Rate Limiting Guard)**: Baik form web maupun endpoint API JSON langsung mengevaluasi kredensial tanpa memeriksa frekuensi request dari IP pemanggil.
2. **Ketergantungan pada Faktor Tunggal Berentropi Rendah**: Akun memiliki hak istimewa administratif penuh hanya dilindungi oleh PIN 4 digit tanpa autentikasi dua faktor (*Multi-Factor Authentication*).
3. **Respon Waktu Cepat Tanpa Jeda Sintetis**: Server mengembalikan respon kesalahan seketika (< 10ms), memungkinkan penyerang memaksimalkan throughput pengujian tebakan via koneksi socket lokal.

---

## 3. Arsitektur Manajemen Identitas & Autentikasi Enterprise Modern

Sistem berdaya tahan tinggi memisahkan gerbang ingress jaringan dan evaluasi autentikasi melalui arsitektur terisolasi:

```text
[ External Traffic ]
        │
        ▼
┌──────────────────────────────────────────────────────────────┐
│                    API GATEWAY / REVERSE PROXY               │
│                                                              │
│  ┌────────────────────────────────────────────────────────┐  │
│  │ Adaptive Rate Limiting (Token Bucket / Redis Cluster)   │  │
│  │ • Max 5 failed attempts per IP per 15 minutes          │  │
│  │ • Exponential Backoff Delay (1s, 2s, 4s, 8s, 16s)      │  │
│  └───────────────────────────┬────────────────────────────┘  │
└──────────────────────────────┼───────────────────────────────┘
                               │ (Clean / Throttled Traffic)
                               ▼
┌──────────────────────────────────────────────────────────────┐
│             CENTRALIZED IDENTITY PROVIDER (IdP)              │
│                                                              │
│  User Verification  ──▶  [ Argon2id / bcrypt Hash ]          │
│                                      │ (Success)             │
│                                      ▼                       │
│  Second Factor Gate ──▶  [ FIDO2 / WebAuthn / TOTP Token ]   │
│                                      │ (Success)             │
│                                      ▼                       │
│  Session Engine     ──▶  [ Cryptographic Token Rotation ]    │
│                          • Invalidate pre-login session ID   │
│                          • Mint high-entropy HttpOnly cookie │
└──────────────────────────────────────────────────────────────┘
```

### Prinsip Arsitektur yang Aman:
- **Distributed Rate Limiting**: Batasan frekuensi ditegakkan di lapisan gateway terluar menggunakan algoritma *Token Bucket* atau *Sliding Window* dengan penyimpanan terdistribusi (Redis).
- **Adaptive Progressive Delays**: Menambahkan penundaan respons buatan (*synthetic delay*) yang meningkat secara eksponensial setelah setiap kegagalan login berturut-turut.
- **Universal Multi-Factor Authentication**: Wajibkan faktor kedua berbasis kepemilikan perangkat keras (*WebAuthn/FIDO2 hardware keys*) atau aplikasi TOTP untuk seluruh akun administratif.
- **Strict Session Isolation**: Seluruh cookie sesi wajib menggunakan flag `HttpOnly`, `Secure`, dan `SameSite=Lax/Strict` dengan regenerasi ID otomatis saat kenaikan status hak akses.
