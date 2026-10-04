# Pembahasan Teknis: Pola Desain Anti-Pattern

## 1. Kegagalan Model Ancaman (Lack of Threat Modeling)
Merancang sistem tanpa memetakan siapa calon penyerang, apa motivasi mereka, dan batasan apa yang harus ditegakkan. Contoh: Mengizinkan pemulihan password hanya dengan pertanyaan keamanan umum seperti *"Apa warna favoritmu?"* yang jawabannya mudah ditebak.

## 2. Percaya Penuh pada Validasi Sisi Klien
Mempercayai kalkulasi diskon atau harga tiket yang dihitung oleh JavaScript di browser tanpa verifikasi ulang di server.

## 3. Alur Status Bisnis yang Dapat Dilewati (State Skipping)
Sistem checkout e-commerce yang memungkinkan pengguna langsung melompat dari keranjang belanja (`/cart`) ke halaman konfirmasi sukses (`/order-success`) tanpa melewati tahap pembayaran (`/payment`).
