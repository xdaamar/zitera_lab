# Panduan Praktik Terbimbing: Audit Kriptografi pada SafeKey Corporate Vault

Dalam latihan ini, Anda akan mempraktikkan proses penemuan kelemahan algoritma hashing pada modul keamanan perbankan **SafeKey Corporate Vault (Banking Encryption & Credential Vault)** pada `http://127.0.0.1:8014`.

---

### Tahap 1: Inspeksi Endpoint Audit Hashes
1. Buka peramban web dan akses beranda sistem:
   `http://127.0.0.1:8014/`
2. Klik tautan **"Inspect Legacy Hash Audit Export"** atau akses langsung via endpoint:
   `http://127.0.0.1:8014/api/audit/hashes`
3. Amati catatan JSON yang dikembalikan:
   ```json
   {
     "username": "admin",
     "role": "Chief Information Security Officer",
     "hash_type": "MD5 (Legacy Migration)",
     "hash": "21232f297a57a5a743894a0e4a801fc3"
   }
   ```
4. **Analisis Kritis:** Sistem menggunakan hash MD5 tanpa salt untuk akun administrator eksekutif.

---

### Tahap 2: Pembalikan Hash MD5 (Rainbow Table Lookup)
1. Periksa panjang dan format string hash:
   - `21232f297a57a5a743894a0e4a801fc3` memiliki panjang 32 karakter heksadesimal (MD5).
2. Lakukan pencarian nilai teks asli hash tersebut menggunakan utilitas lokal atau kalkulator MD5:
   - Nilai hash `21232f297a57a5a743894a0e4a801fc3` berasal dari string plaintext yang sangat umum: `admin`.

---

### Tahap 3: Autentikasi ke Master Vault & Eksfiltrasi
1. Kunjungi formulir autentikasi:
   `http://127.0.0.1:8014/login`
2. Masukkan kredensial hasil pemecahan:
   - **Operator Account:** `admin`
   - **Vault Passphrase:** `admin`
3. Sistem akan memvalidasi hash dan mengalihkan Anda ke halaman rahasia:
   `http://127.0.0.1:8014/vault`
4. Di dalam **Master Cryptographic Key Enclave**, salin segel enkripsi autoritatif `ZITERA{...}`.
5. Kirimkan bukti flag ke Zitera Lab desktop untuk menyelesaikan modul ini!
