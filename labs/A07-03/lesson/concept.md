# Konsep Teknis: Anatomi & Mekanisme MFA Bypass via Direct Endpoint Traversal

## 1. Mengapa Celah Ini Terjadi?
Setelah verifikasi kata sandi utama berhasil, backend menandai sesi pengguna sebagai terotentikasi penuh di cookie tanpa memvalidasi flag mfa_completed pada rute internal aplikasi.

Pengembang perangkat lunak sering kali terjebak dalam asumsi berikut:
1. **Asumsi Kepercayaan Antarmuka**: Mengira bahwa karena aplikasi frontend tidak menampilkan opsi admin, maka pengguna tidak akan pernah mengirimkan header khusus secara manual.
2. **Ketiadaan Validasi Lapisan Data**: Mempercayai input yang masuk tanpa melakukan re-query atau verifikasi silang terhadap sesi login pengguna aktif.

---

## 2. Taksonomi Vektor Serangan
```text
[ PERAMBAN PENYERANG ]
        │
        ▼  Kirim Request HTTP Termodifikasi (Cookie: session_user=operator)
[ API GATEWAY / REVERSE PROXY ]
        │  (Meneruskan Header Tanpa Filter Ketat)
        ▼
[ TARGET SERVICE: TWOFACTOR SECURENET ]
        │  (Mengeksekusi Tindakan Tanpa Verifikasi Server Sesi)
        ▼
[ STATUS: EKSPLOITASI BERHASIL // DATA BOCOR ]
```

---

## 3. Indikator Keberhasilan Eksploitasi
- Server merespon dengan status `200 OK` saat parameter diubah.
- Data atau hak eksekutif yang sebelumnya terkunci kini terbuka secara transparan.
- Flag autoritatif `ZITERA{mf4_byp4ss_d1r3ct_r0ut3_2026}` dapat diekstrak dari respon server.
