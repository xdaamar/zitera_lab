# A06: Insecure Design (Cacat Desain & Arsitektur)

## 1. Apa itu Insecure Design?
Dalam klasifikasi OWASP Top 10:2025, **Insecure Design** merepresentasikan kategori risiko yang berfokus pada cacat mendasar dalam arsitektur sistem, pemodelan ancaman (*threat modeling*), dan perancangan logika bisnis.

Kategori ini secara tegas membedakan antara **Cacat Desain (*Design Flaws*)** dan **Cacat Implementasi (*Implementation Bugs*)**:
- **Cacat Implementasi**: Arsitektur dan alur logikanya sudah dirancang dengan benar dan aman, namun pengembang melakukan kesalahan teknis saat menulis kode (misalnya: lupa melakukan sanitasi query SQL, salah mengetik variabel hak akses, atau salah konfigurasi firewall).
- **Cacat Desain (Insecure Design)**: Kode program ditulis dengan sangat rapi, bersih dari bug sintaksis, dan lulus uji unit teknis 100%. Namun, sejak awal konsep arsitektur, asumsi batas kepercayaan (*trust boundary*), atau alur proses bisnisnya memang cacat. Tidak ada jumlah penulisan kode yang sempurna yang dapat memperbaiki arsitektur yang sejak awal dirancang secara tidak aman.

## 2. Mengapa Insecure Design Menjadi Ancaman Kritis di 2025/2026?
Dalam arsitektur perangkat lunak modern yang mengadopsi microservices, event-driven workflows, dan sistem terdistribusi:
- **Asumsi Kepercayaan Implisit (*Implicit Trust Traps*)**: Layanan hulu sering kali berasumsi bahwa data atau status yang dikirimkan oleh klien atau antarmuka web selalu valid dan jujur.
- **Bypass Mesin Status (*State Machine Skipping*)**: Alur bisnis multi-tahap (seperti pengadaan barang, persetujuan pinjaman, atau pembayaran *e-commerce*) sering kali gagal mempertahankan invarian status, memungkinkan penyerang melompati gerbang persetujuan kritis.
- **Ketiadaan Validasi Batas Bisnis (*Missing Business Logic Controls*)**: Pemeriksaan parameter hanya dilakukan di lapisan UI (frontend), sementara endpoint API backend menerima transisi status apa pun secara mentah.

## 3. Insiden Nyata Dunia Industri
- **E-Commerce Price & Coupon Manipulation**: Cacat logika checkout di mana diskon kupon dapat dipicu berkali-kali pada satu keranjang belanja atau parameter harga barang dapat diubah menjadi nilai negatif sehingga saldo pengguna justru bertambah saat checkout.
- **Procurement & Wire Transfer State Bypasses**: Pada beberapa platform pengadaan barang enterprise, penyerang mengubah parameter `order_status` dari `DRAFT` langsung menjadi `DISPATCHED` atau `APPROVED`, mengakibatkan transfer dana jutaan dolar tanpa otorisasi manajer keuangan.
- **Password Reset Security Questions**: Penggunaan pertanyaan keamanan statis (*"Apa nama hewan peliharaan pertamamu?"*) sebagai satu-satunya faktor pemulihan akun tanpa verifikasi identitas kriptografis.

## 4. Sasaran Pembelajaran di Lab A06
Melalui laboratorium sistem pengadaan barang *Zitera Procurement Portal* pada port 8016, Anda akan:
1. Membedakan antara kerentanan implementasi dan cacat arsitektur logika bisnis.
2. Menganalisis alur mesin status pengadaan (*state machine transitions*) dan mengidentifikasi titik lemah kontrol persetujuan.
3. Membuktikan bagaimana manipulasi payload API transisi status memungkinkan pesanan bernilai tinggi melompati tahap persetujuan manajer.
4. Merekonstruksi status sistem untuk mengambil verifikasi dan flag tantangan berformat `ZITERA{...}`.
5. Merancang arsitektur mesin status yang aman (*Enforced State Machine Invariants*) dengan prinsip *Zero Trust* di tingkat backend.
