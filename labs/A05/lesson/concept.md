# Understanding SQL Injection (SQLi)

## Root Cause: String Concatenation

Consider this vulnerable code:
```python
search_term = request.args.get('q')
# DANGEROUS: String formatting glues user input into SQL syntax!
query = f"SELECT * FROM products WHERE name LIKE '%{search_term}%'"
db.execute(query)
```

If a benign user searches for `laptop`, the query is:
```sql
SELECT * FROM products WHERE name LIKE '%laptop%'
```

If an attacker inputs:
`' OR '1'='1' --`

The query becomes:
```sql
SELECT * FROM products WHERE name LIKE '%' OR '1'='1' --%'
```

Because `'1'='1'` is always true and `--` comments out the remainder of the query, the database returns every single row in the database, bypassing any intended filtering.
