---
title: Thread
type: claim
id: claim-thread
tags:
- concurrency
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Foundations of Scalable Systems/_txt/04-part-i-the-basics.txt
confidence:
  base: 0.85
---

## Definition

A thread is an independent sequence of execution within a process, with its own stack but shared access to the process's heap, globals, and OS resources. Threads are the primary mechanism for in-process concurrency in mainstream languages.

## How It Works

The OS or language runtime schedules threads onto CPU cores, preempting them when their time slice expires or a higher-priority thread becomes ready. Multiple threads on the same data structure require explicit synchronization to avoid race conditions.

## Key Parameters

- Stack size per thread (~1 MB default in Java).
- Thread priority.
- Time slice / preemption interval.

## When To Use

For overlapping CPU work with I/O, exploiting multicore parallelism, or implementing concurrent server APIs.

## Risks & Pitfalls

- Memory overhead grows with thread count.
- Context switching is expensive at high concurrency.
- Bare threads invite race conditions and deadlocks.

## Related Concepts

- [[concepts/concurrency]]
- [[concepts/thread-pool]]
- [[concepts/race-condition]]
- [[concepts/deadlock]]

## Sources

- [[summaries/foundations-scalable-systems-04-part-i-the-basics]]
