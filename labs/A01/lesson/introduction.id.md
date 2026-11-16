# A01: Broken Access Control & IDOR — Primadona Bug Bounty Hunter

Broken Access Control menempati peringkat teratas (**#1**) dalam daftar risiko keamanan web **OWASP Top 10:2025**. Kerentanan ini terjadi ketika sebuah aplikasi web atau API gagal membatasi hak akses pengguna secara ketat di sisi server (*server-side authorization failure*), sehingga pengguna yang sudah login dapat mengakses data, fungsi, atau aset digital milik pengguna lain di luar batas hak akses sahnya.

---

### 💰 Mengapa Ini Menjadi "Tambang Emas" Bug Bounty ($1,500 – $8,000+)?
Di platform bug bounty global seperti **HackerOne**, **Bugcrowd**, dan **Intigriti**, celah Broken Access Control (terutama **IDOR - Insecure Direct Object Reference**) adalah laporan kerentanan yang paling sering diganjar bounty ribuan dolar:
- **Severity Tinggi (P1 / P2 - Critical / High)**: Karena celah ini memungkinkan kebocoran data jutaan pengguna (*mass data exfiltration*) atau pengambilalihan akun secara instan.
- **Dampak Finansial & Hukum Nyata**: Pelanggaran regulasi perlindungan data pribadi (seperti **UU Perlindungan Data Pribadi No. 27/2022** di Indonesia atau **GDPR** internasional) mengancam denda hingga miliaran rupiah dan sanksi operasional berat bagi perusahaan.
- **Eksploitasi Elegan**: Tidak membutuhkan malware atau tool hacking rumit; hanya dengan mengubah satu parameter angka atau UUID pada request HTTP, seluruh data rahasia terbuka.

---

### 🎯 Tujuan Pembelajaran Modul Ini
Setelah menyelesaikan modul ini, kamu diharapkan mampu:
1. **Membentuk Pola Pikir Peretas (Attacker Mindset)**: Memandang web bukan sekadar tombol visual, melainkan aliran parameter data HTTP.
2. **Membedakan AuthN vs AuthZ**: Memahami secara tajam batas antara identitas login (*Authentication*) dan kepemilikan data (*Authorization*).
3. **Mendeteksi Pola IDOR**: Menemukan parameter referensi objek langsung (`id`, `user_id`, `wire_id`, `account_no`) pada request HTTP.
4. **Melakukan Audit Nyata pada ApexFin Treasury**: Menguji kerentanan pada portal perbankan korporat target dan mengekstrak bukti eksploitasi (*Proof of Concept*).
5. **Menerapkan Pertahanan Server-Side**: Menulis kode remediasi yang menerapkan *Object-Level Access Control* yang kebal terhadap serangan tampering.

---

### ⚖️ Memahami Perbedaan Kritis: AuthN vs AuthZ
Banyak programmer pemula keliru mengira aplikasinya sudah aman hanya karena telah memasang login password dan token JWT:

| Aspek Keamanan | Pertanyaan Kunci | Contoh Mekanisme | Titik Kegagalan Umum |
| :--- | :--- | :--- | :--- |
| **Authentication (AuthN)** | *"Siapa kamu?"* | Password, MFA, JWT signature, FaceID | Password lemah, brute force, token forgery |
| **Authorization (AuthZ)** | *"Apa saja hak aksesmu?"* | Role-based ACL, ownership verification | **Broken Access Control**, IDOR, privilege escalation |

> **Prinsip Utama:** Broken Access Control hampir selalu merupakan kegagalan pada lapisan **Authorization**, bukan Authentication. Penyerang biasanya telah berhasil masuk menggunakan akun sah berhak akses rendah, kemudian memanipulasi parameter untuk mengakses data pengguna lain.
