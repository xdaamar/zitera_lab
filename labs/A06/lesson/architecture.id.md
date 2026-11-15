# Arsitektur Sistem: Batas Kepercayaan & Mesin Status Otoritatif

## 1. Topologi Arsitektur Lab A06 (Zitera Procurement Portal)

Layanan lab A06 mensimulasikan sistem alur kerja pengadaan barang perusahaan (*Zitera Procurement Portal*) yang berjalan pada port 8016:

```text
[ Browser / API Client ]
            │
            ▼  (HTTP GET / POST port 8016)
┌──────────────────────────────────────────────────────────┐
│              ZITERA PROCUREMENT SERVICE RUNTIME          │
│                                                          │
│  ┌────────────────────────────────────────────────────┐  │
│  │ HTTP Request Dispatcher                            │  │
│  │ • GET /                        (HTML Dashboard)    │  │
│  │ • GET /health                  (Health Probe)      │  │
│  │ • GET /practice/verify         (Invariant Checker) │  │
│  │ • POST /api/orders/<id>/transition (FLAWED ENDPOINT)│  │
│  └────────────────────────┬───────────────────────────┘  │
│                           │                              │
│                           ▼ (Client Dictated State)      │
│  ┌────────────────────────────────────────────────────┐  │
│  │ In-Memory Orders State Store                       │  │
│  │ • Order 1001 ($120)  — Alice  (DRAFT)              │  │
│  │ • Order 1002 ($15000)— Bob    (PENDING_APPROVAL)   │  │
│  │ • Order 9999 ($50000)— Learner(DRAFT)              │  │
│  └────────────────────────┬───────────────────────────┘  │
│                           │                              │
│                           ▼                              │
│  ┌────────────────────────────────────────────────────┐  │
│  │ Business Invariant Verification Engine             │  │
│  │ (Detects high-value bypass >= $15,000 -> Releases) │  │
│  └────────────────────────────────────────────────────┘  │
└──────────────────────────────────────────────────────────┘
```

---

## 2. Analisis Batas Kepercayaan yang Runtuh (Broken Trust Boundary)
Pada arsitektur yang rentan ini, terjadi keruntuhan batas kepercayaan di 3 titik kritis:
1. **Penerimaan Parameter Kontrol dari Luar**: Parameter `target_state` diizinkan datang langsung dari payload JSON yang dikirim pengguna. Status sistem tidak boleh didikte oleh klien luar.
2. **Ketiadaan Validasi Invarian di Lapisan Domain**: Tidak ada lapisan bisnis yang memeriksa aturan: `IF order.amount >= 500 AND new_state == "APPROVED" THEN REQUIRE manager_signature`.
3. **Ketiadaan Konteks Identitas Sesi**: Permintaan transisi status tidak memverifikasi apakah pemanggil memiliki token sesi manajer atau hanya pengguna biasa pembuat draf.

---

## 3. Arsitektur Mesin Status Berintegritas Tinggi (Authoritative State Pattern)

Untuk mengeliminasi cacat desain, arsitektur harus menerapkan **State Pattern Berbasis Domain Terisolasi**:

```text
[ Incoming Request ]
        │
        ▼
┌──────────────────────────────────────────────────────────────┐
│                  DOMAIN ENTITY WORKFLOW GUARD                │
│                                                              │
│  ┌────────────────────────────────────────────────────────┐  │
│  │ 1. Current State Invariant Check                       │  │
│  │    (Can current state transition to next state?)       │  │
│  │    DRAFT -> [PENDING_APPROVAL, CANCELLED] ONLY         │  │
│  └───────────────────────────┬────────────────────────────┘  │
│                              │                               │
│                              ▼                               │
│  ┌────────────────────────────────────────────────────────┐  │
│  │ 2. Policy Enforcement Point (PEP)                      │  │
│  │    • If amount >= $500 -> Enforce manager approval gate│  │
│  │    • Verify caller identity against required role      │  │
│  └───────────────────────────┬────────────────────────────┘  │
│                              │                               │
│                              ▼                               │
│  ┌────────────────────────────────────────────────────────┐  │
│  │ 3. Atomic State Mutation                               │  │
│  │    Apply change to datastore with audit logging        │  │
│  └────────────────────────────────────────────────────────┘  │
└──────────────────────────────────────────────────────────────┘
```

### Prinsip Arsitektur yang Aman:
- **Server-Controlled State Transitions**: Klien hanya diizinkan mengirimkan *Intent* atau *Action* (misalnya: `submit()`, `cancel()`, `approve()`), bukan menetapkan status target secara langsung (`set_state("APPROVED")`).
- **Enforced State Transition Matrix**: Server memiliki peta matriks transisi yang ketat. Jika suatu entitas berada pada status `DRAFT`, transisi ke `DISPATCHED` akan langsung ditolak sebagai pelanggaran protokol sistem.
- **Strict Role-Based Invariants**: Setiap aksi yang memiliki dampak finansial atau operasional besar harus memiliki penjaga invarian (*invariant guard*) yang terikat pada identitas otentik pengguna.
