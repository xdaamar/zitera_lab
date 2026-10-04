# Panduan Praktik Terbimbing: Menguji Integritas Paket

### Tujuan Investigasi
Dalam latihan ini, kamu akan memeriksa alur verifikasi paket pada sistem di `http://127.0.0.1:8018` dan mendeteksi ketiadaan verifikasi tanda tangan digital.

### Langkah 1: Kunjungi Konsol Paket
- Buka alamat `http://127.0.0.1:8018` pada peramban web.
- Amati proses pembaruan dan verifikasi paket instalasi.

### Langkah 2: Analisis Pemeriksaan Integritas
- Amati request yang dikirimkan ke endpoint instalasi paket.
- Sistem menerima paket tanpa meminta atau memvalidasi checksum SHA-256 maupun tanda tangan GPG.

### Langkah 3: Ekstraksi Flag
- Unggah atau picu deployment paket tak bertanda tangan untuk mengungkap flag autoritas integritas.
