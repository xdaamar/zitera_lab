# Pembahasan Teknis: IDOR & BOLA

## Insecure Direct Object References (IDOR)
IDOR adalah istilah yang digunakan ketika sebuah aplikasi web mengekspos referensi langsung ke suatu data internal (seperti ID database, nama file, atau kunci primer) dan mengizinkan pengguna memanipulasi referensi tersebut tanpa validasi di sisi server.

Dalam konteks API modern, celah ini sering disebut sebagai **BOLA (Broken Object Level Authorization)**.

### Contoh Endpoint Rentan (Vulnerable Endpoint):
```http
GET /api/documents?doc_id=1045 HTTP/1.1
Host: portal.local
Cookie: session_id=alice_session
```

### Kode Server yang Mengandung Bug (Python / Flask):
```python
@app.route('/api/documents')
def get_document():
    doc_id = request.args.get('doc_id')
    # Celah Fatal: Server tidak memeriksa apakah pengguna yang sedang login adalah pemilik sah dari doc_id ini!
    doc = db.query("SELECT * FROM documents WHERE id = ?", (doc_id,)).first()
    return jsonify(doc)
```

Jika pengguna bernama Alice yang memiliki `doc_id=1045` secara sengaja mengubah URL menjadi `doc_id=1046`, server dengan naif langsung mengambil dan menampilkan dokumen rahasia milik pengguna Bob.
