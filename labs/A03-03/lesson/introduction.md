# A03-03: Malicious Preinstall Script & CI/CD Secret Exfiltration

Laboratorium eksplorasi ini memperdalam pemahaman teknik tingkat lanjut dari kategori **A03 (Software Supply Chain Failures)**. Anda akan menghadapi sistem komersial **NodeBuild Engine** yang beroperasi pada port `8046`.

---

### 1. Kenapa Ini Sangat Berbahaya? (Business & Financial Impact)
Celah keamanan ini memiliki dampak destruktif langsung terhadap operasional bisnis:
- **Pelanggaran Batas Kepercayaan Organisasi**: Penyerang mampu melewati gerbang keamanan formal tanpa izin sah, melanggar prinsip integritas dan kerahasiaan data sesuai **UU Perlindungan Data Pribadi No. 27/2022** dan regulasi global **GDPR**.
- **Kerugian Finansial & Hukum**: Kegagalan kontrol pada lingkungan produksi perbankan atau e-commerce berisiko sanksi denda regulasi hingga miliaran rupiah, tuntutan ganti rugi perdata konsumen, dan kerugian reputasi brand jangka panjang.
- **Valuasi Bug Bounty**: Temuan kelemahan jenis ini di platform bug bounty internasional (HackerOne/Bugcrowd) bernilai tinggi dengan tingkat keparahan **High / Critical (CVSS 7.5 – 9.0)**.

---

### 2. Bagaimana Cara Mencari & Mengujinya? (Recon & Vulnerability Indicators)
Metodologi investigasi sistematis yang harus Anda terapkan:
1. **Analisis Permintaan Jaringan (Traffic Inspection)**:
   - Pantau alur HTTP request dan response header menggunakan DevTools Network Tab atau terminal.
   - Amati apakah sistem mempercayai input header klien seperti `X-Pipeline-Agent: GitHub-Runner-01` atau parameter eksternal tanpa validasi sesi server.
2. **Indikator Kerentanan (Vulnerability Signals)**:
   - Server mengembalikan respon `200 OK` dan memberikan hak istimewa atau membocorkan data terisolasi saat parameter pengujian dimodifikasi.
3. **Eksekusi Eksploitasi Terukur**:
   - Gunakan perintah cURL atau antarmuka pengujian untuk membuktikan celah dan mengambil bukti bendera autoritatif.

---

### 3. Bagaimana Cara Memperbaikinya? (Secure Coding & Remediation)
Langkah perbaikan fundamental:
- **Validasi Mutlak di Sisi Server**: Jangan pernah mempercayai header HTTP atau state dari klien yang dapat dimanipulasi peramban.
- **Terapkan Prinsip Least Privilege & Defense-in-Depth**: Batasi akses hanya pada resource yang benar-benar diizinkan oleh sesi terotentikasi.
- **Gunakan Framework Pengerasan Standar**: Implementasikan verifikasi token kriptografis, whitelist ketat, dan error handling fail-closed.
