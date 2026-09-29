# Remediation & Secure Hardening

Preventing security misconfigurations requires systematic, automated hardening processes:

## 1. Disable Directory Browsing
In web server configuration files (Nginx, Apache, Caddy, IIS), explicitly disable auto-indexing:

```nginx
# Nginx Hardening Example
location /backups/ {
    autoindex off;
    deny all;
}
```

```apache
# Apache .htaccess Hardening Example
Options -Indexes
```

## 2. Enforce Mandatory Credential Changes
- Force administrators to choose strong, non-default passwords upon first system initialization.
- Never ship production containers with hardcoded fallback usernames and passwords.

## 3. Restrict or Remove Debug Endpoints
- In production, disable all framework debug flags (`DEBUG = False`).
- Bind internal management and metrics endpoints (e.g. Prometheus, Spring Boot Actuator, debug vars) strictly to internal management networks or require mutual TLS / VPN authentication.

## 4. Automated Hardening Audits
- Implement automated Infrastructure as Code (IaC) linting and configuration scanning (e.g. Checkov, Trivy, Lynis) into CI/CD deployment pipelines.
