---
title: "GBASE"
type: entity
tags: [graph, big-data, sparse-matrix, mapreduce, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/SystemsForBigGraphAnalytics/_txt/04-part-iii-think-like-a-matrix.txt"]
confidence: medium
---

## Overview

GBASE (Kang et al., SIGKDD 2011; VLDB Journal 2012) is a MapReduce-based big-graph system, part of IBM's System G Toolkit. It supports both global queries (full-graph traversal) and targeted queries (touching only part of the graph) via built-in graph operations expressed as exact matrix-vector multiplications on the adjacency or incidence matrix.

## Characteristics

- User interface is the set of built-in algorithms — there is no custom-algorithm API like PEGASUS's.
- Built-in operations reduce to v_out ← M · v_in where M is A, Aᵀ, or Bᵀ.
- Two operation cases: v_out is a vertex-set (length |V|) for A/Aᵀ; v_out is an edge-set (length |E|) for Bᵀ.
- Matrix reordered by node clustering, partitioned into homogeneous dense or sparse blocks, GZip-compressed (achieving <2% of original graph size in experiments).
- Grid placement of blocks: blocks aggregated into files via a coarse grid so that both in-neighbor and out-neighbor queries read O(√n) files.

## Common Strategies

- Compose simple graph queries from built-in matrix-vector primitives (k-hop neighborhood, ego-network).
- Use compressed block storage to fit the graph in cluster disk space.
- Pre-cluster nodes once; reuse across many subsequent targeted queries.

## Related Entities

- [[entities/pegasus]]
- [[entities/systemml]]
- [[concepts/mapreduce]]

## Sources

- [[summaries/systems-big-graph-analytics-04-part-iii-think-like-a-matrix]]
