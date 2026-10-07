# Technical Breakdown: Common Misconfiguration Vectors

## 1. Default Accounts & Insecure Passwords
Many software systems (routers, databases, CMS engines, management portals) ship with default credentials such as `admin:admin`, `root:toor`, or `manager:password`. If administrators fail to change these during deployment, anyone can log in with full administrative privileges.

## 2. Directory Listing / Indexing
When a web server receives a request for a directory (e.g. `/backups/`) and no index file (`index.html`) is present:
- Secure Server: Returns `403 Forbidden` or `404 Not Found`.
- Misconfigured Server: Generates an HTML directory listing allowing anyone to browse, view file sizes, and download internal documents, source code backups, or database dumps.

```http
GET /backups/ HTTP/1.1
Host: target.local

HTTP/1.1 200 OK
Content-Type: text/html

Index of /backups/
- backup_config.json.bak
- database_dump.sql
```

## 3. Unauthenticated Debug Endpoints
Frameworks (Spring Boot Actuator, Flask Debugger, Django Debug Toolbar) offer rich diagnostic tools during development. If deployed to production without access restrictions:
- `/debug/vars` or `/actuator/env` reveals database credentials, API secrets, and server memory state.
