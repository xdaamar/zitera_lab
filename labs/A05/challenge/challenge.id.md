# CTF Challenge: Ekstraksi Brankas Tersembunyi (Hidden Vault Extraction)

### MISI
Portal inventaris gudang perangkat keras militer di `http://127.0.0.1:8015` memungkinkan staf mencari katalog produk. Audit internal mengindikasikan bahwa catatan proyek rahasia yang memuat master flag disimpan di tabel privat bernama `vault_secrets`.

### OBJEKTIF
Lakukan serangan SQL Injection (teknik UNION-based injection) pada kolom pencarian untuk mengekstrak flag rahasia dari tabel database target.

### LINGKUNGAN TARGET
- **Base URL:** `http://127.0.0.1:8015`
- **Bidang Serangan:** Kolom Input Pencarian Produk (Product Search)

### FORMAT SUBMISSION FLAG
`ZITERA{...}`
