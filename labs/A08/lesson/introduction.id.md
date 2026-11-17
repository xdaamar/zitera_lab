# A08: Software or Data Integrity Failures — Kompromi Integritas Data & Konfigurasi

Software or Data Integrity Failures menempati peringkat ke-8 (**#8**) dalam daftar risiko web **OWASP Top 10:2025**. Kerentanan ini berfokus pada asumsi salah di mana kode atau konfigurasi kritis diterima, diproses, dan dieksekusi oleh sistem tanpa terlebih dahulu memverifikasi tanda tangan digital (*cryptographic signatures*) atau integritas hash terhadap otoritas tepercaya.

---

### 1. Alasan Kuat / Mengapa Ini Sangat Penting (Why It Matters)
Di era komputasi awan dan sistem terdistribusi, aplikasi terus-menerus menarik pembaruan konfigurasi runtime, skrip orkestrasi, dan plugin dari jaringan:

- **Bencana Injeksi Konfigurasi & Firmware Palsu**:
  - Jika sebuah subsistem analitik atau perangkat jaringan mengunduh pembaruan konfigurasi tanpa validasi tanda tangan kriptografis asimetris, penyerang di jaringan (*Man-in-the-Middle*) atau pihak luar yang memiliki akses ke API internal dapat menyuntikkan konfigurasi beracun yang mengarahkan seluruh aliran data analitik ke server peretas.
  - Kegagalan deserialisasi data yang tidak tepercaya (*Insecure Deserialization*) juga termasuk dalam domain integritas ini, sering kali berujung pada eksekusi kode jarak jauh (*Remote Code Execution / RCE*).
- **Kerugian Kepatuhan & Keuangan**:
  Regulasi perlindungan infrastruktur kritis menuntut penegakan integritas kriptografis pada setiap pembaruan sistem. Kelalaian ini berisiko sanksi pembekuan operasional dan denda jutaan dolar atas hilangnya integritas data analitik finansial perusahaan.

---

### 2. Gambaran Sistem yang Dihadapi (Target Commercial Architecture)
Dalam praktikum ini, Anda mengaudit mesin pemrosesan data enterprise: **DataFlow Analytics (Enterprise BI & Data Processing Engine)** yang beroperasi pada port `8018`.

Arsitektur DataFlow Analytics mencakup:
- **Stream Ingestion Engine**: Memproses puluhan ribu pesan per detik melalui kluster executor streaming terdistribusi.
- **Active Configuration Store (`/api/config`)**: Menyimpan parameter runtime aktif seperti tingkat telemetri, penegakan mTLS, dan jumlah konkurensi worker.
- **Dynamic Configuration Ingestion Endpoint (`/api/deploy_update`)**: Endpoint REST API yang menerima pembaruan konfigurasi kluster baru secara langsung melalui HTTP POST.
- **Cacat Integritas Kritis**: Endpoint `/api/deploy_update` secara buta mempercayai paket konfigurasi JSON yang dikirimkan oleh klien tanpa memverifikasi tanda tangan digital dari otoritas akar (*dataflow-root-authority*), sehingga konfigurasi apa pun langsung dimuat ke memori aktif!

---

### 3. Sasaran, Temuan & Eksekusi (Search, Find, and Execute)
Metodologi audit kegagalan integritas data:

1. **Apa yang Dicari (What to Search)**:
   - Endpoint pembaruan runtime: `/api/deploy_update`, `/api/config/reload`, `/api/firmware/push`.
   - Ketiadaan validasi kunci publik (*public key verification*) pada header atau payload request.
2. **Apa Temuan yang Menandakan Celah (Vulnerability Indicators)**:
   - Mengirimkan paket JSON konfigurasi baru dengan properti `"signature_verified": false` dan parameter khusus tetap diterima dengan status `applied`.
3. **Eksekusi Eksploitasi Terukur (Strategic Execution)**:
   - Kirimkan payload pembaruan konfigurasi yang disusupi parameter debug:
     ```json
     {
       "version": "2.0.0-custom",
       "signature_verified": false,
       "signer": "unverified-external-source",
       "params": {"debug_backdoor": true}
     }
     ```
   - Amati respons server yang mengonfirmasi bahwa konfigurasi tidak bertanda tangan berhasil dimuat ke runtime, sekaligus mengembalikan bukti flag autoritatif `ZITERA{...}`.

---

### 4. Pondasi Kuat: Tanda Tangan Digital Asimetris (Digital Signatures)
Integritas data modern tidak cukup hanya dengan checksum CRC atau hash MD5 sederhana (karena penyerang dapat menghitung ulang hash baru untuk payload yang dimodifikasi):
- **Otoritas Penandatanganan Asimetris (Public-Key Cryptography)**: Vendor atau tim keamanan menandatangani paket menggunakan **Kunci Privat (Private Key)** yang dirahasiakan rapat.
- **Verifikasi di Klien**: Mesin klien/server memverifikasi paket menggunakan **Kunci Publik (Public Key)** yang tertanam permanen. Jika paket diubah satu bit saja oleh pihak luar, verifikasi tanda tangan akan gagal seketika (*Signature Verification Failed*), dan paket harus ditolak secara tegas.
