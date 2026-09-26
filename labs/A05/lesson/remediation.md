# Remediation: Parameterized Queries & Prepared Statements

The primary defense against SQL Injection is using **Parameterized Queries** (also called Prepared Statements).

## How Parameterized Queries Work
Instead of string concatenation, the database driver sends the query structure (the AST) and the parameters in two completely separate channels:

1. The SQL query structure is compiled first by the database engine:
   `SELECT * FROM products WHERE name LIKE ?`
2. The user input parameter is sent separately:
   `["%laptop%"]`

Even if the input contains `' OR '1'='1' --`, the database engine treats that entire string as literally the name of the product. It is impossible for data to change the logic of the query.

### Secure Python / SQLite Example:
```python
# SECURE IMPLEMENTATION
search_term = request.args.get('q', '')
formatted_param = f"%{search_term}%"

# The '?' placeholder instructs SQLite to treat input strictly as data
cursor.execute(
    "SELECT id, name, category, price FROM products WHERE name LIKE ?",
    (formatted_param,)
)
results = cursor.fetchall()
```
