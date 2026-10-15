# Panduan Praktik Terbimbing: Investigasi Kegagalan Kriptografi & Memecahkan Hash Unsalted

Panduan ini disusun menggunakan metodologi 4-Tier Mission Progression dengan proporsi **60% observasi manual** dan **40% eksekusi perintah terminal**:

---

## Tier 1: Observasi Permukaan Serangan (Bobot: 15%)

1. **Pastikan Kontainer Lab Aktif**:
   Buka peramban web dan arahkan ke alamat target:
   `http://127.0.0.1:8014`

2. **Analisis Halaman Utama CryptoVault**:
   Amati antarmuka web *ZITERA CryptoVault Key Management System*:
   - Perhatikan kotak informasi ungu di bagian bawah: *Public Cryptographic Inspection Endpoints*.
   - Terdapat 3 rute utama:
     - `/health` — Service health probe.
     - `/api/audit/hashes` — Endpoint log audit yang mencatat hash kata sandi pengguna.
     - `/vault` — Brankas rahasia utama (memerlukan hak akses SecurityOfficer/admin).
   - Klik tombol **Sign In to Vault** untuk mengamati formulir login di `/login`.

---

## Tier 2: Penemuan Endpoint & Injeksi Permintaan (Bobot: 25%)

1. **Uji Health Endpoint**:
   Verifikasi kesiapan layanan melalui peramban atau terminal:
   ```bash
   curl -s http://127.0.0.1:8014/health
   ```
   *Respon*: `{"lab":"A04","port":8014,"status":"healthy"}`

2. **Inspeksi Manual Endpoint Audit Hash**:
   Buka `http://127.0.0.1:8014/api/audit/hashes` di peramban atau gunakan curl:
   ```bash
   curl -s http://127.0.0.1:8014/api/audit/hashes
   ```

3. **Analisis Struktur Data Hash**:
   Amati rekaman JSON yang dikembalikan:
   ```json
   {
     "records": [
       {"hash": "482c811da5d5b4bc6d497ffa98491e38", "hash_type": "MD5 (Unsalted)", "role": "Analyst", "username": "alice"},
       {"hash": "5ebe2294ecd0e0f08eab7690d2a6ee69", "hash_type": "MD5 (Unsalted)", "role": "Auditor", "username": "bob"},
       {"hash": "21232f297a57a5a743894a0e4a801fc3", "hash_type": "MD5 (Unsalted)", "role": "SecurityOfficer", "username": "admin"}
     ],
     "status": "success"
   }
   ```
   - Karakteristik Hash Administrator: String heksadesimal 32 karakter (128 bit): `21232f297a57a5a743894a0e4a801fc3`.
   - Informasi Kritis: Algoritma yang digunakan adalah **MD5 murni tanpa salt** (*Unsalted*).

---

## Tier 3: Rekonstruksi Kredensial & Pengambilan Flag (Bobot: 35%)

1. **Pecahkan Hash Deterministik MD5**:
   Karena hash dibuat tanpa salt, nilai hash untuk kata sandi umum bersifat konstan di seluruh dunia. Uji kata sandi kandidat menggunakan terminal:
   ```bash
   # Menguji kata sandi 'admin'
   echo -n "admin" | md5sum
   ```
   *Hasil*: `21232f297a57a5a743894a0e4a801fc3` (Cocok 100% dengan hash administrator).

2. **Autentikasi ke Dalam Vault**:
   Kembali ke formulir login di `http://127.0.0.1:8014/login`:
   - **Username**: `admin`
   - **Password**: `admin`
   - Klik **Authenticate**.

3. **Ekstraksi Master Secret Flag**:
   Sistem memvalidasi kecocokan hash MD5 dan mengarahkan sesi Anda ke halaman rahasia `/vault`:
   - Peran terautentikasi: `SecurityOfficer (admin)`.
   - Flag rahasia vault berformat: `ZITERA{...}`.
   - Salin flag yang muncul di layar dan submit ke tab Challenge atau validator.

---

## Tier 4: Validasi Perbaikan & Hardening Defensif (Bobot: 25%)

1. **Hapus Endpoint Eksposisi Kredensial**:
   Tutup rute publik `/api/audit/hashes` agar tidak membocorkan informasi autentikasi internal ke publik.

2. **Migrasi ke Password Hashing Adaptif (bcrypt/Argon2id)**:
   Ganti hashing MD5 dengan implementasi bcrypt bertingkat:
   ```python
   import bcrypt
   # Hashing kata sandi aman dengan salt unik 12 rounds
   salt = bcrypt.gensalt(rounds=12)
   hashed = bcrypt.hashpw("admin".encode('utf-8'), salt)
   ```

3. **Verifikasi Ketahanan Terhadap Rainbow Table**:
   Dengan bcrypt, setiap pembuatan hash untuk kata sandi `"admin"` akan selalu menghasilkan string yang berbeda (misalnya `$2b$12$K...`), membuat basis data kamus publik menjadi tidak berdaya.
