# A11: Arsitektur & Lingkungan Praktikum

Laboratorium ini berjalan secara lokal dan deterministik di dalam sandbox sistem operasi Windows:

### 🛡️ Spesifikasi Komponen:
- **Target Application**: `UserHub Identity Admin`
- **Active Listening Port**: `127.0.0.1:8034`
- **Runtime Environment**: `native_sandboxed` (Windows Job Objects & AppContainer Isolation)
- **Komunikasi Engine**: Inter-Process Communication (IPC) via Stdio JSON Broker Protocol.

```text
+-----------------------------------------------------------+
| ZITERA ENGINE BROKER (Local Host Loopback)                |
|  - Port Forwarding: 127.0.0.1:8034              |
|  - Session Token Isolation & Zero Internet Leaks          |
+-----------------------------------------------------------+
                              | Stdio IPC
                              v
+-----------------------------------------------------------+
| LAB TARGET BINARY (A11-lab.exe)                  |
|  - In-Memory Stateful Simulation Engine                   |
|  - Authoritative Flag Verification Engine                 |
+-----------------------------------------------------------+
```
