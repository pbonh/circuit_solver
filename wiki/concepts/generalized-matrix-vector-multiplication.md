---
title: Generalized Matrix-Vector Multiplication
type: claim
id: claim-generalized-matrix-vector-multiplication
tags:
- graph
- sparse-matrix
- big-data
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/SystemsForBigGraphAnalytics/_txt/04-part-iii-think-like-a-matrix.txt
confidence:
  base: 0.85
---

## Definition

The PEGASUS generalized matrix-vector multiplication abstracts the standard inner-product update v_out[i] = Σ_j M[i][j]·v[j] into three user-defined operators — `combine2(M[i][j], v[j])` (pairwise combination), `combineAll` (reduction across columns), and `assign` (overwrite the vertex value) — so that the same matrix-vector skeleton expresses many different graph algorithms.

## How It Works

For PageRank, `combine2(m, x)` = m·x, `combineAll` is sum, and `assign` overwrites. For Hash-Min on the 0/1 adjacency matrix, `combine2(m, x)` = m·x, `combineAll` is min, and `assign` takes min(current, combineAll). Each PEGASUS iteration runs two MapReduce jobs: the first emits combine2 values keyed by row, and the second groups by row, applies combineAll, and writes the new vertex value. To improve throughput PEGASUS partitions M into b×b square submatrices and v into b-element blocks, reorders rows/columns by co-clustering to compact non-zeros, and for algorithms like Hash-Min repeatedly multiplies each diagonal block until its corresponding vector block stabilizes — propagating state inside the block before crossing block boundaries.

## Key Parameters

- Block size b.
- Co-clustering quality of row/column permutations.
- Whether to apply repeated-diagonal-block optimization.
- MapReduce platform tuning (split size, mapper/reducer counts).

## When To Use

- Iterative graph algorithms expressible as fix-point of a matrix-vector update with associative-commutative aggregation.
- MapReduce-based pipelines on Hadoop where vertex-centric runtimes are unavailable.
- Educational/benchmark settings for understanding the matrix-graph equivalence.

## Risks & Pitfalls

- Full-matrix multiplication every iteration is wasteful when few vertices change.
- MapReduce overheads (job startup, shuffle materialization) dominate small graphs.
- Cannot natively halt individual vertices, unlike Pregel.

## Related Concepts

- [[concepts/matrix-based-graph-analytics]]
- [[concepts/adjacency-matrix]]
- [[concepts/matrix-blocking]]

## Sources

- [[summaries/systems-big-graph-analytics-04-part-iii-think-like-a-matrix]]
