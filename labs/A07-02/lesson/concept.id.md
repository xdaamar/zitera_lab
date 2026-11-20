# Konsep Teknis: Anatomi & Mekanisme OAuth 2.0 Account Linking Hijacking via Missing State

## 1. Mengapa Celah Ini Terjadi?
Alur integrasi OAuth 2.0 tidak memvalidasi parameter state saat proses callback, sehingga penyerang dapat menjebak korban melalui CSRF untuk menyambungkan akun korban ke akun media sosial penyerang.

Pengembang perangkat lunak sering kali terjebak dalam asumsi berikut:
1. **Asumsi Kepercayaan Antarmuka**: Mengira bahwa karena aplikasi frontend tidak menampilkan opsi admin, maka pengguna tidak akan pernah mengirimkan header khusus secara manual.
2. **Ketiadaan Validasi Lapisan Data**: Mempercayai input yang masuk tanpa melakukan re-query atau verifikasi silang terhadap sesi login pengguna aktif.

---

## 2. Taksonomi Vektor Serangan
```text
[ PERAMBAN PENYERANG ]
        │
        ▼  Kirim Request HTTP Termodifikasi (Accept: text/html)
[ API GATEWAY / REVERSE PROXY ]
        │  (Meneruskan Header Tanpa Filter Ketat)
        ▼
[ TARGET SERVICE: SOCIALCONNECT AUTH ]
        │  (Mengeksekusi Tindakan Tanpa Verifikasi Server Sesi)
        ▼
[ STATUS: EKSPLOITASI BERHASIL // DATA BOCOR ]
```

---

## 3. Indikator Keberhasilan Eksploitasi
- Server merespon dengan status `200 OK` saat parameter diubah.
- Data atau hak eksekutif yang sebelumnya terkunci kini terbuka secara transparan.
- Flag autoritatif `ZITERA{04uth_m1ss1ng_st4t3_4cc0unt_l1nk_2026}` dapat diekstrak dari respon server.
