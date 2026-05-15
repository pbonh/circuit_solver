---
title: "GraphChi"
type: entity
tags: [graph, graph-processing, single-machine, out-of-core, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/SystemsForBigGraphAnalytics/_txt/02-part-i-think-like-a-vertex.txt"]
confidence: high
---

## Overview

GraphChi (Kyrola et al., OSDI 2012) is a single-PC out-of-core graph processing system designed as the single-machine counterpart of Distributed GraphLab. It uses the GAS model with edge-scope access and a sharded representation that enables efficient sliding-window streaming of edges on a single hard disk.

## Characteristics

- Vertex IDs renumbered 1..|V| in a preprocessing pass; partitioned into P intervals → P shards.
- Each shard Ii stores Dv for v ∈ Ii plus all in-edges to Ii, sorted by source vertex ID.
- Loading shard Ii triggers one sequential read per other shard for the relevant out-edge ranges, plus one sequential write back; O(P^2) non-sequential seeks per iteration.
- Supports semi-streaming mode when all Dv fit in memory.
- Supports graph mutation via per-shard edge buffers Bi; supports selective scheduling via per-iteration bitmap.

## Common Strategies

- Use sub-intervals when out-edges of an interval do not fit; reuse the same shard files across different memory sizes.
- Apply selective scheduling for algorithms where activity is sparse, but recognize that whole shards still load.
- Avoid for very-large-diameter graphs where many iterations are required (sequential streaming cost grows linearly with iterations).

## Related Entities

- [[entities/powergraph]]
- [[entities/x-stream]]
- [[entities/venus]]
- [[entities/gridgraph]]

## Sources

- [[summaries/systems-big-graph-analytics-02-part-i-think-like-a-vertex]]
