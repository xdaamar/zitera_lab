# Konsep Teknis: Anatomi & Mekanisme Predictable Password Reset Token via Insecure PRNG

## 1. Mengapa Celah Ini Terjadi?
Pembuatan token reset kata sandi menggunakan fungsi pseudo-random number generator non-kriptografis (seperti Math.random() atau berbasis timestamp) sehingga nilai token berikutnya dapat diprediksi secara matematis.

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
[ TARGET SERVICE: AUTHKEY RECOVERY ]
        │  (Mengeksekusi Tindakan Tanpa Verifikasi Server Sesi)
        ▼
[ STATUS: EKSPLOITASI BERHASIL // DATA BOCOR ]
```

---

## 3. Indikator Keberhasilan Eksploitasi
- Server merespon dengan status `200 OK` saat parameter diubah.
- Data atau hak eksekutif yang sebelumnya terkunci kini terbuka secara transparan.
- Flag autoritatif `ZITERA{prng_pr3d1ct4bl3_r3s3t_t0k3n_2026}` dapat diekstrak dari respon server.
