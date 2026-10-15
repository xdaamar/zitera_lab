# A01: Broken Access Control (Kegagalan Kontrol Akses)

Broken Access Control menempati peringkat teratas (**#1**) dalam daftar risiko keamanan web OWASP Top 10:2025. Kerentanan ini terjadi ketika sebuah aplikasi web atau API gagal membatasi hak akses pengguna secara ketat, sehingga pengguna terotentikasi dapat mengakses data, fungsi, atau aset digital milik entitas lain di luar batas otorisasi yang sah.

---

### Tujuan Pembelajaran
Setelah menyelesaikan modul ini, kamu diharapkan mampu:
1. Menjelaskan perbedaan mendasar antara **Authentication (AuthN)** dan **Authorization (AuthZ)**.
2. Mengidentifikasi pola kerentanan **Insecure Direct Object References (IDOR)** dan **Broken Object Level Authorization (BOLA)** pada request HTTP.
3. Melakukan audit manual terhadap isolasi data multi-penyewa (*multi-tenant boundary*) menggunakan browser developer tools dan cURL.
4. Menerapkan kode defensif (*server-side ownership verification*) untuk menutup celah kontrol akses secara permanen.

---

### Memahami Perbedaan Kritis: AuthN vs AuthZ
Banyak pengembang keliru menganggap sistem mereka aman hanya karena sudah menerapkan mekanisme login (autentikasi) yang kuat:

| Konsep Keamanan | Pertanyaan Kunci | Contoh Mekanisme | Titik Kegagalan Umum |
| :--- | :--- | :--- | :--- |
| **Authentication (AuthN)** | *"Siapa kamu?"* | Password, MFA, JWT signature, biometrik | Kredensial default, brute force, token forgery |
| **Authorization (AuthZ)** | *"Apa saja hak aksesmu?"* | Role-based ACL, ownership check, tenant filtering | **Broken Access Control**, IDOR, privilege escalation |

> **Prinsip Utama:** Broken Access Control hampir selalu merupakan kegagalan pada lapisan **Authorization**, bukan Authentication. Penyerang biasanya telah berhasil masuk menggunakan akun sah berhak akses rendah, kemudian memanipulasi parameter untuk mengakses data pengguna lain.

---

### Mengapa Kerentanan Ini Sangat Berbahaya?
Ketika kontrol akses diabaikan di sisi server:
- **Pelanggaran Batas Multi-Tenant:** Pengguna pada Tenant A dapat membaca faktur, laporan transaksi, dan data rahasia Tenant B.
- **Eksfiltrasi Data Massal:** Penyerang dapat melakukan iterasi sederhana (misalnya `id=1, 2, 3...`) untuk menyedot seluruh database pelanggan secara otomatis.
- **Eskalasi Hak Istimewa (*Privilege Escalation*):** Pengguna biasa dapat memanggil fungsi administratif dengan sekadar mengubah URL atau parameter metode HTTP.
- **Pelanggaran Kepatuhan Regulasi:** Mengakibatkan kegagalan kepatuhan regulasi perlindungan data pribadi (UU PDP No. 27/2022 di Indonesia dan GDPR internasional).
