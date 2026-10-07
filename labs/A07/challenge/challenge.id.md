# CTF Challenge: Pembobolan Autentikasi Tanpa Proteksi

### RINGKASAN MISI
Sebuah konsol manajemen server internal di `http://127.0.0.1:8017` menerapkan mekanisme login administrator yang hanya dilindungi oleh PIN numerik 4 digit tanpa fitur rate limiting atau penundaan request.

### OBJEKTIF
Lakukan analisis dan tebak PIN akun administrator secara sistematis untuk masuk ke dalam portal administratif dan klaim flag tantangan.

### LINGKUNGAN TARGET
- Base URL: `http://127.0.0.1:8017`
- Username Target: `admin`

### FORMAT SUBMISSION FLAG
`ZITERA{...}`
