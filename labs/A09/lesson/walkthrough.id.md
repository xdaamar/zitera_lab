# Panduan Praktik Terbimbing: Menemukan Blindspot Monitoring

### Tujuan Investigasi
Dalam latihan ini, kamu akan menguji sistem pemantauan di `http://127.0.0.1:8019` untuk mengidentifikasi ketiadaan peringatan atas aktivitas anomali.

### Langkah 1: Kunjungi Konsol Audit
- Buka `http://127.0.0.1:8019` pada peramban web.
- Amati antarmuka pemantauan log transaksi dan panel status deteksi.

### Langkah 2: Uji Trigger Aktivitas Anomali
- Kirimkan serangkaian request mencurigakan ke route audit.
- Perhatikan bahwa sistem tidak mencatat peringatan dan tidak memicu penegakan pemblokiran otomatis.

### Langkah 3: Ekstraksi Flag
- Eksploitasi celah blindspot pada endpoint pelaporan untuk mendapatkan flag tantangan.
