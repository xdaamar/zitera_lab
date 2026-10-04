# Remediasi & Pencegahan Mutlak SQL Injection

Satu-satunya solusi yang terbukti 100% efektif membasmi SQL Injection adalah memisahkan kode perintah SQL dengan data pengguna secara arsitektural:

## 1. Gunakan Parameterized Queries (Prepared Statements)
Saat menggunakan prepared statement, database engine akan mengompilasi struktur query SQL terlebih dahulu sebelum data pengguna dimasukkan. Karakter petik tunggal (`'`) atau sintaks `UNION` akan selalu diperlakukan murni sebagai teks string biasa, bukan sebagai perintah kode:

```python
# IMPLEMENTASI AMAN MENGGUNAKAN PARAMETERIZED QUERY
search_query = request.args.get('search', '')

# Gunakan placeholder '?' atau parameter penamaan (bukan konkatenasi string!)
sql = "SELECT name, category, price FROM products WHERE name LIKE ?"
cursor.execute(sql, ('%' + search_query + '%',))
rows = cursor.fetchall()
```

## 2. Gunakan ORM (Object-Relational Mapping)
Gunakan library ORM modern (seperti SQLAlchemy, Prisma, Hibernate, atau Django ORM) yang secara otomatis menerapkan prepared statements pada setiap operasi query database.

## 3. Terapkan Validasi Input & Escape
Lakukan validasi tipe data (misalnya memastikan ID harus berupa bilangan bulat `int`) dan gunakan *whitelist validation* untuk parameter dinamis seperti nama kolom sorting (`ORDER BY`).
