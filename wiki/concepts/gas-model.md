---
title: GAS (Gather-Apply-Scatter) Model
type: claim
id: concepts/gas-model
tags:
- graph
- distributed-systems
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

GAS (Gather-Apply-Scatter) is a vertex programming model, popularized by PowerGraph, that decomposes each vertex's update into three UDFs: gather a value along each adjacent edge, apply the aggregated value to compute a new vertex state, and scatter updates back along adjacent edges.

## How It Works

The four user functions are:
- `gather(u, v)`: invoked on each edge adjacent to v, returns a value contributed toward v.
- `sum(combined, value)`: accumulates per-edge gather results locally.
- `apply(D_v, combined)`: uses the globally combined value to compute the new vertex value.
- `scatter(v, u)`: invoked on each adjacent edge to optionally update u (e.g., re-activate it).

By splitting gather and scatter across the edges of a vertex, the runtime can partition the edges of a high-degree vertex over multiple machines, achieving load balance on power-law graphs. PowerGraph adds delta caching so that, when only a few neighbors have changed, the cached gather result is updated incrementally rather than recomputed from scratch.

## Key Parameters

- Choice of edge-partitioning algorithm (greedy edge placement).
- Whether to enable delta caching.
- Scope: GraphChi adopts edge-scope GAS; VENUS uses vertex-scope; GridGraph fuses scatter and gather into streaming-apply.

## When To Use

- Power-law graphs with high-degree vertices, where vertex partitioning would create stragglers.
- Algorithms naturally expressed as per-edge contributions (PageRank, label propagation, BFS).
- Single-PC disk-based processing where streaming edges is the cost-dominant operation.

## Risks & Pitfalls

- Computing the global combined value requires synchronizing edge-partitioned replicas.
- Some algorithms (pointer jumping, request-response) do not fit GAS naturally.
- Edge partitioning can increase vertex replication; vertex-cut minimization is essential.

## Related Concepts

- [[concepts/vertex-centric-programming]]
- [[concepts/shared-memory-graph-abstraction]]
- [[concepts/vertex-cut-partitioning]]

## Sources

- [[summaries/systems-big-graph-analytics-02-part-i-think-like-a-vertex]]
