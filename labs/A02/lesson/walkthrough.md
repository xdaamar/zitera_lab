# Practice Walkthrough: Enumerating Misconfigurations

### Objective
In this exercise, you will investigate how exposed administrative interfaces, diagnostic routes, and default credentials compromise the OpsGateway running at `http://127.0.0.1:8012`.

### Step 1: Discovering Diagnostic Routes
- Navigate to `http://127.0.0.1:8012` in your web browser.
- Inspect the diagnostic endpoints advertised on the homepage:
  - Visit `http://127.0.0.1:8012/debug/vars`.
  - Observation: Notice that runtime environment variables, version information, and internal storage directory paths (`/app/backups/`) are returned in raw JSON without requiring any authentication!

### Step 2: Testing Default Credentials
- Return to the portal homepage and click `Login to Portal` (or navigate to `http://127.0.0.1:8012/login`).
- Attempt to sign in using standard vendor default credentials:
  - Username: `admin`
  - Password: `admin`
- Observation: Authentication succeeds immediately! You are redirected to `/admin` with full system administrator privileges.

### Step 3: Inspecting Directory Listing
- Based on the diagnostic info from `/debug/vars`, browse to `http://127.0.0.1:8012/backups/`.
- Observation: Because directory listing is enabled, the browser renders an index containing `backup_config.json.bak`.
- Clicking the backup file downloads the archive directly, exposing internal configuration keys and administrative secrets.
