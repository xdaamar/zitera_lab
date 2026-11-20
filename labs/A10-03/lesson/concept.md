# Konsep Teknis: Anatomi & Mekanisme Full-Chain Blind SSRF to Cloud Metadata

## 1. Mengapa Celah Ini Terjadi?
Fitur penarikan gambar atau URL eksternal (PDF converter / webhook) tidak memvalidasi alamat IP tujuan, mengizinkan penyerang memaksa server memanggil endpoint internal metadata cloud (http://169.254.169.254/).

Pengembang perangkat lunak sering kali terjebak dalam asumsi berikut:
1. **Asumsi Kepercayaan Antarmuka**: Mengira bahwa karena aplikasi frontend tidak menampilkan opsi admin, maka pengguna tidak akan pernah mengirimkan header khusus secara manual.
2. **Ketiadaan Validasi Lapisan Data**: Mempercayai input yang masuk tanpa melakukan re-query atau verifikasi silang terhadap sesi login pengguna aktif.

---

## 2. Taksonomi Vektor Serangan
```text
[ PERAMBAN PENYERANG ]
        │
        ▼  Kirim Request HTTP Termodifikasi (Content-Type: application/json)
[ API GATEWAY / REVERSE PROXY ]
        │  (Meneruskan Header Tanpa Filter Ketat)
        ▼
[ TARGET SERVICE: WEBFETCH MICROSERVICE ]
        │  (Mengeksekusi Tindakan Tanpa Verifikasi Server Sesi)
        ▼
[ STATUS: EKSPLOITASI BERHASIL // DATA BOCOR ]
```

---

## 3. Indikator Keberhasilan Eksploitasi
- Server merespon dengan status `200 OK` saat parameter diubah.
- Data atau hak eksekutif yang sebelumnya terkunci kini terbuka secara transparan.
- Flag autoritatif `ZITERA{ssrf_cl0ud_m3t4d4t4_3xf1ltr4t10n_2026}` dapat diekstrak dari respon server.
