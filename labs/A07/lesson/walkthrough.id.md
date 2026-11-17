# Panduan Praktik Terbimbing: Audit Autentikasi pada NexHealth Portal

Dalam panduan terarah ini, Anda akan mempraktikkan proses audit ketahanan autentikasi terhadap portal rekam medis rumah sakit **NexHealth Portal (Hospital Electronic Medical Records & Patient Portal)** pada `http://127.0.0.1:8017`.

---

### Tahap 1: Analisis Antarmuka Masuk & Celah Rate-Limiting
1. Buka peramban web dan akses halaman login:
   `http://127.0.0.1:8017/login`
2. Perhatikan formulir **Staff EMR Gateway**:
   - Akun default adalah `admin` (Chief Medical Officer).
   - Sistem meminta Security PIN 4-Digit.
3. Uji coba input PIN sembarang beberapa kali (misalnya `0000`, `1111`, `1234`).
4. **Analisis Temuan:**
   - Server menampilkan pesan kesalahan namun tidak membatasi jumlah request (*zero rate-limiting*), tidak menerapkan CAPTCHA, dan tidak mengunci akun.

---

### Tahap 2: Pengujian Kredensial Terarah (Targeted Brute-Force)
1. Analisis pola umum PIN 4-digit di lingkungan korporat:
   - Sering kali administrator menggunakan tahun berjalan atau tahun siklus rilis sistem (`2026`).
2. Masukkan kredensial:
   - **Staff Identifier:** `admin`
   - **Security PIN:** `2026`
3. Klik tombol **"Access Clinical Systems"**.
4. Atau kirimkan pengujian via endpoint REST API:
   ```bash
   curl -X POST http://127.0.0.1:8017/api/login \
     -H "Content-Type: application/json" \
     -d '{"username": "admin", "password": "2026"}'
   ```

---

### Tahap 3: Akses Rekam Medis & Eksfiltrasi Flag
1. Autentikasi berhasil dan Anda dialihkan ke konsol klinis `/admin`.
2. Di bagian **Chief Medical Officer Executive Telemetry Seal**, amati token validasi kepatuhan berformat `ZITERA{...}`.
3. Salin bukti flag dan submit ke Zitera Lab desktop untuk menyelesaikan modul ini!
