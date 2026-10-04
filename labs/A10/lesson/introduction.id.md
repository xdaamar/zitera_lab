# A10: Mishandling of Exceptional Conditions (Penanganan Kondisi Pengecualian yang Buruk)

Mishandling of Exceptional Conditions berfokus pada kegagalan sistem dalam menangani error, exception tak terduga, atau kondisi batas secara aman (*Fail-Safe Defaults*).

Ketika aplikasi mengalami kesalahan tak terduga (misal kegagalan koneksi database, format data tidak sesuai, atau timeout), sistem yang rentan sering kali mengadopsi status *Fail-Open* (membuka akses tanpa autentikasi) alih-alih *Fail-Closed* (menutup akses secara aman).
