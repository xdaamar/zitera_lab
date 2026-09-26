# ZITERA_LAB — Security Specification

## 1. Purpose

ZITERA_LAB intentionally runs vulnerable applications.

Security therefore applies to the application itself, its lab environments, and the boundary between them.

The fundamental principle is:

> Vulnerable by design inside the lab; controlled by design outside the lab.

---

## 2. Threat Model

Assume that a lab may contain:

- vulnerable web applications
- intentionally weak authentication
- unsafe input handling
- malicious-looking payloads
- intentionally broken configuration
- untrusted lab files

Assume that a malformed or compromised lab repository could attempt to affect the host.

The system must minimize the blast radius.

---

## 3. Network Isolation

Default:

```text
Lab
 ↓
127.0.0.1 / localhost
```

Avoid default LAN exposure.

Avoid public exposure.

A lab should not need Internet access at runtime unless explicitly justified.

If Internet access is required, document:

- why
- what endpoints are needed
- whether it can be disabled
- what data leaves the machine

---

## 4. Filesystem Isolation

Do not grant arbitrary host filesystem mounts.

Prefer:

```text
container filesystem
container volume
```

over:

```text
host directory mounted writable
```

If a host mount is genuinely required:

- minimize scope
- minimize permissions
- prefer read-only
- document it in the lab manifest

---

## 5. Repository Safety

Do not blindly execute repository-provided scripts.

The engine must distinguish:

```text
data/configuration
```

from:

```text
executable code
```

Only supported lab actions defined by the platform contract may be executed automatically.

---

## 6. Installer Safety

Avoid:

```text
curl ... | powershell
curl ... | bash
```

or equivalent opaque remote execution patterns.

Prefer:

- official installers
- official package managers
- explicit download
- explicit verification
- visible user confirmation

---

## 7. Privilege

Run with the least privilege necessary.

Administrator privileges must not become the default for the entire application.

If an operation requires elevation:

```text
explain why
request elevation
perform operation
return to normal execution
```

Do not keep the entire application elevated.

---

## 8. Tool Installation

Security tools must not be silently installed.

For each tool:

```text
detect
version check
recommend
install/update
verify
```

The user should understand what is being changed.

---

## 9. Secrets

Do not store:

- API keys
- passwords
- private GitHub tokens
- authentication cookies

inside repositories or plain-text application configuration.

ZITERA_LAB V1 does not require remote account authentication.

---

## 10. Input Validation

The Rust engine must validate:

- lab IDs
- repository identifiers
- paths
- versions
- ports
- command arguments

Do not concatenate unchecked user-controlled strings into shell commands.

Prefer argument arrays/process APIs over shell command interpolation.

---

## 11. Port Management

Before starting a lab:

```text
check requested port
↓
available?
    YES → start
    NO  → report conflict / safe alternative
```

Do not silently kill unrelated user processes.

---

## 12. Lab Cleanup

Stop/remove operations must avoid deleting resources that do not belong to ZITERA_LAB.

Use unique naming/prefixing where practical.

Example conceptual prefix:

```text
zitera_lab_<lab_id>
```

---

## 13. Logging

Logs should help diagnose failures without leaking secrets.

Do not log:

- credentials
- tokens
- private keys
- sensitive environment variables

---

## 14. Updates

Downloaded lab updates should be validated before replacing a working lab.

At minimum:

- repository/version validation
- manifest validation
- compatibility check
- successful preparation before marking updated

A failed update must not leave the lab falsely marked as healthy.

---

## 15. Security Modes

Potential future feature:

```text
Standard
Restricted
Offline
```

V1 may use Standard as the default.

The architecture should not prevent future offline/restricted modes.

---

## 16. Responsible Scope

ZITERA_LAB is intended for:

- local educational targets
- intentionally vulnerable applications
- controlled CTF environments
- secure coding education

The product should not optimize for unrestricted scanning or exploitation of arbitrary third-party targets.

---

## 17. Security Review Gate

Every feature that:

- executes commands
- installs software
- modifies system configuration
- starts network services
- mounts host directories
- accesses credentials
- changes firewall/network settings
- downloads executable content

must receive an explicit security review during implementation.

---

## 18. Security Definition of Done

A security-sensitive sprint is incomplete if:

- privilege assumptions are undocumented
- host exposure is unexplained
- arbitrary command injection is possible
- cleanup can delete unrelated resources
- failures can leave dangerous services running
- downloaded content is blindly executed

---

# End
