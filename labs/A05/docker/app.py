from flask import Flask, request, render_template_string, jsonify
import sqlite3
import os

app = Flask(__name__)
DB_PATH = "inventory.db"

def init_db():
    if os.path.exists(DB_PATH):
        try:
            os.remove(DB_PATH)
        except Exception:
            pass
    conn = sqlite3.connect(DB_PATH)
    c = conn.cursor()
    c.execute("""
        CREATE TABLE IF NOT EXISTS products (
            id INTEGER PRIMARY KEY,
            sku TEXT,
            name TEXT,
            category TEXT,
            price REAL,
            stock INTEGER
        )
    """)
    c.execute("""
        CREATE TABLE IF NOT EXISTS vault_secrets (
            id INTEGER PRIMARY KEY,
            secret_name TEXT,
            secret_data TEXT
        )
    """)
    # Seed authentic commercial B2B products
    c.execute("INSERT OR REPLACE INTO products VALUES (1, 'SKU-NET-01', 'Edge Security Firewall Gateway Enterprise', 'Networking & Edge', 1299.00, 48)")
    c.execute("INSERT OR REPLACE INTO products VALUES (2, 'SKU-SRV-02', 'High-Density Compute Blade 2U Xeon', 'Enterprise Compute', 3499.50, 14)")
    c.execute("INSERT OR REPLACE INTO products VALUES (3, 'SKU-NET-03', 'Managed 48-Port PoE+ L3 Aggregation Switch', 'Networking & Edge', 850.00, 62)")
    c.execute("INSERT OR REPLACE INTO products VALUES (4, 'SKU-CAB-04', 'Shielded Cat6A 10Gbps Bulk Cable Reel (1000ft)', 'Infrastructure Cabling', 189.99, 120)")
    c.execute("INSERT OR REPLACE INTO products VALUES (5, 'SKU-SEC-05', 'Biometric Optical Access Control Terminal', 'Physical Security', 450.00, 35)")

    # Seed flag in vault_secrets
    c.execute("INSERT OR REPLACE INTO vault_secrets VALUES (1, 'PROJECT_ZITERA_CORE_FLAG', 'FLAG: ZITERA{5q1_1nj3ct10n_m45t3r_2026}')")
    conn.commit()
    conn.close()

