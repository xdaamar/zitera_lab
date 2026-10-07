# Practice Walkthrough: Investigating IDOR

### Objective
In this guided exercise, you will investigate how parameters are sent between the browser and the target application running at `http://127.0.0.1:8011`.

### Step 1: Log in as Test User
- Open `http://127.0.0.1:8011` in your browser.
- Login using standard credentials:
  - Username: `alice`
  - Password: `password123`

### Step 2: Observe Your Profile URL
- Navigate to "My Invoices".
- Notice the URL in your browser:
  `http://127.0.0.1:8011/invoice/1`

### Step 3: Inspect the Request
- Open Browser Developer Tools (F12) -> Network tab.
- Refresh the page and observe the response payload.
- Notice that `id: 1` corresponds to Alice's account.

### Step 4: Test Object Manipulation
- Change the URL parameter from `/invoice/1` to `/invoice/2`.
- Observation: Notice that Bob's confidential invoice details appear on screen!
- Root Cause: The server queried the invoice directly by ID without verifying if the requesting user (`session['user_id']`) owns that invoice.
