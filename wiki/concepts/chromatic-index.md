---
title: Chromatic Index
type: claim
id: concepts/chromatic-index
tags:
- graph
- foundational
- well-established
- np-hard
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/GuideToGraphAlgorithms/_txt/05-algorithms.txt
confidence:
  base: 0.95
  source_count: 1
  contradicted: false
  effective: 0.95
  inputs_hash: 8331cbe4e16ebf56
---

## Definition

The chromatic index χ'(G) is the minimum number of colors needed to color the edges of G such that no two edges sharing an endpoint have the same color. Equivalently, χ'(G) = χ(L(G)), the chromatic number of the linegraph.

By Vizing's theorem, χ'(G) ∈ {Δ(G), Δ(G) + 1}, where Δ is the maximum degree.

## How It Works

Vizing's algorithm gives a (Δ + 1)-edge-coloring in polynomial time. Holyer (1981) proved that deciding whether χ'(G) = Δ or Δ + 1 is NP-complete even for cubic graphs (Δ = 3).

For bipartite graphs, König's theorem gives χ'(G) = Δ in polynomial time (via matchings).

## Key Parameters

- χ'(G) ∈ {Δ, Δ+1}.
- For multigraphs, Vizing's bound extends to χ' ≤ Δ + μ where μ is the maximum edge-multiplicity.

## When To Use

- Scheduling round-robin tournaments.
- Wavelength assignment in optical networks.
- Edge coloring of dependency graphs.

## Risks & Pitfalls

- "Δ vs. Δ + 1" sounds small but is NP-hard to decide.
- Edge coloring is different from edge clique cover and equivalence cover; the three quantities can differ.

## Related Concepts

- [[concepts/chromatic-number]]
- [[concepts/linegraph]]
- [[concepts/equivalence-cover]]

## Sources

- [[summaries/guide-to-graph-algorithms-05-algorithms]]
