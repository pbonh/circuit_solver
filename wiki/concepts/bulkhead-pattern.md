---
title: "Bulkhead Pattern"
type: concept
tags: [distributed-systems, fault-tolerance, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Foundations of Scalable Systems/_txt/05-part-ii-scalable-systems.txt"]
confidence: medium
---

## Definition

Named after the watertight partitions in a ship's hull, the bulkhead pattern isolates concurrent resource usage so that one overloaded API cannot starve unrelated APIs in the same service.

## How It Works

A bulkhead reserves a portion of the thread pool (or connection pool, queue, semaphore) for a particular operation. Once that operation's reservation is full, further calls fail fast or queue separately, while other operations retain their own dedicated capacity. Implemented in Resilience4j and Spring Cloud Circuit Breaker.

## Key Parameters

- Max concurrent calls per partition.
- Maximum wait duration before BulkheadFullException.

## When To Use

When several APIs share the same application server and one is dramatically more expensive or volatile than the others.

## Risks & Pitfalls

- Over-partitioning fragments capacity, reducing utilization.
- Coarse boundaries fail to isolate the volatile operation.

## Related Concepts

- [[concepts/circuit-breaker]]
- [[concepts/throttling]]
- [[concepts/cascading-failure]]
- [[concepts/thread-pool]]

## Sources

- [[summaries/foundations-scalable-systems-05-part-ii-scalable-systems]]
