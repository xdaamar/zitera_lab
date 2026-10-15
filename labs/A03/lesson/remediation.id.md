# Panduan Remediasi: Mengamankan Rantai Pasok Perangkat Lunak

Untuk memitigasi risiko kegagalan rantai pasok (*Software Supply Chain Failures*) secara komprehensif, terapkan strategi pertahanan berlapis (*defense-in-depth*) berikut:

---

## 1. Kunci Dependensi dengan Lockfile Ketat (Dependency Pinning)
- **Komit Selalu File Kunci ke Git**: Wajib simpan file kunci (`package-lock.json`, `poetry.lock`, `Cargo.lock`, `go.sum`) ke dalam repositori kode sumber.
- **Hindari Rentang Versi Longgar**: Jangan gunakan operator pembaruan otomatis (*wildcards* seperti `^1.0.0` atau `*`) dalam file manifest utama (`package.json`). Gunakan versi absolut terverifikasi.
- **Gunakan Perintah Instalasi Bersih di CI/CD**:
  ```bash
  # Node.js: Gunakan npm ci alih-alih npm install
  npm ci --ignore-scripts

  # Python: Gunakan pip dengan hash verification
  pip install --require-hashes -r requirements.txt
  ```

---

## 2. Nonaktifkan Eksekusi Skrip Siklus Hidup Otomatis
Banyak serangan supply chain memanfaatkan hook instalasi (`preinstall`, `postinstall`) untuk mencuri kredensial.
- Nonaktifkan skrip otomatis secara global di file `.npmrc` proyek:
  ```ini
  # .npmrc
  ignore-scripts=true
  ```
- Jika ada dependensi sah yang membutuhkan kompilasi binary (misal `node-gyp`), jalankan kompilasi tersebut secara eksplisit di langkah build terisolasi.

---

## 3. Terapkan Software Bill of Materials (SBOM)
- Hasilkan manifest SBOM standar industri pada setiap proses kompilasi rilis menggunakan format **CycloneDX** atau **SPDX**:
  ```bash
  # Menghasilkan SBOM CycloneDX untuk proyek Node.js
  npx @cyclonedx/cyclonedx-npm --output-file bom.json
  ```
- Simpan dan indeks SBOM setiap rilis untuk memudahkan pelacakan jika suatu hari ditemukan kerentanan *Zero-Day* pada komponen pihak ketiga.

---

## 4. Cegah Serangan Dependency Confusion
- **Gunakan Scoped Namespaces**: Beri awalan khusus korporat untuk seluruh paket internal, misalnya `@apexcorp/telemetry`.
- **Konfigurasi Registry Scoping**: Pada file konfigurasi package manager, tetapkan rute eksklusif untuk namespace korporat ke registry privat:
  ```ini
  # .npmrc
  @apexcorp:registry=https://npm.corp.internal/
  always-auth=true
  ```
- **Reservasi Nama di Registry Publik**: Jika memungkinkan, daftarkan nama namespace internal Anda di registry publik (seperti npmjs.org) sebagai paket kosong (*placeholder*) agar penyerang tidak dapat merebutnya.

---

## 5. Integrasi Pemindaian SCA & Verifikasi Integritas di CI/CD
Tambahkan langkah pemeriksaan otomatis pada pipeline CI/CD yang secara otomatis menggagalkan build jika terdeteksi dependensi berbahaya:
```yaml
# Contoh GitHub Actions CI/CD Step
- name: Audit Dependencies
  run: |
    npm audit --audit-level=high

- name: Scan Vulnerabilities with Trivy
  uses: aquasecurity/trivy-action@master
  with:
    scan-type: 'fs'
    ignore-unfixed: true
    severity: 'CRITICAL,HIGH'
```
