---
title: VENUS
type: entity
id: entities/venus
tags:
- graph
- graph-processing
- single-machine
- out-of-core
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/SystemsForBigGraphAnalytics/_txt/02-part-i-think-like-a-vertex.txt
---

## Overview

VENUS (Cheng et al., ICDE 2015) is a single-PC graph-processing system that exposes a vertex-scope GAS programming model and pioneers vertex-centric streamlined processing (VSP). It separates static structure data from mutable vertex values so that the available memory caches as many vertex values as possible, while the structure is streamed once per iteration. Not open-sourced.

## Characteristics

- Vertex-scope model: v accesses Dv and Du for u ∈ in(v); v never has to write to edges.
- Per-interval g-shard (in-edges, ordered by destination, plus read-only edge attributes) and v-shard (vertices in the g-shard).
- Two out-of-core algorithms: materialized v-shard with ordered per-interval writes, or merge-join of v-shard IDs with the vertex-value table.
- Restricted to computation on static graphs (no mutation).

## Common Strategies

- Use for static-graph workloads where streaming structure once and caching values pays off (PageRank, label propagation).
- Choose between materialization and merge-join based on memory budget.

## Related Entities

- [[entities/graphchi]]
- [[entities/x-stream]]
- [[entities/gridgraph]]

## Sources

- [[summaries/systems-big-graph-analytics-02-part-i-think-like-a-vertex]]
