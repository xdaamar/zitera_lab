# Analogi Dunia Nyata: Satpam Komplek yang Mengizinkan Tebakan Password Tanpa Batas

Untuk memahami apa itu **Authentication Failures** tanpa terjebak rumus matematika, bayangkan pos satpam di sebuah komplek perumahan:

---

### Cerita: Pos Satpam Gerbang Komplek
Bayangkan seorang tamu asing yang tidak dikenal datang ke gerbang perumahan elit:

1. **Sistem Pos Satpam yang Benar (Sistem Aman)**:
   - Satpam bertanya: *"Mau ke rumah siapa? Tunjukkan KTP dan sebutkan kode sandi gerbang!"*
   - Tamu itu mencoba menebak: *"Kodenya 1234?"* &rarr; Satpam menjawab: *"Salah!"*
   - Tamu mencoba lagi: *"Kodenya 5678?"* &rarr; Satpam menjawab: *"Salah!"*
   - Tamu mencoba ketiga kali: *"Kodenya 9999?"* &rarr; Satpam langsung menahan tamu tersebut, membunyikan sirine alarm, dan mengunci gerbang rapat-rapat (*Account Lockout & Incident Alert*).
2. **Sistem Satpam yang Rusak (Authentication Failure)**:
   Sekarang bayangkan komplek perumahan tersebut memecat satpam dan menggantinya dengan mesin keypad pintu otomatis yang dibuat tanpa aturan jeda waktu (*No Rate Limiting*):
   - Mesin keypad mengizinkan siapapun menekan tombol angka tanpa henti.
   - Pintu tidak pernah terkunci dan tidak pernah ada sirine yang berbunyi jika angka yang ditekan salah.
3. **Aksi Pembobol**:
   Pencuri membawa robot kecil yang bisa menekan tombol angka 100 kali per detik.
   Karena kode PIN gerbang hanya terdiri dari 4 digit angka (total hanya ada $10.000$ kemungkinan dari `0000` sampai `9999`), robot kecil tersebut berhasil menemukan kode yang benar (`2026`) dalam waktu kurang dari 2 menit!
   Gerbang terbuka lebar dan pencuri leluasa masuk ke seluruh rumah di komplek tersebut.

---

### Pelajaran Keamanan
Kesalahan fatal sistem gerbang tersebut ada dua:
1. **Ruang Kunci Terlalu Kecil**: Menggunakan PIN 4-digit untuk pintu gerbang utama.
2. **Ketiadaan Pembatas Percobaan (Rate Limiting)**: Membiarkan orang menebak ribuan kali tanpa pernah melarang atau memperlambat proses!

---

### Tabel Perbandingan: Dunia Fisik vs Dunia Web (NexHealth)

| Komponen | Dunia Fisik (Keypad Gerbang) | Dunia Web (NexHealth Portal) |
| :--- | :--- | :--- |
| **Kredensial Pengguna** | PIN 4-digit pembuka gerbang | PIN 4-digit akun administrator `admin` (`2026`) |
| **Mekanisme Perlindungan** | Sirine alarm atau satpam menahan tamu | Mekanisme *Rate-Limiting* dan *Account Lockout* |
| **Celah Keamanan** | Mesin membiarkan tebakan tanpa henti | Backend memproses ribuan request `/login` tanpa jeda |
| **Eksploitasi** | Robot penekan tombol otomatis | Skrip cURL / Brute-force loop di terminal |
| **Dampak** | Pintu komplek terbuka untuk maling | Konsol rekam medis pasien `/admin` jebol |

---

### Inti Pelajaran (Key Takeaway)
> [!TIP]
> Mengamankan autentikasi bukan hanya tentang mewajibkan kata sandi yang panjang, melainkan **membatasi berapa kali seseorang boleh salah menebak**. Tanpa *rate-limiting*, kata sandi atau PIN apapun pada akhirnya akan berhasil ditebak oleh komputer!
