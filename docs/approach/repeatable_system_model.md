# Repeatable Systems Model

## Overview

This repository follows a **repeatable systems implementation model**:

> The same system is implemented across multiple languages and protocols, then measured and compared.

The goal is not to learn technologies in isolation, but to:

- implement equivalent systems
- observe behavioural differences
- measure performance characteristics
- document tradeoffs

---

## Core Principle

> Same problem, multiple implementations, measured outcomes.

This applies across the dimensions of:

- languages (Rust, Go, Python, etc.)
- protocols (HTTP, WebSocket, gRPC, etc.)
- scenarios (latency, concurrency, payload size)

---

## Canonical Service

Each language must implement the same logical service.

### Minimum Contract

- `GET /health`
- `POST /echo`

Request:

```json
{
    "id": "uuid",
    "timestamp": 1234567890,
    "payload": "string or binary"
}
```

---

## Protocol Adapters

Each service exposes multiple protocols simultaneously:

- HTTP (JSON)
- WebSocket (message echo)
- gRPC (binary)

Key rule:

> Protocols are adapters over the same core logic, not separate applications.

---

## Scenarios

Benchmarks are defined as reusable scenarios:

- baseline request
- high concurrency
- large payload
- streaming (later)

Each scenario must be:

- protocol-agnostic
- language-agnostic
- reproducible

---

## Benchmarking Model

The system measures:

- latency (p50 / p95 / p99)
- throughput
- error rate
- payload size impact

All measurements must be:

- consistent across runs
- consistent across languages
- driven by the same client/orchestrator

---

## Language Implementation

Each language implementation must demonstrate:

- server setup
- routing / handlers
- serialization (JSON / Protobuf)
- concurrency model
- timeout handling
- compatibility with benchmark harness

Not required:

- production hardening
- exhaustive edge cases
- framework-specific optimizations

---

## Extensibility Model

### Add a Language

1. Implement canonical service
2. Add protocol adapters
3. Register in compose
4. Ensure compatibility with scenarios

---

### Add a Protocol

1. Implement adapter in each service
2. Extend client/orchestrator
3. Add protocol-specific notes in `docs/protocols/`

---

### Add a Scenario

1. Define YAML in `benchmarks/scenarios/`
2. Ensure it runs across all languages
3. Validate comparability

---

## Repository Mental Model

```text
Same Inputs
    |
Different Implementations (Rust / Go / Python)
    |
Same Scenarios
    |
Measured Outputs
    |
Comparison + Documentation
```

---

## What This Is Not

- not a framework collection
- not a set of unrelated demos
- not production-ready services

---

## What This Is

A **systems comparison lab** that answers:

- how do different languages behave under the same workload?
- how do protocols impact latency and throughput?
- what tradeoffs exist between implementations?

---

## Output Expectations

Each addition to the repo should produce:

- working implementation
- benchmark results
- short written analysis

---

## Guiding Heuristic

When adding anything, ask:

> "Does this help compare the same system across dimensions?"

If not, it likely does not belong.

---

## Long-Term Direction

After API protocols:

- introduce caching scenarios (Redis)
- introduce streaming systems (NATS, Kafka)
- expand workload complexity

But only after the core model is stable.

---

## Bottom Line

This repository is a **learning and evaluation engine**, not a feature showcase.

Its value comes from:

- consistency
- comparability
- measurement
- clarity of explanation
