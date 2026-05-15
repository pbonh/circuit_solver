---
title: "Edge Clique Cover"
type: concept
tags: [graph, foundational, well-established, np-hard]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/GuideToGraphAlgorithms/_txt/07-recent-trends.txt"]
confidence: high
---

## Definition

An edge clique cover of a graph G (no isolated vertices) is a collection of maximal cliques whose union contains every edge of G. The edge clique cover number cc(G) — also denoted θ_e(G) — is the minimum number of cliques.

Equivalently, θ_e(G) is the minimum size of a set U such that G is the intersection graph of a family of subsets of U.

## How It Works

Computing cc(G) is NP-complete (Kou-Stockmeyer-Wong 1978). For triangle-free graphs, cc(G) = |E(G)| (each edge is its own clique). For tensor products K_n × K_n, cc(K_n × K_n) = n(n - 1) iff a projective plane of order n exists.

For graphs with bounded clique number ω ≤ k, cc is polynomial-time computable.

## Key Parameters

- cc(G).
- For graphs without clique separators, cc = τ (tree-degree).

## When To Use

- Modeling edge constraints as group memberships.
- Recognition of intersection-graph classes.

## Risks & Pitfalls

- cc(G) ≥ m / C(ω, 2) is a trivial lower bound.
- Tight upper bound requires careful clique selection.

## Related Concepts

- [[concepts/clique]]
- [[concepts/maximal-clique]]
- [[concepts/tree-degree]]
- [[concepts/intersection-graph]]
- [[concepts/equivalence-cover]]

## Sources

- [[summaries/guide-to-graph-algorithms-07-recent-trends]]