HTML_PAGE = """
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>MegaStore Global | Enterprise B2B Wholesale Inventory</title>
    <style>
        :root {
            --bg-primary: #0a0e17;
            --bg-secondary: #111827;
            --bg-tertiary: #1f2937;
            --border: #374151;
            --accent: #0284c7;
            --accent-hover: #0369a1;
            --text-main: #f9fafb;
            --text-muted: #9ca3af;
            --success: #10b981;
            --warning: #f59e0b;
            --danger: #ef4444;
        }
        * { box-sizing: border-box; margin: 0; padding: 0; }
        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
            background: var(--bg-primary);
            color: var(--text-main);
            min-height: 100vh;
            display: flex;
            flex-direction: column;
        }
        .navbar {
            background: var(--bg-secondary);
            border-bottom: 1px solid var(--border);
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
        }
        .brand-icon {
            width: 32px;
            height: 32px;
            background: linear-gradient(135deg, #0284c7, #38bdf8);
            border-radius: 6px;
            display: flex;
            align-items: center;
            justify-content: center;
            font-weight: 900;
            color: white;
            font-size: 0.95rem;
        }
        .nav-links { display: flex; align-items: center; gap: 1.5rem; }
        .nav-link { color: var(--text-muted); text-decoration: none; font-size: 0.875rem; font-weight: 500; }
        .nav-link:hover { color: var(--text-main); }
        .container { max-width: 1100px; width: 100%; margin: 2rem auto; padding: 0 1.5rem; flex: 1; }
        .breadcrumbs { display: flex; align-items: center; gap: 0.5rem; font-size: 0.825rem; color: var(--text-muted); margin-bottom: 1.5rem; }
        .breadcrumbs a { color: var(--text-muted); text-decoration: none; }
        .breadcrumbs a:hover { color: var(--accent); }
        .card {
            background: var(--bg-secondary);
            border: 1px solid var(--border);
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
            border-bottom: 1px solid var(--border);
        }
        .card-title { font-size: 1.125rem; font-weight: 600; }
        table { width: 100%; border-collapse: collapse; font-size: 0.875rem; }
        th { text-align: left; padding: 0.75rem 1rem; background: var(--bg-tertiary); color: var(--text-muted); font-weight: 600; border-bottom: 1px solid var(--border); }
        td { padding: 0.875rem 1rem; border-bottom: 1px solid var(--border); }
        tr:hover td { background-color: rgba(255, 255, 255, 0.02); }
        .btn {
            display: inline-block;
            padding: 0.65rem 1.25rem;
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
        input[type=text] {
            width: 100%;
            padding: 0.65rem 0.85rem;
            background: var(--bg-primary);
            border: 1px solid var(--border);
            color: white;
            border-radius: 6px;
            font-size: 0.9rem;
        }
        .err {
            background: rgba(239, 68, 68, 0.15);
            border: 1px solid var(--danger);
            padding: 1rem;
            border-radius: 6px;
            color: #fca5a5;
            margin-top: 1.25rem;
            font-family: monospace;
            font-size: 0.825rem;
        }
        .footer {
            background: var(--bg-secondary);
            border-top: 1px solid var(--border);
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
            <div class="brand-icon">MG</div>
            <span>MegaStore Global</span>
        </a>
        <nav class="nav-links">
            <a href="/" class="nav-link">B2B Wholesale Catalog</a>
            <a href="/" class="nav-link">Warehouse Logistics</a>
            <a href="/" class="nav-link">Bulk Procurement</a>
            <span style="font-size: 0.825rem; color: var(--text-muted);">Partner Account: <strong>Global Retail Partners LLC</strong></span>
        </nav>
    </header>

    <main class="container">
        <div class="breadcrumbs">
            <a href="/">Home</a>
            <span>/</span>
            <a href="/">Enterprise Inventory</a>
            <span>/</span>
            <span>Hardware &amp; Infrastructure Logistics</span>
        </div>

        <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(220px, 1fr)); gap: 1rem; margin-bottom: 1.5rem;">
            <div class="card" style="margin-bottom: 0;">
                <div style="font-size: 0.75rem; color: var(--text-muted); text-transform: uppercase;">Central Fulfillment Hubs</div>
                <div style="font-size: 1.25rem; font-weight: 700; margin-top: 0.25rem; color: var(--success);">All 3 Hubs Active</div>
                <div style="font-size: 0.8rem; color: var(--text-muted); margin-top: 0.25rem;">Rotterdam / Chicago / Singapore</div>
            </div>
            <div class="card" style="margin-bottom: 0;">
                <div style="font-size: 0.75rem; color: var(--text-muted); text-transform: uppercase;">Wholesale Discount Tier</div>
                <div style="font-size: 1.25rem; font-weight: 700; margin-top: 0.25rem; color: #38bdf8;">Tier-1 Platinum</div>
                <div style="font-size: 0.8rem; color: var(--text-muted); margin-top: 0.25rem;">Volume Pricing Enabled</div>
            </div>
            <div class="card" style="margin-bottom: 0;">
                <div style="font-size: 0.75rem; color: var(--text-muted); text-transform: uppercase;">Database Catalog Engine</div>
                <div style="font-size: 1.25rem; font-weight: 700; margin-top: 0.25rem;">SQLite Enterprise</div>
                <div style="font-size: 0.8rem; color: var(--text-muted); margin-top: 0.25rem;">Port 8015 &bull; Realtime Query</div>
            </div>
        </div>

        <div class="card">
            <div class="card-header">
                <div>
                    <h2 class="card-title">Commercial Inventory &amp; Hardware Search</h2>
                    <p style="font-size: 0.825rem; color: var(--text-muted); margin-top: 0.2rem;">
                        Filter enterprise hardware by name, category, or component identifier.
                    </p>
                </div>
            </div>

            <form method="GET" action="/" style="display: flex; gap: 0.75rem; margin-bottom: 1rem;">
                <input type="text" name="q" value="{{ query }}" placeholder="Search hardware catalog (e.g. Server, Switch, Firewall)..." style="flex: 1;">
                <button type="submit" class="btn btn-primary" style="white-space: nowrap;">Search Inventory</button>
            </form>

            {% if error %}
            <div class="err">
                <strong>Internal Database Exception:</strong><br>{{ error }}
            </div>
            {% endif %}

            {% if items %}
            <table>
                <thead>
                    <tr>
                        <th>Hardware Product Name</th>
                        <th>Category / Component</th>
                        <th>Unit Price (USD)</th>
                    </tr>
                </thead>
                <tbody>
                    {% for it in items %}
                    <tr>
                        <td><strong>{{ it[0] }}</strong></td>
                        <td>{{ it[1] }}</td>
                        <td style="font-family: monospace; font-weight: 600; color: #34d399;">
                            {% if it[2] is number %}
                                ${{ "{:,.2f}".format(it[2]) }}
                            {% else %}
                                {{ it[2] }}
                            {% endif %}
                        </td>
                    </tr>
                    {% endfor %}
                </tbody>
            </table>
            {% else %}
                {% if not error %}
                <p style="margin-top: 1.5rem; color: var(--text-muted); font-size: 0.875rem;">
                    No hardware products matching your filter were found in the active catalog.
                </p>
                {% endif %}
            {% endif %}
        </div>
    </main>

    <footer class="footer">
        <p>&copy; 2026 MegaStore Global Retail Logistics Inc. All rights reserved. &bull; Enterprise B2B Supply Chain &amp; Distribution Portal</p>
    </footer>
</body>
</html>
"""

@app.route('/')
def search():
    query = request.args.get('q', '')
    items = []
    error = None

    conn = sqlite3.connect(DB_PATH)
    c = conn.cursor()
    if query:
        # Intentionally vulnerable to SQL injection via unparameterized string formatting
        raw_sql = f"SELECT name, category, price FROM products WHERE name LIKE '%{query}%'"
        try:
            c.execute(raw_sql)
            items = c.fetchall()
        except Exception as e:
            error = str(e)
        finally:
            conn.close()
    else:
        try:
            c.execute("SELECT name, category, price FROM products")
            items = c.fetchall()
        except Exception as e:
            error = str(e)
        finally:
            conn.close()

    return render_template_string(HTML_PAGE, items=items, query=query, error=error)

@app.route('/health')
def health():
    return jsonify({"status": "ok", "service": "MegaStore Global Inventory", "port": 8015})

if __name__ == '__main__':
    init_db()
    app.run(host='0.0.0.0', port=8015)
