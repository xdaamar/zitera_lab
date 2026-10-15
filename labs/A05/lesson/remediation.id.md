# Panduan Remediasi: Rekayasa Pertahanan Mutlak Terhadap SQL Injection

Untuk mengeliminasi risiko serangan SQL Injection secara tuntas, terapkan arsitektur pertahanan berlapis (*defense-in-depth*) berikut:

---

## 1. Gunakan Parameterized Queries (Prepared Statements)
Satu-satunya pertahanan primer yang terbukti 100% efektif adalah memisahkan kode perintah SQL dengan data pengguna pada tingkat parser database:

### Implementasi Python (sqlite3)
```python
# KODE AMAN: Menggunakan placeholder '?'
search_query = request.args.get('q', '')
formatted_param = f"%{search_query}%"

# Query dikompilasi terlebih dahulu; parameter dikirim secara terpisah
cursor.execute(
    "SELECT name, category, price FROM products WHERE name LIKE ?",
    (formatted_param,)
)
results = cursor.fetchall()
```

### Implementasi Python (SQLAlchemy ORM)
```python
from sqlalchemy import select
from models import Product

# ORM modern secara otomatis menyusun parameterized queries
stmt = select(Product).where(Product.name.ilike(f"%{search_query}%"))
products = session.scalars(stmt).all()
```

### Implementasi Node.js (pg / PostgreSQL)
```javascript
// Parameterized query dengan placeholder numerik ($1, $2)
const queryText = 'SELECT name, category, price FROM products WHERE name ILIKE $1';
const values = [`%${searchTerm}%`];
const res = await pool.query(queryText, values);
```

### Implementasi Go (database/sql)
```go
// Parameterized query dengan placeholder '?'
rows, err := db.QueryContext(ctx, "SELECT name, category, price FROM products WHERE name LIKE ?", "%"+searchTerm+"%")
if err != nil {
    return err
}
defer rows.Close()
```

---

## 2. Validasi Khusus untuk Elemen Dinamis (Allow-List Validation)
Prepared statements **TIDAK BISA** memvalidasi nama tabel atau nama kolom sorting (`ORDER BY`). Jika aplikasi Anda mengizinkan pengguna memilih kolom sorting secara dinamis:
- **JANGAN PERNAH** menggabungkan input langsung ke klausul `ORDER BY`.
- **GUNAKAN PETA WHITELIST KETAT**:
```python
ALLOWED_SORT_COLUMNS = {
    'name': 'name',
    'price_asc': 'price ASC',
    'price_desc': 'price DESC',
    'category': 'category'
}

sort_key = request.args.get('sort', 'name')
# Validasi allow-list ketat
safe_order = ALLOWED_SORT_COLUMNS.get(sort_key, 'name')

query = f"SELECT name, category, price FROM products ORDER BY {safe_order}"
cursor.execute(query)
```

---

## 3. Terapkan Prinsip Hak Akses Terkecil (Least Privilege)
- **Akun Database Khusus Aplikasi**: Portal publik harus terhubung ke database menggunakan pengguna (*user role*) dengan izin minimal (`SELECT` hanya pada tabel yang relevan).
- **Blokir Akses ke Tabel Sensitif**: Akun aplikasi web tidak boleh memiliki izin baca/tulis ke tabel kredensial administratif (`vault_secrets`, `users_auth`).
- **Nonaktifkan Fitur Eksekusi Berbahaya**: Nonaktifkan stored procedure berbahaya seperti `xp_cmdshell` (SQL Server) atau pembacaan file lokal seperti `LOAD_FILE()` / `INTO OUTFILE` (MySQL).

---

## 4. Waspadai Celah Raw SQL di Dalam ORM
Menggunakan ORM tidak otomatis menjamin 100% keamanan jika developer menggunakan fungsi mentah (*raw escape hatches*):
```python
# CONTOH SALAH DALAM DJANGO / SQLALCHEMY:
# Product.objects.extra(where=[f"name = '{user_input}'"])  # MASIH RENTAN SQLi!

# SELALU GUNAKAN API FILTER RESMI ORM:
Product.objects.filter(name__icontains=user_input)
```

---

## 5. Tangani Error dengan Aman & Sembunyikan Detail Internal
- Jangan pernah meneruskan string pengecualian database (`sqlite3.OperationalError` atau syntax traceback) ke antarmuka pengguna.
- Tampilkan pesan umum yang ramah: `"Layanan pencarian sedang mengalami gangguan sementara. Silakan coba kembali nanti."`
- Catat detail stack trace hanya ke log server internal yang aman dan terenkripsi.
