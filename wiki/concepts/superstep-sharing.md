---
title: Superstep-Sharing
type: claim
id: claim-superstep-sharing
tags:
- graph
- distributed-systems
- query-processing
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/SystemsForBigGraphAnalytics/_txt/02-part-i-think-like-a-vertex.txt
confidence:
  base: 0.85
---

## Definition

Superstep-sharing, introduced by Quegel, is an execution model in which many graph queries co-execute against a shared in-memory graph; in each "super-round" every currently active query advances by exactly one Pregel-style superstep, and queries see their normal Pregel semantics while the system batches communication across queries.

## How It Works

Each query q's computation takes n_q supersteps; Quegel processes it across n_q + 1 super-rounds (the final super-round prints/dumps results). New queries arrive on a master-side queue. In a super-round every machine processes one superstep for every active query in sequence; messages, aggregators, and Q-data (per-query state held on every machine) are synchronized across the super-round boundary. Each machine stores three kinds of state: V-data (per-vertex graph topology), VQ-data (per-vertex, per-query value/active flag/incoming-message queue, allocated lazily), and Q-data (per-query control state). Distributed indexes can be pre-built (e.g., label→local-vertex inverted index for graph matching).

## Key Parameters

- Maximum number of concurrent queries (memory-bounded by VQ-data overhead).
- Frequency of new-query batching at super-round boundaries.
- Index-build UDFs (per-vertex, per-machine).

## When To Use

- Online graph-query workloads where each query touches a small fraction of vertices (point-to-point shortest path, k-hop reachability, graph matching).
- Settings where standard Pregel underutilizes network bandwidth because each individual query is light.
- Systems that must support both indexes and traversal-based queries.

## Risks & Pitfalls

- Allocating VQ-data for every query at every vertex it touches can balloon memory.
- Heterogeneous queries with very different superstep counts cause uneven super-round workload.
- Termination detection per query is more complex than for a single Pregel job.

## Related Concepts

- [[concepts/vertex-centric-programming]]
- [[concepts/bulk-synchronous-parallel]]

## Sources

- [[summaries/systems-big-graph-analytics-02-part-i-think-like-a-vertex]]
