---
title: Shared-Memory Graph Abstraction
type: claim
id: concepts/shared-memory-graph-abstraction
tags:
- graph
- distributed-systems
- big-data
- graph-processing
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/SystemsForBigGraphAnalytics/_txt/02-part-i-think-like-a-vertex.txt
confidence:
  base: 0.95
  source_count: 1
  contradicted: false
  effective: 0.95
  inputs_hash: 8331cbe4e16ebf56
---

## Definition

A shared-memory graph abstraction is a programming model in which a vertex's UDF directly reads (and sometimes writes) the values of its neighbors and adjacent edges, as if all graph data lived in a single address space, even when the actual runtime is distributed or disk-resident.

## How It Works

The system assigns each vertex a scope — full-scope (in-neighbor values, in-edge values, own value, out-edge values, out-neighbor values) in GraphLab, or restricted vertex-scope or edge-scope variants in single-PC systems. When two endpoints of an edge sit on different machines, the runtime replicates ghost copies of the shared data and synchronizes them; locking enforces consistency in asynchronous mode. Disk-based single-PC systems (GraphChi, X-Stream, VENUS, GridGraph) keep edges on disk and stream them through a small memory buffer so that the user's UDF can pretend it accesses a shared structure.

## Key Parameters

- Scope size: full, vertex, or edge scope.
- Synchronous vs. asynchronous execution.
- Ghost-state synchronization frequency.
- For single-PC systems: number of shards/partitions, block layout (row-major vs. grid).

## When To Use

- Iterative graph algorithms with asymmetric convergence where direct neighbor reads simplify the UDF (e.g., PageRank with delta thresholds).
- Machine-learning-style updates that need bidirectional access to neighbor state.
- Single-PC out-of-core processing of moderate-size graphs.

## Risks & Pitfalls

- High data replication (vertex ghosts) inflates memory and synchronization cost.
- Asynchronous mode requires locking and approximate-result semantics.
- Edge-scope models save replication but cannot express algorithms needing non-neighbor communication (e.g., pointer jumping).
- Single-PC throughput is bounded by one machine's disk bandwidth.

## Related Concepts

- [[concepts/vertex-centric-programming]]
- [[concepts/gas-model]]
- [[concepts/vertex-cut-partitioning]]
- [[concepts/out-of-core-graph-processing]]

## Sources

- [[summaries/systems-big-graph-analytics-02-part-i-think-like-a-vertex]]
