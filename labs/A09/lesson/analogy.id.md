# Analogi Dunia Nyata: Kamera CCTV yang Dicabut dan Buku Tamu Kosong

Untuk memahami **Security Logging & Alerting Failures (OWASP A09:2025)**, bayangkan sistem keamanan fisik di sebuah fasilitas brankas deposit bernilai tinggi.

---

## Skenario 1: Bank dengan Fasilitas Pengawasan Penuh (Sistem Aman)

Di sebuah bank modern dengan prosedur pengawasan ketat:

```
[ Pintu Gerbang Depan ]
       │
       ├─ Ada orang mencurigakan mencoba memasukkan kunci palsu 3 kali berturut-turut.
       │  ▼
       │  Kamera CCTV beresolusi tinggi otomatis merekam wajah dan waktu kejadian.
       │  ▼
       │  Sensor getar mendeteksi upaya pembobolan dan membunyikan sirine di ruang jaga.
       │  ▼
       └─ Petugas keamanan langsung bergerak mengamankan lokasi sebelum pintu terbuka!
```

Jika seorang teknisi meminta akses ke ruang server brankas:
- Petugas keamanan mencatat kartu identitas teknisi, jam masuk, tujuan, dan nama supervisor yang menyetujui di buku register fisik dan digital.
- Setiap perpindahan kunci terekam dalam video tanpa ada jeda rekaman (*zero blind spots*).

---

## Skenario 2: Kegagalan Pengawasan & Alarm Bisu (Skenario Lab Zitera SOC)

Sekarang bayangkan jika bank tersebut dijalankan dengan prosedur pengawasan yang rusak total:

```
[ Pintu Gerbang Depan ]
       │
       ├─ Seorang penyusup mencoba 1.000 kombinasi kunci pintu gerbang.
       │  ▼
       │  Lampu alarm TIDAK PERNAH MENYALA (Alerts Fired = 0).
       │  ▼
       │  Buku tamu penjaga gerbang TETAP KOSONG MELOMPONG.
       │  ▼
       └─ Penjaga keamanan di meja resepsionis tertidur lelap tanpa menyadari ada serangan.
```

Lalu penyusup tersebut berjalan ke loket administrasi dan berkata:
> *"Tolong ubah status saya dari tamu biasa menjadi Direktur Utama Bank."*

Petugas administrasi langsung memberikan tanda pengenal Direktur Utama:
- **TIDAK ADA SATU BARIS PUN CATATAN** yang dibuat di buku register.
- Di layar monitor ruang pengawas, tulisan status tetap berkedip: `MONITORING PASSIVE (0 ALERTS FIRED)`.

---

## Apa Pelajaran Penting untuk Arsitektur Perangkat Lunak?

Dalam implementasi rentan `Zitera SOC Audit & Telemetry Console`:
1. Penyerang dapat mengirimkan puluhan tebakan password ke `/api/auth/login`. Sistem mengembalikan error 401, tetapi **sama sekali tidak mencatat kegagalan tersebut ke array audit log**, dan penghitung alert tetap bernilai 0.
2. Penyerang dapat memanggil `/api/admin/role_escalate` untuk mengangkat hak pengguna menjadi `super_admin`. Sistem mengabulkan eskalasi hak tersebut **tanpa menambahkan rekaman ke audit log**.

Sistem ini beroperasi dalam kegelapan total. Administrator tidak akan pernah mengetahui bahwa mereka telah diretas sampai data perusahaan bocor ke forum gelap di internet.
