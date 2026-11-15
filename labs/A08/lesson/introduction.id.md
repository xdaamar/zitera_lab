# Pengenalan: Software and Data Integrity Failures (OWASP A08:2025)

Dalam arsitektur perangkat lunak modern, aplikasi tidak lagi beroperasi sebagai entitas monolitik yang berdiri sendiri. Sistem mengandalkan pustaka pihak ketiga, *base images*, pipeline CI/CD otomatis, pembaruan firmware *over-the-air* (OTA), serta pertukaran data konfigurasi dinamis. **OWASP A08:2025 – Software and Data Integrity Failures** berfokus pada kerentanan di mana kode dan data diterima, didekodekan, atau dieksekusi tanpa validasi kriptografis yang membuktikan keaslian (*authenticity*) dan integritas (*integrity*) sumbernya.

Ketika integritas data atau perangkat lunak diabaikan, batas kepercayaan (*trust boundary*) runtuh. Penyerang dapat menyuntikkan muatan berbahaya ke dalam rantai pembaruan perangkat lunak, memalsukan konfigurasi operasional, atau memanipulasi objek serialisasi yang dieksekusi oleh mesin runtime.

---

## Mengapa Kategori Ini Krusial dalam OWASP Top 10 2025?

Evolusi lanskap ancaman modern telah membuktikan bahwa menembus perimeter aplikasi sering kali lebih sulit daripada meracuni komponen hilir atau saluran pembaruan yang dipercaya secara buta:

1. **Eksploitasi Rantai Pasok (*Supply Chain Exploits*)**: Peristiwa global seperti insiden SolarWinds Orion dan serangan injeksi dependensi membuktikan bahwa penyerang menargetkan mekanisme build dan rilis software.
2. **Perangkat IoT & Edge Tanpa Validasi Firmware**: Miliaran perangkat tertanam menerima paket pembaruan melalui internet. Jika verifikasi tanda tangan digital tidak dipaksakan pada perangkat keras, penyerang dapat mengganti kernel atau konfigurasi operasional secara permanen.
3. **Insecure Deserialization**: Membaca objek mentah dari sumber tak tepercaya tanpa pemeriksaan tipe (*type-checking*) atau verifikasi integritas dapat menyebabkan *Remote Code Execution* (RCE).
4. **Manipulasi Konfigurasi Dinamis**: Sistem terdistribusi yang memuat konfigurasi dari endpoint HTTP eksternal tanpa validasi kriptografis dapat dibelokkan ke server komando musuh (*Command & Control*).

---

## Skenario Lab: Ingesti Konfigurasi Firmware Zitera Edge

Dalam lab ini, Anda akan berhadapan dengan komponen **Zitera Edge Gateway (Port 8018)**. Layanan ini bertanggung jawab menerima paket konfigurasi parameter operasional dan modul firmware dari server orkestrasi pusat.

```
┌─────────────────────────┐          HTTP POST          ┌─────────────────────────┐
│     Untrusted Client    │  ─────────────────────────> │   Zitera Edge Gateway   │
│  (Attacker / Tester)    │   /api/deploy_update        │       (Port 8018)       │
│                         │   {"signature_verified":    │                         │
│                         │    false, "params": ...}    │ ❌ Blind Trust Runtime   │
└─────────────────────────┘                             └─────────────────────────┘
```

Secara fatal, gateway ini mempercayai klaim integritas yang dikirimkan oleh klien itu sendiri (`"signature_verified": false`) dan langsung menerapkan modifikasi parameter berbahaya ke dalam memori kerja aktif tanpa memverifikasi tanda tangan digital asimetris publik.

---

## Sasaran Pembelajaran

Melalui modul ini, Anda akan:
- **Mengaudit Mekanisme Ingesti Data**: Mengidentifikasi ketiadaan verifikasi kriptografis pada endpoint penerima pembaruan.
- **Mendemonstrasikan Tampering Payload**: Menyusun paket konfigurasi termanipulasi yang mematikan proteksi keamanan (mTLS) dan mengaktifkan pintu belakang (*backdoor*).
- **Menganalisis Rantai Kepercayaan Kriptografis**: Memahami bagaimana tanda tangan digital (Ed25519, RSA, Cosign/Sigstore) bekerja untuk mengunci integritas kode.
- **Membangun Pertahanan Integritas**: Menerapkan validasi berbasis kriptografi kunci publik dan verifikasi checksum SHA-256 yang kebal tampering.
