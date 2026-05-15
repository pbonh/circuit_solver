---
title: "Hypercube"
type: concept
tags: [graph, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/GuideToGraphAlgorithms/_txt/07-recent-trends.txt"]
confidence: high
---

## Definition

The n-dimensional hypercube Q_n is the graph with vertex set {0, 1}^n and edges between strings differing in exactly one coordinate. It is n-regular with 2^n vertices and n · 2^(n-1) edges.

## How It Works

Lemma 4.210: the spectrum of Q_n consists of the values n - 2i with multiplicity C(n, i), for i = 0, …, n.

The Hypercube Theorem (Huang 2019, Theorem 4.211): any induced subgraph H of Q_n with at least 2^(n-1) + 1 vertices has max degree Δ(H) ≥ √n. The proof uses Cauchy's interlace lemma on Huang's {0, -1, +1}-matrix A_n (with eigenvalues ±√n of multiplicity 2^(n-1) each).

Combined with the Gotsman-Linial equivalence theorem and Tal's lemma, this resolves the sensitivity conjecture.

## Key Parameters

- Dimension n.
- Number of vertices 2^n.
- Number of edges n · 2^(n-1).

## When To Use

- Foundational graph in coding theory.
- Models for parallel-machine interconnection.
- Spectral graph theory benchmark.

## Risks & Pitfalls

- The hypercube is bipartite, so its spectrum is symmetric around 0.
- Confusion with Boolean lattice (same vertex set, but with edges replaced by partial order).

## Related Concepts

- [[concepts/sensitivity]]
- [[concepts/cauchy-interlace-lemma]]
- [[concepts/bipartite-graph]]

## Sources

- [[summaries/guide-to-graph-algorithms-07-recent-trends]]
