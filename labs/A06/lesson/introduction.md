# A06: Insecure Design

## 1. What is Insecure Design?
In the OWASP Top 10:2025 standard, **Insecure Design** represents a broad category of architectural risks focused on foundational flaws in system architecture, threat modeling, and business logic formulation.

This category establishes a rigorous conceptual boundary between **Design Flaws** and **Implementation Defects**:
- **Implementation Defects**: The architecture, state transitions, and security policies were properly conceived, but engineers introduced coding mistakes (e.g., forgetting parameter binding, off-by-one errors, or improper header configurations).
- **Design Flaws (Insecure Design)**: The application code is syntactically pristine, free of memory leaks, and passes all static analysis tools without error. However, the fundamental business workflow, trust assumptions, or state machine transitions were conceived without robust threat modeling. No amount of clean coding can secure a system whose architectural foundation is inherently defective.

## 2. Why Insecure Design is Critical in 2025/2026
Across contemporary distributed architectures, microservices, and asynchronous event streams:
- **Implicit Trust Assumptions**: Backend handlers often assume that clients or intermediate gateways strictly adhere to intended sequence paths and business policies.
- **State Machine Sequence Skipping**: Complex multi-step enterprise workflows (such as corporate procurement, credit approvals, or supply chain fulfillment) fail to enforce strict state invariants, allowing attackers to leapfrog authorization gates.
- **Frontend-Only Gatekeeping**: Business rules and threshold limits are frequently evaluated solely within client-side user interfaces, while backend API endpoints accept arbitrary state mutation payloads without server-side validation.

## 3. Notable Real-World Design Failures
- **E-Commerce Checkout & Invariant Bypasses**: Business logic flaws allowing malicious actors to apply discount tokens recursively or alter checkout line-item prices to negative values, inducing ledger balance corruption.
- **Procurement & Financial Wire Dispatches**: Flawed enterprise resource planning (ERP) workflows where purchase orders could transition directly from `DRAFT` to `DISPATCHED` via raw API mutations, bypassing mandatory executive managerial signatures.
- **Insecure Account Recovery Workflows**: Relying on easily guessable static security questions as the sole verification gateway for privileged password resets without cryptographic identity proofs.

## 4. Learning Objectives in Lab A06
Through the *Zitera Procurement Portal* hands-on environment on port 8016, you will:
1. Distinguish between implementation bugs and architectural business logic design flaws.
2. Analyze workflow state machine definitions and locate missing transition enforcement gates.
3. Demonstrate how client-controlled API transition payloads allow high-value procurement orders to bypass mandatory manager sign-off.
4. Execute controlled exploitation to trigger state invariants and recover the challenge key in standard `ZITERA{...}` format.
5. Engineer resilient, authoritative server-side finite state machines (FSM) enforcing *Zero Trust* business logic invariants.
