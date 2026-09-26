# Technical Breakdown: IDOR & BOLA

## Insecure Direct Object References (IDOR)
IDOR occurs when an application provides direct access to objects based on user-supplied input without server-side validation.

### Example Vulnerable Endpoint:
```http
GET /api/documents?doc_id=1045 HTTP/1.1
Host: portal.local
Cookie: session_id=alice_session
```

### Vulnerable Server Code (Python / Flask):
```python
@app.route('/api/documents')
def get_document():
    doc_id = request.args.get('doc_id')
    # Flaw: No check whether the currently logged-in user owns doc_id!
    doc = db.query("SELECT * FROM documents WHERE id = ?", (doc_id,)).first()
    return jsonify(doc)
```

If Alice changes `doc_id=1045` to `doc_id=1046`, the application returns Bob's private document without hesitation.
