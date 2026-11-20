# A12: Panduan Langkah Pengujian (Walkthrough)

Ikuti langkah-langkah terstruktur berikut untuk menyelesaikan misi laboratorium:

### Langkah 1: Jalankan Laboratorium
1. Klik tombol **Start Lab** pada dashboard ZITERA LAB.
2. Tunggu indikator status berubah menjadi **RUNNING** hijau dengan URL target `http://127.0.0.1:8035/`.
3. Klik **Open Browser** untuk membuka antarmuka aplikasi target.

### Langkah 2: Eksplorasi Fitur Target
1. Amati form pengujian dan endpoint yang tersedia pada antarmuka web.
2. Perhatikan parameter yang dikirimkan saat Anda menekan tombol pengujian.

### Langkah 3: Eksekusi Serangan & Dapatkan Flag
1. Gunakan opsi pengujian atau tombol **Trigger Exploit Langsung** pada aplikasi untuk memicu respons kerentanan sistem.
2. Sistem target akan mengekstraksi flag rahasia dengan format:
   `ZITERA{oauth2_state_param_hijack_2026}`
3. Salin string flag tersebut dengan lengkap.

### Langkah 4: Kirimkan Flag CTF
1. Buka kembali aplikasi ZITERA LAB dan pilih tab **Challenge** pada halaman lab `A12`.
2. Tempelkan flag pada kolom isian dan klik **Kirim Flag CTF yang Ditemukan**.
3. Status lab akan berubah menjadi **SOLVED**!
