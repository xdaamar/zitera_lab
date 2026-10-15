# Remediation & Secure Coding Guide

To permanently eliminate Broken Access Control and IDOR vulnerabilities, software engineering teams must implement layered defensive controls across every data ingestion point.

---

### 1. Mandatory Server-Side Ownership Verification
Never trust client-supplied resource IDs submitted via URL paths, query parameters, or JSON payloads. The backend must always independently verify that the active authenticated session (`session['user_id']`) holds legitimate ownership rights over the requested object.

#### Secure Code Implementation (Python / Flask / SQLAlchemy):
```python
from flask import abort, session, jsonify, render_template

@app.route('/invoice/<int:invoice_id>')
@login_required
def view_invoice(invoice_id):
    current_user_id = session.get('user_id')
    if not current_user_id:
        abort(401, description="Authentication required.")

    # Two-factor query verification: Bind the invoice ID directly to the verified owner ID!
    invoice = db.session.query(Invoice).filter(
        Invoice.id == invoice_id,
        Invoice.user_id == current_user_id
    ).first()

    if not invoice:
        # Enforce Deny by Default: Do not distinguish between non-existent IDs and unauthorized access
        # Returning 404 prevents attackers from performing resource enumeration (Information Disclosure)
        abort(404, description="Invoice not found or access denied.")

    return render_template('invoice.html', invoice=invoice)
```

---

### 2. Implement Indirect Reference Maps (UUIDv4)
Exposing sequential integer primary keys (`1, 2, 3...`) makes enumeration attacks trivial. Decouple internal database indices from publicly exposed API identifiers by adopting cryptographically random UUIDs:

```python
import uuid

class Invoice(db.Model):
    __tablename__ = 'invoices'
    # Internal primary key remains an integer for optimal database indexing performance
    id = db.Column(db.Integer, primary_key=True)
    # Public reference exposed via API uses cryptographically secure random UUIDv4
    public_id = db.Column(db.String(36), default=lambda: str(uuid.uuid4()), unique=True, index=True)
    user_id = db.Column(db.Integer, db.ForeignKey('users.id'), nullable=False)
```

---

### 3. Enforce Role-Based Access Control (RBAC) on Sensitive Endpoints
Apply reusable middleware or decorators to safeguard privileged or administrative workflows:

```python
from functools import wraps

def require_role(required_role):
    def decorator(f):
        @wraps(f)
        def decorated_function(*args, **kwargs):
            user_role = session.get('user_role')
            if user_role != required_role:
                abort(403, description="Access Denied: Insufficient role permissions.")
            return f(*args, **kwargs)
        return decorated_function
    return decorator

@app.route('/admin/master-invoices')
@require_role('admin')
def get_master_invoices():
    # Accessible only by authenticated users possessing the verified 'admin' role
    return jsonify(db.session.query(Invoice).filter_by(is_master=True).all())
```

---

### 4. Automated Security Regression Testing
Write automated test cases (e.g., using `pytest`) to verify that cross-user access attempts fail deterministically:

```python
def test_cross_user_access_is_blocked(client):
    # 1. Authenticate as Alice
    client.post('/login', data={'username': 'alice', 'password': 'password123'})

    # 2. Attempt to access Bob's invoice (#2)
    response = client.get('/invoice/2')

    # 3. Assert that access is strictly denied with a 403 or 404 status code
    assert response.status_code in [403, 404], f"IDOR Vulnerability Detected! Received status: {response.status_code}"
```
