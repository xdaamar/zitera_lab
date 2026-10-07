# Pembahasan Teknis: Titik Buta Keamanan (Security Blindspots)

## Kelemahan Logging yang Sering Ditemui:
1. Tidak Mencatat Upaya Login Gagal: Mengabaikan percobaan login gagal sehingga serangan *brute force* ribuan kali tidak terdeteksi.
2. Hanya Mencatat di File Lokal Server: Jika penyerang berhasil masuk ke server (*root access*), mereka dapat dengan mudah menghapus file log lokal (`/var/log/app.log`) untuk melenyapkan jejak kejahatan.
3. Log Injection: Membiarkan karakter baris baru (`\n`) dari input pengguna masuk ke dalam pesan log, sehingga penyerang dapat memalsukan entri log baru untuk menipu tim analisis keamanan (*SOC team*).
4. Tidak Adanya Sistem Alert Real-Time: Log hanya disimpan pasif di hard disk tanpa ada sistem notifikasi otomatis (seperti webhook Slack/PagerDuty atau integrasi SIEM).
