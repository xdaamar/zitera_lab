from flask import Flask, request, session, redirect, render_template_string, jsonify
import sqlite3
import os

app = Flask(__name__)
app.secret_key = "vaultpay_enterprise_treasury_session_secret"
DB_PATH = "treasury.db"

def init_db():
    if os.path.exists(DB_PATH):
        try:
            os.remove(DB_PATH)
        except Exception:
            pass
    conn = sqlite3.connect(DB_PATH)
    c = conn.cursor()
    c.execute("""
        CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY,
            username TEXT UNIQUE,
            password TEXT,
            full_name TEXT,
            role TEXT,
            department TEXT
        )
    """)
    c.execute("""
        CREATE TABLE IF NOT EXISTS wires (
            id INTEGER PRIMARY KEY,
            user_id INTEGER,
            ref_code TEXT,
            title TEXT,
            amount REAL,
            origin TEXT,
            beneficiary TEXT,
            memo TEXT,
            status TEXT,
            created_at TEXT
        )
    """)
    c.execute("INSERT OR REPLACE INTO users VALUES (1, 'alice', 'password123', 'Alice Vance', 'Junior Treasury Analyst', 'Global Liquidity Operations')")
    c.execute("INSERT OR REPLACE INTO users VALUES (2, 'bob', 'bobpass', 'Robert Sterling', 'Senior Treasury Specialist', 'FX Clearing & Settlement')")
    c.execute("INSERT OR REPLACE INTO users VALUES (99, 'admin', 'adminsecret_unbrute', 'Marcus Reid', 'VP Corporate Controller & Auditor', 'Executive Treasury Oversight')")

    c.execute("""
        INSERT OR REPLACE INTO wires VALUES (
            1, 1, 'TRX-VLT-1001', 'Cloud Infrastructure Compute Retainer', 4950.00,
            'VaultPay Primary Treasury (US-9921)', 'AWS Enterprise Cloud Services (AWS-4412)',
            'Standard monthly institutional cloud compute reservation and CDN bandwidth settlement.',
            'SETTLED', '2026-11-15 08:30:00'
        )
    """)
    c.execute("""
        INSERT OR REPLACE INTO wires VALUES (
            2, 2, 'TRX-VLT-1002', 'Annual Wildcard PKI Trust Renewal', 1450.00,
            'VaultPay Merchant Clearing (US-4012)', 'DigiCert Global Trust Authority',
            'Annual enterprise EV SSL/TLS wildcard trust certificates and code-signing HSM tokens.',
            'SETTLED', '2026-11-16 11:15:00'
        )
    """)
    c.execute("""
        INSERT OR REPLACE INTO wires VALUES (
            42, 99, 'TRX-VLT-9901-SOVEREIGN', 'CLASSIFIED: Federal Reserve Escrow & Sovereign Clearance', 99999.00,
            'VaultPay Sovereign Escrow Reserve (US-001099)', 'US Federal Reserve Settlement Depository',
            'SOX-404 Sovereign Treasury Compliance Audit Seal: ZITERA{b10k3n_4cc355_c0ntr01_m45t3r}',
            'RESTRICTED', '2026-11-17 06:00:00'
        )
    """)
    conn.commit()
    conn.close()

init_db()

