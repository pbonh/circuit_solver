---
title: Tensor Product
type: claim
id: concepts/tensor-product
tags:
- graph
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/GuideToGraphAlgorithms/_txt/07-recent-trends.txt
confidence:
  base: 0.95
  source_count: 1
  contradicted: false
  effective: 0.95
  inputs_hash: 8331cbe4e16ebf56
---

## Definition

The tensor product (also called categorical or direct product) of graphs G and H is the graph G × H with V(G × H) = V(G) × V(H), and vertices (g_1, h_1) and (g_2, h_2) adjacent iff {g_1, g_2} ∈ E(G) and {h_1, h_2} ∈ E(H).

## How It Works

Hedetniemi's conjecture: χ(G × H) = min{χ(G), χ(H)}. The lower bound is trivial. Shitov (2019) gave a counterexample disproving the conjecture in general. For perfect graphs (Theorem 4.226), the conjecture holds.

For independence:
- α(G × H) ≥ max{α(G) · |V(H)|, α(H) · |V(G)|} but this is not tight (Jha-Klavžar).
- For cographs, α(G × H) is computable in O(n^2) via tree-based recursion.

The tensor capacity Θ(G) = lim_{k → ∞} r(G^k) (independence ratio of tensor powers) equals a^*(G) (Tóth).

## Key Parameters

- |V(G × H)| = |V(G)| · |V(H)|.
- |E(G × H)| = 2|E(G)| · |E(H)|.

## When To Use

- Coloring conjectures and counterexamples.
- Tensor capacity / Shannon capacity studies.
- Modeling product spaces in combinatorics.

## Risks & Pitfalls

- Tensor product ≠ Cartesian product G □ H (different edge rules).
- The tensor product of two perfect graphs need not be perfect (paw × K_3 contains an induced C_5).

## Related Concepts

- [[concepts/cartesian-product]]
- [[concepts/hedetniemi-conjecture]]
- [[concepts/tensor-capacity]]
- [[concepts/chromatic-number]]
- [[concepts/cograph]]

## Sources

- [[summaries/guide-to-graph-algorithms-07-recent-trends]]
