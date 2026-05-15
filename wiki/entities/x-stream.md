---
title: "X-Stream"
type: entity
tags: [graph, graph-processing, single-machine, out-of-core, edge-centric, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/SystemsForBigGraphAnalytics/_txt/02-part-i-think-like-a-vertex.txt"]
confidence: high
---

## Overview

X-Stream (Roy et al., SOSP 2013) is a single-PC graph-processing system that adopts an edge-centric streaming computation model. Unlike GraphChi, it does not require edges to be sorted, instead streaming a completely unordered edge list twice per iteration (scatter, then gather). It exploits high sequential bandwidth of disk, SSD, or RAM.

## Characteristics

- Three UDFs: `init(v)` re-initializes vertex value, `generate_update((u,v))` produces an update from u's state, `apply_one_update((u,v), m)` applies the update to v's state.
- Each iteration: scatter pass streams edges and writes updates to per-partition update files U_i; gather pass streams U_i and applies updates to vertex chunk V_i.
- Out-of-core support via vertex partitions V_i sized to fit in memory and associated edge partitions E_i.
- Inefficient on hard disks for sparse computation (always streams every edge, no skip mechanism).
- Distributed extension is Chaos (SOSP 2015) for SSD/40GbE clusters.

## Common Strategies

- Use when in-memory or SSD performance is critical and the workload has dense activity each iteration.
- Avoid for large-diameter graphs requiring many iterations of sparse computation.
- Pair with Chaos for scale-out when bandwidth is plentiful.

## Related Entities

- [[entities/graphchi]]
- [[entities/chaos]]
- [[entities/venus]]
- [[entities/gridgraph]]

## Sources

- [[summaries/systems-big-graph-analytics-02-part-i-think-like-a-vertex]]
