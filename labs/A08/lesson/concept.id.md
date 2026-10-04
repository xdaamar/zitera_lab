# Pembahasan Teknis: Insecure Deserialization & Unsigned Updates

## 1. Pembaruan Otomatis Tanpa Tanda Tangan Digital (Unsigned Auto-Updates)
Bila aplikasi desktop atau firmware mengunduh patch update melalui protokol HTTP biasa atau tidak memvalidasi tanda tangan kriptografi (seperti GPG/RSA signature), penyerang yang berada di jaringan yang sama (*Man-in-the-Middle*) dapat mencegat dan menyisipkan file executable berbahaya.

## 2. Insecure Deserialization
Deserialisasi adalah proses merekonstruksi objek data dari aliran byte (misal Python `pickle`, Java serialization, atau PHP `unserialize`). Jika objek yang dideserialisasi berasal dari input pengguna tanpa sanitasi, penyerang dapat menyusun payload objek khusus (*gadget chains*) yang mengeksekusi perintah shell berbahaya di server.
