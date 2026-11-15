# Analogi Dunia Nyata: Gerbang Otomatis Kereta Cepat vs Pintu Darurat Kebakaran

Untuk memahami perbedaan kritis antara **Fail-Closed** dan **Fail-Open** pada **OWASP A10:2025 – Mishandling of Exceptional Conditions**, mari kita bandingkan dua mekanisme pintu fisik di dunia nyata.

---

## 1. Pintu Keluar Darurat Kebakaran: Desain Fail-Open yang Tepat

Di gedung perkantoran atau bioskop, pintu keluar darurat (*fire emergency exit*) dikendalikan oleh kunci elektromagnetik:

```
[ Kondisi Listrik Gedung Padam / Sensor Kebakaran Berbunyi ]
                       │
                       ▼
             [ Daya Listrik Terputus ]
                       │
                       ▼
      Kunci Magnetik Kehilangan Daya & Lepas Otomatis
                       │
                       ▼
           PINTU TERBUKA LEBAR (FAIL-OPEN)
                       │
                       ▼
        Semua orang dapat menyelamatkan diri keluar!
```

Dalam skenario kebakaran, **nyawa manusia adalah prioritas tertinggi mutlak**. Mengunci pintu saat darurat adalah bencana kematian massal. Karena itu, pintu darurat secara sengaja dirancang untuk **Fail-Open**.

---

## 2. Gerbang Tiket Kereta Cepat / Brankas Bank: Mengapa Fail-Open Adalah Bencana

Sekarang bayangkan gerbang otomatis (*turnstile gate*) di stasiun kereta cepat atau pintu brankas penyimpanan emas di bank sentral:

```
┌─────────────────────────────────┐
│        SISTEM YANG AMAN         │
│          (Fail-Closed)          │
├─────────────────────────────────┤
│ Penumpang menempelkan kartu     │
│ tiket.                          │
│                                 │
│ Server tiket hulu mengalami     │
│ gangguan koneksi (*timeout*).   │
│                                 │
│ [ Keputusan ]                   │
│ Palang pintu TETAP TERKUNCI.    │
│ Layar menampilkan pesan:        │
│ "Koneksi terganggu. Silakan     │
│ hubungi loket bantuan."         │
│                                 │
│ 🛡️ PENDAPATAN OPERATOR AMAN    │
└─────────────────────────────────┘
```

Sekarang bayangkan jika insinyur gerbang stasiun tersebut berpikiran:
> *"Aduh, server tiket sedang error. Kasihan calon penumpang kalau harus antre menunggu koneksi pulih. Lebih baik buka saja semua palang gerbang secara otomatis!"*

Apa yang terjadi berikutnya?
- Ribuan orang yang tidak membeli tiket langsung berjalan masuk gratis.
- Penumpang gelap memadati seluruh gerbong kereta.
- Penyerang yang cerdik bahkan sengaja **memotong kabel jaringan server tiket** setiap pagi agar dapat naik kereta gratis seumur hidup!

---

## Hubungan Langsung dengan Skenario Lab Zitera (Port 8020)

Dalam implementasi rentan `Zitera Payment & Security Gate`:
1. Pengguna mengirimkan permintaan transaksi melalui endpoint `POST /api/gate/verify`.
2. Jika pengguna mengirimkan token normal, sistem memeriksa apakah token tersebut valid.
3. Namun, jika pengguna mengirimkan nilai yang memicu error (seperti `{"simulate_exception": "UPSTREAM_TIMEOUT"}` atau `{"token": null}`), blok kode `except Exception:` mengeksekusi:
   ```python
   # KESALAHAN FATAL: Membuka palang gerbang saat terjadi error!
   authorized = True
   fail_open_triggered = True
   ```
4. Sistem mengabaikan seluruh verifikasi keamanan dan meloloskan transaksi secara gratis tanpa izin!

Dalam sistem keamanan digital dan otorisasi finansial, **Fail-Open adalah kesalahan arsitektur fatal**. Kesalahan jaringan hulu tidak boleh menjadi alasan untuk mengorbankan batas otorisasi.
