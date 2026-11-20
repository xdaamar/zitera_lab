# Arsitektur Sistem & Model Alur Data: WebFetch Microservice

Praktikum ini menyimulasikan arsitektur microservice enterprise modern yang berjalan di `http://127.0.0.1:8060`.

```mermaid
flowchart LR
    A["Klien Peramban / Pentester"] -->|"HTTP Request + Payload"| B["Reverse Proxy / API Gateway"]
    B -->|"Meneruskan Request"| C["Service Core (WebFetch Microservice)"]
    C -->|"Query Data"| D["Database / Storage Engine"]
    C -.->|"Pengecekan Cacat"| E["Flag Enclave"]
    
    style A fill:#F1F5F9,stroke:#64748B,stroke-width:2px
    style B fill:#E0F2FE,stroke:#0284C7,stroke-width:2px
    style C fill:#FEF2F2,stroke:#EF4444,stroke-width:2px
    style D fill:#F0FDF4,stroke:#10B981,stroke-width:2px
    style E fill:#FEF3C7,stroke:#D97706,stroke-width:2px
```

### Titik Kelemahan Arsitektur:
1. **Batas Kepercayaan Terbuka (*Missing Trust Boundary*)**: Tidak ada pemisahan tegas antara parameter input yang dikontrol klien dengan data otentikasi internal.
2. **Ketiadaan Validasi Status**: Service Core mengeksekusi logika bisnis sebelum memastikan wewenang pengguna telah terverifikasi secara sah.
