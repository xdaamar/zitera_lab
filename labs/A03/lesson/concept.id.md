# Pembahasan Teknis: Vektor Serangan Rantai Pasok Perangkat Lunak

Ancaman rantai pasok perangkat lunak mengeksploitasi titik buta (*blind spots*) dalam ekosistem pengembangan otomatis. Berikut adalah analisis mendalam mengenai vektor serangan utama:

---

## 1. Typosquatting (Pembodohan Penamaan Paket)
Penyerang mendaftarkan paket berbahaya pada registry publik (seperti npmjs, PyPI, atau crates.io) dengan nama yang sengaja dibuat menyerupai pustaka populer yang memiliki jutaan unduhan:
- **Paket Sah**: `cross-env`, `requests`, `lodash`, `chalk`
- **Paket Typosquatted**: `crossenv`, `requsts`, `loadash`, `chalk-js`

Ketika developer terburu-buru atau mengalami salah ketik satu karakter pada file konfigurasi atau perintah terminal (`npm install crossenv`), manajer paket akan mengunduh paket berbahaya tersebut. Begitu terpasang, muatan trojan langsung dieksekusi di workstation developer atau server CI/CD.

---

## 2. Dependency Confusion (Substitusi Namespace Publik-Privat)
Banyak korporasi mengembangkan paket internal privat (contoh: `@acme-corp/financial-core` atau `internal-auth-token`). Ketika arsitektur build pipeline mengonfigurasi manajer paket untuk membaca dari registry privat dan registry publik secara bersamaan tanpa isolasi namespace yang ketat:
1. Penyerang meneliti nama paket internal perusahaan yang bocor (melalui source code frontend atau manifest publik).
2. Penyerang mendaftarkan paket dengan nama identik di registry publik (npmjs.org atau PyPI), tetapi menetapkan nomor versi yang sengaja sangat tinggi (misalnya `v99.0.0` atau `v10.0.0-release`).
3. Algoritma resolusi standar manajer paket memprioritaskan nomor versi tertinggi dan mengunduh paket penyerang dari internet alih-alih mengambil versi sah dari registry internal.

---

## 3. Eksekusi Skrip Siklus Hidup Otomatis (Lifecycle Hooks)
Sebagian besar manajer paket modern (khususnya npm dan yarn) menyediakan hook siklus hidup seperti `preinstall`, `install`, dan `postinstall` yang bertujuan untuk mengompilasi binding C++ lokal atau mengonfigurasi binary.

Penyerang memanfaatkan hook ini untuk menyusupkan skrip arbitrer:
```json
{
  "name": "apex-internal-telemetry",
  "version": "0.1.0-alpha.build.88",
  "scripts": {
    "postinstall": "node ./scripts/send_env.js"
  }
}
```
Ketika perintah `npm install` dijalankan di server CI/CD, script `send_env.js` dieksekusi secara otomatis dengan hak akses pipeline penuh, membaca variabel lingkungan (`AWS_SECRET_ACCESS_KEY`, `DOCKER_PASSWORD`, token API), dan mengirimkannya ke server Command & Control (C2) penyerang.

---

## 4. Kompromi Maintainer & Hijacking Repositori
Penyerang tidak selalu membuat paket baru. Sering kali mereka menargetkan maintainer paket sumber terbuka yang sudah eksis:
- **Credential Stuffing / Phishing**: Mencuri password akun npm/PyPI maintainer yang tidak mengaktifkan Multi-Factor Authentication (MFA).
- **Social Engineering**: Menawarkan diri sebagai kolaborator sukarela pada proyek yang jarang dipelihara (*abandoned/dormant repo*), kemudian merilis versi minor resmi yang disusupi malware.
- **Domain Hijacking**: Mengambil alih domain email maintainer yang kedaluwarsa untuk mengatur ulang (*reset*) kata sandi akun registry paket.

---

## 5. Anatomi Lockfile & Integritas Kriptografis
File kunci (seperti `package-lock.json`, `poetry.lock`, atau `Cargo.lock`) mencatat pohon dependensi secara deterministik beserta checksum integritas:
- `resolved`: URL absolut tempat tarball diunduh.
- `integrity`: Nilai hash kriptografi (misal SHA-512) dari arsip dependensi.

Jika file lockfile tidak dikomit ke sistem kontrol versi Git atau diubah secara diam-diam tanpa verifikasi audit, pipeline otomatis berisiko mengunduh artefak yang telah dimanipulasi di jaringan transmisi.
