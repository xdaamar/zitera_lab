# Konsep Teknis: Anatomi & Mekanisme Blind Boolean & Time-Based SQL Injection

## 1. Mengapa Celah Ini Terjadi?
Aplikasi tidak menampilkan output query SQL secara langsung ke layar, tetapi penyerang dapat mengekstraksi data dengan mengamati perbedaan respon boolean (panjang konten) atau penundaan waktu eksekusi (sleep/benchmark).

Pengembang perangkat lunak sering kali terjebak dalam asumsi berikut:
1. **Asumsi Kepercayaan Antarmuka**: Mengira bahwa karena aplikasi frontend tidak menampilkan opsi admin, maka pengguna tidak akan pernah mengirimkan header khusus secara manual.
2. **Ketiadaan Validasi Lapisan Data**: Mempercayai input yang masuk tanpa melakukan re-query atau verifikasi silang terhadap sesi login pengguna aktif.

---

## 2. Taksonomi Vektor Serangan
```text
[ PERAMBAN PENYERANG ]
        │
        ▼  Kirim Request HTTP Termodifikasi (Accept: application/json)
[ API GATEWAY / REVERSE PROXY ]
        │  (Meneruskan Header Tanpa Filter Ketat)
        ▼
[ TARGET SERVICE: CUSTPORTAL DATABASE ]
        │  (Mengeksekusi Tindakan Tanpa Verifikasi Server Sesi)
        ▼
[ STATUS: EKSPLOITASI BERHASIL // DATA BOCOR ]
```

---

## 3. Indikator Keberhasilan Eksploitasi
- Server merespon dengan status `200 OK` saat parameter diubah.
- Data atau hak eksekutif yang sebelumnya terkunci kini terbuka secara transparan.
- Flag autoritatif `ZITERA{bl1nd_b00l34n_t1m3_sq1_1nj3ct_2026}` dapat diekstrak dari respon server.
