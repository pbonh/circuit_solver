---
title: Vizing's Conjecture
type: claim
id: claim-vizings-conjecture
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

Vizing's conjecture (1968) states that for any two graphs G and H, γ(G □ H) ≥ γ(G) · γ(H), where γ(·) is the domination number and □ is the Cartesian product.

## Status

Open in general. Proved for many special cases:
- Aharoni-Szabó (2009): true when G is chordal.
- Suen-Tarr (2012): γ(G □ H) ≥ (1/2) γ(G) · γ(H) + min(γ(G), γ(H)) for arbitrary G, H — a "1/2 + small" partial result.

For all graphs: γ(G □ H) ≥ γ_i(G) · γ(H), where γ_i is the independence domination number (Aharoni-Szabó). This is weaker but always provable.

## How It Works

Independence domination γ_i: max γ(A) over independent sets A. Always γ_i ≤ γ. For cographs, γ_i = number of components.

Computing γ on chordal graphs is NP-complete despite Aharoni-Szabó's structural result; Vizing's conjecture proof for chordal G is non-algorithmic.

## Key Parameters

- γ(G), γ(H).
- γ_i(G), γ_i(H).
- Currently known: γ(G □ H) ≥ (1/2) γ(G) γ(H) + min(γ(G), γ(H)).

## When To Use

- Estimating domination in product networks (interconnects, mesh networks).
- Bounding domination numbers from below.

## Risks & Pitfalls

- The conjecture remains open in full generality.
- For non-Cartesian products (tensor, strong, lexicographic) different bounds apply.

## Related Concepts

- [[concepts/dominating-set]]
- [[concepts/cartesian-product]]
- [[concepts/chordal-graph]]

## Sources

- [[summaries/guide-to-graph-algorithms-07-recent-trends]]
