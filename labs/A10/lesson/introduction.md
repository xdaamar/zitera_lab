# A10: Mishandling of Exceptional Conditions — Celah Gerbang Rusak yang Terbuka Bebas (Fail-Open)

Mishandling of Exceptional Conditions menempati peringkat ke-10 (**#10**) dalam daftar risiko keamanan web **OWASP Top 10:2025**. Kerentanan ini terjadi ketika sistem perangkat lunak menghadapi kondisi tak terduga (*unexpected runtime errors, timeouts, parsing exceptions, null pointers*), dan alih-alih mengunci sistem secara aman (*Fail-Safe / Fail-Closed*), kode program justru secara keliru mengizinkan transaksi atau akses tetap lolos (*Fail-Open*).

---

### 1. Kenapa Ini Sangat Berbahaya? (Business & Financial Impact)
Prinsip dasar keamanan komputer adalah: **Jika terjadi keraguan atau kerusakan sistem, tolak semua akses (*Deny by Default / Fail-Secure*)**. Namun banyak sistem komersial melanggarnya demi kenyamanan operasional:

- **Bypass Pembayaran & Kliring Finansial Jutaan Transaksi**:
  - Jika gerbang verifikasi saldo bank atau otentikasi kupon e-commerce mengalami timeout atau error koneksi dan menerapkan perilaku *fail-open*, penyerang dapat membeli tiket atau menarik uang tunai tanpa saldo terpotong sepeser pun.
  - Insiden nyata seperti kerentanan gerbang pembayaran maskapai penerbangan dan gerbang tol otomatis: Saat server pusat mengalami gangguan jaringan, gerbang otomatis membuka palang secara gratis bagi siapapun yang melewatinya.
- **Denda Kepatuhan & Kerugian Likuiditas**:
  Dalam regulasi perbankan modern dan sistem kliring Bank Sentral, sistem yang bersifat *fail-open* pada validasi transaksi moneter dapat langsung dibekukan izin operasionalnya karena mengancam likuiditas cadangan kas institusi.

---

### 2. Bagaimana Cara Mencari & Mengujinya? (Recon & Vulnerability Indicators)
Metodologi pengujian penanganan kondisi luar biasa:

1. **Apa yang Dicari (What to Search)**:
   - Endpoint gerbang otorisasi dan verifikasi token transaksi: `/api/gate/verify`, `/payment/validate`.
   - Parameter uji coba atau input malformasi: payload bernilai `null`, tipe data tidak valid (array di mana string diharapkan), atau header simulasi timeout.
2. **Apa Temuan yang Menandakan Celah (Vulnerability Indicators)**:
   - Mengirim token biasa yang tidak valid menghasilkan respons normal `{"authorized": false}`.
   - Mengirimkan payload malformasi (`{"token": null}`) atau simulasi exception (`{"simulate_exception": "UPSTREAM_GATEWAY_TIMEOUT"}`) memicu blok exception handler yang secara keliru mengembalikan status `{"authorized": true, "fail_open_state": true}`!
3. **Eksekusi Eksploitasi Terukur (Strategic Execution)**:
   - Induksikan kondisi luar biasa ke endpoint verifikasi gerbang transaksi.
   - Buktikan bahwa status otorisasi berubah menjadi `authorized: true` tanpa token sah.
   - Ekstrak bendera validasi autoritatif `ZITERA{f41l_0p3n_3xc3pt10n5_un4uth0r1z3d}`.

---

### 3. Bagaimana Cara Memperbaikinya? (Secure Coding & Remediation)
Arsitektur pertahanan *Fail-Closed* standar industri:
- **Terapkan Prinsip Fail-Safe Defaults**: Variabel status otorisasi wajib diinisialisasi dengan nilai `false` (`authorized = False`). Hanya ubah menjadi `true` jika dan hanya jika seluruh proses verifikasi berhasil tuntas 100%.
- **Tangani Exception dengan Blok Catch yang Menolak Akses**:
  Di dalam blok `try...catch` / `try...except`, jika terjadi kesalahan apapun (*timeout, format error*), kembalikan penolakan mutlak (`return {"authorized": False, "error": "System Error"}`).
- **Gunakan Pola Circuit Breaker yang Aman**: Jika layanan hulu mati, tolak transaksi sementara (*Graceful Rejection*) alih-alih meloloskannya secara otomatis.

---

### Gambaran Sistem yang Dihadapi (Target Commercial Architecture)
Dalam laboratorium ini, kamu mengaudit sistem gerbang dokumen dan transaksi SaaS enterprise: **TemplateCloud Studio / Zitera Gate Transaction Engine (Enterprise SaaS Document & Payment Gateway)** yang beroperasi pada port `8020`.

Komponen arsitektur:
- **Transaction Verification Gate**: Endpoint `POST /api/gate/verify` yang memeriksa token otorisasi klien terhadap layanan validasi hulu (*upstream gateway*).
- **Celah Penanganan Exception (Fail-Open)**: Saat terjadi exception atau input null, blok penanganan error backend secara keliru mengasumsikan akses diizinkan agar operasional tidak terhenti.
- **Flag Enklave**: Bendera CTF yang terbuka saat status otorisasi berhasil ditembus via kondisi fail-open.
