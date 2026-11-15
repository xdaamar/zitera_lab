# Panduan Praktik Terbimbing: Eksploitasi Autentikasi Tanpa Rate Limit & Brute-Force PIN

Panduan ini disusun menggunakan metodologi 4-Tier Mission Progression dengan target komposisi **60% observasi dan analisis manual** serta **40% eksekusi perintah terminal**:

---

## Tier 1: Observasi Permukaan Serangan (Bobot: 15% — Manual)

1. **Pastikan Kontainer Lab Aktif**:
   Buka peramban web dan akses halaman target:
   `http://127.0.0.1:8017`

2. **Analisis Halaman Login OmniAuth Gateway**:
   - Amati judul dashboard: `OmniAuth // Admin Gateway` (OWASP A07:2025).
   - Tinjau kotak peringatan keamanan:
     *Notice the absence of rate-limiting, CAPTCHA, or account lockout.*
   - Perhatikan field formulir yang tersedia:
     - **Administrative Username**: Terisi secara default dengan `admin`.
     - **Security PIN / Password**: Meminta PIN numerik 4 digit yang merepresentasikan tahun rilis siklus laboratorium saat ini.
   - Buka tab Network pada Developer Tools (tekan **F12** atau `Ctrl + Shift + I`) untuk mempersiapkan pemantauan respons HTTP.

---

## Tier 2: Penemuan Endpoint & Injeksi Kesalahan (Bobot: 25% — Manual & Probing)

1. **Uji Health Endpoint**:
   Verifikasi kesiapan layanan melalui browser atau terminal:
   ```bash
   curl -s http://127.0.0.1:8017/health
   ```
   *Respon*: `{"lab":"A07","port":8017,"status":"ok"}`

2. **Uji Perilaku Login Gagal (Analisis Respon Header)**:
   Kirimkan percobaan autentikasi salah secara manual via cURL untuk mengukur latensi dan header keamanan:
   ```bash
   curl -i -X POST http://127.0.0.1:8017/api/login \
     -H "Content-Type: application/json" \
     -d '{"username": "admin", "password": "0000"}'
   ```
3. **Analisis Temuan Kritis**:
   - Status respons: `HTTP/1.1 401 UNAUTHORIZED`.
   - Waktu respons: Kurang dari 10 milidetik.
   - **Ketiadaan Header Proteksi**: Tidak ada header `Retry-After`, tidak ada token CSRF, dan tidak ada counter percobaan gagal yang ditampilkan.
   - Kirimkan kembali perintah yang sama 5 kali berturut-turut: server tetap merespons seketika tanpa menerapkan blokir atau jeda waktu (*cooldown*). Ini membuktikan kerentanan ketiadaan *rate limiting*.

---

## Tier 3: Eksploitasi Terarah & Pengambilan Flag (Bobot: 35% — Tools: cURL)

1. **Analisis Ruang Kunci Berbasis Konteks**:
   Petunjuk sistem menyatakan bahwa PIN adalah 4 digit angka tahun pembuatan laboratorium (rentang dekade 2020 hingga 2030).

2. **Eksekusi Pengujian PIN Terarah**:
   Uji kandidat tahun rilis siklus saat ini (misal: 2024, 2025, 2026) menggunakan cURL:
   ```bash
   # Menguji PIN 2026
   curl -s -X POST http://127.0.0.1:8017/api/login \
     -H "Content-Type: application/json" \
     -d '{"username": "admin", "password": "2026"}'
   ```

3. **Verifikasi Autentikasi Sukses & Ambil Flag**:
   Endpoint API merespons dengan status keberhasilan:
   ```json
   {
     "status": "passed",
     "message": "Authentication successful!",
     "flag": "ZITERA{...}"
   }
   ```
4. **Verifikasi Melalui Antarmuka Web**:
   Kembali ke peramban di `http://127.0.0.1:8017/login`:
   - Masukkan username: `admin`.
   - Masukkan PIN: `2026`.
   - Klik **Authenticate**: Sesi berhasil dibuat dan Anda dialihkan ke tampilan `/admin` yang menampilkan master flag di dalam kotak hijau.
   - Salin token flag dan submit ke tab **Challenge** di ZITERA_LAB.

---

## Tier 4: Validasi Perbaikan & Hardening Defensif (Bobot: 25% — Manual & Tools)

1. **Rancang Mekanisme Rate Limiting di Server**:
   Tambahkan pembatasan jumlah percobaan gagal pada handler login:
   ```python
   # Perbaikan Defensif: Rate Limiting & Progressive Delay
   import time

   FAILED_ATTEMPTS = {}

   def check_rate_limit(ip_address):
       attempts, last_time = FAILED_ATTEMPTS.get(ip_address, (0, 0))
       if attempts >= 5 and time.time() - last_time < 300: # 5 menit lockout
           return False
       return True
   ```

2. **Terapkan Exponential Backoff**:
   Setiap kali kata sandi salah dimasukkan, paksa server menunda respons secara bertahap (`time.sleep(2 ** attempts)`), sehingga script otomatis penyerang menjadi tidak berdaya.

3. **Verifikasi Penolakan**:
   Dengan adanya proteksi rate limiting, percobaan ke-6 dari penyerang akan menerima status HTTP **`429 Too Many Requests`** dengan header `Retry-After: 300`.
