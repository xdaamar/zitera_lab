# Technical Breakdown: IDOR & Object-Level Authorization

## Understanding Insecure Direct Object References (IDOR)
Insecure Direct Object Reference (IDOR) occurs when an application exposes a direct reference to internal database entities (such as database primary keys: `id=1045`, `invoice_id=2`, `file=statement_2026.pdf`) and trusts user-supplied input without verifying authorization rights on the server side.

### Why Does IDOR Occur So Frequently?
1. **Flawed Developer Assumptions:** Developers frequently assume that because the client-side user interface (UI) only renders buttons linked to the user's own data, users will never attempt to request other resource IDs.
2. **Reliance on Security Through Obscurity:** Hiding links or menu items is considered sufficient protection, even though URL paths and query parameters can be trivially modified using a browser address bar or cURL.
3. **Decoupled AuthN and AuthZ Logic:** The application middleware verifies that the user is logged in (`@login_required`), but forgets to check whether the requested resource is actually owned by that authenticated user (`ownership_required`).

---

### Two Dimensions of Broken Access Control

```
                 [ ADMINISTRATOR ]
                        â–²
                        â”‚  Vertical Authorization (Privilege Escalation)
                        â”‚  (e.g., Standard user invoking /admin/dashboard)
                        â–¼
[ USER ALICE ] â—„â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â–º [ USER BOB ]
             Horizontal Authorization (IDOR / BOLA)
             (e.g., Alice reading Bob's /invoice/2)
```

1. **Horizontal Authorization (Horizontal Access Control):** Occurs between users with equal privilege tiers. Alice and Bob are both *Standard Users*, yet Alice can view invoices or private account records belonging to Bob.
2. **Vertical Authorization (Vertical Access Control):** Occurs when a lower-privileged user executes restricted functionalities or accesses administrative interfaces reserved for administrators (*Privilege Escalation*).

---

### Anatomy of an HTTP Request / Response

#### 1. Legitimate Request (Alice's Resource)
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

#### 2. Parameter Tampered Request (IDOR against Bob's Resource)
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
Even though the session cookie belongs strictly to Alice (`session_id=sess_alice_token_991`), the server returns Bob's sensitive invoice because it blindly queried the database using the URL parameter `id: 2`!

---

### Code Comparison: Vulnerable Pattern vs Secure Pattern

#### Vulnerable Implementation:
```python
@app.route('/invoice/<int:invoice_id>')
@login_required
def get_invoice(invoice_id):
    # CRITICAL FLAW: The server queries by ID directly from the URL,
    # without verifying if the active session owns this record!
    invoice = db.session.query(Invoice).filter_by(id=invoice_id).first()
    if not invoice:
        abort(404)
    return jsonify(invoice.to_dict())
```

#### Defensive & Secure Implementation:
```python
@app.route('/invoice/<int:invoice_id>')
@login_required
def get_invoice(invoice_id):
    current_user_id = session['user_id']

    # REMEDIATION: Bind the database query to the authenticated user's ID!
    invoice = db.session.query(Invoice).filter_by(
        id=invoice_id,
        owner_id=current_user_id
    ).first()

    if not invoice:
        # Enforce deny-by-default and prevent resource enumeration
        abort(404, description="Invoice not found or access denied")

    return jsonify(invoice.to_dict())
```
