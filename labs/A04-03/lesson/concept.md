# Konsep Teknis: Anatomi & Mekanisme JWT Algorithm None & HMAC Weak Key Cracking

## 1. Mengapa Celah Ini Terjadi?
Pustaka JWT di sisi server mengizinkan header alg: 'none', sehingga tanda tangan kriptografis dapat diabaikan atau dihapus, memungkinkan penyerang memalsukan klaim wewenang (role: admin).

Pengembang perangkat lunak sering kali terjebak dalam asumsi berikut:
1. **Asumsi Kepercayaan Antarmuka**: Mengira bahwa karena aplikasi frontend tidak menampilkan opsi admin, maka pengguna tidak akan pernah mengirimkan header khusus secara manual.
2. **Ketiadaan Validasi Lapisan Data**: Mempercayai input yang masuk tanpa melakukan re-query atau verifikasi silang terhadap sesi login pengguna aktif.

---

## 2. Taksonomi Vektor Serangan
```text
[ PERAMBAN PENYERANG ]
        │
        ▼  Kirim Request HTTP Termodifikasi (Authorization: Bearer <tampered_jwt>)
[ API GATEWAY / REVERSE PROXY ]
        │  (Meneruskan Header Tanpa Filter Ketat)
        ▼
[ TARGET SERVICE: TOKENSHIELD SSO ]
        │  (Mengeksekusi Tindakan Tanpa Verifikasi Server Sesi)
        ▼
[ STATUS: EKSPLOITASI BERHASIL // DATA BOCOR ]
```

---

## 3. Indikator Keberhasilan Eksploitasi
- Server merespon dengan status `200 OK` saat parameter diubah.
- Data atau hak eksekutif yang sebelumnya terkunci kini terbuka secara transparan.
- Flag autoritatif `ZITERA{jwt_n0n3_4lg0r1thm_f0rg3ry_2026}` dapat diekstrak dari respon server.
