# Bedah Teknis: IDOR & Otorisasi Berbasis Objek

## Memahami Insecure Direct Object References (IDOR)
Insecure Direct Object Reference (IDOR) terjadi ketika sebuah aplikasi web mengekspos referensi langsung ke objek internal database (seperti kunci primer basis data: `id=1045`, `invoice_id=2`, `file=statement_2026.pdf`) dan mempercayai input klien tanpa memverifikasi hak kepemilikan di sisi server.

### Mengapa IDOR Begitu Sering Muncul?
1. **Asumsi Keliru Pengembang:** Pengembang sering menganggap bahwa karena antarmuka web (UI) hanya menampilkan tombol ke data milik pengguna itu sendiri, maka pengguna tidak akan pernah mencoba meminta data nomor lain.
2. **Ketergantungan pada Obscurity:** Menyembunyikan link di menu navigasi dianggap cukup sebagai perlindungan, padahal URL dan parameter HTTP dapat dimanipulasi dengan mudah menggunakan peramban atau cURL.
3. **Pemisahan Logika AuthN dan AuthZ:** Middleware memeriksa apakah pengguna telah login (`@login_required`), tetapi lupa memeriksa apakah objek yang diminta memang milik pengguna tersebut (`ownership_required`).

---

### Dua Dimensi Broken Access Control

```
                 [ ADMINISTRATOR ]
                        â–²
                        â”‚  Otorisasi Vertikal (Privilege Escalation)
                        â”‚  (Contoh: User biasa mengakses /admin/dashboard)
                        â–¼
[ PENGGUNA ALICE ] â—„â”€â”€â”€â”€â”€â”€â”€â”€â”€â–º [ PENGGUNA BOB ]
              Otorisasi Horizontal (IDOR / BOLA)
              (Contoh: Alice membaca /invoice/2 milik Bob)
```

1. **Otorisasi Horizontal (Horizontal Access Control):** Terjadi antar pengguna yang memiliki tingkat hak akses setara. Alice dan Bob sama-sama berstatus *Standard User*, namun Alice dapat mengintip faktur atau data akun Bob.
2. **Otorisasi Vertikal (Vertical Access Control):** Terjadi ketika pengguna berhak akses rendah berhasil menjalankan fungsi atau mengakses antarmuka yang seharusnya dikhususkan bagi administrator (*Privilege Escalation*).

---

### Contoh Anatomi Protokol HTTP

#### 1. Request Normal (Milik Alice)
```http
GET /invoice/1 HTTP/1.1
Host: 127.0.0.1:8011
Cookie: session_id=sess_alice_token_991

HTTP/1.1 200 OK
Content-Type: application/json

{
  "id": 1,
  "owner": "alice",
  "item": "Cybersecurity Fundamentals Lab Kit",
  "amount": 250000,
  "status": "Paid"
}
```

#### 2. Request Manipulasi Parameter (IDOR ke Milik Bob)
```http
GET /invoice/2 HTTP/1.1
Host: 127.0.0.1:8011
Cookie: session_id=sess_alice_token_991

HTTP/1.1 200 OK
Content-Type: application/json

{
  "id": 2,
  "owner": "bob",
  "item": "Enterprise Penetration Testing Retainer",
  "amount": 18500000,
  "status": "Confidential"
}
```
Meskipun sesi login yang dilampirkan adalah milik Alice (`session_id=sess_alice_token_991`), server tetap mengembalikan data milik Bob karena server hanya mengeksekusi filter berdasarkan `id: 2` dari URL!

---

### Perbandingan Kode: Pola Rentan vs Pola Aman

#### Pola Rentan (Vulnerable Pattern):
```python
@app.route('/invoice/<int:invoice_id>')
@login_required
def get_invoice(invoice_id):
    # KESALAHAN FATAL: Server hanya memeriksa keberadaan ID di database,
    # tanpa membandingkannya dengan ID pengguna yang sedang login!
    invoice = db.session.query(Invoice).filter_by(id=invoice_id).first()
    if not invoice:
        abort(404)
    return jsonify(invoice.to_dict())
```

#### Pola Defensif yang Aman (Secure Pattern):
```python
@app.route('/invoice/<int:invoice_id>')
@login_required
def get_invoice(invoice_id):
    current_user_id = session['user_id']

    # PERBAIKAN: Ikat query ke ID kepemilikan pengguna yang sedang login!
    invoice = db.session.query(Invoice).filter_by(
        id=invoice_id,
        owner_id=current_user_id
    ).first()

    if not invoice:
        # Tolak akses dan hindari membocorkan informasi keberadaan ID (Deny by Default)
        abort(404, description="Invoice not found or access denied")

    return jsonify(invoice.to_dict())
```
