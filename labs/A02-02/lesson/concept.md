# Konsep Teknis: Anatomi & Mekanisme Insecure CORS Misconfiguration & Origin Reflection

## 1. Mengapa Celah Ini Terjadi?
Konfigurasi CORS server memantulkan kembali Origin header apapun yang dikirim penyerang (Access-Control-Allow-Origin: <origin_pengirim>) sekaligus menyertakan Access-Control-Allow-Credentials: true.

Pengembang perangkat lunak sering kali terjebak dalam asumsi berikut:
1. **Asumsi Kepercayaan Antarmuka**: Mengira bahwa karena aplikasi frontend tidak menampilkan opsi admin, maka pengguna tidak akan pernah mengirimkan header khusus secara manual.
2. **Ketiadaan Validasi Lapisan Data**: Mempercayai input yang masuk tanpa melakukan re-query atau verifikasi silang terhadap sesi login pengguna aktif.

---

## 2. Taksonomi Vektor Serangan
```text
[ PERAMBAN PENYERANG ]
        │
        ▼  Kirim Request HTTP Termodifikasi (Origin: https://attacker-website.com)
[ API GATEWAY / REVERSE PROXY ]
        │  (Meneruskan Header Tanpa Filter Ketat)
        ▼
[ TARGET SERVICE: APINEXUS GATEWAY ]
        │  (Mengeksekusi Tindakan Tanpa Verifikasi Server Sesi)
        ▼
[ STATUS: EKSPLOITASI BERHASIL // DATA BOCOR ]
```

---

## 3. Indikator Keberhasilan Eksploitasi
- Server merespon dengan status `200 OK` saat parameter diubah.
- Data atau hak eksekutif yang sebelumnya terkunci kini terbuka secara transparan.
- Flag autoritatif `ZITERA{c0r5_0r1g1n_r3fl3ct10n_cr3d3nt14l_2026}` dapat diekstrak dari respon server.