BASE_LAYOUT = """
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{{ title }} | VaultPay Enterprise Treasury</title>
    <style>
        :root {
            --bg-primary: #0a0e17;
            --bg-secondary: #111827;
            --bg-tertiary: #1f2937;
            --card-border: #374151;
            --accent: #2563eb;
            --accent-hover: #1d4ed8;
            --text-main: #f9fafb;
            --text-muted: #9ca3af;
            --success: #10b981;
            --warning: #f59e0b;
            --danger: #ef4444;
        }
        * { box-sizing: border-box; margin: 0; padding: 0; }
        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Helvetica, Arial, sans-serif;
            background-color: var(--bg-primary);
            color: var(--text-main);
            min-height: 100vh;
            display: flex;
            flex-direction: column;
        }
        .navbar {
            background-color: var(--bg-secondary);
            border-bottom: 1px solid var(--card-border);
            padding: 0.75rem 2rem;
            display: flex;
            justify-content: space-between;
            align-items: center;
        }
        .brand {
            display: flex;
            align-items: center;
            gap: 0.75rem;
            text-decoration: none;
            color: var(--text-main);
            font-weight: 700;
            font-size: 1.15rem;
            letter-spacing: -0.025em;
        }
        .brand-icon {
            width: 32px;
            height: 32px;
            background: linear-gradient(135deg, #2563eb, #38bdf8);
            border-radius: 6px;
            display: flex;
            align-items: center;
            justify-content: center;
            font-weight: 900;
            color: white;
            font-size: 1rem;
        }
        .nav-links {
            display: flex;
            align-items: center;
            gap: 1.5rem;
        }
        .nav-link {
            color: var(--text-muted);
            text-decoration: none;
            font-size: 0.875rem;
            font-weight: 500;
            transition: color 0.15s;
        }
        .nav-link:hover { color: var(--text-main); }
        .user-pill {
            display: flex;
            align-items: center;
            gap: 0.5rem;
            background: var(--bg-tertiary);
            padding: 0.35rem 0.75rem;
            border-radius: 9999px;
            border: 1px solid var(--card-border);
            font-size: 0.825rem;
        }
        .user-dot { width: 8px; height: 8px; border-radius: 50%; background: var(--success); }
        .container {
            max-width: 1100px;
            width: 100%;
            margin: 2rem auto;
            padding: 0 1.5rem;
            flex: 1;
        }
        .breadcrumbs {
            display: flex;
            align-items: center;
            gap: 0.5rem;
            font-size: 0.825rem;
            color: var(--text-muted);
            margin-bottom: 1.5rem;
        }
        .breadcrumbs a { color: var(--text-muted); text-decoration: none; }
        .breadcrumbs a:hover { color: var(--accent); }
        .breadcrumbs span { color: #4b5563; }
        .card {
            background-color: var(--bg-secondary);
            border: 1px solid var(--card-border);
            border-radius: 8px;
            padding: 1.5rem;
            margin-bottom: 1.5rem;
            box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.3);
        }
        .card-header {
            display: flex;
            justify-content: space-between;
            align-items: center;
            margin-bottom: 1.25rem;
            padding-bottom: 0.75rem;
            border-bottom: 1px solid var(--card-border);
        }
        .card-title { font-size: 1.125rem; font-weight: 600; }
        .badge {
            display: inline-block;
            padding: 0.25rem 0.6rem;
            border-radius: 9999px;
            font-size: 0.75rem;
            font-weight: 600;
            text-transform: uppercase;
            letter-spacing: 0.05em;
        }
        .badge-settled { background: rgba(16, 185, 129, 0.15); color: #34d399; border: 1px solid rgba(16, 185, 129, 0.3); }
        .badge-restricted { background: rgba(245, 158, 11, 0.15); color: #fbbf24; border: 1px solid rgba(245, 158, 11, 0.3); }
        table {
            width: 100%;
            border-collapse: collapse;
            font-size: 0.875rem;
        }
        th {
            text-align: left;
            padding: 0.75rem 1rem;
            background: var(--bg-tertiary);
            color: var(--text-muted);
            font-weight: 600;
            border-bottom: 1px solid var(--card-border);
        }
        td {
            padding: 0.875rem 1rem;
            border-bottom: 1px solid var(--card-border);
        }
        tr:hover td { background-color: rgba(255, 255, 255, 0.02); }
        .btn {
            display: inline-block;
            padding: 0.5rem 1rem;
            border-radius: 6px;
            font-size: 0.875rem;
            font-weight: 500;
            text-decoration: none;
            cursor: pointer;
            border: 1px solid transparent;
            transition: all 0.15s;
        }
        .btn-primary { background: var(--accent); color: white; }
        .btn-primary:hover { background: var(--accent-hover); }
        .btn-outline {
            background: transparent;
            border-color: var(--card-border);
            color: var(--text-main);
        }
        .btn-outline:hover { background: var(--bg-tertiary); }
        .footer {
            background: var(--bg-secondary);
            border-top: 1px solid var(--card-border);
            padding: 1.5rem 2rem;
            font-size: 0.8rem;
            color: var(--text-muted);
            text-align: center;
        }
    </style>
</head>
<body>
    <header class="navbar">
        <a href="/" class="brand">
            <div class="brand-icon">VP</div>
            <span>VaultPay Enterprise</span>
        </a>
        <nav class="nav-links">
            <a href="/" class="nav-link">Treasury Operations</a>
            <a href="/" class="nav-link">Liquidity Ledgers</a>
            {% if session.get('username') %}
            <div class="user-pill">
                <span class="user-dot"></span>
                <span>{{ session.get('full_name') }}</span>
                <span style="color: #6b7280;">({{ session.get('role') }})</span>
            </div>
            <a href="/logout" class="nav-link" style="color: #f87171;">Sign Out</a>
            {% else %}
            <a href="/login" class="btn btn-primary" style="padding: 0.35rem 0.85rem;">Portal Login</a>
            {% endif %}
        </nav>
    </header>

    <main class="container">
        {% block content %}{% endblock %}
    </main>

    <footer class="footer">
        <p>&copy; 2026 VaultPay Financial Technologies Inc. All rights reserved. &bull; SOC2 Type II Certified &bull; ISO/IEC 27001 FinTech Settlement Infrastructure</p>
    </footer>
</body>
</html>
"""

