# A01: Broken Access Control (Kegagalan Kontrol Akses)

Broken Access Control menempati peringkat teratas (#1) dalam daftar risiko keamanan web OWASP Top 10:2025. Kerentanan ini terjadi ketika sebuah aplikasi web atau API gagal membatasi hak akses pengguna secara ketat, sehingga seorang pengguna dapat mengakses data atau fungsi yang bukan menjadi haknya.

## Mengapa Kerentanan Ini Sangat Krusial?
Saat kontrol akses gagal ditegakkan oleh server, penyerang dapat:
- Bertindak seolah-olah sebagai administrator atau pengguna lain tanpa izin.
- Melihat file rahasia, rekaman data pribadi, atau laporan keuangan milik korban (IDOR / BOLA).
- Mengubah profil, saldo transaksi, atau data sensitif pengguna lain.
- Mengesampingkan (bypass) mekanisme verifikasi keamanan.

## Memahami Perbedaan Authentication vs Authorization
Banyak pengembang pemula keliru menganggap keduanya sama, padahal fungsinya sangat berbeda:
- **Authentication (AuthN):** Menjawab pertanyaan *"Siapa kamu?"* (Contoh: Proses login dengan username dan password yang valid).
- **Authorization (AuthZ):** Menjawab pertanyaan *"Apa saja hak yang boleh kamu akses?"* (Contoh: Pengguna Alice hanya diizinkan melihat tagihan milik Alice, bukan milik Bob).

Broken Access Control hampir selalu merupakan kegagalan pada lapisan Authorization, bukan pada Authentication.
