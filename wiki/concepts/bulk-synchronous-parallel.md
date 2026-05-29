---
title: Bulk Synchronous Parallel (BSP)
type: claim
id: claim-bulk-synchronous-parallel
tags:
- parallel
- distributed-systems
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/SystemsForBigGraphAnalytics/_txt/02-part-i-think-like-a-vertex.txt
confidence:
  base: 0.85
---

## Definition

Bulk Synchronous Parallel (BSP) is a parallel-execution model in which computation proceeds in a sequence of supersteps; within a superstep each worker computes on its local state in parallel, exchanges messages, and waits at a global barrier before advancing to the next superstep.

## How It Works

Each worker holds a partition of the data. In a superstep, every worker (a) processes incoming messages from the previous superstep, (b) performs local computation that may generate outgoing messages, and (c) signals completion. A global synchronization barrier ensures all messages are delivered and all workers have finished before the next superstep starts. Pregel adapts BSP to graph processing: a vertex-level `compute(messages)` UDF is invoked once per active vertex per superstep.

## Key Parameters

- Superstep boundary: when to flush messages, run aggregators, and checkpoint.
- Communication primitive (all-to-all exchange, point-to-point sends).
- Termination condition (often: all vertices halted AND no pending messages).

## When To Use

- Iterative graph algorithms with a clear superstep structure.
- Workloads tolerant of synchronization overhead but benefiting from deterministic global state per step.
- Problems where checkpointing at superstep boundaries gives clean fault tolerance.

## Risks & Pitfalls

- The barrier introduces a round-trip network delay each superstep; algorithms requiring many supersteps suffer.
- The slowest worker dominates each superstep — straggler-sensitive.
- Asynchronous algorithms (e.g., GraphLab) can sometimes converge faster but at the cost of consistency complexity.

## Related Concepts

- [[concepts/vertex-centric-programming]]
- [[concepts/superstep-sharing]]
- [[concepts/lightweight-checkpointing]]

## Sources

- [[summaries/ddia-05-part-iii-derived-data]]
- [[summaries/systems-big-graph-analytics-02-part-i-think-like-a-vertex]]
