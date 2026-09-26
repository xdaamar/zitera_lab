from flask import Flask, request, render_template_string, jsonify
import sqlite3
import os

app = Flask(__name__)
DB_PATH = "inventory.db"

def init_db():
    if os.path.exists(DB_PATH):
        os.remove(DB_PATH)
    conn = sqlite3.connect(DB_PATH)
    c = conn.cursor()
    c.execute("""
        CREATE TABLE products (
            id INTEGER PRIMARY KEY,
            name TEXT,
            category TEXT,
            price REAL
        )
    """)
    c.execute("""
        CREATE TABLE vault_secrets (
            id INTEGER PRIMARY KEY,
            secret_name TEXT,
            secret_data TEXT
        )
    """)
    # Seed products
    c.execute("INSERT INTO products VALUES (1, 'Edge Firewall Gateway', 'Hardware', 1299.00)")
    c.execute("INSERT INTO products VALUES (2, 'Rackmount Server 2U', 'Hardware', 3499.50)")
    c.execute("INSERT INTO products VALUES (3, 'Managed 48-Port PoE Switch', 'Networking', 850.00)")
    c.execute("INSERT INTO products VALUES (4, 'Cat6A Ethernet Bulk Cable (1000ft)', 'Cables', 189.99)")
    c.execute("INSERT INTO products VALUES (5, 'Biometric Access Terminal', 'Security', 450.00)")

    # Seed flag
    c.execute("INSERT INTO vault_secrets VALUES (1, 'PROJECT_ZITERA_CORE_FLAG', 'FLAG: ZITERA{5q1_1nj3ct10n_m45t3r_2026}')")
    conn.commit()
    conn.close()

HTML_PAGE = """
<!DOCTYPE html>
<html>
<head>
    <title>ZITERA Warehouse Inventory — A05 Lab</title>
    <style>
        body { font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif; background: #0f172a; color: #f8fafc; margin: 0; padding: 40px; }
        .card { background: #1e293b; border: 1px solid #334155; border-radius: 8px; padding: 24px; max-width: 750px; margin: 0 auto; box-shadow: 0 4px 12px rgba(0,0,0,0.3); }
        .badge { display: inline-block; background: #0284c7; color: white; padding: 4px 8px; border-radius: 4px; font-size: 11px; font-weight: bold; margin-bottom: 12px; }
        input[type=text] { width: 75%; padding: 10px; background: #0f172a; border: 1px solid #475569; color: white; border-radius: 4px; box-sizing: border-box; }
        button { width: 22%; background: #0284c7; color: white; border: none; padding: 10px; border-radius: 4px; cursor: pointer; font-weight: bold; }
        button:hover { background: #0369a1; }
        table { width: 100%; border-collapse: collapse; margin-top: 20px; }
        th, td { text-align: left; padding: 10px; border-bottom: 1px solid #334155; }
        th { background: #0f172a; color: #94a3b8; }
        .err { background: #450a0a; border: 1px solid #991b1b; padding: 12px; border-radius: 4px; color: #fca5a5; margin-top: 16px; font-family: monospace; font-size: 12px; }
    </style>
</head>
<body>
    <div class="card">
        <span class="badge">ZITERA_LAB // A05:2025</span>
        <h2>Central Logistics Hardware Catalog</h2>
        <form method="GET" action="/">
            <input type="text" name="q" value="{{ query }}" placeholder="Search products (e.g. Server, Switch, Firewall)...">
            <button type="submit">Search</button>
        </form>

        {% if error %}
            <div class="err">
                <strong>Database Error:</strong><br>{{ error }}
            </div>
        {% endif %}

        {% if items %}
            <table>
                <thead>
                    <tr>
                        <th>Product Name</th>
                        <th>Category</th>
                        <th>Price (USD)</th>
                    </tr>
                </thead>
                <tbody>
                    {% for it in items %}
                    <tr>
                        <td><strong>{{ it[0] }}</strong></td>
                        <td>{{ it[1] }}</td>
                        <td>${{ it[2] }}</td>
                    </tr>
                    {% endfor %}
                </tbody>
            </table>
        {% else %}
            {% if not error %}
                <p style="margin-top: 24px; color: #94a3b8;">No inventory items found matching your query.</p>
            {% endif %}
        {% endif %}
    </div>
</body>
</html>
"""

@app.route('/')
def search():
    query = request.args.get('q', '')
    items = []
    error = None

    if query:
        conn = sqlite3.connect(DB_PATH)
        c = conn.cursor()
        # INTENTIONALLY VULNERABLE: Direct string formatting into SQL query
        raw_sql = f"SELECT name, category, price FROM products WHERE name LIKE '%{query}%'"
        try:
            c.execute(raw_sql)
            items = c.fetchall()
        except Exception as e:
            error = str(e)
        finally:
            conn.close()

    return render_template_string(HTML_PAGE, items=items, query=query, error=error)

@app.route('/health')
def health():
    return jsonify({"status": "ok", "lab": "A05", "port": 8015})

if __name__ == '__main__':
    init_db()
    app.run(host='0.0.0.0', port=8015)
