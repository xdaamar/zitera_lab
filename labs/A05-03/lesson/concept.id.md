# Konsep Teknis: Anatomi & Mekanisme OS Command Injection via Unsanitized File Utilities

## 1. Mengapa Celah Ini Terjadi?
Input pengguna langsung disambungkan ke dalam fungsi eksekusi shell sistem operasi (child_process.exec atau system()) tanpa sanitasi karakter pemisah baris perintah (; | &&).

Pengembang perangkat lunak sering kali terjebak dalam asumsi berikut:
1. **Asumsi Kepercayaan Antarmuka**: Mengira bahwa karena aplikasi frontend tidak menampilkan opsi admin, maka pengguna tidak akan pernah mengirimkan header khusus secara manual.
2. **Ketiadaan Validasi Lapisan Data**: Mempercayai input yang masuk tanpa melakukan re-query atau verifikasi silang terhadap sesi login pengguna aktif.

---

## 2. Taksonomi Vektor Serangan
```text
[ PERAMBAN PENYERANG ]
        │
        ▼  Kirim Request HTTP Termodifikasi (Content-Type: application/x-www-form-urlencoded)
[ API GATEWAY / REVERSE PROXY ]
        │  (Meneruskan Header Tanpa Filter Ketat)
        ▼
[ TARGET SERVICE: CLOUDPING NETWORK TOOL ]
        │  (Mengeksekusi Tindakan Tanpa Verifikasi Server Sesi)
        ▼
[ STATUS: EKSPLOITASI BERHASIL // DATA BOCOR ]
```

---

## 3. Indikator Keberhasilan Eksploitasi
- Server merespon dengan status `200 OK` saat parameter diubah.
- Data atau hak eksekutif yang sebelumnya terkunci kini terbuka secara transparan.
- Flag autoritatif `ZITERA{c0mm4nd_1nj3ct10n_sh3ll_3x3c_2026}` dapat diekstrak dari respon server.
