# Konsep Teknis: Anatomi & Mekanisme Multi-Tenant Boundary Escape & Insecure Direct Object Deletion

## 1. Mengapa Celah Ini Terjadi?
Endpoint penghapusan data (DELETE /api/workspaces/:id) mengeksekusi query database langsung berdasarkan ID tanpa menyertakan tenant_id atau ownership validation pada klausa WHERE.

Pengembang perangkat lunak sering kali terjebak dalam asumsi berikut:
1. **Asumsi Kepercayaan Antarmuka**: Mengira bahwa karena aplikasi frontend tidak menampilkan opsi admin, maka pengguna tidak akan pernah mengirimkan header khusus secara manual.
2. **Ketiadaan Validasi Lapisan Data**: Mempercayai input yang masuk tanpa melakukan re-query atau verifikasi silang terhadap sesi login pengguna aktif.

---

## 2. Taksonomi Vektor Serangan
```text
[ PERAMBAN PENYERANG ]
        │
        ▼  Kirim Request HTTP Termodifikasi (Authorization: Bearer <tenant_a_token>)
[ API GATEWAY / REVERSE PROXY ]
        │  (Meneruskan Header Tanpa Filter Ketat)
        ▼
[ TARGET SERVICE: TENANTCLOUD WORKSPACES ]
        │  (Mengeksekusi Tindakan Tanpa Verifikasi Server Sesi)
        ▼
[ STATUS: EKSPLOITASI BERHASIL // DATA BOCOR ]
```

---

## 3. Indikator Keberhasilan Eksploitasi
- Server merespon dengan status `200 OK` saat parameter diubah.
- Data atau hak eksekutif yang sebelumnya terkunci kini terbuka secara transparan.
- Flag autoritatif `ZITERA{mult1_t3n4nt_3sc4p3_d3l3t3_b0l4_2026}` dapat diekstrak dari respon server.
