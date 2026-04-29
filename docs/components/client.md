# Client / Benchmark Harness

## Overview

The client is a **load generator and benchmark harness** responsible for:

- generating traffic across protocols
- controlling concurrency and payloads
- collecting latency and throughput metrics
- producing reproducible benchmark results

It is a core component of the system:
> All comparisons are driven through the client.

---

## Responsibilities

- protocol-aware request generation
- concurrency control
- timing and measurement
- result aggregation
- scenario execution

---

## Design Principles

### 1. Single Client, Multiple Protocols

The client supports:

- HTTP
- WebSocket
- gRPC

All using the same scenario definitions.

---

### 2. Deterministic Execution

Each benchmark run must be:

- repeatable
- configurable
- isolated

---

### 3. Minimal Overhead

The client should not become the bottleneck:

- efficient connection handling
- minimal allocations in hot paths
- controlled concurrency

---

## Structure

```text
clients/
  load_client/
    main.rs
    protocols/
      grpc.rs
      http.rs
      websocket.rs
    scenarios/
      mod.rs
    metrics/
      collector.rs
```

---

## Scenario Model

Each benchmark is defined as a scenario.  
Each scenario must exercise the same canonical service contract regardless of language or protocol.

### Example

```yaml
name: high_concurrency_http
protocol: http
concurrency: 100
duration_seconds: 30
payload_size: 1024
connection_reuse: true
compression: false
multiplexing: false
```

---

## Supported Parameters

| Parameter | Value |
| :- | -:|
| protocol | http <br> websocket <br> grpc |
| concurrency | number of parallel workers |
| duration_seconds | test duration |
| payload_size | bytes |
| connection_reuse | boolean |
| compression | boolean |
| multiplexing | boolean |

---

## Metrics Collected

Per request:

- latency (ns/ms)
- success / failure
- payload size

Aggregated:

- p50 / p95 / p99 latency
- requests/sec
- error rate

---

## Execution Flow

```text
Load Scenario
    |
Initialize Connections
    |
Spawn Workers
    |
Send Requests
    |
Record Metrics
    |
Aggregate Results
    |
Output Report
```

---

## Output

Each run produces:

- raw metrics (JSON)
- summarized report
- optional visualization (UI)

---

## Integration with System

The client is controlled by:

- CLI (Makefile)
- UI (via orchestrator)

---

## Extending

### Add Protocol

- implement client adapter
- integrate into scenario runner

### Add Scenario

- define new YAML
- ensure consistency across protocols

---

## Non-Goals

- production load testing tool replacement
- full observability stack
- distributed benchmarking (initially)

---

## Goal

Provide:
> consistent, comparable, reproducible performance measurements across protocols and languages
