# Pembahasan Teknis: Anatomi Serangan Autentikasi & Brute-Force

Keamanan autentikasi bergantung pada dua faktor fundamental: **entropi kredensial (kompleksitas ruang pencarian)** dan **kemampuan sistem menahan laju tebakan otomatis (*rate limiting & throttling*)**.

---

## 1. Analisis Matematis Ruang Pencarian (Keyspace Feasibility)
Ketika sebuah aplikasi mengizinkan PIN numerik 4 digit (`0000` hingga `9999`):
- Total kemungkinan kombinasi:
  $$10^4 = 10.000\text{ kombinasi}$$
- **Waktu Pencarian Tanpa Rate Limiting**:
  Jika server memproses satu request HTTP dalam 10 milidetik (100 request/detik), maka seluruh 10.000 kombinasi dapat diselesaikan dalam:
  $$\frac{10.000}{100} = 100\text{ detik (kurang dari 2 menit!)}$$
- **Pencarian Terarah Berbasis Konteks**:
  Jika penyerang mengetahui petunjuk bahwa PIN merupakan tahun rilis proyek dalam dekade saat ini (rentang 2020 hingga 2030), ruang pencarian menciut dari 10.000 menjadi hanya **11 kemungkinan angka**!

---

## 2. Taksonomi Serangan Terhadap Lapisan Autentikasi

### A. Brute-Force Attack
Mencoba seluruh kemungkinan karakter secara berurutan atau acak terhadap satu akun spesifik (seperti akun `admin`). Serangan ini sangat efektif bila kata sandi pendek atau batas tebakan tidak dibatasi.

### B. Dictionary Attack (Serangan Kamus)
Menggunakan daftar kata sandi populer yang paling sering digunakan manusia (seperti daftar `rockyou.txt` yang memuat jutaan password populer: `password`, `123456`, `admin2026`).

### C. Credential Stuffing
Penyerang tidak menebak dari nol, melainkan menggunakan database hasil kebocoran data (*dump breach*) dari situs lain yang memuat jutaan pasang email dan password nyata.

### D. Password Spraying
Kebalikan dari brute-force biasa: penyerang mencoba 1 atau 2 kata sandi yang sangat umum (misal `Winter2026!`) terhadap ribuan akun pengguna yang berbeda secara perlahan untuk menghindari ambang batas penguncian akun per individu.

---

## 3. Bahaya Session Fixation & Hijacking
Selain masalah tebakan kata sandi, kerentanan autentikasi mencakup siklus hidup token sesi:
- **Session Fixation**: Penyerang membuat sesi kosong di server, lalu memaksa korban menggunakan session ID tersebut (melalui tautan URL). Ketika korban berhasil login, penyerang langsung menggunakan session ID yang sama untuk mengakses akun korban.
- **Missing Session Regeneration**: Aplikasi yang aman **WAJIB** menghancurkan session ID lama dan menerbitkan token sesi acak baru (*session regeneration*) tepat saat verifikasi login berhasil.

---

## 4. Anatomi Ketiadaan Rate Limiting pada Kode
Perhatikan kode endpoint login yang rentan di bawah ini:
```python
# IMPLEMENTASI RENTAN: Tanpa jeda waktu dan tanpa penguncian akun
@app.route("/api/login", methods=["POST"])
def api_login():
    data = request.get_json()
    user = data.get("username")
    pw = data.get("password")

    # KESALAHAN FATAL: Memverifikasi langsung tanpa menghitung jumlah kegagalan
    if user == ADMIN_USER and pw == ADMIN_PIN:
        return jsonify({"status": "passed", "flag": FLAG})

    # Mengembalikan 401 seketika dalam hitungan milidetik
    return jsonify({"status": "failed", "message": "Invalid credentials"}), 401
```
Karena tidak ada penyimpanan jejak kegagalan (misalnya di Redis atau memory cache) dan tidak ada header `Retry-After`, bot penyerang dapat membanjiri endpoint ini tanpa hambatan sama sekali.
