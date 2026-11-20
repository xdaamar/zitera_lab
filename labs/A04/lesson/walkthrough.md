# Panduan Praktik Terbimbing: Audit Kriptografi pada SafeKey Corporate Vault

Dalam latihan terarah ini, kamu akan mempraktikkan proses penemuan kelemahan algoritma hashing dan eksfiltrasi kredensial pada modul perbankan **SafeKey Corporate Vault (Banking Encryption & Credential Vault)** yang beroperasi pada `http://127.0.0.1:8014`.

---

### Langkah 1: Observasi Antarmuka & Uji Ketersediaan Layanan
1. Uji ketersediaan layanan melalui terminal:
   ```bash
   curl -s http://127.0.0.1:8014/health
   ```
   *Respons yang diharapkan:*
   ```json
   {"lab":"A04","port":8014,"status":"healthy"}
   ```
2. Buka peramban web dan akses halaman beranda CryptoVault:
   `http://127.0.0.1:8014/`
3. Amati antarmuka brankas:
   - Terlihat panel informasi *Public Cryptographic Inspection Endpoints*.
   - Rute penting yang terdaftar:
     - `/health` — Endpoint status layanan.
     - `/api/audit/hashes` — Tabel audit telemetri kredensial warisan (*legacy*).
     - `/vault` — Enklave brankas master (dibatasi khusus peran SecurityOfficer/admin).
4. Klik tombol **"Sign In to Vault"** untuk memeriksa antarmuka login di `/login`.

---

### Langkah 2: Inspeksi Endpoint Audit Hashes
1. Akses endpoint audit melalui peramban atau jalankan perintah cURL:
   ```bash
   curl -s http://127.0.0.1:8014/api/audit/hashes
   ```
2. **Analisis Struktur Dokumen JSON:**
   Perhatikan objek records yang dikembalikan:
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
3. **Temuan Kritis:**
   - Semua hash menggunakan algoritma **MD5 tanpa salt** (*Unsalted*).
   - Pengguna dengan peran tertinggi (`SecurityOfficer`) adalah akun `admin` dengan nilai hash:
     `21232f297a57a5a743894a0e4a801fc3`

> [!NOTE]
> [!NOTE]
> **Di Balik Layar: Mengapa MD5 tanpa Salt sangat mudah dibongkar?**  
> Salt adalah deretan karakter acak unik yang digabungkan ke kata sandi sebelum di-hash (misal: `password` + `x89#A!` &rarr; hash). Tanpa salt, kata sandi yang sama akan selalu menghasilkan hash yang identik di seluruh dunia:  
> `MD5("admin")` = `21232f297a57a5a743894a0e4a801fc3`  
> Karena deterministik, peretas telah menghitung miliaran nilai hash ini ke dalam basis data daring (*Rainbow Tables*).

---

### Langkah 3: Pembalikan Hash MD5 (Rainbow Table Lookup)
1. Periksa panjang dan format string hash:
   - `21232f297a57a5a743894a0e4a801fc3` memiliki panjang 32 karakter heksadesimal (MD5).
2. Lakukan pencarian nilai teks asli hash tersebut menggunakan utilitas lokal, lookup table, atau kalkulator MD5:
   - Nilai hash `21232f297a57a5a743894a0e4a801fc3` berasal dari string plaintext yang sangat umum: **`admin`**.
3. Kredensial akun eksekutif berhasil diidentifikasi:
   - **Username:** `admin`
   - **Password:** `admin`

---

### Langkah 4: Autentikasi ke Master Vault & Eksfiltrasi Flag
1. Kunjungi formulir autentikasi di peramban pada `http://127.0.0.1:8014/login`.
2. Masukkan kredensial hasil pemecahan:
   - **Operator Account:** `admin`
   - **Vault Passphrase:** `admin`
3. Sistem akan memvalidasi hash dan mengalihkan Anda ke halaman rahasia:
   `http://127.0.0.1:8014/vault`
4. Atau kamu dapat mengeksekusi proses login dan akses brankas secara langsung melalui terminal:
   ```bash
   # 1. Login dan simpan sesi cookie
   curl -s -c cookies.txt -d "username=admin&password=admin" http://127.0.0.1:8014/login

   # 2. Akses enklave brankas master menggunakan sesi admin
   curl -s -b cookies.txt http://127.0.0.1:8014/vault
   ```
5. Di dalam **Master Cryptographic Key Enclave**, amati banner aktivasi sistem yang memuat segel flag autoritatif:
   `ZITERA{cryp70_f41lur35_w34k_k3y_2026}`
6. Salin bukti flag dan submit ke formulir tantangan Zitera Lab desktop untuk menyelesaikan modul ini!
