# A07: Identification and Authentication Failures (Kegagalan Autentikasi)

Authentication Failures mencakup kerentanan pada mekanisme verifikasi identitas pengguna, pengelolaan sesi (*session management*), dan proteksi terhadap serangan tebak kata sandi otomatis.

## Mengapa Autentikasi Sering Gagal?
Kelemahan paling umum pada sistem login meliputi:
- Tidak adanya pembatasan frekuensi percobaan (*Rate Limiting*) atau penguncian akun (*Account Lockout*), sehingga penyerang bebas melakukan *Brute-Force Attack* atau *Credential Stuffing*.
- Mengizinkan penggunaan kata sandi yang sangat lemah (misal `123456`, `password`, atau PIN 4 digit sederhana).
- Mekanisme pemulihan akun (*Account Recovery / Forgot Password*) yang lemah atau membocorkan jawaban rahasia.
- Session ID diekspos di URL, tidak diacak dengan baik (*Session Fixation*), atau tidak dihapus setelah logout.
