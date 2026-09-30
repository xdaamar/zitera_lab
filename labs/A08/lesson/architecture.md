# Target Architecture

The target application runs as an isolated micro-service exposed locally on `127.0.0.1:8018`.

```text
[Client / Learner]
       │
       ▼ (HTTP)
[127.0.0.1:8018] ──► [Zitera Target Container]
                                     │
                                     ▼
                            [State & Validation Engine]
```
