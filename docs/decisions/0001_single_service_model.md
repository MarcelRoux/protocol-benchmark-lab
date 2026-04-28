# ADR-0001: Single Service Model

## Status

Accepted

---

## Context

Initial design considered:

- separate applications per protocol

This created:

- duplication
- inconsistent state
- complex orchestration

---

## Decision

Use a single serveice per language with multiple protocol adapters.

---

## Consequences

### Positive

- direct comparability
- shared state
- simple orchestration
- better UX (no restarts)

### Negative

- slightly more complex service implementation
- cross-protocol interference possible

---

## Rationale

This repository is a **measurement system**, not a set of demos.

The single-service model ensures:

- consistency
-repeatablity
-realism

---

## Alternatives Considered

### Separate services per protocol

Rejected due to:

- duplication
- coordination complexity
- reduced comparability
a