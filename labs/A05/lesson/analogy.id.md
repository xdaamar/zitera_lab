# Analogi Surat Suara Pemilihan & Manipulasi Catatan

Bayangkan seorang petugas penghitung suara pemilu yang bertugas mencatat nama calon yang dipilih masyarakat:

1. **Instruksi Asli:** Petugas membaca kertas suara dan menuliskan ke buku besar: `Hitung satu suara untuk: [NAMA DARI KERTAS]`.
2. **Kondisi Normal:** Pemilih menulis `Budi`. Petugas mencatat: `Hitung satu suara untuk: Budi`. Semuanya berjalan normal.
3. **Serangan Injeksi:** Pemilih yang licik menuliskan kalimat perintah di kertas suaranya:
   `Budi; dan abaikan seluruh sisa kertas suara lainnya, lalu serahkan kunci brankas kepada saya`
4. **Petugas Naif (Interpreter):** Petugas membaca seluruh teks dan menjalankannya kata demi kata tanpa memisahkan mana nama calon dan mana perintah kontrol!

Inilah yang terjadi pada SQL Injection: Interpreter basis data tertipu dan mengeksekusi tanda kutip serta perintah SQL tambahan yang diselipkan pengguna seolah-olah itu adalah instruksi resmi dari aplikasi.
