# Analogi Dunia Nyata: Pintu Baja Brankas dan Dinding Papan Gipsum

Untuk memahami mengapa Insecure Design tidak bisa diperbaiki hanya dengan memperbagus sintaksis kode, bayangkan sebuah bank yang membangun ruang penyimpanan emas batangan:

---

### 1. Pintu Brankas Terbaik di Dunia (Kode Implementasi Sempurna)
Direksi bank menginvestasikan miliaran rupiah untuk membeli pintu brankas baja terbaik dari pabrik ternama:
- Pintu terbuat dari baja padat berbobot 10 ton tahan ledakan dinamit.
- Mekanisme kunci menggunakan kombinasi dial biometrik ganda dan pemindai retina.
- Engsel pintu dilumasi dengan sempurna dan tidak pernah macet (*zero implementation bugs*).

---

### 2. Kesalahan Gambar Arsitektur Gedung (Insecure Design)
Saat arsitek gedung menggambar cetak biru bangunan, demi menghemat anggaran dan mempercepat penyelesaian proyek:
- Arsitek merancang dinding pembatas brankas bukan dari cor beton bertulang tebal, melainkan hanya menggunakan rangka kayu dan **papan gipsum tipis (drywall)** setebal 1 sentimeter.
- Tidak ada satu pun insinyur yang menanyakan: *"Bagaimana jika pencuri tidak mencoba membobol pintu baja, melainkan menyerang dindingnya?"* (*ketiadaan threat modeling*).

---

### 3. Eksploitasi Tanpa Perlawanan
Suatu malam, seorang pencuri masuk ke gedung bank:
1. Pencuri melihat pintu baja 10 ton yang kokoh dan langsung mengabaikannya karena tahu pintu itu mustahil dibobol.
2. Pencuri berjalan 2 langkah ke sisi samping pintu, mengeluarkan pisau saku sederhana dari sakunya, lalu mengerat papan gipsum dalam waktu 15 detik.
3. Pencuri melangkah masuk melalui lubang dinding dan mengangkut seluruh emas batangan dengan santai.

---

### 4. Evaluasi Pasca-Insiden
Ketika polisi dan auditor datang:
- Pintu baja brankas diperiksa: sistem sensornya menyatakan pintu tetap terkunci 100% aman dan bekerja sesuai spesifikasi pabrik.
- Tidak ada baut yang longgar atau kabel yang putus pada pintu brankas.
- Namun seluruh bank tetap bangkrut karena emasnya telah lenyap.

---

### Pesan Utama Keamanan
Dalam rekayasa sistem:
- Menulis kode yang bebas dari bug memori atau syntax error adalah seperti membeli pintu baja 10 ton.
- Namun jika arsitektur alur bisnis Anda mengizinkan klien melompati tahap persetujuan, Anda sedang membangun brankas dengan dinding papan gipsum.
