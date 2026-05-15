---
title: "LRU Vertex Cache"
type: concept
tags: [distributed-systems, caching, graph-processing, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/SystemsForBigGraphAnalytics/_txt/03-part-ii-think-like-a-graph.txt"]
confidence: medium
---

## Definition

An LRU vertex cache is a per-worker least-recently-used cache that holds non-local vertices (and their adjacency lists) previously pulled from remote workers, so that multiple tasks on the same worker can share the cost of retrieving common vertices.

## How It Works

In G-thinker, each worker keeps a local table T_local of its assigned vertices plus a cache T_cache for remote vertices. When a task's `compute(frontier)` needs a vertex u, the worker first checks T_local and T_cache; on miss the request is added to the next batch of remote pulls. When the response arrives, ⟨u, Γ(u)⟩ is inserted into T_cache for use by every task that asked for u. The user-defined `respond(v)` UDF can prune Γ(v) before transmission (e.g., return only Γ_>(v) for clique enumeration) to save bandwidth and memory.

## Key Parameters

- Cache capacity (per worker, in bytes or vertex count).
- Eviction policy (LRU is standard; LFU or window-based variants possible).
- Whether to compress cached adjacency lists.
- `respond(v)` UDF pruning rules.

## When To Use

- Subgraph-centric mining where overlapping tasks repeatedly pull the same neighbors.
- Any workload with stable locality of access across concurrent tasks.

## Risks & Pitfalls

- Without coordination, two tasks may concurrently request the same vertex; deduplication at request time is essential.
- Cache thrashing if working-set exceeds cache capacity.
- Stale entries can be a problem if the graph is mutable (G-thinker assumes a static graph).

## Related Concepts

- [[concepts/subgraph-centric-computation]]
- [[concepts/task-batching]]

## Sources

- [[summaries/systems-big-graph-analytics-03-part-ii-think-like-a-graph]]
