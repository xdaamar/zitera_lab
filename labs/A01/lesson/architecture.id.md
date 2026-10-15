# Arsitektur Keamanan: Penegakan Otorisasi Multi-Lapis

Untuk mencegah kerentanan Broken Access Control secara menyeluruh pada skala enterprise, sistem tidak boleh hanya bergantung pada satu pengecekan ad-hoc di controller. Arsitektur harus menerapkan model **Policy Enforcement Point (PEP)** dan **Policy Decision Point (PDP)** berlapis.

---

### Alur Arsitektur Penegakan Kontrol Akses

```
[ KLIEN / BROWSER ]
       â”‚  Request HTTP: GET /invoice/2
       â–¼
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚ 1. API Gateway / Reverse Proxy                         â”‚
â”‚    - Rate Limiting                                     â”‚
â”‚    - Validasi Format Token / Session Signature         â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
                       â”‚
                       â–¼
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚ 2. Middleware Autentikasi (AuthN)                      â”‚
â”‚    - Membaca Cookie / Header Authorization             â”‚
â”‚    - Mengidentifikasi identitas pengguna: user_id=1    â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
                       â”‚
                       â–¼
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚ 3. Policy Enforcement Point (PEP)                      â”‚
â”‚    - Memeriksa apakah user_id=1 diizinkan membaca      â”‚
â”‚      faktur invoice_id=2                               â”‚
â”‚    - Mengirim konteks ke Policy Decision Point (PDP)   â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
                       â”‚  Otorisasi GAGAL (Bukan Pemilik)
                       â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â–º [ 403 / 404 Deny ]
                       â”‚  Otorisasi SUKSES
                       â–¼
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚ 4. Lapisan Akses Data (Data Access Layer / DAL)        â”‚
â”‚    - Query berlingkup penyewa (Tenant Scoping)         â”‚
â”‚    - SELECT * FROM invoices WHERE id=2 AND owner_id=1  â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
                       â”‚
                       â–¼
[ DATABASE SISTEM TERISOLASI ]
```

---

### Tiga Pilar Arsitektur Kontrol Akses
1. **Penolakan Secara Default (*Deny by Default*):**
   Setiap endpoint atau rute baru harus dianggap tertutup secara default, kecuali jika aturan otorisasi eksplisit telah didefinisikan dan disetujui.
2. **Keterikatan Konteks Sesi (*Session-Bound Ownership*):**
   Identitas pengguna tidak boleh diambil dari parameter request (`$_GET['user_id']` atau JSON body), melainkan wajib dibaca dari data sesi server yang telah terenkripsi atau ditandatangani (*signed session*).
3. **Peta Referensi Tidak Langsung (*Indirect Reference Maps*):**
   Jangan pernah mengekspos ID integer berurutan (`1, 2, 3...`) ke antarmuka publik. Gunakan UUIDv4 kriptografis acak atau peta token sementara (*ephemeral tokens*) per sesi pengguna, sehingga penyerang tidak dapat menebak nomor urut data lain.

---

### Matriks Otorisasi Berbasis Peran (RBAC)

| Peran (Role) | Baca Faktur Sendiri | Baca Faktur Orang Lain | Unduh Master Invoice | Ubah Tarif Penagihan |
| :--- | :---: | :---: | :---: | :---: |
| **Anonymous (Tamu)** | âŒ | âŒ | âŒ | âŒ |
| **Standard User (Alice/Bob)** | âœ… | âŒ | âŒ | âŒ |
| **Auditor / Finance Staff** | âœ… | âœ… (Read-Only) | âŒ | âŒ |
| **System Administrator** | âœ… | âœ… | âœ… | âœ… |
