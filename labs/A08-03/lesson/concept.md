# Konsep Teknis: Anatomi & Mekanisme Python Pickle & YAML Unsafe Object Deserialization

## 1. Mengapa Celah Ini Terjadi?
Backend menggunakan modul pickle.loads() atau yaml.load() tanpa SafeLoader terhadap input serialisasi yang dikirimkan klien, memungkinkan penyerang menyisipkan bytecode eksploitasi eksekusi kode jarak jauh (RCE).

Pengembang perangkat lunak sering kali terjebak dalam asumsi berikut:
1. **Asumsi Kepercayaan Antarmuka**: Mengira bahwa karena aplikasi frontend tidak menampilkan opsi admin, maka pengguna tidak akan pernah mengirimkan header khusus secara manual.
2. **Ketiadaan Validasi Lapisan Data**: Mempercayai input yang masuk tanpa melakukan re-query atau verifikasi silang terhadap sesi login pengguna aktif.

---

## 2. Taksonomi Vektor Serangan
```text
[ PERAMBAN PENYERANG ]
        │
        ▼  Kirim Request HTTP Termodifikasi (Content-Type: application/x-python-pickle)
[ API GATEWAY / REVERSE PROXY ]
        │  (Meneruskan Header Tanpa Filter Ketat)
        ▼
[ TARGET SERVICE: PIPELINESERIALIZER ]
        │  (Mengeksekusi Tindakan Tanpa Verifikasi Server Sesi)
        ▼
[ STATUS: EKSPLOITASI BERHASIL // DATA BOCOR ]
```

---

## 3. Indikator Keberhasilan Eksploitasi
- Server merespon dengan status `200 OK` saat parameter diubah.
- Data atau hak eksekutif yang sebelumnya terkunci kini terbuka secara transparan.
- Flag autoritatif `ZITERA{uns4f3_d3s3r14l1z4t10n_p1ckl3_2026}` dapat diekstrak dari respon server.
