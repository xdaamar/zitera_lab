# Analogi Dunia Nyata: Segel Holografis Farmasi vs Catatan Tempel Kurir

Untuk memahami bahaya kegagalan integritas perangkat lunak dan data (*OWASP A08:2025*), bayangkan rantai distribusi pasokan obat-obatan penyelamat hidup di sebuah rumah sakit rujukan nasional.

---

## Skenario 1: Rantai Pasok Bersegel Kriptografis Resmi (Sistem yang Aman)

Dalam protokol farmasi standar, setiap peti vaksin dan obat cair kritis dikunci dalam wadah baja khusus sebelum meninggalkan laboratorium manufaktur pusat.

```
┌─────────────────────┐
│ Laboratorium Resmi  │ ──> Kunci Peti dengan Segel Holografis bernomor seri unik
└─────────────────────┘     (Tanda Tangan Digital Kunci Privat)
          │
          ▼
    [ Jalur Truk / Kurir Pengiriman Tak Tepercaya ]
          │
          ▼
┌─────────────────────┐
│   Apotek Rumah Sakit│ ──> Pindai Nomor Seri Segel ke Database Otoritas Pusat
│ (Sistem Verifikasi) │     Jika segel rusak, robek, atau nomor palsu:
└─────────────────────┘     ❌ PETI DITOLAK MENTAH-MENTAH & DIMUSNAHKAN!
```

- Rumah sakit **tidak memercayai pengemudi truk atau kurir**.
- Rumah sakit mengandalkan hukum fisika dan kriptografi: segel tidak dapat dipalsukan tanpa instrumen laser manufaktur pusat, dan nomor seri harus cocok persis dengan sertifikat otoritas pusat.
- Bahkan jika kurir nakal berniat menyuntikkan zat berbahaya ke dalam botol obat, kerusakan fisik pada segel langsung terdeteksi.

---

## Skenario 2: Kegagalan Integritas (Skenario Kerentanan Zitera Edge)

Sekarang bayangkan jika prosedur rumah sakit diubah oleh pihak manajemen yang ceroboh demi "kemudahan dan fleksibilitas integrasi":

```
┌─────────────────────┐
│  Kurir Tidak Dikenal│ ──> Membawa kardus terbuka berisi botol tak bertutup.
└─────────────────────┘     Di kardus tertempel secarik kertas:
          │                 "Segel_Sudah_Diperiksa: SALAH (TIDAK ADA SEGEL)"
          ▼
┌─────────────────────┐
│  Petugas Apotek     │ ──> Membaca catatan: "Oh, kardus ini bilang tidak ada segel.
│  (Zitera Edge API)  │     Baiklah, langsung masukkan obat ini ke infus pasien!"
└─────────────────────┘     ⚠️ PASIEN MENGALAMI KERACUNAN FATAL!
```

### Apa yang Terjadi di Balik Layar Kode?

Dalam implementasi rentan Zitera Edge Gateway:
1. Klien mengirim paket konfigurasi melalui HTTP POST ke `/api/deploy_update`.
2. Paket secara eksplisit menyatakan `"signature_verified": false` dan `"signer": "unverified-external-source"`.
3. Alih-alih menolak paket dengan status HTTP 403 Forbidden atau 400 Bad Request karena ketiadaan tanda tangan digital publik yang valid, sistem backend **menerima muatan tersebut apa adanya** dan langsung menyuntikkannya ke memori konfigurasi aktif (`active_config`).

Sistem ini gagal menerapkan fungsi paling mendasar dari gerbang integritas: **menolak setiap input yang tidak memiliki bukti otentikasi kriptografis terverifikasi**.
