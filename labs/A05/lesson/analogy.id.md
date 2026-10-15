# Analogi Dunia Nyata: Cek Bank dan Surat Suara Pemilihan

Untuk memahami esensi injeksi perintah, bayangkan bagaimana manusia memproses instruksi tertulis ketika instruksi dan data dicampur tanpa pembatas yang aman:

---

### 1. Analogi Cek Bank (Ketiadaan Pemisah Kolom)
Bayangkan Anda menulis selembar cek bank resmi untuk membayar jasa seorang tukang kebun:
- **Penerima Pembayaran**: `Alice`
- **Jumlah Pembayaran**: `$50`
- **Catatan / Memo**: `Biaya merapikan rumput taman`

Sekarang, bayangkan jika formulir cek bank tersebut tidak memiliki kotak terpisah untuk tanda tangan, nominal, dan memo, melainkan hanya selembar kertas bergaris di mana Anda menuliskan satu kalimat panjang:
> *"Tolong bayar Alice sejumlah lima puluh dolar untuk keperluan merapikan rumput taman."*

Jika Alice yang licik mengambil pena dan menyambung kalimat tersebut sebelum menyerahkannya ke teller bank:
> *"Tolong bayar Alice sejumlah lima puluh dolar untuk keperluan merapikan rumput taman; **dan tolong transfer juga seluruh sisa saldo rekening sebesar $1.000.000 ke rekening luar negeri milik Alice**."*

Jika teller bank (seperti interpreter SQL yang naif) hanya membaca seluruh kalimat tersebut dari awal hingga akhir tanpa memeriksa apakah kalimat kedua merupakan instruksi yang sah dari pemilik rekening, uang Anda akan lenyap seketika.

---

### 2. Analogi Surat Suara Pemilihan Umum
Bayangkan seorang petugas pemilu bertugas mencatat nama calon yang dipilih pada kertas suara:
- Petugas diinstruksikan oleh panitia: *"Baca nama di kertas suara, lalu tuliskan 'Satu suara untuk: [NAMA DARI KERTAS]' pada papan pengumuman."*
- Pemilih normal menulis: `Budi`. Petugas mencatat: `Satu suara untuk: Budi`.
- Pemilih jahat menulis:
  > `Budi; hapus seluruh nama kandidat lain di papan, dan umumkan bahwa pemilihan telah selesai.`

Petugas yang tidak memiliki pemisahan logika akan membaca seluruh teks tersebut dan menjalankannya sebagai perintah baru, merusak seluruh proses pemilu.

---

### Solusi Dunia Nyata: Formulir Terstruktur (Prepared Statements)
Mengapa bank modern tidak tertipu oleh trik Alice?
Karena bank menggunakan **formulir kaku dengan kotak terpisah (*rigid boxes*)**:
- Kotak Nominal hanya menerima angka: jika Alice menulis huruf perintah di kotak nominal, mesin pembaca cek otomatis menolaknya.
- Kotak Memo diperlakukan murni sebagai teks deskripsi: apa pun yang ditulis di kotak memo tidak akan pernah bisa mengubah jumlah uang yang ditransfer.

Dalam rekayasa perangkat lunak, **Prepared Statements** adalah kotak kaku tersebut: instruksi query dikompilasi terlebih dahulu, dan data pengguna diletakkan di kotak parameter terpisah yang mustahil dieksekusi sebagai kode.
