# Konsep Teknis: Anatomi & Mekanisme Vertical Privilege Escalation & Header Role Manipulation

## 1. Mengapa Celah Ini Terjadi?
Aplikasi web mempercayai header HTTP yang dikirimkan klien (seperti X-User-Role: admin atau X-Original-Role) alih-alih membaca hak akses dari sesi terotentikasi di server.

Pengembang perangkat lunak sering kali terjebak dalam asumsi berikut:
1. **Asumsi Kepercayaan Antarmuka**: Mengira bahwa karena aplikasi frontend tidak menampilkan opsi admin, maka pengguna tidak akan pernah mengirimkan header khusus secara manual.
2. **Ketiadaan Validasi Lapisan Data**: Mempercayai input yang masuk tanpa melakukan re-query atau verifikasi silang terhadap sesi login pengguna aktif.

---

## 2. Taksonomi Vektor Serangan
```text
[ PERAMBAN PENYERANG ]
        │
        ▼  Kirim Request HTTP Termodifikasi (X-User-Role: admin)
[ API GATEWAY / REVERSE PROXY ]
        │  (Meneruskan Header Tanpa Filter Ketat)
        ▼
[ TARGET SERVICE: OPSADMIN PORTAL ]
        │  (Mengeksekusi Tindakan Tanpa Verifikasi Server Sesi)
        ▼
[ STATUS: EKSPLOITASI BERHASIL // DATA BOCOR ]
```

---

## 3. Indikator Keberhasilan Eksploitasi
- Server merespon dengan status `200 OK` saat parameter diubah.
- Data atau hak eksekutif yang sebelumnya terkunci kini terbuka secara transparan.
- Flag autoritatif `ZITERA{v3rt1c4l_pr1v_3sc4l4t10n_h34d3r_2026}` dapat diekstrak dari respon server.
