# ZITERA_LAB — Lab Specification

## 1. Purpose

This document defines the contract that every ZITERA_LAB lab must follow.

It exists so that new labs can be created by different people or AI agents without changing the core application architecture.

---

## 2. Lab Modes

Every applicable lab should support:

### Learn

Explains:

- concept
- analogy
- vulnerable behavior
- impact
- identification
- remediation

### Practice

Provides:

- guided scenario
- objectives
- controlled experiments
- explanations
- observations

### Challenge

Provides:

- scenario
- objective
- clues
- optional progressive hints
- validation
- flag or challenge completion

The Challenge mode must not simply repeat the Practice walkthrough.

---

## 3. Standard Repository

```text
zitera-lab-<id>/
│
├── manifest.json
├── lesson/
├── challenge/
├── docker/
├── assets/
└── README.md
```

Optional directories are allowed only when justified.

Do not create empty folders purely for symmetry.

---

## 4. Manifest Contract

Every lab must have a manifest.

Minimum conceptual fields:

```json
{
  "schemaVersion": 1,
  "id": "A01",
  "slug": "broken-access-control",
  "title": "Broken Access Control",
  "owasp": "A01:2025",
  "version": "1.0.0",
  "difficulty": "beginner",
  "runtime": "docker",
  "entrypoint": "docker/compose.yml",
  "defaultPort": 8011,
  "estimatedMinutes": 45,
  "modes": [
    "learn",
    "practice",
    "challenge"
  ]
}
```

The exact schema can evolve through versioning.

---

## 5. Manifest Responsibilities

Manifest data describes:

- identity
- version
- compatibility
- runtime requirements
- entrypoint
- ports
- difficulty
- learning modes

Manifest must NOT contain arbitrary executable scripts.

---

## 6. Lesson Structure

Lesson content should normally cover:

```text
Introduction
↓
Why it matters
↓
Analogy
↓
Technical explanation
↓
Vulnerable behavior
↓
Guided practice
↓
Remediation
↓
Knowledge check
```

Keep writing beginner-accessible without becoming technically inaccurate.

---

## 7. Practice Design

Practice should teach investigation, not blind command copying.

Every meaningful step should ideally answer:

```text
What are we doing?
Why are we doing it?
What should we observe?
What does the result mean?
```

---

## 8. Challenge Design

Challenge must define:

```text
scenario
objective
target
constraints
clues
hints
validation
reset behavior
```

Hints should be progressive.

Example:

```text
Hint 1 — conceptual
Hint 2 — technical area
Hint 3 — investigation direction
Hint 4 — near-solution guidance
```

Do not reveal the full solution immediately.

---

## 9. Difficulty

Supported conceptual levels:

```text
beginner
intermediate
advanced
```

Difficulty must reflect investigation complexity, not simply the number of steps.

---

## 10. Runtime Contract

A lab must declare:

- runtime type
- required services
- required ports
- health check
- start/stop behavior
- reset method

The core engine should not need lab-specific hardcoded logic wherever a manifest can describe the behavior.

---

## 11. Networking

Default:

```text
localhost only
```

A lab must justify any need for additional network exposure.

Never assume LAN/public exposure.

---

## 12. Reset Contract

A reset must return the lab to a known initial state.

Possible reset actions:

- recreate containers
- recreate database state
- regenerate seed data
- clear challenge state

Reset must be deterministic.

---

## 13. Validation

Challenge validation may be:

- flag-based
- state-based
- endpoint-based
- database-state-based
- file-state-based

The validation method must remain inside the isolated lab environment whenever practical.

---

## 14. Content Quality

Every lab should be reviewed for:

- technical correctness
- reproducibility
- beginner clarity
- security isolation
- predictable reset
- challenge solvability
- absence of accidental real-world targets

---

## 15. Reference Lab

A01 Broken Access Control is the reference implementation.

Future labs should copy the contract, not blindly copy the implementation.

---

## 16. Lab Completion

A lab is complete when:

```text
install works
start works
health check works
lesson renders
practice works
challenge works
validation works
reset works
stop works
update works
```

---

# End
