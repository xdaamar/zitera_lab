# Analogi Dunia Nyata: Kartu Kunci Hotel & Nomor Penitipan Koper

Untuk memahami mengapa **Broken Access Control** dan **IDOR** terjadi secara intuitif, mari kita bayangkan dua peristiwa yang sering kita jumpai dalam kehidupan sehari-hari:

---

### Cerita 1: Kunci Kamar Hotel Berbintang
Bayangkan kamu sedang menginap di sebuah hotel berbintang di pusat kota:

1. **Autentikasi di Meja Resepsionis**:
   Kamu tiba di lobi hotel dan menunjukkan KTP serta bukti pembayaran reservasi. Resepsionis memvalidasi identitasmu di sistem komputer. Setelah identitas terbukti sah (*Authentication Sukses*), resepsionis menyerahkan kartu kunci bertuliskan **Kamar #302**.
2. **Kontrol Akses yang Benar**:
   Kamu naik ke lantai 3 dan menempelkan kartu ke pintu kamar #302. Kunci elektronik berbunyi *klik* dan pintu terbuka. Jika kamu mencoba menempelkan kartu kuncimu ke pintu **Kamar #303** milik tamu lain, sensor kunci menyala merah dan menolak akses (*Authorization Berhasil Ditegakkan*).
3. **Celah Keamanan Digital (IDOR)**:
   Sekarang bayangkan jika hotel tersebut mengganti kunci fisik dengan aplikasi smartphone modern yang dibuat asal-asalan. Di layar ponsel terdapat kotak teks:
   `"Masukkan nomor kamar yang ingin kamu buka: [ 302 ]"`
   Secara default terisi 302. Namun, jika kamu iseng mengganti angka tersebut menjadi **`303`** atau **`101`** (Kamar Presidential Suite), pintu kamar tamu lain langsung terbuka!
   Hotel tahu siapa dirimu (kamu tamu terdaftar), tetapi sistem pintu percaya begitu saja pada angka yang kamu ketik di ponsel tanpa mengecek apakah kamu penyewa sah kamar tersebut!

---

### Cerita 2: Nomor Penitipan Koper di Stasiun Kereta
Bayangkan kamu menitipkan koper ranselmu di loket penitipan stasiun:
- Petugas menempelkan stiker bernomor **#101** pada ranselmu dan memberikan kartu nomor antrean **#101** ke tanganmu.
- Di sebelah ranselmu, ada koper koper mahal milik pengusaha eksekutif dengan nomor antrean **#42**.
- Saat hendak mengambil barang, kamu iseng berkata kepada petugas: *"Tolong ambilkan barang nomor 42 ya mas."*
- Petugas loket yang polos langsung mengambil koper nomor 42 dan menyerahkannya kepadamu tanpa meminta kartu bukti nomor antreanmu!

---

### Tabel Perbandingan: Dunia Fisik vs Dunia Web (VaultPay)

| Unsur Sistem | Dunia Fisik (Penitipan Koper / Hotel) | Dunia Web (VaultPay Enterprise) |
| :--- | :--- | :--- |
| **Identitas (AuthN)** | Menunjukkan KTP sah ke petugas loket / resepsionis | Login sukses sebagai user `alice` dengan password yang benar |
| **Objek Data** | Koper nomor 101 milikmu, atau koper nomor 42 milik orang lain | Voucher transfer `/wire/1` milik Alice, atau `/wire/42` milik eksekutif |
| **Permintaan (Request)** | Kamu menyebutkan: *"Ambilkan koper nomor 42"* | Browser mengirim HTTP request: `GET /wire/42` |
| **Celah IDOR** | Petugas tidak memeriksa apakah kamu pemegang kupon 42 | Server tidak memeriksa apakah sesi login Alice memiliki voucher 42 |
| **Dampak Kebocoran** | Koper orang lain terbawa pulang oleh orang asing | Data finansial dan rahasia perusahaan lain bocor ke pihak tidak berhak |

---

### Inti Pelajaran (Key Takeaway)
> [!TIP]
> **Autentikasi** membuktikan bahwa kamu adalah tamu yang sah.  
> **Otorisasi** memastikan kamu hanya boleh membuka pintu kamarmu sendiri, bukan pintu kamar tamu lain!  
> Kerentanan IDOR terjadi bukan karena peretas membobol password, melainkan karena sistem server lupa mengecek kepemilikan objek yang diminta.
