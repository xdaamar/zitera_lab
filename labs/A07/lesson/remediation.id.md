# Strategi Remediasi dan Pertahanan Autentikasi Modern

Serangan *brute-force* dan kegagalan autentikasi (OWASP A07:2025) merupakan salah satu vektor serangan paling sering dieksploitasi karena antarmuka autentikasi secara inheren terekspos ke publik. Mengamankan gateway autentikasi membutuhkan pendekatan berlapis (*defense-in-depth*) yang menggabungkan pembatasan laju (*rate limiting*), verifikasi multi-faktor (*MFA*), manajemen sesi yang kebal terhadap pembajakan, serta audit keamanan real-time sesuai panduan **NIST SP 800-63B Digital Identity Guidelines**.

---

## 1. Arsitektur Rate Limiting & Perlindungan Anti-Automasi

Untuk menghentikan serangan tebakan kata sandi atau PIN berkecepatan tinggi, gateway harus membatasi frekuensi percobaan pada berbagai lapisan: per alamat IP, per identitas pengguna (*username*), dan per kluster jaringan (*subnet* / AS number).

### Perbandingan Algoritma Rate Limiting

| Algoritma | Mekanisme Kerja | Kelebihan | Kelemahan | Kasus Penggunaan Ideal |
| :--- | :--- | :--- | :--- | :--- |
| **Fixed Window Counter** | Menghitung hitungan request dalam slot waktu tetap (misal: 60 detik). | Implementasi memori sangat ringan. | Rentan terhadap *burst traffic* di batas jendela (*boundary spikes*). | API publik berbiaya rendah. |
| **Sliding Window Log** | Mencatat stempel waktu setiap request dalam struktur data terurut. | Sangat akurat; tidak ada lonjakan batas jendela. | Konsumsi memori tinggi jika volume request masif. | Endpoint autentikasi sensitif / login admin. |
| **Sliding Window Counter** | Menggabungkan counter jendela sebelumnya dan jendela saat ini secara proporsional. | Akurasi tinggi dengan memori konstan $O(1)$. | Pendekatan perkiraan (*smooth approximation*). | Gateway API skala besar dan microservices. |
| **Token Bucket** | Token ditambahkan ke keranjang dengan laju konstan; request mengonsumsi token. | Mendukung *burst traffic* terkendali tanpa penolakan instan. | Memerlukan koordinasi waktu akurat. | Upload data dan pipeline transaksi. |

### Implementasi Sliding Window Rate Limiter dengan Redis

Dalam arsitektur terdistribusi, status rate limiting harus disimpan pada shared cache seperti Redis agar penyerang tidak dapat mendistribusikan request ke beberapa instance web server.

```python
import time
import redis
from flask import request, jsonify

r = redis.Redis(host='localhost', port=6379, db=0, decode_responses=True)

def check_rate_limit(key: str, max_requests: int = 5, window_seconds: int = 300) -> bool:
    """
    Mengimplementasikan Sliding Window Rate Limiting menggunakan Redis Sorted Sets.
    Mengizinkan maksimal `max_requests` dalam jendela waktu `window_seconds`.
    """
    now = time.time()
    pipeline = r.pipeline()

    # 1. Hapus entri di luar jendela waktu aktif
    pipeline.zremrangebyscore(key, 0, now - window_seconds)

    # 2. Hitung jumlah request tersisa di jendela aktif
    pipeline.zcard(key)

    # 3. Tambahkan stempel waktu request saat ini
    pipeline.zadd(key, {str(now): now})

    # 4. Set waktu kadaluarsa kunci Redis
    pipeline.expire(key, window_seconds)

    _, current_count, _, _ = pipeline.execute()

    return current_count <= max_requests
```

---

## 2. Exponential Backoff dan Progressive Delay

Selain membatasi jumlah request secara biner (izinkan / blokir), penerapan penundaan respons secara bertahap (*progressive delay* atau *exponential backoff*) secara dramatis mereduksi kecepatan skrip otomatis tanpa menimbulkan efek *Denial of Service* total pada pengguna yang sah.

### Rumus Matematis Exponential Backoff

Penundaan response $T_{delay}$ setelah $n$ kegagalan beruntun diformulasikan sebagai:

$$T_{delay}(n) = \min(T_{max}, T_{base} \times 2^{n - 1})$$

Di mana:
- $T_{base}$ adalah penundaan awal (misalnya $0.5$ detik).
- $n$ adalah jumlah percobaan gagal beruntun ($n \ge 1$).
- $T_{max}$ adalah batas atas penundaan maksimal (misalnya $15$ detik) untuk mencegah penumpukan worker server.

