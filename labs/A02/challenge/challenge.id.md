# CTF Challenge: Kebocoran Konfigurasi OpsGateway

### RINGKASAN MISI
Kamu sedang menjalankan audit keamanan pada gateway operasi internal kritis di `http://127.0.0.1:8012`. Pengintaian pasif mengindikasikan tim deployment mengunggah konfigurasi bawaan dan membiarkan direktori cadangan dapat diakses publik.

### OBJEKTIF
Identifikasi kesalahan konfigurasi pada sistem target untuk menemukan arsip backup konfigurasi milik administrator dan ekstrak flag aktivasi sistem.

### LINGKUNGAN TARGET
- Base URL: `http://127.0.0.1:8012`
- Cakupan Pengujian: Terbatas hanya pada `127.0.0.1:8012`.

### FORMAT FLAG
`ZITERA{...}`
