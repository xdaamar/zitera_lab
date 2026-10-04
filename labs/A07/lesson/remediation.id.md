# Remediasi: Mengamankan Sistem Autentikasi

Langkah esensial melindungi lapisan autentikasi:

## 1. Terapkan Rate Limiting & Account Lockout
- Batasi jumlah percobaan login (misal maksimal 5 kali percobaan gagal per IP/akun dalam jangka waktu 15 menit).
- Terapkan penundaan eksponensial (*Exponential Backoff*) pada endpoint verifikasi kata sandi.

## 2. Multi-Factor Authentication (MFA / 2FA)
- Wajibkan faktor otentikasi kedua (seperti aplikasi authenticator berbasis TOTP atau kunci FIDO2/WebAuthn) untuk semua akun dengan hak akses tinggi.

## 3. Kebijakan Kata Sandi yang Kuat
- Cegah penggunaan kata sandi umum dengan memeriksa input pendaftaran terhadap database kebocoran kata sandi (seperti API HaveIBeenPwned).
