---
title: "Concurrency"
type: concept
tags: [distributed-systems, concurrency, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Foundations of Scalable Systems/_txt/04-part-i-the-basics.txt"]
confidence: high
---

## Definition

Concurrency is the ability of a software system to make progress on multiple computations simultaneously, either through interleaving on a single CPU or true parallel execution on multicore hardware. Concurrent execution overlaps useful work with I/O waits and exploits available cores.

## How It Works

The primary primitive in most mainstream languages is the thread: each thread has its own stack but shares the process global state. Other models include CSP/channels (Go), actors with mailboxes (Erlang), and single-threaded event loops (Node.js). Programmers must protect shared mutable state from race conditions using synchronization primitives.

## Key Parameters

- Number of threads or workers (and the size of any thread pool).
- Critical-section granularity.
- Scheduling policy (priority, time-slicing).

## When To Use

Whenever a server must process many requests or overlap CPU work with I/O. In scalable systems concurrency is unavoidable because every platform multiplexes requests internally.

## Risks & Pitfalls

- Race conditions silently corrupt shared state.
- Deadlocks block forward progress.
- Long critical sections cap parallel scalability (Amdahl's law).
- Context-switching overhead grows with thread count.

## Related Concepts

- [[concepts/thread]]
- [[concepts/race-condition]]
- [[concepts/deadlock]]
- [[concepts/amdahls-law]]

## Sources

- [[summaries/foundations-scalable-systems-03-preface]]
- [[summaries/foundations-scalable-systems-04-part-i-the-basics]]
