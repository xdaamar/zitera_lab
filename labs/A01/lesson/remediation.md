# Remediation & Secure Coding

To prevent Broken Access Control vulnerabilities:

## 1. Always Verify Ownership on the Server
Never rely on client-provided IDs without verifying the active session owns that resource:

```python
# SECURE IMPLEMENTATION
@app.route('/invoice/<int:invoice_id>')
def view_invoice(invoice_id):
    current_user_id = session.get('user_id')
    if not current_user_id:
        return redirect('/login')

    invoice = db.query(
        "SELECT * FROM invoices WHERE id = ? AND user_id = ?",
        (invoice_id, current_user_id)
    ).first()

    if not invoice:
        # Deny by default: 404 or 403
        abort(404, description="Invoice not found or unauthorized access.")

    return render_template('invoice.html', invoice=invoice)
```

## 2. Principle of Least Privilege
- Deny access by default.
- Use Indirect Reference Maps (e.g. session-specific tokens instead of database primary keys).
- Enforce strict Role-Based Access Control (RBAC) middleware on administrative endpoints.
