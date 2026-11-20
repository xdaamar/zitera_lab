# A12: Arsitektur & Lingkungan Praktikum

Laboratorium ini berjalan secara lokal dan deterministik di dalam sandbox sistem operasi Windows:

### 🛡️ Spesifikasi Komponen:
- **Target Application**: `SocialAuth Connect Gateway`
- **Active Listening Port**: `127.0.0.1:8035`
- **Runtime Environment**: `native_sandboxed` (Windows Job Objects & AppContainer Isolation)
- **Komunikasi Engine**: Inter-Process Communication (IPC) via Stdio JSON Broker Protocol.

```text
+-----------------------------------------------------------+
| ZITERA ENGINE BROKER (Local Host Loopback)                |
|  - Port Forwarding: 127.0.0.1:8035              |
|  - Session Token Isolation & Zero Internet Leaks          |
+-----------------------------------------------------------+
                              | Stdio IPC
                              v
+-----------------------------------------------------------+
| LAB TARGET BINARY (A12-lab.exe)                  |
|  - In-Memory Stateful Simulation Engine                   |
|  - Authoritative Flag Verification Engine                 |
+-----------------------------------------------------------+
```
