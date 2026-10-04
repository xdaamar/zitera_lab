# CTF Challenge: Kegagalan Integritas Paket Tanpa Tanda Tangan

### RINGKASAN MISI
Layanan distribusi paket perangkat lunak internal di `http://127.0.0.1:8018` memproses instalasi update tanpa memvalidasi integritas checksum atau tanda tangan digital.

### OBJEKTIF
Eksploitasi ketiadaan verifikasi tanda tangan paket pada sistem target untuk mengekstrak master flag verifikasi integritas.

### LINGKUNGAN TARGET
- **Base URL:** `http://127.0.0.1:8018`

### FORMAT SUBMISSION FLAG
`ZITERA{...}`