LOGIN_TEMPLATE = BASE_LAYOUT.replace("{% block content %}{% endblock %}", """
<div style="max-width: 440px; margin: 3rem auto;">
    <div class="card">
        <div class="card-header" style="text-align: center; display: block; border-bottom: none; margin-bottom: 0.5rem;">
            <div class="brand-icon" style="margin: 0 auto 1rem; width: 44px; height: 44px; font-size: 1.3rem;">VP</div>
            <h2 class="card-title" style="font-size: 1.35rem;">Institutional Treasury Login</h2>
            <p style="font-size: 0.825rem; color: var(--text-muted); margin-top: 0.25rem;">Authorized Personnel Only &bull; Global Clearing Network</p>
        </div>
        
        {% if error %}
        <div style="background: rgba(239, 68, 68, 0.15); border: 1px solid var(--danger); color: #fca5a5; padding: 0.75rem; border-radius: 6px; font-size: 0.825rem; margin-bottom: 1rem;">
            {{ error }}
        </div>
        {% endif %}

        <form method="POST" action="/login">
            <div style="margin-bottom: 1rem;">
                <label style="display: block; font-size: 0.825rem; color: var(--text-muted); margin-bottom: 0.35rem;">Operator Username</label>
                <input type="text" name="username" value="alice" required style="width: 100%; padding: 0.65rem 0.85rem; background: var(--bg-primary); border: 1px solid var(--card-border); color: white; border-radius: 6px; font-size: 0.9rem;">
            </div>
            <div style="margin-bottom: 1.25rem;">
                <label style="display: block; font-size: 0.825rem; color: var(--text-muted); margin-bottom: 0.35rem;">Security Password</label>
                <input type="password" name="password" value="password123" required style="width: 100%; padding: 0.65rem 0.85rem; background: var(--bg-primary); border: 1px solid var(--card-border); color: white; border-radius: 6px; font-size: 0.9rem;">
            </div>
            <button type="submit" class="btn btn-primary" style="width: 100%; padding: 0.75rem;">Authenticate to Treasury</button>
        </form>

        <div style="margin-top: 1.5rem; padding-top: 1rem; border-top: 1px solid var(--card-border); font-size: 0.8rem; color: var(--text-muted);">
            <strong>Authorized Workstation Scope:</strong> Analyst Account <code>alice</code> (Global Liquidity Operations).
        </div>
    </div>
</div>
""")

