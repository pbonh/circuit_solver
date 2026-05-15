---
title: "k-Shortest Path on DAGs"
type: concept
tags: [algorithm, graph, foundational, optimization]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/AdvancedSymbolicAnalysisForVLSISystems/_txt/08-4-determinant-decision-diagrams.txt"]
confidence: medium
---

## Definition

k-Shortest Path on a DAG returns the `k` lowest-weight paths from source to sink in a directed acyclic graph. In DDD-based symbolic analysis it is used (with edge weights `0` for zero-edges and `-log|a_i|` for one-edges) to extract the `k` dominant product terms of a determinant.

## How It Works

The first shortest path is found by topological-order relaxation (`O(V+E)`). For subsequent paths, the chosen path is subtracted from the graph using DDD `SubtractAndRelax`, which creates only new vertices on the depth of the path. Relaxation is restricted to these new vertices, giving `O(n)` per additional path where `n` is graph depth.

## Key Parameters

- Edge-weight choice (here `-log|a_i|`).
- `k`, the number of paths to extract.

## When To Use

- Dominant-term extraction for symbolic approximation.
- Top-k path enumeration in any DAG-encoded combinatorial set.

## Risks & Pitfalls

- Numerical underflow with very small `|a_i|` requires log-space accumulation.
- Requires a canonical DDD (DDD variants with vertex duplication, e.g., some non-shared forms, do not support subtraction cleanly).

## Related Concepts

- [[concepts/determinant-decision-diagram]]
- [[concepts/symbolic-approximation]]

## Sources

- [[summaries/advanced-symbolic-analysis-for-vlsi-systems-08-4-determinant-decision-diagrams]]
