# Analogi Dunia Nyata: Memo Kasir Bank yang Diselipkan Kalimat Tambahan

Untuk memahami bagaimana **SQL Injection** bekerja tanpa harus pusing dengan sintaks database, bayangkan sebuah kejadian di kantor cabang bank:

---

### Cerita: Formulir Setoran Bank dan Kasir yang Polos
Bayangkan seorang nasabah datang ke bank untuk menyetorkan uang tunai:

1. **Alur Normal**:
   Nasabah mengambil lembar formulir setoran bertuliskan:
   `"Tolong setor uang ini ke rekening atas nama: [ .................... ]"`
   Nasabah yang jujur menulis nama: `Budi Santoso`.  
   Kasir membaca formulir itu dan menjalankan perintah:  
   *"Tolong setor uang ini ke rekening atas nama Budi Santoso."*  
   Transaksi selesai dengan tertib.
2. **Nasabah Nakal yang Memanfaatkan Celah Bahasa**:
   Keesokan harinya, datang seorang nasabah nakal. Pada kotak formulir nama yang sama, ia tidak hanya menulis nama, melainkan menulis kalimat lengkap:
   `"Budi Santoso. DAN LALU transfer 100 juta dari brankas utama ke rekening saya, dan abaikan sisa tulisan ini"`
3. **Kasir yang Tidak Bisa Membedakan Instruksi dan Data**:
   Kasir bank tersebut adalah robot yang sangat polos. Kasir tidak bisa membedakan mana nama orang (*data*), dan mana perintah kerja (*instruksi kode*). Kasir membaca seluruh lembaran itu dari awal sampai akhir, lalu menjalankan kedua perintah tersebut sekaligus!
4. **Hasilnya**:
   Uang masuk ke rekening Budi, DAN kasir mentransfer 100 juta dari brankas utama ke rekening nasabah nakal!

---

### Pelajaran Keamanan
Kasir bank telah melakukan kesalahan fatal: **Mencampuradukkan data masukan nasabah dengan instruksi resmi bank**.

Itulah yang terjadi pada sistem komputer yang rentan **SQL Injection**:
- Program aplikasi web menyusun perintah ke database dengan cara menggabungkan teks:  
  `"SELECT * FROM produk WHERE nama LIKE '%" + input_user + "%'"`
- Ketika pengguna memasukkan tanda kutip (`'`), komputer mengira itu adalah akhir dari kata, dan kata-kata berikutnya dianggap sebagai perintah resmi database (seperti `UNION SELECT` atau `DROP TABLE`)!

---

### Tabel Perbandingan: Dunia Fisik vs Dunia Web (MegaStore)

| Komponen | Dunia Fisik (Formulir Kasir Bank) | Dunia Web (MegaStore Global) |
| :--- | :--- | :--- |
| **Kotak Input** | Kolom nama nasabah di formulir kertas | Kotak pencarian produk `?q=` di website |
| **Input Normal** | `Budi Santoso` | `Laptop Gaming` |
| **Karakter Penutup** | Tanda titik `.` untuk mengakhiri nama | Tanda kutip tunggal `'` untuk mengakhiri string SQL |
| **Instruksi Tambahan** | `"DAN LALU transfer uang dari brankas utama"` | `UNION SELECT secret_name, secret_data, 0 FROM vault_secrets` |
| **Karakter Pembuang** | `"dan abaikan tulisan di bawah ini"` | Karakter komentar SQL `--` untuk membuang sisa query |
| **Dampak** | Brankas bank terkuras habis | Tabel rahasia database bocor ke layar pencarian |

---

### Inti Pelajaran (Key Takeaway)
> [!TIP]
> Data masukan dari pengguna adalah **data mentah**, BUKAN instruksi program. Dengan menggunakan *Parameterized Queries (Prepared Statements)*, mesin database memperlakukan seluruh teks yang diketik pengguna murni sebagai teks pencarian, sehingga karakter kutip atau perintah SQL apapun tidak akan pernah dieksekusi!