```python
def calculate_backoff_delay(failed_attempts: int) -> float:
    base_delay = 0.5   # 500 ms
    max_delay = 15.0   # Maksimal 15 detik
    if failed_attempts <= 0:
        return 0.0
    delay = base_delay * (2 ** (failed_attempts - 1))
    return min(delay, max_delay)
```

---

## 3. Kebijakan Penguncian Akun (*Account Lockout*) & Mitigasi DoS

Penguncian akun (*Account Lockout*) tradisional (misal: kunci akun setelah 5 kegagalan login) memiliki risiko tinggi disalahgunakan penyerang untuk melakukan **Denial of Service terarah** terhadap akun target (*Account Lockout DoS*).

### Best Practice NIST SP 800-63B untuk Penguncian Akun:
1. **Jangan Kunci Akun Secara Permanen**: Jangan mewajibkan kontak admin manual untuk membuka akun kecuali terindikasi pelanggaran berat. Gunakan *temporary suspension* otomatis (misal: 15–30 menit).
2. **Kombinasikan dengan Verifikasi CAPTCHA Beradaptasi**: Picu CAPTCHA (seperti Cloudflare Turnstile atau reCAPTCHA v3) hanya setelah 3 kegagalan berurutan, bukan pada setiap percobaan awal pengguna sah.
3. **Pemisahan Penguncian Berdasarkan Konteks**:
   - Batasi IP agresif secara independen tanpa memblokir akun secara global.
   - Jika satu akun diserang dari puluhan IP berbeda (*distributed credential stuffing*), aktifkan tantangan langkah kedua (*step-up MFA*) alih-alih mengunci akses total.

---

## 4. Multi-Factor Authentication (MFA) yang Kuat

Otentikasi faktor tunggal (kata sandi atau PIN saja) tidak lagi memadai untuk melindungi data sensitif maupun konsol administratif.

### Hierarki Keamanan Metode MFA:

```
[Tertinggi] Hardware Security Key (FIDO2 / WebAuthn, YubiKey)
     │      - Tahan terhadap Phishing (Origin-bound Cryptographic Credentials)
     │      - Tidak menggunakan shared secret yang dapat dibocorkan server
     ▼
[Tinggi]    Software Authenticator App (TOTP - RFC 6238, Google Authenticator)
     │      - Kode 6-digit berubah setiap 30 detik
     │      - Memerlukan shared secret simetris
     ▼
[Rendah]    SMS / Voice OTP (TIDAK DIREKOMENDASIKAN untuk Administrasi)
            - Rentan terhadap SIM Swapping, SS7 Interception, dan Phishing Real-time
```

### Implementasi Verifikasi TOTP (RFC 6238) dalam Python

```python
import pyotp

def verify_user_totp(user_secret: str, user_provided_otp: str) -> bool:
    """
    Memverifikasi token TOTP dengan jendela toleransi 30 detik sebelum dan sesudah
    untuk mengantisipasi desinkronisasi jam perangkat klien.
    """
    totp = pyotp.TOTP(user_secret)
    # valid_window=1 mengizinkan 1 interval (30s) sebelum dan 1 interval sesudah
    return totp.verify(user_provided_otp, valid_window=1)
```

---

## 5. Manajemen Sesi yang Kebal Terhadap Pembajakan

Setelah pengguna berhasil melewati proses autentikasi, integritas sesi harus dijaga agar token tidak dapat dicuri atau dipalsukan.

### Rangkaian Pengamanan Cookie Sesi HTTP:

```http
Set-Cookie: zitera_session_id=d8f4a179e8c3b40182f7c0a9...;
    Path=/;
    Domain=zitera.lab;
    Secure;
    HttpOnly;
    SameSite=Strict;
    Max-Age=3600
```

1. **HttpOnly**: Mencegah script klien (`document.cookie`) membaca token sesi, memitigasi pencurian sesi melalui Cross-Site Scripting (XSS).
2. **Secure**: Menginstruksikan browser hanya mengirimkan cookie melalui koneksi terenkripsi HTTPS / TLS.
3. **SameSite=Strict**: Memblokir cookie agar tidak disertakan dalam permintaan lintas-situs (*cross-site requests*), mengeliminasi vektor serangan Cross-Site Request Forgery (CSRF).
4. **Session Fixation Defense**: Selalu hapus sesi lama dan buat ID sesi baru (*session regeneration*) segera setelah pengguna berhasil login.
5. **Entropy Token**: ID sesi harus dihasilkan menggunakan generator bilangan acak kriptografis (*CSPRNG* seperti `secrets.token_hex(32)` atau `crypto/rand` di Go) dengan entropi minimal 128 bit.

