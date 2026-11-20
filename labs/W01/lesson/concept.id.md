# W01: Konsep Teknis Mendalam

### ⚙️ Bagaimana Cara Kerja Sistem?
Pada aplikasi **NexusPay Wire Portal**, sistem beroperasi dengan memproses request dari klien melalui jaringan lokal port `8021`.

```text
[ Browser / Klien ] ─── HTTP Request ───> [ Reverse Proxy / Broker ]
                                                  │
                                                  ▼
                                      [ Sandboxed Service (W01) ]
                                                  │
                                       ( Evaluasi Parameter & State )
                                                  ▼
                                     [ Database / Target Resource ]
```

### ⚠️ Titik Kerentanan (Vulnerability Mechanics):
1. **Kurangnya Validasi Sisi Server**: Parameter yang dikirimkan oleh pengguna dipercaya mentah-mentah tanpa pengecekan kepemilikan yang sah.
2. **Asumsi yang Salah**: Developer mengasumsikan bahwa pengguna hanya akan menggunakan antarmuka web normal dan tidak akan memodifikasi request HTTP secara manual.
3. **Penyatuan Data dan Perintah**: Data pengguna tercampur dengan logika kontrol sistem, memungkinkan manipulasi alur kerja aplikasi.
