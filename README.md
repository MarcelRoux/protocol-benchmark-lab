# protocol-benchmark-lab

A containerized lab for evaluating backend communication protocols and language runtimes under identical conditions.

This repository implements the same canonical service across multiple languages and protocols, then benchmarks them for reproducible comparison.

## What This Covers

- Protocol comparison (HTTP, WebSocket, gRPC)
- Language comparison (Rust, Go, Python)
- I/O-bound workloads (`/echo`)
- CPU-bound workloads (`/work/primes`)

## Quick Start

```bash
make up
```

## Documentation

- Architecture → `docs/architecture.md`
- Benchmarks → `docs/benchmarks.md`
- Language details → `docs/languages/`
- Protocols → `docs/protocols/`

## Goal

> Same problem, multiple implementations, measured tradeoffs.
