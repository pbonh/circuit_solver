---
title: Cartesian Product
type: claim
id: claim-cartesian-product
tags:
- graph
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/GuideToGraphAlgorithms/_txt/07-recent-trends.txt
confidence:
  base: 0.85
---

## Definition

The Cartesian product G □ H of two graphs G and H has vertex set V(G) × V(H). Two vertices (g_1, h_1) and (g_2, h_2) are adjacent iff:
- g_1 = g_2 and {h_1, h_2} ∈ E(H), or
- h_1 = h_2 and {g_1, g_2} ∈ E(G).

## How It Works

The Cartesian product preserves many properties: χ(G □ H) = max(χ(G), χ(H)); ω(G □ H) = max(ω(G), ω(H)).

Vizing's conjecture: γ(G □ H) ≥ γ(G) · γ(H). Open in general; proved for chordal G by Aharoni-Szabó (2009). Suen-Tarr (2012): γ(G □ H) ≥ (1/2) γ(G) · γ(H) + min{γ(G), γ(H)}.

Independence domination γ_i: γ(G □ H) ≥ γ_i(G) · γ(H) and γ_i(G □ H) ≥ γ_i(G) · γ_i(H).

## Key Parameters

- |V(G □ H)| = |V(G)| · |V(H)|.
- |E(G □ H)| = |E(G)| · |V(H)| + |V(G)| · |E(H)|.

## When To Use

- Grids and their analogues (Cartesian products of paths).
- Vizing's domination conjecture.
- Hypercubes Q_n = K_2 □ K_2 □ … □ K_2.

## Risks & Pitfalls

- Cartesian product ≠ tensor product G × H (different adjacency rules).
- The Cartesian product of two trees is not a tree in general.

## Related Concepts

- [[concepts/tensor-product]]
- [[concepts/vizings-conjecture]]
- [[concepts/dominating-set]]
- [[concepts/hypercube]]

## Sources

- [[summaries/guide-to-graph-algorithms-07-recent-trends]]
