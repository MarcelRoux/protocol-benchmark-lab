# Work Endpoint: Prime Computation

## Overview

This endpoint introduces a **CPU-bound workload** to complement I/O-bound benchmarks (e.g. `/echo`).

It computes the number of prime numbers up to a given limit using different algorithms, allowing comparison of:

- runtime overhead
- CPU utilization
- concurrency behaviour under load
- framework impact on compute-heavy handlers

---

## Endpoints

### HTTP Request

```bash
GET /work/primes/brute/{limit}
GET /work/primes/brute-odd/{limit}
GET /work/primes/sieve/{limit}
```

### Response

```json
{
    "limit": 1000000,
    "algorithm": "sieve",
    "prime_count": 78498,
    "duration_ms": 123
}
```

---

## Algorithms

### 1. Brute Force

#### Idea

For each number `n` from `2` to `limit`:

- test divisibility by all integers from `2` to `n-1`

#### Intuition

A number is prime if nothing divides it.

#### Complexity

- Time: O($n^2$)
- Very slow for large inputs

#### Purpose

- baseline reference
- demonstrates naive approach

---

### 2. Optimized Brute Force

#### Improvements

- skip even numbers after `2`
- test divisors only up to $\sqrt{n}$

#### Intuition

If `n = a x b`, at least one factor is $\le$ $\sqrt{n}$

- no need to test beyond that

#### Complexity

- Time: ~O($n\sqrt{n}$)
- significantly faster than naive

#### Purpose

- demonstrates algorithmic improvement
- still CPU-heavy

---

### 3. Sieve of Eratosthenes (Primary)

#### Idea

- create boolean array `is_prime[0..limit]`
- initialize all values as `true`
- starting from `2`, mark multiples as `false`

#### Process

1. assume all numbers are prime
2. pick next unmarked number `p`
3. mark all multiples of `p` as non-prime
4. repeat until $p^2 \gt limit$

#### Intuition

Instead of checking if a number is prime:

> eliminate all numbers that are definitely not prime

#### Complexity

- Time: ~O($n log log n$)
- Space: O($n$)

#### Purpose

- primary algorithm for benchmarking
- efficient and deterministic
- widely understood baseline

---

## Why Single-Threaded

All algorithms are implemented **single-threaded** for the initial version.

Rationale:

- ensures comparability across languages
- avoids runtime-specific scheduling differences
- isolates framework + runtime overhead
- simplifies implementation

---

## Future Extensions

- parallel sieve (`algorithm: "sieve_parallel"`)
- alternative workloads:
  - hashing
  - sorting
  - matrix operations

---

## Benchmark Scenarios

### 1. CPU Baseline

- single request
- measure raw compute time

### 2. CPU Concurrency

- multiple concurrent requests
- observe contention and scheduling

### 3. Mixed Workload

- combine `/echo` and `/work/primes`
- observe interference between I/O and CPU

---

## Metrics Focus

- p50 / p95 / p99 latency
- requests/sec under load
- CPU utilization
- error rate under saturation

---

## Notes

- `/echo` remains the primary protocol benchmark
- `/work/primes` introduces a **CPU-bound dimension**
- focus is on **relative comparison**, not absolute optimization
