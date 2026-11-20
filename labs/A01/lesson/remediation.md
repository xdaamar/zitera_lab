# Remediasi & Panduan Penulisan Kode Aman (Secure Coding)

Untuk mencegah terjadinya kerentanan Broken Access Control dan IDOR secara tuntas, tim rekayasa perangkat lunak wajib menerapkan kontrol defensif berlapis di setiap titik masuk data.

---

### 1. Wajib Verifikasi Hak Kepemilikan di Sisi Server (Server-Side Ownership Check)
Jangan pernah mempercayai parameter ID yang dikirimkan oleh pengguna melalui URL, query string, atau JSON body. Backend wajib selalu memvalidasi bahwa sesi login pengguna yang sedang aktif (`session['user_id']`) adalah pemilik sah dari data yang diminta.

#### Contoh Implementasi Aman (Python / Flask / SQLAlchemy):
```python
from flask import abort, session, jsonify, render_template

@app.route('/invoice/<int:invoice_id>')
@login_required
def view_invoice(invoice_id):
    current_user_id = session.get('user_id')
    if not current_user_id:
        abort(401, description="Autentikasi diperlukan.")

    # Verifikasi ganda: Ikat pencarian faktur DENGAN ID pemilik akun yang sah!
    invoice = db.session.query(Invoice).filter(
        Invoice.id == invoice_id,
        Invoice.user_id == current_user_id
    ).first()

    if not invoice:
        # Terapkan prinsip Deny by Default: Jangan bedakan antara ID salah atau ID milik orang lain
        # Mengembalikan 404 mencegah penyerang melakukan enumerasi keberadaan data (Information Disclosure)
        abort(404, description="Faktur tidak ditemukan atau akses ditolak.")

    return render_template('invoice.html', invoice=invoice)
```

---

### 2. Gunakan Peta Referensi Tidak Langsung (Indirect Reference Maps / UUID)
Menggunakan integer berurutan (`1, 2, 3...`) memudahkan penyerang melakukan serangan tebak ID (*enumeration attack*). Gantikan primary key berurutan dengan identifier acak kriptografis:

```python
import uuid

class Invoice(db.Model):
    __tablename__ = 'invoices'
    # Kunci internal tetap integer untuk performa index database
    id = db.Column(db.Integer, primary_key=True)
    # Kunci publik yang diekspos ke API menggunakan UUID v4 acak
    public_id = db.Column(db.String(36), default=lambda: str(uuid.uuid4()), unique=True, index=True)
    user_id = db.Column(db.Integer, db.ForeignKey('users.id'), nullable=False)
```

---

### 3. Terapkan Role-Based Access Control (RBAC) pada Endpoint Sensitif
Gunakan decorator atau middleware khusus untuk membatasi endpoint master atau administratif:

```python
from functools import wraps

def require_role(required_role):
    def decorator(f):
        @wraps(f)
        def decorated_function(*args, **kwargs):
            user_role = session.get('user_role')
            if user_role != required_role:
                abort(403, description="Akses ditolak: Wewenang peran tidak mencukupi.")
            return f(*args, **kwargs)
        return decorated_function
    return decorator

@app.route('/admin/master-invoices')
@require_role('admin')
def get_master_invoices():
    # Hanya dapat diakses oleh pengguna dengan role 'admin' terverifikasi
    return jsonify(db.session.query(Invoice).filter_by(is_master=True).all())
```

---

### 4. Otomatisasi Pengujian Keamanan Otorisasi (Automated Unit & Integration Tests)
Tulis pengujian otomatis (misalnya dengan `pytest`) untuk memastikan bahwa akses lintas pengguna selalu ditolak:

```python
def test_cross_user_access_is_blocked(client):
    # 1. Login sebagai Alice
    client.post('/login', data={'username': 'alice', 'password': 'password123'})

    # 2. Coba akses faktur nomor 2 milik Bob
    response = client.get('/invoice/2')

    # 3. Pastikan server menolak dengan status 404 atau 403
    assert response.status_code in [403, 404], f"IDOR Vulnerability Detected! Got status {response.status_code}"
```
