# CTF Challenge: Celah Kegagalan Penanganan Exception

### RINGKASAN MISI
Layanan verifikasi sesi di `http://127.0.0.1:8020` menerapkan penanganan kondisi pengecualian yang buruk sehingga mengadopsi status fail-open saat menerima input abnormal.

### OBJEKTIF
Kirimkan request beranomali untuk memicu exception pada server target dan dapatkan flag otorisasi.

### LINGKUNGAN TARGET
- **Base URL:** `http://127.0.0.1:8020`

### FORMAT SUBMISSION FLAG
`ZITERA{...}`
