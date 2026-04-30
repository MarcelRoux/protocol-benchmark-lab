# Benchmarks

## Purpose

To provide **reproducible, comparable measurements** of the same canonical service across:

- languages (implementation dimension)
- protocols (adapter/transport dimension)
- configurations (scenario dimension)

---

## Metrics

### Latency

- p50
- p95
- p99

### Throughput

- requests/sec
- messages/sec

### Resource Usage

- CPU %
- memory (RSS)

---

## Test Dimensions

| Dimension | Variants |
| :- | -: |
| concurrency | 1, 10, 100, 1000 |
| payload size | small, medium, large |
| connection reuse | on / off |
| compression | on / off |

### Algorithm (CPU workloads)

- brute
- brute_odd_sqrt
- sieve

---

## Scenarios

### 1. Baseline

- single request
- minimal payload

### 2. High Concurrency

- many parallel requests

### 3. Large Payload

- stress serialization / deserialization

### 4. Streaming

- continuous message flow

### 5. CPU Baseline

- single request
- CPU-bound computation (`/work/primes`)

### 6. CPU Concurrency

- multiple concurrent CPU-bound requests
- observe contention and scheduling

### 7. Mixed Workload

- combine `/echo` and `/work/primes`
- observe interaction between I/O and CPU

---

## Methodology

- warmup phase before measurement
- fixed test duration
- identical payloads across runs
- isolated environment (Docker)
- identical logical contract and payload semantics across all language/protocol runs

---

## Output

Each run produces:

- raw metrics (JSON)
- summarized report
- optional visualization

---

## Pitfalls

- cold start skew
- connection setup overhead
- GC effects (Go/Python)
- scheduler noise

---

## Goal

Not absolute performance, but:
> relative comparison under controlled conditions
