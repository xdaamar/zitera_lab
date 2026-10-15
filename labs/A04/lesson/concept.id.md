# Pembahasan Teknis: Hashing, Salting, & Enkripsi Modern

Kriptografi yang aman bergantung pada pemahaman mendasar atas perbedaan peran antara algoritma enkripsi dua arah (*two-way encryption*) dan fungsi hash satu arah (*one-way hashing*).

---

## 1. Perbedaan Mendasar: Enkripsi vs Hashing

| Karakteristik | Enkripsi Simetris / Asimetris | Fungsi Hash Kriptografis |
| :--- | :--- | :--- |
| **Sifat Matematis** | Dua arah (*reversible* via kunci rahasia) | Satu arah (*strictly irreversible*) |
| **Tujuan Utama** | Melindungi kerahasiaan data yang nantinya harus dibaca kembali | Memverifikasi integritas atau mencocokkan kata sandi tanpa membuka teks asli |
| **Contoh Penggunaan** | Menyimpan nomor kartu kredit (AES-256-GCM), pertukaran pesan TLS | Menyimpan kata sandi (Argon2id), verifikasi checksum file (SHA-256) |
| **Risiko Kunci** | Jika kunci enkripsi bocor, seluruh data terbuka | Tidak menggunakan kunci tunggal; mengandalkan kompleksitas tebakan |

---

## 2. Mengapa Algoritma Cepat (MD5, SHA-1, SHA-256) Tidak Boleh untuk Kata Sandi?
Fungsi hash seperti MD5 dan SHA-256 dirancang untuk memproses data berukuran gigabyte secepat mungkin demi verifikasi integritas file. Karakteristik "cepat" ini justru menjadi kelemahan fatal ketika digunakan untuk kata sandi:
- **Kecepatan Komputasi GPU Modern**: Satu rig GPU workstation dapat menghitung lebih dari 100 miliar hash MD5 per detik.
- **Biaya Brute-Force yang Rendah**: Kata sandi 8 karakter huruf kecil dapat dihitung seluruh kemungkinan kombinasinya dalam hitungan menit.
- **Kerapuhan Kolisi (*Collision Attacks*)**: MD5 telah dibuktikan rentan terhadap kolisi sejak tahun 2004 (Wang dkk.), dan SHA-1 telah berhasil dipatahkan pada tahun 2017 (Google SHAttered attack).

---

## 3. Bahaya Hashing Deterministik & Rainbow Tables
Tanpa garam pengacak (*salt*), fungsi hash bersifat **deterministik murni**: input yang sama akan selalu menghasilkan output yang sama persis:
$$\text{MD5}(\text{"admin"}) = \text{21232f297a57a5a743894a0e4a801fc3}$$

Penyerang memanfaatkan sifat ini dengan membangun **Rainbow Tables** (tabel pencarian terkompresi berukuran terabyte) yang berisi ratusan miliar hash dari kata sandi umum. Ketika basis data dicuri, penyerang tidak perlu melakukan komputasi ulang—mereka cukup melakukan query indeks seperti membuka kamus.

---

## 4. Mekanisme Salting & Algoritma Adaptif
Untuk menghentikan serangan rainbow table dan brute-force akselerasi GPU:

### A. Cryptographic Salt
String acak yang unik (minimal 16 byte dari CSPRNG) digabungkan dengan kata sandi sebelum proses hashing:
$$\text{Hash} = \text{Algorithm}(\text{Password} + \text{Unique\_Salt})$$
Meskipun 1.000 pengguna menggunakan kata sandi `"password123"`, setiap pengguna akan memiliki nilai hash yang berbeda total karena salt masing-masing unik. Rainbow table menjadi 100% tidak berguna.

### B. Adaptive Memory-Hard Algorithms
Algoritma modern memaksa komputer menggunakan memori RAM yang besar dan ribuan putaran iterasi matematis untuk memperlambat serangan perangkat keras:
1. **Argon2id**: Pemenang kompetisi Password Hashing Competition (PHC). Kebal terhadap serangan GPU ASIC karena menuntut alokasi memori dinamis (*memory-hard*).
2. **bcrypt**: Berbasis cipher Blowfish dengan faktor kerja adaptif (*cost factor*). Nilai cost 12 membutuhkan ~250ms waktu CPU per verifikasi.
3. **PBKDF2**: Standar NIST yang menerapkan HMAC berulang kali (minimal 600.000 iterasi untuk SHA-256).

---

## 5. Authenticated Encryption with Associated Data (AEAD)
Untuk enkripsi data saat istirahat (*data at rest*):
- **Hindari Mode ECB / CBC Klasik**: Rentan terhadap *padding oracle attacks* dan modifikasi bit data di tengah jalan (*bit-flipping*).
- **Gunakan AEAD (AES-256-GCM atau ChaCha20-Poly1305)**: Menyediakan enkripsi sekaligus tag autentikasi integritas. Jika penyerang mengubah satu bit saja dari data terenkripsi, proses dekripsi akan langsung gagal secara aman.
- **Wajib Nonce / IV Unik**: Jangan pernah menggunakan kembali Nonce (*Initialization Vector*) dengan kunci AES-GCM yang sama, karena akan membocorkan kunci enkripsi internal.
