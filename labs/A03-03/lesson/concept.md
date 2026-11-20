# Konsep Teknis: Anatomi & Mekanisme Malicious Preinstall Script & CI/CD Secret Exfiltration

## 1. Mengapa Celah Ini Terjadi?
Skrip preinstall di package.json dijalankan secara otomatis oleh npm/yarn saat instalasi dependensi, mengekstrak variabel lingkungan CI/CD (ENV variables) ke server eksternal.

Pengembang perangkat lunak sering kali terjebak dalam asumsi berikut:
1. **Asumsi Kepercayaan Antarmuka**: Mengira bahwa karena aplikasi frontend tidak menampilkan opsi admin, maka pengguna tidak akan pernah mengirimkan header khusus secara manual.
2. **Ketiadaan Validasi Lapisan Data**: Mempercayai input yang masuk tanpa melakukan re-query atau verifikasi silang terhadap sesi login pengguna aktif.

---

## 2. Taksonomi Vektor Serangan
```text
[ PERAMBAN PENYERANG ]
        │
        ▼  Kirim Request HTTP Termodifikasi (X-Pipeline-Agent: GitHub-Runner-01)
[ API GATEWAY / REVERSE PROXY ]
        │  (Meneruskan Header Tanpa Filter Ketat)
        ▼
[ TARGET SERVICE: NODEBUILD ENGINE ]
        │  (Mengeksekusi Tindakan Tanpa Verifikasi Server Sesi)
        ▼
[ STATUS: EKSPLOITASI BERHASIL // DATA BOCOR ]
```

---

## 3. Indikator Keberhasilan Eksploitasi
- Server merespon dengan status `200 OK` saat parameter diubah.
- Data atau hak eksekutif yang sebelumnya terkunci kini terbuka secara transparan.
- Flag autoritatif `ZITERA{pr31nst4ll_h00k_c1cd_s3cr3t_l34k_2026}` dapat diekstrak dari respon server.
