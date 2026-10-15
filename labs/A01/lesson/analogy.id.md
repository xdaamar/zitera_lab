# Analogi Dunia Nyata: Kartu Kunci Hotel & Nomor Penitipan Koper

Untuk memahami mengapa Broken Access Control terjadi secara intuitif, bayangkan sebuah pengalaman saat menginap di hotel berbintang:

---

### Tahap 1: Autentikasi di Meja Resepsionis
1. Kamu tiba di lobi hotel dan menunjukkan KTP serta bukti pemesanan kamar kepada staf resepsionis.
2. Resepsionis memvalidasi identitasmu di sistem komputer hotel dan memastikan bahwa kamu memang telah memesan kamar.
3. Setelah identitas terbukti sah (*Authentication Sukses*), resepsionis memberikan kamu sebuah kartu kunci kamar bertuliskan: **Kamar #302**.

---

### Tahap 2: Kontrol Akses yang Benar (Sistem Ideal)
- Kamu naik ke lantai 3 dan menempelkan kartu kuncimu ke pintu kamar #302. Kunci elektronik berbunyi klik dan pintu terbuka hijau.
- Jika kamu mencoba menempelkan kartu kuncimu ke pintu **Kamar #303** milik tamu lain, sensor kunci langsung menyala merah dan tetap terkunci rapat.
- Sistem pintu memeriksa secara fisik apakah chip kartu tersebut memiliki izin untuk mengakses kamar #303 (*Authorization Berhasil Ditegakkan*).

---

### Tahap 3: Celah Broken Access Control (Skenario Kerentanan)
Sekarang bayangkan jika hotel tersebut memasang sistem kunci digital "canggih" berbasis aplikasi ponsel, tetapi dengan cacat desain fatal:
1. Setiap kali kamu berdiri di depan pintu kamar mana pun, aplikasi di ponselmu menampilkan kotak input:
   `"Kamar nomor berapa yang ingin kamu buka?"`
2. Aplikasi ponselmu secara default mengisikan angka `302`.
3. Namun, jika kamu sengaja mengubah angka di aplikasi tersebut menjadi **`303`** atau **`101`** (Kamar Presidential Suite), kunci pintu langsung berbunyi klik dan terbuka!

---

### Pelajaran Keamanan
Resepsionis hotel telah berhasil memvalidasi siapa dirimu (kamu adalah tamu terdaftar yang sah), namun kunci pintu kamar melakukan **kesalahan fatal** dengan mempercayai nomor kamar yang dikirimkan oleh ponselmu tanpa mencocokkan ke database pusat apakah kamu adalah penyewa sah kamar tersebut.

Inilah gambaran tepat dari **IDOR**:
Aplikasi web mengetahui bahwa kamu adalah pengguna yang sah (*Alice yang sudah login*), tetapi server secara ceroboh mengizinkan kamu membuka data milik pengguna lain (*faktur nomor 2 milik Bob*) hanya karena kamu mengubah nomor ID pada alamat URL peramban.
