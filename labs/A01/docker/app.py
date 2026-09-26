from flask import Flask, request, session, redirect, render_template_string, jsonify
import sqlite3
import os

app = Flask(__name__)
app.secret_key = "zitera_a01_super_secret_session_key"
DB_PATH = "accounting.db"

def init_db():
    if os.path.exists(DB_PATH):
        os.remove(DB_PATH)
    conn = sqlite3.connect(DB_PATH)
    c = conn.cursor()
    c.execute("""
        CREATE TABLE users (
            id INTEGER PRIMARY KEY,
            username TEXT,
            password TEXT,
            role TEXT
        )
    """)
    c.execute("""
        CREATE TABLE invoices (
            id INTEGER PRIMARY KEY,
            user_id INTEGER,
            title TEXT,
            amount REAL,
            details TEXT,
            status TEXT
        )
    """)
    # Seed users
    c.execute("INSERT INTO users VALUES (1, 'alice', 'password123', 'user')")
    c.execute("INSERT INTO users VALUES (2, 'bob', 'bobpass', 'user')")
    c.execute("INSERT INTO users VALUES (99, 'admin', 'adminsecret_unbrute', 'admin')")

    # Seed invoices
    c.execute("INSERT INTO invoices VALUES (1, 1, 'Cloud Hosting - March', 49.99, 'Standard 2 vCPU VPS Instance', 'PAID')")
    c.execute("INSERT INTO invoices VALUES (2, 2, 'Domain Registration (bob.io)', 14.50, '1 Year Registration', 'PAID')")
    c.execute("INSERT INTO invoices VALUES (42, 99, 'CLASSIFIED: Master System License & Audit', 99999.00, 'FLAG: ZITERA{b10k3n_4cc355_c0ntr01_m45t3r}', 'CONFIDENTIAL')")
    conn.commit()
    conn.close()

HTML_LAYOUT = """
<!DOCTYPE html>
<html>
<head>
    <title>ZITERA Finance Portal — A01 Lab</title>
    <style>
        body { font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif; background: #0f172a; color: #f8fafc; margin: 0; padding: 40px; }
        .card { background: #1e293b; border: 1px solid #334155; border-radius: 8px; padding: 24px; max-width: 650px; margin: 0 auto; box-shadow: 0 4px 12px rgba(0,0,0,0.3); }
        .badge { display: inline-block; background: #dc2626; color: white; padding: 4px 8px; border-radius: 4px; font-size: 11px; font-weight: bold; margin-bottom: 12px; }
        input[type=text], input[type=password] { width: 100%; padding: 10px; margin: 8px 0; background: #0f172a; border: 1px solid #475569; color: white; border-radius: 4px; box-sizing: border-box; }
        button { background: #e63946; color: white; border: none; padding: 10px 18px; border-radius: 4px; cursor: pointer; font-weight: bold; }
        button:hover { background: #c52233; }
        a { color: #38bdf8; text-decoration: none; }
        a:hover { text-decoration: underline; }
        .invoice-box { background: #0f172a; border-left: 4px solid #e63946; padding: 16px; margin-top: 16px; }
    </style>
</head>
<body>
    <div class="card">
        <span class="badge">ZITERA_LAB // A01:2025</span>
        <h2>Internal Financial Accounting Portal</h2>
        {% if user %}
            <p>Logged in as: <strong>{{ user }}</strong> | <a href="/logout">Logout</a></p>
            <p><a href="/invoice/1">View My Invoice (#1)</a></p>
            {% block content %}{% endblock %}
        {% else %}
            <p>Please log in with your employee credentials (e.g. <code>alice</code> / <code>password123</code>):</p>
            <form method="POST" action="/login">
                <input type="text" name="username" placeholder="Username" required>
                <input type="password" name="password" placeholder="Password" required>
                <button type="submit">Sign In</button>
            </form>
            {% if error %}<p style="color: #ef4444;">{{ error }}</p>{% endif %}
        {% endif %}
    </div>
</body>
</html>
"""

@app.route('/')
def index():
    user = session.get('username')
    return render_template_string(HTML_LAYOUT, user=user)

@app.route('/login', methods=['POST'])
def login():
    username = request.form.get('username')
    password = request.form.get('password')
    conn = sqlite3.connect(DB_PATH)
    c = conn.cursor()
    c.execute("SELECT id, username FROM users WHERE username = ? AND password = ?", (username, password))
    user_row = c.fetchone()
    conn.close()

    if user_row:
        session['user_id'] = user_row[0]
        session['username'] = user_row[1]
        return redirect('/invoice/1')
    return render_template_string(HTML_LAYOUT, user=None, error="Invalid credentials.")

@app.route('/logout')
def logout():
    session.clear()
    return redirect('/')

@app.route('/invoice/<int:invoice_id>')
def view_invoice(invoice_id):
    if 'user_id' not in session:
        return redirect('/')
    
    # INTENTIONALLY VULNERABLE: Direct object reference without verifying ownership
    conn = sqlite3.connect(DB_PATH)
    c = conn.cursor()
    c.execute("SELECT id, user_id, title, amount, details, status FROM invoices WHERE id = ?", (invoice_id,))
    inv = c.fetchone()
    conn.close()

    if not inv:
        content = "<p style='color: #f87171;'>Invoice not found.</p>"
    else:
        content = f"""
        <div class="invoice-box">
            <h3>Invoice #{inv[0]}: {inv[2]}</h3>
            <p><strong>Owner User ID:</strong> {inv[1]}</p>
            <p><strong>Amount:</strong> ${inv[3]:.2f}</p>
            <p><strong>Status:</strong> {inv[5]}</p>
            <p><strong>Details / Line Items:</strong></p>
            <pre style="background: #020617; padding: 12px; color: #a7f3d0; border-radius: 4px;">{inv[4]}</pre>
        </div>
        """
    return render_template_string(HTML_LAYOUT + content, user=session.get('username'))

@app.route('/health')
def health():
    return jsonify({"status": "ok", "lab": "A01", "port": 8011})

if __name__ == '__main__':
    init_db()
    app.run(host='0.0.0.0', port=8011)
