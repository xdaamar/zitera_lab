# CTF Challenge: Kebocoran Master Invoice Perusahaan

### MISI
Kamu ditugaskan untuk mengaudit portal keuangan dan penagihan internal sebuah perusahaan. Intelijen keamanan mengindikasikan bahwa administrator sistem secara sembrono membuat sebuah catatan penagihan induk (*master invoice*) tersembunyi yang menyimpan flag aktivasi sistem rahasia.

### TARGET & OBJEKTIF
Eksploitasi celah kontrol akses pada aplikasi target yang berjalan di `http://127.0.0.1:8011` untuk menemukan master invoice rahasia tersebut dan ambil flag tantangannya.

### LINGKUNGAN TARGET
- **Base URL:** `http://127.0.0.1:8011`
- **Kredensial Awal:** `alice` / `password123`

### BATASAN PENYERANGAN (CONSTRAINTS)
- Lakukan pengujian HANYA pada target lokal `127.0.0.1:8011`.
- Jangan mencoba serangan tebak kata sandi secara membabi buta (*brute force*). Celah keamanan murni berada pada parameter kontrol akses dan otorisasi.

### FORMAT SUBMISSION FLAG
Kirimkan flag yang kamu temukan menggunakan format standar:
`ZITERA{...}`
