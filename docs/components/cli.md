# CLI (Makefile Interface)

## Overview

The CLI provides a simple interface for:

- starting the system
- running benchmarks
- interacting without the UI

It is implemented via a Makefile for simplicity and portability.

---

## Goals

- minimal friction
- consistent commands
- no additional tooling required

---

## Commands

### Start system

```bash
make up
```

Starts:

- services (Rust, Go, Python, etc.)
- orchestrator
- client
- UI (if enabled)

---

### Stop system

```bash
make down
```

---

### Run specific benchmark

```bash
make bench SCENARIO=high_concurrency PROTOCOL=http LANGUAGE=rust
```

---

### View logs

```bash
make logs
```

---

## Execution Model

The CLI interacts with the orchestrator:

```text
Makefile
    |
HTTP call
    |
Orchestrator
    |
Client
    |
Services
```

---

## Design Notes

- thin wrapper over HTTP calls
- no business logic
- mirrors UI capabilities

---

## Non-Goals

- complex CLI UX
- argument parsing frameworks
- scripting replacement

---

## Rationale

The Makefile provides:

- zero-dependency interface
- fast iteration
- easy integration with Docker workflows

---

## Future Extensions

- dedicated CLI binary
- richer parameter validation
- batch execution support
