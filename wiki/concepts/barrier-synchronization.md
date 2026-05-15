---
title: "Barrier Synchronization"
type: concept
tags: [concurrency, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Foundations of Scalable Systems/_txt/04-part-i-the-basics.txt"]
confidence: medium
---

## Definition

Barrier synchronization is a coordination primitive where a set of threads each independently reach a barrier point and then block until all participants have arrived, after which all are released together.

## How It Works

Java provides three barrier primitives. `CountDownLatch` is a one-shot latch where `await()` blocks until `countDown()` has been called N times. `CyclicBarrier` and `Phaser` provide reusable, multi-phase variants. A common pattern is one coordinator thread launching N workers, each calling `countDown()` on completion while the coordinator `await()`s.

## Key Parameters

- Barrier count / number of participants.
- Reusability (single-use latch vs. cyclic).

## When To Use

Fork/join patterns, parallel data processing where downstream stages need all upstream results, integration tests that wait for all setup to complete.

## Risks & Pitfalls

- Forgetting to call `countDown()` causes permanent block.
- A failed participant can hang all others unless timeouts are used.

## Related Concepts

- [[concepts/thread]]
- [[concepts/thread-pool]]
- [[concepts/concurrency]]

## Sources

- [[summaries/foundations-scalable-systems-04-part-i-the-basics]]
