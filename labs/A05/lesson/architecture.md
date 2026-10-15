# System Architecture & Control-Data Plane Separation

## 1. Lab A05 Architecture Topology (Central Logistics Catalog)

The A05 target service models an internal hardware logistics portal (*Central Logistics Hardware Catalog*) exposed on port 8015:

```text
[ Browser / Attacker Client ]
             │
             ▼  HTTP GET /?q=<payload> (Port 8015)
┌──────────────────────────────────────────────────────────┐
│              FLASK WEB RUNTIME CONTAINER                 │
│                                                          │
│  ┌────────────────────────────────────────────────────┐  │
│  │ HTTP Request Handler (app.py)                      │  │
│  │ • Reads raw query parameter q                      │  │
│  │ • Executes direct f-string SQL concatenation       │  │
│  └────────────────────────┬───────────────────────────┘  │
│                           │                              │
│                           ▼ (Raw Formatted SQL String)   │
│  ┌────────────────────────────────────────────────────┐  │
│  │ SQLite Database Engine (inventory.db)              │  │
│  │                                                    │  │
│  │  ┌──────────────────────┐  ┌────────────────────┐  │  │
│  │  │ Table products       │  │ Table vault_secrets│  │  │
│  │  │ • id, name           │  │ • id, secret_name  │  │  │
│  │  │ • category, price    │  │ • secret_data      │  │  │
│  │  └──────────────────────┘  └────────────────────┘  │  │
│  └────────────────────────┬───────────────────────────┘  │
│                           │                              │
│                           ▼                              │
│  ┌────────────────────────────────────────────────────┐  │
│  │ HTML Jinja2 Template Renderer                      │  │
│  │ • Renders product inventory / exfiltrated rows     │  │
│  │ • Displays unhandled SQLite diagnostic errors      │  │
│  └────────────────────────────────────────────────────┘  │
└──────────────────────────────────────────────────────────┘
```

---

## 2. Vulnerable Architectural Analysis
The application features three compounding architectural anti-patterns:
1. **Control-Data Plane Conflation**: No semantic separation exists between authoritative query instructions and untrusted client inputs. Assembling queries via `f"SELECT ... '{query}%'"` flattens code and data into a single interpretable text stream.
2. **Absence of Database Least Privilege**: The web application connects with monolithic privileges over all tables, permitting public searches to read sensitive tables such as `vault_secrets`.
3. **Internal Diagnostic Telemetry Exposure**: Unfiltered database exceptions (`sqlite3.OperationalError`) are passed directly into frontend templates, giving attackers real-time syntax feedback.

---

## 3. High-Assurance Defense-in-Depth Architecture

Resilient architectures physically decouple query compilation from parameter binding:

```text
[ User Input ] ──▶ [ Validation & Sanitization Gate ]
                                │ (Type & Pattern Check)
                                ▼
                   [ AST Query Compiler ] ◀── [ Predefined SQL Template ]
                                │
                                ▼ (Precompiled AST Bytecode)
                   [ Database Execution Engine ]
                                ▲
                                │ (Inert Data Stream)
                   [ Parameter Binding Stream ] ◀── [ Bound Parameters ]
```

### Core Architecture Pillars:
- **Precompiled Abstract Syntax Trees (AST)**: The database engine compiles query structure prior to ingesting input. Bound variables travel over a dedicated parameter stream; quotation marks or injection syntax are treated strictly as inert string literals.
- **Role-Based Schema Isolation**: Public web search queries execute under restricted database credentials possessing read-only access strictly to the `products` table, with all administrative tables partitioned into segregated schemas.
- **Diagnostic Error Suppression**: Generic error identifiers are returned to clients while detailed exception traces are routed exclusively to secure internal telemetry streams.
