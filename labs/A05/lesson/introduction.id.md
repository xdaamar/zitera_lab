# A05: Injection (Injeksi Perintah & Query)

Kerentanan Injection terjadi ketika data yang dimasukkan oleh pengguna (*user-supplied input*) disisipkan secara mentah ke dalam sebuah interpreter (seperti SQL engine, command shell sistem operasi, atau LDAP parser) sebagai bagian dari sebuah perintah atau query.

Interpreter yang tertipu tidak dapat membedakan mana kode perintah asli dan mana data dari pengguna, sehingga menjalankan instruksi berbahaya yang tidak diinginkan.

## Jenis-Jenis Serangan Injection Paling Umum:
1. SQL Injection (SQLi): Input berbahaya memanipulasi struktur query basis data relasional.
2. Command Injection (OS Command Injection): Input pengguna dieksekusi langsung sebagai perintah terminal di sistem operasi server.
3. NoSQL Injection: Memanipulasi query objek pada basis data NoSQL seperti MongoDB.
4. LDAP / XPath Injection: Memanipulasi layanan direktori pengguna atau dokumen XML.

Dalam daftar OWASP Top 10, Injection tetap menjadi salah satu kelas celah paling berbahaya karena eksploitasi yang berhasil dapat menyebabkan pencurian seluruh isi database, modifikasi data, hingga pengambilalihan server secara total (*Remote Code Execution*).
