# Target Architecture

The target application runs as an isolated micro-service exposed locally on `127.0.0.1:8019`.

```text
[Client / Learner]
       │
       ▼ (HTTP)
[127.0.0.1:8019] ──► [Zitera Target Container]
                                     │
                                     ▼
                            [State & Validation Engine]
```
