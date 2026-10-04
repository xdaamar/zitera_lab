# Panduan Praktik Terbimbing: Menguji Celah Fail-Open

### Tujuan Investigasi
Dalam latihan ini, kamu akan memicu kondisi kesalahan tak terduga pada gerbang sistem di `http://127.0.0.1:8020` untuk mengamati perilaku fail-open.

### Langkah 1: Kunjungi Sistem Target
- Buka `http://127.0.0.1:8020` pada peramban web.
- Amati form verifikasi token sesi.

### Langkah 2: Picu Exception Input Tidak Valid
- Kirimkan nilai parameter anomali (seperti tipe data null, array kosong, atau format string khusus) yang memicu exception handler di server.

### Langkah 3: Amati Respons Bypass
- Perhatikan bahwa saat exception terjadi, sistem gagal menutup akses dan justru meloloskan otentikasi serta mengembalikan flag rahasia.
