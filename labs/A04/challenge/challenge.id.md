# CTF Challenge: Brankas Hash yang Rusak (The Broken Hash Vault)

### RINGKASAN MISI
Kamu sedang mengaudit layanan manajemen kunci kriptografi internal di `http://127.0.0.1:8014`. Log infrastruktur lama mengindikasikan bahwa akun lama pengguna dimigrasikan menggunakan skema hashing kuno tanpa penambahan salt acak.

### OBJEKTIF
Pulihkan kredensial akun administrator dari log audit hash yang bocor, lakukan login ke brankas administratif, dan ambil master flag tantangan.

### LINGKUNGAN TARGET
- Base URL: `http://127.0.0.1:8014`
- Cakupan Pengujian: Terbatas hanya pada `127.0.0.1:8014`.

### FORMAT SUBMISSION FLAG
`ZITERA{...}`
