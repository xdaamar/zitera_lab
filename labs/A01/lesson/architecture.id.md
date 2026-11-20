# Arsitektur Keamanan: Penegakan Otorisasi Multi-Lapis

Untuk mencegah kerentanan Broken Access Control secara menyeluruh pada skala enterprise, sistem tidak boleh hanya bergantung pada satu pengecekan ad-hoc di controller. Arsitektur harus menerapkan model **Policy Enforcement Point (PEP)** dan **Policy Decision Point (PDP)** berlapis.

---

### Alur Arsitektur Penegakan Kontrol Akses

```
[ KLIEN / BROWSER ]
       │  Request HTTP: GET /invoice/2
       ▼
┌────────────────────────────────────────────────────────┐
│ 1. API Gateway / Reverse Proxy                         │
│    - Rate Limiting                                     │
│    - Validasi Format Token / Session Signature         │
└──────────────────────┬─────────────────────────────────┘
                       │
                       ▼
┌────────────────────────────────────────────────────────┐
│ 2. Middleware Autentikasi (AuthN)                      │
│    - Membaca Cookie / Header Authorization             │
│    - Mengidentifikasi identitas pengguna: user_id=1    │
└──────────────────────┬─────────────────────────────────┘
                       │
                       ▼
┌────────────────────────────────────────────────────────┐
│ 3. Policy Enforcement Point (PEP)                      │
│    - Memeriksa apakah user_id=1 diizinkan membaca      │
│      faktur invoice_id=2                               │
│    - Mengirim konteks ke Policy Decision Point (PDP)   │
└──────────────────────┬─────────────────────────────────┘
                       │  Otorisasi GAGAL (Bukan Pemilik)
                       ├────────────────────────► [ 403 / 404 Deny ]
                       │  Otorisasi SUKSES
                       ▼
┌────────────────────────────────────────────────────────┐
│ 4. Lapisan Akses Data (Data Access Layer / DAL)        │
│    - Query berlingkup penyewa (Tenant Scoping)         │
│    - SELECT * FROM invoices WHERE id=2 AND owner_id=1  │
└──────────────────────┬─────────────────────────────────┘
                       │
                       ▼
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
| **Anonymous (Tamu)** | [TIDAK] | [TIDAK] | [TIDAK] | [TIDAK] |
| **Standard User (Alice/Bob)** | [DIIZINKAN] | [TIDAK] | [TIDAK] | [TIDAK] |
| **Auditor / Finance Staff** | [DIIZINKAN] | [DIIZINKAN] (Read-Only) | [TIDAK] | [TIDAK] |
| **System Administrator** | [DIIZINKAN] | [DIIZINKAN] | [DIIZINKAN] | [DIIZINKAN] |
