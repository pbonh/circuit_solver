---
title: "PowerGraph"
type: entity
tags: [graph, distributed-systems, graph-processing, shared-memory, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/SystemsForBigGraphAnalytics/_txt/02-part-i-think-like-a-vertex.txt"]
confidence: high
---

## Overview

PowerGraph (Gonzalez et al., OSDI 2012) is the successor to Distributed GraphLab. It replaces the vertex-centric API with the GAS (Gather-Apply-Scatter) model and switches from vertex partitioning to edge partitioning (vertex-cut), enabling scalability to graphs with billions of edges and better load balance on power-law graphs.

## Characteristics

- GAS programming model with four UDFs: `gather(u,v)`, `sum(combined, value)`, `apply(D_v, combined)`, `scatter(v,u)`.
- Greedy edge-placement heuristic with three cases on A(u) ∩ A(v), A(u) ∪ A(v); coordinated periodically among partitioners.
- Delta caching of previously gathered combined values to avoid redundant gather phases when few neighbors changed.
- Reported to scale to a 1.5B-edge test graph, the largest in the chapter's comparison.

## Common Strategies

- Use the GAS API for any algorithm naturally expressed as per-edge contributions (PageRank, label propagation, BFS).
- Enable delta caching for slowly-changing algorithms.
- Plan for vertex-replica synchronization overhead as a tradeoff against load balance.

## Related Entities

- [[entities/graphlab]]
- [[entities/graphchi]]
- [[entities/pregel]]

## Sources

- [[summaries/systems-big-graph-analytics-02-part-i-think-like-a-vertex]]
