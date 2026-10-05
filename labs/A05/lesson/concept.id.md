# Pembahasan Teknis: Anatomi UNION-Based SQL Injection

## Bagaimana SQL Injection Terjadi?
Celah ini muncul ketika pengembang menggabungkan input pengguna secara langsung ke dalam string query SQL menggunakan konkatenasi string:

```python
# KODE RENTAN FATAL
search_query = request.args.get('search')
sql = "SELECT name, category, price FROM products WHERE name LIKE '%" + search_query + "%'"
cursor.execute(sql)
```

## Memanipulasi Logika dengan Operator UNION
Perintah `UNION` dalam SQL digunakan untuk menggabungkan hasil dari dua atau lebih perintah `SELECT` menjadi satu set hasil akhir.

Agar serangan `UNION` berhasil:
1. Jumlah Kolom Harus Sama: Query yang diinjeksi harus memiliki jumlah kolom yang sama persis dengan query asli aplikasi.
2. Tipe Data Harus Kompatibel: Tipe data kolom pada query kedua harus cocok dengan kolom pada query pertama.

### Contoh Payload:
Jika query asli mengambil 3 kolom (`name`, `category`, `price`), penyerang dapat menyuntikkan:
```sql
' UNION SELECT secret_name, secret_data, 0 FROM vault_secrets --
```
Tanda `--` di akhir bertindak sebagai komentar dalam SQL, sehingga sisa kutip dan perintah asli setelahnya diabaikan oleh database engine.
