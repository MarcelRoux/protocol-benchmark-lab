# Benchmark Orchestrator

## Overview

The orchestrator is responsible for coordinating benchmark execution across the system.

It acts as the control plane between:

- UI
- client (load generator)
- target services

---

## Responsibilities

- receive benchmark requests (from UI or CLI)
- validate scenario configuration
- select target service + protocol
- trigger client execution
- collect and return results

---

## Interface

### API (example)

#### `POST /benchmark/run`

```json
{
    "language": "rust",
    "protocol": "http",
    "scenario": "high_concurrency"
}
```

Response:

```json
{
    "run_id": "uuid",
    "status": "running"
}
```

---

#### `GET /benchmark/{run_id}`

Returns:

- status
- partial or final results

---

## Execution Flow

```text
UI / CLI
    |
Orchestrator
    |
Load Client
    |
Target Service
    |
Metrics Collection
    |
Orchestrator (aggregation)
    |
UI / CLI
```

---

## Design Notes

- stateless where possible
- minimal business logic
- delegates execution to client
- does not perform benchmarking itself

---

## Implementation (initial)

- simple HTTP service
- in-memory run tracking
- synchronous execution acceptable for MVP

---

## Future Extensions

- queue-based execution
- distributed benchmarking
- historical result storage
