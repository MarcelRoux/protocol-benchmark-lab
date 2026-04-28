# HTTP/1.1

## Overview

HTTP/1.1 is a request/response protocol widely used for:

- public APIs
- web services
- general client-server communication

---

## Characteristics

- text-based (headers + optional JSON body)
- stateless
- connection reuse via keep-alive

---

## Strengths

- universal support
- simple debugging
- broad tooling ecosystem

---

## Weaknesses

- higher overhead vs binary protocols
- no multiplexing
- head-of-line blocking

---

## Use Cases

- external APIs
- low-to-moderate throughput services
- browser-based interactions

---

## Benchmark Focus

- serialization overhead (JSON)
- connection reuse impact
- latency under concurrency

---

## Security

- TLS (HTTPS)
- standard auth mechanisms (headers, tokens)

---

## Summary

Best for simplicity and compatibility, not maximum performance.
