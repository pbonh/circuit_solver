---
title: "Thread Pool"
type: concept
tags: [concurrency, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Foundations of Scalable Systems/_txt/04-part-i-the-basics.txt"]
confidence: high
---

## Definition

A thread pool is a bounded collection of worker threads that execute submitted tasks. Pools amortize the cost of thread creation, cap concurrency to protect resources, and queue work that exceeds capacity.

## How It Works

Tasks are submitted to a work queue. Idle workers pull tasks; if no worker is available, the task waits until one is. Java's `ExecutorService` (e.g., `Executors.newFixedThreadPool(n)`) is the canonical example; Tomcat exposes container thread pools (default 25 minimum, 200 maximum).

## Key Parameters

- Core and maximum pool size.
- Queue type and capacity (bounded vs. unbounded).
- Idle-thread-killing timeout.
- Rejection policy (block, abort, caller-runs, discard).

## When To Use

Almost any server platform that processes many small concurrent tasks: HTTP servers, message brokers, batch jobs.

## Risks & Pitfalls

- Unbounded queues mask overload until memory exhausts.
- Pool sized too small starves requests; too large wastes context switches.
- Deadlock when tasks in the pool wait for other tasks in the same pool.

## Related Concepts

- [[concepts/concurrency]]
- [[concepts/thread]]
- [[concepts/throttling]]
- [[concepts/bulkhead-pattern]]

## Sources

- [[summaries/foundations-scalable-systems-04-part-i-the-basics]]
- [[summaries/rust-book-21-chapter-20-final-project-building-a-multithreaded-web-server]]
