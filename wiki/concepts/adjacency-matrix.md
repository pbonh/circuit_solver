---
title: Adjacency Matrix
type: claim
id: claim-adjacency-matrix
tags:
- graph
- foundational
- well-established
- sparse-matrix
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/GuideToGraphAlgorithms/_txt/04-graphs.txt
confidence:
  base: 0.85
---

## Definition

The adjacency matrix A of a graph G = (V, E) is the symmetric 0/1 matrix indexed by V × V with A[x,y] = 1 iff {x,y} ∈ E, and 0 otherwise. For an undirected simple graph the diagonal is all zero and A = A^T.

## How It Works

Each row of A encodes the characteristic vector of the neighborhood of a vertex. Matrix powers count walks: A^k[x,y] equals the number of walks of length k between x and y. Many spectral graph algorithms (regularity lemma constructiveness, sensitivity conjecture via Cauchy interlacing, strongly regular graphs) operate directly on A. Fast matrix multiplication (e.g. O(n^α), α < 2.376) enables sub-cubic algorithms for triangle detection, diamond detection, and small-subgraph counting.

## Key Parameters

- Size n × n where n = |V|.
- Sparsity governed by m; for sparse graphs adjacency lists are preferable.
- For weighted graphs A holds the weights; for digraphs A is in general nonsymmetric.

## When To Use

- Spectral analyses (eigenvalues, ranks over GF[2]).
- Fast subgraph counting using matrix multiplication.
- Dense graphs where O(n^2) memory is acceptable.

## Risks & Pitfalls

- O(n^2) memory cost is prohibitive for huge sparse graphs.
- Constructing A from an edge list takes O(m) time but visiting all neighbors of x takes O(n), not O(d(x)).
- Care must be taken to distinguish between adjacency matrix of G and that of its complement Ḡ; they differ by J - I.

## Related Concepts

- [[concepts/graph]]
- [[concepts/sparse-matrix-methods]]
- [[concepts/cauchy-interlace-lemma]]

## Sources

- [[summaries/guide-to-graph-algorithms-04-graphs]]
- [[summaries/systems-big-graph-analytics-04-part-iii-think-like-a-matrix]]
