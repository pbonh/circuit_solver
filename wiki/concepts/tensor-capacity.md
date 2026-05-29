---
title: Tensor Capacity
type: claim
id: claim-tensor-capacity
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

For a graph G, the independence ratio is r(G) = α(G) / |V(G)|. The tensor capacity is
  Θ(G) = lim_{k → ∞} r(G^k),
where G^k = G × G × … × G (k times) is the tensor power. The limit exists because r(G^k) is non-decreasing and bounded by 1.

## How It Works

Tóth (2011) proved Θ(G) = a^*(G), where:
- a(G) = max_{I independent} |I| / (|I| + |N(I)|).
- a^*(G) = a(G) if a(G) ≤ 1/2, else 1.

Equivalently, a^*(G^2) = a^*(G) for every graph G.

For cographs (Theorem 4.231): tensor capacity is polynomial-time computable via cotree-DP that maintains tables of (|I|, |N(I)|) pairs at each node.

Computing Θ is NP-complete in general; for bounded-treewidth graphs it is in time O(3^(k+1) · n^3). For planar graphs of max degree 3, even α(G × K_4) is NP-complete.

## Key Parameters

- a(G) and a^*(G).
- For perfect graphs, Θ relates to the Shannon capacity via Lovász theta.

## When To Use

- Information theory (Shannon capacity for source coding).
- Analysis of "asymptotic" independent set behavior.

## Risks & Pitfalls

- Tensor capacity ≠ Shannon capacity in general.
- Computation is NP-complete in general; even for special graph classes polynomial algorithms are non-trivial.

## Related Concepts

- [[concepts/tensor-product]]
- [[concepts/independent-set]]
- [[concepts/cograph]]

## Sources

- [[summaries/guide-to-graph-algorithms-07-recent-trends]]