DASHBOARD_TEMPLATE = BASE_LAYOUT.replace("{% block content %}{% endblock %}", """
<div class="breadcrumbs">
    <a href="/">Home</a>
    <span>/</span>
    <a href="/">Treasury Operations</a>
    <span>/</span>
    <span>Dispatched Wire Settlement Ledger</span>
</div>

<div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(240px, 1fr)); gap: 1rem; margin-bottom: 1.5rem;">
    <div class="card" style="margin-bottom: 0;">
        <div style="font-size: 0.8rem; color: var(--text-muted); text-transform: uppercase;">Active Settlement Account</div>
        <div style="font-size: 1.3rem; font-weight: 700; margin-top: 0.35rem;">VaultPay Master Treasury</div>
        <div style="font-size: 0.8rem; color: var(--success); margin-top: 0.25rem;">&bull; Synchronized via SWIFT FIN</div>
    </div>
    <div class="card" style="margin-bottom: 0;">
        <div style="font-size: 0.8rem; color: var(--text-muted); text-transform: uppercase;">Operator Identity</div>
        <div style="font-size: 1.3rem; font-weight: 700; margin-top: 0.35rem;">{{ user_full_name }}</div>
        <div style="font-size: 0.8rem; color: var(--text-muted); margin-top: 0.25rem;">Role: {{ user_role }}</div>
    </div>
    <div class="card" style="margin-bottom: 0;">
        <div style="font-size: 0.8rem; color: var(--text-muted); text-transform: uppercase;">Treasury Clearance Tier</div>
        <div style="font-size: 1.3rem; font-weight: 700; margin-top: 0.35rem;">Tier-1 Commercial</div>
        <div style="font-size: 0.8rem; color: #60a5fa; margin-top: 0.25rem;">Standard Liquidity Transit</div>
    </div>
</div>

<div class="card">
    <div class="card-header">
        <div>
            <h2 class="card-title">Dispatched Wire Transfers & Clearing Vouchers</h2>
            <p style="font-size: 0.825rem; color: var(--text-muted); margin-top: 0.2rem;">Records assigned to operator account in current fiscal cycle</p>
        </div>
        <span class="badge badge-settled">{{ wires|length }} Total Transactions</span>
    </div>

    <table>
        <thead>
            <tr>
                <th>Reference</th>
                <th>Transfer Memo / Title</th>
                <th>Beneficiary Entity</th>
                <th>Gross Amount</th>
                <th>Settlement Status</th>
                <th>Voucher Action</th>
            </tr>
        </thead>
        <tbody>
            {% for w in wires %}
            <tr>
                <td><code>{{ w[1] }}</code></td>
                <td><strong>{{ w[2] }}</strong></td>
                <td>{{ w[5] }}</td>
                <td style="font-family: monospace; font-weight: 600;">${{ "{:,.2f}".format(w[3]) }}</td>
                <td>
                    <span class="badge {% if w[4] == 'SETTLED' %}badge-settled{% else %}badge-restricted{% endif %}">
                        {{ w[4] }}
                    </span>
                </td>
                <td>
                    <a href="/wire/{{ w[0] }}" class="btn btn-outline" style="padding: 0.3rem 0.65rem; font-size: 0.8rem;">
                        View Voucher #{{ w[0] }} &rarr;
                    </a>
                </td>
            </tr>
            {% endfor %}
        </tbody>
    </table>
</div>
""")

VOUCHER_TEMPLATE = BASE_LAYOUT.replace("{% block content %}{% endblock %}", """
<div class="breadcrumbs">
    <a href="/">Home</a>
    <span>/</span>
    <a href="/">Treasury Operations</a>
    <span>/</span>
    <span>Wire Settlement Voucher #{{ wire.wire_id }}</span>
</div>

<div class="card" style="max-width: 800px; margin: 0 auto 2rem;">
    <div class="card-header">
        <div>
            <span style="font-size: 0.75rem; text-transform: uppercase; color: var(--text-muted); letter-spacing: 0.05em;">Institutional Wire Settlement Voucher</span>
            <h2 class="card-title" style="margin-top: 0.25rem;">{{ wire.title }}</h2>
        </div>
        <span class="badge {% if wire.status == 'SETTLED' %}badge-settled{% else %}badge-restricted{% endif %}">
            {{ wire.status }}
        </span>
    </div>

    <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 1.5rem; margin-bottom: 1.5rem; padding: 1rem; background: var(--bg-tertiary); border-radius: 6px;">
        <div>
            <div style="font-size: 0.75rem; color: var(--text-muted); text-transform: uppercase;">Transaction Reference</div>
            <div style="font-family: monospace; font-size: 1.05rem; font-weight: 700; margin-top: 0.2rem; color: #60a5fa;">{{ wire.ref }}</div>
        </div>
        <div>
            <div style="font-size: 0.75rem; color: var(--text-muted); text-transform: uppercase;">Settled Amount (USD)</div>
            <div style="font-family: monospace; font-size: 1.25rem; font-weight: 700; margin-top: 0.2rem; color: #34d399;">${{ "{:,.2f}".format(wire.amount) }}</div>
        </div>
        <div>
            <div style="font-size: 0.75rem; color: var(--text-muted); text-transform: uppercase;">Originating Treasury Entity</div>
            <div style="font-size: 0.9rem; font-weight: 500; margin-top: 0.2rem;">{{ wire.origin }}</div>
        </div>
        <div>
            <div style="font-size: 0.75rem; color: var(--text-muted); text-transform: uppercase;">Beneficiary Entity</div>
            <div style="font-size: 0.9rem; font-weight: 500; margin-top: 0.2rem;">{{ wire.beneficiary }}</div>
        </div>
    </div>

    <div style="margin-bottom: 1.5rem;">
        <label style="display: block; font-size: 0.8rem; color: var(--text-muted); text-transform: uppercase; margin-bottom: 0.4rem;">Clearing Record & Compliance Memo</label>
        <div style="background: var(--bg-primary); border: 1px solid var(--card-border); padding: 1rem; border-radius: 6px; font-family: monospace; font-size: 0.85rem; line-height: 1.5; color: #e2e8f0;">
            {{ wire.memo }}
        </div>
    </div>

    <div style="display: flex; justify-content: space-between; align-items: center; padding-top: 1rem; border-top: 1px solid var(--card-border);">
        <a href="/" class="btn btn-outline">&larr; Return to Treasury Dashboard</a>
        <a href="/wire/{{ wire.wire_id }}?format=json" class="btn btn-outline" target="_blank" style="font-size: 0.8rem; color: #60a5fa;">Inspect Raw Wire JSON API</a>
    </div>
</div>
""")

