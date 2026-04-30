# Rust Implementation (Axum)

## Overview

The Rust service is implemented using Axum and serves as the reference for:

- low-latency request handling
- explicit memory and concurrency control
- fine-grained instrumentation

The service exposes multiple protocols simultaneously over a shared core domain.

---

## Stack

| Component | Implementation | Detail |
| :- | -: | -: |
| Framework | Axum | |
| Async runtime | Tokio | |
| Serialization | | |
| | JSON | serde_json |
| | Binary | prost (for gRPC) |
| Metrics | prometheus | |

---

## Architecture

```text
Core domain (pure logic)
    |
Protocol Adapters
  - HTTP (Axum routes)
  - WebSocket (Axum WS)
  - gRPC (tonic)
Instrumentation Layer
```

---

## Project Structure

```text
services/rust/
  src/
    main.rs
    adapters/
      grpc.rs
      http.rs
      websocket.rs
    core/
      handler.rs
      mod.rs
      models.rs
    metrics/
      collector.rs
      mod.rs
```

---

## Service Contract

All protocols must implement the same logical contract.

### Request

```json
{
    "id": "uuid",
    "timestamp": 1234567890,
    "payload": "string or binary"
}
```

### Endpoints

- HTTP:
  - POST /echo
  - GET /health

- WebSocket:
  - message -> echo response

- gRPC:
  - Echo(Request) -> Response

---

## Metrics

Each request should emit:

- protocol
- latency (total)
- handler latency
- payload size
- response size

Optional

- serialization time
- queueing delay

---

## Concurrency Model

- Tokio async runtime
- non-blocking handlers
- minimal shared state
- lock-free where possible

### CPU-Bound Handlers

CPU-bound endpoints (e.g. `/work/primes`) should avoid blocking the async runtime.

Recommended approach:

- use `tokio::task::spawn_blocking` for heavy computation
- keep async handlers responsive

This ensures fair comparison with other runtimes under load.
