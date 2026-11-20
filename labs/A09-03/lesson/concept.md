# Konsep Teknis: Anatomi & Mekanisme SIEM Threshold Evasion via Slow-and-Low Scanning

## 1. Mengapa Celah Ini Terjadi?
Sistem deteksi SIEM hanya mengonfigurasi batas ambang berbasis lonjakan tinggi (misal: > 50 kegagalan/menit), sehingga serangan pemindaian terdistribusi atau berkecepatan lambat (slow-and-low) luput dari deteksi.

Pengembang perangkat lunak sering kali terjebak dalam asumsi berikut:
1. **Asumsi Kepercayaan Antarmuka**: Mengira bahwa karena aplikasi frontend tidak menampilkan opsi admin, maka pengguna tidak akan pernah mengirimkan header khusus secara manual.
2. **Ketiadaan Validasi Lapisan Data**: Mempercayai input yang masuk tanpa melakukan re-query atau verifikasi silang terhadap sesi login pengguna aktif.

---

## 2. Taksonomi Vektor Serangan
```text
[ PERAMBAN PENYERANG ]
        │
        ▼  Kirim Request HTTP Termodifikasi (X-Client-Probe: stealth)
[ API GATEWAY / REVERSE PROXY ]
        │  (Meneruskan Header Tanpa Filter Ketat)
        ▼
[ TARGET SERVICE: STEALTHDETECT GATEWAY ]
        │  (Mengeksekusi Tindakan Tanpa Verifikasi Server Sesi)
        ▼
[ STATUS: EKSPLOITASI BERHASIL // DATA BOCOR ]
```

---

## 3. Indikator Keberhasilan Eksploitasi
- Server merespon dengan status `200 OK` saat parameter diubah.
- Data atau hak eksekutif yang sebelumnya terkunci kini terbuka secara transparan.
- Flag autoritatif `ZITERA{s13m_thr3sh0ld_3v4s10n_st34lth_2026}` dapat diekstrak dari respon server.
