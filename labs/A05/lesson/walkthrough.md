# Practice Walkthrough: Testing for SQL Injection

### Objective
Learn how to identify SQL injection test vectors on the inventory portal running at `http://127.0.0.1:8015`.

### Step 1: Normal Search
- Navigate to `http://127.0.0.1:8015`.
- In the search bar, type `Server` and press Search.
- **Observation:** Notice that only items containing "Server" are displayed in the results table.

### Step 2: Injecting a Single Quote
- In the search box, enter a single apostrophe: `'`
- **Observation:** A database syntax error or 500 error appears:
  `sqlite3.OperationalError: unrecognized token: "'%"`
- **Meaning:** The single quote broke out of the SQL string delimiter, proving the input is evaluated directly by the SQL interpreter!

### Step 3: Balanced Boolean Payload
- Enter:
  `' OR 1=1 --`
- **Observation:** All products in the warehouse database are now dumped onto the page.
- **Why it worked:** The injected boolean condition `1=1` made the WHERE clause universally true.
