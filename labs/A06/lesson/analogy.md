# Analogi Dunia Nyata: Mesin Kasir Toko yang Tombol "Lunas" Bisa Ditekan Pembeli

Untuk memahami apa itu **Insecure Design** tanpa kebingungan teknis, mari kita gunakan analogi kasir toko elektronik:

---

### Cerita: Toko Elektronik Mewah dengan Kasir Otomatis
Bayangkan sebuah toko gadget mewah merancang sistem kasir mandiri (*Self-Checkout Counter*):

1. **Aturan Bisnis Resmi**:
   Pemilik toko menetapkan aturan:
   - Barang seharga di bawah 500 ribu (misal: casing HP, kabel charger) boleh langsung dibayar di mesin kasir tanpa pengawasan staf.
   - Barang mahal di atas 50 juta (misal: Laptop Gaming Tier-0, Server Rak) **wajib diperiksa oleh Manajer Toko** sebelum diserahkan ke pembeli. Manajer harus memeriksa kartu identitas dan bukti transfer bank.
2. **Cacat Desain Mesin Kasir**:
   Arsitek sistem yang merancang mesin kasir membuat layar sentuh dengan 4 tombol status:
   `[ 1. DRAFT ]` $\rightarrow$ `[ 2. TINJAUAN MANAJER ]` $\rightarrow$ `[ 3. KLIRING BANK ]` $\rightarrow$ `[ 4. SUDAH LUNAS / DISPATCHED ]`
   Namun, arsitek tersebut lupa membuat tombol nomor 4 terkunci untuk pembeli! Tombol nomor 4 dibiarkan menyala hijau di layar sentuh pembeli.
3. **Aksi Pembeli Nakal**:
   Seorang pembeli memilih server komputer seharga $50,000 (sekitar 800 juta rupiah). Layar kasir menampilkan status: `DRAFT`.
   Tanpa memanggil manajer toko dan tanpa mentransfer uang sepeser pun, pembeli itu langsung menekan tombol layar sentuh nomor 4: **`[ SUDAH LUNAS / DISPATCHED ]`**!
4. **Hasilnya**:
   Mesin kasir mencetak struk bertuliskan: *"Barang lunas! Silakan bawa pulang barang Anda."* Petugas keamanan di pintu keluar melihat struk bertuliskan "Lunas" dan mempersilakan pembeli itu keluar dengan membawa server mahal tersebut!

---

### Pelajaran Keamanan
Bukan mesin kasirnya yang rusak secara fisik (tidak ada kabel yang konslet, tidak ada kaca yang pecah). Sistem berfungsi 100% sesuai instruksi perangkat lunak.
Namun, **desain logika alur kerjanya yang cacat secara fundamental**: Mengizinkan pembeli menentukan sendiri status pesanannya!

---

### Tabel Perbandingan: Dunia Fisik vs Dunia Web (Zitera Procurement)

| Unsur Alur | Dunia Fisik (Mesin Kasir Toko) | Dunia Web (Zitera Procurement) |
| :--- | :--- | :--- |
| **Aset Bernilai Tinggi** | Server komputer rak seharga $50,000 | Permohonan Order `#9999` ($50,000) status `DRAFT` |
| **Aturan Kebijakan Bisnis** | Wajib verifikasi fisik identitas oleh Manajer | Pesanan $\ge \$500$ wajib status `MANAGER_APPROVED` |
| **Cacat Desain** | Tombol "Lunas" di layar sentuh bebas ditekan pembeli | Endpoint `/api/orders/<id>/transition` menerima input klien |
| **Eksploitasi** | Pembeli menekan tombol "Lunas" langsung | Klien mengirim JSON `{"target_state": "DISPATCHED"}` |
| **Dampak** | Barang dibawa kabur tanpa pembayaran | Sistem menyetujui pengiriman dan membuka Flag rahasia |

---

### Inti Pelajaran (Key Takeaway)
> [!TIP]
> Keamanan yang kuat harus dirancang sejak awal (*Secure by Design*). Jangan pernah mengizinkan klien memilih status berikutnya dalam suatu proses transaksi! Server harus menjadi satu-satunya pihak yang berhak mengubah status berdasarkan aturan bisnis yang ketat.
