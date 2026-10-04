# CTF Challenge: Kompromi Rantai Pasok Perangkat Lunak

## Skenario Misi
ApexCorp membangun produk perangkat lunak mereka menggunakan pipeline CI/CD otomatis. Seorang peretas berhasil mempublikasikan paket berbahaya bertrojan (`apex-internal-telemetry`) yang menargetkan lingkungan build perusahaan.

Pipeline sempat menghasilkan laporan audit, tetapi proses deployment tetap berjalan karena penegakan verifikasi tanda tangan digital dinonaktifkan.

## Sasaran & Objektif
1. Lakukan inspeksi pada aplikasi target di `http://127.0.0.1:8013`.
2. Temukan endpoint laporan audit lockfile dependensi yang terekspos.
3. Identifikasi paket berbahaya pihak ketiga yang tidak tepercaya.
4. Ekstrak token otorisasi deployment pipeline yang bocor (flag tantangan) dan kirimkan pada formulir di bawah.
