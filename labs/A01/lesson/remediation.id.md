# Remediasi & Panduan Penulisan Kode Aman (Secure Coding)

Untuk mencegah terjadinya kerentanan Broken Access Control dan IDOR, terapkan prinsip keamanan berikut:

## 1. Selalu Verifikasi Hak Kepemilikan di Sisi Server (Server-Side Ownership Check)
Jangan pernah mempercayai parameter ID yang dikirim oleh pengguna dari browser. Server wajib selalu memastikan bahwa sesi login aktif memiliki hak legal atas data yang diminta:

```python
# IMPLEMENTASI KODE YANG AMAN
@app.route('/invoice/<int:invoice_id>')
def view_invoice(invoice_id):
    current_user_id = session.get('user_id')
    if not current_user_id:
        return redirect('/login')

    # Verifikasi ganda: Cocokkan ID faktur DENGAN ID pemilik akun yang sah!
    invoice = db.query(
        "SELECT * FROM invoices WHERE id = ? AND user_id = ?",
        (invoice_id, current_user_id)
    ).first()

    if not invoice:
        # Terapkan prinsip tolak secara default (Deny by default)
        abort(404, description="Faktur tidak ditemukan atau akses ditolak.")

    return render_template('invoice.html', invoice=invoice)
```

## 2. Terapkan Prinsip Least Privilege
- Gunakan aturan pembatasan akses secara ketat: tolak semua akses secara default (*Deny by default*), kecuali yang diizinkan secara eksplisit.
- Hindari penggunaan ID berurutan (*sequential integer ID* seperti 1, 2, 3), gantikan dengan UUID atau token acak (*Indirect Reference Maps*).
- Pasang middleware otorisasi berbasis peran (*Role-Based Access Control / RBAC*) pada seluruh endpoint administratif.
