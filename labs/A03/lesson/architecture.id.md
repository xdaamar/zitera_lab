# Arsitektur Sistem & Model Rantai Pasok Berintegritas Tinggi

## 1. Topologi Arsitektur Lab A03 (ApexCorp Package Registry)

Layanan lab A03 mensimulasikan sistem proxy registry dependensi internal perusahaan (*ApexCorp Dependency Inspector*) yang mengelola dan memvalidasi lockfile sebelum disebarkan ke runner produksi:

```text
[ Developer Workstation / CI Runner ]
                 │
                 ▼  (HTTP GET / POST port 8013)
┌──────────────────────────────────────────────────────────┐
│             APEXCORP REGISTRY PROXY GATEWAY              │
│                                                          │
│  ┌──────────────────────┐      ┌──────────────────────┐  │
│  │   Verified Packages  │      │  Compromised Module  │  │
│  │   • express (4.19.2) │      │  • apex-internal-    │  │
│  │   • lodash (4.17.21) │      │    telemetry         │  │
│  └──────────┬───────────┘      └──────────┬───────────┘  │
│             │                             │              │
│             ▼                             ▼              │
│  ┌────────────────────────────────────────────────────┐  │
│  │      Lockfile Audit Manifest Storage Engine        │  │
│  │      Path: /packages/audit/package_lock_audit.json │  │
│  └────────────────────────┬───────────────────────────┘  │
│                           │                              │
│                           ▼                              │
│  ┌────────────────────────────────────────────────────┐  │
│  │   Pipeline Token Verification Endpoint             │  │
│  │   Route: /api/supply-chain/verify                  │  │
│  └────────────────────────────────────────────────────┘  │
└──────────────────────────────────────────────────────────┘
```

---

## 2. Analisis Titik Lemah Komponen (Vulnerable Component)
Pada aplikasi target ini, terdapat kegagalan arsitektur kritis pada mekanisme build otomatis:
1. **Kegagalan Pemisahan Scope Registry**: Sistem mempercayai paket `apex-internal-telemetry` versi `0.1.0-alpha.build.88` yang diunduh dari mirror lokal tanpa tanda tangan GPG korporat resmi.
2. **Eksekusi Lifecycle Hook Tidak Terisolasi**: Paket jahat ini menyematkan skrip `postinstall_script: node ./scripts/send_env.js` yang secara simulatif mengekstraksi token rahasia pipeline deployment (`exfiltrated_pipeline_token`).
3. **Eksposisi Laporan Audit Tanpa Kontrol Akses**: Laporan build mentah yang memuat seluruh metadata paket (`package_lock_audit.json`) dapat diakses secara publik tanpa autentikasi melalui endpoint `/packages/audit/package_lock_audit.json`.

---

## 3. Arsitektur Rantai Pasok Standar Industri (SLSA Framework)
Untuk mencegah kegagalan seperti ini, arsitektur enterprise modern harus mengadopsi kerangka kerja **SLSA (Supply-chain Levels for Software Artifacts)** Level 3/4:

```text
[ Sumber Kode Git ] ──▶ [ Isolated Build Runner ] ──▶ [ Attestation / Provenance ]
 (Signed Commits)       (Ephemeral Container,          (Sigstore Cosign Hash)
                         No Ambient Internet)                    │
                                                                 ▼
                                                    [ Private Quarantined Repo ]
                                                                 │
                                                    (Security Policy Verification)
                                                                 │
                                                                 ▼
                                                    [ Production Deployment ]
```

### Elemen Kunci Arsitektur yang Aman:
- **Build Service Terisolasi (*Hermetic Builds*)**: Server kompilasi dijalankan dalam container sementara (*ephemeral*) tanpa akses internet bebas, hanya dapat mengakses dependensi yang telah di-cache dan diverifikasi hash integritasnya.
- **Verifikasi Asal-Usul (*Provenance Attestation*)**: Setiap artefak binary wajib disertai metadata yang ditandatangani secara kriptografis (misalnya via Sigstore/Cosign) yang membuktikan repositori sumber, commit SHA, dan identitas runner yang membangunnya.
- **Quarantine Registry & Dual-Key Approval**: Dependensi baru yang masuk dari publik ditempatkan di zona karantina hingga lulus pemindaian SCA otomatis dan persetujuan manual tim keamanan.
