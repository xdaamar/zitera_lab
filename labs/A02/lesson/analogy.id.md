# Analogi Dunia Nyata: Rumah Mewah dengan Kode PIN Bawaan Toko

Untuk memahami esensi Security Misconfiguration, bayangkan sebuah rumah pintar (*smart home*) berteknologi tinggi yang baru saja selesai dibangun:

---

### 1. Sistem Penguncian Canggih tapi Kredensial Bawaan
- Pemilik rumah membeli gembok digital termahal di pasaran yang tahan benturan dan dilengkapi pemindai biometrik.
- Di dalam kardus kemasan, pabrikan memasang kode PIN darurat bawaan: **`1234`**, disertai buku petunjuk bertuliskan: *"Ganti kode PIN ini segera setelah pemasangan selesai"*.
- Karena terburu-buru atau merasa kawasan perumahannya aman, pemilik rumah **tidak pernah mengganti PIN bawaan `1234`** tersebut.

---

### 2. Pintu Samping yang Diganjal Batu (Fitur Debugging Berlebih)
- Selama proses renovasi, para pekerja bangunan mengganjal pintu garasi samping menggunakan batu bata agar mereka mudah keluar masuk membawa material tanpa perlu membuka kunci berulang-ulang.
- Ketika renovasi selesai, batu bata tersebut tidak pernah disingkirkan, dan pintu garasi samping dibiarkan terbuka sedikit siang dan malam.

---

### 3. Eksploitasi oleh Penyusup
Seorang pencuri amatir berjalan menyusuri kompleks perumahan:
1. Pencuri tidak perlu repot-repot membawa alat las pemotong baja, bor listrik, atau dinamit untuk membobol dinding rumah.
2. Pencuri cukup berjalan santai ke panel pintu depan dan menekan kombinasi angka paling umum di dunia: **`1234`** â€” dan pintu langsung terbuka lebar!
3. Jika pintu depan terkunci, pencuri cukup melangkah ke samping rumah dan menemukan pintu garasi samping yang diganjal batu, di mana seluruh dokumen asuransi dan kunci cadangan mobil tersimpan rapi di rak terbuka.

---

### Pelajaran Keamanan
Berapa pun mahalnya teknologi atau framework yang kamu gunakan (meskipun menggunakan enkripsi AES-256 militer atau tumpukan cloud tercanggih), jika konfigurasi dasarnya dibiarkan menggunakan setelan bawaan pabrik yang tidak aman, sistem tersebut tetap sangat rapuh dan dapat ditembus dengan usaha minimal.
