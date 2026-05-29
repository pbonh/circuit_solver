---
title: Antipodality
type: claim
id: claim-antipodality
tags:
- graph
- advanced
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/GuideToGraphAlgorithms/_txt/07-recent-trends.txt
confidence:
  base: 0.85
---

## Definition

For a connected planar graph G drawn on a sphere with dual G*, p : E(G) → ℤ_{≥0}, and k ∈ ℤ_{≥0}, an antipodality of p-range ≥ k is a function α with domain E(G) ∪ R(G) such that for every edge e, α(e) is a subgraph; for every region r, α(r) ⊆ V is nonempty; satisfying:

(A1) α(e) does not contain an endpoint of e.
(A2) If e is incident with r, then α(r) ⊆ V(α(e)) and every component of α(e) has a vertex in α(r).
(A3) If e ∈ E and f ∈ E(α(e)), then every closed walk in G* containing e* and f* has p-length ≥ k.

## How It Works

The Seymour-Thomas characterization (Theorem 4.33): for a connected planar graph with all p(δ(x)) < k, the p-carving width is ≥ k iff there is an antipodality of p-range ≥ k.

Computational tool: round sets in an auxiliary graph M decide antipodality existence in O(m^2). The result feeds the planar treewidth approximation.

## Key Parameters

- p-range k.
- Connection to carving width via tilts and slopes.

## When To Use

- Proving lower bounds on carving / branch width in planar graphs.
- As the dual certificate in the Robertson-Seymour Graph Minors program.

## Risks & Pitfalls

- The combinatorial setup (slopes, tilts, biases) is intricate; getting the precise definitions right is essential.
- Applies only to planar graphs in this form.

## Related Concepts

- [[concepts/carving-width]]
- [[concepts/carving]]

## Sources

- [[summaries/guide-to-graph-algorithms-07-recent-trends]]
