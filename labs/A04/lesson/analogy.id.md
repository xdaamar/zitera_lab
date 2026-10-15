# Analogi Dunia Nyata: Kunci Koper Plastik vs Brankas Bank Baja

Untuk mengilustrasikan perbedaan antara kriptografi yang rusak dan kriptografi modern, bayangkan cara mengamankan formula resep rahasia yang bernilai miliaran rupiah:

---

### 1. Teks Terbuka / Cleartext (Kertas di Atas Meja Tamu)
Menyimpan data sensitif dalam bentuk teks terbuka sama seperti menulis formula rahasia tersebut pada selembar kertas memo dan meletakkannya begitu saja di meja lobi kantor yang dilalui ratusan tamu setiap hari. Siapa pun yang melintas dapat langsung membaca, memotret, atau menyalin seluruh resep tanpa hambatan.

---

### 2. Hash MD5 Tanpa Salt (Gembok Koper Plastik 3 Angka)
Anda memutuskan untuk memasukkan resep tersebut ke dalam tas koper kecil, namun Anda menguncinya dengan gembok kombinasi plastik 3 angka murah:
- **Keterbatasan Kombinasi**: Hanya ada 1.000 kemungkinan angka (000 hingga 999). Seseorang yang memegang koper itu dapat mencoba seluruh kombinasi dalam waktu kurang dari 3 menit sambil duduk santai di ruang tunggu.
- **Ketiadaan Salt (Cetakan Pabrik Identik)**: Setiap koper yang dijual pabrik memiliki kode pembuka bawaan yang sama. Penjahat bahkan tidak perlu memutar angka; mereka hanya perlu membeli buku manual pabrik untuk langsung membuka kopernya.

Ini adalah gambaran nyata fungsi hash cepat seperti MD5: karena deterministik dan sangat ringan, komputer modern dapat mencoba miliaran kemungkinan dalam sekejap atau menggunakan kamus rainbow table.

---

### 3. Kriptografi Kuat dengan Salt & Memory-Hard (Brankas Baja Bank)
Kini Anda mengamankan resep tersebut di dalam lemari besi brankas bank modern:
- **Salt Kriptografis (Cetakan Kunci Unik)**: Setiap brankas ditempa dengan gerigi kunci fisik yang sama sekali berbeda dan dibuat khusus per pelanggan. Buku manual pabrik tidak ada gunanya.
- **Argon2 / Work Factor (Mekanisme Putar Berbobot Berat)**: Setiap kali seseorang memutar dial kombinasi, mekanisme hidrolik internal sengaja membutuhkan waktu 30 detik untuk berputar secara presisi sebelum memeriksa angka berikutnya.
- **Ketahanan Terhadap Pencurian Skala Besar**: Jika seorang pencuri ingin menebak kombinasi 10 juta kemungkinan angka, mereka membutuhkan waktu ratusan tahun karena setiap satu kali tebakan dipaksa berjalan sangat lambat.

---

### Pesan Utama Keamanan
Jangan pernah merasa aman hanya karena data Anda terlihat seperti "string acak". Jika string acak tersebut dihasilkan oleh algoritma kuno yang cepat dan tanpa salt, ilusi keamanan itu akan hancur dalam hitungan detik.