---

## 6. Contoh Arsitektur Backend Aman (Refactor Flask Endpoint)

Berikut adalah perbaikan komprehensif untuk gateway autentikasi `OmniAuth Admin Gateway` yang sebelumnya rentan:

```python
import secrets
import time
from flask import Flask, request, jsonify, session
from werkzeug.security import check_password_hash

app = Flask(__name__)
app.secret_key = secrets.token_hex(32)

# Konfigurasi keamanan cookie
app.config.update(
    SESSION_COOKIE_SECURE=True,
    SESSION_COOKIE_HTTPONLY=True,
    SESSION_COOKIE_SAMESITE='Strict',
)

FAILED_ATTEMPTS = {}
LOCKOUT_TIME = {}

@app.route('/api/auth/pin', methods=['POST'])
def secure_admin_pin_login():
    client_ip = request.remote_addr
    now = time.time()

    # 1. Periksa status lockout
    if client_ip in LOCKOUT_TIME:
        if now < LOCKOUT_TIME[client_ip]:
            remaining = int(LOCKOUT_TIME[client_ip] - now)
            return jsonify({
                "error": f"Akses ditangguhkan karena terlalu banyak kegagalan. Coba lagi dalam {remaining} detik."
            }), 429
        else:
            del LOCKOUT_TIME[client_ip]
            FAILED_ATTEMPTS[client_ip] = 0

    data = request.get_json(silent=True) or {}
    pin = data.get("pin", "")

    # Hash tersimpan dari PIN yang aman (menggunakan Argon2id atau PBKDF2)
    # Bukan komparasi plaintext langsung
    CORRECT_PIN_HASH = "pbkdf2:sha256:600000$..."

    is_valid = check_password_hash(CORRECT_PIN_HASH, str(pin))

    if not is_valid:
        current_fails = FAILED_ATTEMPTS.get(client_ip, 0) + 1
        FAILED_ATTEMPTS[client_ip] = current_fails

        # Terapkan progressive backoff delay
        time.sleep(min(0.2 * (2 ** (current_fails - 1)), 2.0))

        if current_fails >= 5:
            LOCKOUT_TIME[client_ip] = now + 900  # Kunci selama 15 menit (900s)
            return jsonify({
                "error": "Batas percobaan terlampaui. Alamat IP ditangguhkan selama 15 menit."
            }), 429

        return jsonify({
            "error": "Kredensial tidak valid",
            "attempts_remaining": 5 - current_fails
        }), 401

    # Login Berhasil: Reset kegagalan dan regenerasi sesi
    FAILED_ATTEMPTS.pop(client_ip, None)
    session.clear()
    session['admin_authenticated'] = True
    session['user_id'] = "admin"
    session['created_at'] = now

    return jsonify({
        "status": "success",
        "message": "Autentikasi berhasil. Selamat datang, Administrator."
    }), 200
```

---

## 7. Checklist Verifikasi Keamanan Autentikasi

Gunakan checklist ini saat melakukan *code review* atau pengujian penetrasi antarmuka autentikasi:

- [ ] Seluruh endpoint autentikasi membatasi request per IP dan per akun (*Rate Limiting*).
- [ ] Penundaan respons bertahap (*Exponential Backoff*) diaktifkan pada kegagalan berulang.
- [ ] Kata sandi dan rahasia diverifikasi menggunakan algoritma *password hashing* lambat berkekuatan tinggi (Argon2id, bcrypt, atau PBKDF2).
- [ ] Waktu respons login konsisten terhadap *valid username* dan *invalid username* guna mencegah *user enumeration timing attacks*.
- [ ] Session ID diregenerasi setelah autentikasi berhasil untuk mencegah serangan *Session Fixation*.
- [ ] Cookie sesi membawa flag `Secure`, `HttpOnly`, dan `SameSite=Strict` atau `Lax`.
- [ ] Fitur MFA (WebAuthn/TOTP) diwajibkan untuk semua akses administratif dan endpoint kritis.
- [ ] Log audit mencatat kegagalan dan keberhasilan login lengkap dengan IP, stempel waktu, dan identitas (tanpa pernah mencatat kata sandi plaintext).
