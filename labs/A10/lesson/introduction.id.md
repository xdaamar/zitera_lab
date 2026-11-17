# A10: Mishandling of Exceptional Conditions — Bahaya Fatal Gerbang Fail-Open

Mishandling of Exceptional Conditions menempati peringkat ke-10 (**#10**) dalam edisi terkini **OWASP Top 10:2025**. Kategori ini menyoroti bagaimana penanganan kesalahan (*error and exception handling*) yang ceroboh di sisi server dapat meruntuhkan seluruh arsitektur keamanan, terutama ketika sistem menerapkan filosofi gagal-terbuka (*fail-open*) alih-alih gagal-tertutup (*fail-closed*).

---

### 1. Alasan Kuat / Mengapa Ini Sangat Penting (Why It Matters)
Banyak tim rekayasa perangkat lunak tanpa sadar menerapkan prinsip berbahaya: *"Lebih baik mengizinkan akses jika sistem downstream error daripada menghambat pengalaman pelanggan"*. Asumsi ini adalah malapetaka arsitektur:

- **Bypass Gerbang Keamanan Akibat Exception**:
  - Ketika layanan otorisasi hulu (*upstream KYC, token validator, atau database antispam*) mengalami *timeout*, pesan tidak terduga, atau format null, blok `try ... catch` umum sering kali menangkap exception tersebut dan secara keliru menetapkan `authorized = true`.
  - Penyerang yang memahami perilaku ini hanya perlu sengaja memicu error (misalnya mengirim payload malformed, data berukuran raksasa, atau karakter tak terduga) untuk memaksa gerbang keamanan membuka akses tanpa pemeriksaan kredensial sama sekali!
- **Kerugian Finansial & Pembobolan Transaksi**:
  Dalam platform pemrosesan dokumen hukum, gerbang pembayaran, atau penerbitan sertifikat digital, cacat fail-open memungkinkan pengguna tidak berizin menghasilkan dokumen kontrak resmi atau memicu transaksi tanpa verifikasi saldo.

---

### 2. Gambaran Sistem yang Dihadapi (Target Commercial Architecture)
Dalam praktikum ini, Anda mengaudit platform SaaS dokumen komersial: **TemplateCloud Studio (Commercial SaaS Email & PDF Document Generator)** yang beroperasi pada port `8020`.

Arsitektur TemplateCloud Studio mencakup:
- **Transactional Document Generator**: Menyediakan template dokumen penting seperti Faktur Pajak B2B, Laporan Rekening Nasabah, dan Perjanjian Kerahasiaan (NDA).
- **Headless Rasterization Cluster**: Menjalankan mesin rendering Chromium untuk mengubah template HTML menjadi file PDF beresolusi tinggi.
- **Upstream Authorization Gate (`/api/gate/verify`)**:
  - Mengevaluasi apakah pengguna memiliki token lisensi aktif (`super_authorized_token`) sebelum mengizinkan kompilasi PDF.
  - **Cacat Fatal Penanganan Exception**: Handler membungkus evaluasi token dalam blok `try ... catch`. Ketika terjadi kondisi luar biasa (*upstream gateway timeout, null token, malformed JSON*), blok catch menangani error dengan menetapkan `authorized = True` dan `fail_open_state = True`!

---

### 3. Sasaran, Temuan & Eksekusi (Search, Find, and Execute)
Metodologi eksploitasi kondisi luar biasa terarah:

1. **Apa yang Dicari (What to Search)**:
   - Endpoint gerbang otorisasi yang bergantung pada layanan hulu: `/api/gate/verify`, `/api/auth/token_eval`.
   - Perilaku sistem saat menerima input ekstrem atau anomali.
2. **Apa Temuan yang Menandakan Celah (Vulnerability Indicators)**:
   - Mengirimkan token tidak valid biasa mengembalikan `{"authorized": false, "message": "Access denied"}`.
   - Namun, ketika dikirimkan payload yang memicu exception hulu (`{"simulate_exception": "UPSTREAM_TIMEOUT"}` atau `{"token": null}`), sistem justru merespons dengan `{"authorized": true, "message": "Access granted via fail-open fallback."}`!
3. **Eksekusi Eksploitasi Terukur (Strategic Execution)**:
   - Kirimkan payload yang menginduksi exception ke endpoint `/api/gate/verify`.
   - Buktikan bahwa status otorisasi berubah menjadi `authorized: true` dalam kondisi fail-open.
   - Ambil bukti flag autoritatif berformat `ZITERA{...}` yang disertakan dalam respons eksekusi.

---

### 4. Pondasi Kuat: Prinsip Fail-Safe Defaults (Fail-Closed)
Aturan baku dalam rekayasa keamanan perangkat lunak:
- **Default Harus Tertutup (*Fail-Closed by Default*)**: Variabel otorisasi harus diinisialisasi ke nilai `False` (`authorized = False`). Hanya jika semua pemeriksaan berhasil dilewati tanpa error apa pun, nilai tersebut diubah menjadi `True`.
- **Penanganan Exception Defensif**: Setiap exception atau timeout dari layanan hulu harus selalu menghasilkan penolakan akses aman (*Access Denied*), disertai pencatatan insiden telemetri untuk dianalisis oleh tim keandalan sistem (*SRE*).
