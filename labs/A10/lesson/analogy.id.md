# Analogi Gerbang Tol Otomatis Saat Mati Listrik

Bayangkan pintu gerbang tol otomatis di jalan bebas hambatan:

1. **Prinsip Fail-Safe (Aman):** Saat terjadi mati listrik mendadak, sistem pengaman hidrolik secara otomatis mengunci pintu gerbang dan menyalakan lampu darurat merah, meminta pengendara berhenti hingga genset menyala.
2. **Kondisi Fail-Open (Rentan):** Pintu gerbang dirancang jika sensor error atau mati listrik, palang pintu otomatis terangkat ke atas dan menggratiskan siapa pun lewat tanpa bayar dan tanpa verifikasi.
3. **Eksploitasi:** Penyerang yang ingin lewat gratis cukup memotong kabel sensor atau memicu konsleting listrik kecil agar pintu gerbang otomatis terbuka lebar.

Dalam software: Jika fungsi pengecekan keamanan mengalami *error/crash*, sistem tidak boleh meloloskan pengguna ke halaman rahasia!
