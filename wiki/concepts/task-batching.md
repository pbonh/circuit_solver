---
title: "Task Batching"
type: concept
tags: [distributed-systems, scheduling, optimization, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/SystemsForBigGraphAnalytics/_txt/03-part-ii-think-like-a-graph.txt"]
confidence: medium
---

## Definition

Task batching is a scheduling strategy in which a worker pulls a group of independent tasks at once, issues all their data requests together, waits for the bulk response, and only then executes the tasks — amortizing communication latency, deduplicating requests, and bounding memory by keeping a fixed-size in-memory active set with overflow on disk.

## How It Works

In G-thinker, each worker maintains an in-memory active-task buffer and a disk-based task queue. The worker fetches a batch of tasks from the queue, runs each task's `compute(frontier)`, collects the union of vertices it needs to pull, sends a combined request to remote workers (one request per remote vertex, even if many tasks need it), receives responses into a shared LRU vertex cache, and then resumes each task in the batch. Tasks that need more vertices are pushed back onto the disk queue.

## Key Parameters

- Batch size (memory budget vs. throughput).
- LRU cache size for non-local vertices.
- On-disk task-queue paging granularity.
- Request-deduplication policy.

## When To Use

- Subgraph-centric graph mining where each task triggers small network round-trips.
- Workloads where small messages would otherwise underutilize bandwidth.
- Out-of-core scheduling that must bound peak RAM.

## Risks & Pitfalls

- Batches that are too small fail to amortize latency; too large blow memory and stall on the slowest task.
- Garbage collection of completed tasks must be deterministic to avoid leaks.
- Recursive task creation (e.g., subgraph subdivision) must be careful not to starve the active batch.

## Related Concepts

- [[concepts/subgraph-centric-computation]]
- [[concepts/lru-vertex-cache]]

## Sources

- [[summaries/systems-big-graph-analytics-03-part-ii-think-like-a-graph]]