@app.route('/')
def index():
    if 'user_id' not in session:
        return redirect('/login')
    conn = sqlite3.connect(DB_PATH)
    c = conn.cursor()
    c.execute("SELECT id, ref_code, title, amount, status, beneficiary FROM wires WHERE user_id = ?", (session['user_id'],))
    wires = c.fetchall()
    conn.close()
    return render_template_string(
        DASHBOARD_TEMPLATE,
        title="Treasury Operations",
        user_full_name=session.get('full_name', 'Analyst'),
        user_role=session.get('role', 'Operator'),
        wires=wires
    )

@app.route('/login', methods=['GET', 'POST'])
def login():
    error = None
    if request.method == 'POST':
        username = request.form.get('username', '').strip()
        password = request.form.get('password', '').strip()
        conn = sqlite3.connect(DB_PATH)
        c = conn.cursor()
        c.execute("SELECT id, username, full_name, role, department FROM users WHERE username = ? AND password = ?", (username, password))
        row = c.fetchone()
        conn.close()
        if row:
            session['user_id'] = row[0]
            session['username'] = row[1]
            session['full_name'] = row[2]
            session['role'] = row[3]
            session['department'] = row[4]
            return redirect('/')
        else:
            error = "Authentication failed. Invalid treasury credentials."
    return render_template_string(LOGIN_TEMPLATE, title="Institutional Sign In", error=error)

@app.route('/logout')
def logout():
    session.clear()
    return redirect('/login')

@app.route('/wire/<int:wire_id>')
@app.route('/invoice/<int:wire_id>')
def wire_detail(wire_id):
    conn = sqlite3.connect(DB_PATH)
    c = conn.cursor()
    c.execute("SELECT id, ref_code, title, amount, origin, beneficiary, memo, status FROM wires WHERE id = ?", (wire_id,))
    row = c.fetchone()
    conn.close()
    if not row:
        return jsonify({"error": "Wire transfer record not found in clearing repository"}), 404

    payload = {
        "wire_id": row[0],
        "ref": row[1],
        "title": row[2],
        "amount": row[3],
        "origin": row[4],
        "beneficiary": row[5],
        "memo": row[6],
        "status": row[7]
    }

    if request.args.get('format') == 'json' or request.headers.get('Accept') == 'application/json':
        return jsonify(payload)

    return render_template_string(VOUCHER_TEMPLATE, title=f"Voucher #{wire_id}", wire=payload)

@app.route('/health')
def health():
    return jsonify({"status": "ok", "service": "VaultPay Enterprise Treasury", "port": 8011})

if __name__ == '__main__':
    app.run(host='0.0.0.0', port=8011)
