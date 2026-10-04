# Remediasi & Mitigasi Risiko Rantai Pasok (Supply Chain Security)

Strategi utama melindungi aplikasi dari ancaman Supply Chain:

## 1. Gunakan Lockfile & Kunci Versi Dependensi (Dependency Pinning)
- Wajib sertakan file kunci (`package-lock.json`, `poetry.lock`, `Cargo.lock`) ke dalam sistem kontrol versi Git.
- Jangan gunakan rentang versi longgar (*wildcards* seperti `^1.0.0` atau `*`), gunakan versi spesifik yang terverifikasi.

## 2. Terapkan Software Bill of Materials (SBOM) & Pemindaian SCA
- Buat daftar lengkap komponen perangkat lunak (SBOM) menggunakan standar seperti CycloneDX atau SPDX.
- Jalankan alat *Software Composition Analysis (SCA)* seperti Trivy, Snyk, atau `npm audit` di setiap pipeline CI/CD.

## 3. Verifikasi Tanda Tangan Kriptografi (Cryptographic Signatures)
- Verifikasi tanda tangan paket sebelum instalasi menggunakan Sigstore/Cosign.
- Terapkan kebijakan registry internal terisolasi (*private proxy repository*) yang memblokir dependensi pihak ketiga tanpa izin tim keamanan.
