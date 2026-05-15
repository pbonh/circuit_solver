---
title: "Race Condition"
type: concept
tags: [concurrency, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Foundations of Scalable Systems/_txt/04-part-i-the-basics.txt"]
confidence: high
---

## Definition

A race condition occurs when the correctness of a program depends on the relative timing of concurrent operations on shared mutable state. The classic example is a lost update where two threads read a counter, each increment it, and each write back, losing one of the updates.

## How It Works

High-level statements like `count++` are not atomic at the machine level; they decompose into load-modify-store sequences that can be interleaved. Without synchronization, interleavings depend on the scheduler, producing non-deterministic, hard-to-reproduce bugs.

## Key Parameters

- Critical-section granularity.
- Synchronization mechanism (monitors, atomics, RWLocks).

## When To Use

Recognize and eliminate race conditions whenever multiple threads modify shared data. Identify critical sections and serialize access (in Java: `synchronized`; in C++: `std::mutex`; etc.) or use thread-safe data structures.

## Risks & Pitfalls

- Bugs occur rarely and only under load; testing rarely catches them.
- Over-synchronization throttles parallelism and approaches Amdahl-bound serial limits.

## Related Concepts

- [[concepts/concurrency]]
- [[concepts/thread]]
- [[concepts/deadlock]]
- [[concepts/amdahls-law]]

## Sources

- [[summaries/foundations-scalable-systems-04-part-i-the-basics]]
