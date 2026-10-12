# Target Architecture: Injection Vulnerabilities

The target application runs as an isolated native micro-service exposed locally on `127.0.0.1:8015`.

```text
[Client / Learner]
       │
       ▼ (HTTP Request with Payload)
[127.0.0.1:8015] ──► [HTTP Query Interpreter]
                             │
                             ▼ (String Concatenation Breach)
                    [Database / Execution Engine]
                             │
                             ▼ (Unauthorized Data Exposure)
                    [Response / Leaked State]
```

### Architectural Trust Boundary
The fundamental flaw in this architecture is the absence of an abstraction layer between user-controlled input data and command execution syntax. When user strings are concatenated directly into SQL query statements, the parser cannot distinguish between developer commands and attacker data.
