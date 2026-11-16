from flask import Flask, request, session, redirect, render_template_string, jsonify
import sqlite3
import os

app = Flask(__name__)
app.secret_key = "zitera_a01_super_secret_session_key"
DB_PATH = "treasury.db"

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
            full_name TEXT,
            role TEXT
        )
    """)
    c.execute("""
        CREATE TABLE wires (
            id INTEGER PRIMARY KEY,
            user_id INTEGER,
            ref_code TEXT,
            title TEXT,
            amount REAL,
            origin TEXT,
            beneficiary TEXT,
            memo TEXT,
            status TEXT
        )
    """)
    c.execute("INSERT INTO users VALUES (1, 'alice', 'password123', 'Alice Vance', 'Junior Treasury Analyst')")
    c.execute("INSERT INTO users VALUES (2, 'bob', 'bobpass', 'Robert Sterling', 'Treasury Specialist')")
    c.execute("INSERT INTO users VALUES (99, 'admin', 'adminsecret_unbrute', 'Marcus Reid', 'VP Corporate Controller & Auditor')")

    c.execute("INSERT INTO wires VALUES (1, 1, 'TRX-APEX-1001', 'Cloud Infrastructure Compute Retainer', 4950.00, 'ApexFin Master Treasury', 'AWS Inc. (AWS-4412)', 'Standard monthly cloud compute reservation.', 'SETTLED')")
    c.execute("INSERT INTO wires VALUES (2, 2, 'TRX-APEX-1002', 'Domain Registration & PKI Renewal', 1450.00, 'ApexFin Merchant Clearing', 'DigiCert Global Trust', 'Annual wildcard trust certification.', 'SETTLED')")
    c.execute("INSERT INTO wires VALUES (42, 99, 'TRX-APEX-9901-SOVEREIGN', 'CLASSIFIED: Sovereign Federal Audit & System License Clearance', 99999.00, 'ApexFin Escrow (US-001099)', 'US Federal Banking Security Oversight', 'SOX-404 Sovereign Security Audit Compliance Clearance. Authoritative Validation Seal: ZITERA{b10k3n_4cc355_c0ntr01_m45t3r}', 'RESTRICTED')")
    conn.commit()
    conn.close()

init_db()

@app.route('/')
def index():
    if 'user_id' not in session:
        return redirect('/login')
    conn = sqlite3.connect(DB_PATH)
    c = conn.cursor()
    c.execute("SELECT id, ref_code, title, amount, status FROM wires WHERE user_id = ?", (session['user_id'],))
    wires = c.fetchall()
    conn.close()
    return jsonify({"status": "ok", "user": session.get('username'), "wires": wires})

@app.route('/wire/<int:wire_id>')
@app.route('/invoice/<int:wire_id>')
def wire_detail(wire_id):
    conn = sqlite3.connect(DB_PATH)
    c = conn.cursor()
    c.execute("SELECT id, ref_code, title, amount, origin, beneficiary, memo, status FROM wires WHERE id = ?", (wire_id,))
    row = c.fetchone()
    conn.close()
    if not row:
        return jsonify({"error": "Wire not found"}), 404
    return jsonify({
        "wire_id": row[0],
        "ref": row[1],
        "title": row[2],
        "amount": row[3],
        "origin": row[4],
        "beneficiary": row[5],
        "memo": row[6],
        "status": row[7]
    })

if __name__ == '__main__':
    app.run(host='0.0.0.0', port=8011)
